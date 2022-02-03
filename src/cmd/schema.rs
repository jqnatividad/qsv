use crate::config::{Config, Delimiter};
use crate::util;
use crate::CliError;
use crate::CliResult;
use crate::cmd::stats::{FieldType};
use csv::ByteRecord;
use log::{debug, error};
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
    --add-constraints-from-stats   Include constraints based on CSV stats (min/max/cardinality, etc)

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
    flag_add_constraints_from_stats: bool,
    flag_no_headers: bool,
    flag_delimiter: Option<Delimiter>,
    arg_input: Option<String>,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;

    // dbg!(&args);

    let rconfig = Config::new(&args.arg_input)
        .delimiter(args.flag_delimiter)
        .no_headers(args.flag_no_headers);

    if args.flag_add_constraints_from_stats {

        let stats_args = crate::cmd::stats::Args {
            arg_input: args.arg_input.clone(),
            flag_select: crate::select::SelectColumns::parse("").unwrap(),
            flag_everything: false,
            flag_mode: false,
            flag_cardinality: true,
            flag_median: false,
            flag_quartiles: false,
            flag_nulls: false,
            flag_nullcount: false,
            flag_jobs: 2,
            flag_output: None,
            flag_no_headers: args.flag_no_headers,
            flag_delimiter: args.flag_delimiter
        };

        let (stats_headers, stats) = match stats_args.rconfig().indexed()? {
            None => stats_args.sequential_stats(),
            Some(idx) => {
                stats_args.parallel_stats(idx)
            }
        }?;

        dbg!(stats_headers);
        for mut stat in stats {
            dbg!(stat.to_record());
        };
    }

    let mut rdr = rconfig.reader()?;

    let headers = rdr.byte_headers()?.clone();

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

            debug!("column_{col_index}[{row_index}]: val={value_slice:?}, type={inferred_type}");

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
        let (inferred_type, count) = match most_frequent.get(0) {
            Some(tuple) => *tuple,
            None => {
                // not good, no type info for this column
                let msg = format!("Cannot determine type for column '{header:?}'");
                error!("{msg}");
                fail!(msg);
            }
        };

        // convert csv header to string
        let header_string: String = match std::str::from_utf8(header){
            Ok(s) => {
                s.to_string()
            },
            Err(e) => {
                let msg = format!("Can't read header from column {i} as utf8: {e}");
                error!("{msg}");
                fail!(msg);
            }
        };

        let required: bool = if *inferred_type != FieldType::TNull && 
                                *inferred_type != FieldType::TUnknown && 
                                count as u32 >= row_index {
                                    true
                                } else {
                                    false
                                };

        debug!("{header_string} has most frequent type of {inferred_type:?}, required={required}");

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
        // to be compatible with "validate" command, has to come after the real type, and only if type is not already JSON Null
        if !required && *inferred_type != FieldType::TNull { 
            type_list.push(Value::String("null".to_string()));
        }

        let mut field_map: Map<String, Value> = Map::new();
        field_map.insert("type".to_string(), Value::Array(type_list));
        let desc = format!("{header_string} column from {input_filename}");
        field_map.insert("description".to_string(), Value::String(desc));
        properties_map.insert(header_string, Value::Object(field_map));

    } // end for loop over all columns

    // create final JSON object for output
    let schema = json!({
        "$schema": "https://json-schema.org/draft-07/schema",
        "title": format!("JSON Schema for {input_filename}"),
        "description": "Inferred JSON Schema from QSV schema command",
        "type": "object",
        "properties": Value::Object(properties_map)
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


