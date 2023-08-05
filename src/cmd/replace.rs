static USAGE: &str = r#"
Replace occurrences of a pattern across a CSV file.

You can of course match groups using parentheses and use those in
the replacement string. But don't forget to escape your $ in bash by using a
backslash or by wrapping the replacement string into single quotes:

  $ qsv replace 'hel(lo)' 'hal$1' file.csv
  $ qsv replace "hel(lo)" "hal\$1" file.csv

Returns exitcode 0 when replacements are done, returning number of replacements to stderr.
Returns exitcode 1 when no replacements are done.

For more examples, see https://github.com/jqnatividad/qsv/blob/master/tests/test_replace.rs.

Usage:
    qsv replace [options] <pattern> <replacement> [<input>]
    qsv replace --help

replace arguments:
    <pattern>              Regular expression to match.
    <replacement>          Replacement string. Set to '<NULL>' if you want to
                           replace matches with ''.
replace options:
    -i, --ignore-case      Case insensitive search. This is equivalent to
                           prefixing the regex with '(?i)'.
    -s, --select <arg>     Select the columns to search. See 'qsv select -h'
                           for the full syntax.
    -u, --unicode          Enable unicode support. When enabled, character classes
                           will match all unicode word characters instead of only
                           ASCII word characters. Decreases performance.
    --size-limit <mb>      Set the approximate size limit (MB) of the compiled
                           regular expression. If the compiled expression exceeds this 
                           number, then a compilation error is returned.
                           [default: 50]
    --dfa-size-limit <mb>  Set the approximate size of the cache (MB) used by the regular
                           expression engine's Discrete Finite Automata.
                           [default: 10]

Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
    -n, --no-headers       When set, the first row will not be interpreted
                           as headers. (i.e., They are not searched, analyzed,
                           sliced, etc.)
    -d, --delimiter <arg>  The field delimiter for reading CSV data.
                           Must be a single character. (default: ,)
    -p, --progressbar      Show progress bars. Not valid for stdin.
    -Q, --quiet            Do not print number of replacements to stderr.

"#;

use std::{borrow::Cow, collections::HashSet, env};

#[cfg(any(feature = "feature_capable", feature = "lite"))]
use indicatif::{HumanCount, ProgressBar, ProgressDrawTarget};
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
    arg_pattern:         String,
    arg_replacement:     String,
    flag_select:         SelectColumns,
    flag_unicode:        bool,
    flag_output:         Option<String>,
    flag_no_headers:     bool,
    flag_delimiter:      Option<Delimiter>,
    flag_ignore_case:    bool,
    flag_size_limit:     usize,
    flag_dfa_size_limit: usize,
    flag_progressbar:    bool,
    flag_quiet:          bool,
}

const NULL_VALUE: &str = "<NULL>";

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;
    let regex_unicode = match env::var("QSV_REGEX_UNICODE") {
        Ok(_) => true,
        Err(_) => args.flag_unicode,
    };
    let pattern = RegexBuilder::new(&args.arg_pattern)
        .case_insensitive(args.flag_ignore_case)
        .unicode(regex_unicode)
        .size_limit(args.flag_size_limit * (1 << 20))
        .dfa_size_limit(args.flag_dfa_size_limit * (1 << 20))
        .build()?;
    let replacement = if args.arg_replacement == NULL_VALUE {
        b""
    } else {
        args.arg_replacement.as_bytes()
    };
    let rconfig = Config::new(&args.arg_input)
        .delimiter(args.flag_delimiter)
        .no_headers(args.flag_no_headers)
        .select(args.flag_select);

    let mut rdr = rconfig.reader()?;
    let mut wtr = Config::new(&args.flag_output).writer()?;

    let headers = rdr.byte_headers()?.clone();
    let sel = rconfig.selection(&headers)?;

    // use a hash set for O(1) time complexity
    // instead of O(n) with the previous vector lookup
    let sel_indices: HashSet<&usize> = sel.iter().collect();

    if !rconfig.no_headers {
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
    let mut total_match_ctr: u64 = 0;
    #[cfg(any(feature = "feature_capable", feature = "lite"))]
    let mut rows_with_matches_ctr: u64 = 0;
    #[cfg(any(feature = "feature_capable", feature = "lite"))]
    let mut match_found;

    while rdr.read_byte_record(&mut record)? {
        #[cfg(any(feature = "feature_capable", feature = "lite"))]
        if show_progress {
            progress.inc(1);
        }

        #[cfg(any(feature = "feature_capable", feature = "lite"))]
        {
            match_found = false;
        }
        record = record
            .into_iter()
            .enumerate()
            .map(|(i, v)| {
                if sel_indices.contains(&i) {
                    if pattern.is_match(v) {
                        total_match_ctr += 1;
                        #[cfg(any(feature = "feature_capable", feature = "lite"))]
                        {
                            match_found = true;
                        }
                        pattern.replace_all(v, replacement)
                    } else {
                        Cow::Borrowed(v)
                    }
                } else {
                    Cow::Borrowed(v)
                }
            })
            .collect();

        #[cfg(any(feature = "feature_capable", feature = "lite"))]
        if match_found {
            rows_with_matches_ctr += 1;
        }

        wtr.write_byte_record(&record)?;
    }

    wtr.flush()?;

    #[cfg(any(feature = "feature_capable", feature = "lite"))]
    if show_progress {
        progress.set_message(format!(
            r#" - {} total matches replaced with "{}" in {} out of {} records."#,
            HumanCount(total_match_ctr),
            args.arg_replacement,
            HumanCount(rows_with_matches_ctr),
            HumanCount(progress.length().unwrap()),
        ));
        util::finish_progress(&progress);
    }

    if !args.flag_quiet {
        eprintln!("{total_match_ctr}");
    }
    if total_match_ctr == 0 {
        return Err(CliError::NoMatch());
    }

    Ok(())
}
