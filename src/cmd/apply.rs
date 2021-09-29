use crate::regex::Regex;

use crate::CliResult;
use crate::config::{Delimiter, Config};
use crate::select::SelectColumns;
use crate::util;
use crate::serde::Deserialize;

static USAGE: &str = "
Apply a series of unary functions to a given CSV column. This can be used to
perform typical cleaning tasks and/or harmonize some values etc.

The series of operations must be given separated by commas as such:

  trim => Trimming the cell
  trim,upper => Trimming the cell then transforming to uppercase
  '' => No-op

Currently supported operations:

  * len: Return string length
  * lower: Transform to lowercase
  * upper: Transform to uppercase
  * squeeze: Compress consecutive whitespaces
  * trim: Trim (drop whitespace left & right of the string)
  * ltrim: Left trim
  * rtrim: Right trim
  * empty0: Replace empty string with '0'
  * emptyNA: Replace emptry string with 'NA'

Example for trimming and transforming to uppercase:

  $ qsv apply trim,upper surname -r uppercase_clean_surname file.csv

You can also use this command to make a copy of a column:

  $ qsv apply '' col -c col_copy file.csv

Usage:
    qsv apply [options] <operations> <column> [<input>]
    qsv apply --help

apply options:
    -c, --new-column <name>  Put the transformed values in a new column instead.
    -r, --rename <name>      New name for the transformed column.

Common options:
    -h, --help               Display this message
    -o, --output <file>      Write output to <file> instead of stdout.
    -n, --no-headers         When set, the first row will not be interpreted
                             as headers.
    -d, --delimiter <arg>    The field delimiter for reading CSV data.
                             Must be a single character. (default: ,)
";

static OPERATIONS: &[&str] = &[
    "len",
    "lower",
    "upper",
    "squeeze",
    "trim",
    "rtrim",
    "ltrim",
    "empty0",
    "emptyNA",
];

#[derive(Deserialize)]
struct Args {
    arg_column: SelectColumns,
    arg_operations: String,
    arg_input: Option<String>,
    flag_rename: Option<String>,
    flag_new_column: Option<String>,
    flag_output: Option<String>,
    flag_no_headers: bool,
    flag_delimiter: Option<Delimiter>,
}

pub fn replace_column_value(record: &csv::StringRecord, column_index: usize, new_value: &String)
                           -> csv::StringRecord {
    record
        .into_iter()
        .enumerate()
        .map(|(i, v)| if i == column_index { new_value } else { v })
        .collect()
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
    let column_index = *sel.iter().next().unwrap();

    let mut headers = rdr.headers()?.clone();

    let operations: Vec<&str> = args.arg_operations.split(",").collect();

    for op in &operations {
        if !OPERATIONS.contains(&op) {
            return fail!(format!("Unknown \"{}\" operations found in \"{}\"", op, operations.join(",")));
        }
    }

    if let Some(new_name) = args.flag_rename {
        headers = replace_column_value(&headers, column_index, &new_name);
    }

    if !rconfig.no_headers {

        if let Some(new_column) = &args.flag_new_column {
            headers.push_field(new_column);
        }

        wtr.write_record(&headers)?;
    }

    let squeezer = Regex::new(r"\s+")?;

    let mut record = csv::StringRecord::new();

    while rdr.read_record(&mut record)? {
        let mut cell = record[column_index].to_owned();

        for op in &operations {
            match op.as_ref() {
                "len" => {
                    cell = cell.len().to_string();
                },
                "lower" => {
                    cell = cell.to_lowercase();
                },
                "upper" => {
                    cell = cell.to_uppercase();
                },
                "squeeze" => {
                    cell = squeezer.replace_all(&cell, " ").to_string();
                },
                "trim" => {
                    cell = String::from(cell.trim());
                },
                "ltrim" => {
                    cell = String::from(cell.trim_start());
                },
                "rtrim" => {
                    cell = String::from(cell.trim_end());
                },
                "empty0" => {
                    if cell.trim().is_empty() {
                        cell = "0".to_string();
                    }
                },
                "emptyNA" => {
                    if cell.trim().is_empty() {
                        cell = "NA".to_string();
                    }
                },
                _ => {}
            }
        }

        match &args.flag_new_column {
            Some(_) => {
                record.push_field(&cell);
            }
            None => {
                record = replace_column_value(&record, column_index, &cell);
            }
        }

        wtr.write_record(&record)?;
    }

    Ok(wtr.flush()?)
}
