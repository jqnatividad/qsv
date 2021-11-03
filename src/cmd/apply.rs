use lazy_static::lazy_static;
use regex::Regex;

use crate::config::{Config, Delimiter};
use crate::select::SelectColumns;
use crate::util;
use crate::CliResult;
use chrono::{NaiveTime, Utc};
use currency::Currency;
use dateparser::parse_with;
use indicatif::ProgressBar;
use natural::phonetics::soundex;
use reverse_geocoder::{Locations, ReverseGeocoder};
use serde::Deserialize;
use strsim::{
    damerau_levenshtein, hamming, jaro_winkler, normalized_damerau_levenshtein, osa_distance,
    sorensen_dice,
};
use titlecase::titlecase;

static USAGE: &str = "
Apply a series of transformation functions to a given CSV column. This can be used to
perform typical cleaning tasks and/or harmonize some values, etc.

It has several subcommands:

OPERATIONS
The series of operations must be given separated by commas as such:

  trim => Trimming the cell
  trim,upper => Trimming the cell then transforming to uppercase
  lower,simdln => Lowercase the cell, then compute the Damerau-Levenshtein similarity

Currently supported operations:

  * len: Return string length
  * lower: Transform to lowercase (unicode-aware)
  * asciilower: Transform to lowecase (ascii-only) ~10% faster
  * upper: Transform to uppercase (unicode-aware)
  * asciiupper: Transform to uppercase (ascii-only) ~10% faster
  * squeeze: Compress consecutive whitespaces
  * trim: Trim (drop whitespace left & right of the string)
  * ltrim: Left trim
  * rtrim: Right trim
  * mtrim: Trims --comparand matches left & right of the string
  * mltrim: Left trim --comparand matches
  * mrtrim: Right trim --comparand matches
  * replace: Replace all matches of a pattern (using --comparand)
      with a string (using --replacement).
  * currencytonum: Gets the numeric value of a currency
  * copy: Mark a column for copying
  * simdl: Damerau-Levenshtein similarity
  * simdln: Normalized Damerau-Levenshtein similarity (between 0.0 & 1.0)
  * simjw: Jaro-Winkler similarity (between 0.0 & 1.0)
  * simsd: Sørensen-Dice similarity (between 0.0 & 1.0)
  * simhm: Hamming distance. Number of positions where characters differ.
  * simod: OSA Distance.
  * soundex: sounds like (boolean)

Examples:
Trim, then transform to uppercase the surname field.

  $ qsv apply operations trim,upper surname file.csv

Trim, then transform to uppercase the surname field and
rename the column uppercase_clean_surname.

  $ qsv apply operations trim,upper surname -r uppercase_clean_surname file.csv

Trim, then transform to uppercase the surname field and 
save it to a new column named uppercase_clean_surname.

  $ qsv apply operations trim,upper surname -c uppercase_clean_surname file.csv

Trim parentheses & brackets from the description field.

  $ qsv apply operations mtrim description --comparand '()<>' file.csv

Replace ' and ' with ' & ' in the description field.u64

  $ qsv apply replace description --comparand ' and ' --replacement ' & ' file.csv

Extract the numeric value of the Salary column and new
column named Salary_num.

  $ qsv apply operations currencytonum Salary file.csv

Compute the Normalized Damerau-Levenshtein similarity of
the neighborhood column to the string 'Roxbury' and save
it to a new column named neighborhood-dl-score.

  $ qsv apply operations lower,simdln neighborhood --comparand roxbury \
    -c dl-score boston311.csv

You can also use this subcommand command to make a copy of a column:

$ qsv apply operations copy col_to_copy -c col_copy file.csv

EMPTYREPLACE
Replace empty cells with <replacement> string.
Non-empty cells are not modified.

Examples:
Replace empty cells in file.csv Measurement column with 'None'.

$ qsv apply emptyreplace Measurement --replacement None file.csv

Replace empty cells in file.csv Measurement column with 'Unknown'.

$ qsv apply emptyreplace --replacement Unknown Measurement file.csv

DATEFMT
Formats a recognized date column to a specified format. See
https://docs.rs/chrono/0.4.19/chrono/format/strftime/ for formats.
Defaults to ISO 8601/RFC 3339 format when --formatstr is not specified.

Examples:
Format dates in Open Date column to ISO 8601/RFC 3339 format:

  $ qsv apply datefmt 'Open Date' file.csv

Format dates in OpenDate column using '%Y-%m-%d' format:

  $ qsv apply datefmt OpenDate --formatstr '%Y-%m-%d' file.csv

Get the week number and store it in the week_number column:

  $ qsv apply dateformat OpenDate --formatstr '%V' --new-column week_number file.csv

GEOCODE
Geocodes to the nearest city center point given a location column
['(lat, long)' or 'lat, long' format] against an embedded copy of
the geonames city database. 

The geocoded information is formatted based on --formatstr, returning
it in 'city-state' format if not specified.

Use the --new-column option if you want to keep the Location column:

Examples:
Geocode file.csv Location column and set the geocoded value to a
new column named City.

$ qsv apply geocode Location --new-column City file.csv

Geocode file.csv Location column with --formatstr=city-state and
set the geocoded value a new column named City.

$ qsv apply geocode Location --formatstr city-state --new-column City file.csv

Usage:
qsv apply operations <operations> [options] <column> [<input>]
qsv apply emptyreplace --replacement=<string> [options] <column> [<input>]
qsv apply datefmt [--formatstr=<string>] [options] <column> [<input>]
qsv apply geocode [--formatstr=<string>] [options] <column> [<input>]
qsv apply --help

apply options:
    -c, --new-column <name>     Put the transformed values in a new column instead.
    -r, --rename <name>         New name for the transformed column.
    -C, --comparand=<string>    The string to compare against for replace & similarity operations.
    -R, --replacement=<string>  The string to use for the replace & emptyreplace operations.
    -f, --formatstr=<string>    The date format to use when formatting dates. For formats, see
                                https://docs.rs/chrono/0.4.19/chrono/format/strftime/index.html
                                [default: %+]

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
    -q, --quiet                 Don't show progress bars.
";

static OPERATIONS: &[&str] = &[
    "len",
    "lower",
    "upper",
    "asciilower",
    "asciiupper",
    "squeeze",
    "trim",
    "rtrim",
    "ltrim",
    "mtrim",
    "mltrim",
    "mrtrim",
    "titlecase",
    "replace",
    "currencytonum",
    "copy",
    "simdl",
    "simdln",
    "simjw",
    "simsd",
    "simhm",
    "simod",
    "soundex",
];

#[derive(Deserialize, Debug)]
struct Args {
    arg_column: SelectColumns,
    cmd_operations: bool,
    arg_operations: String,
    cmd_datefmt: bool,
    cmd_emptyreplace: bool,
    cmd_geocode: bool,
    arg_input: Option<String>,
    flag_rename: Option<String>,
    flag_comparand: String,
    flag_replacement: String,
    flag_formatstr: String,
    flag_new_column: Option<String>,
    flag_output: Option<String>,
    flag_no_headers: bool,
    flag_delimiter: Option<Delimiter>,
    flag_quiet: bool,
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

    if let Some(new_name) = args.flag_rename {
        headers = replace_column_value(&headers, column_index, &new_name);
    }

    if !rconfig.no_headers {
        if let Some(new_column) = &args.flag_new_column {
            headers.push_field(new_column);
        }
        wtr.write_record(&headers)?;
    }

    // validate specified operations
    let operations: Vec<&str> = args.arg_operations.split(',').collect();
    if args.cmd_operations {
        for op in &operations {
            if !OPERATIONS.contains(op) {
                return fail!(format!("Unknown \"{}\" operation", op));
            }
            match op.as_ref() {
                "replace" => {
                    if args.flag_comparand.is_empty() || args.flag_replacement.is_empty() {
                        return fail!(
                            "--comparand (-C) and --replacement (-R) are required for replace operation."
                        );
                    }
                }
                "copy" => {
                    if args.flag_new_column.is_none() {
                        return fail!("--new_column (-c) is required for copy operation.");
                    }
                }
                "mtrim" | "mltrim" | "mrtrim" => {
                    if args.flag_comparand.is_empty() {
                        return fail!("--comparand (-C) is required for match trim operations.");
                    }
                }
                "simdl" | "simdln" | "simjw" | "simsd" | "simhm" | "simod" | "soundex" => {
                    if args.flag_new_column.is_none() {
                        return fail!("--new_column (-c) is required for similarity operations.");
                    }
                }
                _ => {}
            }
        }
    }

    // prep progress bar
    let mut record_count: u64 = 0;
    let progress = ProgressBar::new(record_count);
    if !args.flag_quiet {
        record_count = util::count_rows(&rconfig);
        util::prep_progress(&progress, record_count);
    }

    let mut record = csv::StringRecord::new();
    while rdr.read_record(&mut record)? {
        if !args.flag_quiet {
            progress.inc(1);
        }
        let mut cell = record[column_index].to_owned();

        if args.cmd_operations {
            apply_operations(
                &operations,
                &mut cell,
                &args.flag_comparand,
                &args.flag_replacement,
            );
        } else if args.cmd_emptyreplace {
            if cell.trim().is_empty() {
                cell = args.flag_replacement.to_string();
            }
        } else if args.cmd_datefmt && !cell.is_empty() {
            let parsed_date = parse_with(&cell, &Utc, *MIDNIGHT);
            if let Ok(format_date) = parsed_date {
                let formatted_date = format_date.format(&args.flag_formatstr).to_string();
                if formatted_date.ends_with("T00:00:00+00:00") {
                    cell = formatted_date[..10].to_string();
                } else {
                    cell = formatted_date;
                }
            }
        } else if args.cmd_geocode && !cell.is_empty() {
            geocode(&mut cell, &args.flag_formatstr);
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
    if !args.flag_quiet {
        util::finish_progress(&progress);
    }
    Ok(wtr.flush()?)
}

#[inline]
fn apply_operations(operations: &Vec<&str>, cell: &mut String, comparand: &str, replacement: &str) {
    lazy_static! {
        static ref SQUEEZER: Regex = Regex::new(r"\s+").unwrap();
    }

    for op in operations {
        match op.as_ref() {
            "len" => {
                *cell = cell.len().to_string();
            }
            "lower" => {
                *cell = cell.to_lowercase();
            }
            "asciilower" => {
                cell.make_ascii_lowercase();
            }
            "upper" => {
                *cell = cell.to_uppercase();
            }
            "asciiupper" => {
                cell.make_ascii_uppercase();
            }
            "squeeze" => {
                *cell = SQUEEZER.replace_all(cell, " ").to_string();
            }
            "trim" => {
                *cell = String::from(cell.trim());
            }
            "ltrim" => {
                *cell = String::from(cell.trim_start());
            }
            "rtrim" => {
                *cell = String::from(cell.trim_end());
            }
            "mtrim" => {
                let chars_to_trim: &[char] = &comparand.chars().collect::<Vec<_>>();
                *cell = String::from(cell.trim_matches(chars_to_trim));
            }
            "mltrim" => {
                *cell = String::from(cell.trim_start_matches(comparand));
            }
            "mrtrim" => {
                *cell = String::from(cell.trim_end_matches(comparand));
            }
            "titlecase" => {
                *cell = titlecase(cell);
            }
            "replace" => {
                *cell = cell.replace(comparand, replacement);
            }
            "currencytonum" => {
                let currency_value = Currency::from_str(cell);
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
                        *cell = format!("{}.{}", coin_num, coin_frac);
                    }
                }
            }
            "simdl" => {
                *cell = damerau_levenshtein(cell, comparand).to_string();
            }
            "simdln" => {
                *cell = normalized_damerau_levenshtein(cell, comparand).to_string();
            }
            "simjw" => {
                *cell = jaro_winkler(cell, comparand).to_string();
            }
            "simsd" => {
                *cell = sorensen_dice(cell, comparand).to_string();
            }
            "simhm" => {
                let ham_val = hamming(cell, comparand);
                match ham_val {
                    Ok(val) => *cell = val.to_string(),
                    Err(_) => *cell = String::from("ERROR: Different lengths"),
                }
            }
            "simod" => *cell = osa_distance(cell, comparand).to_string(),
            "soundex" => {
                *cell = format!("{}", (soundex(&*cell, comparand)));
            }
            _ => {} // this also handles copy, which is a noop
        }
    }
}

#[inline(always)]
fn geocode(cell: &mut String, formatstr: &str) {
    // simple regex for "lat, long" or "(lat, long)", does not validate ranges
    lazy_static! {
        static ref LOCREGEX: Regex =
            Regex::new(r"(?-u)^\(*(-?\d+(?:\.\d+)?),\s*(-?\d+(?:\.\d+)?)\)?$").unwrap();
    }
    lazy_static! {
        static ref LOCS: Locations = Locations::from_memory();
        static ref GEOCODER: ReverseGeocoder<'static> = ReverseGeocoder::new(&LOCS);
    }

    let loccaps = LOCREGEX.captures(&*cell);
    if let Some(loccaps) = loccaps {
        let lats = loccaps.get(1).map_or("", |m| m.as_str());
        let longs = loccaps.get(2).map_or("", |m| m.as_str());
        let coords = (lats.parse::<f64>().unwrap(), longs.parse::<f64>().unwrap());
        let search_result = GEOCODER.search(coords);
        if let Some(locdetails) = search_result {
            *cell = match formatstr {
                "%+" | "city-state" => format!(
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
