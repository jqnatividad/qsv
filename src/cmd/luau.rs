static USAGE: &str = r#"
Create a new computed column, filter rows or compute aggregations by executing a
Luau script for every row of a CSV file.

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

  Filter some rows based on numerical filtering
  $ qsv luau filter "tonumber(a) > 45"
  $ qsv luau filter "tonumber(a) >= tonumber(b)"

  Typing long scripts on the command line gets tiresome rather quickly, so use the
  "file:" prefix to read non-trivial scripts from the filesystem.
  $ qsv luau map Type -P "file:init.luau" -x "file:debitcredit.luau" -E "file:end.luau"

The main-script is evaluated on a per row basis.
With "luau map", if the main-script is invalid for a row, "<ERROR>" is returned for that row.
With "luau filter", if the main-script is invalid for a row, that row is not filtered.

If any row has an invalid result, an exitcode of 1 is returned along with an error count to stderr.

There are also special variables - "_idx" that is zero during the prologue, and set to the current 
row number during the main script; and "_rowcount" which is zero during the prologue and the main script,
and set to the rowcount during the epilogue.

Luau's standard library is relatively minimal (https://luau-lang.org/library).
That's why qsv preloads the LuaDate library as date manipulation is a common data-wrangling task.
See https://tieske.github.io/date/#date-id96473 for info on how to use the LuaDate library.

Furthermore, the user can load additional libraries from the LUAU_PATH using luau's "require" function.
See http://lua-users.org/wiki/LibrariesAndBindings for a list of other libraries.

With the judicious use of "require", the prologue & the "_idx"/"_rowcount" variables, one can create
variables/tables/arrays that can be used for complex aggregation operations in the epilogue.

TIP: When developing luau scripts, be sure to set QSV_LOG_LEVEL=debug so you can see the detailed Luau
errors in the logfile. Set QSV_LOG_LEVEL=trace if you want to see the row/global values as well.

For more detailed examples, see https://github.com/jqnatividad/qsv/blob/master/tests/test_luau.rs.

Usage:
    qsv luau map [options] -n <main-script> [<input>]
    qsv luau map [options] <new-column> <main-script> [<input>]
    qsv luau filter [options] <main-script> [<input>]
    qsv luau map --help
    qsv luau filter --help
    qsv luau --help

All <script> arguments/options can either be the Luau code, or if it starts with "file:",
the filepath from which to load the script.

luau options:
    -x, --exec               exec[ute] Luau script, instead of the default eval[uate].
                             eval (default) expects just a single Luau expression,
                             while exec expects one or more statements, allowing
                             full-fledged Luau programs. This only applies to the main-script
                             argument, not the prologue & epilogue scripts.
    -g, --no-globals         Don't create Luau global variables for each column, only col.
                             Useful when some column names mask standard Luau globals.
                             Note: access to Luau globals thru _G remains even without -g.
    -P, --prologue <script>  Luau script/file to execute BEFORE processing the CSV with the main-script.
                             The variables _idx and _rowcount are set to zero before invoking
                             the prologue script.
                             Typically used to initialize global variables.
    -E, --epilogue <script>  Luau script/file to execute AFTER processing the CSV with the main-script.
                             Both _idx and _rowcount variables are set to the rowcount before invoking
                             the epilogue script.
                             Typically used for aggregations.
                             The output of the epilogue is sent to stderr.
    --luau-path <pattern>    The LUAU_PATH pattern to use from which the scripts 
                             can "require" lua/luau library files from.
                             See https://www.lua.org/pil/8.1.html
                             [default: ?;?.luau;?.lua]

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

use std::{env, fs};

#[cfg(any(feature = "full", feature = "lite"))]
use indicatif::{ProgressBar, ProgressDrawTarget};
use log::{debug, info, log_enabled};
use mlua::{Lua, Value};
use serde::Deserialize;
use tempfile;

use crate::{
    config::{Config, Delimiter},
    util, CliError, CliResult,
};

#[derive(Deserialize)]
struct Args {
    cmd_map:          bool,
    cmd_filter:       bool,
    arg_new_column:   Option<String>,
    arg_main_script:  String,
    arg_input:        Option<String>,
    flag_exec:        bool,
    flag_no_globals:  bool,
    flag_prologue:    Option<String>,
    flag_epilogue:    Option<String>,
    flag_luau_path:   String,
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

    // we initialize idx_used with a log_enabled debug check
    // coz if log debug is on, we use idx to track record/row number
    // in the log error messages
    let mut idx_used: bool = log_enabled!(log::Level::Debug);
    let trace_on: bool = log_enabled!(log::Level::Trace);
    let mut idx = 0_usize;

    // setup LUAU_PATH; create a temporary directory and add it to LUAU_PATH and copy date.lua
    // there so qsv luau users can just `local date = require "date"` in their scripts,
    // as well as load other lua/luau libraries as needed.
    let luadate_library = include_bytes!("../../resources/luau/vendor/luadate/date.lua");
    let Ok(temp_dir) = tempfile::tempdir() else {
        return fail!("Cannot create temporary directory to copy luadate library to.");
    };
    let luadate_path = temp_dir.path().join("date.lua");
    fs::write(luadate_path.clone(), luadate_library)?;
    let mut luau_path = args.flag_luau_path;
    luau_path.push_str(&format!(";{}", luadate_path.as_os_str().to_string_lossy()));
    env::set_var("LUAU_PATH", luau_path.clone());
    info!(r#"set LUAU_PATH to "{luau_path}""#);

    // check if an epilogue was specified. We check it first for two reasons:
    // 1) so we don't run the main script only to err out on an invalid epilogue file not found.
    // 2) to see if _idx or _rowcount is used, even if they're not used in the prologue,
    //    so we can init the _idx and _rowcount global vars in the beginning, if the epilogue
    //    referred to the special vars & they're not referenced in the prologue & main scripts.
    let epilogue_script = if let Some(ref epilogue) = args.flag_epilogue {
        if let Some(epilogue_filepath) = epilogue.strip_prefix("file:") {
            match fs::read_to_string(epilogue_filepath) {
                Ok(epilogue) => {
                    // check if the epilogue uses _idx or _rowcount
                    idx_used =
                        idx_used || epilogue.contains("_idx") || epilogue.contains("_rowcount");
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

    // prepare the Luau compiler, so we can compile the scripts into bytecode
    // see https://docs.rs/mlua/latest/mlua/struct.Compiler.html for more info
    let luau_compiler = if log_enabled!(log::Level::Debug) {
        // debugging is on, set more debugging friendly compiler settings
        // so we can see more error details in the logfile
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
    // set it as the default compiler
    luau.set_compiler(luau_compiler.clone());

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
            // we set _idx and _rowcount here just in case they're
            // used in the prologue script
            globals.set("_idx", 0)?;
            globals.set("_rowcount", 0)?;
        }

        info!("Compiling and executing prologue. _idx used: {idx_used}");
        let prologue_bytecode = luau_compiler.compile(&prologue_script);
        if let Err(e) = luau
            .load(&prologue_bytecode)
            .set_mode(mlua::ChunkMode::Binary)
            .exec()
        {
            if trace_on {
                log::trace!("prologue globals: {globals:?}");
            }
            return fail_clierror!("Prologue error: Failed to execute \"{prologue_script}\".\n{e}");
        }
        info!("Prologue executed.");
    }

    let luau_script = if let Some(script_filepath) = args.arg_main_script.strip_prefix("file:") {
        match fs::read_to_string(script_filepath) {
            Ok(file_contents) => file_contents,
            Err(e) => return fail_clierror!("Cannot load Luau file: {e}"),
        }
    } else {
        args.arg_main_script
    };

    idx_used = idx_used || luau_script.contains("_idx") || luau_script.contains("_rowcount");

    let mut luau_main_script = if args.flag_exec {
        String::new()
    } else {
        String::from("return ")
    };

    luau_main_script.push_str(&luau_script);
    debug!("Luau main script: {luau_main_script:?}");

    // prep progress bar
    #[cfg(any(feature = "full", feature = "lite"))]
    let show_progress =
        (args.flag_progressbar || std::env::var("QSV_PROGRESSBAR").is_ok()) && !rconfig.is_stdin();

    #[cfg(any(feature = "full", feature = "lite"))]
    let progress = ProgressBar::with_draw_target(None, ProgressDrawTarget::stderr_with_hz(5));

    #[cfg(any(feature = "full", feature = "lite"))]
    if show_progress {
        util::prep_progress(&progress, util::count_rows(&rconfig)?);
    } else {
        progress.set_draw_target(ProgressDrawTarget::hidden());
    }

    let error_result: Value = luau.load("return \"<ERROR>\";").eval()?;
    let mut error_flag;

    // we init/reset _idx and _rowcount right before the main loop
    if idx_used {
        globals.set("_idx", 0)?;
        globals.set("_rowcount", 0)?;
    }

    // pre-compile main script into bytecode
    let main_bytecode = luau_compiler.compile(&luau_main_script);

    let mut record = csv::StringRecord::new();
    let mut error_count = 0_usize;
    let mut trace_col_values = String::new();

    while rdr.read_record(&mut record)? {
        #[cfg(any(feature = "full", feature = "lite"))]
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
            let col =
                luau.create_table_with_capacity(record.len().try_into().unwrap_or_default(), 1)?;

            for (i, v) in record.iter().enumerate() {
                col.set(i + 1, v)?;
            }
            if !rconfig.no_headers {
                for (h, v) in headers.iter().zip(record.iter()) {
                    col.set(h, v)?;
                }
            }
            if trace_on {
                trace_col_values = format!("{:?}", col.clone());
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
        let computed_value: Value = match luau
            .load(&main_bytecode)
            .set_mode(mlua::ChunkMode::Binary)
            .eval()
        {
            Ok(computed) => computed,
            Err(e) => {
                error_flag = true;
                error_count += 1;
                let err_msg = if idx_used {
                    if trace_on {
                        log::trace!("current row({idx}): {trace_col_values}");
                        log::trace!("current globals({idx}): {globals:?}");
                    }
                    format!("_idx: {idx} error({error_count}): {e:?}")
                } else {
                    format!("error({error_count}): {e:?}")
                };
                log::error!("{err_msg}");
                error_result.clone()
            }
        };

        if args.cmd_map {
            match computed_value {
                Value::String(string) => {
                    record.push_field(&string.to_string_lossy());
                }
                Value::Number(number) => {
                    let mut buffer = ryu::Buffer::new();
                    record.push_field(buffer.format(number));
                }
                Value::Integer(number) => {
                    let mut buffer = itoa::Buffer::new();
                    record.push_field(buffer.format(number));
                }
                Value::Boolean(boolean) => {
                    record.push_field(if boolean { "true" } else { "false" });
                }
                Value::Nil => {
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
            let must_keep_row = if error_flag {
                true
            } else {
                match computed_value {
                    Value::String(strval) => !strval.to_string_lossy().is_empty(),
                    Value::Boolean(boolean) => boolean,
                    Value::Nil => false,
                    Value::Integer(intval) => intval != 0,
                    // we compare to f64::EPSILON as float comparison to zero
                    // unlike int, where we can say intval != 0, we cannot do fltval !=0
                    // https://doc.rust-lang.org/std/primitive.f64.html#associatedconstant.EPSILON
                    Value::Number(fltval) => (fltval).abs() > f64::EPSILON,
                    _ => true,
                }
            };

            if must_keep_row {
                wtr.write_record(&record)?;
            }
        }
    }

    #[cfg(any(feature = "full", feature = "lite"))]
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

        info!("Compiling and executing epilogue. _idx used: {idx_used}, _rowcount: {idx}");
        let epilogue_bytecode = luau_compiler.compile(&epilogue_script);
        let epilogue_value: Value = match luau
            .load(&epilogue_bytecode)
            .set_mode(mlua::ChunkMode::Binary)
            .eval()
        {
            Ok(computed) => computed,
            Err(e) => {
                log::error!("Epilogue error: Cannot evaluate \"{epilogue_script}\".\n{e}");
                if trace_on {
                    log::trace!("epilogue globals: {globals:?}");
                }
                error_result.clone()
            }
        };
        let epilogue_string = match epilogue_value {
            Value::String(string) => string.to_string_lossy().to_string(),
            Value::Number(number) => number.to_string(),
            Value::Integer(number) => number.to_string(),
            Value::Boolean(boolean) => (if boolean { "true" } else { "false" }).to_string(),
            Value::Nil => String::new(),
            _ => {
                return fail_clierror!(
                    "Unexpected epilogue value type returned by provided Luau expression. \
                     {epilogue_value:?}"
                );
            }
        };
        winfo!("{epilogue_string}");
    }
    if trace_on {
        log::trace!("ending globals: {globals:?}");
    }

    wtr.flush()?;

    if error_count > 0 {
        return fail_clierror!("Luau errors encountered: {error_count}");
    }
    Ok(())
}
