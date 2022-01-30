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
use serde_json::{value::Number, Map, Value};
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

    let mut row_index: u32 = 0;
    let mut invalid_count: u32 = 0;

    // array of frequency tables
    let mut frequency_tables: Vec<_> = (0..(headers.len() as u32)).map(|_| Frequencies::<FieldType>::new()).collect();

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
                        frequency_tables[i].add(FieldType::TNull);
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

    // dbg!(&frequency_tables);

    // get most frequent type for each header column
    for (i, header) in headers_iter {
        let most_frequent = frequency_tables[i].most_frequent();
        let inferred_type = match most_frequent.get(0) {
            Some(tuple) => tuple,
            None => &(&FieldType::TNull, 0)
        };
        let header_string = std::str::from_utf8(header).unwrap().to_string();
        print!("{:?}: {:?}\n", header_string, inferred_type);
    }

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


