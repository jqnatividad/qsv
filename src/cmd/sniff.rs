use crate::config::Config;
use crate::util;
use crate::CliResult;
use csv_sniffer::{SampleSize, Sniffer};
use serde::{Deserialize, Serialize};
use serde_json::json;
use thousands::Separable;

static USAGE: &str = r#"
Quickly sniff CSV details (delimiter, quote character, number of fields, data types,
header row, preamble rows).

NOTE: sniff is a thin wrapper around the csv-sniffer crate (https://docs.rs/csv-sniffer).
It "sniffs" a CSV's schema by scanning the first few rows of a CSV file, and its inferences
are sometimes wrong. If you want more robust, guaranteed schemata - use the "schema" or
"stats" commands instead. 

Usage:
    qsv sniff [options] [<input>]

sniff options:
    -l, --len <arg>        How many rows to sample to sniff out the details.
                           [default: 100]
    --json                 Return results in JSON format.
    --pretty-json          Return results in pretty JSON format.

Common options:
    -h, --help             Display this message
"#;

#[derive(Deserialize)]
struct Args {
    arg_input: Option<String>,
    flag_len: usize,
    flag_json: bool,
    flag_pretty_json: bool,
}

#[derive(Serialize, Deserialize)]
struct SniffStruct {
    delimiter_char: char,
    header_row: bool,
    preamble_rows: usize,
    quote_char: String,
    num_records: u64,
    num_fields: usize,
    types: Vec<String>,
}

fn rowcount(conf: &Config, metadata: &csv_sniffer::metadata::Metadata) -> u64 {
    let has_header_row = metadata.dialect.header.has_header_row;
    let num_preamble_rows = metadata.dialect.header.num_preamble_rows;
    let mut final_rowcount = util::count_rows(conf);

    if !has_header_row {
        final_rowcount += 1;
    }

    final_rowcount -= num_preamble_rows as u64;
    final_rowcount
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;

    let conf = Config::new(&args.arg_input).flexible(true);
    let rdr = conf.reader_file_stdin()?;

    let sniff_results = Sniffer::new()
        .sample_size(SampleSize::Records(args.flag_len))
        .sniff_reader(rdr.into_inner());

    if args.flag_json || args.flag_pretty_json {
        match sniff_results {
            Ok(metadata) => {
                let mut sniffedtypes: Vec<String> = Vec::with_capacity(metadata.num_fields);
                for ty in &metadata.types {
                    sniffedtypes.push(ty.to_string());
                }

                let sniffed = SniffStruct {
                    delimiter_char: metadata.dialect.delimiter as char,
                    header_row: metadata.dialect.header.has_header_row,
                    preamble_rows: metadata.dialect.header.num_preamble_rows,
                    quote_char: match metadata.dialect.quote {
                        csv_sniffer::metadata::Quote::Some(chr) => format!("{}", char::from(chr)),
                        csv_sniffer::metadata::Quote::None => "none".into(),
                    },
                    num_records: rowcount(&conf, &metadata),
                    num_fields: metadata.num_fields,
                    types: sniffedtypes,
                };
                if args.flag_pretty_json {
                    println!("{}", serde_json::to_string_pretty(&sniffed).unwrap());
                } else {
                    let json_result = serde_json::to_string(&sniffed).unwrap();
                    println!("{json_result}");
                };
            }
            Err(e) => {
                let json_result = json!({
                    "errors": [{
                        "title": "sniff error",
                        "detail": e.to_string()
                    }]
                });
                return fail!(format!("{json_result}"));
            }
        }
    } else {
        match sniff_results {
            Ok(metadata) => {
                let full_metadata = format!("{}", metadata);
                // show otherwise invisible tab character as "tab"
                let mut disp = full_metadata.replace("\tDelimiter: \t", "\tDelimiter: tab");
                // remove Dialect header
                disp = disp.replace("Dialect:\n", "");
                // add number of records if not stdin, where we can count rows
                let num_rows = rowcount(&conf, &metadata);
                if num_rows > 0 {
                    let rows_str = format!(
                        "\nNumber of records: {}\nNumber of fields:",
                        num_rows.separate_with_commas()
                    );
                    disp = disp.replace("\nNumber of fields:", &rows_str);
                }
                println!("{disp}");
            }
            Err(e) => {
                return fail!(format!("sniff error: {e}"));
            }
        }
    }

    Ok(())
}
