static USAGE: &str = r#"
Create a new computed column or filter rows by evaluating a python expression on 
every row of a CSV file.

The executed Python has 4 ways to reference cell values (as strings):
  1. Directly by using column name (e.g. amount) as a local variable. If a column
     name has spaces and other special characters, they are replaced with underscores
     (e.g. "unit cost" -> unit_cost, "test-units/sec" -> test_units_sec)
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
  $ qsv py map formatted 'f"{qty} {fruit} cost ${(float(unit_cost) * float(qty)):.2f}"'

  You can even have conditionals in your f-string:
  $ qsv py map formatted \
   'f"""{qty} {fruit} cost ${(float(unit_cost) * float(qty)):.2f}. Its quite {"cheap" if ((float(unit_cost) * float(qty)) < 20.0) else "expensive"}!"""'

  Note how we needed to use triple double quotes for the f-string, so we can use the literals
  "cheap" and "expensive" in the f-string expression.

  Strip and prefix cell values
  $ qsv py map prefixed "'clean_' + a.strip()"

  Filter some lines based on numerical filtering
  $ qsv py filter "int(a) > 45"

  Load helper file with function to compute Fibonacci sequence of the column "num_col"
  $ qsv py map --helper fibonacci.py fib qsv_uh.fibonacci(num_col) data.csv

  Below is a detailed example of the --helper option:

  Use case:
  Need to calculate checksum/md5sum of some columns. First column (c1) is "id", and do md5sum of
  the rest of the columns (c2, c3 and c4).

  Given test.csv:
    c1,c2,c3,c4
    1,a2,a3,a4
    2,b2,b3,b4
    3,c2,c3,c4

  and hashhelper.py:
    import hashlib
    def md5hash (*args):
        s = ",".join(args)
        return(hashlib.md5(s.encode('utf-8')).hexdigest())

  with the following command:
  $ qsv py map --helper hashhelper.py hashcol 'qsv_uh.md5hash(c2,c3,c4)' test.csv

  we get:
  c1,c2,c3,c4,hashcol
  1,a2,a3,a4,cb675342ed940908eef0844d17c35fab
  2,b2,b3,b4,7d594b33f82bdcbc1cfa6f924a84c4cd
  3,c2,c3,c4,6eabbfdbfd9ab6ae7737fb2b82f6a1af
  
  Note that qsv with the `python` feature enabled will panic on startup even if you're not
  using the `py` command if Python's shared libraries are not found.
  
  Also, the following Python modules are automatically loaded and available to the user -
  builtsin, math, random & datetime. The user can import additional modules with the --helper option,
  with the ability to use any python module that's installed in the current python virtualenv. 

  The python expression is evaluated on a per record basis.
  With "py map", if the expression is invalid for a record, "<ERROR>" is returned for that record.
  With "py filter", if the expression is invalid for a record, that record is not filtered.

  If any record has an invalid result, an exitcode of 1 is returned and an error count is logged.

For more extensive examples, see https://github.com/jqnatividad/qsv/blob/master/tests/test_py.rs.

Usage:
    qsv py map [options] -n <script> [<input>]
    qsv py map [options] <new-column> <script> [<input>]
    qsv py map --helper <file> [options] <new-column> <script> [<input>]
    qsv py filter [options] <script> [<input>]
    qsv py map --help
    qsv py filter --help
    qsv py --help

py options:
    -f, --helper <file>    File containing Python code that's loaded into the 
                           qsv_uh Python module. Functions with a return statement
                           in the file can be called with the prefix "qsv_uh".
                           The returned value is used in the map or filter operation.

    -b, --batch <size>     The number of rows per batch to process before
                           releasing memory and acquiring a new GILpool.
                           Set to 0 to process the entire file in one batch.
                           See https://pyo3.rs/v0.22.0/memory.html#gil-bound-memory
                           for more info. [default: 30000]

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
use log::{error, log_enabled, Level::Debug};
use pyo3::{
    intern,
    prelude::*,
    types::{PyDict, PyModule},
    PyErr, PyResult, Python,
};
use serde::Deserialize;

use crate::{
    config::{Config, Delimiter},
    util, CliError, CliResult,
};

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

#[derive(Deserialize)]
struct Args {
    cmd_map:          bool,
    cmd_filter:       bool,
    arg_new_column:   Option<String>,
    arg_script:       String,
    flag_batch:       usize,
    flag_helper:      Option<String>,
    arg_input:        Option<String>,
    flag_output:      Option<String>,
    flag_no_headers:  bool,
    flag_delimiter:   Option<Delimiter>,
    flag_progressbar: bool,
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

    if log_enabled!(Debug) {
        Python::with_gil(|py| {
            let msg = format!("Detected python={}", py.version());
            winfo!("{msg}");
        });
    }

    let mut helper_text = String::new();
    if let Some(helper_file) = args.flag_helper {
        helper_text = match fs::read_to_string(helper_file) {
            Ok(helper_file) => helper_file,
            Err(e) => return fail_clierror!("Cannot load python file: {e}"),
        }
    }

    let mut headers = rdr.headers()?.clone();
    let headers_len = headers.len();

    if rconfig.no_headers {
        headers = csv::StringRecord::new();

        let mut buffer = itoa::Buffer::new();
        for i in 0..headers_len {
            headers.push_field(buffer.format(i));
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
        (args.flag_progressbar || util::get_envvar_flag("QSV_PROGRESSBAR")) && !rconfig.is_stdin();
    let progress = ProgressBar::with_draw_target(None, ProgressDrawTarget::stderr_with_hz(5));
    if show_progress {
        util::prep_progress(&progress, util::count_rows(&rconfig)?);
    } else {
        progress.set_draw_target(ProgressDrawTarget::hidden());
    }

    // ensure col/header names are valid and safe python variables
    let (header_vec, _) = util::safe_header_names(&headers, true, false, &None, "_", false);

    // amortize memory allocation by reusing record
    #[allow(unused_assignments)]
    let mut batch_record = csv::StringRecord::new();
    let mut error_count = 0_usize;

    let batch_size = if args.flag_batch == 0 {
        util::count_rows(&rconfig)? as usize
    } else {
        args.flag_batch
    };

    // reuse batch buffers
    let mut batch = Vec::with_capacity(batch_size);

    // main loop to read CSV and construct batches.
    // we batch python operations so that the GILPool does not get very large
    // as we release the pool after each batch
    // loop exits when batch is empty.
    // see https://pyo3.rs/latest/memory.html#gil-bound-memory for more info.
    'batch_loop: loop {
        for _ in 0..batch_size {
            match rdr.read_record(&mut batch_record) {
                Ok(has_data) => {
                    if has_data {
                        batch.push(std::mem::take(&mut batch_record));
                    } else {
                        // nothing else to add to batch
                        break;
                    }
                },
                Err(e) => {
                    return fail_clierror!("Error reading file: {e}");
                },
            }
        }

        if batch.is_empty() {
            // break out of infinite loop when at EOF
            break 'batch_loop;
        }

        Python::with_gil(|py| -> PyResult<()> {
            let curr_batch = batch.clone();
            let helpers = PyModule::from_code_bound(py, HELPERS, "qsv_helpers.py", "qsv_helpers")?;
            let batch_globals = PyDict::new_bound(py);
            let batch_locals = PyDict::new_bound(py);

            let user_helpers =
                PyModule::from_code_bound(py, &helper_text, "qsv_user_helpers.py", "qsv_uh")?;
            batch_globals.set_item(intern!(py, "qsv_uh"), user_helpers)?;

            // Global imports
            let builtins = PyModule::import_bound(py, "builtins")?;
            let math_module = PyModule::import_bound(py, "math")?;
            let random_module = PyModule::import_bound(py, "random")?;
            let datetime_module = PyModule::import_bound(py, "datetime")?;

            batch_globals.set_item("__builtins__", builtins)?;
            batch_globals.set_item("math", math_module)?;
            batch_globals.set_item("random", random_module)?;
            batch_globals.set_item("datetime", datetime_module)?;

            let py_row = helpers
                .getattr("QSVRow")?
                .call1((headers.iter().collect::<Vec<&str>>(),))?;

            batch_locals.set_item("col", py_row.clone())?;

            let error_result = intern!(py, "<ERROR>");

            for mut record in curr_batch {
                // Initializing locals
                let mut row_data: Vec<&str> = Vec::with_capacity(headers_len);

                // assert so the record.get() below skips bounds check
                assert!(record.len() == headers_len);
                header_vec
                    .iter()
                    .enumerate()
                    .take(headers_len)
                    .for_each(|(i, key)| {
                        let cell_value = record.get(i).unwrap_or_default();
                        batch_locals
                            .set_item(key, cell_value)
                            .expect("cannot set_item");
                        row_data.push(cell_value);
                    });

                py_row.call_method1(intern!(py, "_update_underlying_data"), (row_data,))?;

                let result = py
                    .eval_bound(&args.arg_script, Some(&batch_globals), Some(&batch_locals))
                    .map_err(|e| {
                        e.print_and_set_sys_last_vars(py);
                        error_count += 1;
                        if log_enabled!(Debug) {
                            error!("{e:?}");
                        }
                        "Evaluation of given expression failed with the above error!"
                    })
                    .unwrap_or_else(|_| error_result.clone().into_any());

                if args.cmd_map {
                    let result = helpers
                        .getattr(intern!(py, "cast_as_string"))?
                        .call1((result,))?;
                    let value: String = result.extract()?;

                    record.push_field(&value);
                    if let Err(e) = wtr.write_record(&record) {
                        // we do this since we cannot use the ? operator here
                        // since this closure returns a PyResult
                        // this is converted to a CliError::Other anyway
                        return Err(pyo3::PyErr::new::<pyo3::exceptions::PyIOError, _>(format!(
                            "cannot write record ({e})"
                        )));
                    }
                } else if args.cmd_filter {
                    let result = helpers
                        .getattr(intern!(py, "cast_as_bool"))?
                        .call1((result,))?;
                    let include_record: bool = result.extract().unwrap_or(false);

                    if include_record {
                        if let Err(e) = wtr.write_record(&record) {
                            return Err(pyo3::PyErr::new::<pyo3::exceptions::PyIOError, _>(
                                format!("cannot write record ({e})"),
                            ));
                        }
                    }
                }
            }

            Ok(())
        })?;
        if show_progress {
            progress.inc(batch.len() as u64);
        }

        batch.clear();
    } // end batch loop

    if show_progress {
        util::finish_progress(&progress);
    }

    wtr.flush()?;

    if error_count > 0 {
        return fail_clierror!("Python errors encountered: {error_count}");
    }

    Ok(())
}
