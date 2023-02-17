static USAGE: &str = r#"
Drop a CSV file's header.

Usage:
    qsv behead [options] [<input>]
    qsv behead --help

Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
"#;

use serde::Deserialize;

use crate::{config::Config, util, CliResult};

#[derive(Deserialize)]
struct Args {
    arg_input:   Option<String>,
    flag_output: Option<String>,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;
    let conf = Config::new(&args.arg_input).no_headers(false);

    let mut rdr = conf.reader()?;
    let mut wtr = Config::new(&args.flag_output).writer()?;
    let mut record = csv::ByteRecord::new();

    while rdr.read_byte_record(&mut record)? {
        wtr.write_byte_record(&record)?;
    }

    Ok(wtr.flush()?)
}
