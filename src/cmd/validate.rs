use crate::config::{Config, Delimiter};
use crate::util;
use crate::CliResult;
use crate::CliError;
use cached::proc_macro::cached;
use indicatif::{ProgressBar, ProgressDrawTarget};
use csv::ByteRecord;
use serde_json::{Deserializer, Value, Map, value::Number};
use jsonschema::{JSONSchema, output::BasicOutput};
use serde::Deserialize;
use anyhow::{Result, anyhow};
use log::{debug, warn, error};
use std::{fs::File, io::BufReader, io::Read, path::Path, net::TcpStream, collections::HashMap};

static USAGE: &str = "
Validate CSV data with JSON Schema, and put invalid records into separate file.

Example output files from `mydata.csv`. If piped from stdin, then filename is `stdin.csv`.

* mydata.csv.valid
* mydata.csv.invalid
* mydata.csv.validation-report

JSON Schema can be a local file or a URL. 

When run without JSON Schema, only a simple CSV check (RFC 4180) is performed, with the caveat that 
 on non-Windows machines, each record is delimited by a CR (\n) instead of CRLF (\n\r).


Usage:
    qsv validate [options] [<input>] [<json-schema>]

fetch options:
    --fail-fast                Stops on first error.
    --valid <suffix>           Valid record output file suffix [default: valid]
    --invalid <suffix>         Invalid record output file suffix [default: invalid]


Common options:
    -h, --help                 Display this message
    -n, --no-headers           When set, the first row will not be interpreted
                               as headers. Namely, it will be sorted with the rest
                               of the rows. Otherwise, the first row will always
                               appear as the header row in the output.
    -d, --delimiter <arg>      The field delimiter for reading CSV data.
                               Must be a single character. (default: ,)
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

    let input_path: &str = &args.arg_input.unwrap_or("stdin.csv".to_string());

    let valid_suffix: &str = &args.flag_valid.unwrap_or("valid".to_string());
    let mut valid_wtr = Config::new(&Some(input_path.to_owned() + "." + valid_suffix)).writer()?;

    let invalid_suffix: &str = &args.flag_invalid.unwrap_or("invalid".to_string());
    let mut invalid_wtr = Config::new(&Some(input_path.to_owned() + "." + invalid_suffix)).writer()?;


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

        let (schema_json, schema_compiled):(Value, JSONSchema) = match load_json(json_schema_uri) {
            Ok(s) =>  {
                match serde_json::from_str(&s) {
                    Ok(json) => {
                        match JSONSchema::options().compile(&json) {
                            Ok(schema) => (json, schema),
                            Err(e) => {
                                return Err(CliError::Other(format!("Not a valid JSONSchema: {}", json_schema_uri)))
                            }
                        }
                    },
                    Err(e)=> {
                        return Err(CliError::Other(format!("Unable to parse json from: {}", json_schema_uri)))
                    }
                }
            }
            Err(e) => {
                return Err(CliError::Other(format!("Unable to retrieve json from: {}", json_schema_uri)));
            }
        };
        // dbg!(&schema_compiled);

        let mut valid_file_empty: bool = true;
        let mut invalid_file_empty: bool = true;
    
        // amortize memory allocation by reusing record
        #[allow(unused_assignments)]
        let mut record = csv::ByteRecord::new();
    
        while rdr.read_byte_record(&mut record)? {

    
            let instance: Value = match to_json_instance(&headers, &record, &schema_json) {
                Ok(obj) => obj,
                Err(e) => {
                    return Err(CliError::Other(format!("Unable to convert CSV to json. record: {:?}, schema: {:?}", 
                                                    &record, 
                                                    &schema_json)));
                }
            };

            match validate_json_instance(&instance, &schema_compiled) {

                Ok(validation_result) => {

                    let results = &validation_result["valid"];

                    let valid_flag = match results.as_bool() {
                        Some(b) => b,
                        None => {
                            return Err(CliError::Other(format!("Unexpected validation result. {:?}", 
                                                    results)));
                        }
                    };

                    match valid_flag {
                        true => {
                            if valid_file_empty {
                                valid_wtr.write_byte_record(&headers)?;
                                valid_file_empty = false;
                            }

                            valid_wtr.write_byte_record(&record)?;
                        },
                        false => {
                            if invalid_file_empty {
                                invalid_wtr.write_byte_record(&headers)?;
                                invalid_file_empty = false;
                            }

                            invalid_wtr.write_byte_record(&record)?;
                        },
                    }
                },
                Err(e) => {
                    return Err(CliError::Other(format!("Unable to validate. error: {:?}", e)));
                }

            }

            if !args.flag_quiet {
                progress.inc(1);
            }
            if !args.flag_quiet {
                use cached::Cached;
                use thousands::Separable;
        
                let cache = GET_CACHED_RESPONSE.lock().unwrap();
                let cache_size = cache.cache_size();
                let hits = cache.cache_hits().unwrap();
                let misses = cache.cache_misses().unwrap();
                let hit_ratio = (hits as f64 / (hits + misses) as f64) * 100.0;
                progress.set_message(format!(
                    " of {} records. Cache hit ratio: {:.2}% - {} entries",
                    progress.length().separate_with_commas(),
                    hit_ratio,
                    cache_size.separate_with_commas(),
                ));
                util::finish_progress(&progress);
            }
        }
    } else {
        // just read csv file and let csv reader report problems
        let mut record = csv::ByteRecord::new();
        while rdr.read_byte_record(&mut record)? {
            // this loop is for csv::reader to do basic csv validation on read
        }
    }


    Ok(())
}

use serde_json::json;

/// convert CSV Record into JSON instance by referencing Type from Schema
fn to_json_instance(headers:&ByteRecord, record: &ByteRecord, schema: &Value) -> Result<Value> {

    // grab Type from Schema, and convert CSV field accordingly
    if let Some(schema_map) = schema["properties"].as_object() {

        // map holds individual CSV fields converted as serde_json::Value
        let mut json_object_map: Map<String,Value> = Map::new();

        // iterate over each CSV field and convert to JSON type
        let mut headers_iter = headers.iter().enumerate();

        while let Some((i, header)) = headers_iter.next() {
            // convert csv header to string
            let header_string = std::str::from_utf8(header)?.to_string();
            // convert csv value to string; trim whitespace
            let value_string = std::str::from_utf8(&record[i])?.trim().to_string();
            // get json type from schema
            let json_type = &schema_map[&header_string]["type"].as_str();

            // dbg!(i, &header_string, &value_string, &json_type);

            match json_type {
                Some("string") => {
                    json_object_map.insert(header_string, Value::String(value_string));
                },
                Some("number") => {
                    if let Ok(float) = value_string.parse::<f64>() {
                        json_object_map.insert(header_string, Value::Number(Number::from_f64(float).expect("not a valid f64 float")));
                    } else {
                        return Err(anyhow!("Can't cast into Float. header: {:?}, value: {:?}, json type: {:?}",
                                &header_string,
                                &value_string,
                                &json_type));
                    }
                },
                Some("integer") => {
                    if let Ok(int) = value_string.parse::<i64>() {
                        json_object_map.insert(header_string, Value::Number(Number::from(int)));
                    } else {
                        return Err(anyhow!("Can't cast into Integer. header: {:?}, value: {:?}, json type: {:?}",
                                &header_string,
                                &value_string,
                                &json_type));
                    }
                },
                Some("boolean") => {
                    if let Ok(boolean) = value_string.parse::<bool>() {
                        json_object_map.insert(header_string, Value::Bool(boolean));
                    } else {
                        return Err(anyhow!("Can't cast into Boolean. header: {:?}, value: {:?}, json type: {:?}",
                                &header_string,
                                &value_string,
                                &json_type));
                    }
                },
                None => {
                    return Err(anyhow!("Missing JSON type in schema. header: {:?}, value: {:?}, json type: {:?}",
                                &header_string,
                                &value_string,
                                &json_type));
                },
                _ => {
                    return Err(anyhow!("Unsupported JSON type. header: {:?}, value: {:?}, json type: {:?}",
                                &header_string,
                                &value_string,
                                &json_type));
                }
            }
        }

        // dbg!(&json_object_map);

        Ok(Value::Object(json_object_map))

    } else {
        // can't use schema to determine field type...abort
        Err(anyhow!("Unable to get Type info from JSON schema."))
    }

}

#[test]
fn test_to_json_instance() {

    // from https://json-schema.org/learn/miscellaneous-examples.html
    let schema_json = serde_json::json!({
        "$id": "https://example.com/person.schema.json",
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "title": "Person",
        "type": "object",
        "properties": {
          "firstName": {
            "type": "string",
            "description": "The person's first name."
          },
          "lastName": {
            "type": "string",
            "description": "The person's last name.",
            "minLength": 2
          },
          "age": {
            "description": "Age in years which must be equal to or greater than 18.",
            "type": "integer",
            "minimum": 18
          }
        }
      });

    let csv = 
      "firstName,lastName,age
      John,Doe,21";

    let mut rdr = csv::Reader::from_reader(csv.as_bytes());
    let headers = rdr.byte_headers().unwrap().clone();

    if let Some(r) = rdr.byte_records().next() {
        let record = r.unwrap();

        let instance = to_json_instance(&headers, &record, &schema_json).unwrap();

        assert_eq!(
            instance,
            json!({
                "firstName": "John",
                "lastName": "Doe",
                "age": 21
            })
        );
    }

}

/// Validate JSON instance against compiled JSON schema
fn validate_json_instance(instance: &Value, schema_compiled: &JSONSchema) -> Result<Value> {

    let output: BasicOutput = schema_compiled.apply(instance).basic();

    match serde_json::to_value(output) {
        Ok(json) => Ok(json),
        Err(e) => {
            Err(anyhow!("Cannot convert schema validation output to json: {}", e))
        }
    }
}

#[test]
fn test_validate_json_instance() {

    // from https://json-schema.org/learn/miscellaneous-examples.html
    let schema_json = serde_json::json!({
        "$id": "https://example.com/person.schema.json",
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "title": "Person",
        "type": "object",
        "properties": {
          "firstName": {
            "type": "string",
            "description": "The person's first name."
          },
          "lastName": {
            "type": "string",
            "description": "The person's last name.",
            "minLength": 2
          },
          "age": {
            "description": "Age in years which must be equal to or greater than 18.",
            "type": "integer",
            "minimum": 18
          }
        }
      });

      let schema = JSONSchema::options()
      .compile(&schema_json)
      .expect("A valid schema");


    let csv = 
      "firstName,lastName,age
      John,Doe,21
      Mickey,Mouse,10
      Little,A,16";

    let mut rdr = csv::Reader::from_reader(csv.as_bytes());
    let headers = rdr.byte_headers().unwrap().clone();

    for r in rdr.byte_records() {
        let record = r.unwrap();

        let instance = to_json_instance(&headers, &record, &schema_json).unwrap();

        let result = validate_json_instance(&instance, &schema);

        // dbg!(result);

    }

}


fn load_json(uri: &String) -> Result<String> {

    let json_string = match uri {
        url if url.starts_with("http") => {
	    // dbg!(&url);
            let response = reqwest::blocking::get(url)?;
            response.text()?
        },
        path => {
	    // dbg!(&path);
            let mut buffer = String::new();
            BufReader::new(File::open(uri)?).read_to_string(&mut buffer);
            buffer
        }
    };

    // dbg!(&json_string);


    Ok(json_string)
}

#[test]
fn test_load_json_via_url() {
    let json_string_result = load_json(&("https://geojson.org/schema/FeatureCollection.json".to_owned()));
    assert!(&json_string_result.is_ok());

    let json_result: Result<Value, serde_json::Error> = serde_json::from_str(&json_string_result.unwrap());
    assert!(&json_result.is_ok());
}

#[cached(
    key = "String",
    convert = r#"{ format!("{}", field) }"#,
    sync_writes = false
)]
fn get_cached_response(field: String) -> String {

    String::default()
}

