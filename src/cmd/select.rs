static USAGE: &str = r#"
Select columns from CSV data efficiently.

This command lets you manipulate the columns in CSV data. You can re-order
them, duplicate them or drop them. Columns can be referenced by index or by
name if there is a header row (duplicate column names can be disambiguated with
more indexing). Column ranges can also be specified. Finally, columns can be
selected using regular expressions.

  Select the first and fourth columns:
  $ qsv select 1,4

  Select the first 4 columns (by index and by name):
  $ qsv select 1-4
  $ qsv select Header1-Header4

  Ignore the first 2 columns (by range and by omission):
  $ qsv select 3-
  $ qsv select '!1-2'

  Select the third column named 'Foo':
  $ qsv select 'Foo[2]'

  Select the first and last columns, _ is a special character for the last column:
  $ qsv select 1,_

  Reverse the order of columns:
  $ qsv select _-1

  Sort the columns lexicographically. Note that you must provide a dummy selector:
  $ qsv select 1 --sort

  Randomly shuffle the columns:
  $ qsv select 1 --random
  # with a seed
  $ qsv select 1 --random --seed 42

  Select columns using a regex using '/<regex>/':
  $ qsv select /^a/
  $ qsv select '/^.*\d.*$/'
  # remove SSN, account_no and password columns
  $ qsv select '!/SSN|account_no|password/'

  Re-order and duplicate columns arbitrarily:
  $ qsv select 3-1,Header3-Header1,Header1,Foo[2],Header1

  Quote column names that conflict with selector syntax:
  $ qsv select '\"Date - Opening\",\"Date - Actual Closing\"'

For more examples, see https://github.com/jqnatividad/qsv/blob/master/tests/test_select.rs.

Usage:
    qsv select [options] [--] <selection> [<input>]
    qsv select --help

select options:
These options only apply to the `select` command, not the `--select` flag in other commands.
Be sure to provide a dummy selector (e.g. '1') to avoid command-line parsing errors.

    -R, --random           Randomly reorder the columns. 
    --seed <number>        Seed for the random number generator.

    -S, --sort             Sort the columns lexicographically, i.e. by their
                           byte values.

Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
    -n, --no-headers       When set, the first row will not be interpreted
                           as headers. (i.e., They are not searched, analyzed,
                           sliced, etc.)
    -d, --delimiter <arg>  The field delimiter for reading CSV data.
                           Must be a single character. (default: ,)
"#;

use rand::{seq::SliceRandom, SeedableRng};
use serde::Deserialize;

use crate::{
    config::{Config, Delimiter},
    select::SelectColumns,
    util, CliResult,
};

#[derive(Deserialize)]
struct Args {
    arg_input:       Option<String>,
    arg_selection:   SelectColumns,
    flag_random:     bool,
    flag_seed:       Option<u64>,
    flag_sort:       bool,
    flag_output:     Option<String>,
    flag_no_headers: bool,
    flag_delimiter:  Option<Delimiter>,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let mut args: Args = util::get_args(USAGE, argv)?;

    if args.flag_sort && args.flag_random {
        return fail_clierror!("Cannot use both --random and --sort flags.");
    }

    if args.flag_random {
        // get the number of columns
        let num_cols = Config::new(&args.arg_input)
            .delimiter(args.flag_delimiter)
            .no_headers(true)
            .reader()?
            .byte_headers()?
            .len();

        // make a vector of the column indices (1-indexed).
        let mut original_selection: Vec<usize> = (1..=num_cols).collect();

        // Use seed if it is provided.
        let mut rng = if let Some(seed) = args.flag_seed {
            rand::rngs::StdRng::seed_from_u64(seed) // DevSkim: ignore DS148264
        } else {
            rand::rngs::StdRng::from_entropy()
        };

        // Shuffle the vector of column indices.
        original_selection.shuffle(&mut rng);

        // Convert the shuffled indices into a comma-separated string.
        let randomized_selection = original_selection
            .into_iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join(",");

        // Parse the shuffled string into a SelectColumns object.
        args.arg_selection = SelectColumns::parse(&randomized_selection)?;
    }

    if args.flag_sort {
        // get the headers
        let headers = Config::new(&args.arg_input)
            .delimiter(args.flag_delimiter)
            .reader()?
            .byte_headers()?
            .clone();

        // sort the headers lexicographically
        let mut sorted_headers = Vec::with_capacity(headers.len());
        sorted_headers.extend(headers.iter().map(<[u8]>::to_vec));
        sorted_headers.sort_unstable();

        // make a comma-separated string of the sorted, quoted headers
        let sorted_selection = sorted_headers
            .iter()
            .map(|h| format!("\"{}\"", String::from_utf8_lossy(h)))
            .collect::<Vec<String>>()
            .join(",");

        args.arg_selection = SelectColumns::parse(&sorted_selection)?;
    }

    let rconfig = Config::new(&args.arg_input)
        .delimiter(args.flag_delimiter)
        .no_headers(args.flag_no_headers)
        .select(args.arg_selection);

    let mut rdr = rconfig.reader()?;
    let mut wtr = Config::new(&args.flag_output).writer()?;

    let headers = rdr.byte_headers()?.clone();
    let sel = rconfig.selection(&headers)?;

    if !rconfig.no_headers {
        wtr.write_record(sel.iter().map(|&i| &headers[i]))?;
    }
    let mut record = csv::ByteRecord::new();
    while rdr.read_byte_record(&mut record)? {
        wtr.write_record(sel.iter().map(|&i| &record[i]))?;
    }
    wtr.flush()?;
    Ok(())
}
