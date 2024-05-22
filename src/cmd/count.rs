static USAGE: &str = r#"
Prints a count of the number of records in the CSV data. If the polars feature is
enabled, it will use the much faster multithreaded, mem-mapped Polars CSV reader.
Otherwise, it will use the regular, single-threaded CSV reader.

Note that the count will not include the header row (unless --no-headers is
given).

For examples, see https://github.com/jqnatividad/qsv/blob/master/tests/test_count.rs.

Usage:
    qsv count [options] [<input>]
    qsv count --help

count options:
    -H, --human-readable   Comma separate row count.
    --width                Also return the estimated length of the longest record.
                           Its an estimate as it doesn't count quotes, and will be an
                           undercount if the record has quoted fields.
                           The count and width are separated by a semicolon.

                           WHEN THE POLARS FEATURE IS ENABLED:
    --no-polars            Use the "regular", single-threaded, streaming CSV reader instead
                           of the much faster multithreaded, mem-mapped Polars CSV reader.
                           Use this when you encounter memory issues when counting with the
                           Polars CSV reader. The streaming reader is slower but can read
                           any valid CSV file of any size.
    --low-memory           Use the Polars CSV Reader's low-memory mode. This
                           mode is slower but uses less memory. If counting still fails,
                           use --no-polars instead to use the streaming CSV reader.


Common options:
    -h, --help             Display this message
    -f, --flexible         Do not validate if the CSV has different number of
                           fields per record, increasing performance when counting
                           without an index. Automatically enabled when --width is set.
    -n, --no-headers       When set, the first row will be included in
                           the count.
"#;

use log::info;
use serde::Deserialize;

use crate::{config::Config, util, CliResult};

#[allow(dead_code)]
#[derive(Deserialize)]
struct Args {
    arg_input:           Option<String>,
    flag_human_readable: bool,
    flag_width:          bool,
    flag_no_polars:      bool,
    flag_low_memory:     bool,
    flag_flexible:       bool,
    flag_no_headers:     bool,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;
    let conf = Config::new(&args.arg_input)
        .no_headers(args.flag_no_headers)
        // we also want to count the quotes when computing width
        .quoting(!args.flag_width)
        // and ignore differing column counts as well
        .flexible(args.flag_width || args.flag_flexible);

    // this comment left here for Logging.md example
    // log::debug!(
    //     "input: {:?}, no_header: {}",
    //     (args.arg_input).clone().unwrap(),
    //     &args.flag_no_headers,
    // );

    // if --width or --flexible is set, we need to use the regular CSV reader
    let (count, width) = if args.flag_width || args.flag_flexible {
        count_input(&conf, args.flag_width)?
    } else {
        let index_status = conf.indexed().unwrap_or_else(|_| {
            info!("index is stale");
            None
        });
        match index_status {
            // there's a valid index, use it
            Some(idx) => {
                info!("index used");
                (idx.count(), 0)
            },
            None => {
                // if --no-polars or its a snappy compressed file, use the
                // regular CSV reader
                #[cfg(feature = "polars")]
                if args.flag_no_polars || conf.is_snappy() {
                    count_input(&conf, args.flag_width)?
                } else {
                    polars_count_input(&conf, args.flag_low_memory)?
                }

                #[cfg(not(feature = "polars"))]
                count_input(&conf, args.flag_width)?
            },
        }
    };

    if args.flag_human_readable {
        use indicatif::HumanCount;

        if args.flag_width {
            woutinfo!("{};{}", HumanCount(count), HumanCount(width as u64));
        } else {
            woutinfo!("{}", HumanCount(count));
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
    let mut record_numdelimiters = 0_usize;
    let mut record = csv::ByteRecord::new();

    if compute_width {
        let mut curr_width;

        // read the first record to get the number of delimiters
        // and the width of the first record
        rdr.read_byte_record(&mut record)?;
        max_width = record.as_slice().len();
        count = 1;

        // number of delimiters is number of fields minus 1
        // we subtract 1 because the last field doesn't have a delimiter
        record_numdelimiters = record.len().saturating_sub(1);

        while rdr.read_byte_record(&mut record)? {
            count += 1;

            curr_width = record.as_slice().len();
            if curr_width > max_width {
                max_width = curr_width;
            }
        }
    } else {
        while rdr.read_byte_record(&mut record)? {
            count += 1;
        }
    }
    // record_numdelimiters is a count of the delimiters
    // which we also want to count when returning width
    Ok((count, max_width + record_numdelimiters))
}

#[cfg(feature = "polars")]
pub fn polars_count_input(
    conf: &Config,
    low_memory: bool,
) -> Result<(u64, usize), crate::clitypes::CliError> {
    use polars::{prelude::*, sql::SQLContext};

    log::info!("using polars");

    let is_stdin = conf.is_stdin();

    let filepath = if is_stdin {
        let mut temp_file = tempfile::Builder::new().suffix(".csv").tempfile()?;
        let stdin = std::io::stdin();
        let mut stdin_handle = stdin.lock();
        std::io::copy(&mut stdin_handle, &mut temp_file)?;
        drop(stdin_handle);

        let (_, tempfile_pb) =
            temp_file.keep().or(Err(
                "Cannot keep temporary file created for stdin".to_string()
            ))?;

        tempfile_pb
    } else {
        conf.path.as_ref().unwrap().clone()
    };

    let mut comment_char = String::new();
    let comment_prefix = if let Some(c) = conf.comment {
        comment_char.push(c as char);
        Some(comment_char.as_str())
    } else {
        None
    };

    let optimization_state = polars::lazy::frame::OptState {
        projection_pushdown: true,
        predicate_pushdown:  true,
        type_coercion:       true,
        simplify_expr:       true,
        file_caching:        true,
        slice_pushdown:      true,
        comm_subplan_elim:   false,
        comm_subexpr_elim:   true,
        streaming:           true,
        fast_projection:     true,
        eager:               false,
        row_estimate:        true,
    };

    // read the file into a Polars LazyFrame
    let lazy_df = match LazyCsvReader::new(filepath.clone())
        .with_separator(conf.get_delimiter())
        .with_comment_prefix(comment_prefix)
        .with_low_memory(low_memory)
        .finish()
    {
        Ok(lazy_df) => lazy_df,
        Err(e) => {
            log::warn!("polars error: {}", e);
            let (count_regular, _) = count_input(conf, false)?;
            return Ok((count_regular, 0));
        },
    };

    // and leverage the magic of Polars SQL with its lazy evaluation, to count the records
    // in an optimized manner with its blazing fast multithreaded, mem-mapped CSV reader!
    let mut ctx = SQLContext::new();
    ctx.register("sql_lf", lazy_df.with_optimizations(optimization_state));
    let sqlresult_lf = ctx.execute("SELECT COUNT(*) FROM sql_lf")?;

    let mut count = if let Ok(cnt) = sqlresult_lf.collect()?["len"].u32() {
        cnt.get(0).ok_or("polars error: cannot get count")? as u64
    } else {
        // there was a Polars error, so we fall back to the regular CSV reader
        log::warn!("polars error, falling back to regular reader");
        let (count_regular, _) = count_input(conf, false)?;
        count_regular
    };

    // remove the temporary file we created to read from stdin
    // we use the keep() method to prevent the file from being deleted
    // when the tempfile went out of scope, so we need to manually delete it
    if is_stdin {
        std::fs::remove_file(filepath)?;
    }

    // Polars SQL requires headers, so it made the first row the header row
    // regardless of the --no-headers flag. That's why we need to add 1 to the count
    if conf.no_headers {
        count += 1;
    }

    Ok((count, 0))
}
