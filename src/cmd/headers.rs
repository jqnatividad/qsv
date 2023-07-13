static USAGE: &str = r#"
Prints the fields of the first row in the CSV data.

These names can be used in commands like 'select' to refer to columns in the
CSV data.

Note that multiple CSV files may be given to this command. This is useful with
the --intersect flag.

For examples, see https://github.com/jqnatividad/qsv/blob/master/tests/test_headers.rs.

Usage:
    qsv headers [options] [<input>...]
    qsv headers --help

headers options:
    -j, --just-names       Only show the header names (hide column index).
                           This is automatically enabled if more than one
                           input is given.
    --intersect            Shows the intersection of all headers in all of
                           the inputs given.
    --trim                 Trim space & quote characters from header name.

Common options:
    -h, --help             Display this message
    -d, --delimiter <arg>  The field delimiter for reading CSV data.
                           Must be a single character. (default: ,)
"#;

use std::io;

use serde::Deserialize;
use tabwriter::TabWriter;

use crate::{config::Delimiter, util, CliResult};

#[derive(Deserialize)]
struct Args {
    arg_input:       Vec<String>,
    flag_just_names: bool,
    flag_intersect:  bool,
    flag_trim:       bool,
    flag_delimiter:  Option<Delimiter>,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;
    let configs = util::many_configs(&args.arg_input, args.flag_delimiter, true)?;

    let num_inputs = configs.len();
    let mut headers: Vec<Vec<u8>> = vec![];
    for conf in configs {
        let mut rdr = conf.reader()?;
        for header in rdr.byte_headers()? {
            if !args.flag_intersect || !headers.iter().any(|h| &**h == header) {
                headers.push(header.to_vec());
            }
        }
    }

    let mut wtr: Box<dyn io::Write> = if args.flag_just_names {
        Box::new(io::stdout())
    } else {
        Box::new(TabWriter::new(io::stdout()))
    };
    for (i, header) in headers.iter().enumerate() {
        if num_inputs == 1 && !args.flag_just_names {
            write!(&mut wtr, "{}\t", i + 1)?;
        }
        if args.flag_trim {
            wtr.write_all(
                std::string::String::from_utf8_lossy(header)
                    .trim_matches(|c| c == '"' || c == ' ')
                    .as_bytes(),
            )?;
        } else {
            wtr.write_all(header)?;
        }
        wtr.write_all(b"\n")?;
    }
    wtr.flush()?;
    Ok(())
}
