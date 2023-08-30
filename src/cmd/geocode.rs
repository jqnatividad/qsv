static USAGE: &str = r#"
Geocodes a location against an updatable local copy of the Geonames cities index.

By default, it uses the Geonames Gazeteer cities15000.zip file. It contains cities with
populations > 15,000 (about ~26k cities). 
See https://download.geonames.org/export/dump/ for more information.

It has three major subcommands:
 * suggest - given a City name, return the closest location coordinate by default.
 * reverse - given a location coordinate, return the closest City by default.
 * index-* - operations to update the local Geonames cities index.
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

Geocode file.csv city column with --formatstr=%state and set the 
geocoded value a new column named state.

$ qsv geocode suggest city --formatstr %state --new-column state file.csv

Use dynamic formatting to create a custom format.

$ qsv geocode suggest city --formatstr "{name}, {admin1}, {country} in {timezone}" file.csv

REVERSE
Reverse geocode a WGS 84 coordinate to the nearest Geonames city record.
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

$ qsv geocode reverse LatLong --formatstr %timezone -c tz file.csv -o file_with_tz.csv

INDEX-<operation>
Updates the local Geonames cities index used by the geocode command.

It has four operations:
 * check  - checks if the local Geonames index is up-to-date compared to the Geonames website.
 * update - updates the local Geonames index with the latest changes from the Geonames website.
            use this command judiciously as it downloads about ~200mb of data from Geonames
            and rebuilds the index from scratch using the --languages option.
 * reset  - resets the local Geonames index to the default Geonames cities index, downloading
            it from the qsv GitHub repo for that release.
 * load   - load a Geonames cities index from a file, making it the default index going forward.

Examples:
Update the Geonames cities index with the latest changes.

$ qsv geocode index-update

Load a Geonames cities index from a file.

$ qsv geocode index-load my_geonames_index.bincode

For more extensive examples, see https://github.com/jqnatividad/qsv/blob/master/tests/test_geocode.rs.

Usage:
qsv geocode suggest [--formatstr=<string>] [options] <column> [<input>]
qsv geocode reverse [--formatstr=<string>] [options] <column> [<input>]
qsv geocode index-load <index-file>
qsv geocode index-check
qsv geocode index-update
qsv geocode index-reset
qsv geocode --help

geocode arguments:
        
    <input>                     The input file to read from. If not specified, reads from stdin.
    <column>                    The column to geocode.
    <index-file>                The alternate geonames index file to use. It must be a .bincode file.
                                Only used by the 'load' operations.

geocode options:
    -c, --new-column <name>     Put the transformed values in a new column instead.
    -r, --rename <name>         New name for the transformed column.
    --min-score <score>         The minimum score to use for suggest subcommand.
                                [default: 0.8]
    -k, --k_weight <weight>     Use population-weighted distance for reverse subcommand.
                                (i.e. nearest.distance - k * city.population)
                                Larger values will favor more populated cities.
                                If not set (default), the population is not used and the
                                nearest city is returned.
    -f, --formatstr=<string>    The place format to use. The predefined formats are:
                                  - '%city-state' - e.g. Brooklyn, New York
                                  - '%city-country' - Brooklyn, US
                                  - '%city-state-country' | '%city-admin1-country' - Brooklyn, New York US
                                  - '%city' - Brooklyn
                                  - '%state' | '%admin1' - New York
                                  - '%country' - US
                                  - '%cityrecord' - returns the full city record as a string
                                  - '%lat-long' - <latitude>, <longitude>
                                  - '%location' - (<latitude>, <longitude>)
                                  - '%id' - the Geonames ID
                                  - '%population' - the population
                                  - '%timezone' - the timezone
                                  - '%+' - use the subcommand's default format. For suggest, '%location'.
                                           For reverse, '%city-admin1'.
                                
                                If an invalid format is specified, it will be treated as '%+'.

                                Alternatively, you can use dynamic formatting to create a custom format.
                                To do so, set the --formatstr to a dynfmt template, enclosing field names
                                in curly braces.
                                The following eight fields are available:
                                  id, name, latitude, longitude, country, admin1, timezone, population
                                    
                                  e.g. "City: {name}, State: {admin1}, Country: {country} - {timezone}"

                                If an invalid dynfmt template is specified, it will return "Invalid dynfmt template."
                                [default: %+]

    --invalid-result <string>   The string to use when the geocode result is empty/invalid.
                                If not set, the original value is used.
    -j, --jobs <arg>            The number of jobs to run in parallel.
                                When not set, the number of jobs is set to the number of CPUs detected.
    -b, --batch <size>          The number of rows per batch to load into memory, before running in parallel.
                                [default: 50000]
    --timeout <seconds>         Timeout for downloading Geonames cities index.
                                [default: 60]
    --languages <lang>          The languages to use when building the Geonames cities index.
                                Only used by the 'index-update' subcommand.
                                The languages are specified as a comma-separated list of ISO 639-1 codes.
                                [default: en]
    --cache-dir <dir>           The directory to use for caching the Geonames cities index.
                                If the directory does not exist, qsv will attempt to create it.
                                If the QSV_CACHE_DIR envvar is set, it will be used instead.
                                [default: ~/.qsv-cache]

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

use cached::proc_macro::cached;
use dynfmt::Format;
use geosuggest_core::{CitiesRecord, Engine, EngineDumpFormat};
use geosuggest_utils::{IndexUpdater, IndexUpdaterSettings, SourceItem};
use indicatif::{ProgressBar, ProgressDrawTarget};
use log::{debug, info};
use rayon::{
    iter::{IndexedParallelIterator, ParallelIterator},
    prelude::IntoParallelRefIterator,
};
use regex::Regex;
use serde::Deserialize;
use simple_home_dir::expand_tilde;

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
    cmd_suggest:         bool,
    cmd_reverse:         bool,
    cmd_index_check:     bool,
    cmd_index_update:    bool,
    cmd_index_load:      bool,
    cmd_index_reset:     bool,
    arg_input:           Option<String>,
    arg_index_file:      Option<String>,
    flag_rename:         Option<String>,
    flag_min_score:      Option<f32>,
    flag_k_weight:       Option<f32>,
    flag_formatstr:      String,
    flag_invalid_result: Option<String>,
    flag_batch:          u32,
    flag_timeout:        u16,
    flag_languages:      String,
    flag_cache_dir:      String,
    flag_jobs:           Option<usize>,
    flag_new_column:     Option<String>,
    flag_output:         Option<String>,
    flag_delimiter:      Option<Delimiter>,
    flag_progressbar:    bool,
}

static QSV_VERSION: &str = env!("CARGO_PKG_VERSION");
static DEFAULT_GEOCODE_INDEX_FILENAME: &str =
    concat!("qsv-", env!("CARGO_PKG_VERSION"), "-geocode-index.bincode");

static DEFAULT_CITIES_DB_URL: &str = "https://download.geonames.org/export/dump/cities15000.zip";
static DEFAULT_CITIES_DB_FILENAME: &str = "cities15000.txt";
static DEFAULT_CITIES_NAMES_URL: &str =
    "https://download.geonames.org/export/dump/alternateNamesV2.zip";
static DEFAULT_CITIES_NAMES_FILENAME: &str = "alternateNamesV2.txt";
static DEFAULT_COUNTRY_INFO_URL: &str = "https://download.geonames.org/export/dump/countryInfo.txt";
static DEFAULT_ADMIN1_CODES_URL: &str =
    "https://download.geonames.org/export/dump/admin1CodesASCII.txt";

static EMPTY_STRING: String = String::new();
static INVALID_DYNFMT: &str = "Invalid dynfmt template.";

// valid subcommands
#[derive(Clone, Copy, PartialEq)]
enum GeocodeSubCmd {
    Suggest,
    Reverse,
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

    // we need to use tokio runtime as geosuggest uses async
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(geocode_main(args))?;

    Ok(())
}

// main async geocode function that does the actual work
async fn geocode_main(args: Args) -> CliResult<()> {
    let mut index_cmd = true;
    let geocode_cmd = if args.cmd_suggest {
        index_cmd = false;
        GeocodeSubCmd::Suggest
    } else if args.cmd_reverse {
        index_cmd = false;
        GeocodeSubCmd::Reverse
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
        unreachable!("No geocode subcommand specified.");
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

    // setup languages
    let languages_string_vec = args
        .flag_languages
        .to_ascii_lowercase()
        .split(',')
        .map(|s| s.trim().to_string())
        .collect::<Vec<String>>();
    let languages_vec: Vec<&str> = languages_string_vec
        .iter()
        .map(std::string::String::as_str)
        .collect();

    debug!("geocode_index_file: {geocode_index_file} Languages: {languages_vec:?}");

    let updater = IndexUpdater::new(IndexUpdaterSettings {
        http_timeout_ms:  util::timeout_secs(args.flag_timeout)? * 1000,
        cities:           SourceItem {
            url:      DEFAULT_CITIES_DB_URL,
            filename: DEFAULT_CITIES_DB_FILENAME,
        },
        names:            Some(SourceItem {
            url:      DEFAULT_CITIES_NAMES_URL,
            filename: DEFAULT_CITIES_NAMES_FILENAME,
        }),
        countries_url:    Some(DEFAULT_COUNTRY_INFO_URL),
        admin1_codes_url: Some(DEFAULT_ADMIN1_CODES_URL),
        filter_languages: languages_vec.clone(),
    })?;

    let rconfig = Config::new(&args.arg_input)
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
                let engine = load_engine(geocode_index_file.clone().into(), &progress).await?;
                if updater.has_updates(&engine).await? {
                    winfo!(
                        "Updating/Rebuilding Geonames index. This will take a while as we need to \
                         download ~200mb of data from Geonames and rebuild the index..."
                    );
                    let engine = updater.build().await?;
                    engine.dump_to(geocode_index_file.clone(), EngineDumpFormat::Bincode)?;
                    winfo!("Updates applied: {geocode_index_file}");
                } else {
                    winfo!("Skipping update. Geonames index is up-to-date.");
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
                         It will be used from now on or until you reset it.",
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
            _ => unreachable!("index_cmd is true, so this is unreachable."),
        }
        return Ok(());
    }

    // we're not doing an index subcommand, so we're doing a suggest or reverse
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

    if let Some(new_column) = &args.flag_new_column {
        headers.push_field(new_column);
    }
    wtr.write_record(&headers)?;

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
                if !cell.is_empty() {
                    let search_result = search_cached(
                        &engine,
                        geocode_cmd,
                        &cell,
                        &args.flag_formatstr,
                        min_score,
                        k_weight,
                    );
                    if let Some(geocoded_result) = search_result {
                        // we have a valid geocode result, so use that
                        cell = geocoded_result;
                    } else {
                        // we have an invalid geocode result
                        if !invalid_result.is_empty() {
                            // --invalid-result is set, so use that instead
                            // otherwise, we leave cell untouched, and the original value remains
                            cell = invalid_result.clone();
                        }
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

        if show_progress {
            progress.inc(batch.len() as u64);
        }

        batch.clear();
    } // end batch loop

    if show_progress {
        util::update_cache_info!(progress, SEARCH_CACHED);
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
        if !progressbar.is_hidden() {
            progressbar.println(format!(
                "Loading existing Geonames index from {}",
                index_file.display()
            ));
        }
    } else {
        // initial load, download index file from qsv releases
        if !progressbar.is_hidden() {
            progressbar.println(format!(
                "Downloading default Geonames index for qsv {QSV_VERSION} release..."
            ));
        }
        util::download_file(
            &format!(
                "https://github.com/jqnatividad/qsv/releases/download/{QSV_VERSION}/qsv-{QSV_VERSION}-geocode-index.bincode"
            ),
            &geocode_index_file.to_string_lossy(),
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

#[cached(
    key = "String",
    convert = r#"{ format!("{cell}") }"#,
    option = true,
    sync_writes = false
)]
fn search_cached(
    engine: &Engine,
    mode: GeocodeSubCmd,
    cell: &str,
    formatstr: &str,
    min_score: Option<f32>,
    k: Option<f32>,
) -> Option<String> {
    if mode == GeocodeSubCmd::Suggest {
        let search_result = engine.suggest(cell, 1, min_score);
        let Some(cityrecord) = search_result.into_iter().next() else {
            return None;
        };

        let Some((_admin1_key, admin1_name)) = (match &cityrecord.admin1_names {
            Some(admin1) => admin1.iter().next().map(|s| s.to_owned()),
            None => Some((&EMPTY_STRING, &EMPTY_STRING)),
        }) else {
            return None;
        };

        if formatstr == "%+" {
            // default for suggest is location - e.g. "(lat, long)"
            return Some(format!(
                "({latitude}, {longitude})",
                latitude = cityrecord.latitude,
                longitude = cityrecord.longitude
            ));
        }

        return Some(format_result(cityrecord, formatstr, true, admin1_name));
    } else if mode == GeocodeSubCmd::Reverse {
        // regex for Location field. Accepts (lat, long) & lat, long
        let locregex: &'static Regex = regex_oncelock!(
            r"(?-u)([+-]?[0-9]+\.?[0-9]*|\.[0-9]+),\s*([+-]?[0-9]+\.?[0-9]*|\.[0-9]+)"
        );

        let loccaps = locregex.captures(cell);
        if let Some(loccaps) = loccaps {
            let lat = fast_float::parse(&loccaps[1]).unwrap_or_default();
            let long = fast_float::parse(&loccaps[2]).unwrap_or_default();
            if (-90.0..=90.0).contains(&lat) && (-180.0..=180.0).contains(&long) {
                let search_result = engine.reverse((lat, long), 1, k);
                let Some(cityrecord) = (match search_result {
                    Some(search_result) => search_result.into_iter().next().map(|ri| ri.city),
                    None => return None,
                }) else {
                    return None;
                };

                let Some((_admin1_key, admin1_name)) = (match &cityrecord.admin1_names {
                    Some(admin1) => admin1.iter().next().map(|s| s.to_owned()),
                    None => Some((&EMPTY_STRING, &EMPTY_STRING)),
                }) else {
                    return None;
                };

                if formatstr == "%+" {
                    // default for reverse is city-admin1 - e.g. "Brooklyn, New York"
                    return Some(format!(
                        "{city}, {admin1}",
                        city = cityrecord.name.clone(),
                        admin1 = admin1_name.clone()
                    ));
                }

                return Some(format_result(cityrecord, formatstr, false, admin1_name));
            }
        } else {
            // not a valid lat, long
            return None;
        }
    }

    None
}

/// format the geocoded result based on formatstr if its not %+
#[inline]
fn format_result(
    cityrecord: &CitiesRecord,
    formatstr: &str,
    suggest_mode: bool,
    admin1_name: &str,
) -> String {
    if formatstr.starts_with('%') {
        // if formatstr starts with %, then we're using a predefined format
        match formatstr {
            "%city-admin1" | "%city-state" => format!("{}, {}", cityrecord.name, admin1_name),
            "%lat-long" => format!("{}, {}", cityrecord.latitude, cityrecord.longitude),
            "%location" => format!("({}, {})", cityrecord.latitude, cityrecord.longitude),
            "%city-country" => format!(
                "{}, {}",
                cityrecord.name,
                cityrecord.country.clone().unwrap().name
            ),
            "%city" => cityrecord.name.clone(),
            "%city-state-country" | "%city-admin1-country" => format!(
                "{}, {} {}",
                cityrecord.name,
                admin1_name,
                cityrecord.country.clone().unwrap().name
            ),
            "%state" | "%admin1" => admin1_name.to_owned(),
            "%country" => cityrecord.country.clone().unwrap().name,
            "%id" => format!("{}", cityrecord.id),
            "%population" => format!("{}", cityrecord.population),
            "%timezone" => cityrecord.timezone.clone(),
            "%cityrecord" => format!("{cityrecord:?}"),
            _ => {
                // invalid formatstr, so we use the default for suggest or reverse
                if suggest_mode {
                    // default for suggest is location - e.g. "(lat, long)"
                    format!(
                        "({latitude}, {longitude})",
                        latitude = cityrecord.latitude,
                        longitude = cityrecord.longitude
                    )
                } else {
                    // default for reverse is city-admin1 - e.g. "Brooklyn, New York"
                    format!(
                        "{city}, {admin1}",
                        city = cityrecord.name.clone(),
                        admin1 = admin1_name.to_owned()
                    )
                }
            },
        }
    } else {
        // if formatstr does not start with %, then we're using dynfmt,
        // i.e. eight predefined fields below in curly braces are replaced with values
        // e.g. "City: {name}, State: {admin1}, Country: {country} - {timezone}"

        let mut cityrecord_map: HashMap<&str, String> = HashMap::with_capacity(8);
        cityrecord_map.insert("id", cityrecord.id.to_string());
        cityrecord_map.insert("name", cityrecord.name.clone());
        cityrecord_map.insert("latitude", cityrecord.latitude.to_string());
        cityrecord_map.insert("longitude", cityrecord.longitude.to_string());
        cityrecord_map.insert("country", cityrecord.country.clone().unwrap().name);
        cityrecord_map.insert("admin1", admin1_name.to_owned());
        cityrecord_map.insert("timezone", cityrecord.timezone.clone());
        cityrecord_map.insert("population", cityrecord.population.to_string());

        if let Ok(formatted) = dynfmt::SimpleCurlyFormat.format(formatstr, cityrecord_map) {
            formatted.to_string()
        } else {
            INVALID_DYNFMT.to_string()
        }
    }
}
