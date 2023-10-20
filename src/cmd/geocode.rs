static USAGE: &str = r#"
Geocodes a location in CSV data against an updatable local copy of the Geonames cities index.

When you run the command for the first time, it will download a prebuilt Geonames cities
index from the qsv GitHub repo and use it going forward. You can operate on the local
index using the index-* subcommands.

By default, the prebuilt index uses the Geonames Gazeteer cities15000.zip file using
English names. It contains cities with populations > 15,000 (about ~26k cities). 
See https://download.geonames.org/export/dump/ for more information.

It has seven major subcommands:
 * suggest        - given a partial City name, return the closest City's location metadata
                    per the local Geonames cities index (Jaro-Winkler distance)
 * suggestnow     - same as suggest, but using a partial City name from the command line,
                    instead of CSV data.
 * reverse        - given a WGS-84 location coordinate, return the closest City's location
                    metadata per the local Geonames cities index.
                    (Euclidean distance - shortest distance "as the crow flies")
 * reversenow     - sames as reverse, but using a coordinate from the command line,
                    instead of CSV data.
 * countryinfo    - returns the country information for the ISO-3166 2-letter country code
                    (e.g. US, CA, MX, etc.)
 * countryinfonow - same as countryinfo, but using a country code from the command line,
                    instead of CSV data.
 * index-*        - operations to update the local Geonames cities index.
                    (index-check, index-update, index-load & index-reset)
 
SUGGEST
Suggest a Geonames city based on a partial city name. It returns the closest Geonames
city record based on the Jaro-Winkler distance between the partial city name and the
Geonames city name.

The geocoded information is formatted based on --formatstr, returning it in 
'%location' format (i.e. "(lat, long)") if not specified.

Use the --new-column option if you want to keep the location column:

Examples:
Geocode file.csv city column and set the geocoded value to a new column named lat_long.

  $ qsv geocode suggest city --new-column lat_long file.csv

Limit suggestions to the US, Canada and Mexico.

  $ qsv geocode suggest city --country us,ca,mx file.csv

Limit suggestions to New York State and California, with matches in New York state
having higher priority as its listed first.

  $ qsv geocode suggest city --country us --admin1 "New York,US.CA" file.csv

If we use admin1 codes, we can omit --country as it will be inferred from the admin1 code prefix.

  $ qsv geocode suggest city --admin1 "US.NY,US.CA" file.csv

Geocode file.csv city column with --formatstr=%state and set the 
geocoded value a new column named state.

  $ qsv geocode suggest city --formatstr %state --new-column state file.csv

Use dynamic formatting to create a custom format.

  $ qsv geocode suggest city -f "{name}, {admin1}, {country} in {timezone}" file.csv
  # using French place names. You'll need to rebuild the index with the --languages option first
  $ qsv geocode suggest city -f "{name}, {admin1}, {country} in {timezone}" -l fr file.csv

SUGGESTNOW
Accepts the same options as suggest, but does not require an input file.
Its default format is more verbose - "{name}, {admin1} {country}: {latitude}, {longitude}"

  $ qsv geocode suggestnow "New York"
  $ qsv geocode suggestnow --country US -f %cityrecord "Paris"
  $ qsv geocode suggestnow --admin1 "US:OH" "Athens"

REVERSE
Reverse geocode a WGS 84 coordinate to the nearest City. It returns the closest Geonames
city record based on the Euclidean distance between the coordinate and the nearest city.
It accepts "lat, long" or "(lat, long)" format.

The geocoded information is formatted based on --formatstr, returning it in
'%city-admin1' format if not specified.

Examples:
Reverse geocode file.csv LatLong column. Set the geocoded value to a new column named City.

  $ qsv geocode reverse LatLong -c City file.csv

Reverse geocode file.csv LatLong column and set the geocoded value to a new column
named CityState, output to a file named file_with_citystate.csv.

  $ qsv geocode reverse LatLong -c CityState file.csv -o file_with_citystate.csv

The same as above, but get the timezone instead of the city and state.

  $ qsv geocode reverse LatLong -f %timezone -c tz file.csv -o file_with_tz.csv

REVERSENOW
Accepts the same options as reverse, but does not require an input file.

  $ qsv geocode reversenow "40.71427, -74.00597"
  $ qsv geocode reversenow --country US -f %cityrecord "40.71427, -74.00597"
  $ qsv geocode reversenow --admin1 "US:OH" "(39.32924, -82.10126)"

COUNTRYINFO
Returns the country information for the specified ISO-3166 2-letter country code.

  $ qsv geocode countryinfo country_col data.csv
  $ qsv geocode countryinfo --formatstr "%json" country_col data.csv
  $ qsv geocode countryinfo -f "%continent" country_col data.csv
  $ qsv geocode countryinfo -f "{country_name} ({fips}) in {continent}" country_col data.csv

COUNTRYINFONOW
Accepts the same options as countryinfo, but does not require an input file.

  $ qsv geocode countryinfonow US
  $ qsv geocode countryinfonow --formatstr "%pretty-json" US
  $ qsv geocode countryinfonow -f "%continent" US
  $ qsv geocode countryinfonow -f "{country_name} ({fips}) in {continent}" US

INDEX-<operation>
Manage the local Geonames cities index used by the geocode command.

It has four operations:
 * check  - checks if the local Geonames index is up-to-date compared to the Geonames website.
            returns the index file's metadata JSON to stdout.
 * update - updates the local Geonames index with the latest changes from the Geonames website.
            use this command judiciously as it downloads about ~200mb of data from Geonames
            and rebuilds the index from scratch using the --languages option.
 * reset  - resets the local Geonames index to the default prebuilt, English language Geonames
            cities index - downloading it from the qsv GitHub repo for that release.
 * load   - load a Geonames cities index from a file, making it the default index going forward.

Examples:
Update the Geonames cities index with the latest changes.

  $ qsv geocode index-update
  # or rebuild the index using the latest Geonames data
  # with English, French, German & Spanish place names
  $ qsv geocode index-update --languages en,fr,de,es

Load an alternative Geonames cities index from a file, making it the default index going forward.

  $ qsv geocode index-load my_geonames_index.bincode

For more extensive examples, see https://github.com/jqnatividad/qsv/blob/master/tests/test_geocode.rs.

Usage:
qsv geocode suggest [--formatstr=<string>] [options] <column> [<input>]
qsv geocode suggestnow [options] <location>
qsv geocode reverse [--formatstr=<string>] [options] <column> [<input>]
qsv geocode reversenow [options] <location>
qsv geocode countryinfo [options] <column> [<input>]
qsv geocode countryinfonow [options] <location>
qsv geocode index-load <index-file>
qsv geocode index-check
qsv geocode index-update [--languages=<lang>] [--cities-url=<url>] [--force] [--timeout=<seconds>]
qsv geocode index-reset
qsv geocode --help

geocode arguments:
        
    <input>                     The input file to read from. If not specified, reads from stdin.

    <column>                    The column to geocode. Used by suggest, reverse & countryinfo subcommands.
                                For suggest, it must be a column with a City string pattern.
                                For reverse, it must be a column using WGS 84 coordinates in
                                "lat, long" or "(lat, long)" format.
                                For countryinfo, it must be a column with a ISO 3166-1 alpha-2 country code.

    <location>                  The location to geocode for suggestnow, reversenow & countryinfonow subcommands.
                                For suggestnow, its a City string pattern.
                                For reversenow, it must be a WGS 84 coordinate.
                                For countryinfonow, it must be a ISO 3166-1 alpha-2 code.
                                
    <index-file>                The alternate geonames index file to use. It must be a .bincode file.
                                Only used by the index-load subcommand.

geocode options:
    -c, --new-column <name>     Put the transformed values in a new column instead. Not valid when
                                using the '%dyncols:' --formatstr option.
    -r, --rename <name>         New name for the transformed column.
    --country <country_list>    The comma-delimited, case-insensitive list of countries to filter for.
                                Country is specified as a ISO 3166-1 alpha-2 (two-letter) country code.
                                https://en.wikipedia.org/wiki/ISO_3166-2

                                It is the topmost priority filter, and will be applied first. If multiple
                                countries are specified, they are matched in priority order.
                                
                                For suggest, this will limit the search to the specified countries.

                                For reverse, this ensures that the returned city is in the specified
                                countries (especially when geocoding coordinates near country borders).
                                If the coordinate is outside the specified countries, the returned city
                                will be the closest city as the crow flies in the specified countries.

                                SUGGEST only options:
    --min-score <score>         The minimum Jaro-Winkler distance score.
                                [default: 0.8]
    --admin1 <admin1_list>      The comma-delimited, case-insensitive list of admin1s to filter for.
    
                                If all uppercase, it will be treated as an admin1 code (e.g. US.NY, JP.40, CN.23).
                                Otherwise, it will be treated as an admin1 name (e.g New York, Tokyo, Shanghai).

                                Requires the --country option. However, if all admin1 codes have the same
                                prefix (e.g. US.TX, US.NJ, US.CA), the country can be inferred from the
                                admin1 code (in this example - US), and the --country option is not required.

                                If specifying multiple admin1 filters, you can mix admin1 codes and names,
                                and they are matched in priority order.

                                Matches are made using a starts_with() comparison (i.e. "US" will match "US.NY",
                                "US.NJ", etc. for admin1 code. "New" will match "New York", "New Jersey",
                                "Newfoundland", etc. for admin1 name.)

                                admin1 is the second priority filter, and will be applied after country filters.
                                See https://download.geonames.org/export/dump/admin1CodesASCII.txt for
                                recognized admin1 codes/names.

                                REVERSE only option:
    -k, --k_weight <weight>     Use population-weighted distance for reverse subcommand.
                                (i.e. nearest.distance - k * city.population)
                                Larger values will favor more populated cities.
                                If not set (default), the population is not used and the
                                nearest city is returned.

    -f, --formatstr=<string>    The place format to use. It has three options:
                                1. Use one of the predefined formats.
                                2. Use dynamic formatting to create a custom format.
                                3. Use the special format "%dyncols:" to dynamically add multiple
                                   columns to the output CSV using fields from a geocode result.
    
                                PREDEFINED FORMATS:
                                  - '%city-state' - e.g. Brooklyn, New York
                                  - '%city-country' - Brooklyn, US
                                  - '%city-state-country' | '%city-admin1-country' - Brooklyn, New York US
                                  - '%city-county-state' | '%city-admin2-admin1' - Brooklyn, Kings County, New York
                                  - '%city' - Brooklyn
                                  - '%state' | '%admin1' - New York
                                  - "%county' | '%admin2' - Kings County
                                  - '%country' - US
                                  - '%country_name' - United States
                                  - '%cityrecord' - returns the full city record as a string
                                  - '%admin1record' - returns the full admin1 record as a string
                                  - '%admin2record' - returns the full admin2 record as a string
                                  - '%lat-long' - <latitude>, <longitude>
                                  - '%location' - (<latitude>, <longitude>)
                                  - '%id' - the Geonames ID
                                  - '%capital' - the capital
                                  - '%continent' - the continent (only valid for countryinfo subcommand)
                                  - '%population' - the population
                                  - '%timezone' - the timezone
                                  - '%json' - the full city record as JSON
                                  - '%pretty-json' - the full city record as pretty JSON
                                  - '%+' - use the subcommand's default format. 
                                           suggest - '%location'
                                           suggestnow - '{name}, {admin1} {country}: {latitude}, {longitude}'
                                           reverse & reversenow - '%city-admin1-country'
                                           countryinfo - '%country_name'
                                
                                If an invalid format is specified, it will be treated as '%+'.

                                Note that when using the JSON predefined formats with the now subcommands,
                                the output will be valid JSON, as the "Location" header will be omitted.

                                DYNAMIC FORMATTING:
                                Alternatively, you can use dynamic formatting to create a custom format.
                                To do so, set the --formatstr to a dynfmt template, enclosing field names
                                in curly braces.
                                The following ten cityrecord fields are available:
                                  id, name, latitude, longitude, country, admin1, admin2, capital,
                                  timezone, population

                                Fifteen additional countryinfo field are also available:
                                  iso3, fips, area, country_population, continent, tld, currency_code,
                                  currency_name, phone, postal_code_format, postal_code_regex, languages,
                                  country_geonameid, neighbours, equivalent_fips_code

                                For US places, two additional fields are available:
                                  us_county_fips_code and us_state_fips_code
                                    
                                  e.g. "City: {name}, State: {admin1}, Country: {country} {continent} - {languages}"

                                If an invalid template is specified, "Invalid dynfmt template" is returned.

                                Both predefined and dynamic formatting are cached. Subsequent calls
                                with the same result will be faster as it will use the cached result instead
                                of searching the Geonames index.

                                DYNAMIC COLUMNS ("%dyncols:") FORMATTING:
                                Finally, you can use the special format "%dyncols:" to dynamically add multiple
                                columns to the output CSV using fields from a geocode result.
                                To do so, set --formatstr to "%dyncols:" followed by a comma-delimited list
                                of key:value pairs enclosed in curly braces.
                                The key is the desired column name and the value is one of the same fields
                                available for dynamic formatting.

                                 e.g. "%dyncols: {city_col:name}, {state_col:admin1}, {county_col:admin2}"

                                will add three columns to the output CSV named city_col, state_col & county_col.

                                Note that using "%dyncols:" will cause the the command to geocode EACH row without
                                using the cache, so it will be slower than predefined or dynamic formatting.
                                Also, countryinfo and countryinfonow subcommands currently do not support "%dyncols:".
                                [default: %+]
    -l, --language <lang>       The language to use when geocoding. The language is specified as a ISO 639-1 code.
                                Note that the Geonames index must have been built with the specified language
                                using the `index-update` subcommand with the --languages option.
                                If the language is not available, the first language in the index is used.
                                [default: en]

    --invalid-result <string>   The string to return when the geocode result is empty/invalid.
                                If not set, the original value is used.
    -j, --jobs <arg>            The number of jobs to run in parallel.
                                When not set, the number of jobs is set to the number of CPUs detected.
    -b, --batch <size>          The number of rows per batch to load into memory, before running in parallel.
                                [default: 50000]
    --timeout <seconds>         Timeout for downloading Geonames cities index.
                                [default: 120]
    --cache-dir <dir>           The directory to use for caching the Geonames cities index.
                                If the directory does not exist, qsv will attempt to create it.
                                If the QSV_CACHE_DIR envvar is set, it will be used instead.
                                [default: ~/.qsv-cache]                                

                                INDEX-UPDATE only options:
    --languages <lang-list>     The comma-delimited, case-insentive list of languages to use when building
                                the Geonames cities index.
                                The languages are specified as a comma-separated list of ISO 639-1 codes.
                                [default: en]
    --cities-url <url>          The URL to download the Geonames cities file from. There are several
                                available at https://download.geonames.org/export/dump/.
                                  cities500.zip   - cities with populations > 500; ~200k cities
                                  cities1000.zip  - population > 1000; ~140k cities
                                  cities5000.zip  - population > 5000; ~53k cities
                                  cities15000.zip - population > 15000; ~26k cities
                                Note that the more cities are included, the larger the local index file will be,
                                lookup times will be slower, and the search results will be different.
                                [default: https://download.geonames.org/export/dump/cities15000.zip]
    --force                     Force update the Geonames cities index. If not set, qsv will check if there
                                are updates available at Geonames.org before updating the index.

Common options:
    -h, --help                  Display this message
    -o, --output <file>         Write output to <file> instead of stdout.
    -d, --delimiter <arg>       The field delimiter for reading CSV data.
                                Must be a single character. (default: ,)
    -p, --progressbar           Show progress bars. Will also show the cache hit rate upon completion.
                                Not valid for stdin.
"#;

use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use ahash::RandomState;
use cached::{proc_macro::cached, SizedCache};
use dynfmt::Format;
use geosuggest_core::{
    storage::{self, IndexStorage},
    CitiesRecord, CountryRecord, Engine,
};
use geosuggest_utils::{IndexUpdater, IndexUpdaterSettings, SourceItem};
use indicatif::{ProgressBar, ProgressDrawTarget};
use log::info;
use rayon::{
    iter::{IndexedParallelIterator, ParallelIterator},
    prelude::IntoParallelRefIterator,
};
use regex::Regex;
use serde::Deserialize;
use serde_json::json;
use simple_home_dir::expand_tilde;
use url::Url;
use uuid::Uuid;

use crate::{
    clitypes::CliError,
    config::{Config, Delimiter},
    regex_oncelock,
    select::SelectColumns,
    util, CliResult,
};

#[derive(Deserialize)]
struct Args {
    arg_column:          String,
    arg_location:        String,
    cmd_suggest:         bool,
    cmd_suggestnow:      bool,
    cmd_reverse:         bool,
    cmd_reversenow:      bool,
    cmd_countryinfo:     bool,
    cmd_countryinfonow:  bool,
    cmd_index_check:     bool,
    cmd_index_update:    bool,
    cmd_index_load:      bool,
    cmd_index_reset:     bool,
    arg_input:           Option<String>,
    arg_index_file:      Option<String>,
    flag_rename:         Option<String>,
    flag_country:        Option<String>,
    flag_min_score:      Option<f32>,
    flag_admin1:         Option<String>,
    flag_k_weight:       Option<f32>,
    flag_formatstr:      String,
    flag_language:       String,
    flag_invalid_result: Option<String>,
    flag_batch:          u32,
    flag_timeout:        u16,
    flag_cache_dir:      String,
    flag_languages:      String,
    flag_cities_url:     String,
    flag_force:          bool,
    flag_jobs:           Option<usize>,
    flag_new_column:     Option<String>,
    flag_output:         Option<String>,
    flag_delimiter:      Option<Delimiter>,
    flag_progressbar:    bool,
}

#[derive(Clone, Debug)]
struct Admin1Filter {
    admin1_string: String,
    is_code:       bool,
}

#[derive(Clone)]
struct NamesLang {
    cityname:    String,
    admin1name:  String,
    admin2name:  String,
    countryname: String,
}

static QSV_VERSION: &str = env!("CARGO_PKG_VERSION");
static DEFAULT_GEOCODE_INDEX_FILENAME: &str =
    concat!("qsv-", env!("CARGO_PKG_VERSION"), "-geocode-index.bincode");

static DEFAULT_CITIES_NAMES_URL: &str =
    "https://download.geonames.org/export/dump/alternateNamesV2.zip";
static DEFAULT_CITIES_NAMES_FILENAME: &str = "alternateNamesV2.txt";
static DEFAULT_COUNTRY_INFO_URL: &str = "https://download.geonames.org/export/dump/countryInfo.txt";
static DEFAULT_ADMIN1_CODES_URL: &str =
    "https://download.geonames.org/export/dump/admin1CodesASCII.txt";
static DEFAULT_ADMIN2_CODES_URL: &str = "https://download.geonames.org/export/dump/admin2Codes.txt";

// ensure the state is sorted alphabetically
// as we use binary_search to lookup the state FIPS code
static US_STATES_FIPS_CODES: &[(&str, &str)] = &[
    ("AK", "02"),
    ("AL", "01"),
    ("AR", "05"),
    ("AZ", "04"),
    ("CA", "06"),
    ("CO", "08"),
    ("CT", "09"),
    ("DC", "11"),
    ("DE", "10"),
    ("FL", "12"),
    ("GA", "13"),
    ("HI", "15"),
    ("IA", "19"),
    ("ID", "16"),
    ("IL", "17"),
    ("IN", "18"),
    ("KS", "20"),
    ("KY", "21"),
    ("LA", "22"),
    ("MA", "25"),
    ("MD", "24"),
    ("ME", "23"),
    ("MI", "26"),
    ("MN", "27"),
    ("MO", "29"),
    ("MS", "28"),
    ("MT", "30"),
    ("NC", "37"),
    ("ND", "38"),
    ("NE", "31"),
    ("NH", "33"),
    ("NJ", "34"),
    ("NM", "35"),
    ("NV", "32"),
    ("NY", "36"),
    ("OH", "39"),
    ("OK", "40"),
    ("OR", "41"),
    ("PA", "42"),
    ("RI", "44"),
    ("SC", "45"),
    ("SD", "46"),
    ("TN", "47"),
    ("TX", "48"),
    ("UT", "49"),
    ("VT", "50"),
    ("VA", "51"),
    ("WA", "53"),
    ("WI", "55"),
    ("WV", "54"),
    ("WY", "56"),
    // the following are territories
    // and are not included in the default index
    // leaving them here for reference
    // ("AS", "60"),
    // ("GU", "66"),
    // ("MP", "69"),
    // ("PR", "72"),
    // ("UM", "74"),
    // ("VI", "78"),
];

// max number of entries in LRU cache
static CACHE_SIZE: usize = 2_000_000;
// max number of entries in fallback LRU cache if we can't allocate CACHE_SIZE
static FALLBACK_CACHE_SIZE: usize = CACHE_SIZE / 4;

static INVALID_DYNFMT: &str = "Invalid dynfmt template.";
static INVALID_COUNTRY_CODE: &str = "Invalid country code.";

// when suggesting with --admin1, how many suggestions to fetch from the engine
// before filtering by admin1
static SUGGEST_ADMIN1_LIMIT: usize = 10;

// valid column values for %dyncols
// when adding new columns, make sure to maintain the sort order
// otherwise, the dyncols check will fail as it uses binary search
static SORTED_VALID_DYNCOLS: [&str; 28] = [
    "admin1",
    "admin2",
    "area",
    "capital",
    "continent",
    "country",
    "country_geonameid",
    "country_name",
    "country_population",
    "currency_code",
    "currency_name",
    "equivalent_fips_code",
    "fips",
    "id",
    "iso3",
    "languages",
    "latitude",
    "longitude",
    "name",
    "neighbours",
    "phone",
    "population",
    "postal_code_format",
    "postal_code_regex",
    "timezone",
    "tld",
    "us_county_fips_code",
    "us_state_fips_code",
];

// dyncols populated sentinel value
static DYNCOLS_POPULATED: &str = "_POPULATED";

// valid subcommands
#[derive(Clone, Copy, PartialEq)]
enum GeocodeSubCmd {
    Suggest,
    SuggestNow,
    Reverse,
    ReverseNow,
    CountryInfo,
    CountryInfoNow,
    IndexCheck,
    IndexUpdate,
    IndexLoad,
    IndexReset,
}

// we need this as geosuggest uses anyhow::Error
impl From<anyhow::Error> for CliError {
    fn from(err: anyhow::Error) -> CliError {
        CliError::Other(format!("Error: {err}"))
    }
}

#[inline]
fn replace_column_value(
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

    if args.flag_new_column.is_some() && args.flag_rename.is_some() {
        return fail_incorrectusage_clierror!(
            "Cannot use --new-column and --rename at the same time."
        );
    }

    if args.flag_new_column.is_some() && args.flag_formatstr.starts_with("%dyncols:") {
        return fail_incorrectusage_clierror!(
            "Cannot use --new-column with the '%dyncols:' --formatstr option."
        );
    }

    if let Err(err) = Url::parse(&args.flag_cities_url) {
        return fail_incorrectusage_clierror!(
            "Invalid --cities-url: {url} - {err}",
            url = args.flag_cities_url,
            err = err
        );
    }

    // we need to use tokio runtime as geosuggest uses async
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(geocode_main(args))?;

    Ok(())
}

// main async geocode function that does the actual work
async fn geocode_main(args: Args) -> CliResult<()> {
    let mut index_cmd = false;
    let mut now_cmd = false;
    let geocode_cmd = if args.cmd_suggest {
        GeocodeSubCmd::Suggest
    } else if args.cmd_reverse {
        GeocodeSubCmd::Reverse
    } else if args.cmd_countryinfo {
        GeocodeSubCmd::CountryInfo
    } else if args.cmd_suggestnow {
        now_cmd = true;
        GeocodeSubCmd::SuggestNow
    } else if args.cmd_reversenow {
        now_cmd = true;
        GeocodeSubCmd::ReverseNow
    } else if args.cmd_countryinfonow {
        now_cmd = true;
        GeocodeSubCmd::CountryInfoNow
    } else if args.cmd_index_check {
        index_cmd = true;
        GeocodeSubCmd::IndexCheck
    } else if args.cmd_index_update {
        index_cmd = true;
        GeocodeSubCmd::IndexUpdate
    } else if args.cmd_index_load {
        index_cmd = true;
        GeocodeSubCmd::IndexLoad
    } else if args.cmd_index_reset {
        index_cmd = true;
        GeocodeSubCmd::IndexReset
    } else {
        // should not happen as docopt won't allow it
        unreachable!();
    };

    // setup cache directory
    let geocode_cache_dir = if let Ok(cache_dir) = std::env::var("QSV_CACHE_DIR") {
        // if QSV_CACHE_DIR env var is set, check if it exists. If it doesn't, create it.
        if cache_dir.starts_with('~') {
            // QSV_CACHE_DIR starts with ~, expand it
            expand_tilde(&cache_dir).unwrap()
        } else {
            PathBuf::from(cache_dir)
        }
    } else {
        // QSV_CACHE_DIR env var is not set, use args.flag_cache_dir
        // first check if it starts with ~, expand it
        if args.flag_cache_dir.starts_with('~') {
            expand_tilde(&args.flag_cache_dir).unwrap()
        } else {
            PathBuf::from(&args.flag_cache_dir)
        }
    };
    if !Path::new(&geocode_cache_dir).exists() {
        fs::create_dir_all(&geocode_cache_dir)?;
    }

    info!("Using cache directory: {}", geocode_cache_dir.display());

    let geocode_index_filename = std::env::var("QSV_GEOCODE_INDEX_FILENAME")
        .unwrap_or_else(|_| DEFAULT_GEOCODE_INDEX_FILENAME.to_string());
    let geocode_index_file = args
        .arg_index_file
        .clone()
        .unwrap_or_else(|| format!("{}/{}", geocode_cache_dir.display(), geocode_index_filename));

    // create a TempDir for the one record CSV we're creating if we're doing a Now command
    // we're doing this at this scope so the TempDir is automatically dropped after we're done
    let tempdir = tempfile::Builder::new()
        .prefix("qsv-geocode")
        .tempdir()
        .unwrap();

    // we're doing a SuggestNow, ReverseNow or CountryInfoNow - create a one record CSV in tempdir
    // with one column named "Location" and the passed location value and use it as the input
    let input = if now_cmd {
        let tempdir_path = tempdir.path().to_string_lossy().to_string();
        let temp_csv_path = format!("{}/{}.csv", tempdir_path, Uuid::new_v4());
        let temp_csv_path = Path::new(&temp_csv_path);
        let mut temp_csv_wtr = csv::WriterBuilder::new().from_path(temp_csv_path)?;
        temp_csv_wtr.write_record(["Location"])?;
        temp_csv_wtr.write_record([&args.arg_location])?;
        temp_csv_wtr.flush()?;
        Some(temp_csv_path.to_string_lossy().to_string())
    } else {
        args.arg_input
    };

    let rconfig = Config::new(&input)
        .delimiter(args.flag_delimiter)
        .select(SelectColumns::parse(&args.arg_column)?);

    // prep progress bar
    let show_progress =
        (args.flag_progressbar || util::get_envvar_flag("QSV_PROGRESSBAR")) && !rconfig.is_stdin();

    let progress = ProgressBar::with_draw_target(None, ProgressDrawTarget::stderr_with_hz(5));
    if show_progress {
        util::prep_progress(&progress, util::count_rows(&rconfig)?);
    } else {
        progress.set_draw_target(ProgressDrawTarget::hidden());
    }

    if index_cmd {
        // cities_filename is derived from the cities_url
        // the filename is the last component of the URL with a .txt extension
        // e.g. https://download.geonames.org/export/dump/cities15000.zip -> cities15000.txt
        let cities_filename = args
            .flag_cities_url
            .split('/')
            .last()
            .unwrap()
            .replace(".zip", ".txt");

        // setup languages
        let languages_string_vec = args
            .flag_languages
            .split(',')
            .map(|s| s.trim().to_ascii_lowercase())
            .collect::<Vec<String>>();
        let languages_vec: Vec<&str> = languages_string_vec
            .iter()
            .map(std::string::String::as_str)
            .collect();

        info!("geocode_index_file: {geocode_index_file} Languages: {languages_vec:?}");

        let indexupdater_settings = IndexUpdaterSettings {
            http_timeout_ms:  util::timeout_secs(args.flag_timeout)? * 1000,
            cities:           SourceItem {
                url:      &args.flag_cities_url,
                filename: &cities_filename,
            },
            names:            Some(SourceItem {
                url:      DEFAULT_CITIES_NAMES_URL,
                filename: DEFAULT_CITIES_NAMES_FILENAME,
            }),
            countries_url:    Some(DEFAULT_COUNTRY_INFO_URL),
            admin1_codes_url: Some(DEFAULT_ADMIN1_CODES_URL),
            admin2_codes_url: Some(DEFAULT_ADMIN2_CODES_URL),
            filter_languages: languages_vec.clone(),
        };

        let updater = IndexUpdater::new(indexupdater_settings.clone())?;

        let storage = storage::bincode::Storage::new();

        match geocode_cmd {
            // check if Geoname index needs to be updated from the Geonames website
            // also returns the index file metadata as JSON
            GeocodeSubCmd::IndexCheck => {
                winfo!("Checking main Geonames website for updates...");
                check_index_file(&geocode_index_file)?;

                let metadata = storage
                    .read_metadata(geocode_index_file)
                    .map_err(|e| format!("index-check error: {e}"))?;

                let index_metadata_json = match serde_json::to_string_pretty(&metadata) {
                    Ok(json) => json,
                    Err(e) => {
                        let json_error = json!({
                            "errors": [{
                                "title": "Cannot serialize index metadata to JSON",
                                "detail": e.to_string()
                            }]
                        });
                        format!("{json_error}")
                    },
                };

                match metadata {
                    Some(m) if updater.has_updates(&m).await? => {
                        winfo!(
                            "Updates available at Geonames.org. Use `qsv geocode index-update` to \
                             update/rebuild the index.\nPlease use this judiciously as Geonames \
                             is a free service."
                        );
                    },
                    Some(_) => {
                        winfo!("Geonames index up-to-date.");
                    },
                    None => return fail_incorrectusage_clierror!("Invalid Geonames index file."),
                }
                println!("{index_metadata_json}");
            },
            GeocodeSubCmd::IndexUpdate => {
                // update/rebuild Geonames index from Geonames website
                // will only update if there are changes unless --force is specified
                check_index_file(&geocode_index_file)?;

                let metadata = storage
                    .read_metadata(geocode_index_file.clone())
                    .map_err(|e| format!("index-update error: {e}"))?;

                if args.flag_force {
                    winfo!("Forcing fresh build of Geonames index: {geocode_index_file}");
                    winfo!(
                        "Using cities URL: {}  Languages: {:?}",
                        args.flag_cities_url,
                        languages_vec
                    );
                    winfo!(
                        "This will take a while as we need to download data & rebuild the index..."
                    );

                    let engine = updater.build().await?;
                    storage
                        .dump_to(geocode_index_file.clone(), &engine)
                        .map_err(|e| format!("{e}"))?;
                    winfo!("Geonames index successfully rebuilt: {geocode_index_file}");
                } else {
                    winfo!("Checking main Geonames website for updates...");

                    if updater.has_updates(&metadata.unwrap()).await? {
                        winfo!(
                            "Updating/Rebuilding Geonames index. This will take a while as we \
                             need to download data from Geonames & rebuild the index..."
                        );
                        let engine = updater.build().await?;
                        let _ = storage.dump_to(geocode_index_file.clone(), &engine);
                        winfo!("Updates successfully applied: {geocode_index_file}");
                    } else {
                        winfo!("Skipping update. Geonames index is up-to-date.");
                    }
                }
            },
            GeocodeSubCmd::IndexLoad => {
                // load alternate geocode index file
                if let Some(index_file) = args.arg_index_file {
                    winfo!("Validating alternate Geonames index: {index_file}...");
                    check_index_file(&index_file)?;

                    let engine = load_engine(index_file.clone().into(), &progress).await?;
                    // we successfully loaded the alternate geocode index file, so its valid
                    // copy it to the default geocode index file

                    if engine.metadata.is_some() {
                        let _ = storage.dump_to(geocode_index_file.clone(), &engine);
                        winfo!(
                            "Valid Geonames index file {index_file} successfully copied to \
                             {geocode_index_file}. It will be used from now on or until you \
                             reset/rebuild it.",
                        );
                    } else {
                        return fail_incorrectusage_clierror!(
                            "Alternate Geonames index file {index_file} is invalid.",
                        );
                    }
                } else {
                    return fail_incorrectusage_clierror!(
                        "No alternate Geonames index file specified."
                    );
                }
            },
            GeocodeSubCmd::IndexReset => {
                // reset geocode index by deleting the current local copy
                // and downloading the default geocode index for the current qsv version
                winfo!("Resetting Geonames index to default: {geocode_index_file}...");
                fs::remove_file(&geocode_index_file)?;
                load_engine(geocode_index_file.clone().into(), &progress).await?;
                winfo!("Default Geonames index file successfully reset to {QSV_VERSION} release.");
            },
            // index_cmd is true, so we should never get a non-index subcommand
            _ => unreachable!(),
        }
        return Ok(());
    }

    // we're not doing an index subcommand, so we're doing a suggest/now, reverse/now
    // or countryinfo/now subcommand. Load the current local Geonames index
    let engine = load_engine(geocode_index_file.clone().into(), &progress).await?;

    let mut rdr = rconfig.reader()?;
    let mut wtr = Config::new(&args.flag_output)
        .quote_style(
            // if we're doing a now subcommand with JSON output, we don't want the CSV writer
            // to close quote the output as it will produce invalid JSON
            if now_cmd && (args.flag_formatstr == "%json" || args.flag_formatstr == "%pretty-json")
            {
                csv::QuoteStyle::Never
            } else {
                csv::QuoteStyle::Necessary
            },
        )
        .writer()?;

    let headers = rdr.byte_headers()?.clone();
    let sel = rconfig.selection(&headers)?;
    let column_index = *sel.iter().next().unwrap();

    let mut headers = rdr.headers()?.clone();

    if let Some(new_name) = args.flag_rename {
        let new_col_names = util::ColumnNameParser::new(&new_name).parse()?;
        if new_col_names.len() != sel.len() {
            return fail_incorrectusage_clierror!(
                "Number of new columns does not match input column selection."
            );
        }
        for (i, col_index) in sel.iter().enumerate() {
            headers = replace_column_value(&headers, *col_index, &new_col_names[i]);
        }
    }

    // setup output headers
    if let Some(new_column) = &args.flag_new_column {
        headers.push_field(new_column);
    }

    // if formatstr starts with "%dyncols:"", then we're using dynfmt to add columns.
    // To add columns, we enclose in curly braces a key:value pair for each column with
    // the key being the desired column name and the value being the CityRecord field
    // we want to add to the CSV
    // e.g. "%dyncols: {city_col:name}, {state_col:admin1}, {country_col:country}"
    // will add three columns to the CSV named city_col, state_col and country_col.

    // first, parse the formatstr to get the column names and values in parallel vectors
    let mut column_names = Vec::new();
    let mut column_values = Vec::new();
    // dyncols_len is the number of columns we're adding in dyncols mode
    // it also doubles as a flag to indicate if we're using dyncols mode
    // i.e. if dyncols_len > 0, we're using dyncols mode; 0 we're not
    let dyncols_len = if args.flag_formatstr.starts_with("%dyncols:") {
        for column in args.flag_formatstr[9..].split(',') {
            let column = column.trim();
            let column_key_value: Vec<&str> = column.split(':').collect();
            if column_key_value.len() == 2 {
                column_names.push(column_key_value[0].trim_matches('{'));
                column_values.push(column_key_value[1].trim_matches('}'));
            }
        }

        // now, validate the column values
        // the valid column values are in SORTED_VALID_DYNCOLS
        for column_value in &column_values {
            if SORTED_VALID_DYNCOLS.binary_search(column_value).is_err() {
                return fail_incorrectusage_clierror!(
                    "Invalid column value: {column_value}. Valid values are: \
                     {SORTED_VALID_DYNCOLS:?}"
                );
            }
        }

        // its valid, add the columns to the CSV headers
        for column in column_names {
            headers.push_field(column);
        }
        column_values.len() as u8
    } else {
        0_u8
    };

    // now, write the headers to the output CSV, unless its a now subcommand with JSON output
    if !(now_cmd && (args.flag_formatstr == "%json" || args.flag_formatstr == "%pretty-json")) {
        wtr.write_record(&headers)?;
    }

    // setup admin1 filter for Suggest/Now
    let mut admin1_code_prefix = String::new();
    let mut admin1_same_prefix = true;
    let mut flag_country = args.flag_country.clone();
    let admin1_filter_list = match geocode_cmd {
        GeocodeSubCmd::Suggest | GeocodeSubCmd::SuggestNow => {
            // admin1 filter: if all uppercase, search for admin1 code, else, search for admin1 name
            // see https://download.geonames.org/export/dump/admin1CodesASCII.txt for valid codes
            if let Some(admin1_list) = args.flag_admin1.clone() {
                // this regex matches admin1 codes (e.g. US.NY, JP.40, CN.23, HK.NYL, GG.6417214)
                let admin1_code_re = Regex::new(r"^[A-Z]{2}.[A-Z0-9]{1,8}$").unwrap();
                let admin1_list_work = Some(
                    admin1_list
                        .split(',')
                        .map(|s| {
                            let temp_s = s.trim();
                            let is_code_flag = admin1_code_re.is_match(temp_s);
                            Admin1Filter {
                                admin1_string: if is_code_flag {
                                    if admin1_same_prefix {
                                        // check if all admin1 codes have the same prefix
                                        if admin1_code_prefix.is_empty() {
                                            // first admin1 code, so set the prefix
                                            admin1_code_prefix = temp_s[0..3].to_string();
                                        } else if admin1_code_prefix != temp_s[0..3] {
                                            // admin1 codes have different prefixes, so we can't
                                            // infer the country from the admin1 code
                                            admin1_same_prefix = false;
                                        }
                                    }
                                    temp_s.to_string()
                                } else {
                                    // its an admin1 name, lowercase it
                                    // so we can do case-insensitive starts_with() comparisons
                                    temp_s.to_lowercase()
                                },
                                is_code:       is_code_flag,
                            }
                        })
                        .collect::<Vec<Admin1Filter>>(),
                );

                // if admin1 is set, country must also be set
                // however, if all admin1 codes have the same prefix, we can infer the country from
                // the admin1 codes. Otherwise, we can't infer the country from the
                // admin1 code, so we error out.
                if args.flag_admin1.is_some() && flag_country.is_none() {
                    if !admin1_code_prefix.is_empty() && admin1_same_prefix {
                        admin1_code_prefix.pop(); // remove the dot
                        flag_country = Some(admin1_code_prefix);
                    } else {
                        return fail_incorrectusage_clierror!(
                            "If --admin1 is set, --country must also be set unless admin1 codes \
                             are used with a common country prefix (e.g. US.CA,US.NY,US.OH, etc)."
                        );
                    }
                }
                admin1_list_work
            } else {
                None
            }
        },
        _ => {
            // reverse/now and countryinfo/now subcommands don't support admin1 filter
            if args.flag_admin1.is_some() {
                return fail_incorrectusage_clierror!(
                    "reverse/reversenow & countryinfo subcommands do not support the --admin1 \
                     filter option."
                );
            }
            None
        },
    }; // end setup admin1 filters

    // setup country filter - both suggest/now and reverse/now support country filters
    // countryinfo/now subcommands ignores the country filter
    let country_filter_list = flag_country.map(|country_list| {
        country_list
            .split(',')
            .map(|s| s.trim().to_ascii_lowercase())
            .collect::<Vec<String>>()
    });

    log::debug!("country_filter_list: {country_filter_list:?}");
    log::debug!("admin1_filter_list: {admin1_filter_list:?}");

    // amortize memory allocation by reusing record
    #[allow(unused_assignments)]
    let mut batch_record = csv::StringRecord::new();

    // reuse batch buffers
    let batchsize: usize = args.flag_batch as usize;
    let mut batch = Vec::with_capacity(batchsize);
    let mut batch_results = Vec::with_capacity(batchsize);

    // set RAYON_NUM_THREADS
    util::njobs(args.flag_jobs);

    let invalid_result = args.flag_invalid_result.unwrap_or_default();

    // main loop to read CSV and construct batches for parallel processing.
    // each batch is processed via Rayon parallel iterator.
    // loop exits when batch is empty.
    'batch_loop: loop {
        for _ in 0..batchsize {
            match rdr.read_record(&mut batch_record) {
                Ok(has_data) => {
                    if has_data {
                        batch.push(batch_record.clone());
                    } else {
                        // nothing else to add to batch
                        break;
                    }
                },
                Err(e) => {
                    return fail_clierror!("Error reading file: {e}");
                },
            }
        }

        if batch.is_empty() {
            // break out of infinite loop when at EOF
            break 'batch_loop;
        }

        let min_score = args.flag_min_score;
        let k_weight = args.flag_k_weight;

        // do actual apply command via Rayon parallel iterator
        batch
            .par_iter()
            .map(|record_item| {
                let mut record = record_item.clone();
                let mut cell = record[column_index].to_owned();
                if cell.is_empty() {
                    // cell to geocode is empty. If in dyncols mode, we need to add empty columns.
                    // Otherwise, we leave the row untouched.
                    if dyncols_len > 0 {
                        // we're in dyncols mode, so add empty columns
                        (0..dyncols_len).for_each(|_| {
                            record.push_field("");
                        });
                    }
                } else if geocode_cmd == GeocodeSubCmd::CountryInfo
                    || geocode_cmd == GeocodeSubCmd::CountryInfoNow
                {
                    // we're doing a countryinfo or countryinfonow subcommand
                    cell =
                        get_countryinfo(&engine, &cell, &args.flag_language, &args.flag_formatstr)
                            .unwrap_or(cell);
                } else if dyncols_len > 0 {
                    // we're in dyncols mode, so use search_index_NO_CACHE fn
                    // as we need to inject the column values into each row of the output csv
                    // so we can't use the cache
                    let search_results = search_index_no_cache(
                        &engine,
                        geocode_cmd,
                        &cell,
                        &args.flag_formatstr,
                        &args.flag_language,
                        min_score,
                        k_weight,
                        &country_filter_list,
                        &admin1_filter_list,
                        &column_values,
                        &mut record,
                    );

                    // if search_results.is_some but we don't get the DYNCOLS_POPULATED
                    // sentinel value or its None, then we have an invalid result
                    let invalid = if let Some(res) = search_results {
                        res != DYNCOLS_POPULATED
                    } else {
                        true
                    };
                    if invalid {
                        if invalid_result.is_empty() {
                            // --invalid-result is not set, so add empty columns
                            (0..dyncols_len).for_each(|_| {
                                record.push_field("");
                            });
                        } else {
                            // --invalid-result is set
                            // so add columns set to --invalid-result value
                            (0..dyncols_len).for_each(|_| {
                                record.push_field(&invalid_result.clone());
                            });
                        }
                    }
                } else {
                    // not in dyncols mode so call the CACHED search_index fn
                    // as we want to take advantage of the cache
                    let search_result = search_index(
                        &engine,
                        geocode_cmd,
                        &cell,
                        &args.flag_formatstr,
                        &args.flag_language,
                        min_score,
                        k_weight,
                        &country_filter_list,
                        &admin1_filter_list,
                        &column_values,
                        &mut record,
                    );

                    if let Some(geocoded_result) = search_result {
                        // we have a valid geocode result, so use that
                        cell = geocoded_result;
                    } else {
                        // we have an invalid geocode result
                        if !invalid_result.is_empty() {
                            // --invalid-result is set, so use that instead
                            // otherwise, we leave cell untouched.
                            cell = invalid_result.clone();
                        }
                    }
                }
                // }
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

        if show_progress {
            progress.inc(batch.len() as u64);
        }

        batch.clear();
    } // end batch loop

    if show_progress {
        // the geocode result cache is NOT used in dyncols mode,
        // so update the cache info only when dyncols_len == 0
        if dyncols_len == 0 {
            util::update_cache_info!(progress, SEARCH_INDEX);
        }
        util::finish_progress(&progress);
    }
    Ok(wtr.flush()?)
}

/// check if index_file exists and ends with a .bincode extension
fn check_index_file(index_file: &String) -> CliResult<()> {
    if !index_file.ends_with(".bincode") {
        return fail_incorrectusage_clierror!(
            "Alternate Geonames index file {index_file} does not have a .bincode extension."
        );
    }
    // check if index_file exist
    if !Path::new(index_file).exists() {
        return fail_incorrectusage_clierror!(
            "Alternate Geonames index file {index_file} does not exist."
        );
    }

    winfo!("Valid: {index_file}");
    Ok(())
}

/// load_engine loads the Geonames index file into memory
/// if the index file does not exist, it will download the default index file
/// from the qsv GitHub repo
async fn load_engine(geocode_index_file: PathBuf, progressbar: &ProgressBar) -> CliResult<Engine> {
    let index_file = std::path::Path::new(&geocode_index_file);

    if index_file.exists() {
        // load existing local index
        progressbar.println(format!(
            "Loading existing Geonames index from {}",
            index_file.display()
        ));
    } else {
        // initial load or index-reset, download index file from qsv releases
        progressbar.println(format!(
            "Downloading default Geonames index for qsv {QSV_VERSION} release..."
        ));

        util::download_file(
            &format!(
                "https://github.com/jqnatividad/qsv/releases/download/{QSV_VERSION}/{DEFAULT_GEOCODE_INDEX_FILENAME}"
            ),
            geocode_index_file.clone(),
            !progressbar.is_hidden(),
            None,
            None,
            None,
        )
        .await?;
    }
    let storage = storage::bincode::Storage::new();

    let engine = storage
        .load_from(geocode_index_file)
        .map_err(|e| format!("On load index file: {e}"))?;

    Ok(engine)
}

/// search_index is a cached function that returns a geocode result for a given cell value.
/// It is used by the suggest/suggestnow and reverse/reversenow subcommands.
/// It uses an LRU cache using the cell value/language as the key, storing the formatted geocoded
/// result in the cache. As such, we CANNOT use the cache when in dyncols mode as the cached result
/// is the formatted result, not the individual fields.
/// search_index_no_cache() is automatically derived from search_index() by the cached macro.
/// search_index_no_cache() is used in dyncols mode, and as the name implies, does not use a cache.
#[cached(
    type = "SizedCache<String, String>",
    create = "{ SizedCache::try_with_size(CACHE_SIZE).unwrap_or_else(|_| \
              SizedCache::with_size(FALLBACK_CACHE_SIZE)) }",
    key = "String",
    convert = r#"{ format!("{cell}-{lang_lookup}") }"#,
    option = true
)]
fn search_index(
    engine: &Engine,
    mode: GeocodeSubCmd,
    cell: &str,
    formatstr: &str,
    lang_lookup: &str,
    min_score: Option<f32>,
    k: Option<f32>,
    country_filter_list: &Option<Vec<String>>,
    admin1_filter_list: &Option<Vec<Admin1Filter>>,
    column_values: &[&str], //&Vec<&str>,
    record: &mut csv::StringRecord,
) -> Option<String> {
    if mode == GeocodeSubCmd::Suggest || mode == GeocodeSubCmd::SuggestNow {
        let search_result: Vec<&CitiesRecord>;
        let cityrecord = if admin1_filter_list.is_none() {
            // no admin1 filter, run a search for 1 result (top match)
            search_result = engine.suggest(cell, 1, min_score, country_filter_list.as_deref());
            let Some(cr) = search_result.into_iter().next() else {
                // no results, so return early with None
                return None;
            };
            cr
        } else {
            // we have an admin1 filter, run a search for top SUGGEST_ADMIN1_LIMIT results
            search_result = engine.suggest(
                cell,
                SUGGEST_ADMIN1_LIMIT,
                min_score,
                country_filter_list.as_deref(),
            );

            // first, get the first result and store that in cityrecord
            let Some(cr) = search_result.clone().into_iter().next() else {
                // no results, so return early with None
                return None;
            };
            let first_result = cr;

            // then iterate through search results and find the first one that matches admin1
            // the search results are already sorted by score, so we just need to find the first
            if let Some(admin1_filter_list) = admin1_filter_list {
                // we have an admin1 filter, so we need to find the first admin1 result that matches
                let mut admin1_filter_map: HashMap<String, bool, RandomState> = HashMap::default();
                for admin1_filter in admin1_filter_list {
                    admin1_filter_map
                        .insert(admin1_filter.clone().admin1_string, admin1_filter.is_code);
                }
                let mut matched_record: Option<&CitiesRecord> = None;
                'outer: for cr in &search_result {
                    if let Some(admin_division) = &cr.admin_division {
                        for (admin1_filter, is_code) in &admin1_filter_map {
                            if *is_code {
                                // admin1 is a code, so we search for admin1 code
                                if admin_division.code.starts_with(admin1_filter) {
                                    matched_record = Some(cr);
                                    break 'outer;
                                }
                            } else {
                                // admin1 is a name, so we search for admin1 name, case-insensitive
                                if admin_division
                                    .name
                                    .to_lowercase()
                                    .starts_with(admin1_filter)
                                {
                                    matched_record = Some(cr);
                                    break 'outer;
                                }
                            }
                        }
                    }
                }

                if let Some(cr) = matched_record {
                    cr
                } else {
                    // no admin1 match, so we return the first result
                    first_result
                }
            } else {
                // no admin1 filter, so we return the first result
                first_result
            }
        };

        let country = cityrecord.country.clone().unwrap().code;

        let nameslang = get_cityrecord_name_in_lang(cityrecord, lang_lookup);

        if formatstr == "%+" {
            // default for suggest is location - e.g. "(lat, long)"
            if mode == GeocodeSubCmd::SuggestNow {
                // however, make SuggestNow default more verbose
                return Some(format!(
                    "{name}, {admin1name} {country}: {latitude}, {longitude}",
                    name = nameslang.cityname,
                    admin1name = nameslang.admin1name,
                    latitude = cityrecord.latitude,
                    longitude = cityrecord.longitude
                ));
            }
            return Some(format!(
                "({latitude}, {longitude})",
                latitude = cityrecord.latitude,
                longitude = cityrecord.longitude
            ));
        }

        let capital = engine
            .capital(&country)
            .map(|cr| cr.name.clone())
            .unwrap_or_default();

        if formatstr.starts_with("%dyncols:") {
            let Some(countryrecord) = engine.country_info(&country) else {
                return None;
            };
            add_dyncols(
                record,
                cityrecord,
                countryrecord,
                &nameslang,
                &country,
                &capital,
                column_values,
            );
            return Some(DYNCOLS_POPULATED.to_string());
        }

        return Some(format_result(
            engine, cityrecord, &nameslang, &country, &capital, formatstr, true,
        ));
    }

    // we're doing a Reverse/Now command and expect a WGS 84 coordinate
    // the regex validates for "(lat, long)" or "lat, long"
    // note that it is not pinned to the start of the string, so it can be in the middle
    // of a string, e.g. "The location of the incident is 40.7128, -74.0060"
    let locregex: &'static Regex =
        regex_oncelock!(r"(?-u)([+-]?[0-9]+\.?[0-9]*|\.[0-9]+),\s*([+-]?[0-9]+\.?[0-9]*|\.[0-9]+)");

    let loccaps = locregex.captures(cell);
    if let Some(loccaps) = loccaps {
        let lat = fast_float::parse(&loccaps[1]).unwrap_or_default();
        let long = fast_float::parse(&loccaps[2]).unwrap_or_default();
        if (-90.0..=90.0).contains(&lat) && (-180.0..=180.0).contains(&long) {
            let search_result = engine.reverse((lat, long), 1, k, country_filter_list.as_deref());
            let Some(cityrecord) = (match search_result {
                Some(search_result) => search_result.into_iter().next().map(|ri| ri.city),
                None => return None,
            }) else {
                return None;
            };

            let nameslang = get_cityrecord_name_in_lang(cityrecord, lang_lookup);

            let country = cityrecord.country.clone().unwrap().code;

            if formatstr == "%+" {
                // default for reverse is city, admin1 country - e.g. "Brooklyn, New York US"
                return Some(format!(
                    "{cityname}, {admin1name} {country}",
                    cityname = nameslang.cityname,
                    admin1name = nameslang.admin1name,
                    country = country,
                ));
            }

            let capital = engine
                .capital(&country)
                .map(|cr| cr.name.clone())
                .unwrap_or_default();

            if formatstr.starts_with("%dyncols:") {
                let Some(countryrecord) = engine.country_info(&country) else {
                    return None;
                };
                add_dyncols(
                    record,
                    cityrecord,
                    countryrecord,
                    &nameslang,
                    &country,
                    &capital,
                    column_values,
                );
                return Some(DYNCOLS_POPULATED.to_string());
            }

            return Some(format_result(
                engine, cityrecord, &nameslang, &country, &capital, formatstr, false,
            ));
        }
    }

    // not a valid lat, long
    return None;
}

/// "%dyncols:" formatstr used. Adds dynamic columns to CSV.
fn add_dyncols(
    record: &mut csv::StringRecord,
    cityrecord: &CitiesRecord,
    countryrecord: &CountryRecord,
    nameslang: &NamesLang,
    country: &str,
    capital: &str,
    column_values: &[&str],
) {
    for column in column_values {
        match *column {
            // CityRecord fields
            "id" => record.push_field(&cityrecord.id.to_string()),
            "name" => record.push_field(&nameslang.cityname),
            "latitude" => record.push_field(&cityrecord.latitude.to_string()),
            "longitude" => record.push_field(&cityrecord.longitude.to_string()),
            "country" => record.push_field(country),
            "admin1" => record.push_field(&nameslang.admin1name),
            "admin2" => record.push_field(&nameslang.admin2name),
            "capital" => record.push_field(capital),
            "timezone" => record.push_field(&cityrecord.timezone),
            "population" => record.push_field(&cityrecord.population.to_string()),

            // US FIPS fields
            "us_state_fips_code" => {
                let us_state_code = if let Some(admin1_code) =
                    cityrecord.admin_division.as_ref().map(|ad| ad.code.clone())
                {
                    if let Some(state_fips_code) = admin1_code.strip_prefix("US.") {
                        // admin1 code is a US state code, the two-letter state code
                        // is the last two characters of the admin1 code
                        state_fips_code.to_string()
                    } else {
                        // admin1 code is not a US state code
                        // set to empty string
                        String::new()
                    }
                } else {
                    // no admin1 code
                    // set to empty string
                    String::new()
                };
                // lookup US state FIPS code
                record.push_field(lookup_us_state_fips_code(&us_state_code).unwrap_or_default());
            },
            "us_county_fips_code" => {
                let us_county_fips_code = if let Some(admin2_code) = cityrecord
                    .admin2_division
                    .as_ref()
                    .map(|ad| ad.code.clone())
                {
                    if admin2_code.starts_with("US.") && admin2_code.len() == 9 {
                        // admin2 code is a US county code, the three-digit county code
                        // is the last three characters of the admin2 code
                        // start at index 7 to skip the US. prefix
                        // e.g. US.NY.061 -> 061
                        format!("{:0>3}", admin2_code[7..].to_string())
                    } else {
                        // admin2 code is not a US county code
                        // set to empty string
                        String::new()
                    }
                } else {
                    // no admin2 code
                    // set to empty string
                    String::new()
                };
                record.push_field(&us_county_fips_code);
            },

            // CountryRecord fields
            "country_name" => record.push_field(&nameslang.countryname),
            "iso3" => record.push_field(&countryrecord.info.iso3),
            "fips" => record.push_field(&countryrecord.info.fips),
            "area" => record.push_field(&countryrecord.info.area.to_string()),
            "country_population" => record.push_field(&countryrecord.info.population.to_string()),
            "continent" => record.push_field(&countryrecord.info.continent),
            "tld" => record.push_field(&countryrecord.info.tld),
            "currency_code" => record.push_field(&countryrecord.info.currency_code),
            "currency_name" => record.push_field(&countryrecord.info.currency_name),
            "phone" => record.push_field(&countryrecord.info.phone),
            "postal_code_format" => record.push_field(&countryrecord.info.postal_code_format),
            "postal_code_regex" => record.push_field(&countryrecord.info.postal_code_regex),
            "languages" => record.push_field(&countryrecord.info.languages),
            "country_geonameid" => record.push_field(&countryrecord.info.geonameid.to_string()),
            "neighbours" => record.push_field(&countryrecord.info.neighbours),
            "equivalent_fips_code" => record.push_field(&countryrecord.info.equivalent_fips_code),

            // this should not happen as column_values has been pre-validated for these values
            _ => unreachable!(),
        }
    }
}

/// format the geocoded result based on formatstr if its not %+
#[cached(
    key = "String",
    convert = r#"{ format!("{}-{}-{}", cityrecord.id, formatstr, suggest_mode) }"#
)]
fn format_result(
    engine: &Engine,
    cityrecord: &CitiesRecord,
    nameslang: &NamesLang,
    country: &str,
    capital: &str,
    formatstr: &str,
    suggest_mode: bool,
) -> String {
    if formatstr.starts_with('%') {
        // if formatstr starts with %, then we're using a predefined format
        match formatstr {
            "%city-state" | "%city-admin1" => {
                format!("{}, {}", nameslang.cityname, nameslang.admin1name)
            },
            "%location" => format!("({}, {})", cityrecord.latitude, cityrecord.longitude),
            "%city-state-country" | "%city-admin1-country" => {
                format!(
                    "{}, {} {}",
                    nameslang.cityname, nameslang.admin1name, country
                )
            },
            "%lat-long" => format!("{}, {}", cityrecord.latitude, cityrecord.longitude),
            "%city-country" => format!("{}, {}", nameslang.cityname, country),
            "%city" => nameslang.cityname.clone(),
            "%city-county-state" | "%city-admin2-admin1" => {
                format!(
                    "{}, {}, {}",
                    nameslang.cityname, nameslang.admin2name, nameslang.admin1name,
                )
            },
            "%state" | "%admin1" => nameslang.admin1name.clone(),
            "%county" | "%admin2" => nameslang.admin2name.clone(),
            "%country" => country.to_owned(),
            "%country_name" => nameslang.countryname.clone(),
            "%id" => format!("{}", cityrecord.id),
            "%capital" => capital.to_owned(),
            "%population" => format!("{}", cityrecord.population),
            "%timezone" => cityrecord.timezone.clone(),
            "%cityrecord" => format!("{cityrecord:?}"),
            "%admin1record" => format!("{:?}", cityrecord.admin_division),
            "%admin2record" => format!("{:?}", cityrecord.admin2_division),
            "%json" => serde_json::to_string(cityrecord).unwrap_or_else(|_| "null".to_string()),
            "%pretty-json" => {
                serde_json::to_string_pretty(cityrecord).unwrap_or_else(|_| "null".to_string())
            },
            _ => {
                // invalid formatstr, so we use the default for suggest/now or reverse/now
                if suggest_mode {
                    // default for suggest is location - e.g. "(lat, long)"
                    format!(
                        "({latitude}, {longitude})",
                        latitude = cityrecord.latitude,
                        longitude = cityrecord.longitude
                    )
                } else {
                    // default for reverse/now is city-admin1-country - e.g. "Brooklyn, New York US"
                    format!(
                        "{city}, {admin1} {country}",
                        city = nameslang.cityname,
                        admin1 = nameslang.admin1name,
                    )
                }
            },
        }
    } else {
        // if formatstr does not start with %, then we're using dynfmt,
        // i.e. twenty-eight predefined fields below in curly braces are replaced with values
        // e.g. "City: {name}, State: {admin1}, Country: {country} - {continent}"
        // unlike the predefined formats, we don't have a default format for dynfmt
        // so we return INVALID_DYNFMT if dynfmt fails to format the string
        // also, we have access to the country info fields as well

        // check if we have a valid country record
        let Some(countryrecord) = engine.country_info(country) else {
            return INVALID_COUNTRY_CODE.to_string();
        };

        // Now, parse the formatstr to get the fields to initialize in
        // the hashmap lookup. We do this so we only populate the hashmap with fields
        // that are actually used in the formatstr.
        let mut dynfmt_fields = Vec::with_capacity(10); // 10 is a reasonable default to save allocs
        let formatstr_re: &'static Regex = crate::regex_oncelock!(r"\{(?P<key>\w+)?\}");
        for format_fields in formatstr_re.captures_iter(formatstr) {
            dynfmt_fields.push(format_fields.name("key").unwrap().as_str());
        }

        let mut cityrecord_map: HashMap<&str, String> = HashMap::with_capacity(dynfmt_fields.len());

        for field in &dynfmt_fields {
            match *field {
                // cityrecord fields
                "id" => cityrecord_map.insert("id", cityrecord.id.to_string()),
                "name" => cityrecord_map.insert("name", nameslang.cityname.clone()),
                "latitude" => cityrecord_map.insert("latitude", cityrecord.latitude.to_string()),
                "longitude" => cityrecord_map.insert("longitude", cityrecord.longitude.to_string()),
                "country" => cityrecord_map.insert("country", country.to_owned()),
                "country_name" => {
                    cityrecord_map.insert("country_name", nameslang.countryname.clone())
                },
                "admin1" => cityrecord_map.insert("admin1", nameslang.admin1name.clone()),
                "admin2" => cityrecord_map.insert("admin2", nameslang.admin2name.clone()),
                "capital" => cityrecord_map.insert("capital", capital.to_owned()),
                "timezone" => cityrecord_map.insert("timezone", cityrecord.timezone.clone()),
                "population" => {
                    cityrecord_map.insert("population", cityrecord.population.to_string())
                },

                // US FIPS fields
                // set US state FIPS code
                "us_state_fips_code" => {
                    let us_state_code = if let Some(admin1_code) =
                        cityrecord.admin_division.as_ref().map(|ad| ad.code.clone())
                    {
                        if let Some(state_fips_code) = admin1_code.strip_prefix("US.") {
                            // admin1 code is a US state code, the two-letter state code
                            // is the last two characters of the admin1 code
                            state_fips_code.to_string()
                        } else {
                            // admin1 code is not a US state code
                            // set to empty string
                            String::new()
                        }
                    } else {
                        // no admin1 code
                        // set to empty string
                        String::new()
                    };
                    cityrecord_map.insert(
                        "us_state_fips_code",
                        lookup_us_state_fips_code(&us_state_code)
                            .unwrap_or_default()
                            .to_string(),
                    )
                },

                // set US county FIPS code
                "us_county_fips_code" => cityrecord_map.insert("us_county_fips_code", {
                    match cityrecord
                        .admin2_division
                        .as_ref()
                        .map(|ad| ad.code.clone())
                    {
                        Some(admin2_code) => {
                            if admin2_code.starts_with("US.") && admin2_code.len() == 9 {
                                // admin2 code is a US county code, the three-digit county code
                                // is the last three characters of the admin2 code
                                // start at index 7 to skip the US. prefix
                                // e.g. US.NY.061 -> 061
                                format!("{:0>3}", admin2_code[7..].to_string())
                            } else {
                                // admin2 code is not a US county code
                                // set to empty string
                                String::new()
                            }
                        },
                        None => {
                            // no admin2 code
                            // set to empty string
                            String::new()
                        },
                    }
                }),

                // countryrecord fields
                "iso3" => cityrecord_map.insert("iso3", countryrecord.info.iso3.clone()),
                "fips" => cityrecord_map.insert("fips", countryrecord.info.fips.clone()),
                "area" => cityrecord_map.insert("area", countryrecord.info.area.to_string()),
                "country_population" => cityrecord_map.insert(
                    "country_population",
                    countryrecord.info.population.to_string(),
                ),
                "continent" => {
                    cityrecord_map.insert("continent", countryrecord.info.continent.clone())
                },
                "tld" => cityrecord_map.insert("tld", countryrecord.info.tld.clone()),
                "currency_code" => {
                    cityrecord_map.insert("currency_code", countryrecord.info.currency_code.clone())
                },
                "currency_name" => {
                    cityrecord_map.insert("currency_name", countryrecord.info.currency_name.clone())
                },
                "phone" => cityrecord_map.insert("phone", countryrecord.info.phone.clone()),
                "postal_code_format" => cityrecord_map.insert(
                    "postal_code_format",
                    countryrecord.info.postal_code_format.clone(),
                ),
                "postal_code_regex" => cityrecord_map.insert(
                    "postal_code_regex",
                    countryrecord.info.postal_code_regex.clone(),
                ),
                "languages" => {
                    cityrecord_map.insert("languages", countryrecord.info.languages.clone())
                },
                "country_geonameid" => cityrecord_map.insert(
                    "country_geonameid",
                    countryrecord.info.geonameid.to_string(),
                ),
                "neighbours" => {
                    cityrecord_map.insert("neighbours", countryrecord.info.neighbours.clone())
                },
                "equivalent_fips_code" => cityrecord_map.insert(
                    "equivalent_fips_code",
                    countryrecord.info.equivalent_fips_code.clone(),
                ),
                _ => return INVALID_DYNFMT.to_string(),
            };
        }

        if let Ok(formatted) = dynfmt::SimpleCurlyFormat.format(formatstr, cityrecord_map) {
            formatted.to_string()
        } else {
            INVALID_DYNFMT.to_string()
        }
    }
}

/// get_countryinfo is a cached function that returns a countryinfo result for a given cell value.
/// It is used by the countryinfo/countryinfonow subcommands.
#[cached(
    key = "String",
    convert = r#"{ format!("{cell}-{lang_lookup}-{formatstr}") }"#,
    option = true
)]
fn get_countryinfo(
    engine: &Engine,
    cell: &str,
    lang_lookup: &str,
    formatstr: &str,
) -> Option<String> {
    let Some(countryrecord) = engine.country_info(cell) else {
        // no results, so return early with None
        return None;
    };

    if formatstr.starts_with('%') {
        // if formatstr starts with %, then we're using a predefined format
        let formatted = match formatstr {
            "%capital" => countryrecord.info.capital.clone(),
            "%continent" => countryrecord.info.continent.clone(),
            "%json" => serde_json::to_string(countryrecord).unwrap_or_else(|_| "null".to_string()),
            "%pretty-json" => {
                serde_json::to_string_pretty(countryrecord).unwrap_or_else(|_| "null".to_string())
            },
            _ => countryrecord
                .names
                .clone()
                .unwrap_or_default()
                .get(lang_lookup)
                .cloned()
                .unwrap_or_default(),
        };
        Some(formatted)
    } else {
        // if formatstr does not start with %, then we're using dynfmt,
        // i.e. sixteen predefined fields below in curly braces are replaced with values
        // e.g. "Country: {country_name}, Continent: {continent} Currency: {currency_name}
        // ({currency_code})})"

        // first, parse the formatstr to get the fields to initialixe in the hashmap lookup
        // we do this so we only populate the hashmap with fields that are actually used
        // in the formatstr.
        let mut dynfmt_fields = Vec::with_capacity(10); // 10 is a reasonable default to save allocs
        let formatstr_re: &'static Regex = crate::regex_oncelock!(r"\{(?P<key>\w+)?\}");
        for format_fields in formatstr_re.captures_iter(formatstr) {
            dynfmt_fields.push(format_fields.name("key").unwrap().as_str());
        }

        let mut countryrecord_map: HashMap<&str, String> =
            HashMap::with_capacity(dynfmt_fields.len());

        for field in &dynfmt_fields {
            match *field {
                "country_name" => countryrecord_map.insert("country_name", {
                    countryrecord
                        .names
                        .clone()
                        .unwrap_or_default()
                        .get(lang_lookup)
                        .cloned()
                        .unwrap_or_default()
                }),
                "iso3" => countryrecord_map.insert("iso3", countryrecord.info.iso3.clone()),
                "fips" => countryrecord_map.insert("fips", countryrecord.info.fips.clone()),
                "capital" => {
                    countryrecord_map.insert("capital", countryrecord.info.capital.clone())
                },
                "area" => countryrecord_map.insert("area", countryrecord.info.area.to_string()),
                "country_population" => countryrecord_map.insert(
                    "country_population",
                    countryrecord.info.population.to_string(),
                ),
                "continent" => {
                    countryrecord_map.insert("continent", countryrecord.info.continent.clone())
                },
                "tld" => countryrecord_map.insert("tld", countryrecord.info.tld.clone()),
                "currency_code" => countryrecord_map
                    .insert("currency_code", countryrecord.info.currency_code.clone()),
                "currency_name" => countryrecord_map
                    .insert("currency_name", countryrecord.info.currency_name.clone()),
                "phone" => countryrecord_map.insert("phone", countryrecord.info.phone.clone()),
                "postal_code_format" => countryrecord_map.insert(
                    "postal_code_format",
                    countryrecord.info.postal_code_format.clone(),
                ),
                "postal_code_regex" => countryrecord_map.insert(
                    "postal_code_regex",
                    countryrecord.info.postal_code_regex.clone(),
                ),
                "languages" => {
                    countryrecord_map.insert("languages", countryrecord.info.languages.clone())
                },
                "geonameid" => {
                    countryrecord_map.insert("geonameid", countryrecord.info.geonameid.to_string())
                },
                "neighbours" => {
                    countryrecord_map.insert("neighbours", countryrecord.info.neighbours.clone())
                },
                "equivalent_fips_code" => countryrecord_map.insert(
                    "equivalent_fips_code",
                    countryrecord.info.equivalent_fips_code.clone(),
                ),
                _ => return Some(INVALID_DYNFMT.to_string()),
            };
        }

        if let Ok(formatted) = dynfmt::SimpleCurlyFormat.format(formatstr, countryrecord_map) {
            Some(formatted.to_string())
        } else {
            Some(INVALID_DYNFMT.to_string())
        }
    }
}

/// get_cityrecord_name_in_lang is a cached function that returns a NamesLang struct
/// containing the city, admin1, admin2, and country names in the specified language.
/// Note that the index file needs to be built with the desired languages for this to work.
/// Use the "index-update" subcommand with the --languages option to rebuild the index
/// with the desired languages. Otherwise, all names will be in English (en)
#[cached(
    key = "String",
    convert = r#"{ format!("{}-{}", cityrecord.id, lang_lookup) }"#
)]
fn get_cityrecord_name_in_lang(cityrecord: &CitiesRecord, lang_lookup: &str) -> NamesLang {
    let cityname = cityrecord
        .names
        .clone()
        .unwrap_or_default()
        .get(lang_lookup)
        .cloned()
        // Note that the city name is the default name if the language is not found.
        .unwrap_or_else(|| cityrecord.name.clone());
    let admin1name = cityrecord
        .admin1_names
        .clone()
        .unwrap_or_default()
        .get(lang_lookup)
        .cloned()
        .unwrap_or_default();
    let admin2name = cityrecord
        .admin2_names
        .clone()
        .unwrap_or_default()
        .get(lang_lookup)
        .cloned()
        .unwrap_or_default();
    let countryname = cityrecord
        .country_names
        .clone()
        .unwrap_or_default()
        .get(lang_lookup)
        .cloned()
        .unwrap_or_default();

    NamesLang {
        cityname,
        admin1name,
        admin2name,
        countryname,
    }
}

#[inline]
fn lookup_us_state_fips_code(state: &str) -> Option<&str> {
    if let Ok(i) = US_STATES_FIPS_CODES.binary_search_by_key(&state, |&(abbrev, _)| abbrev) {
        Some(US_STATES_FIPS_CODES[i].1)
    } else {
        None
    }
}
