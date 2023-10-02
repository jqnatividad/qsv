static USAGE: &str = r#"
Prints a count of the number of records in the CSV data.

Note that the count will not include the header row (unless --no-headers is
given).

For examples, see https://github.com/jqnatividad/qsv/blob/master/tests/test_count.rs.

Usage:
    qsv count [options] [<input>]
    qsv count --help

count options:
    -H, --human-readable   Comma separate row count.
    --width                Also return the length of the longest record.
                           The count and width are separated by a semicolon.

Common options:
    -h, --help             Display this message
    -n, --no-headers       When set, the first row will be included in
                           the count.
"#;

use log::info;
use serde::Deserialize;

use crate::{config::Config, util, CliResult};

#[derive(Deserialize)]
struct Args {
    arg_input:           Option<String>,
    flag_human_readable: bool,
    flag_width:          bool,
    flag_no_headers:     bool,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;
    let conf = Config::new(&args.arg_input)
        .no_headers(args.flag_no_headers)
        // we also want to count the quotes when computing width
        .quoting(!args.flag_width)
        // and ignore differing column counts as well
        .flexible(args.flag_width);

    // this comment left here for Logging.md example
    // log::debug!(
    //     "input: {:?}, no_header: {}",
    //     (args.arg_input).clone().unwrap(),
    //     &args.flag_no_headers,
    // );

    let (count, width) = if args.flag_width {
        count_input(&conf, args.flag_width)?
    } else {
        match conf.indexed().unwrap_or_else(|_| {
            info!("index is stale");
            None
        }) {
            Some(idx) => {
                info!("index used");
                (idx.count(), 0)
            },
            None => count_input(&conf, args.flag_width)?,
        }
    };

    if args.flag_human_readable {
        use thousands::Separable;

        if args.flag_width {
            woutinfo!(
                "{};{}",
                count.separate_with_commas(),
                width.separate_with_commas()
            );
        } else {
            woutinfo!("{}", count.separate_with_commas());
        }
    } else if args.flag_width {
        woutinfo!("{count};{width}");
    } else {
        woutinfo!("{count}");
    }
    Ok(())
}

fn count_input(
    conf: &Config,
    compute_width: bool,
) -> Result<(u64, usize), crate::clitypes::CliError> {
    let mut rdr = conf.reader()?;
    let mut count = 0_u64;
    let mut max_width = 0_usize;
    let mut record_numfields = 0_usize;
    let mut record = csv::ByteRecord::new();

    if compute_width {
        let mut curr_width;
        while rdr.read_byte_record(&mut record)? {
            count += 1;

            curr_width = record.as_slice().len();
            if curr_width > max_width {
                record_numfields = record.len();
                max_width = curr_width;
            }
        }
    } else {
        while rdr.read_byte_record(&mut record)? {
            count += 1;
        }
    }
    // record_numfields is a count of the delimiters
    // which we also want to count when returning width
    Ok((count, max_width + record_numfields))
}
