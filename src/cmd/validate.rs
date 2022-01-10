use crate::config::{Config, Delimiter};
use crate::util;
use crate::CliResult;
use log::{debug, error};
use serde::Deserialize;

static USAGE: &str = "
Validate CSV data with JSON Schema.

Usage:
    qsv validate [options] [<input>] <json-schema>

fetch options:
    -c, --new-column <name>    Put error(s) in a new column instead.
    --fail-fast                Stops on first error.


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
    flag_fail_fast: bool,
    flag_output: Option<String>,
    flag_no_headers: bool,
    flag_delimiter: Option<Delimiter>,
    flag_quiet: bool,
    arg_input: Option<String>,
    arg_json_schema: String,
}


pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;

    dbg!(&args);

    let rconfig = Config::new(&args.arg_input)
        .delimiter(args.flag_delimiter)
        .no_headers(args.flag_no_headers);

    let mut rdr = rconfig.reader()?;
    let mut wtr = Config::new(&None).writer()?;

    let mut headers = rdr.byte_headers()?.clone();


    Ok(())
}



