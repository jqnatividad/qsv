static USAGE: &str = r#"
Read CSV data with special quoting, trimming, line-skipping & UTF-8 transcoding rules.

Generally, all qsv commands support basic options like specifying the delimiter
used in CSV data. This does not cover all possible types of CSV data. For
example, some CSV files don't use '"' for quotes or use different escaping
styles.

Also, CSVs with preamble lines can have them skipped with the --skip-lines & --auto-skip
options. Similarly, --skip-lastlines allows epilog lines to be skipped.

Finally, non-UTF8 encoded files are transcoded to UTF-8 with this command, replacing all
invalid UTF-8 sequences with �.

This command is typically used in the beginning of a data pipeline to normalize & prepare CSVs
for further processing with other qsv commands.

For examples, see https://github.com/jqnatividad/qsv/blob/master/tests/test_input.rs.

Usage:
    qsv input [options] [<input>]
    qsv input --help

input options:
    --quote <arg>            The quote character to use. [default: "]
    --escape <arg>           The escape character to use. When not specified,
                             quotes are escaped by doubling them.
    --no-quoting             Disable quoting completely.
    --skip-lines <arg>       The number of preamble lines to skip.
    --auto-skip              Sniff's a CSV for preamble lines and automatically
                             skips them. Takes precedence over --skip-lines option.
                             Does not work with <stdin>.
    --skip-lastlines <arg>   The number of epilog lines to skip.
    --trim-headers           Trim leading & trailing whitespace & quotes from header values.
    --trim-fields            Trim leading & trailing whitespace from field values.

Common options:
    -h, --help               Display this message
    -o, --output <file>      Write output to <file> instead of stdout.
    -d, --delimiter <arg>    The field delimiter for reading CSV data.
                             Must be a single character. (default: ,)
"#;

use log::info;
use serde::Deserialize;

use crate::{
    config::{Config, Delimiter},
    util, CliResult,
};

#[derive(Deserialize)]
struct Args {
    arg_input:           Option<String>,
    flag_output:         Option<String>,
    flag_delimiter:      Option<Delimiter>,
    flag_quote:          Delimiter,
    flag_escape:         Option<Delimiter>,
    flag_no_quoting:     bool,
    flag_skip_lines:     Option<u64>,
    flag_skip_lastlines: Option<u64>,
    flag_auto_skip:      bool,
    flag_trim_headers:   bool,
    flag_trim_fields:    bool,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;

    let trim_setting = match (args.flag_trim_headers, args.flag_trim_fields) {
        (false, false) => csv::Trim::None,
        (true, true) => csv::Trim::All,
        (true, false) => csv::Trim::Headers,
        (false, true) => csv::Trim::Fields,
    };

    if args.flag_auto_skip {
        std::env::set_var("QSV_SNIFF_PREAMBLE", "1");
    }
    let mut rconfig = Config::new(&args.arg_input)
        .checkutf8(false)
        .delimiter(args.flag_delimiter)
        .no_headers(true)
        .quote(args.flag_quote.as_byte())
        .trim(trim_setting);
    if args.flag_auto_skip {
        std::env::remove_var("QSV_SNIFF_PREAMBLE");
    }
    let wconfig = Config::new(&args.flag_output);

    if let Some(escape) = args.flag_escape {
        rconfig = rconfig.escape(Some(escape.as_byte())).double_quote(false);
    }
    if args.flag_no_quoting {
        rconfig = rconfig.quoting(false);
    }
    if args.flag_auto_skip || args.flag_skip_lines.is_some() || args.flag_skip_lastlines.is_some() {
        rconfig = rconfig.flexible(true);
    }

    let mut total_lines = 0_u64;
    if let Some(skip_llines) = args.flag_skip_lastlines {
        let row_count = util::count_rows(&rconfig)?;
        if skip_llines > row_count {
            return fail_clierror!(
                "--skip-lastlines: {skip_llines} is greater than row_count: {row_count}."
            );
        }
        info!("Set to skip last {skip_llines} lines...");
        total_lines = row_count - skip_llines;
    }

    let mut rdr = rconfig.reader()?;
    let mut wtr = wconfig.writer()?;
    let mut row = csv::ByteRecord::new();
    let mut str_row = csv::StringRecord::new();

    let preamble_rows: u64 = if args.flag_auto_skip {
        info!("auto-skip on...");
        rconfig.preamble_rows
    } else if args.flag_skip_lines.is_some() {
        args.flag_skip_lines.unwrap()
    } else {
        0
    };

    if preamble_rows > 0 {
        info!("skipping {preamble_rows} preamble rows...");
        for _i in 1..=preamble_rows {
            rdr.read_byte_record(&mut row)?;
        }
        if total_lines.saturating_sub(preamble_rows) > 0 {
            total_lines -= preamble_rows;
        }
    }
    // the first rdr record is the header, since
    // we have no_headers = true, we manually trim the first record
    if trim_setting == csv::Trim::Headers || trim_setting == csv::Trim::All {
        info!("trimming...");
        rdr.read_byte_record(&mut row)?;
        row.trim();

        for field in row.iter() {
            // we also trim excess quotes from the header, to be consistent with safenames
            str_row.push_field(String::from_utf8_lossy(field).trim_matches('"'));
        }
        wtr.write_record(&str_row)?;
    }

    let mut idx = 1_u64;
    loop {
        match rdr.read_byte_record(&mut row) {
            Ok(moredata) => {
                if !moredata {
                    break;
                }
            }
            Err(e) => {
                return fail_clierror!("Invalid CSV. Last valid row ({idx}): {e}");
            }
        };

        str_row.clear();
        for field in row.iter() {
            str_row.push_field(&String::from_utf8_lossy(field));
        }
        wtr.write_record(&str_row)?;
        idx += 1;

        if total_lines > 0 && idx > total_lines {
            break;
        }
    }
    info!("Wrote {} rows...", idx - 1);
    wtr.flush()?;
    Ok(())
}
