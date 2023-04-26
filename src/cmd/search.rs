static USAGE: &str = r#"
Filters CSV data by whether the given regex matches a row.

The regex is applied to each field in each row, and if any field matches,
then the row is written to the output, and the number of matches to stderr.

The columns to search can be limited with the '--select' flag (but the full row
is still written to the output if there is a match).

Returns exitcode 0 when matches are found, returning number of matches to stderr.
Returns exitcode 1 when no match is found.

When --quick is enabled, no output is produced and exitcode 0 is returned on 
the first match.

For examples, see https://github.com/jqnatividad/qsv/blob/master/tests/test_search.rs.

Usage:
    qsv search [options] <regex> [<input>]
    qsv search --help

search options:
    -i, --ignore-case      Case insensitive search. This is equivalent to
                           prefixing the regex with '(?i)'.
    -s, --select <arg>     Select the columns to search. See 'qsv select -h'
                           for the full syntax.
    -v, --invert-match     Select only rows that did not match
    -u, --unicode          Enable unicode support. When enabled, character classes
                           will match all unicode word characters instead of only
                           ASCII word characters. Decreases performance.
    -f, --flag <column>    If given, the command will not filter rows
                           but will instead flag the found rows in a new
                           column named <column>, with the row numbers
                           of the matched rows.
    -q, --quick            Return on first match with an exitcode of 0, returning
                           the row number of the first match to stderr.
                           Return exit code 1 if no match is found.
                           No output is produced.
    -c, --count            Return number of matches to stderr.
    --size-limit <mb>      Set the approximate size limit (MB) of the compiled
                           regular expression. If the compiled expression exceeds this 
                           number, then a compilation error is returned.
                           Modify this only if you're getting regular expression
                           compilation errors. [default: 50]
    --dfa-size-limit <mb>  Set the approximate size of the cache (MB) used by the regular
                           expression engine's Discrete Finite Automata.
                           Modify this only if you're getting regular expression
                           compilation errors. [default: 10]
                           
Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
    -n, --no-headers       When set, the first row will not be interpreted
                           as headers. (i.e., They are not searched, analyzed,
                           sliced, etc.)
    -d, --delimiter <arg>  The field delimiter for reading CSV data.
                           Must be a single character. (default: ,)
    -p, --progressbar      Show progress bars. Not valid for stdin.
    -Q, --quiet            Do not return number of matches to stderr.
"#;

#[cfg(any(feature = "feature_capable", feature = "lite"))]
use indicatif::{HumanCount, ProgressBar, ProgressDrawTarget};
use log::{debug, info};
use regex::bytes::RegexBuilder;
use serde::Deserialize;

use crate::{
    config::{Config, Delimiter},
    select::SelectColumns,
    util, CliError, CliResult,
};

#[allow(dead_code)]
#[derive(Deserialize)]
struct Args {
    arg_input:           Option<String>,
    arg_regex:           String,
    flag_select:         SelectColumns,
    flag_output:         Option<String>,
    flag_no_headers:     bool,
    flag_delimiter:      Option<Delimiter>,
    flag_invert_match:   bool,
    flag_unicode:        bool,
    flag_ignore_case:    bool,
    flag_flag:           Option<String>,
    flag_size_limit:     usize,
    flag_dfa_size_limit: usize,
    flag_quick:          bool,
    flag_count:          bool,
    flag_progressbar:    bool,
    flag_quiet:          bool,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;
    let regex_unicode = if util::get_envvar_flag("QSV_REGEX_UNICODE") {
        true
    } else {
        args.flag_unicode
    };

    debug!("Compiling regular expression <{}>", args.arg_regex);
    let pattern = RegexBuilder::new(&args.arg_regex)
        .case_insensitive(args.flag_ignore_case)
        .unicode(regex_unicode)
        .size_limit(args.flag_size_limit * (1 << 20))
        .dfa_size_limit(args.flag_dfa_size_limit * (1 << 20))
        .build()?;
    debug!("Successfully compiled regular expression!");

    let rconfig = Config::new(&args.arg_input)
        .delimiter(args.flag_delimiter)
        .no_headers(args.flag_no_headers)
        .select(args.flag_select);

    let mut rdr = rconfig.reader()?;
    let mut wtr = Config::new(&args.flag_output).writer()?;

    let mut headers = rdr.byte_headers()?.clone();
    let sel = rconfig.selection(&headers)?;

    let flag = args.flag_flag.map_or(false, |column_name| {
        headers.push_field(column_name.as_bytes());
        true
    });

    if !rconfig.no_headers && !args.flag_quick {
        wtr.write_record(&headers)?;
    }

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

    let mut record = csv::ByteRecord::new();
    let mut flag_rowi: u64 = 1;
    let mut match_ctr: u64 = 0;
    let mut row_ctr: u64 = 0;

    #[allow(unused_assignments)]
    let mut matched_rows = String::with_capacity(20); // to save on allocs
    while rdr.read_byte_record(&mut record)? {
        row_ctr += 1;
        #[cfg(any(feature = "feature_capable", feature = "lite"))]
        if show_progress {
            progress.inc(1);
        }
        let mut m = sel.select(&record).any(|f| pattern.is_match(f));
        if args.flag_invert_match {
            m = !m;
        }
        if m {
            match_ctr += 1;
            if args.flag_quick {
                break;
            }
        }

        if flag {
            flag_rowi += 1;
            record.push_field(if m {
                let mut buffer = itoa::Buffer::new();
                buffer.format(flag_rowi).clone_into(&mut matched_rows);
                matched_rows.as_bytes()
            } else {
                b"0"
            });
            wtr.write_byte_record(&record)?;
        } else if m {
            wtr.write_byte_record(&record)?;
        }
    }
    wtr.flush()?;

    #[cfg(any(feature = "feature_capable", feature = "lite"))]
    if show_progress {
        progress.set_message(format!(
            " - {} matches found in {} records.",
            HumanCount(match_ctr),
            HumanCount(progress.length().unwrap()),
        ));
        util::finish_progress(&progress);
    }

    if args.flag_count && !args.flag_quick {
        if !args.flag_quiet {
            eprintln!("{match_ctr}");
        }
        info!("matches: {match_ctr}");
    }

    if match_ctr == 0 {
        return Err(CliError::NoMatch());
    } else if args.flag_quick {
        if !args.flag_quiet {
            eprintln!("{row_ctr}");
        }
        info!("quick search first match at {row_ctr}");
    }

    Ok(())
}
