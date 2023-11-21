static USAGE: &str = r#"
Check if a CSV is sorted. The check is done on a streaming basis (i.e. constant memory).
With the --json options, also retrieve record count, sort breaks & duplicate count.

This command can be used in tandem with other qsv commands that sort or require sorted data
to ensure that they also work on a stream of data - i.e. without loading an entire CSV into memory.

For instance, a naive `dedup` requires loading the entire CSV into memory to sort it
first before deduping. However, if you know a CSV is sorted beforehand, you can invoke
`dedup` with the --sorted option, and it will skip loading entire CSV into memory to sort
it first. It will just immediately dedupe on a streaming basis.

`sort` also requires loading the entire CSV into memory. For simple "sorts" (not numeric,
reverse, unique & random sorts), particularly of very large CSV files that will not fit in memory,
`extsort` - a multi-threaded streaming sort that is exponentially faster and can work with 
arbitrarily large files, can be used instead.

Simply put, sortcheck allows you to make informed choices on how to compose pipelines that
require sorted data.

Returns exit code 0 if a CSV is sorted, and exit code 1 otherwise.

For examples, see https://github.com/jqnatividad/qsv/blob/master/tests/test_sortcheck.rs.

Usage:
    qsv sortcheck [options] [<input>]
    qsv sortcheck --help

sort options:
    -s, --select <arg>      Select a subset of columns to check for sort.
                            See 'qsv select --help' for the format details.
    -i, --ignore-case       Compare strings disregarding case
    --all                   Check all records. Do not stop/short-circuit the check 
                            on the first unsorted record.
    --json                  Return results in JSON format, scanning --all records. 
                            The JSON result has the following properties - 
                            sorted (boolean), record_count (number), 
                            unsorted_breaks (number) & dupe_count (number).
                            Unsorted breaks count the number of times two consecutive
                            rows are unsorted (i.e. n row > n+1 row).
                            Dupe count is the number of times two consecutive
                            rows are equal. Note that dupe count does not apply
                            if the file is not sorted and is set to -1.
    --pretty-json           Same as --json but in pretty JSON format.

Common options:
    -h, --help              Display this message
    -n, --no-headers        When set, the first row will not be interpreted
                            as headers. That is, it will be sorted with the rest
                            of the rows. Otherwise, the first row will always
                            appear as the header row in the output.
    -d, --delimiter <arg>   The field delimiter for reading CSV data.
                            Must be a single character. (default: ,)
    -p, --progressbar       Show progress bars. Not valid for stdin.
"#;

use std::cmp;

use csv::ByteRecord;
#[cfg(any(feature = "feature_capable", feature = "lite"))]
use indicatif::{HumanCount, ProgressBar, ProgressDrawTarget};
use serde::{Deserialize, Serialize};

use crate::{
    cmd::{dedup, sort::iter_cmp},
    config::{Config, Delimiter},
    select::SelectColumns,
    util, CliResult,
};

#[allow(dead_code)]
#[derive(Deserialize)]
struct Args {
    arg_input:        Option<String>,
    flag_select:      SelectColumns,
    flag_ignore_case: bool,
    flag_all:         bool,
    flag_no_headers:  bool,
    flag_delimiter:   Option<Delimiter>,
    flag_progressbar: bool,
    flag_json:        bool,
    flag_pretty_json: bool,
}

#[derive(Serialize, Deserialize)]
struct SortCheckStruct {
    sorted:          bool,
    record_count:    u64,
    unsorted_breaks: u64,
    dupe_count:      i64,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;
    let ignore_case = args.flag_ignore_case;
    let rconfig = Config::new(&args.arg_input)
        .delimiter(args.flag_delimiter)
        .no_headers(args.flag_no_headers)
        .select(args.flag_select);

    let mut rdr = rconfig.reader()?;

    let headers = rdr.byte_headers()?.clone();
    let sel = rconfig.selection(&headers)?;
    let record_count;

    // prep progress bar
    #[cfg(any(feature = "feature_capable", feature = "lite"))]
    let show_progress =
        (args.flag_progressbar || util::get_envvar_flag("QSV_PROGRESSBAR")) && !rconfig.is_stdin();
    #[cfg(any(feature = "feature_capable", feature = "lite"))]
    let progress = ProgressBar::with_draw_target(None, ProgressDrawTarget::stderr_with_hz(5));
    #[cfg(any(feature = "feature_capable", feature = "lite"))]
    {
        record_count = if show_progress {
            let count = util::count_rows(&rconfig)?;
            util::prep_progress(&progress, count);
            count
        } else {
            progress.set_draw_target(ProgressDrawTarget::hidden());
            0
        };
    }
    #[cfg(feature = "datapusher_plus")]
    {
        record_count = 0;
    }

    let do_json = args.flag_json | args.flag_pretty_json;

    let mut record = ByteRecord::new();
    let mut next_record = ByteRecord::new();
    let mut sorted = true;
    let mut scan_ctr: u64 = 0;
    let mut dupe_count: u64 = 0;
    let mut unsorted_breaks: u64 = 0;

    rdr.read_byte_record(&mut record)?;
    loop {
        #[cfg(any(feature = "feature_capable", feature = "lite"))]
        if show_progress {
            progress.inc(1);
        }
        scan_ctr += 1;
        let more_records = rdr.read_byte_record(&mut next_record)?;
        if !more_records {
            break;
        };
        let a = sel.select(&record);
        let b = sel.select(&next_record);
        let comparison =
            if ignore_case {
                dedup::iter_cmp_ignore_case(a, b)
            } else {
                iter_cmp(a, b)
            };

        match comparison {
            cmp::Ordering::Equal => {
                dupe_count += 1;
            },
            cmp::Ordering::Less => {
                record.clone_from(&next_record);
            },
            cmp::Ordering::Greater => {
                sorted = false;
                if args.flag_all || do_json {
                    unsorted_breaks += 1;
                    record.clone_from(&next_record);
                } else {
                    break;
                }
            },
        }
    } // end loop

    #[cfg(any(feature = "feature_capable", feature = "lite"))]
    if show_progress {
        if sorted {
            progress.set_message(format!(
                " - ALL {} records checked. {} duplicates found. Sorted.",
                HumanCount(record_count),
                HumanCount(dupe_count),
            ));
        } else if args.flag_all || do_json {
            progress.set_message(format!(
                " - ALL {} records checked. {} unsorted breaks. NOT Sorted.",
                HumanCount(record_count),
                HumanCount(unsorted_breaks),
            ));
        } else {
            progress.set_message(format!(
                " - {} of {} records checked before aborting. {} duplicates found so far. NOT \
                 sorted.",
                HumanCount(scan_ctr),
                HumanCount(record_count),
                HumanCount(dupe_count),
            ));
        }
        util::finish_progress(&progress);
    }

    if do_json {
        let sortcheck_struct = SortCheckStruct {
            sorted,
            record_count: if record_count == 0 {
                scan_ctr
            } else {
                record_count
            },
            unsorted_breaks,
            dupe_count: if sorted { dupe_count as i64 } else { -1 },
        };
        // it's OK to have unwrap here as we know sortcheck_struct is valid json
        if args.flag_pretty_json {
            println!(
                "{}",
                serde_json::to_string_pretty(&sortcheck_struct).unwrap()
            );
        } else {
            println!("{}", serde_json::to_string(&sortcheck_struct).unwrap());
        };
    }

    if !sorted {
        return fail!("not sorted");
    }

    Ok(())
}
