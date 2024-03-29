static USAGE: &str = r#"
Formats recognized date fields (19 formats recognized) to a specified date format
using strftime date format specifiers.

See https://github.com/jqnatividad/belt/tree/main/dateparser#accepted-date-formats for
recognized date formats.
See https://docs.rs/chrono/latest/chrono/format/strftime/ for 
accepted date format specifiers for --formatstr.
Defaults to ISO 8601/RFC 3339 format when --formatstr is not specified.
( "%Y-%m-%dT%H:%M:%S%z" - e.g. 2001-07-08T00:34:60.026490+09:30 )

Examples:
Format dates in Open Date column to ISO 8601/RFC 3339 format:

  $ qsv datefmt 'Open Date' file.csv

Format multiple date columns in file.csv to ISO 8601/RFC 3339 format:

  $ qsv datefmt 'Open Date,Modified Date,Closed Date' file.csv

Format all columns that end with "_date" case-insensitive in file.csv to ISO 8601/RFC 3339 format:

  $ qsv datefmt '/(?i) date$/' file.csv

Format dates in OpenDate column using '%Y-%m-%d' format:

  $ qsv datefmt OpenDate --formatstr '%Y-%m-%d' file.csv

Format multiple date columns using '%Y-%m-%d' format:

  $ qsv datefmt OpenDate,CloseDate,ReopenDate --formatstr '%Y-%m-%d' file.csv

Get the week number for OpenDate and store it in the week_number column:

  $ qsv datefmt OpenDate --formatstr '%V' --new-column week_number file.csv

Get the day of the week for several date columns and store it in the corresponding weekday columns:

  $ qsv datefmt OpenDate,CloseDate --formatstr '%u' --rename Open_weekday,Close_weekday file.csv

For more extensive examples, see https://github.com/jqnatividad/qsv/blob/master/tests/test_datefmt.rs.

Usage:
qsv datefmt [--formatstr=<string>] [options] <column> [<input>]
qsv datefmt --help

datefmt arguments:
    <column>                    The column/s to apply the date formats to.
                                Note that the <column> argument supports multiple columns.
                                See 'qsv select --help' for the format details.

    --formatstr=<string>        The date format to use for the datefmt operation.
                                The date format to use. For formats, see
                                https://docs.rs/chrono/latest/chrono/format/strftime/
                                Default to ISO 8601 / RFC 3339 date & time format -
                                "%Y-%m-%dT%H:%M:%S%z" - e.g. 2001-07-08T00:34:60.026490+09:30
                                [default: %+]
        
    <input>                     The input file to read from. If not specified, reads from stdin.

datefmt options:
    -c, --new-column <name>     Put the transformed values in a new column instead.
    -r, --rename <name>         New name for the transformed column.
    --prefer-dmy                Prefer to parse dates in dmy format. Otherwise, use mdy format.
    --keep-zero-time            If a formatted date ends with "T00:00:00+00:00", keep the time
                                instead of removing it. Only used with the DATEFMT subcommand.
    -j, --jobs <arg>            The number of jobs to run in parallel.
                                When not set, the number of jobs is set to the number of CPUs detected.
    -b, --batch <size>          The number of rows per batch to load into memory, before running in parallel.
                                [default: 50000]

Common options:
    -h, --help                  Display this message
    -o, --output <file>         Write output to <file> instead of stdout.
    -n, --no-headers            When set, the first row will not be interpreted
                                as headers.
    -d, --delimiter <arg>       The field delimiter for reading CSV data.
                                Must be a single character. (default: ,)
    -p, --progressbar           Show progress bars. Not valid for stdin.
"#;

#[cfg(any(feature = "feature_capable", feature = "lite"))]
use indicatif::{ProgressBar, ProgressDrawTarget};
use qsv_dateparser::parse_with_preference;
use rayon::{
    iter::{IndexedParallelIterator, ParallelIterator},
    prelude::IntoParallelRefIterator,
};
use serde::Deserialize;

use crate::{
    config::{Config, Delimiter},
    select::SelectColumns,
    util,
    util::replace_column_value,
    CliResult,
};

#[derive(Deserialize)]
struct Args {
    arg_column:          SelectColumns,
    arg_input:           Option<String>,
    flag_rename:         Option<String>,
    flag_prefer_dmy:     bool,
    flag_keep_zero_time: bool,
    flag_formatstr:      String,
    flag_batch:          u32,
    flag_jobs:           Option<usize>,
    flag_new_column:     Option<String>,
    flag_output:         Option<String>,
    flag_no_headers:     bool,
    flag_delimiter:      Option<Delimiter>,
    flag_progressbar:    bool,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;
    let rconfig = Config::new(&args.arg_input)
        .delimiter(args.flag_delimiter)
        .no_headers(args.flag_no_headers)
        .select(args.arg_column);

    let mut rdr = rconfig.reader()?;
    let mut wtr = Config::new(&args.flag_output).writer()?;

    let headers = rdr.byte_headers()?.clone();
    let sel = rconfig.selection(&headers)?;

    let mut headers = rdr.headers()?.clone();

    if let Some(new_name) = args.flag_rename {
        let new_col_names = util::ColumnNameParser::new(&new_name).parse()?;
        if new_col_names.len() != sel.len() {
            return fail_incorrectusage_clierror!(
                "Number of new columns does not match input column selection."
            );
        }
        for (i, col_index) in sel.iter().enumerate() {
            headers = replace_column_value(&headers, *col_index, &new_col_names[i]);
        }
    }

    if !rconfig.no_headers {
        if let Some(new_column) = &args.flag_new_column {
            headers.push_field(new_column);
        }
        wtr.write_record(&headers)?;
    }

    let flag_formatstr = args.flag_formatstr;
    let flag_new_column = args.flag_new_column;

    // prep progress bar
    #[cfg(any(feature = "feature_capable", feature = "lite"))]
    let show_progress =
        (args.flag_progressbar || util::get_envvar_flag("QSV_PROGRESSBAR")) && !rconfig.is_stdin();

    #[cfg(any(feature = "feature_capable", feature = "lite"))]
    let progress = ProgressBar::with_draw_target(None, ProgressDrawTarget::stderr_with_hz(5));

    #[cfg(any(feature = "feature_capable", feature = "lite"))]
    if show_progress {
        util::prep_progress(&progress, util::count_rows(&rconfig)?);
    } else {
        progress.set_draw_target(ProgressDrawTarget::hidden());
    }

    let prefer_dmy = args.flag_prefer_dmy || rconfig.get_dmy_preference();
    let keep_zero_time = args.flag_keep_zero_time;

    // amortize memory allocation by reusing record
    #[allow(unused_assignments)]
    let mut batch_record = csv::StringRecord::new();

    // reuse batch buffers
    let batchsize: usize = args.flag_batch as usize;
    let mut batch = Vec::with_capacity(batchsize);
    let mut batch_results = Vec::with_capacity(batchsize);

    // set RAYON_NUM_THREADS
    util::njobs(args.flag_jobs);

    // main loop to read CSV and construct batches for parallel processing.
    // each batch is processed via Rayon parallel iterator.
    // loop exits when batch is empty.
    'batch_loop: loop {
        for _ in 0..batchsize {
            match rdr.read_record(&mut batch_record) {
                Ok(has_data) => {
                    if has_data {
                        batch.push(std::mem::take(&mut batch_record));
                    } else {
                        // nothing else to add to batch
                        break;
                    }
                },
                Err(e) => {
                    return fail_clierror!("Error reading file: {e}");
                },
            }
        }

        if batch.is_empty() {
            // break out of infinite loop when at EOF
            break 'batch_loop;
        }

        // do actual datefmt via Rayon parallel iterator
        batch
            .par_iter()
            .map(|record_item| {
                let mut record = record_item.clone();

                let mut cell = String::new();
                #[allow(unused_assignments)]
                let mut formatted_date = String::new();
                let mut parsed_date;
                let new_column = flag_new_column.is_some();
                for col_index in &*sel {
                    record[*col_index].clone_into(&mut cell);
                    if !cell.is_empty() {
                        parsed_date = parse_with_preference(&cell, prefer_dmy);
                        // log::debug!("Parsed date: {:?}", parsed_date);
                        if let Ok(format_date) = parsed_date {
                            formatted_date = format_date.format(&flag_formatstr).to_string();
                            if !keep_zero_time && formatted_date.ends_with("T00:00:00+00:00") {
                                formatted_date[..10].clone_into(&mut cell);
                            } else {
                                formatted_date.clone_into(&mut cell);
                            }
                        }
                    }
                    if new_column {
                        record.push_field(&cell);
                    } else {
                        record = replace_column_value(&record, *col_index, &cell);
                    }
                }
                record
            })
            .collect_into_vec(&mut batch_results);

        // rayon collect() guarantees original order, so we can just append results each batch
        for result_record in &batch_results {
            wtr.write_record(result_record)?;
        }

        #[cfg(any(feature = "feature_capable", feature = "lite"))]
        if show_progress {
            progress.inc(batch.len() as u64);
        }

        batch.clear();
    } // end batch loop

    #[cfg(any(feature = "feature_capable", feature = "lite"))]
    if show_progress {
        util::finish_progress(&progress);
    }
    Ok(wtr.flush()?)
}
