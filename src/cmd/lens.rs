static USAGE: &str = r#"
Explore a CSV file interactively using the csvlens (https://github.com/YS-L/csvlens) engine.

Press 'q' to exit. Press '?' for help.

Usage:
    qsv lens [options] [<input>]
    qsv lens --help

lens options:
  -d, --delimiter <char>           Delimiter character (comma by default)
                                   "auto" to auto-detect the delimiter
  -t, --tab-separated              Use tab separation. Shortcut for -d '\t'
      --no-headers                 Do not interpret the first row as headers
      --columns <regex>            Use this regex to select columns to display by default
      --filter <regex>             Use this regex to filter rows to display by default
      --find <regex>               Use this regex to find and highlight matches by default
  -i, --ignore-case                Searches ignore case. Ignored if any uppercase letters
                                   are present in the search string
      --echo-column <column_name>  Print the value of this column to stdout for the selected row
      --debug                      Show stats for debugging

Common options:
    -h, --help      Display this message
"#;

use csvlens::run_csvlens;
use serde::Deserialize;

use crate::{util, CliError, CliResult};

#[derive(Deserialize)]
struct Args {
    arg_input:          Option<String>,
    flag_delimiter:     Option<String>,
    flag_tab_separated: bool,
    flag_no_headers:    bool,
    flag_columns:       Option<String>,
    flag_filter:        Option<String>,
    flag_find:          Option<String>,
    flag_ignore_case:   bool,
    flag_echo_column:   Option<String>,
    flag_debug:         bool,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;

    let mut lens_args = Vec::new();

    if let Some(input) = &args.arg_input {
        lens_args.push(input.to_string());
    }

    if let Some(delimiter) = &args.flag_delimiter {
        lens_args.push(format!("--delimiter={delimiter}"));
    }

    if args.flag_tab_separated {
        lens_args.push("--tab-separated".to_string());
    }

    if args.flag_no_headers {
        lens_args.push("--no-headers".to_string());
    }

    if let Some(columns) = &args.flag_columns {
        lens_args.push(format!("--columns={columns}"));
    }

    if let Some(filter) = &args.flag_filter {
        lens_args.push(format!("--filter={filter}"));
    }

    if let Some(find) = &args.flag_find {
        lens_args.push(format!("--find={find}"));
    }

    if args.flag_ignore_case {
        lens_args.push("--ignore-case".to_string());
    }

    if let Some(echo_column) = &args.flag_echo_column {
        lens_args.push(format!("--echo-column {echo_column}"));
    }

    if args.flag_debug {
        lens_args.push("--debug".to_string());
    }

    let out =
        run_csvlens(&lens_args).map_err(|e| CliError::Other(format!("csvlens error: {e}")))?;

    if let Some(selected_cell) = out {
        println!("{selected_cell}");
    }

    Ok(())
}
