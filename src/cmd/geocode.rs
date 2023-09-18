static USAGE: &str = r#"
Geocodes a location in CSV data against an updatable local copy of the Geonames cities index.

When you run the command for the first time, it will download a prebuilt Geonames cities
index from the qsv GitHub repo and use it going forward. You can operate on the local
index using the index-* subcommands.

By default, the prebuilt index uses the Geonames Gazeteer cities15000.zip file using
English names. It contains cities with populations > 15,000 (about ~26k cities). 
See https://download.geonames.org/export/dump/ for more information.

It has five major subcommands:
 * suggest     - given a partial City name, return the closest City's location metadata
                 per the local Geonames cities index (Jaro-Winkler distance)
 * suggestnow  - same as suggest, but using a City name from the command line,
                 instead of CSV data.
 * reverse     - given a location coordinate, return the closest City's location metadata
                 per the local Geonames cities index.
                 (Euclidean distance - shortest distance "as the crow flies")
 * reversenow  - sames as reverse, but using a coordinate from the command line,
                 instead of CSV data.
 * countryinfo - returns the country information for the specified country code.
                 (e.g. US, CA, MX, etc.)
 * index-*     - operations to update the local Geonames cities index.
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
Returns the country information for the specified country code.
(e.g. US, CA, MX, etc.)

  $ qsv geocode countryinfo US

INDEX-<operation>
Updates the local Geonames cities index used by the geocode command.

It has four operations:
 * check  - checks if the local Geonames index is up-to-date compared to the Geonames website.
 * update - updates the local Geonames index with the latest changes from the Geonames website.
            use this command judiciously as it downloads about ~200mb of data from Geonames
            and rebuilds the index from scratch using the --languages option.
 * reset  - resets the local Geonames index to the default prebuilt, English language Geonames
            cities index - downloading it from the qsv GitHub repo for that release.
 * load   - load a Geonames cities index from a file, making it the default index going forward.

Examples:
Update the Geonames cities index with the latest changes.

  $ qsv geocode index-update

Load a Geonames cities index from a file.

  $ qsv geocode index-load my_geonames_index.bincode

For more extensive examples, see https://github.com/jqnatividad/qsv/blob/master/tests/test_geocode.rs.

Usage:
qsv geocode suggest [--formatstr=<string>] [options] <column> [<input>]
qsv geocode suggestnow [options] <location>
qsv geocode reverse [--formatstr=<string>] [options] <column> [<input>]
qsv geocode reversenow [options] <location>
qsv geocode countryinfo <country> [options]
qsv geocode index-load <index-file>
qsv geocode index-check
qsv geocode index-update [--languages=<lang>] [--cities-url=<url>] [--force]
qsv geocode index-reset
qsv geocode --help

geocode arguments:
        
    <input>                     The input file to read from. If not specified, reads from stdin.
    <column>                    The column to geocode.
    <location>                  The location to geocode. For suggestnow, its a City string pattern.
                                For reversenow, it must be a WGS 84 coordinate "lat, long" or
                                "(lat, long)" format.
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

    -f, --formatstr=<string>    The place format to use. The predefined formats are:
                                  - '%city-state' - e.g. Brooklyn, New York
                                  - '%city-country' - Brooklyn, US
                                  - '%city-state-country' | '%city-admin1-country' - Brooklyn, New York US
                                  - '%city-county-state' | '%city-admin2-admin1' - Brooklyn, Kings County, New York
                                  - '%city' - Brooklyn
                                  - '%state' | '%admin1' - New York
                                  - "%county' | '%admin2' - Kings County
                                  - '%country' - US
                                  - '%cityrecord' - returns the full city record as a string
                                  - '%admin1record' - returns the full admin1 record as a string
                                  - '%admin2record' - returns the full admin2 record as a string
                                  - '%lat-long' - <latitude>, <longitude>
                                  - '%location' - (<latitude>, <longitude>)
                                  - '%id' - the Geonames ID
                                  - '%capital' - the capital
                                  - '%population' - the population
                                  - '%timezone' - the timezone
                                  - '%+' - use the subcommand's default format. 
                                           suggest - '%location'
                                           suggestnow - '{name}, {admin1} {country}: {latitude}, {longitude}'
                                           reverse & reversenow - '%city-admin1-country'
                                
                                If an invalid format is specified, it will be treated as '%+'.

                                Alternatively, you can use dynamic formatting to create a custom format.
                                To do so, set the --formatstr to a dynfmt template, enclosing field names
                                in curly braces.
                                The following ten fields are available:
                                  id, name, latitude, longitude, country, admin1, admin2, capital,
                                  timezone, population
                                    
                                  e.g. "City: {name}, State: {admin1}, Country: {country} - {timezone}"

                                If an invalid template is specified, "Invalid dynfmt template" is returned.

                                Both predefined and dynamic formatting are cached. Subsequent calls
                                with the same result will be faster as it will use the cached result instead
                                of searching the Geonames index.

                                Finally, you can use the special format "%dyncols:" to dynamically add multiple
                                columns to the output CSV for each field in a geocode result.
                                To do so, set --formatstr to "%dyncols:" followed by a comma-delimited list
                                of key:value pairs enclosed in curly braces.
                                The key is the desired column name and the value is one of the same ten fields
                                available for dynamic formatting.

                                 e.g. "%dyncols: {city_col:name}, {state_col:admin1}, {country_col:country}"

                                will add three columns to the output CSV named city_col, state_col & country_col.

                                Note that using "%dyncols:" will cause the the command to geocode EACH row without
                                using the cache, so it will be slower than predefined or dynamic formatting.
                                [default: %+]

    --invalid-result <string>   The string to return when the geocode result is empty/invalid.
                                If not set, the original value is used.
    -j, --jobs <arg>            The number of jobs to run in parallel.
                                When not set, the number of jobs is set to the number of CPUs detected.
    -b, --batch <size>          The number of rows per batch to load into memory, before running in parallel.
                                [default: 50000]
    --timeout <seconds>         Timeout for downloading Geonames cities index.
                                [default: 60]
    --cache-dir <dir>           The directory to use for caching the Geonames cities index.
                                If the directory does not exist, qsv will attempt to create it.
                                If the QSV_CACHE_DIR envvar is set, it will be used instead.
                                [default: ~/.qsv-cache]                                

                                INDEX-UPDATE only options:
    --languages <lang>          The comma-delimited, case-insentive list of languages to use when building
                                the Geonames cities index.
                                The languages are specified as a comma-separated list of ISO 639-1 codes.
                                https://en.wikipedia.org/wiki/List_of_ISO_639-1_codes
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
    -p, --progressbar           Show progress bars. Not valid for stdin.
"#;

use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use cached::{proc_macro::cached, SizedCache};
use dynfmt::Format;
use geosuggest_core::{CitiesRecord, Engine, EngineDumpFormat};
use geosuggest_utils::{IndexUpdater, IndexUpdaterSettings, SourceItem};
use indicatif::{ProgressBar, ProgressDrawTarget};
use log::info;
use rayon::{
    iter::{IndexedParallelIterator, ParallelIterator},
    prelude::IntoParallelRefIterator,
};
use regex::Regex;
use serde::Deserialize;
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

// max number of entries in LRU cache
static CACHE_SIZE: usize = 2_000_000;
// max number of entries in fallback LRU cache if we can't allocate CACHE_SIZE
static FALLBACK_CACHE_SIZE: usize = CACHE_SIZE / 4;

static EMPTY_STRING: String = String::new();
static INVALID_DYNFMT: &str = "Invalid dynfmt template.";
static INVALID_COUNTRY_CODE: &str = "Invalid country code.";

// when suggesting with --admin1, how many suggestions to fetch from the engine
// before filtering by admin1
static SUGGEST_ADMIN1_LIMIT: usize = 10;

// valid column values for %dyncols
static VALID_DYNCOLS: [&str; 10] = [
    "id",
    "name",
    "latitude",
    "longitude",
    "country",
    "admin1",
    "admin2",
    "capital",
    "timezone",
    "population",
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
    let mut index_cmd = true;
    let mut now_cmd = false;
    let geocode_cmd = if args.cmd_suggest {
        index_cmd = false;
        GeocodeSubCmd::Suggest
    } else if args.cmd_reverse {
        index_cmd = false;
        GeocodeSubCmd::Reverse
    } else if args.cmd_suggestnow {
        index_cmd = false;
        now_cmd = true;
        GeocodeSubCmd::SuggestNow
    } else if args.cmd_reversenow {
        index_cmd = false;
        now_cmd = true;
        GeocodeSubCmd::ReverseNow
    } else if args.cmd_countryinfo {
        index_cmd = false;
        GeocodeSubCmd::CountryInfo
    } else if args.cmd_index_check {
        GeocodeSubCmd::IndexCheck
    } else if args.cmd_index_update {
        GeocodeSubCmd::IndexUpdate
    } else if args.cmd_index_load {
        GeocodeSubCmd::IndexLoad
    } else if args.cmd_index_reset {
        GeocodeSubCmd::IndexReset
    } else {
        // should not happen as docopt won't allow it
        unreachable!();
    };

    // setup cache directory
    let mut geocode_cache_dir = if let Ok(cache_dir) = std::env::var("QSV_CACHE_DIR") {
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
    let geocode_index_file = args.arg_index_file.clone().unwrap_or_else(|| {
        geocode_cache_dir.push(geocode_index_filename);
        geocode_cache_dir.to_string_lossy().to_string()
    });

    // create a TempDir for the one record CSV we're creating if we're doing a Now command
    // we're doing this at this scope so the TempDir is automatically dropped after we're done
    let tempdir = tempfile::Builder::new()
        .prefix("qsv-geocode")
        .tempdir()
        .unwrap();

    // we're doing a SuggestNow or ReverseNow, create a one record CSV in tempdir
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

        let updater = IndexUpdater::new(IndexUpdaterSettings {
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
        })?;

        match geocode_cmd {
            GeocodeSubCmd::IndexCheck => {
                // check if we have updates
                winfo!("Checking main Geonames website for updates...");
                check_index_file(&geocode_index_file)?;
                let engine = load_engine(geocode_index_file.clone().into(), &progress).await?;

                if updater.has_updates(&engine).await? {
                    winfo!(
                        "Updates available at Geonames.org. Use `qsv geocode index-update` to \
                         update/rebuild the index.\nPlease use this judiciously as Geonames is a \
                         free service."
                    );
                } else {
                    winfo!("Geonames index up-to-date.");
                }
            },
            GeocodeSubCmd::IndexUpdate => {
                // update/rebuild Geonames index from Geonames website
                // will only update if there are changes
                check_index_file(&geocode_index_file)?;
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
                    engine.dump_to(geocode_index_file.clone(), EngineDumpFormat::Bincode)?;
                    winfo!("Geonames index built: {geocode_index_file}");
                } else {
                    winfo!("Checking main Geonames website for updates...");

                    let engine = load_engine(geocode_index_file.clone().into(), &progress).await?;
                    if updater.has_updates(&engine).await? {
                        winfo!(
                            "Updating/Rebuilding Geonames index. This will take a while as we \
                             need to download data from Geonames & rebuild the index..."
                        );
                        let engine = updater.build().await?;
                        engine.dump_to(geocode_index_file.clone(), EngineDumpFormat::Bincode)?;
                        winfo!("Updates applied: {geocode_index_file}");
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
                    engine.dump_to(geocode_index_file.clone(), EngineDumpFormat::Bincode)?;
                    winfo!(
                        "Valid Geonames index file {index_file} copied to {geocode_index_file}. \
                         It will be used from now on or until you reset/rebuild it.",
                    );
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
                if Path::new(&geocode_index_file).exists() {
                    fs::remove_file(&geocode_index_file)?;
                }
                // if there's no index file, load_engine will download the default geocode index
                // from the qsv GitHub repo
                let _ = load_engine(geocode_index_file.clone().into(), &progress).await?;
                winfo!("Default Geonames index file reset to {QSV_VERSION} release.");
            },
            // index_cmd is true, so we should never get a non-index subcommand
            _ => unreachable!(),
        }
        return Ok(());
    }

    // we're not doing an index subcommand, so we're doing a suggest/now or reverse/now
    // load the current local Geonames index
    let engine = load_engine(geocode_index_file.clone().into(), &progress).await?;

    let mut rdr = rconfig.reader()?;
    let mut wtr = Config::new(&args.flag_output).writer()?;

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
        // the valid column values are in VALID_DYNCOLS
        for column_value in &column_values {
            if !VALID_DYNCOLS.contains(column_value) {
                return fail_incorrectusage_clierror!(
                    "Invalid column value: {column_value}. Valid values are: {VALID_DYNCOLS:?}"
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

    // now, write the headers to the output CSV
    wtr.write_record(&headers)?;

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
            // reverse/now and countryinfo subcommands don't support admin1 filter
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
                } else if geocode_cmd == GeocodeSubCmd::CountryInfo {
                    // we're doing a countryinfo subcommand

                    cell =
                        get_countryinfo(&engine, &cell.to_ascii_uppercase(), &args.flag_formatstr)
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

// check if index_file exists and ends with a .bincode extension
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

async fn load_engine(geocode_index_file: PathBuf, progressbar: &ProgressBar) -> CliResult<Engine> {
    let index_file = std::path::Path::new(&geocode_index_file);

    if index_file.exists() {
        // load existing local index
        progressbar.println(format!(
            "Loading existing Geonames index from {}",
            index_file.display()
        ));
    } else {
        // initial load, download index file from qsv releases
        progressbar.println(format!(
            "Downloading default Geonames index for qsv {QSV_VERSION} release..."
        ));

        util::download_file(
            &format!(
                "https://github.com/jqnatividad/qsv/releases/download/{QSV_VERSION}/qsv-{QSV_VERSION}-geocode-index.bincode"
            ),
            geocode_index_file.clone(),
            !progressbar.is_hidden(),
            None,
            None,
            None,
        )
        .await?;
    }
    let engine = Engine::load_from(index_file, EngineDumpFormat::Bincode)
        .map_err(|e| format!("On load index file: {e}"))?;
    Ok(engine)
}

/// search_index is a cached function that returns a geocode result for a given cell value.
/// It uses an LRU cache using the cell value as the key, storing the formatted geocoded result
/// in the cache. As such, we CANNOT use the cache when in dyncols mode as the cached result is
/// the formatted result, not the individual fields.
/// search_index_no_cache() is automatically derived from search_index() by the cached macro.
/// search_index_no_cache() is used in dyncols mode, and as the name implies, does not use a cache.
#[cached(
    type = "SizedCache<String, String>",
    create = "{ SizedCache::try_with_size(CACHE_SIZE).unwrap_or_else(|_| \
              SizedCache::with_size(FALLBACK_CACHE_SIZE)) }",
    key = "String",
    convert = r#"{ format!("{cell}") }"#,
    option = true
)]
fn search_index(
    engine: &Engine,
    mode: GeocodeSubCmd,
    cell: &str,
    formatstr: &str,
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
                let mut admin1_filter_map: HashMap<String, bool> = HashMap::new();
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

        if formatstr == "%+" {
            // default for suggest is location - e.g. "(lat, long)"
            if mode == GeocodeSubCmd::SuggestNow {
                // however, make SuggestNow default more verbose
                return Some(format!(
                    "{name}, {admin1} {country}: {latitude}, {longitude}",
                    name = cityrecord.name.clone(),
                    admin1 = get_admin_names(cityrecord, 1).0,
                    country = country,
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
            add_dyncols(record, cityrecord, &country, &capital, column_values);
            return Some(DYNCOLS_POPULATED.to_string());
        }

        return Some(format_result(
            engine, cityrecord, &country, &capital, formatstr, true,
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

            let country = cityrecord.country.clone().unwrap().code;

            if formatstr == "%+" {
                // default for reverse is city, admin1 country - e.g. "Brooklyn, New York US"
                let (admin1_name, _admin2_name) = get_admin_names(cityrecord, 1);

                return Some(format!(
                    "{city}, {admin1} {country}",
                    city = cityrecord.name.clone(),
                    admin1 = admin1_name.clone()
                ));
            }

            let capital = engine
                .capital(&country)
                .map(|cr| cr.name.clone())
                .unwrap_or_default();

            if formatstr.starts_with("%dyncols:") {
                add_dyncols(record, cityrecord, &country, &capital, column_values);
                return Some(DYNCOLS_POPULATED.to_string());
            }

            return Some(format_result(
                engine, cityrecord, &country, &capital, formatstr, false,
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
    country: &str,
    capital: &str,
    column_values: &[&str],
) {
    for column in column_values {
        match *column {
            "id" => record.push_field(&cityrecord.id.to_string()),
            "name" => record.push_field(&cityrecord.name),
            "latitude" => record.push_field(&cityrecord.latitude.to_string()),
            "longitude" => record.push_field(&cityrecord.longitude.to_string()),
            "country" => record.push_field(country),
            "admin1" => {
                let (admin1_name, _) = get_admin_names(cityrecord, 1);
                record.push_field(admin1_name);
            },
            "admin2" => {
                let (_, admin2_name) = get_admin_names(cityrecord, 2);
                record.push_field(admin2_name);
            },
            "capital" => record.push_field(capital),
            "timezone" => record.push_field(&cityrecord.timezone),
            "population" => record.push_field(&cityrecord.population.to_string()),
            // this should not happen as column_values has been pre-validated for these values
            _ => unreachable!(),
        }
    }
}

/// format the geocoded result based on formatstr if its not %+
fn format_result(
    engine: &Engine,
    cityrecord: &CitiesRecord,
    country: &str,
    capital: &str,
    formatstr: &str,
    suggest_mode: bool,
) -> String {
    let (admin1_name, admin2_name) = get_admin_names(cityrecord, 3);

    let Some(countryrecord) = engine.country_info(country) else {
        return INVALID_COUNTRY_CODE.to_string();
    };

    if formatstr.starts_with('%') {
        // if formatstr starts with %, then we're using a predefined format
        match formatstr {
            "%city-state" | "%city-admin1" => format!("{}, {}", cityrecord.name, admin1_name),
            "%location" => format!("({}, {})", cityrecord.latitude, cityrecord.longitude),
            "%city-state-country" | "%city-admin1-country" => {
                format!("{}, {} {}", cityrecord.name, admin1_name, country)
            },
            "%lat-long" => format!("{}, {}", cityrecord.latitude, cityrecord.longitude),
            "%city-country" => format!("{}, {}", cityrecord.name, country),
            "%city" => cityrecord.name.clone(),
            "%city-county-state" | "%city-admin2-admin1" => {
                format!("{}, {}, {}", cityrecord.name, admin2_name, admin1_name,)
            },
            "%state" | "%admin1" => admin1_name.to_owned(),
            "%county" | "%admin2" => admin2_name.to_owned(),
            "%country" => country.to_owned(),
            "%id" => format!("{}", cityrecord.id),
            "%capital" => capital.to_owned(),
            "%population" => format!("{}", cityrecord.population),
            "%timezone" => cityrecord.timezone.clone(),
            "%cityrecord" => format!("{cityrecord:?}"),
            "%admin1record" => format!("{:?}", cityrecord.admin_division),
            "%admin2record" => format!("{:?}", cityrecord.admin2_division),

            // countryrecord fields
            "%continent" => countryrecord.info.continent.clone(),

            // "%json" => format!(
            //     "{{\"id\":{},\"name\":\"{}\",\"latitude\":{},\"longitude\":{},\"country\":\"{}\",
            // \\      "admin1\":\"{}\",\"admin2\":\"{}\",\"capital\":\"{}\",\"timezone\
            // ":\"{}\",\"\      population\":{}}}",
            //     cityrecord.id,
            //     cityrecord.name,
            //     cityrecord.latitude,
            //     cityrecord.longitude,
            //     country,
            //     admin1_name,
            //     admin2_name,
            //     capital,
            //     cityrecord.timezone,
            //     cityrecord.population,
            // ),
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
                        city = cityrecord.name,
                        admin1 = admin1_name,
                    )
                }
            },
        }
    } else {
        // if formatstr does not start with %, then we're using dynfmt,
        // i.e. twenty-five predefined fields below in curly braces are replaced with values
        // e.g. "City: {name}, State: {admin1}, Country: {country} - {timezone}"
        let mut cityrecord_map: HashMap<&str, String> = HashMap::with_capacity(25);
        cityrecord_map.insert("id", cityrecord.id.to_string());
        cityrecord_map.insert("name", cityrecord.name.clone());
        cityrecord_map.insert("latitude", cityrecord.latitude.to_string());
        cityrecord_map.insert("longitude", cityrecord.longitude.to_string());
        cityrecord_map.insert("country", country.to_owned());
        cityrecord_map.insert("admin1", admin1_name.to_owned());
        cityrecord_map.insert("admin2", admin2_name.to_owned());
        cityrecord_map.insert("capital", capital.to_owned());
        cityrecord_map.insert("timezone", cityrecord.timezone.clone());
        cityrecord_map.insert("population", cityrecord.population.to_string());

        // countryrecord fields
        cityrecord_map.insert("iso3", countryrecord.info.iso3.clone());
        cityrecord_map.insert("fips", countryrecord.info.fips.clone());
        cityrecord_map.insert("area", countryrecord.info.area.to_string());
        cityrecord_map.insert(
            "country_population",
            countryrecord.info.population.to_string(),
        );
        cityrecord_map.insert("continent", countryrecord.info.continent.clone());
        cityrecord_map.insert("tld", countryrecord.info.tld.clone());
        cityrecord_map.insert("currency_code", countryrecord.info.currency_code.clone());
        cityrecord_map.insert("currency_name", countryrecord.info.currency_name.clone());
        cityrecord_map.insert("phone", countryrecord.info.phone.clone());
        cityrecord_map.insert(
            "postal_code_format",
            countryrecord.info.postal_code_format.clone(),
        );
        cityrecord_map.insert(
            "postal_code_regex",
            countryrecord.info.postal_code_regex.clone(),
        );
        cityrecord_map.insert("languages", countryrecord.info.languages.clone());
        cityrecord_map.insert("country_geonameid", countryrecord.info.geonameid.to_string());
        cityrecord_map.insert("neighbours", countryrecord.info.neighbours.clone());
        cityrecord_map.insert(
            "equivalent_fips_code",
            countryrecord.info.equivalent_fips_code.clone(),
        );

        if let Ok(formatted) = dynfmt::SimpleCurlyFormat.format(formatstr, cityrecord_map) {
            formatted.to_string()
        } else {
            INVALID_DYNFMT.to_string()
        }
    }
}

/// get admin1 and admin2 names based on selector
/// selector = 0: none; selector = 1: admin1 only
/// selector = 2: admin2 only; selector >= 3: admin1 and admin2
fn get_admin_names(cityrecord: &CitiesRecord, selector: u8) -> (&String, &String) {
    let (_admin1_key, admin1_name) = if selector == 1 || selector >= 3 {
        match &cityrecord.admin1_names {
            Some(admin1) => admin1
                .iter()
                .next()
                .unwrap_or((&EMPTY_STRING, &EMPTY_STRING)),
            None => (&EMPTY_STRING, &EMPTY_STRING),
        }
    } else {
        (&EMPTY_STRING, &EMPTY_STRING)
    };

    let (_admin2_key, admin2_name) = if selector == 2 || selector >= 3 {
        match &cityrecord.admin2_names {
            Some(admin2) => admin2
                .iter()
                .next()
                .unwrap_or((&EMPTY_STRING, &EMPTY_STRING)),
            None => (&EMPTY_STRING, &EMPTY_STRING),
        }
    } else {
        (&EMPTY_STRING, &EMPTY_STRING)
    };
    (admin1_name, admin2_name)
}

#[cached(
    key = "String",
    convert = r#"{ format!("{cell}-{formatstr}") }"#,
    option = true
)]
fn get_countryinfo(engine: &Engine, cell: &str, formatstr: &str) -> Option<String> {
    let Some(countryrecord) = engine.country_info(cell) else {
        // no results, so return early with None
        return None;
    };

    if formatstr.starts_with('%') {
        // if formatstr starts with %, then we're using a predefined format
        let formatted = match formatstr {
            "%capital" => countryrecord.info.capital.clone(),
            "%continent" => countryrecord.info.continent.clone(),
            _ => format!("{:?}", countryrecord.names), /* default is to return all names
                                                        * "%+" => format!("{:?}",
                                                        * countryrecord.names), */
        };
        Some(formatted)
    } else {
        // if formatstr does not start with %, then we're using dynfmt,
        // i.e. sixteen predefined fields below in curly braces are replaced with values
        // e.g. "Country name/s: {name}, Continent: {continent} Currency: {currency_name}
        // ({currency_code})})"
        let mut countryrecord_map: HashMap<&str, String> = HashMap::with_capacity(16);
        countryrecord_map.insert("iso3", countryrecord.info.iso3.clone());
        countryrecord_map.insert("fips", countryrecord.info.fips.clone());
        countryrecord_map.insert("capital", countryrecord.info.capital.clone());
        countryrecord_map.insert("area", countryrecord.info.area.to_string());
        countryrecord_map.insert(
            "country_population",
            countryrecord.info.population.to_string(),
        );
        countryrecord_map.insert("continent", countryrecord.info.continent.clone());
        countryrecord_map.insert("tld", countryrecord.info.tld.clone());
        countryrecord_map.insert("currency_code", countryrecord.info.currency_code.clone());
        countryrecord_map.insert("currency_name", countryrecord.info.currency_name.clone());
        countryrecord_map.insert("phone", countryrecord.info.phone.clone());
        countryrecord_map.insert(
            "postal_code_format",
            countryrecord.info.postal_code_format.clone(),
        );
        countryrecord_map.insert(
            "postal_code_regex",
            countryrecord.info.postal_code_regex.clone(),
        );
        countryrecord_map.insert("languages", countryrecord.info.languages.clone());
        countryrecord_map.insert("geonameid", countryrecord.info.geonameid.to_string());
        countryrecord_map.insert("neighbours", countryrecord.info.neighbours.clone());
        countryrecord_map.insert(
            "equivalent_fips_code",
            countryrecord.info.equivalent_fips_code.clone(),
        );

        if let Ok(formatted) = dynfmt::SimpleCurlyFormat.format(formatstr, countryrecord_map) {
            Some(formatted.to_string())
        } else {
            Some(INVALID_DYNFMT.to_string())
        }
    }
}
