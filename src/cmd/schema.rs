use crate::config::{Config, Delimiter};
use crate::util;
use crate::CliError;
use crate::CliResult;
use crate::cmd::stats::{FieldType, FieldType::*};
use anyhow::{anyhow, Result};
use csv::ByteRecord;
use indicatif::{ProgressBar, ProgressDrawTarget};
use jsonschema::{output::BasicOutput, JSONSchema};
use log::{debug, info};
use serde::Deserialize;
use serde_json::{value::Number, Map, Value, json};
use stats::Frequencies;
use std::{env, fs::File, io::BufReader, io::BufWriter, io::Read, io::Write, ops::Add};

macro_rules! fail {
    ($mesg:expr) => {
        Err(CliError::Other($mesg))
    };
}

static USAGE: &str = "
Infer schmea from CSV data and output in JSON Schema format.

Example output file from `mydata.csv`. If piped from stdin, then filename is `stdin.csv`.

* mydata.csv.schema.json

Usage:
    qsv schema [options] [<input>]

fetch options:
    --no-nulls                 Skip NULL values in type inference

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
    flag_no_nulls: bool,
    flag_no_headers: bool,
    flag_delimiter: Option<Delimiter>,
    flag_quiet: bool,
    arg_input: Option<String>,
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

    let mut schema_output_file = File::create(input_path.to_owned() + ".schema.json")
            .expect("unable to create schema output file");

    // prep progress bar
    let progress = ProgressBar::new(0);
    if !args.flag_quiet {
        let record_count = util::count_rows(&rconfig.flexible(true));
        util::prep_progress(&progress, record_count);
    } else {
        progress.set_draw_target(ProgressDrawTarget::hidden());
    }

    // amortize memory allocation by reusing record
    #[allow(unused_assignments)]
    let mut record = csv::ByteRecord::new();

    let mut row_index: u64 = 0;

    // array of frequency tables to track non-NULL type occurrences
    let mut frequency_tables: Vec<_> = (0..(headers.len() as u32)).map(|_| Frequencies::<FieldType>::new()).collect();
    // array of boolean to track if column in NULLABLE
    let mut nullable_flags: Vec<bool> = vec![false; headers.len()];

    // iterate over each CSV field and determine type
    let headers_iter = headers.iter().enumerate();

    while rdr.read_byte_record(&mut record)? {
        row_index = row_index.add(1);

        // dbg!(&record);


        for (i, header) in headers_iter.clone() {
            // convert csv header to string
            let header_string = std::str::from_utf8(header).unwrap().to_string();
            // convert csv value to string; trim whitespace
            let value_string = std::str::from_utf8(&record[i]).unwrap().trim().to_string();

            let sample_type = FieldType::from_sample(&value_string.as_bytes());

            debug!("{}[{}]: val={}, type={}", &header_string, &row_index, &value_string, &sample_type);

            match sample_type {
                FieldType::TNull => {
                    if args.flag_no_nulls {
                        // skip
                        debug!("Skipped: {}[{}]", &header_string, &row_index);
                    } else {
                        // only count NULL once, so it dominate frequency table when value is optional
                        if nullable_flags[i] == false {
                            frequency_tables[i].add(FieldType::TNull);
                        }
                        nullable_flags[i] = true;
                    }
                }
                FieldType::TUnknown => {
                    // default to String
                    frequency_tables[i].add(FieldType::TUnicode);
                }
                x => {
                    frequency_tables[i].add(x);
                }
            }

        }

        if !args.flag_quiet {
            progress.inc(1);
        }
    } // end main while loop over csv records

    debug!("freq tables: {:?}", &frequency_tables);

    // map holds "properties" object of json schema
    let mut properties_map: Map<String, Value> = Map::new();

    // get most frequent type for each header column
    for (i, header) in headers_iter {
        let most_frequent = frequency_tables[i].most_frequent();
        let (inferred_type, count) = match most_frequent.get(0) {
            Some(tuple) => *tuple,
            None => (&FieldType::TNull, 0)
        };
        dbg!(&inferred_type, count, row_index);

        let header_string = std::str::from_utf8(header).unwrap().to_string();
        let required: bool = if *inferred_type != FieldType::TNull && 
                                *inferred_type != FieldType::TUnknown && 
                                count >= row_index {
                                    true
                                } else {
                                    false
                                };
        debug!("{}: {:?} {}\n", header_string, inferred_type, required);

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
        // to be compatible with "validate" command, has to come after the real type, and only if type is not NULL
        if !required && *inferred_type != FieldType::TNull { 
            type_list.push(Value::String("null".to_string()));
        }

        let mut field_map: Map<String, Value> = Map::new();
        field_map.insert("type".to_string(), Value::Array(type_list));
        properties_map.insert(header_string, Value::Object(field_map));
    } // end for loop over headers

    print!("\n");

    let properties = Value::Object(properties_map);

    let schema = json!({
        "$schema": "https://json-schema.org/draft-07/schema",
        "title": format!("JSON Schema for {}", input_path),
        "description": "Inferred JSON Schema from QSV schema command",
        "type": "object",
        "properties": properties
    });

    let schema_pretty = serde_json::to_string_pretty(&schema).expect("prettify schema json");

    println!("{}\n", &schema_pretty);

    schema_output_file
        .write_all(schema_pretty.as_bytes())
        .expect("unable to write schema file");

    // flush error report; file gets closed automagically when out-of-scope
    schema_output_file.flush().unwrap();

    use thousands::Separable;

    if !args.flag_quiet {
        progress.set_message(format!(
            " processed {} records.",
            progress.length().separate_with_commas()
        ));
        util::finish_progress(&progress);
    }

    Ok(())
}


