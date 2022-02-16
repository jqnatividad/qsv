use std::fs;

use pyo3::prelude::*;
use pyo3::types::PyDict;

use crate::config::{Config, Delimiter};
use crate::util;
use crate::CliError;
use crate::CliResult;
use indicatif::{ProgressBar, ProgressDrawTarget};
use serde::Deserialize;

const HELPERS: &str = r#"
def cast_as_string(value):
    if isinstance(value, str):
        return value
    return str(value)

def cast_as_bool(value):
    return bool(value)

class QSVRow(object):
    def __init__(self, headers):
        self.__data = None
        self.__headers = headers
        self.__mapping = {h: i for i, h in enumerate(headers)}

    def _update_underlying_data(self, row_data):
        self.__data = row_data

    def __getitem__(self, key):
        if isinstance(key, int):
            return self.__data[key]

        return self.__data[self.__mapping[key]]

    def __getattr__(self, key):
        return self.__data[self.__mapping[key]]
"#;

// fn template_execution(statements: &str) -> String {
//     format!("def __run__():\n{}\n__return_value__ = __run__()", textwrap::indent(statements, "  "))
// }

// TODO: options for boolean return coercion

static USAGE: &str = r#"
Create a new column, filter rows or compute aggregations by evaluating a python
expression on every row of a CSV file.

The executed Python has 4 ways to reference cell values (as strings):
  1. Directly by using column name (e.g. amount) as a local variable
  2. Indexing cell value by column name as an attribute: row.amount
  3. Indexing cell value by column name as a key: row["amount"]
  4. Indexing cell value by column position: row[0]

Of course, if your input has no headers, then 4. will be the only available
option.

Some usage examples:

  Sum numeric columns 'a' and 'b' and call new column 'c'
  $ qsv py map c "int(a) + int(b)"
  $ qsv py map c "int(col.a) + int(col['b'])"
  $ qsv py map c "int(col[0]) + int(col[1])"

  Use Python f-strings to calculate using multiple columns (qty, fruit & unitcost) 
    and format into a new column 'formatted'
  $ qsv py map formatted "f'{qty} {fruit} cost ${(float(unitcost) * float(qty)):.2f}'"

  Strip and prefix cell values
  $ qsv py map prefixed "'clean_' + a.strip()"

  Filter some lines based on numerical filtering
  $ qsv py filter "int(a) > 45"

  Load a helper file with function to compute the Fibonacci sequence of the column "num_col"
  $ qsv py map --helper-file fibonacci.py fib qsv_uh.fibonacci(num_col) data.csv

Usage:
    qsv py map [options] -n <script> [<input>]
    qsv py map [options] <new-column> <script> [<input>]
    qsv py map --helper <file> [options] <new-column> <script> [<input>]
    qsv py filter [options] <script> [<input>]
    qsv py map --help
    qsv py filter --help
    qsv py --help

py options:
    -f, --helper <file>    File containing Python code that's loaded
                           into the qsv_uh Python module.

Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
    -n, --no-headers       When set, the first row will not be interpreted
                           as headers. Namely, it will be sorted with the rest
                           of the rows. Otherwise, the first row will always
                           appear as the header row in the output.
    -d, --delimiter <arg>  The field delimiter for reading CSV data.
                           Must be a single character. (default: ,)
    -q, --quiet            Do not display progress bar.
"#;

#[derive(Deserialize)]
struct Args {
    cmd_map: bool,
    cmd_filter: bool,
    arg_new_column: Option<String>,
    arg_script: String,
    flag_helper: Option<String>,
    arg_input: Option<String>,
    flag_output: Option<String>,
    flag_no_headers: bool,
    flag_delimiter: Option<Delimiter>,
    flag_quiet: bool,
}

impl From<PyErr> for CliError {
    fn from(err: PyErr) -> CliError {
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

    pyo3::prepare_freethreaded_python();
    let gil = Python::acquire_gil();
    let py = gil.python();

    let helpers = PyModule::from_code(py, HELPERS, "qsv_helpers.py", "qsv_helpers")?;
    let globals = PyDict::new(py);
    let locals = PyDict::new(py);

    let mut helper_text = String::new();
    if let Some(helper_file) = args.flag_helper {
        helper_text = fs::read_to_string(helper_file)?;
    }
    let user_helpers = PyModule::from_code(py, &helper_text, "qsv_user_helpers.py", "qsv_uh")?;
    globals.set_item("qsv_uh", user_helpers)?;

    // Global imports
    let builtins = PyModule::import(py, "builtins")?;
    let math_module = PyModule::import(py, "math")?;

    globals.set_item("__builtins__", builtins)?;
    globals.set_item("math", math_module)?;

    let mut headers = rdr.headers()?.clone();

    let headers_len = headers.len();

    let py_row = helpers
        .getattr("QSVRow")?
        .call1((headers.iter().collect::<Vec<&str>>(),))?;

    locals.set_item("row", py_row)?;

    if !rconfig.no_headers {
        if !args.cmd_filter {
            let new_column = args
                .arg_new_column
                .as_ref()
                .ok_or("Specify new column name")?;
            headers.push_field(new_column);
        }

        wtr.write_record(&headers)?;
    } else {
        headers = csv::StringRecord::new();

        for i in 0..headers_len {
            headers.push_field(&i.to_string());
        }
    }

    let progress = ProgressBar::new(0);
    if !args.flag_quiet {
        let record_count = util::count_rows(&rconfig);
        util::prep_progress(&progress, record_count);
    } else {
        progress.set_draw_target(ProgressDrawTarget::hidden());
    }

    let mut record = csv::StringRecord::new();

    while rdr.read_record(&mut record)? {
        if !args.flag_quiet {
            progress.inc(1);
        }

        // Initializing locals
        let mut row_data: Vec<&str> = Vec::with_capacity(headers_len);

        for (i, h) in headers.iter().take(headers_len).enumerate() {
            let cell_value = record.get(i).unwrap();
            locals.set_item(h, cell_value)?;
            row_data.push(cell_value);
        }

        py_row.call_method1("_update_underlying_data", (row_data,))?;

        let result = py
            .eval(&args.arg_script, Some(globals), Some(locals))
            .map_err(|e| {
                e.print_and_set_sys_last_vars(py);
                "Evaluation of given expression failed with the above error!"
            })?;

        if args.cmd_map {
            let result = helpers.getattr("cast_as_string")?.call1((result,))?;
            let value: String = result.extract()?;

            record.push_field(&value);
            wtr.write_record(&record)?;
        } else if args.cmd_filter {
            let result = helpers.getattr("cast_as_bool")?.call1((result,))?;
            let value: bool = result.extract()?;

            if value {
                wtr.write_record(&record)?;
            }
        }
    }
    if !args.flag_quiet {
        util::finish_progress(&progress);
    }

    Ok(wtr.flush()?)
}
