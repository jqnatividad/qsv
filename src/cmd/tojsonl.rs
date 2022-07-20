use indexmap::IndexMap;
use std::{fs::File, path::Path};

use crate::config::{Config, Delimiter};
use crate::util;
use crate::CliResult;
use log::{debug, error, info, warn};
use serde::Deserialize;
use serde_json::{json, value::Number, Map, Value};

use super::schema::infer_schema_from_stats;

static USAGE: &str = "
Converts CSV to a newline-delimited JSON (JSONL/NDJSON).

Usage:
    qsv tojsonl [options] [<input>]
    qsv tojsonl --help

Common options:
    -h, --help             Display this message
    -d, --delimiter <arg>  The field delimiter for reading CSV data.
                           Must be a single character. (default: ,)
    -o, --output <file>    Write output to <file> instead of stdout.
";

const STDIN_CSV: &str = "stdin.csv";
#[derive(Deserialize, Clone)]
struct Args {
    arg_input: Option<String>,
    flag_delimiter: Option<Delimiter>,
    flag_output: Option<String>,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let preargs: Args = util::get_args(USAGE, argv)?;
    let mut args = preargs.clone();
    let conf = Config::new(&args.arg_input).delimiter(args.flag_delimiter);

    // if using stdin, we create a stdin.csv file as stdin is not seekable and we need to
    // open the file multiple times to compile stats/unique values, etc.
    let (input_path, input_filename) = if preargs.arg_input.is_none() {
        let mut stdin_file = File::create(STDIN_CSV)?;
        let stdin = std::io::stdin();
        let mut stdin_handle = stdin.lock();
        std::io::copy(&mut stdin_handle, &mut stdin_file)?;
        args.arg_input = Some(STDIN_CSV.to_string());
        (STDIN_CSV.to_string(), STDIN_CSV.to_string())
    } else {
        let filename = Path::new(args.arg_input.as_ref().unwrap())
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string();
        (args.arg_input.clone().unwrap(), filename)
    };

    let schema_args = crate::cmd::schema::Args {
        flag_enum_threshold: 50,
        flag_strict_dates: false,
        flag_pattern_columns: crate::select::SelectColumns::parse("").unwrap(),
        flag_dates_whitelist: "none".to_string(),
        flag_prefer_dmy: std::env::var("QSV_PREFER_DMY").is_ok(),
        flag_stdout: false,
        flag_jobs: Some(util::njobs(Some(util::max_jobs()))),
        flag_no_headers: false,
        flag_delimiter: args.flag_delimiter,
        arg_input: args.arg_input.clone(),
    };

    // build schema for each field by their inferred type, min/max value/length, and unique values
    let mut properties_map: Map<String, Value> =
        match infer_schema_from_stats(&schema_args, &input_filename) {
            Ok(map) => map,
            Err(e) => {
                let msg = format!("Failed to infer schema via stats and frequency: {e}");
                return fail!(msg);
            }
        };

    debug!("{properties_map:?}");

    let mut rdr = conf.reader()?;
    let mut wtr = Config::new(&args.flag_output)
        .flexible(true)
        .no_headers(true)
        .quote_style(csv::QuoteStyle::Never)
        .writer()?;

    let headers = rdr.headers()?.clone();
    let mut record = csv::StringRecord::new();
    let mut kv: IndexMap<String, String> = IndexMap::with_capacity(headers.len());
    while rdr.read_record(&mut record)? {
        for (idx, field) in record.iter().enumerate() {
            kv.insert(headers[idx].to_string(), field.to_string());
        }
        let json = serde_json::to_string(&kv).unwrap_or_default();
        wtr.write_record(&[json])?;
        kv.clear();
    }

    Ok(wtr.flush()?)
}
