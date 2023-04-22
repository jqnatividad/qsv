static USAGE: &str = r#"
Transpose the rows/columns of CSV data.

Note that by default this reads all of the CSV data into memory,
unless --multipass is given.

Usage:
    qsv transpose [options] [<input>]
    qsv transpose --help

transpose options:
    -m, --multipass        Process the transpose by making multiple
                           passes over the dataset. Useful for really
                           big datasets. Consumes memory relative to
                           the number of rows.
                           Note that in general it is faster to
                           process the transpose in memory.

Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
    -d, --delimiter <arg>  The field delimiter for reading CSV data.
                           Must be a single character. (default: ,)
    --memcheck             Check if there is enough memory to load the
                           entire CSV into memory. Ignored with --multipass.
"#;

use std::str;

use csv::ByteRecord;
use serde::Deserialize;

use crate::{
    config::{Config, Delimiter},
    util, CliResult,
};

#[derive(Deserialize)]
struct Args {
    arg_input:      Option<String>,
    flag_output:    Option<String>,
    flag_delimiter: Option<Delimiter>,
    flag_multipass: bool,
    flag_memcheck:  bool,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;

    let input_is_stdin = match args.arg_input {
        Some(ref s) if s == "-" => true,
        None => true,
        _ => false,
    };

    if args.flag_multipass && !input_is_stdin {
        args.multipass_transpose()
    } else {
        args.in_memory_transpose()
    }
}

impl Args {
    fn in_memory_transpose(&self) -> CliResult<()> {
        // we're loading the entire file into memory, we need to check avail mem
        if let Some(path) = self.rconfig().path {
            util::mem_file_check(&path, false, self.flag_memcheck)?;
        }

        let mut rdr = self.rconfig().reader()?;
        let mut wtr = self.wconfig().writer()?;
        let nrows = rdr.byte_headers()?.len();

        let all = rdr.byte_records().collect::<Result<Vec<_>, _>>()?;
        for i in 0..nrows {
            let mut record = ByteRecord::new();

            for row in &all {
                record.push_field(&row[i]);
            }
            wtr.write_byte_record(&record)?;
        }
        Ok(wtr.flush()?)
    }

    fn multipass_transpose(&self) -> CliResult<()> {
        let mut wtr = self.wconfig().writer()?;
        let nrows = self.rconfig().reader()?.byte_headers()?.len();

        for i in 0..nrows {
            let mut rdr = self.rconfig().reader()?;

            let mut record = ByteRecord::new();
            for row in rdr.byte_records() {
                record.push_field(&row?[i]);
            }
            wtr.write_byte_record(&record)?;
        }
        Ok(wtr.flush()?)
    }

    fn wconfig(&self) -> Config {
        Config::new(&self.flag_output)
    }

    fn rconfig(&self) -> Config {
        Config::new(&self.arg_input)
            .delimiter(self.flag_delimiter)
            .no_headers(true)
    }
}
