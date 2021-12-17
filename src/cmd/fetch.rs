use crate::config::{Config, Delimiter};
use crate::select::SelectColumns;
use crate::util;
use crate::CliResult;
use cached::proc_macro::cached;
use indicatif::{ProgressBar, ProgressDrawTarget};
use log::{debug, error};
use serde::Deserialize;

static USAGE: &str = "
Fetch values via an URL column, and optionally store them in a new column.

This command fetches HTML/data from web pages or web services for every row in the URL column, 
and optionally stores them in a new column.

URL column must contain full and valid URL path, which can be constructed via the 'lua' command.

To set proxy, please set env var HTTP_PROXY and HTTPS_PROXY (eg export HTTPS_PROXY=socks5://127.0.0.1:1086)

Usage:
    qsv fetch [options] [<column>] [<input>]

fetch options:
    -c, --new-column <name>    Put the fetched values in a new column instead.
    --jql <selector>           Apply jql selector to API returned JSON value.
    -j, --jobs <value>         Number of concurrent requests.
    --throttle <ms>            Set throttle delay between requests in milliseconds. Recommend 1000 ms or greater (default: 5000 ms).
    --header <file>            File containing additional HTTP Request Headers. Useful for setting Authorization or overriding User Agent.
    --store-error              On error, store HTTP error instead of blank value.
    --cookies                  Automatically store and send cookies. Useful for authenticated sessions.

Common options:
    -h, --help                 Display this message
    -o, --output <file>        Write output to <file> instead of stdout.
    -n, --no-headers           When set, the first row will not be interpreted
                               as headers. Namely, it will be sorted with the rest
                               of the rows. Otherwise, the first row will always
                               appear as the header row in the output.
    -d, --delimiter <arg>      The field delimiter for reading CSV data.
                               Must be a single character. (default: ,)
    -q, --quiet                Don't show progress bars.
";

#[derive(Deserialize, Debug)]
struct Args {
    flag_new_column: Option<String>,
    flag_jql: Option<String>,
    flag_jobs: Option<u8>,
    flag_throttle: Option<usize>,
    flag_header: Option<String>,
    flag_store_error: bool,
    flag_cookies: bool,
    flag_output: Option<String>,
    flag_no_headers: bool,
    flag_delimiter: Option<Delimiter>,
    flag_quiet: bool,
    arg_column: SelectColumns,
    arg_input: Option<String>,
}

static DEFAULT_USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    " (https://github.com/jqnatividad/qsv)",
);

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;

    debug!(
        "url column: {:?}, 
            input: {:?}, 
            new column: {:?}, 
            jql: {:?},
            threads: {:?},
            throttle delay: {:?},
            http header file: {:?},
            store error: {:?},
            store cookie: {:?},
            output: {:?}, 
            no_header: {:?}, 
            delimiter: {:?}, 
            quiet: {:?}",
        (&args.arg_column).clone(),
        (&args.arg_input).clone().unwrap(),
        &args.flag_new_column,
        &args.flag_jql,
        &args.flag_jobs,
        &args.flag_throttle,
        &args.flag_header,
        &args.flag_store_error,
        &args.flag_cookies,
        &args.flag_output,
        &args.flag_no_headers,
        &args.flag_delimiter,
        &args.flag_quiet
    );

    let rconfig = Config::new(&args.arg_input)
        .delimiter(args.flag_delimiter)
        .no_headers(args.flag_no_headers)
        .select(args.arg_column);

    let mut rdr = rconfig.reader()?;
    let mut wtr = Config::new(&None).writer()?;

    let mut headers = rdr.byte_headers()?.clone();
    let sel = rconfig.selection(&headers)?;
    let column_index = *sel.iter().next().unwrap();

    assert!(
        sel.len() == 1,
        "Only one single URL column may be selected."
    );

    use reqwest::blocking::Client;
    let client = Client::builder()
        .user_agent(DEFAULT_USER_AGENT)
        .build()
        .unwrap();

    let mut include_existing_columns = false;

    if let Some(name) = &args.flag_new_column {
        include_existing_columns = true;

        // write header with new column
        headers.push_field(name.as_bytes());
        wtr.write_byte_record(&headers)?;
    }

    // prep progress bar
    let progress = ProgressBar::new(0);
    if !args.flag_quiet {
        let record_count = util::count_rows(&rconfig);
        util::prep_progress(&progress, record_count);
    } else {
        progress.set_draw_target(ProgressDrawTarget::hidden());
    }

    #[allow(unused_assignments)]
    let mut record = csv::ByteRecord::new();
    for row in rdr.byte_records() {
        record = row?;

        if !args.flag_quiet {
            progress.inc(1);
        }

        let selected_col_value = record[column_index].to_owned();
        let url = String::from_utf8_lossy(&selected_col_value).to_string();
        debug!("Fetching URL: {:?}", &url);

        let final_value = get_cached_response(&url, &client, &args.flag_jql);

        if include_existing_columns {
            record.push_field(final_value.as_bytes());
            wtr.write_byte_record(&record)?;
        } else {
            let mut output_record = csv::ByteRecord::new();
            output_record.push_field(final_value.as_bytes());
            wtr.write_byte_record(&output_record)?;
        }
    }

    if !args.flag_quiet {
        use cached::Cached;
        use thousands::Separable;

        let cache = GET_CACHED_RESPONSE.lock().unwrap();
        let cache_size = cache.cache_size();
        let hits = cache.cache_hits().unwrap();
        let misses = cache.cache_misses().unwrap();
        let hit_ratio = (hits as f64 / (hits + misses) as f64) * 100.0;
        progress.set_message(format!(
            " of {} records. Cache hit ratio: {:.2}% - {} entries",
            progress.length().separate_with_commas(),
            hit_ratio,
            cache_size.separate_with_commas(),
        ));
        util::finish_progress(&progress);
    }

    Ok(())
}

#[cached(
    key = "String",
    convert = r#"{ format!("{}", url) }"#,
    sync_writes = false
)]
fn get_cached_response(
    url: &str,
    client: &reqwest::blocking::Client,
    flag_jql: &Option<String>,
) -> String {
    let resp = client.get(url).send().unwrap();
    debug!("response: {:?}", &resp);
    let api_value = resp.text().unwrap();
    debug!("api value: {:?}", &api_value);
    let mut final_value: String = (&api_value).clone();
    if let Some(selectors) = flag_jql {
        use jql::walker;
        use serde_json::{Deserializer, Value};

        // TODO: check if api returns JSON
        Deserializer::from_str(&api_value)
            .into_iter::<Value>()
            .for_each(|value| match value {
                Ok(valid_json) => {
                    // Walk through the JSON content with the provided selectors as
                    // input.
                    match walker(&valid_json, Some(selectors)) {
                        Ok(selection) => {
                            final_value = String::from(selection.as_str().unwrap_or_default());
                            debug!("jql selected value: {:?}", &final_value);
                        }
                        Err(error) => {
                            error!("Error selecting from JSON: {}", error);
                        }
                    }
                }
                Err(_) => {
                    error!("Invalid JSON file or content");
                }
            });
    }
    final_value
}
