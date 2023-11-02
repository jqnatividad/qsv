static USAGE: &str = r#"
Pseudonymise the value of the given column by replacing them by an
incremental identifier.

For examples, see https://github.com/jqnatividad/qsv/blob/master/tests/test_pseudo.rs.

Usage:
    qsv pseudo [options] <column> [<input>]
    qsv pseudo --help

Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
    -n, --no-headers       When set, the first row will not be interpreted
                           as headers.
    -d, --delimiter <arg>  The field delimiter for reading CSV data.
                           Must be a single character. (default: ,)
"#;

use ahash::AHashMap;
use serde::Deserialize;

use crate::{
    config::{Config, Delimiter},
    select::SelectColumns,
    util, CliResult,
};

#[derive(Deserialize)]
struct Args {
    arg_column:      SelectColumns,
    arg_input:       Option<String>,
    flag_output:     Option<String>,
    flag_no_headers: bool,
    flag_delimiter:  Option<Delimiter>,
}

pub fn replace_column_value(
    record: &csv::StringRecord,
    column_index: usize,
    new_value: &str,
) -> csv::StringRecord {
    record
        .into_iter()
        .enumerate()
        .map(|(i, v)| if i == column_index { new_value } else { v })
        .collect()
}

type Values = AHashMap<String, u64>;

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;
    let rconfig = Config::new(&args.arg_input)
        .delimiter(args.flag_delimiter)
        .no_headers(args.flag_no_headers)
        .select(args.arg_column);

    let mut rdr = rconfig.reader()?;
    let mut wtr = Config::new(&args.flag_output).writer()?;

    let headers = rdr.byte_headers()?.clone();
    let column_index = match rconfig.selection(&headers) {
        Ok(sel) => {
            let sel_len = sel.len();
            if sel_len > 1 {
                return fail_incorrectusage_clierror!(
                    "{sel_len} columns selected. Only one column can be selected for \
                     pseudonymisation."
                );
            }
            // safety: we checked that sel.len() == 1
            *sel.iter().next().unwrap()
        },
        Err(e) => return fail_clierror!("{e}"),
    };

    if !rconfig.no_headers {
        wtr.write_record(&headers)?;
    }

    let mut record = csv::StringRecord::new();
    let mut values = Values::new();
    let mut counter: u64 = 0;

    while rdr.read_record(&mut record)? {
        let value = record[column_index].to_owned();

        if let Some(id) = values.get(&value) {
            record = replace_column_value(&record, column_index, &id.to_string());
        } else {
            values.insert(value, counter);
            record = replace_column_value(&record, column_index, &counter.to_string());
            counter += 1;
        }
        wtr.write_record(&record)?;
    }
    Ok(wtr.flush()?)
}
