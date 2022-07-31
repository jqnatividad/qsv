#![allow(dead_code)]
use crate::config::{Config, Delimiter};
use crate::select::SelectColumns;
use crate::CliError;
use crate::CliResult;
use crate::{regex_once_cell, util};
use cached::proc_macro::cached;
use censor::{Censor, Sex, Zealous};
use dynfmt::Format;
use eudex::Hash;
use indicatif::{ProgressBar, ProgressDrawTarget};
use log::debug;
use once_cell::sync::OnceCell;
use qsv_currency::Currency;
use qsv_dateparser::parse_with_preference;
use rayon::prelude::*;
use regex::Regex;
use reverse_geocoder::{Locations, ReverseGeocoder};
use serde::Deserialize;
use strsim::{
    damerau_levenshtein, hamming, jaro_winkler, normalized_damerau_levenshtein, osa_distance,
    sorensen_dice,
};
use titlecase::titlecase;
use vader_sentiment::SentimentIntensityAnalyzer;
use whatlang::detect;

static USAGE: &str = "
Apply a series of transformation functions to a given CSV column. This can be used to
perform typical cleaning tasks and/or harmonize some values, etc.

It has several subcommands:

OPERATIONS
The series of operations must be given separated by commas as such:

  trim => Trim the cell
  trim,upper => Trim the cell, then transform to uppercase
  lower,simdln => Lowercase the cell, then compute the normalized 
      Damerau-Levenshtein similarity to --comparand

Currently supported operations:

  * len: Return string length
  * lower: Transform to lowercase
  * upper: Transform to uppercase
  * squeeze: Compress consecutive whitespaces
  * trim: Trim (drop whitespace left & right of the string)
  * ltrim: Left trim whitespace
  * rtrim: Right trim whitespace
  * mtrim: Trims --comparand matches left & right of the string (Rust trim_matches)
  * mltrim: Left trim --comparand matches (Rust trim_start_matches)
  * mrtrim: Right trim --comparand matches (Rust trim_end_matches)
  * replace: Replace all matches of a pattern (using --comparand)
      with a string (using --replacement) (Rust replace)
  * regex_replace: Replace all regex matches in --comparand w/ --replacement.
  * titlecase - capitalizes English text using Daring Fireball titlecase style
      https://daringfireball.net/2008/05/title_case 
  * censor_check: check if profanity is detected (boolean).
      Add additional comma-delimited profanities with -comparand. 
  * censor: profanity filter. Add additional comma-delimited profanities 
      with --comparand.
  * currencytonum: Gets the numeric value of a currency. Supports currency symbols
      (e.g. $,¥,£,€,֏,₱,₽,₪,₩,ƒ,฿,₫) and strings (e.g. USD, EUR, RMB, JPY, etc.). 
      Recognizes point, comma and space separators.
  * copy: Mark a column for copying
  * simdl: Damerau-Levenshtein similarity to --comparand
  * simdln: Normalized Damerau-Levenshtein similarity to --comparand 
     (between 0.0 & 1.0)
  * simjw: Jaro-Winkler similarity to --comparand (between 0.0 & 1.0)
  * simsd: Sørensen-Dice similarity to --comparand (between 0.0 & 1.0)
  * simhm: Hamming distance to --comparand. Num of positions characters differ.
  * simod: OSA Distance to --comparand.
  * eudex: Multi-lingual sounds like --comparand (boolean)
  * sentiment: Normalized VADER sentiment score (English only - between -1.0 to 1.0).
  * whatlang: Language Detection. For 87 supported languages, see
       https://github.com/greyblake/whatlang-rs/blob/master/SUPPORTED_LANGUAGES.md

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

Extract the numeric value of the Salary column in a new
column named Salary_num.

  $ qsv apply operations currencytonum Salary -c Salary_num file.csv

Compute the Normalized Damerau-Levenshtein similarity of
the neighborhood column to the string 'Roxbury' and save
it to a new column named dln_roxbury_score.

  $ qsv apply operations lower,simdln neighborhood --comparand roxbury \
    -c dln_roxbury_score boston311.csv

You can also use this subcommand command to make a copy of a column:

$ qsv apply operations copy col_to_copy -c col_copy file.csv

EMPTYREPLACE
Replace empty cells with <--replacement> string.
Non-empty cells are not modified. See the `fill` command for more
complex empty field operations.

Examples:
Replace empty cells in file.csv Measurement column with 'None'.

$ qsv apply emptyreplace Measurement --replacement None file.csv

Replace empty cells in file.csv Measurement column with 'Unknown'.

$ qsv apply emptyreplace --replacement Unknown Measurement file.csv

DATEFMT
Formats a recognized date column to a specified format using --formatstr. 
See https://github.com/jqnatividad/belt/tree/main/dateparser#accepted-date-formats for
recognized date formats.
See https://docs.rs/chrono/0.4.19/chrono/format/strftime/ for 
accepted date formats for --formatstr.
Defaults to ISO 8601/RFC 3339 format when --formatstr is not specified.

Examples:
Format dates in Open Date column to ISO 8601/RFC 3339 format:

  $ qsv apply datefmt 'Open Date' file.csv

Format dates in OpenDate column using '%Y-%m-%d' format:

  $ qsv apply datefmt OpenDate --formatstr '%Y-%m-%d' file.csv

Get the week number and store it in the week_number column:

  $ qsv apply dateformat OpenDate --formatstr '%V' --new-column week_number file.csv

DYNFMT
Dynamically constructs a new column from other columns using the --formatstr template.
The template can contain arbitrary characters. To insert a column value, enclose the
column name in curly braces, replacing all non-alphanumeric characters with underscores.

Examples:
Create a new column 'mailing address' from 'house number', 'street', 'city' and 'zip-code' columns:

  $ qsv apply dynfmt --formatstr '{house_number} {street}, {city} {zip_code} USA' -c 'mailing address' file.csv

Create a new column 'FullName' from 'FirstName', 'MI', and 'LastName' columns:

  $ qsv apply dynfmt --formatstr 'Sir/Madam {FirstName} {MI}. {LastName}' -c 'FullName' file.csv

GEOCODE
Geocodes to the nearest city center point given a location column
[i.e. a column which contains a latitude, longitude WGS84 coordinate] against
an embedded copy of the geonames city database. 

The geocoded information is formatted based on --formatstr, returning
it in 'city-state' format if not specified.

Use the --new-column option if you want to keep the location column:

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
qsv apply dynfmt --formatstr=<string> [options] --new-column=<name> [<input>]
qsv apply geocode [--formatstr=<string>] [options] <column> [<input>]
qsv apply --help

apply options:
    -c, --new-column <name>     Put the transformed values in a new column instead.
    -r, --rename <name>         New name for the transformed column.
    -C, --comparand=<string>    The string to compare against for replace & similarity operations.
    -R, --replacement=<string>  The string to use for the replace & emptyreplace operations.
    --prefer-dmy                Prefer to parse dates in dmy format. Otherwise, use mdy format.
                                Only used with the DATEFMT subcommand.
    -f, --formatstr=<string>    This option is used by several subcommands:

                                DATEFMT: The date format to use. For formats, see
                                  https://docs.rs/chrono/0.4.19/chrono/format/strftime/index.html
                                  [default: %+]

                                DYNFMT: the template to use to construct a new column.

                                GEOCODE: the place format to use with the geocode subcommand.
                                  The available formats are:
                                  - 'city-state' (default) - e.g. Brooklyn, New York
                                  - 'city-country' - Brooklyn, US 
                                  - 'city-state-country' | 'city-admin1-country' - Brooklyn, New York US
                                  - 'city' - Brooklyn
                                  - 'county' | 'admin2' - Kings County
                                  - 'state' | 'admin1' - New York
                                  - 'county-country' | 'admin2-country' - Kings County, US
                                  - 'county-state-country' | 'admin2-admin1-country' - Kings County, New York US
                                  - 'country' - US
    -j, --jobs <arg>            The number of jobs to run in parallel.
                                When not set, the number of jobs is set to the number of CPUs detected.

Common options:
    -h, --help                  Display this message
    -o, --output <file>         Write output to <file> instead of stdout.
    -n, --no-headers            When set, the first row will not be interpreted
                                as headers.
    -d, --delimiter <arg>       The field delimiter for reading CSV data.
                                Must be a single character. (default: ,)
    -q, --quiet                 Don't show progress bars.
";

// number of CSV rows to process in a batch
const BATCH_SIZE: usize = 24_000;

static OPERATIONS: &[&str] = &[
    "len",
    "lower",
    "upper",
    "squeeze",
    "trim",
    "rtrim",
    "ltrim",
    "mtrim",
    "mltrim",
    "mrtrim",
    "titlecase",
    "replace",
    "regex_replace",
    "censor_check",
    "censor",
    "currencytonum",
    "copy",
    "simdl",
    "simdln",
    "simjw",
    "simsd",
    "simhm",
    "simod",
    "eudex",
    "sentiment",
    "whatlang",
];

#[derive(Deserialize, Debug)]
struct Args {
    arg_column: SelectColumns,
    cmd_operations: bool,
    arg_operations: String,
    cmd_datefmt: bool,
    cmd_dynfmt: bool,
    cmd_emptyreplace: bool,
    cmd_geocode: bool,
    arg_input: Option<String>,
    flag_rename: Option<String>,
    flag_comparand: String,
    flag_replacement: String,
    flag_prefer_dmy: bool,
    flag_formatstr: String,
    flag_jobs: Option<usize>,
    flag_new_column: Option<String>,
    flag_output: Option<String>,
    flag_no_headers: bool,
    flag_delimiter: Option<Delimiter>,
    flag_quiet: bool,
}

static CENSOR: OnceCell<Censor> = OnceCell::new();
static LOCS: OnceCell<Locations> = OnceCell::new();
static GEOCODER: OnceCell<ReverseGeocoder> = OnceCell::new();
static EUDEX_COMPARAND_HASH: OnceCell<eudex::Hash> = OnceCell::new();
static REGEX_REPLACE: OnceCell<Regex> = OnceCell::new();
static SENTIMENT_ANALYZER: OnceCell<SentimentIntensityAnalyzer> = OnceCell::new();

#[inline]
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
    let mut censor_invokes = 0_usize;
    let mut replace_invokes = 0_usize;
    let mut regex_replace_invokes = 0_usize;
    let mut sim_invokes = 0_usize;
    let mut eudex_invokes = 0_usize;
    let mut sentiment_invokes = 0_usize;
    let operations_lowercase = args.arg_operations.to_lowercase();
    let operations: Vec<&str> = operations_lowercase.split(',').collect();
    if args.cmd_operations {
        for op in &operations {
            if !OPERATIONS.contains(op) {
                return fail!(format!("Unknown '{op}' operation"));
            }
            #[allow(clippy::useless_asref)]
            match op.as_ref() {
                "replace" => {
                    if args.flag_comparand.is_empty() || args.flag_replacement.is_empty() {
                        return fail!(
                            "--comparand (-C) and --replacement (-R) are required for replace operation."
                        );
                    }
                    replace_invokes += 1;
                }
                "regex_replace" => {
                    if args.flag_comparand.is_empty() || args.flag_replacement.is_empty() {
                        return fail!(
                            "--comparand (-C) and --replacement (-R) are required for regex_replace operation."
                        );
                    }
                    regex_replace_invokes += 1;
                }
                "copy" => {
                    if args.flag_new_column.is_none() {
                        return fail!("--new_column (-c) is required for copy operation.");
                    }
                }
                "censor" | "censor_check" => {
                    if args.flag_new_column.is_none() {
                        return fail!("--new_column (-c) is required for censor operations.");
                    }
                    censor_invokes += 1;
                }
                "mtrim" | "mltrim" | "mrtrim" => {
                    if args.flag_comparand.is_empty() {
                        return fail!("--comparand (-C) is required for match trim operations.");
                    }
                }
                "simdl" | "simdln" | "simjw" | "simsd" | "simhm" | "simod" => {
                    if args.flag_new_column.is_none() {
                        return fail!("--new_column (-c) is required for similarity operations.");
                    }
                    sim_invokes += 1;
                }
                "eudex" => {
                    if args.flag_new_column.is_none() {
                        return fail!("--new_column (-c) is required for eudex.");
                    }
                    eudex_invokes += 1;
                }
                "sentiment" => {
                    if args.flag_new_column.is_none() {
                        return fail!("--new_column (-c) is required for sentiment operation.");
                    }
                    sentiment_invokes += 1;
                }
                "whatlang" => {
                    if args.flag_new_column.is_none() {
                        return fail!(
                            "--new_column (-c) is required for whatlang language detection."
                        );
                    }
                }
                _ => {}
            }
        }
    }
    if censor_invokes > 1
        || replace_invokes > 1
        || regex_replace_invokes > 1
        || sim_invokes > 1
        || eudex_invokes > 1
        || sentiment_invokes > 1
    {
        return fail!("you can only use censor, replace, regex_replace, similarity, eudex or sentiment ONCE per operation series.");
    };

    // for dynfmt, safe_headers are the "safe" version of colnames - alphanumeric only,
    // all other chars replaced with underscore
    // dynfmt_fields are the columns used in the dynfmt --formatstr option
    // we prep it so we only populate the lookup vec with the index of these columns
    // so SimpleCurlyFormat is performant
    let mut dynfmt_fields = Vec::with_capacity(10); // 10 is a reasonable default to save allocs
    let mut dynfmt_template = args.flag_formatstr.clone();
    if args.cmd_dynfmt {
        if args.flag_no_headers {
            return fail!("dynfmt operation requires headers.");
        }
        // first, get the fields used in the dynfmt template
        let safe_headers = util::safe_header_names(&headers, false);
        let formatstr_re: &'static Regex = crate::regex_once_cell!(r"\{(?P<key>\w+)?\}");
        for format_fields in formatstr_re.captures_iter(&args.flag_formatstr) {
            dynfmt_fields.push(format_fields.name("key").unwrap().as_str());
        }
        // we sort the fields so we can do binary_search
        dynfmt_fields.sort_unstable();
        // now, get the indices of the columns for the lookup vec
        for (i, field) in safe_headers.into_iter().enumerate() {
            if dynfmt_fields.binary_search(&field.as_str()).is_ok() {
                let field_with_curly = format!("{{{field}}}");
                let field_index = format!("{{{i}}}");
                dynfmt_template = dynfmt_template.replace(&field_with_curly, &field_index);
            }
        }
        debug!("dynfmt_fields: {dynfmt_fields:?}  dynfmt_template: {dynfmt_template}");
    }

    pub enum ApplySubCmd {
        Operations,
        DateFmt,
        DynFmt,
        Geocode,
        EmptyReplace,
        Unknown,
    }

    let apply_cmd = if args.cmd_operations {
        ApplySubCmd::Operations
    } else if args.cmd_geocode {
        ApplySubCmd::Geocode
    } else if args.cmd_datefmt {
        ApplySubCmd::DateFmt
    } else if args.cmd_dynfmt {
        ApplySubCmd::DynFmt
    } else if args.cmd_emptyreplace {
        ApplySubCmd::EmptyReplace
    } else {
        ApplySubCmd::Unknown
    };

    // prep progress bar
    let progress = ProgressBar::new(0);
    if args.flag_quiet {
        progress.set_draw_target(ProgressDrawTarget::hidden());
    } else {
        util::prep_progress(&progress, util::count_rows(&rconfig)?);
        progress.set_draw_target(ProgressDrawTarget::stderr_with_hz(5));
    }
    let not_quiet = !args.flag_quiet;

    let prefer_dmy = args.flag_prefer_dmy || rconfig.get_dmy_preference();

    // amortize memory allocation by reusing record
    #[allow(unused_assignments)]
    let mut batch_record = csv::StringRecord::new();

    // reuse batch buffers
    let mut batch = Vec::with_capacity(BATCH_SIZE);
    let mut batch_results = Vec::with_capacity(BATCH_SIZE);

    // set RAYON_NUM_THREADS
    util::njobs(args.flag_jobs);

    // main loop to read CSV and construct batches for parallel processing.
    // each batch is processed via Rayon parallel iterator.
    // loop exits when batch is empty.
    loop {
        for _ in 0..BATCH_SIZE {
            match rdr.read_record(&mut batch_record) {
                Ok(has_data) => {
                    if has_data {
                        batch.push(batch_record.clone());
                    } else {
                        // nothing else to add to batch
                        break;
                    }
                }
                Err(e) => {
                    return Err(CliError::Other(format!("Error reading file: {e}")));
                }
            }
        }

        if batch.is_empty() {
            // break out of infinite loop when at EOF
            break;
        }

        // do actual apply command via Rayon parallel iterator
        batch
            .par_iter()
            .map(|record_item| {
                let mut record = record_item.clone();
                let mut cell = record[column_index].to_owned();

                match apply_cmd {
                    ApplySubCmd::Geocode => {
                        if !cell.is_empty() {
                            let search_result = search_cached(&cell, &args.flag_formatstr);
                            if let Some(geocoded_result) = search_result {
                                cell = geocoded_result;
                            }
                        }
                    }
                    ApplySubCmd::Operations => {
                        apply_operations(
                            &operations,
                            &mut cell,
                            &args.flag_comparand,
                            &args.flag_replacement,
                        );
                    }
                    ApplySubCmd::EmptyReplace => {
                        if cell.trim().is_empty() {
                            cell = args.flag_replacement.to_string();
                        }
                    }
                    ApplySubCmd::DateFmt => {
                        if !cell.is_empty() {
                            let parsed_date = parse_with_preference(&cell, prefer_dmy);
                            if let Ok(format_date) = parsed_date {
                                let formatted_date =
                                    format_date.format(&args.flag_formatstr).to_string();
                                if formatted_date.ends_with("T00:00:00+00:00") {
                                    cell = formatted_date[..10].to_string();
                                } else {
                                    cell = formatted_date;
                                }
                            }
                        }
                    }
                    ApplySubCmd::DynFmt => {
                        if !cell.is_empty() {
                            let mut record_vec: Vec<String> = Vec::with_capacity(record.len());
                            for field in &record {
                                record_vec.push(field.to_string());
                            }
                            if let Ok(formatted) =
                                dynfmt::SimpleCurlyFormat.format(&dynfmt_template, record_vec)
                            {
                                cell = formatted.to_string();
                            }
                        }
                    }
                    ApplySubCmd::Unknown => {
                        unreachable!("apply subcommands are always known");
                    }
                }

                if args.flag_new_column.is_some() {
                    record.push_field(&cell);
                } else {
                    record = replace_column_value(&record, column_index, &cell);
                }
                record
            })
            .collect_into_vec(&mut batch_results);

        // rayon collect() guarantees original order, so we can just append results each batch
        for result_record in &batch_results {
            wtr.write_record(result_record)?;
        }

        if not_quiet {
            progress.inc(batch.len() as u64);
        }

        batch.clear();
    } // end infinite loop

    if not_quiet {
        if args.cmd_geocode {
            util::update_cache_info!(progress, SEARCH_CACHED);
        }
        util::finish_progress(&progress);
    }
    Ok(wtr.flush()?)
}

#[inline]
fn apply_operations(operations: &[&str], cell: &mut String, comparand: &str, replacement: &str) {
    for op in operations {
        #[allow(clippy::useless_asref)]
        match op.as_ref() {
            "len" => {
                *cell = cell.len().to_string();
            }
            "lower" => {
                *cell = cell.to_lowercase();
            }
            "upper" => {
                *cell = cell.to_uppercase();
            }
            "squeeze" => {
                let squeezer: &'static Regex = regex_once_cell!(r"\s+");
                *cell = squeezer.replace_all(cell, " ").to_string();
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
            "regex_replace" => {
                let regexreplace =
                    REGEX_REPLACE.get_or_init(|| regex::Regex::new(comparand).unwrap());
                *cell = regexreplace.replace_all(cell, replacement).to_string();
            }
            "censor_check" | "censor" => {
                let censor = CENSOR.get_or_init(|| {
                    let mut censored_words = Censor::Standard + Zealous + Sex;
                    for word in comparand.split(',') {
                        censored_words += word.trim();
                    }
                    censored_words
                });
                if *op == "censor_check" {
                    *cell = censor.check(cell).to_string();
                } else {
                    *cell = censor.censor(cell);
                }
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
                        *cell = format!("{coin_num}.{coin_frac}");
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
            "eudex" => {
                let eudex_comparand_hash =
                    EUDEX_COMPARAND_HASH.get_or_init(|| eudex::Hash::new(comparand));
                let cell_hash = Hash::new(cell);
                *cell = format!("{}", (cell_hash - *eudex_comparand_hash).similar());
            }
            "sentiment" => {
                let sentiment_analyzer = SENTIMENT_ANALYZER
                    .get_or_init(vader_sentiment::SentimentIntensityAnalyzer::new);
                let sentiment_scores = sentiment_analyzer.polarity_scores(cell);
                *cell = sentiment_scores.get("compound").unwrap_or(&0.0).to_string();
            }
            "whatlang" => {
                let lang_info = detect(cell);
                if let Some(lang_info) = lang_info {
                    if lang_info.is_reliable() && lang_info.confidence() >= 0.5 {
                        *cell = format!("{:?}", lang_info.lang());
                    } else {
                        // if confidence < 0.5 and !is_reliable(),
                        // do best-guessed language and add a question mark
                        *cell = format!("{:?}?", lang_info.lang());
                    }
                }
            }
            _ => {} // this also handles copy, which is a noop
        }
    }
}

#[cached(
    key = "String",
    convert = r#"{ format!("{}", cell) }"#,
    option = true,
    sync_writes = false
)]
fn search_cached(cell: &str, formatstr: &str) -> Option<String> {
    let geocoder =
        GEOCODER.get_or_init(|| ReverseGeocoder::new(LOCS.get_or_init(Locations::from_memory)));

    let locregex: &'static Regex = regex_once_cell!(
        r"(?-u)([+-]?[0-9]+\.?[0-9]*|\.[0-9]+),\s*([+-]?[0-9]+\.?[0-9]*|\.[0-9]+)"
    );

    let loccaps = locregex.captures(cell);
    loccaps.and_then(|loccaps| {
        let lat = loccaps[1].to_string().parse::<f64>().unwrap_or_default();
        let long = loccaps[2].to_string().parse::<f64>().unwrap_or_default();
        if (-90.0..=90.00).contains(&lat) && (-180.0..=180.0).contains(&long) {
            let search_result = geocoder.search((lat, long));
            search_result.map(|locdetails| {
                #[allow(clippy::match_same_arms)]
                match formatstr {
                    "%+" | "city-state" => format!(
                        "{name}, {admin1}",
                        name = locdetails.record.name,
                        admin1 = locdetails.record.admin1,
                    ),
                    "city-country" => format!(
                        "{name}, {cc}",
                        name = locdetails.record.name,
                        cc = locdetails.record.cc
                    ),
                    "city-state-country" | "city-admin1-country" => format!(
                        "{name}, {admin1} {cc}",
                        name = locdetails.record.name,
                        admin1 = locdetails.record.admin1,
                        cc = locdetails.record.cc
                    ),
                    "city" => locdetails.record.name.to_string(),
                    "county" | "admin2" => locdetails.record.admin2.to_string(),
                    "state" | "admin1" => locdetails.record.admin1.to_string(),
                    "county-country" | "admin2-country" => format!(
                        "{admin2}, {cc}",
                        admin2 = locdetails.record.admin2,
                        cc = locdetails.record.cc
                    ),
                    "county-state-country" | "admin2-admin1-country" => format!(
                        "{admin2}, {admin1} {cc}",
                        admin2 = locdetails.record.admin2,
                        admin1 = locdetails.record.admin1,
                        cc = locdetails.record.cc
                    ),
                    "country" => locdetails.record.cc.to_string(),
                    #[allow(clippy::match_same_arms)]
                    _ => locdetails.record.name.to_string(),
                }
            })
        } else {
            None
        }
    })
}
