use std::fs;

use pyo3::prelude::*;
use pyo3::types::PyDict;

use crate::config::{Config, Delimiter};
use crate::util;
use crate::CliError;
use crate::CliResult;
use indicatif::{ProgressBar, ProgressDrawTarget};
use log::Level::Debug;
use log::{debug, log_enabled};
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

static USAGE: &str = r#"
Create a new column, filter rows or compute aggregations by evaluating a python
expression on every row of a CSV file.

The executed Python has 4 ways to reference cell values (as strings):
  1. Directly by using column name (e.g. amount) as a local variable. If a column
     name has spaces, they are replaced with underscores (e.g. "unit cost" -> unit_cost)
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

  Use Python f-strings to calculate using multiple columns (qty, fruit & "unit cost") 
    and format into a new column 'formatted'
  $ qsv py map formatted "f'{qty} {fruit} cost ${(float(unit_cost) * float(qty)):.2f}'"

  Strip and prefix cell values
  $ qsv py map prefixed "'clean_' + a.strip()"

  Filter some lines based on numerical filtering
  $ qsv py filter "int(a) > 45"

  Load helper file with function to compute Fibonacci sequence of the column "num_col"
  $ qsv py map --helper-file fibonacci.py fib qsv_uh.fibonacci(num_col) data.csv

  NOTE: The py command requires Python 3.8+. If you wish qsv to use a specific 
  Python version other than your system's default - either run it in a python 
  virtual environment, or copy the shared library of the desired Python version 
  in the same directory as qsv (libpython* on Linux/macOS, python*.dll on Windows).

  Also, the following Python modules are automatically loaded and available to the user -
  builtsin, math and random. The user can import additional modules with the --helper option.

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
    -p, --progressbar      Show progress bars. Not valid for stdin.
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
    flag_progressbar: bool,
}

impl From<PyErr> for CliError {
    fn from(err: PyErr) -> CliError {
        CliError::Other(err.to_string())
    }
}

const BATCH_SIZE: usize = 30_000;

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;
    let rconfig = Config::new(&args.arg_input)
        .delimiter(args.flag_delimiter)
        .no_headers(args.flag_no_headers);

    let mut rdr = rconfig.reader()?;
    let mut wtr = Config::new(&args.flag_output).writer()?;

    if log_enabled!(Debug) || args.flag_progressbar {
        _ = Python::with_gil(|py| {
            let msg = format!("Detected python={}", py.version());
            eprintln!("{msg}");
            debug!("{msg}");
        });
    }

    let mut helper_text = String::new();
    if let Some(helper_file) = args.flag_helper {
        helper_text = match fs::read_to_string(helper_file) {
            Ok(helper_file) => helper_file,
            Err(e) => return fail!(format!("Cannot load python file: {e}")),
        }
    }

    let mut headers = rdr.headers()?.clone();
    let headers_len = headers.len();

    if rconfig.no_headers {
        headers = csv::StringRecord::new();

        for i in 0..headers_len {
            headers.push_field(&i.to_string());
        }
    } else {
        if !args.cmd_filter {
            let new_column = args
                .arg_new_column
                .as_ref()
                .ok_or("Specify new column name")?;
            headers.push_field(new_column);
        }

        wtr.write_record(&headers)?;
    }

    // prep progress bar
    let show_progress =
        (args.flag_progressbar || std::env::var("QSV_PROGRESSBAR").is_ok()) && !rconfig.is_stdin();
    let progress = ProgressBar::with_draw_target(None, ProgressDrawTarget::stderr_with_hz(5));
    if show_progress {
        util::prep_progress(&progress, util::count_rows(&rconfig)?);
    } else {
        progress.set_draw_target(ProgressDrawTarget::hidden());
    }

    // ensure col/header names are valid and safe python variables
    let header_vec = util::safe_header_names(&headers, true);

    // amortize memory allocation by reusing record
    #[allow(unused_assignments)]
    let mut batch_record = csv::StringRecord::new();

    // reuse batch buffers
    let mut batch = Vec::with_capacity(BATCH_SIZE);

    // main loop to read CSV and construct batches.
    // we batch python operations so that the GILPool does not get very large
    // as we release the pool after each batch
    // loop exits when batch is empty.
    loop {
        for _ in 0..BATCH_SIZE {
            match rdr.read_record(&mut batch_record) {
                Ok(has_data) => {
                    if has_data {
                        batch.push(batch_record.clone());
                    } else {
                        // nothing else to add to batch
                        break;
                    }
                }
                Err(e) => {
                    return Err(CliError::Other(format!("Error reading file: {e}")));
                }
            }
        }

        if batch.is_empty() {
            // break out of infinite loop when at EOF
            break;
        }

        _ = Python::with_gil(|py| -> PyResult<()> {
            {
                let curr_batch = batch.clone();
                let helpers = PyModule::from_code(py, HELPERS, "qsv_helpers.py", "qsv_helpers")?;
                let globals = PyDict::new(py);
                let locals = PyDict::new(py);

                let user_helpers =
                    match PyModule::from_code(py, &helper_text, "qsv_user_helpers.py", "qsv_uh") {
                        Ok(helper_code) => helper_code,
                        Err(e) => {
                            eprintln!("Cannot compile user module \"{helper_text}\".\n{e}");
                            panic!();
                        }
                    };
                globals.set_item("qsv_uh", user_helpers)?;

                // Global imports
                let builtins = PyModule::import(py, "builtins")?;
                let math_module = PyModule::import(py, "math")?;
                let random_module = PyModule::import(py, "random")?;

                globals.set_item("__builtins__", builtins)?;
                globals.set_item("math", math_module)?;
                globals.set_item("random", random_module)?;

                let py_row = helpers
                    .getattr("QSVRow")?
                    .call1((headers.iter().collect::<Vec<&str>>(),))?;

                locals.set_item("row", py_row)?;

                for mut record in curr_batch {
                    if show_progress {
                        progress.inc(1);
                    }

                    // Initializing locals
                    let mut row_data: Vec<&str> = Vec::with_capacity(headers_len);

                    for (i, key) in header_vec.iter().enumerate().take(headers_len) {
                        let cell_value = record.get(i).unwrap_or_default();
                        locals.set_item(key, cell_value).expect("cannot set_item");
                        row_data.push(cell_value);
                    }

                    py_row
                        .call_method1("_update_underlying_data", (row_data,))
                        .expect("cannot call method1");

                    let mut result_err = false;
                    let result = py
                        .eval(&args.arg_script, Some(globals), Some(locals))
                        .map_err(|e| {
                            result_err = true;
                            e.print_and_set_sys_last_vars(py);
                            "Evaluation of given expression failed with the above error!"
                        })
                        .expect("cannot eval code");

                    if result_err && log_enabled!(Debug) {
                        debug!("{result:?}");
                    }

                    if args.cmd_map {
                        let result = helpers
                            .getattr("cast_as_string")
                            .unwrap()
                            .call1((result,))
                            .expect("cannot get result");
                        let value: String = result.extract().unwrap();

                        record.push_field(&value);
                        wtr.write_record(&record).expect("cannot write record");
                    } else if args.cmd_filter {
                        let result = helpers
                            .getattr("cast_as_bool")
                            .unwrap()
                            .call1((result,))
                            .expect("cannot get result");
                        let value: bool = result.extract().unwrap();

                        if value {
                            wtr.write_record(&record).expect("cannot write record");
                        }
                    }
                }

                Ok(())
            }
        });

        batch.clear();
    } // end loop

    if show_progress {
        util::finish_progress(&progress);
    }

    Ok(wtr.flush()?)
}
