use crate::util;
use crate::CliResult;
use csv_sniffer::{SampleSize, Sniffer};
use serde::Deserialize;

static USAGE: &str = "
Quickly sniff CSV details (delimiter, quote character, number of fields, data types,
header row, preamble rows).

Usage:
    qsv sniff [options] [<input>]

sniff options:
    -l, --len <arg>        How many rows to sample to sniff out the details.
                           [default: 100]

Common options:
    -h, --help             Display this message
";

#[derive(Deserialize)]
struct Args {
    arg_input: Option<String>,
    flag_len: usize,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;

    if let Some(path) = args.arg_input {
        match Sniffer::new()
            .sample_size(SampleSize::Records(args.flag_len))
            .sniff_path(path)
        {
            Ok(metadata) => {
                println!("{metadata}");
            }
            Err(e) => {
                return fail!(format!("sniff error: {e}"));
            }
        }
    }

    Ok(())
}
