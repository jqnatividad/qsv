static USAGE: &str = r#"
Create a new column, filter rows or compute aggregations by executing a LuaJIT
script of every line of a CSV file.

LuaJIT is a Just-In-Time (JIT) for the Lua 5.1 language and is much faster than Lua
(up to 134x faster - see https://luajit.org/performance.html).

The executed LuaJIT has 3 ways to reference row columns (as strings):
  1. Directly by using column name (e.g. Amount), can be disabled with -g
  2. Indexing col variable by column name: col.Amount or col["Total Balance"]
  3. Indexing col variable by column 1-based index: col[1], col[2], etc.

Of course, if your input has no headers, then 3. will be the only available
option.

Some usage examples:

  Sum numeric columns 'a' and 'b' and call new column 'c'
  $ qsv luajit map c "a + b"
  $ qsv luajit map c "col.a + col['b']"
  $ qsv luajit map c "col[1] + col[2]"

  There is some magic in the previous example as 'a' and 'b' are passed in
  as strings (not numbers), but LuaJIT still manages to add them up.
  A more explicit way of doing it, is by using tonumber
  $ qsv luajit map c "tonumber(a) + tonumber(b)"

  Add running total column for Amount
  $ qsv luajit map Total -x "tot = (tot or 0) + Amount; return tot"

  Add running total column for Amount when previous balance was 900
  $ qsv luajit map Total -x "tot = (tot or 900) + Amount; return tot"

  Convert Amount to always-positive AbsAmount and Type (debit/credit) columns
  $ qsv luajit map Type -x \
        "if tonumber(Amount) < 0 then return 'debit' else return 'credit' end" | \
    qsv luajit map AbsAmount "math.abs(tonumber(Amount))"

  Filter some lines based on numerical filtering
  $ qsv luajit filter "tonumber(a) > 45"
  $ qsv luajit filter "tonumber(a) >= tonumber(b)"

  Typing long scripts at command line gets tiresome rather quickly,
  so -f should be used for non-trivial scripts to read them from a file
  $ qsv luajit map Type -x -f debitcredit.lua

  With "luajit map", if a LuaJIT script is invalid, "<ERROR>" is returned.
  With "luajit filter", if a LuaJIT script is invalid, no filtering is done.

Usage:
    qsv luajit map [options] -n <script> [<input>]
    qsv luajit map [options] <new-column> <script> [<input>]
    qsv luajit filter [options] <script> [<input>]
    qsv luajit map --help
    qsv luajit filter --help
    qsv luajit --help

lua options:
    -x, --exec         exec[ute] LuaJIT script, instead of the default eval[uate].
                       eval (default) expects just a single LuaJIT expression,
                       while exec expects one or more statements, allowing
                       full-fledged LuaJIT programs.
    -f, --script-file  <script> is a file name containing LuaJIT script.
                       By default (no -f) <script> is the script text.
    -g, --no-globals   Don't create LuaJIT global variables for each column, only col.
                       Useful when some column names mask standard LuaJIT globals.
                       Note: access to LuaJIT globals thru _G remains even without -g.

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
use log::debug;
use mluajit::Lua;
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
    flag_script_file: bool,
    flag_no_globals:  bool,
    flag_output:      Option<String>,
    flag_no_headers:  bool,
    flag_delimiter:   Option<Delimiter>,
    flag_progressbar: bool,
}

impl From<mluajit::Error> for CliError {
    fn from(err: mluajit::Error) -> CliError {
        CliError::Other(err.to_string())
    }
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;
    let rconfig = Config::new(&args.arg_input)
        .delimiter(args.flag_delimiter)
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

    let lua = Lua::new();
    let globals = lua.globals();
    globals.set("cols", "{}")?;

    let lua_script = if args.flag_script_file {
        match fs::read_to_string(&args.arg_script) {
            Ok(script_file) => script_file,
            Err(e) => return fail_clierror!("Cannot load LuaJIT file: {e}"),
        }
    } else {
        args.arg_script
    };

    let mut lua_program = if args.flag_exec {
        String::new()
    } else {
        String::from("return ")
    };

    lua_program.push_str(&lua_script);
    debug!("LuaJIT program: {lua_program:?}");

    // prep progress bar
    let show_progress =
        (args.flag_progressbar || std::env::var("QSV_PROGRESSBAR").is_ok()) && !rconfig.is_stdin();

    let progress = ProgressBar::with_draw_target(None, ProgressDrawTarget::stderr_with_hz(5));
    if show_progress {
        util::prep_progress(&progress, util::count_rows(&rconfig)?);
    } else {
        progress.set_draw_target(ProgressDrawTarget::hidden());
    }

    let error_result: mluajit::Value = lua.load("return \"<ERROR>\";").eval().unwrap();
    let mut error_flag;

    let mut record = csv::StringRecord::new();

    while rdr.read_record(&mut record)? {
        if show_progress {
            progress.inc(1);
        }

        // Updating col
        {
            let col = lua.create_table()?;

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
        let computed_value: mluajit::Value = match lua.load(&lua_program).eval() {
            Ok(computed) => computed,
            Err(e) => {
                log::error!("Cannot evaluate \"{lua_program}\".\n{e}");
                error_flag = true;
                error_result.clone()
            }
        };

        if args.cmd_map {
            match computed_value {
                mluajit::Value::String(string) => {
                    record.push_field(&string.to_string_lossy());
                }
                mluajit::Value::Number(number) => {
                    let mut buffer = ryu::Buffer::new();
                    record.push_field(buffer.format(number));
                }
                mluajit::Value::Integer(number) => {
                    let mut buffer = itoa::Buffer::new();
                    record.push_field(buffer.format(number));
                }
                mluajit::Value::Boolean(boolean) => {
                    record.push_field(if boolean { "true" } else { "false" });
                }
                mluajit::Value::Nil => {
                    record.push_field("");
                }
                _ => {
                    return fail_clierror!("Unexpected value type returned by provided LuaJIT expression. {computed_value:?}");
                }
            }

            wtr.write_record(&record)?;
        } else if args.cmd_filter {
            let must_keep_line = if error_flag {
                true
            } else {
                match computed_value {
                    mluajit::Value::String(strval) => !strval.to_string_lossy().is_empty(),
                    mluajit::Value::Boolean(boolean) => boolean,
                    mluajit::Value::Nil => false,
                    mluajit::Value::Integer(intval) => intval != 0,
                    mluajit::Value::Number(fltval) => {
                        let mut buffer = ryu::Buffer::new();
                        buffer.format(fltval) != "0.0"
                    }
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

    Ok(wtr.flush()?)
}
