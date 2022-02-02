use crate::cmd::stats::FieldType;
use crate::config::{Config, Delimiter};
use crate::util;
use crate::CliError;
use crate::CliResult;
use csv::ByteRecord;
use indicatif::{ProgressBar, ProgressDrawTarget};
use log::{debug, error};
use serde::Deserialize;
use serde_json::{json, Map, Value};
use stats::Frequencies;
use std::{fs::File, io::Write, ops::Add};

macro_rules! fail {
    ($mesg:expr) => {
        return Err(CliError::Other($mesg));
    };
}

static USAGE: &str = "
Infer schema from CSV data and output in JSON Schema format.

Example output file from `mydata.csv`. If piped from stdin, then filename is `stdin.csv`.

* mydata.csv.schema.json

Usage:
    qsv schema [options] [<input>]

schema options:


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
    let mut record = ByteRecord::new();

    let mut row_index: u32 = 0;

    // array of frequency tables to track non-NULL type occurrences
    let mut frequency_tables: Vec<_> = (0..(headers.len() as u32))
        .map(|_| Frequencies::<FieldType>::new())
        .collect();
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
                    if !nullable_flags[col_index] {
                        frequency_tables[col_index].add(FieldType::TNull);
                    }
                    nullable_flags[col_index] = true;
                }
                FieldType::TUnknown => {
                    // default to String
                    frequency_tables[col_index].add(FieldType::TString);
                }
                x => {
                    frequency_tables[col_index].add(x);
                }
            }
        }

        if !args.flag_quiet {
            progress.inc(1);
        }
    } // end main while loop over csv records

    use thousands::Separable;

    if !args.flag_quiet {
        progress.set_message(format!(
            " processed {} records.",
            progress.length().separate_with_commas()
        ));
        util::finish_progress(&progress);
    }

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
        let header_string: String = match std::str::from_utf8(header) {
            Ok(s) => s.to_string(),
            Err(e) => {
                let msg = format!("Can't read header from column {i} as utf8: {e}");
                error!("{msg}");
                fail!(msg);
            }
        };

        let required: bool = *inferred_type != FieldType::TNull
            && *inferred_type != FieldType::TUnknown
            && count as u32 >= row_index;

        debug!("{header_string} has most frequent type of {inferred_type:?}, required={required}");

        // use list since optional columns get appended a "null" type
        let mut type_list: Vec<Value> = Vec::new();

        match inferred_type {
            FieldType::TString => {
                type_list.push(Value::String("string".to_string()));
            }
            FieldType::TDate => {
                type_list.push(Value::String("string".to_string()));
            }
            FieldType::TInteger => {
                type_list.push(Value::String("integer".to_string()));
            }
            FieldType::TFloat => {
                type_list.push(Value::String("number".to_string()));
            }
            FieldType::TNull => {
                type_list.push(Value::String("null".to_string()));
            }
            _ => {
                // defaults to JSON String
                type_list.push(Value::String("string".to_string()));
            }
        }

        // "null" type denotes optinal value
        // to be compatible with "validate" command, has to come after the real type, and only if type is not already JSON Null
        if !required && *inferred_type != FieldType::TNull {
            type_list.push(Value::String("null".to_string()));
        }

        let mut field_map: Map<String, Value> = Map::new();
        field_map.insert("type".to_string(), Value::Array(type_list));
        properties_map.insert(header_string, Value::Object(field_map));
    } // end for loop over all columns

    // create final JSON object for output
    let schema = json!({
        "$schema": "https://json-schema.org/draft-07/schema",
        "title": format!("JSON Schema for {input_path}"),
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

    Ok(())
}
