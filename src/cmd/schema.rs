use crate::config::{Config, Delimiter};
use crate::util;
use crate::CliError;
use crate::CliResult;
use crate::select::{SelectColumns, Selection};
use crate::cmd::stats::{FieldType};
use anyhow::{anyhow, Result};
use csv::ByteRecord;
use log::{debug, info, warn, error};
use serde::Deserialize;
use serde_json::{Map, Value, json};
use stats::Frequencies;
use std::{fs::File, path::Path, io::Write, ops::Add};

macro_rules! fail {
    ($mesg:expr) => {
        return Err(CliError::Other($mesg));
    };
}

static USAGE: &str = "
Infer schmea from CSV data and output in JSON Schema format.

Example output file from `mydata.csv`. If piped from stdin, then filename is `stdin.csv`.

* mydata.csv.schema.json

Usage:
    qsv schema [options] [<input>]

Schema options:
    --value-constraints        Add value constraints based on CSV data (enum, min, max, minLength, MaxLength, multipleOf)
    --enum-threshold NUM       Cardinality threshold for adding enum constraints [default: 30]
    --pattern-columns <args>   Select columns to add pattern constraints [default: none]

Common options:
    -h, --help                 Display this message
    -n, --no-headers           When set, the first row will not be interpreted
                               as headers. Namely, it will be sorted with the rest
                               of the rows. Otherwise, the first row will always
                               appear as the header row in the output.
    -d, --delimiter <arg>      The field delimiter for reading CSV data.
                               Must be a single character. [default: ,]
";

#[derive(Deserialize, Debug)]
struct Args {
    flag_value_constraints: bool,
    flag_enum_threshold: u32,
    flag_pattern_columns: SelectColumns,
    flag_no_headers: bool,
    flag_delimiter: Option<Delimiter>,
    arg_input: Option<String>,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;

    // dbg!(&args);

    let input_path = match &args.arg_input {
        Some(path) => path,
        None => "stdin.csv"
    };
    let input_filename: &str = match &args.arg_input {
        Some(path) => Path::new(path).file_name().unwrap().to_str().unwrap(),
        None => "stdin.csv"
    };

    let schema_output_filename = input_path.to_owned() + ".schema.json";
    let mut schema_output_file = File::create(&schema_output_filename)
            .expect("unable to create schema output file");

    let properties_map: Map<String, Value> = 
        if args.flag_value_constraints {
            match infer_schema_from_stats(&args, input_filename) {
                Ok(map) => map,
                Err(e) => {
                    let msg = format!("Failed to infer schema by running 'stats' command: {e}");
                    fail!(msg);
                }
            }
        } else {
            match infer_schema_simple_frequency(&args, input_filename) {
                Ok(map) => map,
                Err(e) => {
                    let msg = format!("Failed to infer schema via simple frequency: {e}");
                    error!("{msg}");
                    fail!(msg);
                }
            }
        };

    let mut fields: Vec<Value> = Vec::new();
    for key in properties_map.keys() {
        fields.push(Value::String(key.clone()));
    };

    // create final JSON object for output
    let schema = json!({
        "$schema": "https://json-schema.org/draft-07/schema",
        "title": format!("JSON Schema for {input_filename}"),
        "description": "Inferred JSON Schema from QSV schema command",
        "type": "object",
        "properties": Value::Object(properties_map),
        "required": Value::Array(fields)
    });

    let schema_pretty = serde_json::to_string_pretty(&schema).expect("prettify schema json");

    schema_output_file
        .write_all(schema_pretty.as_bytes())
        .expect("unable to write schema file");

    // flush error report; file gets closed automagically when out-of-scope
    schema_output_file.flush().unwrap();

    println!("Schema written to {schema_output_filename}");

    Ok(())
}

fn infer_schema_from_stats(args: &Args, input_filename: &str) -> CliResult<Map<String, Value>> {
    let stats_args = crate::cmd::stats::Args {
        arg_input: args.arg_input.clone(),
        flag_select: crate::select::SelectColumns::parse("").unwrap(),
        flag_everything: false,
        flag_mode: false,
        flag_cardinality: true,
        flag_median: false,
        flag_quartiles: false,
        flag_nulls: false,
        flag_nullcount: true,
        flag_jobs: 3,
        flag_output: None,
        flag_no_headers: args.flag_no_headers,
        flag_delimiter: args.flag_delimiter
    };

    let (stats_headers, stats_list) = match stats_args.rconfig().indexed() {
        Ok(o) => 
            match o {
                None => {
                    info!("no index, triggering sequential stats");
                    stats_args.sequential_stats()
                },
                Some(idx) => {
                    info!("has index, triggering parallel stats");
                    stats_args.parallel_stats(idx)
                }
            },
        Err(e) => {
            warn!("error determining if indexed, triggering sequential stats: {e}");
            stats_args.sequential_stats()
        }

    }?;

    debug!("stat headers: {stats_headers:?}");

    for stat in &stats_list {
        debug!("stat rec: {:?}", stat.clone().to_record());
    };

    // map holds "properties" object of json schema
    let mut properties_map: Map<String, Value> = Map::new();

    for i in 0..stats_headers.len() {
        let header = stats_headers.get(i).unwrap();
        // convert csv header to string
        let header_string: String = match std::str::from_utf8(header){
            Ok(s) => {
                s.to_string()
            },
            Err(e) => {
                fail!(format!("Can't read header from column {i} as utf8: {e}"));
            }
        };

        // grab stats record for current column
        let col_stats_record = stats_list.get(i).unwrap().clone().to_record();
        // get Type from 1st column of stats record
        let col_type = col_stats_record.get(0).unwrap();
        // get NullCount from 11th column
        let col_null_count = match col_stats_record.get(10) {
            Some(s) => {
                debug!("null count: {s}");
                s.parse::<u32>().unwrap_or(0_u32)
            },
            None => {
                0_u32
            }
        };
        // get Cardinality from 10th column
        let col_cardinality = match col_stats_record.get(9) {
            Some(s) => {
                debug!("cardinality: {s}");
                s.parse::<u32>().unwrap_or(0_u32)
            },
            None => {
                0_u32
            }
        };


        debug!("{header_string} has most frequent type of {col_type}, cardinality={col_cardinality}, optional={}", col_null_count>0);

        // use list since optional columns get appended a "null" type
        let mut type_list: Vec<Value> = Vec::new();
        
        match col_type {
            "Unicode" => {
                type_list.push(Value::String("string".to_string()));
            },
            "Date" => {
                type_list.push(Value::String("string".to_string()));
            },
            "Integer" => {
                type_list.push(Value::String("integer".to_string()));
            },
            "Float" => {
                type_list.push(Value::String("number".to_string()));
            },
            "NULL" => {
                type_list.push(Value::String("null".to_string()));
            },
            _ => {
                // defaults to JSON String
                type_list.push(Value::String("string".to_string()));
            },
        }

        // "null" type denotes optinal value
        // to be compatible with "validate" command, has to come after the real type, and only once
        if col_null_count>0 && !type_list.contains(&Value::String("null".to_string())) { 
            type_list.push(Value::String("null".to_string()));
        }

        let mut field_map: Map<String, Value> = Map::new();
        field_map.insert("type".to_string(), Value::Array(type_list));
        let desc = format!("{header_string} column from {input_filename}");
        field_map.insert("description".to_string(), Value::String(desc));



        properties_map.insert(header_string, Value::Object(field_map));
    }

    Ok(properties_map)
}

fn infer_schema_simple_frequency(args: &Args, input_filename: &str) -> Result<Map<String, Value>> {

    let rconfig = Config::new(&args.arg_input)
    .delimiter(args.flag_delimiter)
    .no_headers(args.flag_no_headers);

    let mut rdr = rconfig.reader()?;
    let headers = rdr.byte_headers()?.clone();

    // amortize memory allocation by reusing record
    #[allow(unused_assignments)]
    let mut record = ByteRecord::new();

    let mut row_index: u32 = 0;

    // array of frequency tables to track non-NULL type occurrences
    let mut frequency_tables: Vec<_> = (0..(headers.len() as u32)).map(|_| Frequencies::<FieldType>::new()).collect();
    // array of boolean to track if column is NULLABLE
    let mut nullable_flags: Vec<bool> = vec![false; headers.len()];

    // iterate over each CSV field and determine type
    let headers_iter = headers.iter().enumerate();

    while rdr.read_byte_record(&mut record)? {
        row_index = row_index.add(1);

        // dbg!(&record);

        for col_index in 0..headers.len() {

            // since from_sample() parses byte slice to string, no need to do it here
            let value_slice: &[u8] = &record[col_index];

            let inferred_type: FieldType = FieldType::from_sample(value_slice);

            // debug!("column_{col_index}[{row_index}]: val={value_slice:?}, type={inferred_type}");

            // update frequency table for this column
            match inferred_type {
                FieldType::TNull => {

                    // only count NULL once, so it won't dominate frequency table when value is optional
                    if nullable_flags[col_index] == false {
                        frequency_tables[col_index].add(FieldType::TNull);
                    }
                    nullable_flags[col_index] = true;

                }
                FieldType::TUnknown => {
                    // default to String
                    frequency_tables[col_index].add(FieldType::TUnicode);
                }
                x => {
                    frequency_tables[col_index].add(x);
                }
            }

        }

    } // end main while loop over csv records

    debug!("freq tables: {frequency_tables:?}");

    // map holds "properties" object of json schema
    let mut properties_map: Map<String, Value> = Map::new();

    // iterate through headers again and get most frequent type for each column
    for (i, header) in headers_iter {
        let most_frequent = frequency_tables[i].most_frequent();
        let (inferred_type, _count) = match most_frequent.get(0) {
            Some(tuple) => *tuple,
            None => {
                // not good, no type info for this column
                return Err(anyhow!("Cannot determine type for column '{header:?}'"));
            }
        };

        // convert csv header to string
        let header_string: String = match std::str::from_utf8(header){
            Ok(s) => {
                s.to_string()
            },
            Err(e) => {
                return Err(anyhow!("Can't read header from column {i} as utf8: {e}"));
            }
        };

        debug!("{header_string} has most frequent type of {inferred_type:?}, optional={}", nullable_flags[i]);

        // use list since optional columns get appended a "null" type
        let mut type_list: Vec<Value> = Vec::new();
        
        match inferred_type {
            FieldType::TUnicode => {
                type_list.push(Value::String("string".to_string()));
            },
            FieldType::TDate => {
                type_list.push(Value::String("string".to_string()));
            },
            FieldType::TInteger => {
                type_list.push(Value::String("integer".to_string()));
            },
            FieldType::TFloat => {
                type_list.push(Value::String("number".to_string()));
            },
            FieldType::TNull => {
                type_list.push(Value::String("null".to_string()));
            },
            _ => {
                // defaults to JSON String
                type_list.push(Value::String("string".to_string()));
            },
        }

        // "null" type denotes optinal value
        // to be compatible with "validate" command, has to come after the real type, and only once
        if nullable_flags[i] && !type_list.contains(&Value::String("null".to_string())) { 
            type_list.push(Value::String("null".to_string()));
        }

        let mut field_map: Map<String, Value> = Map::new();
        field_map.insert("type".to_string(), Value::Array(type_list));
        let desc = format!("{header_string} column from {input_filename}");
        field_map.insert("description".to_string(), Value::String(desc));
        properties_map.insert(header_string, Value::Object(field_map));

    } // end for loop over all columns

    Ok(properties_map)
}
