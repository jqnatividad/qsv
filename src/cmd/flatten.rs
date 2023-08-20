static USAGE: &str = r#"
Prints flattened records such that fields are labeled separated by a new line.
This mode is particularly useful for viewing one record at a time. Each
record is separated by a special '#' character (on a line by itself), which
can be changed with the --separator flag.

There is also a condensed view (-c or --condense) that will shorten the
contents of each field to provide a summary view.

For examples, see https://github.com/jqnatividad/qsv/blob/master/tests/test_flatten.rs.

Usage:
    qsv flatten [options] [<input>]
    qsv flatten --help

flatten options:
    -c, --condense <arg>          Limits the length of each field to the value
                                  specified. If the field is UTF-8 encoded, then
                                  <arg> refers to the number of code points.
                                  Otherwise, it refers to the number of bytes.
    -f, --field-separator <arg>   A string of character to write between a column name
                                  and its value.
    -s, --separator <arg>         A string of characters to write after each record.
                                  When non-empty, a new line is automatically
                                  appended to the separator.
                                  [default: #]

Common options:
    -h, --help             Display this message
    -n, --no-headers       When set, the first row will not be interpreted
                           as headers. When set, the name of each field
                           will be its index.
    -d, --delimiter <arg>  The field delimiter for reading CSV data.
                           Must be a single character. (default: ,)
"#;

use std::{
    borrow::Cow,
    io::{self, BufWriter, Write},
};

use serde::Deserialize;
use tabwriter::TabWriter;

use crate::{
    config::{Config, Delimiter, DEFAULT_WTR_BUFFER_CAPACITY},
    util, CliResult,
};

#[derive(Deserialize)]
struct Args {
    arg_input:            Option<String>,
    flag_condense:        Option<usize>,
    flag_field_separator: Option<String>,
    flag_separator:       String,
    flag_no_headers:      bool,
    flag_delimiter:       Option<Delimiter>,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;
    let rconfig = Config::new(&args.arg_input)
        .delimiter(args.flag_delimiter)
        .no_headers(args.flag_no_headers);
    let mut rdr = rconfig.reader()?;
    let headers = rdr.byte_headers()?.clone();

    let stdoutlock = io::stdout().lock();
    let bufwtr = BufWriter::with_capacity(DEFAULT_WTR_BUFFER_CAPACITY, stdoutlock);
    let mut wtr = TabWriter::new(bufwtr);

    let mut first = true;
    let mut record = csv::ByteRecord::new();
    let separator_flag = !args.flag_separator.is_empty();
    let separator = args.flag_separator;
    let field_separator_flag = args.flag_field_separator.is_some();
    let field_separator = args.flag_field_separator.unwrap_or_default().into_bytes();

    while rdr.read_byte_record(&mut record)? {
        if !first && separator_flag {
            writeln!(&mut wtr, "{separator}")?;
        }
        first = false;
        for (i, (header, field)) in headers.iter().zip(&record).enumerate() {
            if rconfig.no_headers {
                write!(&mut wtr, "{i}")?;
            } else {
                wtr.write_all(header)?;
            }
            wtr.write_all(b"\t")?;
            if field_separator_flag {
                wtr.write_all(&field_separator)?;
            }
            wtr.write_all(&util::condense(Cow::Borrowed(field), args.flag_condense))?;
            wtr.write_all(b"\n")?;
        }
    }
    wtr.flush()?;
    Ok(())
}
