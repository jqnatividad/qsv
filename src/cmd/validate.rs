use crate::config::{Config, Delimiter, DEFAULT_WTR_BUFFER_CAPACITY};
use crate::util;
use crate::CliError;
use crate::CliResult;
use anyhow::{anyhow, Result};
use csv::ByteRecord;
use indicatif::{ProgressBar, ProgressDrawTarget};
use jsonschema::{output::BasicOutput, JSONSchema};
use log::{debug, info};
use rayon::prelude::*;
use serde::Deserialize;
use serde_json::{value::Number, Map, Value};
use std::{env, fs::File, io::BufReader, io::BufWriter, io::Read, io::Write, str};

macro_rules! fail {
    ($mesg:expr) => {
        Err(CliError::Other($mesg))
    };
}

const BATCH_SIZE: usize = 16000;

static USAGE: &str = "
Validate CSV data with JSON Schema, and put invalid records into a separate file.

Example output files from `mydata.csv`. If piped from stdin, then filename is `stdin.csv`.

* mydata.csv.valid
* mydata.csv.invalid
* mydata.csv.validation-errors.jsonl

JSON Schema can be a local file or a URL.

When run without JSON Schema, only a simple CSV check (RFC 4180) is performed, with the caveat that
 on non-Windows machines, each record is delimited by a LF (\\n) instead of CRLF (\\r\\n).


Usage:
    qsv validate [options] [<input>] [<json-schema>]

Validate options:
    --fail-fast                Stops on first error.
    --valid <suffix>           Valid record output file suffix. [default: valid]
    --invalid <suffix>         Invalid record output file suffix. [default: invalid]


Common options:
    -h, --help                 Display this message
    -n, --no-headers           When set, the first row will not be interpreted
                               as headers. Namely, it will be sorted with the rest
                               of the rows. Otherwise, the first row will always
                               appear as the header row in the output.
    -d, --delimiter <arg>      The field delimiter for reading CSV data.
                               Must be a single character. [default: ,]
    -q, --quiet                Don't show progress bars.
";

#[derive(Deserialize, Debug)]
struct Args {
    flag_fail_fast: bool,
    flag_valid: Option<String>,
    flag_invalid: Option<String>,
    flag_no_headers: bool,
    flag_delimiter: Option<Delimiter>,
    flag_quiet: bool,
    arg_input: Option<String>,
    arg_json_schema: Option<String>,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;

    // dbg!(&args);

    let rconfig = Config::new(&args.arg_input)
        .delimiter(args.flag_delimiter)
        .no_headers(args.flag_no_headers);

    let rdr = &mut rconfig.reader()?;

    // if no json schema supplied, only let csv reader validate csv file
    if args.arg_json_schema.is_none() {
        // just read csv file and let csv reader report problems
        let mut record = csv::ByteRecord::new();
        while rdr.read_byte_record(&mut record)? {
            // this loop is for csv::reader to do basic csv validation on read
        }

        let msg = "Can't validate without schema, but csv looks good.".to_string();
        info!("{msg}");
        println!("{msg}");

        return Ok(());
    }

    let headers = rdr.byte_headers()?.clone();

    let input_path = args
        .arg_input
        .clone()
        .unwrap_or_else(|| "stdin.csv".to_string());

    let wtr_capacitys = env::var("QSV_WTR_BUFFER_CAPACITY")
        .unwrap_or_else(|_| DEFAULT_WTR_BUFFER_CAPACITY.to_string());
    let wtr_buffer: usize = wtr_capacitys.parse().unwrap_or(DEFAULT_WTR_BUFFER_CAPACITY);

    let mut error_report_file = BufWriter::with_capacity(
        wtr_buffer,
        File::create(input_path.to_owned() + ".validation-errors.jsonl")
            .expect("unable to create error report file"),
    );

    // prep progress bar
    let progress = ProgressBar::new(0);
    let record_count = util::count_rows(&rconfig.flexible(true));

    if !args.flag_quiet {
        util::prep_progress(&progress, record_count);
    } else {
        progress.set_draw_target(ProgressDrawTarget::hidden());
    }

    // parse and compile supplied JSON Schema
    let (schema_json, schema_compiled): (Value, JSONSchema) =
        match load_json(&args.arg_json_schema.unwrap()) {
            Ok(s) => {
                // parse JSON string
                match serde_json::from_str(&s) {
                    Ok(json) => {
                        // compile JSON Schema
                        match JSONSchema::options().compile(&json) {
                            Ok(schema) => (json, schema),
                            Err(e) => {
                                return fail!(format!("Cannot compile schema json. error: {e}"));
                            }
                        }
                    }
                    Err(e) => {
                        //error!("Unable to parse schema json. error: {}", e);
                        return fail!(format!("Unable to parse schema json. error: {e}"));
                    }
                }
            }
            Err(e) => {
                return fail!(format!("Unable to retrieve json. error: {e}"));
            }
        };

    debug!("compiled schema: {:?}", &schema_compiled);

    // keep track of where we are
    let mut row_number: usize = 0;
    let mut invalid_count: usize = 0;

    // amortize memory allocation by reusing record
    #[allow(unused_assignments)]
    let mut record = csv::ByteRecord::new();
    // reuse batch buffer
    let mut batch = Vec::with_capacity(BATCH_SIZE);
    let mut valid_flags: Vec<bool> = Vec::with_capacity(record_count as usize);

    // main loop to read CSV and construct batches for parallel processing.
    // each batch is processed via Rayon parallel iterator.
    // loop exits when batch is empty.
    loop {
        for _ in 0..BATCH_SIZE {
            match rdr.read_byte_record(&mut record) {
                Ok(has_data) => {
                    if has_data {
                        row_number += 1;
                        record.push_field(row_number.to_string().as_bytes());
                        batch.push(record.to_owned());
                    } else {
                        // nothing else to add to batch
                        break;
                    }
                }
                Err(e) => {
                    return Err(CliError::Other(format!(
                        "Error reading row: {row_number}: {e}"
                    )));
                }
            }
        }

        if batch.is_empty() {
            // break out of infinite loop when at EOF
            break;
        }

        let batch_size = batch.len();

        // do actual validation via Rayon parallel iterator
        let validation_results: Vec<Option<Value>> = batch
            .par_iter()
            .map(|record| {
                match do_json_validation(&headers, record, &schema_json, &schema_compiled) {
                    Ok(o) => o,
                    Err(e) => {
                        panic!("Unrecoverable error: {e:?}");
                    }
                }
            })
            .collect();

        batch.clear();

        // write to validation error report, but keep Vec<bool> to gen valid/invalid files later
        // because Rayon collect() guaranteeds original order, can just update valid_flags vector sequentially
        for result in validation_results {
            match result {
                Some(validation_error) => {
                    invalid_count += 1;
                    valid_flags.push(false);

                    error_report_file
                        .write_all(format!("{validation_error}\n").as_bytes())
                        .expect("unable to write to validation error report");

                    // for fail-fast, just break out of loop after first error
                    if args.flag_fail_fast {
                        break;
                    }
                }
                None => {
                    valid_flags.push(true);
                }
            }
        }

        if !args.flag_quiet {
            progress.inc(batch_size as u64);
        }
    } // end infinite loop

    debug!("valid flags: {valid_flags:?}");
    debug!("invalid count: {invalid_count}");

    // flush error report; file gets closed automagically when out-of-scope
    error_report_file.flush().unwrap();

    // only write out invalid/valid output files if there are actually invalid records
    if invalid_count > 0 {
        let mut row_num: usize = 0;

        // prepare output files
        let valid_suffix: &str = &args.flag_valid.unwrap_or_else(|| "valid".to_string());
        let mut valid_wtr =
            Config::new(&Some(input_path.to_owned() + "." + valid_suffix)).writer()?;
        valid_wtr.write_byte_record(&headers)?;

        let invalid_suffix: &str = &args.flag_invalid.unwrap_or_else(|| "invalid".to_string());
        let mut invalid_wtr = Config::new(&Some(input_path + "." + invalid_suffix)).writer()?;
        invalid_wtr.write_byte_record(&headers)?;

        // get another reader to split into valid/invalid files
        let rconfig = Config::new(&args.arg_input)
            .delimiter(args.flag_delimiter)
            .no_headers(args.flag_no_headers);
        let rdr = &mut rconfig.reader()?;

        let mut record = csv::ByteRecord::new();
        while rdr.read_byte_record(&mut record)? {
            row_num += 1;

            // vector is 0-based, row_num is 1-based
            let is_valid = valid_flags[row_num - 1];

            debug!("row[{row_num}]: valid={is_valid}");

            if is_valid {
                valid_wtr.write_byte_record(&record)?;
            } else {
                invalid_wtr.write_byte_record(&record)?;
            }
        }

        valid_wtr.flush().unwrap();
        invalid_wtr.flush().unwrap();
    }

    use thousands::Separable;

    if !args.flag_quiet {
        progress.set_message(format!(
            " validated {} records.",
            progress.length().separate_with_commas()
        ));
        util::finish_progress(&progress);
    }

    // done with validation; print output
    if args.flag_fail_fast {
        let msg = format!(
            "fail-fast enabled. stopping after first invalid record at row {}",
            row_number.separate_with_commas()
        );
        info!("{msg}");
        println!("{msg}");
    } else {
        let msg = format!(
            "{} out of {} records invalid.",
            invalid_count.separate_with_commas(),
            row_number.separate_with_commas()
        );
        info!("{msg}");
        println!("{msg}");
    }

    Ok(())
}

/// if given record is valid, CliResult would hold None, otherwise, Some(Value)
fn do_json_validation(
    headers: &ByteRecord,
    record: &ByteRecord,
    schema_json: &Value,
    schema_compiled: &JSONSchema,
) -> CliResult<Option<Value>> {
    // row number was added as last column
    let row_number_string = str::from_utf8(record.get(headers.len()).unwrap()).unwrap();
    let row_number: usize = row_number_string.parse::<usize>().unwrap();

    let instance: Value = match to_json_instance(headers, record, schema_json) {
        Ok(obj) => obj,
        Err(e) => {
            return fail!(format!(
                "Unable to convert CSV to json. row: {row_number}, error: {e}"
            ));
        }
    };

    debug!("instance[{row_number}]: {instance:?}");

    match validate_json_instance(&instance, schema_compiled) {
        Ok(mut validation_result) => {
            let is_valid = validation_result.get("valid").unwrap().as_bool().unwrap();

            if is_valid {
                Ok(None)
            } else {
                debug!("row[{row_number}] is invalid");

                // insert row index and return validation error
                validation_result.as_object_mut().unwrap().insert(
                    "row_number".to_string(),
                    Value::Number(Number::from(row_number)),
                );

                Ok(Some(validation_result))
            }
        }
        Err(e) => {
            return fail!(format!("Unable to validate. row: {row_number}, error: {e}"));
        }
    }
}

/// convert CSV Record into JSON instance by referencing Type from Schema
fn to_json_instance(headers: &ByteRecord, record: &ByteRecord, schema: &Value) -> Result<Value> {
    // make sure schema has expected structure
    let schema_properties = schema
        .get("properties")
        .expect("JSON Schema missing 'properties' object");

    // map holds individual CSV fields converted as serde_json::Value
    let mut json_object_map: Map<String, Value> = Map::new();

    // iterate over each CSV field and convert to JSON type
    let headers_iter = headers.iter().enumerate();

    let null_type: Value = Value::String("null".to_string());

    for (i, header) in headers_iter {
        // convert csv header to string
        let header_string = std::str::from_utf8(header)?.to_string();
        // convert csv value to string; trim whitespace
        let value_string = std::str::from_utf8(&record[i])?.trim().to_string();

        // get json type from schema; defaults to STRING if not specified
        let field_def: &Value = schema_properties
            .get(&header_string)
            .unwrap_or(&Value::Null);

        let field_type_def: &Value = field_def.get("type").unwrap_or(&Value::Null);

        let json_type = match field_type_def {
            Value::String(s) => s,
            Value::Array(vec) => {
                // if can't find usable type info, defaults to "string"
                let mut return_val = "string";

                // grab the first entry that's not a "null", since it just means value is optional
                for val in vec {
                    if *val != null_type {
                        return_val = val.as_str().expect("type info should be a JSON string");
                    } else {
                        // keep looking
                        continue;
                    }
                }

                return_val
            }
            _ => {
                // default to JSON String
                "string"
            }
        };

        // dbg!(i, &header_string, &value_string, &json_type);

        // if value_string is empty, then just put an empty JSON String
        if value_string.is_empty() {
            json_object_map.insert(header_string, Value::Null);
            continue;
        }

        match json_type {
            "string" => {
                json_object_map.insert(header_string, Value::String(value_string));
            }
            "number" => {
                if let Ok(float) = value_string.parse::<f64>() {
                    json_object_map.insert(
                        header_string,
                        Value::Number(Number::from_f64(float).expect("not a valid f64 float")),
                    );
                } else {
                    return Err(anyhow!(
                        "Can't cast into Float. header: {header_string}, value: {value_string}, json type: {json_type}"
                    ));
                }
            }
            "integer" => {
                if let Ok(int) = value_string.parse::<i64>() {
                    json_object_map.insert(header_string, Value::Number(Number::from(int)));
                } else {
                    return Err(anyhow!(
                        "Can't cast into Integer. header: {header_string}, value: {value_string}, json type: {json_type}"
                    ));
                }
            }
            "boolean" => {
                if let Ok(boolean) = value_string.parse::<bool>() {
                    json_object_map.insert(header_string, Value::Bool(boolean));
                } else {
                    return Err(anyhow!(
                        "Can't cast into Boolean. header: {header_string}, value: {value_string}, json type: {json_type}"
                    ));
                }
            }
            _ => {
                return Err(anyhow!(
                    "Unsupported JSON type. header: {header_string}, value: {value_string}, json type: {json_type}"
                ));
            }
        }
    }

    // dbg!(&json_object_map);

    Ok(Value::Object(json_object_map))
}

#[cfg(test)]
mod tests_for_csv_to_json_conversion {

    use super::*;
    use serde_json::json;

    /// get schema used for unit tests
    fn schema_json() -> Value {
        // from https://json-schema.org/learn/miscellaneous-examples.html
        serde_json::json!({
            "$id": "https://example.com/test.schema.json",
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "title": "test",
            "type": "object",
            "properties": {
                "A": {
                    "type": "string",
                },
                "B": {
                    "type": "number",
                },
                "C": {
                    "type": "integer",
                },
                "D": {
                    "type": "boolean",
                },
                "E": {
                    "type": ["string", "null"],
                },
                "F": {
                    "type": ["number", "null"],
                },
                "G": {
                    "type": ["integer", "null"],
                },
                "H": {
                    "type": ["boolean", "null"],
                },
                "I": {
                    "type": ["string", "null"],
                },
                "J": {
                    "type": ["number", "null"],
                },
                "K": {
                    "type": ["null", "integer"],
                },
                "L": {
                    "type": ["boolean", "null"],
                },
            }
        })
    }

    #[test]
    fn test_to_json_instance() {
        let csv = "A,B,C,D,E,F,G,H,I,J,K,L
        hello,3.1415,300000000,true,,,,,hello,3.1415,300000000,true";

        let mut rdr = csv::Reader::from_reader(csv.as_bytes());
        let headers = rdr.byte_headers().unwrap().clone();

        assert_eq!(
            to_json_instance(
                &headers,
                &rdr.byte_records().next().unwrap().unwrap(),
                &schema_json()
            )
            .expect("can convert csv to json instance"),
            json!({
                "A": "hello",
                "B": 3.1415,
                "C": 300000000,
                "D": true,
                "E": null,
                "F": null,
                "G": null,
                "H": null,
                "I": "hello",
                "J": 3.1415,
                "K": 300000000,
                "L": true,
            })
        );
    }

    #[test]
    fn test_to_json_instance_cast_integer_error() {
        let csv = "A,B,C,D,E,F,G,H
        hello,3.1415,3.0e8,true,,,,";

        let mut rdr = csv::Reader::from_reader(csv.as_bytes());
        let headers = rdr.byte_headers().unwrap().clone();

        let result = to_json_instance(
            &headers,
            &rdr.byte_records().next().unwrap().unwrap(),
            &schema_json(),
        );
        assert!(&result.is_err());
        let error = result.err().unwrap();
        assert_eq!(
            "Can't cast into Integer. header: C, value: 3.0e8, json type: integer",
            error.to_string()
        );
    }
}

/// Validate JSON instance against compiled JSON schema
fn validate_json_instance(instance: &Value, schema_compiled: &JSONSchema) -> Result<Value> {
    let output: BasicOutput = schema_compiled.apply(instance).basic();

    match serde_json::to_value(output) {
        Ok(json) => Ok(json),
        Err(e) => Err(anyhow!(
            "Cannot convert schema validation output to json: {e}"
        )),
    }
}

#[cfg(test)]
mod tests_for_schema_validation {
    use super::*;

    fn schema_json() -> Value {
        // from https://json-schema.org/learn/miscellaneous-examples.html
        serde_json::json!({
            "$id": "https://example.com/person.schema.json",
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "title": "Person",
            "type": "object",
            "properties": {
                "title": {
                    "type": "string",
                    "description": "The person's title.",
                    "minLength": 2
                },
                "name": {
                    "type": "string",
                    "description": "The person's name.",
                    "minLength": 2
                },
                "age": {
                    "description": "Age in years which must be equal to or greater than 18.",
                    "type": "integer",
                    "minimum": 18
                }
            }
        })
    }

    fn compiled_schema() -> JSONSchema {
        JSONSchema::options()
            .compile(&schema_json())
            .expect("A valid schema")
    }

    #[test]
    fn test_validate_with_no_errors() {
        let csv = "title,name,age
        Professor,Xaviers,60";

        let mut rdr = csv::Reader::from_reader(csv.as_bytes());
        let headers = rdr.byte_headers().unwrap().clone();

        let record = &rdr.byte_records().next().unwrap().unwrap();

        let instance = to_json_instance(&headers, &record, &schema_json()).unwrap();

        let result = validate_json_instance(&instance, &compiled_schema()).unwrap();

        assert_eq!(true, result["valid"].as_bool().unwrap());
    }

    #[test]
    fn test_validate_with_error() {
        let csv = "title,name,age
        Professor,X,60";

        let mut rdr = csv::Reader::from_reader(csv.as_bytes());
        let headers = rdr.byte_headers().unwrap().clone();

        let record = &rdr.byte_records().next().unwrap().unwrap();

        let instance = to_json_instance(&headers, &record, &schema_json()).unwrap();

        let result = validate_json_instance(&instance, &compiled_schema()).unwrap();

        assert_eq!(false, result["valid"].as_bool().unwrap());
    }
}

fn load_json(uri: &str) -> Result<String> {
    let json_string = match uri {
        url if url.starts_with("http") => {
            // dbg!(&url);
            let response = reqwest::blocking::get(url)?;
            response.text()?
        }
        path => {
            // dbg!(&_path);
            let mut buffer = String::new();
            BufReader::new(File::open(path)?).read_to_string(&mut buffer)?;
            buffer
        }
    };

    // dbg!(&json_string);

    Ok(json_string)
}

#[test]
fn test_load_json_via_url() {
    let json_string_result =
        load_json(&("https://geojson.org/schema/FeatureCollection.json".to_owned()));
    assert!(&json_string_result.is_ok());

    let json_result: Result<Value, serde_json::Error> =
        serde_json::from_str(&json_string_result.unwrap());
    assert!(&json_result.is_ok());
}
