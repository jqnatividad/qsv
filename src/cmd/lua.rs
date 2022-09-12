static USAGE: &str = r#"
Create a new column, filter rows or compute aggregations by executing a LuaJIT
script of every line of a CSV file.

The executed LuaJIT has 3 ways to reference row columns (as strings):
  1. Directly by using column name (e.g. Amount), can be disabled with -g
  2. Indexing col variable by column name: col.Amount or col["Total Balance"]
  3. Indexing col variable by column 1-based index: col[1], col[2], etc.

Of course, if your input has no headers, then 3. will be the only available
option.

Some usage examples:

  Sum numeric columns 'a' and 'b' and call new column 'c'
  $ qsv lua map c "a + b"
  $ qsv lua map c "col.a + col['b']"
  $ qsv lua map c "col[1] + col[2]"

  There is some magic in the previous example as 'a' and 'b' are passed in
  as strings (not numbers), but LuaJIT still manages to add them up.
  A more explicit way of doing it, is by using tonumber
  $ qsv lua map c "tonumber(a) + tonumber(b)"

  Add running total column for Amount
  $ qsv lua map Total -x "tot = (tot or 0) + Amount; return tot"

  Add running total column for Amount when previous balance was 900
  $ qsv lua map Total -x "tot = (tot or 900) + Amount; return tot"

  Convert Amount to always-positive AbsAmount and Type (debit/credit) columns
  $ qsv lua map Type -x \
        "if tonumber(Amount) < 0 then return 'debit' else return 'credit' end" | \
    qsv lua map AbsAmount "math.abs(tonumber(Amount))"

  Filter some lines based on numerical filtering
  $ qsv lua filter "tonumber(a) > 45"
  $ qsv lua filter "tonumber(a) >= tonumber(b)"

  Typing long scripts at command line gets tiresome rather quickly,
  so -f should be used for non-trivial scripts to read them from a file
  $ qsv lua map Type -x -f debitcredit.lua

Usage:
    qsv lua map [options] -n <script> [<input>]
    qsv lua map [options] <new-column> <script> [<input>]
    qsv lua filter [options] <script> [<input>]
    qsv lua map --help
    qsv lua filter --help
    qsv lua --help

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

use crate::config::{Config, Delimiter};
use crate::util;
use crate::CliError;
use crate::CliResult;
use indicatif::{ProgressBar, ProgressDrawTarget};
use log::debug;
use mlua::Lua;
use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
struct Args {
    cmd_map: bool,
    cmd_filter: bool,
    arg_new_column: Option<String>,
    arg_script: String,
    arg_input: Option<String>,
    flag_exec: bool,
    flag_script_file: bool,
    flag_no_globals: bool,
    flag_output: Option<String>,
    flag_no_headers: bool,
    flag_delimiter: Option<Delimiter>,
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
            Err(e) => return fail_format!("Cannot load LuaJIT file: {e}"),
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

        let computed_value: mlua::Value = match lua.load(&lua_program).eval() {
            Ok(computed) => computed,
            Err(e) => return fail_format!("Cannot evaluate \"{lua_program}\".\n{e}"),
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
                    return fail_format!("Unexpected value type returned by provided LuaJIT expression. {computed_value:?}");
                }
            }

            wtr.write_record(&record)?;
        } else if args.cmd_filter {
            let must_keep_line: bool = match computed_value {
                mlua::Value::String(strval) => !strval.to_string_lossy().is_empty(),
                mlua::Value::Boolean(boolean) => boolean,
                mlua::Value::Nil => false,
                mlua::Value::Integer(intval) => intval != 0,
                mlua::Value::Number(fltval) => {
                    let mut buffer = ryu::Buffer::new();
                    buffer.format(fltval) != "0.0"
                }
                _ => false,
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
