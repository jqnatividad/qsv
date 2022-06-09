use crate::config::{Config, Delimiter};
use crate::util;
use crate::CliResult;
use log::{debug, info};
use serde::Deserialize;

static USAGE: &str = "
Prints a count of the number of records in the CSV data.

Note that the count will not include the header row (unless --no-headers is
given).

Usage:
    qsv count [options] [<input>]

count options:
    -H, --human-readable   Comma separate row count.

Common options:
    -h, --help             Display this message
    -n, --no-headers       When set, the first row will be included in
                           the count.
    -d, --delimiter <arg>  The field delimiter for reading CSV data.
                           Must be a single character. (default: ,)
";

#[derive(Deserialize)]
struct Args {
    arg_input: Option<String>,
    flag_human_readable: bool,
    flag_no_headers: bool,
    flag_delimiter: Option<Delimiter>,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;
    let conf = Config::new(&args.arg_input)
        .delimiter(args.flag_delimiter)
        .no_headers(args.flag_no_headers);

    debug!(
        "input: {:?}, no_header: {}, delimiter: {:?}",
        (&args.arg_input).clone().unwrap(),
        &args.flag_no_headers,
        &args.flag_delimiter
    );

    let count = if let Some(idx) = conf.indexed().unwrap_or_else(|_| {
        info!("index is stale...");
        None
    }) {
        info!("index used");
        idx.count()
    } else {
        info!(r#"counting "manually"..."#);
        let mut rdr = conf.reader()?;
        let mut count = 0u64;
        let mut record = csv::ByteRecord::new();
        while rdr.read_byte_record(&mut record)? {
            count += 1;
        }
        count
    };

    if args.flag_human_readable {
        use thousands::Separable;

        println!("{}", count.separate_with_commas());
    } else {
        println!("{count}");
    }
    Ok(())
}
