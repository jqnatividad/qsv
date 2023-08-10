static USAGE: &str = r#"
Convert newline-delimited JSON (JSONL/NDJSON) to CSV.

The command tries to do its best but since it is not possible to
straightforwardly convert JSON lines to CSV, the process might lose some complex
fields from the input.

Also, it will fail if the JSON documents are not consistent with one another,
as the first JSON line will be use to infer the headers of the CSV output.

For examples, see https://github.com/jqnatividad/qsv/blob/master/tests/test_jsonl.rs.

Usage:
    qsv jsonl [options] [<input>]
    qsv jsonl --help

jsonl options:
    --ignore-errors        Skip malformed input lines.

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

use serde::Deserialize;
use serde_json::Value;

use crate::{
    config::{Config, Delimiter},
    util, CliResult,
};

#[derive(Deserialize)]
struct Args {
    arg_input:          Option<String>,
    flag_output:        Option<String>,
    flag_delimiter:     Option<Delimiter>,
    flag_ignore_errors: bool,
}

#[allow(clippy::needless_pass_by_value)]
fn recurse_to_infer_headers(value: &Value, headers: &mut Vec<Vec<String>>, path: Vec<String>) {
    match value {
        Value::Object(map) => {
            for (key, value) in map {
                match value {
                    Value::Null
                    | Value::Bool(_)
                    | Value::Number(_)
                    | Value::String(_)
                    | Value::Array(_) => {
                        let mut full_path = path.clone();
                        full_path.push(key.to_string());

                        headers.push(full_path);
                    }
                    Value::Object(_) => {
                        let mut new_path = path.clone();
                        new_path.push(key.to_string());

                        recurse_to_infer_headers(value, headers, new_path);
                    }
                    #[allow(unreachable_patterns)]
                    _ => {}
                }
            }
        }
        _ => {
            headers.push(vec![String::from("value")]);
        }
    }
}

fn infer_headers(value: &Value) -> Vec<Vec<String>> {
    let mut headers: Vec<Vec<String>> = Vec::new();

    recurse_to_infer_headers(value, &mut headers, Vec::new());

    headers
}

fn get_value_at_path(value: &Value, path: &[String]) -> Option<Value> {
    let mut current = value;

    for key in path {
        match current.get(key) {
            Some(new_value) => {
                current = new_value;
            }
            None => {
                return None;
            }
        }
    }

    Some(current.clone())
}

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
                }
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

    let rdr: Box<dyn BufRead> = match args.arg_input {
        None => Box::new(BufReader::new(io::stdin())),
        Some(p) => Box::new(BufReader::new(fs::File::open(p)?)),
    };

    let mut headers: Vec<Vec<String>> = Vec::new();
    let mut headers_emitted: bool = false;

    for (rowidx, line) in rdr.lines().enumerate() {
        let value: Value = match serde_json::from_str(&line?) {
            Ok(v) => v,
            Err(e) => {
                if args.flag_ignore_errors {
                    continue;
                }
                let human_idx = rowidx + 1; // not zero based, for readability
                return fail_clierror!(
                    r#"Could not parse line {human_idx} as JSON!: {e}
Use `--ignore-errors` option to skip malformed input lines.
Use `tojsonl` command to convert _to_ jsonl instead of _from_ jsonl."#,
                );
            }
        };

        if !headers_emitted {
            headers = infer_headers(&value);

            let headers_formatted = headers.iter().map(|v| v.join(".")).collect::<Vec<String>>();
            let headers_record = csv::StringRecord::from(headers_formatted);
            wtr.write_record(&headers_record)?;

            headers_emitted = true;
        }

        let record = json_line_to_csv_record(&value, &headers);
        wtr.write_record(&record)?;
    }

    Ok(())
}
