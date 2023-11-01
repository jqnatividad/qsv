static USAGE: &str = r#"
Rename the columns of a CSV efficiently.

The new column names are given as a comma-separated list of names.
The number of column names given must match the number of columns in the
CSV unless "_all_generic" is used.

  Change the column names of a CSV with three columns:
    $ qsv rename id,name,title

  Replace the column names with generic ones (_col_N):
    $ qsv rename _all_generic

  Add generic column names to a CSV with no headers:
    $ qsv rename _all_generic --no-headers

  Use column names that contains commas and conflict with the separator:
    $ qsv rename '"Date - Opening","Date - Actual Closing"'

For more examples, see https://github.com/jqnatividad/qsv/blob/master/tests/test_rename.rs.

Usage:
    qsv rename [options] [--] <headers> [<input>]
    qsv rename --help

rename arguments:
    <headers>              The new headers to use for the CSV.
                           Separate multiple headers with a comma.
                           If "_all_generic" is given, the headers will be renamed
                           to generic column names, where the column name uses
                           the format "_col_N" where N is the 1-based column index.

Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
    -n, --no-headers       When set, the header will be inserted on top.    
    -d, --delimiter <arg>  The field delimiter for reading CSV data.
                           Must be a single character. (default: ,)
"#;

use serde::Deserialize;

use crate::{
    config::{Config, Delimiter},
    util, CliResult,
};

#[derive(Deserialize)]
struct Args {
    arg_input:       Option<String>,
    arg_headers:     String,
    flag_output:     Option<String>,
    flag_no_headers: bool,
    flag_delimiter:  Option<Delimiter>,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let mut args: Args = util::get_args(USAGE, argv)?;

    let rconfig = Config::new(&args.arg_input)
        .delimiter(args.flag_delimiter)
        .no_headers(args.flag_no_headers);

    let mut rdr = rconfig.reader()?;
    let mut wtr = Config::new(&args.flag_output).writer()?;
    let headers = rdr.byte_headers()?;

    if args.arg_headers.to_lowercase() == "_all_generic" {
        args.arg_headers = rename_headers_all_generic(headers.len());
    }

    let mut new_rdr = csv::Reader::from_reader(args.arg_headers.as_bytes());
    let new_headers = new_rdr.byte_headers()?;

    if headers.len() != new_headers.len() {
        return fail_incorrectusage_clierror!(
            "The length of the CSV headers ({}) is different from the provided one ({}).",
            headers.len(),
            new_headers.len()
        );
    }

    wtr.write_record(new_headers)?;

    let mut record = csv::ByteRecord::new();
    while rdr.read_byte_record(&mut record)? {
        wtr.write_record(&record)?;
    }
    wtr.flush()?;
    Ok(())
}

pub fn rename_headers_all_generic(num_of_cols: usize) -> String {
    let mut generic_headers = String::new();
    for i in 1..=num_of_cols {
        generic_headers.push_str(&format!("_col_{i},"));
    }
    // remove the trailing comma
    generic_headers.pop();
    generic_headers
}
