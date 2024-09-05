#![allow(clippy::cast_precision_loss)] // we're not worried about precision loss here
static USAGE: &str = r#"
Returns a count of the number of records in the CSV data.

It has three modes of operation:
 1. If a valid index is present, it will use it to lookup the count and
    return instantaneously. (fastest)

 If no index is present, it will read the CSV and count the number
 of records by scanning the file. 
   
   2. If the polars feature is enabled, it will use the multithreaded,
      mem-mapped Polars CSV reader. (faster - not available on qsvlite)
      
   3. If the polars feature is not enabled, it will use the "regular",
      single-threaded CSV reader.

Note that the count will not include the header row (unless --no-headers is
given).

For examples, see https://github.com/jqnatividad/qsv/blob/master/tests/test_count.rs.

Usage:
    qsv count [options] [<input>]
    qsv count --help

count options:
    -H, --human-readable   Comma separate counts.

WIDTH OPTIONS:
    --width                Also return the estimated widths of each record.
                           Its an estimate as it doesn't count quotes, and will be an
                           undercount if the record has quoted fields.
                           The count and width are separated by a semicolon. It will
                           return the max, avg, median, min, variance, stddev & MAD widths,
                           separated by hyphens. If --human-readable is set, the widths will
                           be labeled as "max", "avg", "median", "min", "stddev" & "mad"
                           respectively, separated by spaces.
                           Note that this option will require scanning the entire file
                           using the "regular", single-threaded, streaming CSV reader,
                           using the index if available for the count.
                           If the file is very large, it may not be able to compile some
                           stats - particularly avg, variance, stddev & MAD. In this case,
                           it will return 0.0 for those stats.
    --width-no-delims      Same as --width but does not count the delimiters in the width.
    --json                 Output the width stats in JSON format.

WHEN THE POLARS FEATURE IS ENABLED:
    --no-polars            Use the "regular", single-threaded, streaming CSV reader instead
                           of the much faster multithreaded, mem-mapped Polars CSV reader.
                           Use this when you encounter memory issues when counting with the
                           Polars CSV reader. The streaming reader is slower but can read
                           any valid CSV file of any size.
    --low-memory           Use the Polars CSV Reader's low-memory mode. This mode
                           is slower but uses less memory. If counting still fails,
                           use --no-polars instead to use the streaming CSV reader.


Common options:
    -h, --help             Display this message
    -f, --flexible         Do not validate if the CSV has different number of
                           fields per record, increasing performance when counting
                           without an index.
    -n, --no-headers       When set, the first row will be included in
                           the count.
"#;

use log::info;
use serde::Deserialize;

use crate::{config::Config, util, CliError, CliResult};

#[allow(dead_code)]
#[derive(Deserialize)]
struct Args {
    arg_input:            Option<String>,
    flag_human_readable:  bool,
    flag_width:           bool,
    flag_width_no_delims: bool,
    flag_json:            bool,
    flag_no_polars:       bool,
    flag_low_memory:      bool,
    flag_flexible:        bool,
    flag_no_headers:      bool,
}

#[derive(Copy, Clone, PartialEq)]
enum CountDelimsMode {
    IncludeDelims,
    ExcludeDelims,
    NotRequired,
}

#[derive(Default)]
struct WidthStats {
    max:      usize,
    avg:      f64,
    median:   usize,
    min:      usize,
    variance: f64,
    stddev:   f64,
    mad:      f64,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;
    let conf = Config::new(&args.arg_input)
        .no_headers(args.flag_no_headers)
        // we also want to count the quotes when computing width
        .quoting(!args.flag_width || !args.flag_width_no_delims)
        // and ignore differing column counts as well
        .flexible(args.flag_flexible);

    // this comment left here for Logging.md example
    // log::debug!(
    //     "input: {:?}, no_header: {}",
    //     (args.arg_input).clone().unwrap(),
    //     &args.flag_no_headers,
    // );

    let count_delims_mode = if args.flag_width_no_delims {
        CountDelimsMode::ExcludeDelims
    } else if args.flag_width {
        CountDelimsMode::IncludeDelims
    } else {
        CountDelimsMode::NotRequired
    };

    let empty_record_stats = WidthStats::default();

    // if doing width or --flexible is set, we need to use the regular CSV reader
    let (count, record_stats) =
        if count_delims_mode != CountDelimsMode::NotRequired || args.flag_flexible {
            count_input(&conf, count_delims_mode)?
        } else {
            let index_status = conf.indexed().unwrap_or_else(|_| {
                info!("index is stale");
                None
            });
            match index_status {
                // there's a valid index, use it
                Some(idx) => {
                    info!("index used");
                    (idx.count(), empty_record_stats)
                },
                None => {
                    // if --no-polars or its a snappy compressed file, use the
                    // regular CSV reader
                    #[cfg(feature = "polars")]
                    if args.flag_no_polars || conf.is_snappy() {
                        count_input(&conf, count_delims_mode)?
                    } else {
                        let (count, _) = polars_count_input(&conf, args.flag_low_memory)?;
                        (count, empty_record_stats)
                    }

                    #[cfg(not(feature = "polars"))]
                    count_input(&conf, count_delims_mode)?
                },
            }
        };

    if args.flag_json {
        woutinfo!(
            r#"{{"count":{},"max":{},"avg":{},"median":{},"min":{},"variance":{},"stddev":{},"mad":{}}}"#,
            count,
            record_stats.max,
            util::round_num(record_stats.avg, 4),
            record_stats.median,
            record_stats.min,
            util::round_num(record_stats.variance, 4),
            util::round_num(record_stats.stddev, 4),
            util::round_num(record_stats.mad, 4),
        );
    } else if args.flag_human_readable {
        use indicatif::{HumanCount, HumanFloatCount};

        if count_delims_mode == CountDelimsMode::NotRequired {
            woutinfo!("{}", HumanCount(count));
        } else {
            woutinfo!(
                "{};max:{} avg:{} median:{} min:{} variance:{} stddev:{} mad:{}",
                HumanCount(count),
                HumanCount(record_stats.max as u64),
                HumanFloatCount(record_stats.avg),
                HumanCount(record_stats.median as u64),
                HumanCount(record_stats.min as u64),
                HumanFloatCount(record_stats.variance),
                HumanFloatCount(record_stats.stddev),
                HumanFloatCount(record_stats.mad),
            );
        }
    } else if count_delims_mode == CountDelimsMode::NotRequired {
        woutinfo!("{count}");
    } else {
        woutinfo!(
            "{count};{max}-{avg}-{median}-{min}-{variance}-{stddev}-{mad}",
            max = record_stats.max,
            avg = util::round_num(record_stats.avg, 4),
            median = record_stats.median,
            min = record_stats.min,
            variance = util::round_num(record_stats.variance, 4),
            stddev = util::round_num(record_stats.stddev, 4),
            mad = util::round_num(record_stats.mad, 4),
        );
    }
    Ok(())
}

fn count_input(conf: &Config, count_delims_mode: CountDelimsMode) -> CliResult<(u64, WidthStats)> {
    use rayon::{
        iter::{IntoParallelRefIterator, ParallelIterator},
        prelude::ParallelSliceMut,
    };

    // if conf is indexed, we still get the count from the index
    let mut use_index_count = false;
    let mut count = if let Some(idx) = conf.indexed()? {
        use_index_count = true;
        info!("index used");
        idx.count()
    } else {
        0_u64
    };

    let mut rdr = conf.reader()?;
    let mut record = csv::ByteRecord::new();
    let empty_record_stats = WidthStats::default();

    if count_delims_mode == CountDelimsMode::NotRequired {
        if !use_index_count {
            // if we're not using the index, we need to read the file
            // to get the count
            while rdr.read_byte_record(&mut record)? {
                count += 1;
            }
        }
        Ok((count, empty_record_stats))
    } else {
        // read the first record to get the number of delimiters
        // and the width of the first record
        if !rdr.read_byte_record(&mut record)? {
            return Ok((0, empty_record_stats));
        };

        let mut curr_width = record.as_slice().len();

        let mut max = curr_width;
        let mut min = curr_width;
        let mut total_width = curr_width;
        let mut widths = Vec::new();
        widths
            .try_reserve(if use_index_count {
                count as usize
            } else {
                1_000 // reasonable default to minimize reallocations
            })
            .map_err(|e| CliError::OutOfMemory(e.to_string()))?;

        widths.push(curr_width);
        let mut manual_count = 1_u64;

        // number of delimiters is number of fields minus 1
        // we subtract 1 because the last field doesn't have a delimiter
        let record_numdelims = if count_delims_mode == CountDelimsMode::IncludeDelims {
            record.len().saturating_sub(1)
        } else {
            0
        };

        while rdr.read_byte_record(&mut record)? {
            manual_count += 1;

            curr_width = record.as_slice().len() + record_numdelims;

            // we don't want to overflow total_width, so we do saturating_add
            total_width = total_width.saturating_add(curr_width);
            widths.push(curr_width);

            if curr_width > max {
                max = curr_width;
            } else if curr_width < min {
                min = curr_width;
            }
        }

        if !use_index_count {
            count = manual_count;
        }

        // Calculate average width
        // if total_width is saturated (== usize::MAX), then avg will be 0.0
        let avg = if total_width == usize::MAX {
            0.0_f64
        } else {
            total_width as f64 / count as f64
        };

        // Calculate median width
        widths.par_sort_unstable();
        let median = if count % 2 == 0 {
            (widths[(count / 2) as usize - 1] + widths[(count / 2) as usize]) / 2
        } else {
            widths[(count / 2) as usize]
        };

        // Calculate standard deviation & variance
        // if avg_width is 0 (because total_width > usize::MAX),
        // then variance & stddev will be 0
        let (variance, stddev) = if avg > 0.0 {
            let variance = widths
                .par_iter()
                .map(|&width| {
                    let diff = width as f64 - avg;
                    diff * diff
                })
                .sum::<f64>()
                / count as f64;
            (variance, variance.sqrt())
        } else {
            (0.0_f64, 0.0_f64)
        };

        // Calculate median absolute deviation (MAD)
        let mad = {
            let mut abs_devs: Vec<f64> = widths
                .iter()
                .map(|&width| (width as f64 - median as f64).abs())
                .collect();
            abs_devs.par_sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
            if count % 2 == 0 {
                (abs_devs[(count / 2) as usize - 1] + abs_devs[(count / 2) as usize]) / 2.0
            } else {
                abs_devs[(count / 2) as usize]
            }
        };

        Ok((
            count,
            WidthStats {
                max,
                avg,
                median,
                min,
                variance,
                stddev,
                mad,
            },
        ))
    }
}

#[cfg(feature = "polars")]
pub fn polars_count_input(conf: &Config, low_memory: bool) -> CliResult<(u64, usize)> {
    use polars::{
        lazy::frame::{LazyFrame, OptFlags},
        prelude::*,
        sql::SQLContext,
    };

    info!("using polars");

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
        Some(PlSmallStr::from_str(comment_char.as_str()))
    } else {
        None
    };

    let mut ctx = SQLContext::new();
    let lazy_df: LazyFrame;
    let delimiter = conf.get_delimiter();

    // if its a "regular" CSV, use polars' read_csv() SQL table function
    // which is much faster than the LazyCsvReader
    let count_query = if comment_prefix.is_none() && delimiter == b',' && !low_memory {
        format!(
            "SELECT COUNT(*) FROM read_csv('{}')",
            filepath.to_string_lossy(),
        )
    } else {
        // otherwise, read the file into a Polars LazyFrame
        // using the LazyCsvReader builder to set CSV read options
        lazy_df = match LazyCsvReader::new(filepath.clone())
            .with_separator(delimiter)
            .with_comment_prefix(comment_prefix)
            .with_low_memory(low_memory)
            .finish()
        {
            Ok(lazy_df) => lazy_df,
            Err(e) => {
                log::warn!("polars error loading CSV: {e}");
                let (count_regular, _) = count_input(conf, CountDelimsMode::NotRequired)?;
                return Ok((count_regular, 0));
            },
        };
        let optflags = OptFlags::from_bits_truncate(0)
            | OptFlags::PROJECTION_PUSHDOWN
            | OptFlags::PREDICATE_PUSHDOWN
            | OptFlags::CLUSTER_WITH_COLUMNS
            | OptFlags::TYPE_COERCION
            | OptFlags::SIMPLIFY_EXPR
            | OptFlags::FILE_CACHING
            | OptFlags::SLICE_PUSHDOWN
            | OptFlags::COMM_SUBPLAN_ELIM
            | OptFlags::COMM_SUBEXPR_ELIM
            | OptFlags::FAST_PROJECTION
            | OptFlags::STREAMING;
        ctx.register("sql_lf", lazy_df.with_optimizations(optflags));
        "SELECT COUNT(*) FROM sql_lf".to_string()
    };

    // now leverage the magic of Polars SQL with its lazy evaluation, to count the records
    // in an optimized manner with its blazing fast multithreaded, mem-mapped CSV reader!
    let sqlresult_lf = match ctx.execute(&count_query) {
        Ok(sqlresult_lf) => sqlresult_lf,
        Err(e) => {
            // there was a Polars error, so we fall back to the regular CSV reader
            log::warn!("polars error executing count query: {e}");
            let (count_regular, _) = count_input(conf, CountDelimsMode::NotRequired)?;
            return Ok((count_regular, 0));
        },
    };

    let mut count = if let Ok(cnt) = sqlresult_lf.collect()?["len"].u32() {
        cnt.get(0).ok_or("polars error: cannot get count")? as u64
    } else {
        // Polars error, fall back to the regular CSV reader
        log::warn!("polars error, falling back to regular reader");
        let (count_regular, _) = count_input(conf, CountDelimsMode::NotRequired)?;
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
