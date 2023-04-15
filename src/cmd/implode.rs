static USAGE: &str = r#"
Implodes/folds multiple values of a field into cells of groups by combining 
column values based on the given separator.

For instance the following CSV:

name,color
John,blue
John,yellow
Mary,red

Can be imploded/folded on the "color" <column> based on the "|" <separator>:

qsv implode color "|" --rename colors <input.csv>

name,colors
John,blue|yellow
Mary,red


Usage:
    qsv implode [options] <column> <separator> [<input>]
    qsv implode --help

implode options:
    -r, --rename <name>    New name for the imploded/folded column.
    -j, --jobs <arg>       The number of jobs to run in parallel.
                           This works better when the given CSV data has
                           an index already created.
                           When not set, the number of jobs is set to the
                           number of CPUs detected.

Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
    -n, --no-headers       When set, the first row will not be interpreted
                           as headers.
    -d, --delimiter <arg>  The field delimiter for reading CSV data.
                           Must be a single character. (default: ,)
    --no-memcheck          Do not check if there is enough memory to load the
                           entire CSV into memory.
"#;

use ahash::AHashMap;
use csv::ByteRecord;
use log::debug;
use serde::Deserialize;
use stats::Frequencies;

use crate::{
    config::{Config, Delimiter},
    select::SelectColumns,
    util, CliResult,
};
#[derive(Deserialize)]
struct Args {
    arg_column:       SelectColumns,
    arg_separator:    String,
    arg_input:        Option<String>,
    flag_rename:      Option<String>,
    flag_jobs:        Option<usize>,
    flag_output:      Option<String>,
    flag_no_headers:  bool,
    flag_delimiter:   Option<Delimiter>,
    flag_no_memcheck: bool,
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

    if let Some(new_name) = args.flag_rename {
        headers = replace_column_value(&headers, column_index, &new_name);
    }

    if !rconfig.no_headers {
        wtr.write_record(&headers)?;
    }

    let mut record = csv::StringRecord::new();

    // struct Column {
    //     name: String,
    //     values: Vec<String>,
    // }

    // // first pass: read all records and collect the values for the column
    // // we want to implode. Put the values in a hashmap keyed by the selected column/s.
    // let mut columns_map = HashMap::new();
    // let mut values_map = HashMap::new();
    // while rdr.read_record(&mut record)? {
    //     let key = record[column_index].to_string();
    //     // let value = record[column_index].to_string();
    //     let value = "".to_string();
    //     columns_map.entry(key).or_insert(vec![]).push(value);
    // }

    // second pass: iterate over the CSV records again
    // for each record, check if the selected column is in the hashmap
    // if it is, then implode the values, making sure

    // while rdr.read_record(&mut record)? {

    //     // for val in record[column_index].split(&args.arg_separator) {
    //     //     let new_record = replace_column_value(&record, column_index, val);
    //     //     wtr.write_record(&new_record)?;
    //     // }
    // }

    Ok(wtr.flush()?)
}

/// get frequency tables from `cmd::frequency`
/// returns map of unique values keyed by header
fn get_unique_values(
    args: &Args,
    column_select_arg: &str,
) -> CliResult<AHashMap<String, Vec<String>>> {
    // prepare arg for invoking cmd::frequency
    let freq_args = crate::cmd::frequency::Args {
        arg_input:        args.arg_input.clone(),
        flag_select:      crate::select::SelectColumns::parse(column_select_arg).unwrap(),
        flag_limit:       0, // get all unique values for a column
        flag_asc:         false,
        flag_no_nulls:    true,
        flag_jobs:        Some(util::njobs(args.flag_jobs)),
        flag_output:      None,
        flag_no_headers:  args.flag_no_headers,
        flag_delimiter:   args.flag_delimiter,
        flag_no_memcheck: args.flag_no_memcheck,
    };

    let (headers, ftables) = match freq_args.rconfig().indexed()? {
        Some(ref mut idx) => freq_args.parallel_ftables(idx),
        _ => freq_args.sequential_ftables(),
    }?;

    let unique_values_map = construct_map_of_unique_values(&headers, &ftables)?;

    Ok(unique_values_map)
}

/// construct map of unique values keyed by header
fn construct_map_of_unique_values(
    freq_csv_fields: &ByteRecord,
    frequency_tables: &[Frequencies<Vec<u8>>],
) -> CliResult<AHashMap<String, Vec<String>>> {
    let mut unique_values_map: AHashMap<String, Vec<String>> = AHashMap::new();
    let mut unique_values = Vec::with_capacity(freq_csv_fields.len());

    // iterate through fields and gather unique values for each field
    for (i, header_byte_slice) in freq_csv_fields.iter().enumerate() {
        unique_values.clear();

        for (val_byte_vec, _count) in frequency_tables[i].most_frequent() {
            let val_string = convert_to_string(val_byte_vec.as_slice())?;
            unique_values.push(val_string);
        }

        let header_string = convert_to_string(header_byte_slice)?;

        // sort the values so enum list so schema can be diff'ed between runs
        unique_values.sort_unstable();

        if log::log_enabled!(log::Level::Debug) {
            // we do this as this debug is relatively expensive
            debug!(
                "enum[{header_string}]: len={}, val={:?}",
                unique_values.len(),
                unique_values
            );
        }
        unique_values_map.insert(header_string, unique_values.clone());
    }

    // dbg!(&unique_values_map);

    Ok(unique_values_map)
}

/// convert byte slice to UTF8 String
#[inline]
fn convert_to_string(byte_slice: &[u8]) -> CliResult<String> {
    // convert csv header to string
    if let Ok(s) = simdutf8::basic::from_utf8(byte_slice) {
        Ok(s.to_string())
    } else {
        let lossy_string = String::from_utf8_lossy(byte_slice);
        fail_clierror!(
            "Can't convert byte slice to utf8 string. slice={byte_slice:?}: {lossy_string}"
        )
    }
}
