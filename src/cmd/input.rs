use crate::config::{Config, Delimiter};
use crate::util;
use crate::CliResult;
use serde::Deserialize;

static USAGE: &str = r#"
Read CSV data with special quoting, trimming and line-skipping rules.

Generally, all qsv commands support basic options like specifying the delimiter
used in CSV data. This does not cover all possible types of CSV data. For
example, some CSV files don't use '"' for quotes or use different escaping
styles.

Also, CSVs with preamble lines can be have the preamble skipped with the --skip-lines 
option. Similarly, --skip-lastlines allows epilog lines to be skipped.

Usage:
    qsv input [options] [<input>]

input options:
    --quote <arg>            The quote character to use. [default: "]
    --escape <arg>           The escape character to use. When not specified,
                             quotes are escaped by doubling them.
    --no-quoting             Disable quoting completely.
    --skip-lines <arg>       The number of preamble lines to skip.
    --skip-lastlines <arg>   The number of epilog lines to skip.
    --trim-headers           Trim leading & trailing whitespace from header values.
    --trim-fields            Trim leading & trailing whitespace from field values.

Common options:
    -h, --help               Display this message
    -o, --output <file>      Write output to <file> instead of stdout.
    -d, --delimiter <arg>    The field delimiter for reading CSV data.
                             Must be a single character. (default: ,)
"#;

#[derive(Deserialize)]
struct Args {
    arg_input: Option<String>,
    flag_output: Option<String>,
    flag_delimiter: Option<Delimiter>,
    flag_quote: Delimiter,
    flag_escape: Option<Delimiter>,
    flag_no_quoting: bool,
    flag_skip_lines: Option<u64>,
    flag_skip_lastlines: Option<u64>,
    flag_trim_headers: bool,
    flag_trim_fields: bool,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;

    let trim_setting = match (args.flag_trim_headers, args.flag_trim_fields) {
        (false, false) => csv::Trim::None,
        (true, true) => csv::Trim::All,
        (true, false) => csv::Trim::Headers,
        (false, true) => csv::Trim::Fields,
    };

    let mut rconfig = Config::new(&args.arg_input)
        .delimiter(args.flag_delimiter)
        .no_headers(true)
        .quote(args.flag_quote.as_byte())
        .trim(trim_setting);
    let wconfig = Config::new(&args.flag_output);

    if let Some(escape) = args.flag_escape {
        rconfig = rconfig.escape(Some(escape.as_byte())).double_quote(false);
    }
    if args.flag_no_quoting {
        rconfig = rconfig.quoting(false);
    }
    if args.flag_skip_lines.is_some() || args.flag_skip_lastlines.is_some() {
        rconfig = rconfig.flexible(true);
    }

    let mut total_lines = 0_u64;
    if let Some(skip_llines) = args.flag_skip_lastlines {
        let row_count = util::count_rows(&rconfig);
        if skip_llines > row_count {
            return fail!("--skip-lastlines: {skip_llines} is greater than row_count: {rowcount}.");
        } else {
            total_lines = row_count - skip_llines;
        }
    }

    let mut rdr = rconfig.reader()?;
    let mut wtr = wconfig.writer()?;
    let mut row = csv::ByteRecord::new();
    if let Some(skip_lines) = args.flag_skip_lines {
        for _i in 1..=skip_lines {
            rdr.read_byte_record(&mut row)?;
        }
        if total_lines.saturating_sub(skip_lines) > 0 {
            total_lines -= skip_lines;
        }
    }
    // the first rdr record is the header, since
    // we have no_headers = true, we manually trim the first record
    if trim_setting == csv::Trim::Headers || trim_setting == csv::Trim::All {
        rdr.read_byte_record(&mut row)?;
        row.trim();
        wtr.write_record(&row)?;
    }

    let mut i = 1_u64;
    while rdr.read_byte_record(&mut row)? {
        wtr.write_record(&row)?;
        if total_lines > 0 {
            i += 1;
            if i > total_lines {
                break;
            }
        }
    }
    wtr.flush()?;
    Ok(())
}
