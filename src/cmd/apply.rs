use crate::regex::Regex;

use crate::chrono::prelude::*;
use crate::config::{Config, Delimiter};
use crate::currency::Currency;
use crate::dateparser::parse_with;
use crate::reverse_geocoder::{Locations, ReverseGeocoder};
use crate::select::SelectColumns;
use crate::serde::Deserialize;
use crate::util;
use crate::CliResult;

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
  * currencytonum: Gets the numeric value of a currency
  * emptyreplace: Replace empty string with <replacement> string
  * datefmt: formats a recognized date column to a specified format.
             Date recognition is powered by https://docs.rs/dateparser/
  * geocode: Geocodes to the nearest city given a Location column 
             '(lat, long)' or 'lat, long'

Replace empty strings with 'Unknown' in column Measurement:

  $ qsv apply emptyreplace --replacement Unknown Measurement file.csv

Format dates in OpenDate column to ISO 8601/RFC 3339 format:

  $ qsv apply datefmt OpenDate file.csv

Format dates in OpenDate column using '%Y-%m-%d' format:

  $ qsv apply datefmt OpenDate --formatstr %Y-%m-%d file.csv

Example for trimming and transforming to uppercase:

  $ qsv apply trim,upper surname -r uppercase_clean_surname file.csv

You can also use this command to make a copy of a column:

  $ qsv apply '' col -c col_copy file.csv

To geocode, use the --new-column option if you want to keep the Location column:

  $ qsv apply geocode Location --new-column City file.csv

Usage:
    qsv apply [options] <operations> <column> [<input>]
    qsv apply --help

apply options:
    -c, --new-column <name>     Put the transformed values in a new column instead.
    -r, --rename <name>         New name for the transformed column.
    -R, --replacement <string>  the string to use for emptyreplace operation.
                                (default: 'None')
    -f, --formatstr <string>    the date format to use when formatting dates. For formats, see
                                https://docs.rs/chrono/0.4.19/chrono/format/strftime/index.html
                                (default: '%+')

                                the place format to use when geocoding. The available formats are:
                                  - 'city-state' (default) - e.g. Brooklyn, New York
                                  - 'city-country' - Brooklyn, US 
                                  - 'city-state-county' | 'city-admin1-country' - Brooklyn, New York US
                                  - 'city' - Brooklyn
                                  - 'county' | 'admin2' - Kings County
                                  - 'state' | 'admin1' - New York
                                  - 'county-country' | 'admin2-country' - Kings County, US
                                  - 'county-state-country' | 'admin2-admin1-country' - Kings County, New York US
                                  - 'country' - US

Common options:
    -h, --help                  Display this message
    -o, --output <file>         Write output to <file> instead of stdout.
    -n, --no-headers            When set, the first row will not be interpreted
                                as headers.
    -d, --delimiter <arg>       The field delimiter for reading CSV data.
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
    "currencytonum",
    "emptyreplace",
    "datefmt",
    "geocode",
];

#[derive(Deserialize)]
struct Args {
    arg_column: SelectColumns,
    arg_operations: String,
    arg_input: Option<String>,
    flag_rename: Option<String>,
    flag_replacement: String,
    flag_formatstr: String,
    flag_new_column: Option<String>,
    flag_output: Option<String>,
    flag_no_headers: bool,
    flag_delimiter: Option<Delimiter>,
}

lazy_static! {
    static ref MIDNIGHT: chrono::NaiveTime = NaiveTime::from_hms(0, 0, 0);
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

    let operations: Vec<&str> = args.arg_operations.split(',').collect();

    let mut replacement = String::from("None");
    if !args.flag_replacement.is_empty() {
        replacement = args.flag_replacement.to_string();
    }

    let mut formatstr = String::from("%+");
    if !args.flag_formatstr.is_empty() {
        formatstr = args.flag_formatstr.to_string();
    }

    for op in &operations {
        if !OPERATIONS.contains(op) {
            return fail!(format!(
                "Unknown \"{}\" operations found in \"{}\"",
                op,
                operations.join(",")
            ));
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

    let loc = Locations::from_memory();
    let geocoder = ReverseGeocoder::new(&loc);
    // validating regex for "lat, long" or "(lat, long)"
    let locregex = Regex::new(
        r"([-+]?(?:[1-8]?\d(?:\.\d+)?|90(?:\.0+)?)),\s*([-+]?(?:180(?:\.0+)?|(?:(?:1[0-7]\d)|(?:[1-9]?\d))(?:\.\d+)?))\s*\)?$",
    )?;

    let squeezer = Regex::new(r"\s+")?;

    let mut record = csv::StringRecord::new();

    while rdr.read_record(&mut record)? {
        let mut cell = record[column_index].to_owned();

        for op in &operations {
            match op.as_ref() {
                "len" => {
                    cell = cell.len().to_string();
                }
                "lower" => {
                    cell = cell.to_lowercase();
                }
                "upper" => {
                    cell = cell.to_uppercase();
                }
                "squeeze" => {
                    cell = squeezer.replace_all(&cell, " ").to_string();
                }
                "trim" => {
                    cell = String::from(cell.trim());
                }
                "ltrim" => {
                    cell = String::from(cell.trim_start());
                }
                "rtrim" => {
                    cell = String::from(cell.trim_end());
                }
                "currencytonum" => {
                    let currency_value = Currency::from_str(&cell);
                    if let Ok(currency_val) = currency_value {
                        // its kludgy as currency is stored as BigInt, with
                        // 1 currency unit being 100 coins
                        let currency_coins = currency_val.value();
                        let coins = format!("{:03}", &currency_coins);
                        let coinlen = coins.len();
                        if coinlen > 2 && coins != "000" {
                            let decpoint = coinlen - 2;
                            let coin_num = &coins[..decpoint];
                            let coin_frac = &coins[decpoint..];
                            cell = format!("{}.{}", coin_num, coin_frac);
                        }
                    }
                }
                "emptyreplace" => {
                    if cell.trim().is_empty() {
                        cell = replacement.to_string();
                    }
                }
                "datefmt" => {
                    let parsed_date = parse_with(&cell, &Utc, *MIDNIGHT);
                    if let Ok(format_date) = parsed_date {
                        let formatted_date = format_date.format(&formatstr).to_string();
                        if formatted_date.ends_with("T00:00:00+00:00") {
                            cell = formatted_date[..10].to_string();
                        } else {
                            cell = formatted_date;
                        }
                    }
                }
                "geocode" => {
                    geocode(&locregex, &mut cell, &geocoder, &formatstr);
                }
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

fn geocode(locregex: &Regex, cell: &mut String, geocoder: &ReverseGeocoder, formatstr: &str) {
    let loccaps = locregex.captures(&*cell);
    if let Some(loccaps) = loccaps {
        let lats = loccaps.get(1).map_or("", |m| m.as_str());
        let longs = loccaps.get(2).map_or("", |m| m.as_str());
        let coords = (lats.parse::<f64>().unwrap(), longs.parse::<f64>().unwrap());
        let search_result = geocoder.search(coords);
        if let Some(locdetails) = search_result {
            *cell = match formatstr {
                "city-state" | "%+" => format!(
                    "{name}, {admin1}",
                    name = locdetails.record.name,
                    admin1 = locdetails.record.admin1,
                ),
                "city-country" => format!(
                    "{name}, {admin3}",
                    name = locdetails.record.name,
                    admin3 = locdetails.record.admin3
                ),
                "city-state-county" | "city-admin1-country" => format!(
                    "{name}, {admin1} {admin3}",
                    name = locdetails.record.name,
                    admin1 = locdetails.record.admin1,
                    admin3 = locdetails.record.admin3
                ),
                "city" => locdetails.record.name.to_string(),
                "county" | "admin2" => locdetails.record.admin2.to_string(),
                "state" | "admin1" => locdetails.record.admin1.to_string(),
                "county-country" | "admin2-country" => format!(
                    "{admin2}, {admin3}",
                    admin2 = locdetails.record.admin2,
                    admin3 = locdetails.record.admin3
                ),
                "county-state-country" | "admin2-admin1-country" => format!(
                    "{admin2}, {admin1} {admin3}",
                    admin2 = locdetails.record.admin2,
                    admin1 = locdetails.record.admin1,
                    admin3 = locdetails.record.admin3
                ),
                "country" => locdetails.record.admin3.to_string(),
                _ => locdetails.record.name.to_string(),
            };
        }
    }
}
