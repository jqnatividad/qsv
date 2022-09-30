static USAGE: &str = r#"
Quickly sniff CSV metadata (delimiter, header row, preamble rows, quote character, 
flexible, is_utf8, number of records, number of fields, field names & data types).

NOTE: This command "sniffs" a CSV's schema by sampling the first n rows of a file.
Its inferences are sometimes wrong if the sample is not large enough (use --sample 
to adjust). 

If you want more robust, guaranteed schemata, use the "schema" or "stats" commands
instead as they scan the entire file.

Usage:
    qsv sniff [options] [<input>]
    qsv sniff --help

sniff options:
    --sample <size>        First n rows to sample to sniff out the metadata.
                           When sample size is between 0 and 1 exclusive, 
                           it is treated as a percentage of the CSV to sample
                           (e.g. 0.20 is 20 percent).
                           When it is zero, the entire file will be sampled.
                           [default: 100]
    --prefer-dmy           Prefer to parse dates in dmy format.
                           Otherwise, use mdy format.
    --json                 Return results in JSON format.
    --pretty-json          Return results in pretty JSON format.

Common options:
    -h, --help             Display this message
"#;

use qsv_sniffer::{DatePreference, SampleSize, Sniffer};
use serde::{Deserialize, Serialize};
use serde_json::json;
use thousands::Separable;

use crate::{config::Config, util, CliResult};

#[derive(Deserialize)]
struct Args {
    arg_input:        Option<String>,
    flag_sample:      f64,
    flag_prefer_dmy:  bool,
    flag_json:        bool,
    flag_pretty_json: bool,
}

#[derive(Serialize, Deserialize)]
struct SniffStruct {
    delimiter_char: char,
    header_row:     bool,
    preamble_rows:  usize,
    quote_char:     String,
    flexible:       bool,
    is_utf8:        bool,
    num_records:    u64,
    num_fields:     usize,
    fields:         Vec<String>,
    types:          Vec<String>,
}

const fn rowcount(metadata: &qsv_sniffer::metadata::Metadata, rowcount: u64) -> u64 {
    let has_header_row = metadata.dialect.header.has_header_row;
    let num_preamble_rows = metadata.dialect.header.num_preamble_rows;
    let mut final_rowcount = rowcount;

    if !has_header_row {
        final_rowcount += 1;
    }

    final_rowcount -= num_preamble_rows as u64;
    final_rowcount
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;

    let conf = Config::new(&args.arg_input).flexible(true).checkutf8(false);
    let n_rows = util::count_rows(&conf)?;

    let mut sample_size = args.flag_sample;
    let mut sample_all = false;
    // its a percentage, get the actual sample size
    #[allow(clippy::cast_precision_loss)]
    if sample_size < 1.0 {
        sample_size *= n_rows as f64;
    } else if (sample_size - 0.0).abs() < f64::EPSILON {
        // its zero, the epsilon bit is because comparing a float
        // is really not precise - see https://floating-point-gui.de/errors/comparison/
        sample_all = true;
    }

    let rdr = conf.reader_file_stdin()?;

    let dt_preference = if args.flag_prefer_dmy || conf.get_dmy_preference() {
        DatePreference::DmyFormat
    } else {
        DatePreference::MdyFormat
    };

    let sniff_results = if sample_all {
        log::info!("Sniffing ALL {n_rows} rows...");
        Sniffer::new()
            .sample_size(SampleSize::All)
            .date_preference(dt_preference)
            .sniff_reader(rdr.into_inner())
    } else {
        let mut sniff_size = sample_size as usize;
        // sample_size is at least 10
        if sniff_size < 10 {
            sniff_size = 10;
        }
        log::info!("Sniffing {sniff_size} of {n_rows} rows...");
        Sniffer::new()
            .sample_size(SampleSize::Records(sniff_size))
            .date_preference(dt_preference)
            .sniff_reader(rdr.into_inner())
    };

    if args.flag_json || args.flag_pretty_json {
        match sniff_results {
            Ok(metadata) => {
                let sniffedfields = metadata
                    .fields
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect();
                let sniffedtypes = metadata
                    .types
                    .iter()
                    .map(std::string::ToString::to_string)
                    .collect();

                let sniffed = SniffStruct {
                    delimiter_char: metadata.dialect.delimiter as char,
                    header_row:     metadata.dialect.header.has_header_row,
                    preamble_rows:  metadata.dialect.header.num_preamble_rows,
                    quote_char:     match metadata.dialect.quote {
                        qsv_sniffer::metadata::Quote::Some(chr) => format!("{}", char::from(chr)),
                        qsv_sniffer::metadata::Quote::None => "none".into(),
                    },
                    flexible:       metadata.dialect.flexible,
                    is_utf8:        metadata.dialect.is_utf8,
                    num_records:    rowcount(&metadata, n_rows),
                    num_fields:     metadata.num_fields,
                    fields:         sniffedfields,
                    types:          sniffedtypes,
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
                return fail_format!("{json_result}");
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
                let num_rows = rowcount(&metadata, n_rows);
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
                return fail_format!("sniff error: {e}");
            }
        }
    }

    Ok(())
}
