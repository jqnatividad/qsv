static USAGE: &str = r#"
Geocodes a location against the Geonames cities database.

It has three subcommands:
 * suggest - given a City name, return the closest location coordinate.
 * reverse - given a location coordinate, return the closest City.
 * index - operations to update the Geonames cities database used by the geocode command.
 
SUGGEST
Geocodes to the nearest city center point given a location column
[i.e. a column which contains a latitude, longitude WGS84 coordinate] against
an embedded copy of the Geonames city database.

The geocoded information is formatted based on --formatstr, returning
it in 'city-state' format if not specified.

Use the --new-column option if you want to keep the location column:

Examples:
Geocode file.csv Location column and set the geocoded value to a
new column named City.

$ qsv geocode suggest Location --new-column City file.csv

Geocode file.csv Location column with --formatstr=city-state and
set the geocoded value a new column named City.

$ qsv geocode suggest Location --formatstr city-state --new-column City file.csv

REVERSE
Reverse geocode a WG84 coordinate to the nearest city center point.

Examples:
Reverse geocode file.csv LatLong column and set the geocoded value to a
new column named City.

$ qsv geocode reverse LatLong --new-column City file.csv

INDEX
Updates the Geonames cities database used by the geocode command.

It has three operations:
 * check - checks if the Geonames cities database is up-to-date.
 * update - updates the Geonames cities database with the latest changes.
 * load - load a Geonames cities database from a file.
 * save - save the Geonames cities database to a file.

 Examples:
Update the Geonames cities database with the latest changes.

$ qsv geocode index update

Load a Geonames cities database from a file.

$ qsv geocode index load geonames_cities.dmp

Save the Geonames cities database to a file.

$ qsv geocode index save geonames_cities.dmp

For more extensive examples, see https://github.com/jqnatividad/qsv/blob/master/tests/test_geocode.rs.

Usage:
qsv geocode suggest [--formatstr=<string>] [options] <column> [<input>]
qsv geocode reverse [--formatstr=<string>] [options] <column> [<input>]
qsv geocode index <operation> [<op_input>]
qsv geocode --help

geocode arguments:
The <column> argument can be a list of columns for the operations, emptyreplace &
datefmt subcommands. See 'qsv select --help' for the format details.
        
    <input>                     The input file to read from. If not specified, reads from stdin.

                                INDEX ARGUMENTS ONLY:
    <operation>                 The operation to perform. The available operations are:
                                    - 'update' - updates the Geonames cities database with the latest changes
                                               from the Geonames website.
                                    - 'load' - load a Geonames cities database from a file.
                                    - 'save' - save the Geonames cities database to a file.
    <op_input>                  The input file to read from. Only used by the 'load' & 'save' operations.

geocode options:
    -c, --new-column <name>     Put the transformed values in a new column instead.
    -r, --rename <name>         New name for the transformed column.
    -f, --formatstr=<string>    This option is used by several subcommands:

                                The place format to use. The available formats are:
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
    -b, --batch <size>          The number of rows per batch to load into memory, before running in parallel.
                                [default: 50000]
    --timeout <seconds>         Timeout for downloading lookup_tables using
                                the qsv_register_lookup() helper function.
                                [default: 30]                                
    --languages <lang>          The languages to use for the Geonames cities database.
                                The languages are specified as a comma-separated list of ISO 639-1 codes.
                                [default: en]
    --cache-dir <dir>           The directory to use for caching the Geonames cities database.
                                If the directory does not exist, qsv will attempt to create it.
                                If the QSV_CACHE_DIR envvar is set, it will be used instead.
                                [default: qsv-cache]

Common options:
    -h, --help                  Display this message
    -o, --output <file>         Write output to <file> instead of stdout.
    -n, --no-headers            When set, the first row will not be interpreted
                                as headers.
    -d, --delimiter <arg>       The field delimiter for reading CSV data.
                                Must be a single character. (default: ,)
    -p, --progressbar           Show progress bars. Not valid for stdin.
"#;

use std::{
    fs,
    path::{Path, PathBuf},
    str::FromStr,
    sync::OnceLock,
};

use cached::proc_macro::cached;
use geosuggest_core::{Engine, EngineDumpFormat};
use geosuggest_utils::{IndexUpdater, IndexUpdaterSettings, SourceItem};
use indicatif::{ProgressBar, ProgressDrawTarget};
use log::{debug, info};
use rayon::{
    iter::{IndexedParallelIterator, ParallelIterator},
    prelude::IntoParallelRefIterator,
};
use regex::Regex;
use serde::Deserialize;
use strum_macros::EnumString;

use crate::{
    clitypes::CliError,
    config::{Config, Delimiter},
    regex_oncelock,
    select::SelectColumns,
    util, CliResult,
};

#[derive(EnumString)]
#[strum(ascii_case_insensitive)]
#[allow(non_camel_case_types)]
enum Operation {
    Check,
    Update,
    Load,
    Save,
    None,
}

#[derive(Deserialize)]
struct Args {
    arg_column:       SelectColumns,
    cmd_suggest:      bool,
    arg_operation:    String,
    cmd_reverse:      bool,
    cmd_index:        bool,
    arg_input:        Option<String>,
    arg_op_input:     Option<String>,
    flag_rename:      Option<String>,
    flag_formatstr:   String,
    flag_batch:       u32,
    flag_timeout:     u16,
    flag_languages:   String,
    flag_cache_dir:   String,
    flag_jobs:        Option<usize>,
    flag_new_column:  Option<String>,
    flag_output:      Option<String>,
    flag_no_headers:  bool,
    flag_delimiter:   Option<Delimiter>,
    flag_progressbar: bool,
}

// static LOCS: OnceLock<Locations> = OnceLock::new();
// static GEOCODER: OnceLock<ReverseGeocoder> = OnceLock::new();
static DEFAULT_GEOCODE_INDEX_FILENAME: &str = "qsv-geocode-index.bincode";

static DEFAULT_CITIES_DB_URL: &str = "https://download.geonames.org/export/dump/cities15000.zip";
static DEFAULT_CITIES_DB_FILENAME: &str = "cities15000.txt";
static DEFAULT_CITIES_NAMES_URL: &str =
    "https://download.geonames.org/export/dump/alternateNamesV2.zip";
static DEFAULT_CITIES_NAMES_FILENAME: &str = "alternateNamesV2.txt";
static DEFAULT_COUNTRY_INFO_URL: &str = "https://download.geonames.org/export/dump/countryInfo.txt";
static DEFAULT_ADMIN1_CODES_URL: &str =
    "https://download.geonames.org/export/dump/admin1CodesASCII.txt";

// valid subcommands
#[derive(Clone, Copy, PartialEq)]
enum GeocodeSubCmd {
    Suggest,
    Reverse,
    Index,
}

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

    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(geocode_main(args))?;

    Ok(())
}

async fn geocode_main(args: Args) -> CliResult<()> {
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
        let new_col_names = util::ColumnNameParser::new(&new_name).parse()?;
        if new_col_names.len() != sel.len() {
            return fail!("Number of new columns does not match input column selection.");
        }
        for (i, col_index) in sel.iter().enumerate() {
            headers = replace_column_value(&headers, *col_index, &new_col_names[i]);
        }
    }

    if !rconfig.no_headers {
        if let Some(new_column) = &args.flag_new_column {
            headers.push_field(new_column);
        }
        wtr.write_record(&headers)?;
    }

    let geocode_cmd = if args.cmd_index {
        GeocodeSubCmd::Index
    } else if args.cmd_suggest {
        GeocodeSubCmd::Suggest
    } else if args.cmd_reverse {
        GeocodeSubCmd::Reverse
    } else {
        return fail!("Unknown geocode subcommand.");
    };

    // setup cache directory
    let geocode_cache_dir = if let Ok(cache_dir) = std::env::var("QSV_CACHE_DIR") {
        // if QSV_CACHE_DIR env var is set, check if it exists. If it doesn't, create it.
        if !Path::new(&cache_dir).exists() {
            fs::create_dir_all(&cache_dir)?;
        }
        cache_dir
    } else {
        if !Path::new(&args.flag_cache_dir).exists() {
            fs::create_dir_all(&args.flag_cache_dir)?;
        }
        args.flag_cache_dir.clone()
    };
    info!("Using cache directory: {geocode_cache_dir}");

    let geocode_index_filename = std::env::var("QSV_GEOCODE_INDEX_FILENAME")
        .unwrap_or_else(|_| DEFAULT_GEOCODE_INDEX_FILENAME.to_string());
    let geocode_index_file = args.arg_op_input.unwrap_or_else(|| {
        let mut path = PathBuf::from(geocode_cache_dir);
        path.push(geocode_index_filename);
        path.to_string_lossy().to_string()
    });

    // setup languages
    let languages_vec = args
        .flag_languages
        .to_ascii_lowercase()
        .split(',')
        .map(|s| s.trim().to_string())
        .collect::<Vec<String>>();
    let languages_vec: Vec<&str> = languages_vec
        .iter()
        .map(std::string::String::as_str)
        .collect();

    // load geocode engine
    let engine = load_engine(geocode_index_file.clone().into(), languages_vec.clone()).await?;

    // its an index operation, apply the requested operation to the geonames index
    if geocode_cmd == GeocodeSubCmd::Index {
        let Ok(op) = Operation::from_str(&args.arg_operation) else {
            return fail_clierror!("Invalid operation: {}", args.arg_operation);
        };
        let updater = IndexUpdater::new(IndexUpdaterSettings {
            http_timeout_ms:  util::timeout_secs(args.flag_timeout)? * 1000,
            cities:           SourceItem {
                url:      DEFAULT_CITIES_DB_URL, //"http://download.geonames.org/export/dump/cities5000.zip",
                filename: DEFAULT_CITIES_DB_FILENAME, //"cities5000.txt",
            },
            names:            Some(SourceItem {
                url:      DEFAULT_CITIES_NAMES_URL, //"http://download.geonames.org/export/dump/alternateNamesV2.zip",
                filename: DEFAULT_CITIES_NAMES_FILENAME, //"alternateNamesV2.txt",
            }),
            countries_url:    Some(DEFAULT_COUNTRY_INFO_URL), /* Some("http://download.geonames.org/export/dump/countryInfo.txt"), */
            admin1_codes_url: Some(DEFAULT_ADMIN1_CODES_URL), /* Some("http://download.geonames.org/export/dump/admin1CodesASCII.txt"), */
            filter_languages: languages_vec,
        })?;

        match op {
            Operation::Check => {
                if updater.has_updates(&engine).await? {
                    println!("Updates available.");
                } else {
                    println!("No updates available.");
                }
            },
            Operation::Update => {
                let engine = updater.build().await?;
                engine.dump_to(geocode_index_file.clone(), EngineDumpFormat::Bincode)?;
                println!("Updates applied: {geocode_index_file}");
            },
            Operation::Load => {},
            Operation::Save => {
                engine.dump_to(geocode_index_file.clone(), EngineDumpFormat::Bincode)?;
                println!("Index saved: {geocode_index_file}");
            },
            Operation::None => return fail_clierror!("No operation specified."),
        }
        return Ok(());
    }

    // prep progress bar
    let show_progress =
        (args.flag_progressbar || util::get_envvar_flag("QSV_PROGRESSBAR")) && !rconfig.is_stdin();

    let progress = ProgressBar::with_draw_target(None, ProgressDrawTarget::stderr_with_hz(5));
    if show_progress {
        util::prep_progress(&progress, util::count_rows(&rconfig)?);
    } else {
        progress.set_draw_target(ProgressDrawTarget::hidden());
    }

    // amortize memory allocation by reusing record
    #[allow(unused_assignments)]
    let mut batch_record = csv::StringRecord::new();

    // reuse batch buffers
    let batchsize: usize = args.flag_batch as usize;
    let mut batch = Vec::with_capacity(batchsize);
    let mut batch_results = Vec::with_capacity(batchsize);

    // set RAYON_NUM_THREADS
    util::njobs(args.flag_jobs);

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

        // do actual apply command via Rayon parallel iterator
        batch
            .par_iter()
            .map(|record_item| {
                let mut record = record_item.clone();
                let mut cell = record[column_index].to_owned();
                if !cell.is_empty() {
                    let search_result =
                        search_cached(&engine, geocode_cmd, &cell, &args.flag_formatstr);
                    if let Some(geocoded_result) = search_result {
                        cell = geocoded_result;
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
        // if args.cmd_geocode {
        //     util::update_cache_info!(progress, SEARCH_CACHED);
        // }
        util::finish_progress(&progress);
    }
    Ok(wtr.flush()?)
}

async fn load_engine(geocode_index_file: PathBuf, languages_vec: Vec<&str>) -> CliResult<Engine> {
    let index_file = std::path::Path::new(&geocode_index_file);

    if languages_vec.is_empty() {
        return fail!("No languages specified.");
    }

    let updater = IndexUpdater::new(IndexUpdaterSettings {
        filter_languages: languages_vec,
        ..Default::default()
    })?;

    Ok(if index_file.exists() {
        // load existing index
        let engine = Engine::load_from(index_file, EngineDumpFormat::Bincode)
            .map_err(|e| format!("On load index file: {e}"))?;

        if updater.has_updates(&engine).await? {
            // rewrite index file
            let engine = updater
                .build()
                .await
                .map_err(|e| format!("Geonames index update failed: {e}"))?;
            engine.dump_to(index_file, EngineDumpFormat::Bincode)?;
            engine
        } else {
            engine
        }
    } else {
        // initial load
        let engine = updater.build().await?;
        engine
            .dump_to(index_file, EngineDumpFormat::Bincode)
            .map_err(|e| format!("Geonames initial load failed: {e}"))?;
        engine
    })
}

#[cached(
    key = "String",
    convert = r#"{ format!("{cell}") }"#,
    option = true,
    sync_writes = true
)]
fn search_cached(
    engine: &Engine,
    mode: GeocodeSubCmd,
    cell: &str,
    formatstr: &str,
) -> Option<String> {
    static EMPTY_STRING: String = String::new();

    let mut id = 0_usize;
    let mut city_name = String::new();
    let mut country = String::new();
    let mut admin1_name_value = String::new();
    let mut latitude = 0_f32;
    let mut longitude = 0_f32;
    let mut population = 0_usize;
    let mut timezone = String::new();
    let mut cityrecord_dbg = String::new();

    if mode == GeocodeSubCmd::Suggest {
        let search_result = engine.suggest(cell, 1, None);
        let Some(cityrecord) = search_result.into_iter().next() else {
            return None;
        };

        let Some((_admin1_name_key, admin1_name_value_work)) = (match &cityrecord.admin1_names {
            Some(admin1) => admin1.iter().next().map(|s| s.to_owned()),
            None => Some((&EMPTY_STRING, &EMPTY_STRING)),
        }) else {
            return None;
        };

        id = cityrecord.id;
        city_name = cityrecord.name.clone();
        latitude = cityrecord.latitude;
        longitude = cityrecord.longitude;
        country = cityrecord.country.clone().unwrap().name;
        admin1_name_value = admin1_name_value_work.clone();
        population = cityrecord.population;
        timezone = cityrecord.timezone.clone();
        cityrecord_dbg = if formatstr == "cityrecord" {
            format!("{cityrecord:?}")
        } else {
            EMPTY_STRING.clone()
        };
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
                let search_result = engine.reverse((lat, long), 1, None);
                let Some(cityrecord) = (match search_result {
                    Some(search_result) => search_result.into_iter().next().map(|ri| ri.city),
                    None => return None,
                }) else {
                    return None;
                };

                let Some((_admin1_name_key, admin1_name_value_work)) =
                    (match &cityrecord.admin1_names {
                        Some(admin1) => admin1.iter().next().map(|s| s.to_owned()),
                        None => Some((&EMPTY_STRING, &EMPTY_STRING)),
                    })
                else {
                    return None;
                };

                id = cityrecord.id;
                city_name = cityrecord.name.clone();
                latitude = cityrecord.latitude;
                longitude = cityrecord.longitude;
                country = cityrecord.country.clone().unwrap().name;
                admin1_name_value = admin1_name_value_work.clone();
                population = cityrecord.population;
                timezone = cityrecord.timezone.clone();
                cityrecord_dbg = if formatstr == "cityrecord" {
                    format!("{cityrecord:?}")
                } else {
                    EMPTY_STRING.clone()
                };
            }
        } else {
            return None;
        }
    } else {
        return None;
    }

    #[allow(clippy::match_same_arms)]
    // match arms are evaluated in order,
    // so we're optimizing for the most common cases first
    let result = match formatstr {
        "%+" | "city-state" => format!("{city_name}, {admin1_name_value}"),
        "lat-long" => format!("{latitude}, {longitude}"),
        "location" => format!("({latitude}, {longitude})"),
        "city-country" => format!("{city_name}, {country}"),
        "city" => city_name,
        "state" => admin1_name_value,
        "country" => country,
        "id" => format!("{id}"),
        "population" => format!("{population}"),
        "timezone" => timezone,
        "cityrecord" => cityrecord_dbg,
        _ => format!("{city_name}, {admin1_name_value}, {country}"),
    };
    return Some(result);
}
