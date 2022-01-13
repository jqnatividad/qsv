use crate::config::{Config, Delimiter};
use crate::util;
use crate::CliResult;
use log::{debug, error};
use serde::Deserialize;

static USAGE: &str = "
Validate CSV data with JSON Schema, and put invalid records into separate file.

Example output files from `mydata.csv`:

* mydata.csv.valid
* mydata.csv.invalid
* mydata.csv.validation-report

JSON Schema can be a local file or a URL. 

When run without JSON Schema, only a simple CSV check (RFC 4180) is performed, with the caveat that 
 on non-Windows machines, each record is delimited by a CR (\n) instead of CRLF (\n\r).


Usage:
    qsv validate [options] [<input>] [<json-schema>]

fetch options:
    --fail-fast                Stops on first error.
    --valid <suffix>           Valid record output file suffix [default: valid]
    --invalid <suffix>         Invalid record output file suffix [default: invalid]


Common options:
    -h, --help                 Display this message
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
    flag_fail_fast: bool,
    flag_valid: Option<String>,
    flag_invalid: Option<String>,
    flag_no_headers: bool,
    flag_delimiter: Option<Delimiter>,
    flag_quiet: bool,
    arg_input: Option<String>,
    arg_json_schema: Option<String>,
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



