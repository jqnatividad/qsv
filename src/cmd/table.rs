static USAGE: &str = r#"
Outputs CSV data as a table with columns in alignment.

This will not work well if the CSV data contains large fields.

Note that formatting a table requires buffering all CSV data into memory.
Therefore, you should use the 'sample' or 'slice' command to trim down large
CSV data before formatting it with this command.

Usage:
    qsv table [options] [<input>]
    qsv table --help

table options:
    -w, --width <arg>      The minimum width of each column.
                           [default: 2]
    -p, --pad <arg>        The minimum number of spaces between each column.
                           [default: 2]
    -a, --align <arg>      How entries should be aligned in a column.
                           Options: "left", "right", "center".
                           [default: left]
    -c, --condense <arg>   Limits the length of each field to the value
                           specified. If the field is UTF-8 encoded, then
                           <arg> refers to the number of code points.
                           Otherwise, it refers to the number of bytes.

Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
    -d, --delimiter <arg>  The field delimiter for reading CSV data.
                           Must be a single character. (default: ,)
    --memcheck             Check if there is enough memory to load the entire
                           CSV into memory using CONSERVATIVE heuristics.
"#;

use std::{borrow::Cow, convert::From};

use serde::Deserialize;
use tabwriter::{Alignment, TabWriter};

use crate::{
    config::{Config, Delimiter},
    util, CliResult,
};

#[derive(Deserialize)]
struct Args {
    arg_input:      Option<String>,
    flag_width:     usize,
    flag_pad:       usize,
    flag_output:    Option<String>,
    flag_delimiter: Option<Delimiter>,
    flag_align:     Align,
    flag_condense:  Option<usize>,
    flag_memcheck:  bool,
}

#[derive(Deserialize, Clone, Copy)]
enum Align {
    Left,
    Right,
    Center,
}

impl From<Align> for Alignment {
    fn from(align: Align) -> Self {
        match align {
            Align::Left => Alignment::Left,
            Align::Right => Alignment::Right,
            Align::Center => Alignment::Center,
        }
    }
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;
    let rconfig = Config::new(&args.arg_input)
        .delimiter(args.flag_delimiter)
        .no_headers(true)
        .flexible(true);

    // we're loading the entire file into memory, we need to check avail mem
    if let Some(path) = rconfig.path.clone() {
        util::mem_file_check(&path, false, args.flag_memcheck)?;
    }

    let wconfig = Config::new(&args.flag_output).delimiter(Some(Delimiter(b'\t')));

    let tw = TabWriter::new(wconfig.io_writer()?)
        .minwidth(args.flag_width)
        .padding(args.flag_pad)
        .alignment(args.flag_align.into());
    let mut wtr = wconfig.from_writer(tw);
    let mut rdr = rconfig.reader()?;

    let mut record = csv::ByteRecord::new();
    while rdr.read_byte_record(&mut record)? {
        wtr.write_record(
            record
                .iter()
                .map(|f| util::condense(Cow::Borrowed(f), args.flag_condense)),
        )?;
    }
    wtr.flush()?;
    Ok(())
}
