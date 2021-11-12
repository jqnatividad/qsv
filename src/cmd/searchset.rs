use regex::bytes::RegexSetBuilder;

use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::path::Path;
use std::env;

use crate::config::{Config, Delimiter};
use crate::select::SelectColumns;
use crate::util;
use crate::CliResult;
use serde::Deserialize;

static USAGE: &str = "
Filters CSV data by whether the given regex set matches a row.

Unlike the search operation, this allows regex matching of multiple regexes 
in a single pass.

The regexset-file is a plain text file with multiple regexes, with a regex on 
each line.

The regex set is applied to each field in each row, and if any field matches,
then the row is written to the output. The columns to search can be limited
with the '--select' flag (but the full row is still written to the output if
there is a match).

Usage:
    qsv searchset [options] (<regexset-file>) [<input>]
    qsv searchset --help

search options:
    -i, --ignore-case      Case insensitive search. This is equivalent to
                           prefixing the regex with '(?i)'.
    -s, --select <arg>     Select the columns to search. See 'qsv select -h'
                           for the full syntax.
    -v, --invert-match     Select only rows that did not match
    -u, --unicode          Enable unicode support. When enabled, character classes
                           will match all unicode word characters instead of only
                           ASCII word characters. Decreases performance.

Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
    -n, --no-headers       When set, the first row will not be interpreted
                           as headers. (i.e., They are not searched, analyzed,
                           sliced, etc.)
    -d, --delimiter <arg>  The field delimiter for reading CSV data.
                           Must be a single character. (default: ,)
    -f, --flag <column>    If given, the command will not filter rows
                           but will instead flag the found rows in a new
                           column named <column>. For each found row, <column>
                           is set to the row number of the row, followed by a
                           semicolon, then a list of the matching regexes.
";

#[derive(Deserialize)]
struct Args {
    arg_input: Option<String>,
    arg_regexset_file: String,
    flag_select: SelectColumns,
    flag_output: Option<String>,
    flag_no_headers: bool,
    flag_delimiter: Option<Delimiter>,
    flag_invert_match: bool,
    flag_unicode: bool,
    flag_ignore_case: bool,
    flag_flag: Option<String>,
}

fn read_regexset(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    match File::open(filename) {
        Ok(f) => BufReader::new(f).lines().collect(),
        Err(_) => Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Cannot open regexset file.",
        )),
    }
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;

    let regexset = read_regexset(&*args.arg_regexset_file)?;
    let regex_unicode = match env::var("QSV_REGEX_UNICODE") {
        Ok(_) => true,
        Err(_) => args.flag_unicode,
    };
    let pattern = RegexSetBuilder::new(&regexset)
        .case_insensitive(args.flag_ignore_case)
        .unicode(regex_unicode)
        .build()?;
    let rconfig = Config::new(&args.arg_input)
        .delimiter(args.flag_delimiter)
        .no_headers(args.flag_no_headers)
        .select(args.flag_select);

    let mut rdr = rconfig.reader()?;
    let mut wtr = Config::new(&args.flag_output).writer()?;

    let mut headers = rdr.byte_headers()?.clone();
    let sel = rconfig.selection(&headers)?;

    if let Some(column_name) = args.flag_flag.clone() {
        headers.push_field(column_name.as_bytes());
    }

    if !rconfig.no_headers {
        wtr.write_record(&headers)?;
    }

    let mut match_list: String = String::from("");
    let do_match_list = args.flag_flag.is_some();

    let mut record = csv::ByteRecord::new();
    let mut flag_rowi: u64 = 1;
    let mut _matched_rows = String::from("");
    let mut _match_list_with_row = String::from("");
    while rdr.read_byte_record(&mut record)? {
        let mut m = sel.select(&record).any(|f| {
            let matched = pattern.is_match(f);
            if matched && do_match_list {
                let mut matches: Vec<_> = pattern.matches(f).into_iter().collect();
                for j in matches.iter_mut() {
                    *j += 1; // so the list is human readable - i.e. not zero-based
                }
                match_list = format!("{:?}", matches);
            }
            matched
        });
        if args.flag_invert_match {
            m = !m;
        }

        if do_match_list {
            flag_rowi += 1;
            record.push_field(if m {
                _matched_rows = flag_rowi.to_string();
                if args.flag_invert_match {
                    _matched_rows.as_bytes()
                } else {
                    _match_list_with_row = format!("{};{}", _matched_rows, match_list);
                    _match_list_with_row.as_bytes()
                }
            } else {
                b"0"
            });
            wtr.write_byte_record(&record)?;
        } else if m {
            wtr.write_byte_record(&record)?;
        }
    }
    Ok(wtr.flush()?)
}
