use indexmap::IndexMap;

use crate::config::{Config, Delimiter};
use crate::util;
use crate::CliResult;
use serde::Deserialize;

static USAGE: &str = "
Converts a CSV to a newline-delimited JSON (JSONL/NDJSON) file.

Usage:
    qsv tojsonl [options] [<input>]
    qsv tojsonl --help

Common options:
    -h, --help             Display this message
    -d, --delimiter <arg>  The field delimiter for reading CSV data.
                           Must be a single character. (default: ,)
    -o, --output <file>    Write output to <file> instead of stdout.
";

#[derive(Deserialize)]
struct Args {
    arg_input: Option<String>,
    flag_delimiter: Option<Delimiter>,
    flag_output: Option<String>,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;
    let conf = Config::new(&args.arg_input).delimiter(args.flag_delimiter);

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
