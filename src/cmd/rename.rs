use crate::config::{Config, Delimiter};
use crate::util;
use crate::CliResult;
use serde::Deserialize;

static USAGE: &str = "
Rename the columns of CSV data efficiently.

This command lets you rename the columns in CSV data. You must specify
all of the headers, and separate them by a comma.

  Change the name of the columns:
  $ qsv rename id,name,title

  Use column names that contains commas and conflict with the separator:
  $ qsv rename '\"Date - Opening\",\"Date - Actual Closing\"'

Usage:
    qsv rename [options] [--] <headers> [<input>]
    qsv rename --help

Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
    -n, --no-headers       When set, the header will be inserted on top.    
    -d, --delimiter <arg>  The field delimiter for reading CSV data.
                           Must be a single character. (default: ,)
";

#[derive(Deserialize)]
struct Args {
    arg_input: Option<String>,
    arg_headers: String,
    flag_output: Option<String>,
    flag_no_headers: bool,
    flag_delimiter: Option<Delimiter>,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;

    let rconfig = Config::new(&args.arg_input)
        .delimiter(args.flag_delimiter)
        .no_headers(args.flag_no_headers);

    let mut rdr = rconfig.reader()?;
    let mut wtr = Config::new(&args.flag_output).writer()?;
    let headers = rdr.byte_headers()?;

    let mut new_rdr = csv::Reader::from_reader(args.arg_headers.as_bytes());
    let new_headers = new_rdr.byte_headers()?;

    if headers.len() != new_headers.len() {
        return fail!("The length of the CSV headers is different from the provided one.");
    }

    wtr.write_record(new_headers)?;

    let mut record = csv::ByteRecord::new();
    while rdr.read_byte_record(&mut record)? {
        wtr.write_record(&record)?;
    }
    wtr.flush()?;
    Ok(())
}
