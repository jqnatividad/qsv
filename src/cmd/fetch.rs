use crate::config::{Config, Delimiter};
use crate::select::SelectColumns;
use crate::util;
use crate::CliResult;
use cached::proc_macro::cached;
use indicatif::{ProgressBar, ProgressDrawTarget};
use log::{debug, error};
use serde::Deserialize;

static USAGE: &str = "
Fetch data via a URL column, and optionally store them in a new column.

This command fetches data from web api for every row in the URL column, 
and optionally stores them in a new column.

Fetch is integrated with `jql` to directly parse out values from api JSON response.

URL column must contain full and valid URL path, which can be constructed via the `lua` command.

To set proxy, please set env var HTTP_PROXY and HTTPS_PROXY (eg export HTTPS_PROXY=socks5://127.0.0.1:1086)

Usage:
    qsv fetch [options] [--http-header <k:v>...] [<column>] [<input>]

fetch options:
    -c, --new-column <name>    Put the fetched values in a new column instead.
    --jql <selector>           Apply jql selector to API returned JSON value.
    --rate-limit <qps>         Rate Limit in Queries Per Second. [default: 5]
    --http-header <k:v>        Pass custom header(s) to the server.
    --store-error              On error, store error code/message instead of blank value.
    --cookies                  Allow cookies.

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
    flag_rate_limit: Option<u32>,
    flag_http_header: Vec<String>,
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

    let rconfig = Config::new(&args.arg_input)
        .delimiter(args.flag_delimiter)
        .no_headers(args.flag_no_headers)
        .select(args.arg_column);

    let mut rdr = rconfig.reader()?;
    let mut wtr = Config::new(&args.flag_output).writer()?;

    let mut headers = rdr.byte_headers()?.clone();
    let sel = rconfig.selection(&headers)?;
    let column_index = *sel.iter().next().unwrap();

    assert!(
        sel.len() == 1,
        "Only one single URL column may be selected."
    );

    use std::num::NonZeroU32;
    // default rate limit is actually set via docopt, so below init is just to satisfy compiler
    let mut rate_limit: NonZeroU32 = NonZeroU32::new(5).unwrap();
    if let Some(qps) = args.flag_rate_limit {
        assert!(
            // on my laptop, no more sleep traces with qps > 24, so use round number of 20 as single-thread qps limit
            qps <= 20 && qps > 0,
            "Rate Limit should be between 1 to 20 queries per second."
        );
        rate_limit = NonZeroU32::new(qps).unwrap();
    }

    use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

    let http_headers: HeaderMap = {
        let mut map = HeaderMap::new();
        for header in args.flag_http_header {
            let vals: Vec<&str> = header.split(':').collect();

            // allocate new String for header key to put into map
            let k: String = String::from(vals[0].trim());
            let header_name: HeaderName =
                HeaderName::from_lowercase(k.to_lowercase().as_bytes()).unwrap();

            // allocate new String for header value to put into map
            let v: String = String::from(vals[1].trim());
            let header_val: HeaderValue = HeaderValue::from_str(v.as_str()).unwrap();

            map.append(header_name, header_val);
        }

        map
    };

    use reqwest::blocking::Client;
    let client = Client::builder()
        .user_agent(DEFAULT_USER_AGENT)
        .default_headers(http_headers)
        .cookie_store(args.flag_cookies)
        .build()
        .unwrap();

    use governor::{Quota, RateLimiter};

    let limiter = RateLimiter::direct(Quota::per_second(rate_limit));

    let mut include_existing_columns = false;

    if let Some(name) = args.flag_new_column {
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
        let url = String::from_utf8_lossy(&selected_col_value)
            .trim()
            .to_string();
        debug!("Fetching URL: {:?}", &url);

        let final_value = get_cached_response(
            &url,
            &client,
            &limiter,
            &args.flag_jql,
            args.flag_store_error,
        );

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

use governor::{
    clock::DefaultClock, middleware::NoOpMiddleware, state::direct::NotKeyed, state::InMemoryState,
};
use std::{thread, time};

#[cached(
    key = "String",
    convert = r#"{ format!("{}", url) }"#,
    sync_writes = false
)]
fn get_cached_response(
    url: &str,
    client: &reqwest::blocking::Client,
    limiter: &governor::RateLimiter<NotKeyed, InMemoryState, DefaultClock, NoOpMiddleware>,
    flag_jql: &Option<String>,
    flag_store_error: bool,
) -> String {
    // wait until RateLimiter gives Okay
    while limiter.check().is_err() {
        thread::sleep(time::Duration::from_millis(10));
    }

    let resp: reqwest::blocking::Response;

    match client.get(url).send() {
        Ok(response) => {
            resp = response;
        }
        Err(error) => {
            error!("Cannot fetch url: {:?}, error: {:?}", url, error);
            if flag_store_error {
                return error.to_string();
            } else {
                return String::default();
            }
        }
    }
    debug!("response: {:?}", &resp);

    let api_status = resp.status();
    let api_value: String = resp.text().unwrap();
    debug!("api value: {}", &api_value);

    let final_value: String;

    if api_status.is_client_error() || api_status.is_server_error() {
        error!(
            "HTTP error. url: {:?}, error: {:?}",
            url,
            api_status.canonical_reason().unwrap_or("unknown error")
        );

        if flag_store_error {
            final_value = format!(
                "HTTP {} - {}",
                api_status.as_str(),
                api_status.canonical_reason().unwrap_or("unknown error")
            );
        } else {
            final_value = String::default();
        }
    } else {
        // apply JQL selector if provided
        if let Some(selectors) = flag_jql {
            match apply_jql(&api_value, selectors) {
                Ok(s) => {
                    final_value = s;
                }
                Err(e) => {
                    error!(
                        "jql error. json: {:?}, selectors: {:?}, error: {:?}",
                        &api_value,
                        selectors,
                        e.to_string()
                    );

                    if flag_store_error {
                        final_value = e.to_string();
                    } else {
                        final_value = String::default();
                    }
                }
            }
        } else {
            final_value = api_value;
        }
    }

    debug!("final value: {}", &final_value);

    final_value
}

use jql::walker;
use serde_json::{Deserializer, Value};

use anyhow::{anyhow, Result};

fn apply_jql(json: &str, selectors: &str) -> Result<String> {
    // check if api returned valid JSON before applying JQL selector
    if let Err(error) = serde_json::from_str::<Value>(json) {
        return Err(anyhow!("Invalid json: {:?}", error));
    }

    let mut result: Result<String> = Ok(String::default());

    Deserializer::from_str(json)
        .into_iter::<Value>()
        .for_each(|value| match value {
            Ok(valid_json) => {
                // Walk through the JSON content with the provided selectors as
                // input.
                match walker(&valid_json, Some(selectors)) {
                    Ok(selection) => {
                        fn get_value_string(v: &Value) -> String {
                            if v.is_null() {
                                "null".to_string()
                            } else if v.is_boolean() {
                                v.as_bool().unwrap().to_string()
                            } else if v.is_f64() {
                                v.as_f64().unwrap().to_string()
                            } else if v.is_i64() {
                                v.as_i64().unwrap().to_string()
                            } else if v.is_u64() {
                                v.as_u64().unwrap().to_string()
                            } else if v.is_string() {
                                v.as_str().unwrap().to_string()
                            } else {
                                v.to_string()
                            }
                        }

                        match &selection {
                            Value::Array(array) => {
                                let mut concat_string = String::new();

                                let mut values = array.iter();

                                if let Some(v) = values.next() {
                                    let str_val = get_value_string(v);
                                    concat_string.push_str(&str_val);
                                }

                                for v in values {
                                    let str_val = get_value_string(v);
                                    concat_string.push_str(", ");
                                    concat_string.push_str(&str_val);
                                }

                                result = Ok(concat_string);
                            }
                            Value::Object(_object) => {
                                result = Err(anyhow!("Unsupported jql result type: OBJECT"));
                            }
                            _ => {
                                result = Ok(get_value_string(&selection));
                            }
                        }
                    }
                    Err(error) => {
                        result = Err(anyhow!(error));
                    }
                }
            }
            Err(error) => {
                // shouldn't happen, but do same thing earlier when checking for invalid json
                result = Err(anyhow!("Invalid json: {:?}", error));
            }
        });

    result
}

#[test]
fn test_apply_jql_invalid_json() {
    let json =
        r#"<!doctype html><html lang="en"><meta charset=utf-8><title>shortest html5</title>"#;
    let selectors = r#"."places"[0]."place name""#;

    let value: String = apply_jql(json, selectors).unwrap_err().to_string();

    assert_eq!(
        "Invalid json: Error(\"expected value\", line: 1, column: 1)",
        value
    );
}

#[test]
fn test_apply_jql_invalid_selector() {
    let json = r#"{"post code": "90210", "country": "United States", "country abbreviation": "US", "places": [{"place name": "Beverly Hills", "longitude": "-118.4065", "state": "California", "state abbreviation": "CA", "latitude": "34.0901"}]}"#;
    let selectors = r#"."place"[0]."place name""#;

    let value = apply_jql(json, selectors).unwrap_err().to_string();

    assert_eq!("Node \"place\" not found on the parent element", value);
}

#[test]
fn test_apply_jql_string() {
    let json = r#"{"post code": "90210", "country": "United States", "country abbreviation": "US", "places": [{"place name": "Beverly Hills", "longitude": "-118.4065", "state": "California", "state abbreviation": "CA", "latitude": "34.0901"}]}"#;
    let selectors = r#"."places"[0]."place name""#;

    let value: String = apply_jql(json, selectors).unwrap();

    assert_eq!("Beverly Hills", value);
}

#[test]
fn test_apply_jql_number() {
    let json = r#"{"post code": "90210", "country": "United States", "country abbreviation": "US", "places": [{"place name": "Beverly Hills", "longitude": -118.4065, "state": "California", "state abbreviation": "CA", "latitude": 34.0901}]}"#;
    let selectors = r#"."places"[0]."longitude""#;

    let value: String = apply_jql(json, selectors).unwrap();

    assert_eq!("-118.4065", value);
}

#[test]
fn test_apply_jql_bool() {
    let json = r#"{"post code": "90210", "country": "United States", "country abbreviation": "US", "places": [{"place name": "Beverly Hills", "longitude": -118.4065, "state": "California", "state abbreviation": "CA", "latitude": 34.0901, "expensive": true}]}"#;
    let selectors = r#"."places"[0]."expensive""#;

    let value: String = apply_jql(json, selectors).unwrap();

    assert_eq!(true.to_string(), value);
}

#[test]
fn test_apply_jql_null() {
    let json = r#"{"post code": "90210", "country": "United States", "country abbreviation": "US", "places": [{"place name": "Beverly Hills", "longitude": -118.4065, "state": "California", "state abbreviation": "CA", "latitude": 34.0901, "university":null}]}"#;
    let selectors = r#"."places"[0]."university""#;

    let value: String = apply_jql(json, selectors).unwrap();

    assert_eq!("null".to_string(), value);
}

#[test]
fn test_apply_jql_array() {
    let json = r#"{"post code": "90210", "country": "United States", "country abbreviation": "US", "places": [{"place name": "Beverly Hills", "longitude": -118.4065, "state": "California", "state abbreviation": "CA", "latitude": 34.0901}]}"#;
    let selectors = r#"."places"[0]."longitude",."places"[0]."latitude""#;

    let value: String = apply_jql(json, selectors).unwrap();

    assert_eq!("-118.4065, 34.0901".to_string(), value);
}
