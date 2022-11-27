static USAGE: &str = r#"
Create a new computed column, filter rows or compute aggregations by executing a
Luau script for every line of a CSV file.

The executed Luau has 3 ways to reference row columns (as strings):
  1. Directly by using column name (e.g. Amount), can be disabled with -g
  2. Indexing col variable by column name: col.Amount or col["Total Balance"]
  3. Indexing col variable by column 1-based index: col[1], col[2], etc.

Of course, if your input has no headers, then 3. will be the only available
option.

Some usage examples:

  Sum numeric columns 'a' and 'b' and call new column 'c'
  $ qsv luau map c "a + b"
  $ qsv luau map c "col.a + col['b']"
  $ qsv luau map c "col[1] + col[2]"

  There is some magic in the previous example as 'a' and 'b' are passed in
  as strings (not numbers), but Luau still manages to add them up.
  A more explicit way of doing it, is by using tonumber
  $ qsv luau map c "tonumber(a) + tonumber(b)"

  Add running total column for Amount
  $ qsv luau map Total -x "tot = (tot or 0) + Amount; return tot"

  Or use the prologue and epilogue options to compute the running & grand totals
  $ qsv luau map Total --prologue "tot = 0; gtotal = 0" -x \
        "tot = tot + Amount; gtotal = gtotal + tot; return tot" --epilogue "return gtotal"

  Add running total column for Amount when previous balance was 900
  $ qsv luau map Total -x "tot = (tot or 900) + Amount; return tot"

  Convert Amount to always-positive AbsAmount and Type (debit/credit) columns
  $ qsv luau map Type -x \
        "if tonumber(Amount) < 0 then return 'debit' else return 'credit' end" | \
    qsv luau map AbsAmount "math.abs(tonumber(Amount))"

  Filter some lines based on numerical filtering
  $ qsv luau filter "tonumber(a) > 45"
  $ qsv luau filter "tonumber(a) >= tonumber(b)"

  Typing long scripts at command line gets tiresome rather quickly, so use the
  "file:" prefix to read non-trivial scripts from a file
  $ qsv luau map Type -P "file:init.lua" -x "file:debitcredit.lua" -E "file:end.lua"

With "luau map", if a Luau script is invalid, "<ERROR>" is returned.
With "luau filter", if a Luau script is invalid, no filtering is done.
Invalid scripts will also result in an exitcode of 1.

There are also special variables named "_idx" that is set to the current row number; and
"_rowcount" which is zero during the prologue and the main script, and set to the rowcount
during the epilogue.

Note that with the judicious use of the prologue and the _idx variable, one can create arrays
that can be used for complex aggregation operations in the epilogue.

When debugging luau, be sure to set the environment variable QSV_LOG_LEVEL=debug to see
detailed error messages in the logfile.

For more examples, see https://github.com/jqnatividad/qsv/blob/master/tests/test_luau.rs.

Usage:
    qsv luau map [options] -n <script> [<input>]
    qsv luau map [options] <new-column> <script> [<input>]
    qsv luau filter [options] <script> [<input>]
    qsv luau map --help
    qsv luau filter --help
    qsv luau --help

All <script> arguments/options can either be the Luau code, or if it starts with "file:",
the filepath from which to load the script. 

luau options:
    -x, --exec               exec[ute] Luau script, instead of the default eval[uate].
                             eval (default) expects just a single Luau expression,
                             while exec expects one or more statements, allowing
                             full-fledged Luau programs. This only applies to the main script
                             argument, not the prologue & epilogue scripts.
    -g, --no-globals         Don't create Luau global variables for each column, only col.
                             Useful when some column names mask standard Luau globals.
                             Note: access to Luau globals thru _G remains even without -g.
    -P, --prologue <script>  Luau script to execute BEFORE processing the CSV.
                             Typically used to initialize global variables.
    -E, --epilogue <script>  Luau script to execute AFTER processing the CSV.
                             Typically used for aggregations.
                             The output of the epilogue is sent to stderr.

Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
    -n, --no-headers       When set, the first row will not be interpreted
                           as headers. Namely, it will be sorted with the rest
                           of the rows. Otherwise, the first row will always
                           appear as the header row in the output.
    -d, --delimiter <arg>  The field delimiter for reading CSV data.
                           Must be a single character. (default: ,)
    -p, --progressbar      Show progress bars. Not valid for stdin.
"#;

use std::fs;

use indicatif::{ProgressBar, ProgressDrawTarget};
use log::{debug, info, log_enabled};
use mlua::Lua;
use serde::Deserialize;

use crate::{
    config::{Config, Delimiter},
    util, CliError, CliResult,
};

#[derive(Deserialize)]
struct Args {
    cmd_map:          bool,
    cmd_filter:       bool,
    arg_new_column:   Option<String>,
    arg_script:       String,
    arg_input:        Option<String>,
    flag_exec:        bool,
    flag_no_globals:  bool,
    flag_prologue:    Option<String>,
    flag_epilogue:    Option<String>,
    flag_output:      Option<String>,
    flag_no_headers:  bool,
    flag_delimiter:   Option<Delimiter>,
    flag_progressbar: bool,
}

impl From<mlua::Error> for CliError {
    fn from(err: mlua::Error) -> CliError {
        CliError::Other(err.to_string())
    }
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;
    let rconfig = Config::new(&args.arg_input)
        .delimiter(args.flag_delimiter)
        .checkutf8(false)
        .no_headers(args.flag_no_headers);

    let mut rdr = rconfig.reader()?;
    let mut wtr = Config::new(&args.flag_output).writer()?;

    let mut headers = rdr.headers()?.clone();

    if !rconfig.no_headers {
        if !args.cmd_filter {
            let new_column = args
                .arg_new_column
                .as_ref()
                .ok_or("Specify new column name")?;
            headers.push_field(new_column);
        }

        wtr.write_record(&headers)?;
    }

    let luau = Lua::new();
    let globals = luau.globals();
    globals.set("cols", "{}")?;

    let mut idx_used = false;
    let mut idx = 0_usize;

    // check if an epilogue was specified. We check it first for two reasons:
    // 1) so we don't run the main script only to err out on an invalid epilogue file not found.
    // 2) to see if _idx or _rowcount is used, even if they're not used in the prologue,
    //    so we can init the _idx and _rowcount global vars.
    let epilogue_script = if let Some(ref epilogue) = args.flag_epilogue {
        if let Some(epilogue_filepath) = epilogue.strip_prefix("file:") {
            match fs::read_to_string(epilogue_filepath) {
                Ok(epilogue) => {
                    // check if the epilogue uses _idx or _rowcount
                    idx_used = epilogue.contains("_idx") || epilogue.contains("_rowcount");
                    if idx_used {
                        globals.set("_idx", 0)?;
                        globals.set("_rowcount", 0)?;
                    }
                    epilogue
                }
                Err(e) => return fail_clierror!("Cannot load Luau epilogue file: {e}"),
            }
        } else {
            epilogue.to_string()
        }
    } else {
        String::new()
    };

    // check if a prologue was specified
    if let Some(prologue) = args.flag_prologue {
        let prologue_script = if let Some(prologue_filepath) = prologue.strip_prefix("file:") {
            match fs::read_to_string(prologue_filepath) {
                Ok(file_contents) => file_contents,
                Err(e) => return fail_clierror!("Cannot load Luau prologue file: {e}"),
            }
        } else {
            prologue
        };

        idx_used =
            idx_used || prologue_script.contains("_idx") || prologue_script.contains("_rowcount");
        if idx_used {
            globals.set("_idx", 0)?;
            globals.set("_rowcount", 0)?;
        }

        info!("Executing prologue. _idx used: {idx_used}");
        match luau.load(&prologue_script).exec() {
            Ok(_) => (),
            Err(e) => {
                return fail_clierror!(
                    "Prologue error: Failed to execute \"{prologue_script}\".\n{e}"
                )
            }
        }
        info!("Prologue executed.");
    }

    let luau_script = if let Some(script_filepath) = args.arg_script.strip_prefix("file:") {
        match fs::read_to_string(script_filepath) {
            Ok(file_contents) => file_contents,
            Err(e) => return fail_clierror!("Cannot load Luau file: {e}"),
        }
    } else {
        args.arg_script
    };

    idx_used = idx_used || luau_script.contains("_idx") || luau_script.contains("_rowcount");
    if idx_used {
        globals.set("_idx", 0)?;
        globals.set("_rowcount", 0)?;
    }

    let mut luau_program = if args.flag_exec {
        String::new()
    } else {
        String::from("return ")
    };

    luau_program.push_str(&luau_script);
    debug!("Luau program: {luau_program:?}");

    // prep progress bar
    let show_progress =
        (args.flag_progressbar || std::env::var("QSV_PROGRESSBAR").is_ok()) && !rconfig.is_stdin();

    let progress = ProgressBar::with_draw_target(None, ProgressDrawTarget::stderr_with_hz(5));
    if show_progress {
        util::prep_progress(&progress, util::count_rows(&rconfig)?);
    } else {
        progress.set_draw_target(ProgressDrawTarget::hidden());
    }

    let error_result: mlua::Value = luau.load("return \"<ERROR>\";").eval()?;
    let mut error_flag;
    let mut global_error_flag = false;

    // check if _idx or _rowcount was used in the main script
    idx_used = idx_used || luau_program.contains("_idx") || luau_program.contains("_rowcount");

    // pre-compile main script into bytecode
    // see https://docs.rs/mlua/latest/mlua/struct.Compiler.html
    let luau_compiler = if log_enabled!(log::Level::Debug) {
        // set more debugging friendly compiler settings if QSV_LOG_LEVEL=debug
        mlua::Compiler::new()
            .set_optimization_level(0)
            .set_debug_level(2)
            .set_coverage_level(2)
    } else {
        // use more performant compiler settings
        mlua::Compiler::new()
            .set_optimization_level(2)
            .set_debug_level(1)
            .set_coverage_level(0)
    };
    let bytecode = luau_compiler.compile(&luau_program);

    let mut record = csv::StringRecord::new();

    while rdr.read_record(&mut record)? {
        if show_progress {
            progress.inc(1);
        }

        // _idx is used, be sure to keep _idx set to current row number
        if idx_used {
            idx += 1;
            globals.set("_idx", idx)?;
        }

        // Updating col
        {
            let col = luau.create_table()?;

            for (i, v) in record.iter().enumerate() {
                col.set(i + 1, v)?;
            }
            if !rconfig.no_headers {
                for (h, v) in headers.iter().zip(record.iter()) {
                    col.set(h, v)?;
                }
            }
            globals.set("col", col)?;
        }

        // Updating global
        if !args.flag_no_globals && !rconfig.no_headers {
            for (h, v) in headers.iter().zip(record.iter()) {
                globals.set(h, v)?;
            }
        }

        error_flag = false;
        let computed_value: mlua::Value = match luau
            .load(&bytecode)
            .set_mode(mlua::ChunkMode::Binary)
            .eval()
        {
            Ok(computed) => computed,
            Err(e) => {
                log::error!("Cannot evaluate \"{luau_program}\".\n{e}");
                error_flag = true;
                global_error_flag = true;
                error_result.clone()
            }
        };

        if args.cmd_map {
            match computed_value {
                mlua::Value::String(string) => {
                    record.push_field(&string.to_string_lossy());
                }
                mlua::Value::Number(number) => {
                    let mut buffer = ryu::Buffer::new();
                    record.push_field(buffer.format(number));
                }
                mlua::Value::Integer(number) => {
                    let mut buffer = itoa::Buffer::new();
                    record.push_field(buffer.format(number));
                }
                mlua::Value::Boolean(boolean) => {
                    record.push_field(if boolean { "true" } else { "false" });
                }
                mlua::Value::Nil => {
                    record.push_field("");
                }
                _ => {
                    return fail_clierror!(
                        "Unexpected value type returned by provided Luau expression. \
                         {computed_value:?}"
                    );
                }
            }

            wtr.write_record(&record)?;
        } else if args.cmd_filter {
            let must_keep_line = if error_flag {
                true
            } else {
                match computed_value {
                    mlua::Value::String(strval) => !strval.to_string_lossy().is_empty(),
                    mlua::Value::Boolean(boolean) => boolean,
                    mlua::Value::Nil => false,
                    mlua::Value::Integer(intval) => intval != 0,
                    mlua::Value::Number(fltval) => (fltval).abs() > f64::EPSILON,
                    _ => true,
                }
            };

            if must_keep_line {
                wtr.write_record(&record)?;
            }
        }
    }

    if show_progress {
        util::finish_progress(&progress);
    }

    if !epilogue_script.is_empty() {
        // if idx_used, also set a convenience variable named _rowcount;
        // true, _rowcount is equal to _idx at this point, but this
        // should make for more readable epilogues.
        // Also, _rowcount is zero during the main script, and only set
        // to _idx during the epilogue.
        if idx_used {
            globals.set("_rowcount", idx)?;
        }

        info!("Executing epilogue. _idx used: {idx_used}, _rowcount: {idx}");
        let epilogue_value: mlua::Value = match luau.load(&epilogue_script).eval() {
            Ok(computed) => computed,
            Err(e) => {
                log::error!("Epilogue error: Cannot evaluate \"{epilogue_script}\".\n{e}");
                error_result.clone()
            }
        };
        let epilogue_string = match epilogue_value {
            mlua::Value::String(string) => string.to_string_lossy().to_string(),
            mlua::Value::Number(number) => number.to_string(),
            mlua::Value::Integer(number) => number.to_string(),
            mlua::Value::Boolean(boolean) => (if boolean { "true" } else { "false" }).to_string(),
            mlua::Value::Nil => String::new(),
            _ => {
                return fail_clierror!(
                    "Unexpected epilogue value type returned by provided Luau expression. \
                     {epilogue_value:?}"
                );
            }
        };
        winfo!("{epilogue_string}");
    }

    wtr.flush()?;

    if global_error_flag {
        return fail!("Luau errors encountered.");
    }
    Ok(())
}
