static USAGE: &str = r#"
Filters CSV data by whether the given regex set matches a row.

Unlike the search operation, this allows regex matching of multiple regexes 
in a single pass.

The regexset-file is a plain text file with multiple regexes, with a regex on 
each line.

The regex set is applied to each field in each row, and if any field matches,
then the row is written to the output, and the number of matches to stderr.

The columns to search can be limited with the '--select' flag (but the full row
is still written to the output if there is a match).

Returns exitcode 0 when matches are found, returning number of matches to stderr.
Returns exitcode 1 when no match is found.

When --quick is enabled, no output is produced and exitcode 0 is returned on 
the first match.

For examples, see https://github.com/jqnatividad/qsv/blob/master/tests/test_searchset.rs.

Usage:
    qsv searchset [options] (<regexset-file>) [<input>]
    qsv searchset --help

search options:
    -i, --ignore-case          Case insensitive search. This is equivalent to
                               prefixing the regex with '(?i)'.
    -s, --select <arg>         Select the columns to search. See 'qsv select -h'
                               for the full syntax.
    -v, --invert-match         Select only rows that did not match
    -u, --unicode              Enable unicode support. When enabled, character classes
                               will match all unicode word characters instead of only
                               ASCII word characters. Decreases performance.

    -f, --flag <column>        If given, the command will not filter rows
                               but will instead flag the found rows in a new
                               column named <column>. For each found row, <column>
                               is set to the row number of the row, followed by a
                               semicolon, then a list of the matching regexes.
    --flag-matches-only        When --flag is enabled, only rows that match are
                               sent to output. Rows that do not match are filtered.
    --unmatched-output <file>  When --flag-matches-only is enabled, output the rows
                               that did not match to <file>.

    -q, --quick                Return on first match with an exitcode of 0, returning
                               the row number of the first match to stderr.
                               Return exit code 1 if no match is found.
                               No output is produced. Ignored if --json is enabled.
    -c, --count                Return number of matches to stderr.
                               Ignored if --json is enabled.
    -j, --json                 Return number of matches, number of rows with matches,
                               and number of rows to stderr in JSON format.
    --size-limit <mb>          Set the approximate size limit (MB) of the compiled
                               regular expression. If the compiled expression exceeds this 
                               number, then a compilation error is returned.
                               Modify this only if you're getting regular expression
                               compilation errors. [default: 50]
    --dfa-size-limit <mb>      Set the approximate size of the cache (MB) used by the regular
                               expression engine's Discrete Finite Automata.
                               Modify this only if you're getting regular expression
                               compilation errors. [default: 10]

Common options:
    -h, --help                 Display this message
    -o, --output <file>        Write output to <file> instead of stdout.
    -n, --no-headers           When set, the first row will not be interpreted
                               as headers. (i.e., They are not searched, analyzed,
                               sliced, etc.)
    -d, --delimiter <arg>      The field delimiter for reading CSV data.
                               Must be a single character. (default: ,)
    -p, --progressbar          Show progress bars. Not valid for stdin.
    -Q, --quiet                Do not return number of matches to stderr.
"#;

use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

#[cfg(any(feature = "feature_capable", feature = "lite"))]
use indicatif::{HumanCount, ProgressBar, ProgressDrawTarget};
use log::{debug, info};
use regex::{bytes::RegexSetBuilder, Regex};
use serde::Deserialize;
use serde_json::json;

use crate::{
    config::{Config, Delimiter},
    select::SelectColumns,
    util, CliError, CliResult,
};

#[allow(dead_code)]
#[derive(Deserialize)]
struct Args {
    arg_input:              Option<String>,
    arg_regexset_file:      String,
    flag_select:            SelectColumns,
    flag_output:            Option<String>,
    flag_no_headers:        bool,
    flag_delimiter:         Option<Delimiter>,
    flag_invert_match:      bool,
    flag_unicode:           bool,
    flag_ignore_case:       bool,
    flag_flag:              Option<String>,
    flag_flag_matches_only: bool,
    flag_unmatched_output:  Option<String>,
    flag_size_limit:        usize,
    flag_dfa_size_limit:    usize,
    flag_quick:             bool,
    flag_count:             bool,
    flag_json:              bool,
    flag_progressbar:       bool,
    flag_quiet:             bool,
}

fn read_regexset(filename: &String) -> io::Result<Vec<String>> {
    match File::open(filename) {
        Ok(f) => BufReader::new(f).lines().collect(),
        Err(e) => Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Cannot open regexset file {filename}: {e}"),
        )),
    }
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;

    if args.flag_flag.is_none() && args.flag_flag_matches_only {
        return fail_incorrectusage_clierror!("Cannot use --flag-matches-only without --flag",);
    }
    if !args.flag_flag_matches_only && args.flag_unmatched_output.is_some() {
        return fail_incorrectusage_clierror!(
            "Cannot use --unmatched-output without --flag-matches-only",
        );
    }

    let regexset = read_regexset(&args.arg_regexset_file)?;

    let mut regex_labels: Vec<String> = Vec::with_capacity(regexset.len());
    let labels_re = Regex::new(r".?#(?P<label>.*)$").unwrap();

    // use regex comment labels if they exist, so matches are easier to understand
    for (i, regex) in regexset.iter().enumerate() {
        let label = labels_re
            .captures(regex)
            .and_then(|cap| cap.name("label"))
            .map_or_else(|| (i + 1).to_string(), |m| m.as_str().to_string());
        regex_labels.push(label);
    }

    let regex_unicode = if util::get_envvar_flag("QSV_REGEX_UNICODE") {
        true
    } else {
        args.flag_unicode
    };

    debug!("Compiling {} regex set expressions...", regexset.len());
    let pattern = RegexSetBuilder::new(&regexset)
        .case_insensitive(args.flag_ignore_case)
        .unicode(regex_unicode)
        .size_limit(args.flag_size_limit * (1 << 20))
        .dfa_size_limit(args.flag_dfa_size_limit * (1 << 20))
        .build()?;
    debug!("Successfully compiled regex set!");

    let rconfig = Config::new(&args.arg_input)
        .delimiter(args.flag_delimiter)
        .no_headers(args.flag_no_headers)
        .select(args.flag_select);

    let mut rdr = rconfig.reader()?;
    let mut wtr = Config::new(&args.flag_output).writer()?;
    let mut unmatched_wtr = Config::new(&args.flag_unmatched_output).writer()?;

    let mut headers = rdr.byte_headers()?.clone();
    let sel = rconfig.selection(&headers)?;

    let do_match_list = args.flag_flag.map_or(false, |column_name| {
        headers.push_field(column_name.as_bytes());
        true
    });

    if !rconfig.no_headers && !args.flag_quick {
        wtr.write_record(&headers)?;
    }

    let record_count = util::count_rows(&rconfig)?;
    // prep progress bar
    #[cfg(any(feature = "feature_capable", feature = "lite"))]
    let show_progress =
        (args.flag_progressbar || util::get_envvar_flag("QSV_PROGRESSBAR")) && !rconfig.is_stdin();
    #[cfg(any(feature = "feature_capable", feature = "lite"))]
    let progress = ProgressBar::with_draw_target(None, ProgressDrawTarget::stderr_with_hz(5));
    #[cfg(any(feature = "feature_capable", feature = "lite"))]
    if show_progress {
        util::prep_progress(&progress, record_count);
    } else {
        progress.set_draw_target(ProgressDrawTarget::hidden());
    }

    let mut record = csv::ByteRecord::new();
    let mut flag_rowi: u64 = 1;
    let mut match_row_ctr: u64 = 0;
    let mut total_matches: u64 = 0;
    let mut row_ctr: u64 = 0;

    // minimize allocs
    #[allow(unused_assignments)]
    let mut flag_column: Vec<u8> = Vec::with_capacity(20);
    let mut match_list_vec = Vec::with_capacity(20);
    #[allow(unused_assignments)]
    let mut match_list = String::with_capacity(20);
    let mut matched_rows = String::with_capacity(20);
    #[allow(unused_assignments)]
    let mut match_list_with_row = String::with_capacity(20);
    let mut m;
    let mut matched = false;
    let mut matches: Vec<usize> = Vec::with_capacity(20);
    let mut buffer = itoa::Buffer::new();

    while rdr.read_byte_record(&mut record)? {
        row_ctr += 1;
        #[cfg(any(feature = "feature_capable", feature = "lite"))]
        if show_progress {
            progress.inc(1);
        }
        m = sel.select(&record).any(|f| {
            matched = pattern.is_match(f);
            if matched && do_match_list {
                matches = pattern.matches(f).into_iter().collect();
                total_matches += matches.len() as u64;
                for j in &mut matches {
                    *j += 1; // so the list is human readable - i.e. not zero-based
                }
                match_list_vec.clone_from(&matches);
            }
            matched
        });
        if args.flag_invert_match {
            m = !m;
        }
        if m {
            match_row_ctr += 1;
            if args.flag_quick {
                break;
            }
        }

        if do_match_list {
            flag_rowi += 1;
            flag_column = if m {
                buffer.format(flag_rowi).clone_into(&mut matched_rows);
                if args.flag_invert_match {
                    matched_rows.as_bytes().to_vec()
                } else {
                    match_list = match_list_vec
                        .iter()
                        .map(|i| regex_labels[*i - 1].clone())
                        .collect::<Vec<String>>()
                        .join(",");
                    match_list_with_row = format!("{matched_rows};{match_list}");
                    match_list_with_row.as_bytes().to_vec()
                }
            } else {
                b"0".to_vec()
            };
            if args.flag_flag_matches_only && !m {
                if args.flag_unmatched_output.is_some() {
                    unmatched_wtr.write_byte_record(&record)?;
                }
                continue;
            }
            record.push_field(&flag_column);
            wtr.write_byte_record(&record)?;
        } else if m {
            wtr.write_byte_record(&record)?;
        }
    }
    unmatched_wtr.flush()?;
    wtr.flush()?;

    #[cfg(any(feature = "feature_capable", feature = "lite"))]
    if show_progress {
        if do_match_list {
            progress.set_message(format!(
                " - {} total matches in {} rows with matches found in {} records.",
                HumanCount(total_matches),
                HumanCount(match_row_ctr),
                HumanCount(record_count),
            ));
        } else {
            progress.set_message(format!(
                " - {} rows with matches found in {} records.",
                HumanCount(match_row_ctr),
                HumanCount(record_count),
            ));
        }
        util::finish_progress(&progress);
    }

    if args.flag_json {
        let json = json!({
            "rows_with_matches": match_row_ctr,
            "total_matches": total_matches,
            "record_count": record_count,
        });
        eprintln!("{json}");
    } else {
        if args.flag_count && !args.flag_quick {
            if !args.flag_quiet {
                eprintln!("{match_row_ctr}");
            }
            info!("matches: {match_row_ctr}");
        }

        if match_row_ctr == 0 {
            return Err(CliError::NoMatch());
        } else if args.flag_quick {
            if !args.flag_quiet {
                eprintln!("{row_ctr}");
            }
            info!("quick searchset first match at {row_ctr}");
        }
    }

    Ok(())
}
