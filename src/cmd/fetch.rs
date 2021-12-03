use std::str::from_utf8;

use crate::config::{Config, Delimiter};
use crate::util;
use crate::CliResult;
use crate::select::SelectColumns;
use serde::Deserialize;
use log::{debug, info};

static USAGE: &str = "
Create a new column or fetch values from an URL column.

This command fetches HTML/data from web pages or web services for every row in the URL column.

URL column must contain full and valid URL path, which can be constructed via the 'lua' command.

Usage:
    qsv fetch [options] <column> [<input>]

fetch options:
    -c, --new-column <name>    Put the fetched values in a new column instead.

Common options:
    -h, --help                 Display this message
    -o, --output <file>        Write output to <file> instead of stdout.
    -n, --no-headers           When set, the first row will not be interpreted
                               as headers. Namely, it will be sorted with the rest
                               of the rows. Otherwise, the first row will always
                               appear as the header row in the output.
    -d, --delimiter <arg>      The field delimiter for reading CSV data.
                               Must be a single character. (default: ,)
    -q, --quiet                Don't show progress bars.
";

#[derive(Deserialize, Debug)]
struct Args {
    flag_new_column: Option<String>,
    flag_output: Option<String>,
    flag_no_headers: bool,
    flag_delimiter: Option<Delimiter>,
    flag_quiet: bool,
    arg_column: SelectColumns,
    arg_input: Option<String>,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;

    debug!("url column: {:?}, input: {:?}, new column: {:?}, output: {:?}, no_header: {:?}, delimiter: {:?}, quiet: {:?}", 
        (&args.arg_column).clone(),
        (&args.arg_input).clone().unwrap(),
        &args.flag_new_column,
        &args.flag_output,
        &args.flag_no_headers,
        &args.flag_delimiter,
        &args.flag_quiet
    );

    let rconfig = Config::new(&args.arg_input)
        .delimiter(args.flag_delimiter)
        .no_headers(args.flag_no_headers)
        .select(args.arg_column);



    Ok(())
}
