static USAGE: &str = r#"
Convert newline-delimited JSON (JSONL/NDJSON) to CSV.

The command tries to do its best but since it is not possible to
straightforwardly convert JSON lines to CSV, the process might lose some complex
fields from the input.

Also, it will fail if the JSON documents are not consistent with one another,
as the first JSON line will be used to infer the headers of the CSV output.

For examples, see https://github.com/jqnatividad/qsv/blob/master/tests/test_jsonl.rs.

Usage:
    qsv jsonl [options] [<input>]
    qsv jsonl --help

jsonl options:
    --ignore-errors        Skip malformed input lines.
    -j, --jobs <arg>       The number of jobs to run in parallel.
                           When not set, the number of jobs is set to the 
                           number of CPUs detected.
    -b, --batch <size>     The number of rows per batch to load into memory,
                           before running in parallel. Set to 0 to load all
                           rows at once. [default: 50000]

Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
    -d, --delimiter <arg>  The delimiter to use when writing CSV data.
                           Must be a single character. [default: ,]
"#;

use std::{
    fs,
    io::{self, BufRead, BufReader},
};

use rayon::{
    iter::{IndexedParallelIterator, ParallelIterator},
    prelude::IntoParallelRefIterator,
};
use serde::Deserialize;
use serde_json::Value;

use crate::{
    config::{Config, Delimiter, DEFAULT_RDR_BUFFER_CAPACITY},
    util, CliResult,
};

#[derive(Deserialize)]
struct Args {
    arg_input:          Option<String>,
    flag_output:        Option<String>,
    flag_delimiter:     Option<Delimiter>,
    flag_ignore_errors: bool,
    flag_jobs:          Option<usize>,
    flag_batch:         usize,
}

fn recurse_to_infer_headers(value: &Value, headers: &mut Vec<Vec<String>>, path: &[String]) {
    match value {
        Value::Object(map) => {
            for (key, value) in map {
                match value {
                    Value::Null
                    | Value::Bool(_)
                    | Value::Number(_)
                    | Value::String(_)
                    | Value::Array(_) => {
                        let mut full_path = path.to_owned();
                        full_path.push(key.to_string());

                        headers.push(full_path);
                    },
                    Value::Object(_) => {
                        let mut new_path = path.to_owned();
                        new_path.push(key.to_string());

                        recurse_to_infer_headers(value, headers, &new_path);
                    },
                    #[allow(unreachable_patterns)]
                    _ => {},
                }
            }
        },
        _ => {
            headers.push(vec![String::from("value")]);
        },
    }
}

fn infer_headers(value: &Value) -> Vec<Vec<String>> {
    let mut headers: Vec<Vec<String>> = Vec::new();

    recurse_to_infer_headers(value, &mut headers, &Vec::new());

    headers
}

fn get_value_at_path(value: &Value, path: &[String]) -> Option<Value> {
    let mut current = value;

    for key in path {
        match current.get(key) {
            Some(new_value) => {
                current = new_value;
            },
            None => {
                return None;
            },
        }
    }

    Some(current.clone())
}

#[inline]
fn json_line_to_csv_record(value: &Value, headers: &[Vec<String>]) -> csv::StringRecord {
    let mut record = csv::StringRecord::new();

    for path in headers {
        let value = get_value_at_path(value, path);

        if let Some(value) = value {
            record.push_field(&match value {
                Value::Bool(v) => {
                    if v {
                        String::from("true")
                    } else {
                        String::from("false")
                    }
                },
                Value::Number(v) => v.to_string(),
                Value::String(v) => v,
                Value::Array(v) => v
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(","),
                _ => String::new(),
            });
        } else {
            record.push_field("");
        }
    }

    record
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;
    let mut wtr = Config::new(&args.flag_output)
        .delimiter(args.flag_delimiter)
        .writer()?;

    let mut is_stdin = false;
    let mut rdr: Box<dyn BufRead> = match args.arg_input {
        None => {
            is_stdin = true;
            Box::new(BufReader::new(io::stdin()))
        },
        Some(p) => Box::new(BufReader::with_capacity(
            DEFAULT_RDR_BUFFER_CAPACITY,
            fs::File::open(p)?,
        )),
    };

    let mut headers: Vec<Vec<String>> = Vec::new();
    let mut headers_emitted: bool = false;

    // amortize memory allocation by reusing record
    let mut batch_line = String::new();

    // reuse batch buffers
    let batchsize: usize = if args.flag_batch == 0 {
        if is_stdin {
            // if stdin, we don't know how many lines there are
            // so just make a reasonably big batch size
            1_000_000
        } else {
            // safety: we know flag_output is Some coz of the std_in check above
            util::count_lines_in_file(&args.flag_output.unwrap())? as usize
        }
    } else {
        args.flag_batch
    };
    let mut batch = Vec::with_capacity(batchsize);
    let mut batch_results = Vec::with_capacity(batchsize);

    // set RAYON_NUM_THREADS
    util::njobs(args.flag_jobs);

    let mut result_idx = 0_u64;

    'batch_loop: loop {
        for _ in 0..batchsize {
            batch_line.clear();
            match rdr.read_line(&mut batch_line) {
                Ok(0) => {
                    // EOF
                    break;
                },
                Ok(_) => {
                    batch.push(batch_line.clone());
                },
                Err(e) => {
                    if args.flag_ignore_errors {
                        continue;
                    }
                    return fail_clierror!(
                        r#"Could not read input line!: {e}
Use `--ignore-errors` option to skip malformed input lines.
Use `tojsonl` command to convert _to_ jsonl instead of _from_ jsonl."#,
                    );
                },
            }
        }

        if batch.is_empty() {
            break 'batch_loop; // EOF
        }

        if !headers_emitted {
            let value: Value = match serde_json::from_str(&batch[0]) {
                Ok(v) => v,
                Err(e) => {
                    return fail_clierror!(
                        "Could not parse first input line as JSON to infer headers: {e}",
                    );
                },
            };
            headers = infer_headers(&value);

            let headers_formatted = headers.iter().map(|v| v.join(".")).collect::<Vec<String>>();
            let headers_record = csv::StringRecord::from(headers_formatted);
            wtr.write_record(&headers_record)?;

            headers_emitted = true;
        }

        // do actual work via rayon
        batch
            .par_iter()
            .map(|json_line| match serde_json::from_str(json_line) {
                Ok(v) => Some(json_line_to_csv_record(&v, &headers)),
                Err(e) => {
                    if !args.flag_ignore_errors {
                        log::error!("serde_json::from_str error: {:#?}", e);
                    }
                    None
                },
            })
            .collect_into_vec(&mut batch_results);

        // rayon collect() guarantees original order, so we can just append results of each batch
        for result_record in &batch_results {
            result_idx += 1;
            if let Some(record) = result_record {
                wtr.write_record(record)?;
            } else if !args.flag_ignore_errors {
                // there was an error parsing a json line
                return fail_clierror!(
                    r#"Could not parse input line {result_idx} as JSON
Use `--ignore-errors` option to skip malformed input lines.
Use `tojsonl` command to convert _to_ jsonl instead of _from_ jsonl."#,
                );
            }
        }

        batch.clear();
    } // end batch loop

    Ok(wtr.flush()?)
}
