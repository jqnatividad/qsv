static USAGE: &str = r#"
Returns the rows in the range specified (starting at 0, half-open interval).
The range does not include headers.

If the start of the range isn't specified, then the slice starts from the first
record in the CSV data.

If the end of the range isn't specified, then the slice continues to the last
record in the CSV data.

This operation can be made much faster by creating an index with 'qsv index'
first. Namely, a slice on an index requires parsing just the rows that are
sliced. Without an index, all rows up to the first row in the slice must be
parsed.

Usage:
    qsv slice [options] [<input>]
    qsv slice --help

slice options:
    -s, --start <arg>      The index of the record to slice from.
                           If negative, starts from the last record.
    -e, --end <arg>        The index of the record to slice to.
    -l, --len <arg>        The length of the slice (can be used instead
                           of --end).
    -i, --index <arg>      Slice a single record (shortcut for -s N -l 1).

Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
    -n, --no-headers       When set, the first row will not be interpreted
                           as headers. Otherwise, the first row will always
                           appear in the output as the header row.
    -d, --delimiter <arg>  The field delimiter for reading CSV data.
                           Must be a single character. (default: ,)
"#;

use std::fs;

use serde::Deserialize;

use crate::{
    config::{Config, Delimiter},
    index::Indexed,
    util, CliResult,
};

#[derive(Deserialize)]
struct Args {
    arg_input:       Option<String>,
    flag_start:      Option<isize>,
    flag_end:        Option<usize>,
    flag_len:        Option<usize>,
    flag_index:      Option<usize>,
    flag_output:     Option<String>,
    flag_no_headers: bool,
    flag_delimiter:  Option<Delimiter>,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;
    match args.rconfig().indexed()? {
        None => args.no_index(),
        Some(idxed) => args.with_index(idxed),
    }
}

impl Args {
    fn no_index(&self) -> CliResult<()> {
        let mut rdr = self.rconfig().reader()?;
        let mut wtr = self.wconfig().writer()?;
        self.rconfig().write_headers(&mut rdr, &mut wtr)?;

        let (start, end) = self.range()?;
        for r in rdr.byte_records().skip(start).take(end - start) {
            wtr.write_byte_record(&r?)?;
        }
        Ok(wtr.flush()?)
    }

    fn with_index(&self, mut idx: Indexed<fs::File, fs::File>) -> CliResult<()> {
        let mut wtr = self.wconfig().writer()?;
        self.rconfig().write_headers(&mut *idx, &mut wtr)?;

        let (start, end) = self.range()?;
        if end - start == 0 {
            return Ok(());
        }
        idx.seek(start as u64)?;
        for r in idx.byte_records().take(end - start) {
            wtr.write_byte_record(&r?)?;
        }
        Ok(wtr.flush()?)
    }

    fn range(&self) -> Result<(usize, usize), String> {
        let mut start = None;
        if let Some(start_arg) = self.flag_start {
            if start_arg < 0 {
                start = Some(
                    (util::count_rows(&self.rconfig()).unwrap() as usize)
                        .abs_diff(start_arg.unsigned_abs()),
                );
            } else {
                start = Some(start_arg as usize);
            }
        }
        util::range(start, self.flag_end, self.flag_len, self.flag_index)
    }

    fn rconfig(&self) -> Config {
        Config::new(&self.arg_input)
            .delimiter(self.flag_delimiter)
            .no_headers(self.flag_no_headers)
    }

    fn wconfig(&self) -> Config {
        Config::new(&self.flag_output)
    }
}
