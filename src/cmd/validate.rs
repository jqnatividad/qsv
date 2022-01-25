use crate::config::{Config, Delimiter, DEFAULT_WTR_BUFFER_CAPACITY};
use crate::util;
use crate::CliError;
use crate::CliResult;
use anyhow::{anyhow, Result};
use csv::ByteRecord;
use indicatif::{ProgressBar, ProgressDrawTarget};
use jsonschema::{output::BasicOutput, JSONSchema};
use log::{debug, info};
use serde::Deserialize;
use serde_json::{value::Number, Map, Value};
use std::{env, fs::File, io::BufReader, io::BufWriter, io::Read, io::Write, ops::Add};

macro_rules! fail {
    ($mesg:expr) => {
        Err(CliError::Other($mesg))
    };
}

static USAGE: &str = "
Validate CSV data with JSON Schema, and put invalid records into separate file.

Example output files from `mydata.csv`. If piped from stdin, then filename is `stdin.csv`.

* mydata.csv.valid
* mydata.csv.invalid
* mydata.csv.validation-errors.jsonl

JSON Schema can be a local file or a URL. 

When run without JSON Schema, only a simple CSV check (RFC 4180) is performed, with the caveat that 
 on non-Windows machines, each record is delimited by a LF (\n) instead of CRLF (\r\n).


Usage:
    qsv validate [options] [<input>] [<json-schema>]

fetch options:
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

    let mut rdr = rconfig.reader()?;

    let headers = rdr.byte_headers()?.clone();

    let input_path: &str = &args.arg_input.unwrap_or_else(|| "stdin.csv".to_string());

    let valid_suffix: &str = &args.flag_valid.unwrap_or_else(|| "valid".to_string());
    let mut valid_wtr = Config::new(&Some(input_path.to_owned() + "." + valid_suffix)).writer()?;

    let invalid_suffix: &str = &args.flag_invalid.unwrap_or_else(|| "invalid".to_string());
    let mut invalid_wtr =
        Config::new(&Some(input_path.to_owned() + "." + invalid_suffix)).writer()?;

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
    if !args.flag_quiet {
        let record_count = util::count_rows(&rconfig.flexible(true));
        util::prep_progress(&progress, record_count);
    } else {
        progress.set_draw_target(ProgressDrawTarget::hidden());
    }

    // check if need to validate via json schema, or just let csv reader validate csv file
    if let Some(json_schema_uri) = &args.arg_json_schema {
        let (schema_json, schema_compiled): (Value, JSONSchema) = match load_json(json_schema_uri) {
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

        let mut valid_file_empty: bool = true;
        let mut invalid_file_empty: bool = true;

        // amortize memory allocation by reusing record
        #[allow(unused_assignments)]
        let mut record = csv::ByteRecord::new();

        let mut row_index: u32 = 0;
        let mut invalid_count: u32 = 0;

        while rdr.read_byte_record(&mut record)? {
            row_index = row_index.add(1);

            let instance: Value = match to_json_instance(&headers, &record, &schema_json) {
                Ok(obj) => obj,
                Err(e) => {
                    return fail!(format!(
                        "Unable to convert CSV to json. row: {row_index}, error: {e}"
                    ));
                }
            };

            debug!("instance[{}]: {:?}", &row_index, &instance);

            match validate_json_instance(&instance, &schema_compiled) {
                Ok(validation_result) => {
                    let results = &validation_result["valid"];

                    debug!("validation[{row_index}]: {results:?}");

                    let valid_flag = match results.as_bool().to_owned() {
                        Some(b) => b,
                        None => {
                            return fail!(format!(
                                "Unexpected validation result. row: {row_index}, result: {results:?}"
                            ));
                        }
                    };

                    match valid_flag {
                        true => {
                            if valid_file_empty {
                                valid_wtr.write_byte_record(&headers)?;
                                valid_file_empty = false;
                            }

                            valid_wtr.write_byte_record(&record)?;
                        }
                        false => {
                            invalid_count = invalid_count.add(1);

                            debug!(
                                "schema violation. row: {row_index}, violation: {validation_result:?}"
                            );
                            // dbg!(&validation_result, &record);

                            // write to invalid file

                            if invalid_file_empty {
                                invalid_wtr.write_byte_record(&headers)?;
                                invalid_file_empty = false;
                            }

                            invalid_wtr.write_byte_record(&record)?;

                            // write to error report
                            let mut enriched_results_map = validation_result
                                .as_object()
                                .expect("get validation results as map")
                                .clone();
                            let _ = enriched_results_map.insert(
                                "row_index".to_string(),
                                Value::Number(Number::from(row_index)),
                            );
                            let enriched_results: Value = Value::Object(enriched_results_map);

                            error_report_file
                                .write_all(format!("{enriched_results}\n").as_bytes())
                                .expect("unable to write to validation error report");

                            // for fail-fast, just break out of loop
                            if args.flag_fail_fast {
                                break;
                            }
                        }
                    }
                }
                Err(e) => {
                    return fail!(format!("Unable to validate. row: {row_index}, error: {e}"));
                }
            }

            if !args.flag_quiet {
                progress.inc(1);
            }
        } // end main while loop over csv records

        // flush error report; file gets closed automagically when out-of-scope
        error_report_file.flush().unwrap();

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
                row_index.separate_with_commas()
            );
            info!("{msg}");
            println!("{msg}");
        } else {
            let msg = format!(
                "{} out of {} records invalid.",
                invalid_count.separate_with_commas(),
                row_index.separate_with_commas()
            );
            info!("{msg}");
            println!("{msg}");
        }
    } else {
        // just read csv file and let csv reader report problems
        let mut record = csv::ByteRecord::new();
        while rdr.read_byte_record(&mut record)? {
            // this loop is for csv::reader to do basic csv validation on read
        }

        let msg = "Can't validate without schema, but csv looks good.".to_string();
        info!("{msg}");
        println!("{msg}");
    }

    Ok(())
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

    for (i, header) in headers_iter {
        // convert csv header to string
        let header_string = std::str::from_utf8(header)?.to_string();
        // convert csv value to string; trim whitespace
        let value_string = std::str::from_utf8(&record[i])?.trim().to_string();

        // get json type from schema; defaults to STRING if not specified
        let field_def = schema_properties
            .get(&header_string)
            .unwrap_or_else(|| &Value::Null);

        let field_type_def = field_def.get("type").unwrap_or_else(|| &Value::Null);

        let json_type = field_type_def.as_str().unwrap_or_else(|| "string");

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
            }
        })
    }

    #[test]
    fn test_to_json_instance() {
        let csv = "A,B,C,D,E,F,G,H
        hello,3.1415,300000000,true,,,,";

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
