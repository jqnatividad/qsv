use crate::config::{Config, Delimiter};
use crate::select::SelectColumns;
use crate::CliResult;
use crate::{regex_once_cell, util};
use cached::proc_macro::{cached, io_cached};
use cached::{RedisCache, Return};
use dynfmt::Format;
use indicatif::{ProgressBar, ProgressDrawTarget};
use log::{debug, error};
use once_cell::sync::{Lazy, OnceCell};
use regex::Regex;
use serde::Deserialize;
use std::fs;
use thiserror::Error;

static USAGE: &str = "
This command fetches data from a web API for every row in the URL column, 
and optionally stores them in a new column.

Fetch is integrated with `jql` to directly parse out values from an API JSON response.

The URL column needs to be a fully qualified URL path. Alternatively, you can dynamically
construct URLs for each CSV record with the --url-template option (see Examples below).

To use a proxy, please set env vars HTTP_PROXY and HTTPS_PROXY
(e.g. export HTTPS_PROXY=socks5://127.0.0.1:1086).

To use Redis for response caching, set the --redis flag. By default, it will connect to
a local Redis instance at redis://127.0.0.1:6379, with a cache expiry Time-to-Live (TTL)
of 2,419,200 seconds (28 days), and cache hits NOT refreshing the TTL of cached values.

Set the env vars QSV_REDIS_CONNECTION_STRING, QSV_REDIS_TTL_SECONDS and 
QSV_REDIS_TTL_REFRESH to change default Redis settings.

EXAMPLES USING THE COLUMN ARGUMENT:

data.csv
  URL
  http://api.zippopotam.us/us/90210
  http://api.zippopotam.us/us/94105
  https://api.zippopotam.us/us/92802

Given the data.csv above, fetch the JSON response.

  $ qsv fetch URL data.csv 

Note the output will be a JSONL file - with a minified JSON response per line,
not a CSV file.

Now, if we want to generate a CSV file with the parsed City and State, we use the 
new-column and jql options.

  $ qsv fetch URL --new-column CityState \
    --jql '\"\"\"places\"\"\"[0].\"\"\"place name\"\"\",\"\"\"places\"\"\"[0].\"\"\"state abbreviation\"\"\"' \
    data.csv > datatest.csv

data_with_CityState.csv
  URL, CityState,
  http://api.zippopotam.us/us/90210, \"Beverly Hills, CA\"
  http://api.zippopotam.us/us/94105, \"San Francisco, CA\"
  https://api.zippopotam.us/us/92802, \"Anaheim, CA\"

As you can see, entering jql selectors can quickly become cumbersome, more so because
of the need to escape quotes on the command line. Alternatively, the jql selector
can be saved and loaded from a file using the --jqlfile option. As an added bonus, there is
no need to escape quotes in the file, making for a more readable jql.

  $ qsv fetch URL --new-column CityState --jqlfile places.jql data.csv > datatest.csv

EXAMPLES USING THE --URL-TEMPLATE OPTION:

Geocode addresses in addr_data.csv, pass the latitude and longitude fields and store
the response in a new column called response into enriched_addr_data.csv.

  $ qsv fetch --url-template \"https://geocode.test/api/lookup.json?lat={latitude}&long={longitude}\" \
       addr_data.csv -c response > enriched_addr_data.csv

Geocode addresses in addr_data.csv, pass the \"street address\" and \"zip-code\" fields
and use jql to parse CityState from the JSON response into a new column in enriched.csv.
Note how field name non-alphanumeric characters in the url-template were replace with _.

  $ qsv fetch --url-template \"https://geocode.test/api/addr.json?addr={street_address}&zip={zip_code}\" \
       --jql '\"\"\"places\"\"\"[0].\"\"\"place name\"\"\",\"\"\"places\"\"\"[0].\"\"\"state abbreviation\"\"\"' \
       addr_data.csv -c CityState > enriched.csv

Usage:
    qsv fetch (<column> | --url-template <template>) [--jql <selector> | --jqlfile <file>] [--http-header <k:v>...] [options] [<input>]

Fetch options:
    --url-template <template>  URL template to use. Use column names enclosed with
                               curly braces to insert the CSV data for a record.
    -c, --new-column <name>    Put the fetched values in a new column instead.
    --jql <selector>           Apply jql selector to API returned JSON value.
    --jqlfile <file>           Load jql selector from file instead.
    --pretty                   Prettify JSON responses. Otherwise, they're minified.
                               If the response is not in JSON format, it's passed through.
    --rate-limit <qps>         Rate Limit in Queries Per Second. [default: 10]
    --http-header <key:value>  Pass custom header(s) to the server.
    --store-error              On error, store error code/message instead of blank value.
    --cookies                  Allow cookies.
    --redis                    Use Redis to cache responses.

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
    flag_url_template: Option<String>,
    flag_new_column: Option<String>,
    flag_jql: Option<String>,
    flag_jqlfile: Option<String>,
    flag_pretty: bool,
    flag_rate_limit: Option<u32>,
    flag_http_header: Vec<String>,
    flag_store_error: bool,
    flag_cookies: bool,
    flag_redis: bool,
    flag_output: Option<String>,
    flag_no_headers: bool,
    flag_delimiter: Option<Delimiter>,
    flag_quiet: bool,
    arg_column: SelectColumns,
    arg_input: Option<String>,
}

static DEFAULT_REDIS_CONN_STR: &str = "redis://127.0.0.1:6379";
static DEFAULT_REDIS_TTL_SECONDS: u64 = 60 * 60 * 24 * 28; // 28 days in seconds

struct RedisConfig {
    conn_str: String,
    ttl_secs: u64,
    ttl_refresh: bool,
}
impl RedisConfig {
    fn load() -> Self {
        Self {
            conn_str: std::env::var("QSV_REDIS_CONNECTION_STRING")
                .unwrap_or_else(|_| DEFAULT_REDIS_CONN_STR.to_string()),
            ttl_secs: std::env::var("QSV_REDIS_TTL_SECS")
                .unwrap_or_else(|_| DEFAULT_REDIS_TTL_SECONDS.to_string())
                .parse()
                .unwrap(),
            ttl_refresh: std::env::var("QSV_REDIS_TTL_REFRESH").is_ok(),
        }
    }
}

static REDISCONFIG: Lazy<RedisConfig> = Lazy::new(RedisConfig::load);

static JQL_GROUPS: once_cell::sync::OnceCell<Vec<jql::Group>> = OnceCell::new();

#[derive(Error, Debug, PartialEq, Clone)]
enum FetchRedisError {
    #[error("error with redis cache `{0}`")]
    RedisError(String),
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;

    let mut rconfig = Config::new(&args.arg_input)
        .delimiter(args.flag_delimiter)
        .no_headers(args.flag_no_headers);

    let mut rdr = rconfig.reader()?;
    let mut wtr = Config::new(&args.flag_output).writer()?;

    let mut headers = rdr.byte_headers()?.clone();

    let mut column_index = 0_usize;
    if args.flag_url_template.is_none() {
        rconfig = rconfig.select(args.arg_column);
        let sel = rconfig.selection(&headers)?;
        column_index = *sel.iter().next().unwrap();
        if sel.len() != 1 {
            return fail!("Only one single URL column may be selected.");
        }
    }

    let str_headers = rdr.headers()?.clone();

    let mut dynfmt_fields = Vec::with_capacity(10); // 10 is a reasonable default to save allocs
    let mut dynfmt_url_template = String::from("");
    if let Some(ref url_template) = args.flag_url_template {
        if args.flag_no_headers {
            return fail!("--url-template option requires headers.");
        } else {
            dynfmt_url_template = url_template.to_string();
            // first, get the fields used in the url template
            let safe_headers = util::safe_header_names(&str_headers, false);
            let formatstr_re: &'static Regex = regex_once_cell!(r"\{(?P<key>\w+)?\}");
            for format_fields in formatstr_re.captures_iter(url_template) {
                dynfmt_fields.push(format_fields.name("key").unwrap().as_str());
            }
            // we sort the fields so we can do binary_search
            dynfmt_fields.sort_unstable();
            // now, get the indices of the columns for the lookup vec
            for (i, field) in safe_headers.into_iter().enumerate() {
                if dynfmt_fields.binary_search(&field.as_str()).is_ok() {
                    let field_with_curly = format!("{{{field}}}");
                    let field_index = format!("{{{i}}}");
                    dynfmt_url_template =
                        dynfmt_url_template.replace(&field_with_curly, &field_index);
                }
            }
            debug!("dynfmt_fields: {dynfmt_fields:?}  url_template: {dynfmt_url_template}");
        }
    }

    use std::num::NonZeroU32;
    // default rate limit is actually set via docopt, so below init is just to satisfy compiler
    let mut rate_limit: NonZeroU32 = NonZeroU32::new(10).unwrap();
    if let Some(qps) = args.flag_rate_limit {
        // on my laptop, no more sleep traces with qps > 24, so use round number of 20 as single-thread qps limit
        if !(qps <= 20 && qps > 0) {
            return fail!("Rate Limit should be between 1 to 20 queries per second.");
        }
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
        .user_agent(util::DEFAULT_USER_AGENT)
        .default_headers(http_headers)
        .cookie_store(args.flag_cookies)
        .brotli(true)
        .gzip(true)
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
    let mut record_count = 0;
    if args.flag_quiet {
        progress.set_draw_target(ProgressDrawTarget::hidden());
    } else {
        record_count = util::count_rows(&rconfig);
        util::prep_progress(&progress, record_count);
    }

    let not_quiet = !args.flag_quiet;

    #[allow(unused_assignments)]
    let mut record = csv::ByteRecord::new();
    #[allow(unused_assignments)]
    let mut url = String::default();
    let mut redis_cache_hits: u64 = 0;
    #[allow(unused_assignments)]
    let mut final_value = String::default();

    let mut jql_selector: Option<String> = None;
    if let Some(jql_file) = args.flag_jqlfile {
        jql_selector = Some(fs::read_to_string(jql_file)?);
    } else if let Some(ref jql) = args.flag_jql {
        jql_selector = Some(jql.to_string());
    }

    while rdr.read_byte_record(&mut record)? {
        if not_quiet {
            progress.inc(1);
        }

        url.clear();
        if args.flag_url_template.is_some() {
            let mut record_vec: Vec<String> = Vec::with_capacity(record.len());
            for field in &record {
                let str_value = unsafe { std::str::from_utf8_unchecked(field).trim().to_string() };
                record_vec.push(str_value);
            }
            if let Ok(formatted) =
                dynfmt::SimpleCurlyFormat.format(&dynfmt_url_template, record_vec)
            {
                url = formatted.to_string();
            }
        } else if let Ok(s) = std::str::from_utf8(&record[column_index]) {
            url = s.trim().to_string();
        } else {
            final_value = "Invalid URL".to_string();
        }

        debug!("Fetching URL: {url:?}");
        if !url.is_empty() {
            if args.flag_redis {
                let intermediate_value = get_redis_response(
                    &url,
                    &client,
                    &limiter,
                    &jql_selector,
                    args.flag_store_error,
                    args.flag_pretty,
                )
                .unwrap();
                final_value = intermediate_value.to_string();
                if intermediate_value.was_cached {
                    redis_cache_hits += 1;
                }
            } else {
                final_value = get_cached_response(
                    &url,
                    &client,
                    &limiter,
                    &jql_selector,
                    args.flag_store_error,
                    args.flag_pretty,
                );
            }
        }

        if include_existing_columns {
            record.push_field(final_value.as_bytes());
            wtr.write_byte_record(&record)?;
        } else {
            let mut output_record = csv::ByteRecord::new();
            output_record.push_field(final_value.as_bytes());
            wtr.write_byte_record(&output_record)?;
        }
    }

    if not_quiet {
        if args.flag_redis {
            util::update_cache_info!(progress, redis_cache_hits, record_count);
        } else {
            util::update_cache_info!(progress, GET_CACHED_RESPONSE);
        }
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
    convert = r#"{ format!("{}{:?}", url, flag_jql) }"#,
    sync_writes = false
)]
fn get_cached_response(
    url: &str,
    client: &reqwest::blocking::Client,
    limiter: &governor::RateLimiter<NotKeyed, InMemoryState, DefaultClock, NoOpMiddleware>,
    flag_jql: &Option<String>,
    flag_store_error: bool,
    flag_pretty: bool,
) -> String {
    get_response(
        url,
        client,
        limiter,
        flag_jql,
        flag_store_error,
        flag_pretty,
    )
}

#[io_cached(
    type = "cached::RedisCache<String, String>",
    key = "String",
    convert = r#"{ format!("{}{:?}", url, flag_jql) }"#,
    create = r##" {
        RedisCache::new("f", REDISCONFIG.ttl_secs)
            .set_namespace("q")
            .set_refresh(REDISCONFIG.ttl_refresh)
            .set_connection_string(&REDISCONFIG.conn_str)
            .build()
            .expect("error building redis cache")
    } "##,
    map_error = r##"|e| FetchRedisError::RedisError(format!("{:?}", e))"##,
    with_cached_flag = true
)]
fn get_redis_response(
    url: &str,
    client: &reqwest::blocking::Client,
    limiter: &governor::RateLimiter<NotKeyed, InMemoryState, DefaultClock, NoOpMiddleware>,
    flag_jql: &Option<String>,
    flag_store_error: bool,
    flag_pretty: bool,
) -> Result<cached::Return<String>, FetchRedisError> {
    Ok(Return::new(get_response(
        url,
        client,
        limiter,
        flag_jql,
        flag_store_error,
        flag_pretty,
    )))
}

fn get_response(
    url: &str,
    client: &reqwest::blocking::Client,
    limiter: &governor::RateLimiter<NotKeyed, InMemoryState, DefaultClock, NoOpMiddleware>,
    flag_jql: &Option<String>,
    flag_store_error: bool,
    flag_pretty: bool,
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
            error!("Cannot fetch url: {url:?}, error: {error:?}");
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
            // instead of repeatedly parsing the jql selector,
            // we compile it only once and cache it for performance using once_cell
            let jql_groups = JQL_GROUPS.get_or_init(|| jql::selectors_parser(selectors).unwrap());
            match apply_jql(&api_value, jql_groups) {
                Ok(s) => {
                    final_value = s;
                }
                Err(e) => {
                    error!(
                        "jql error. json: {api_value:?}, selectors: {selectors:?}, error: {e:?}"
                    );

                    if flag_store_error {
                        final_value = e.to_string();
                    } else {
                        final_value = String::default();
                    }
                }
            }
        } else if flag_pretty {
            if let Ok(pretty_json) = jsonxf::pretty_print(&api_value) {
                final_value = pretty_json;
            } else {
                final_value = api_value;
            }
        } else if let Ok(minimized_json) = jsonxf::minimize(&api_value) {
            final_value = minimized_json;
        } else {
            final_value = api_value;
        }
    }

    debug!("final value: {final_value}");

    final_value
}

use jql::groups_walker;
use serde_json::{Deserializer, Value};

use anyhow::{anyhow, Result};

fn apply_jql(json: &str, groups: &[jql::Group]) -> Result<String> {
    // check if api returned valid JSON before applying JQL selector
    if let Err(error) = serde_json::from_str::<Value>(json) {
        return Err(anyhow!("Invalid json: {error:?}"));
    }

    let mut result: Result<String> = Ok(String::default());

    Deserializer::from_str(json)
        .into_iter::<Value>()
        .for_each(|value| match value {
            Ok(valid_json) => {
                // Walk through the JSON content with the provided selectors as
                // input.
                match groups_walker(&valid_json, groups) {
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
                result = Err(anyhow!("Invalid json: {error:?}"));
            }
        });

    result
}

#[test]
fn test_apply_jql_invalid_json() {
    let json =
        r#"<!doctype html><html lang="en"><meta charset=utf-8><title>shortest html5</title>"#;
    let selectors = r#"."places"[0]."place name""#;

    let jql_groups = jql::selectors_parser(selectors).unwrap();
    let value: String = apply_jql(json, &jql_groups).unwrap_err().to_string();

    assert_eq!(
        "Invalid json: Error(\"expected value\", line: 1, column: 1)",
        value
    );
}

#[test]
fn test_apply_jql_invalid_selector() {
    let json = r#"{"post code": "90210", "country": "United States", "country abbreviation": "US", "places": [{"place name": "Beverly Hills", "longitude": "-118.4065", "state": "California", "state abbreviation": "CA", "latitude": "34.0901"}]}"#;
    let selectors = r#"."place"[0]."place name""#;

    let jql_groups = jql::selectors_parser(selectors).unwrap();
    let value = apply_jql(json, &jql_groups).unwrap_err().to_string();

    assert_eq!("Node \"place\" not found on the parent element", value);
}

#[test]
fn test_apply_jql_string() {
    let json = r#"{"post code": "90210", "country": "United States", "country abbreviation": "US", "places": [{"place name": "Beverly Hills", "longitude": "-118.4065", "state": "California", "state abbreviation": "CA", "latitude": "34.0901"}]}"#;
    let selectors = r#"."places"[0]."place name""#;

    let jql_groups = jql::selectors_parser(selectors).unwrap();
    let value: String = apply_jql(json, &jql_groups).unwrap();

    assert_eq!("Beverly Hills", value);
}

#[test]
fn test_apply_jql_number() {
    let json = r#"{"post code": "90210", "country": "United States", "country abbreviation": "US", "places": [{"place name": "Beverly Hills", "longitude": -118.4065, "state": "California", "state abbreviation": "CA", "latitude": 34.0901}]}"#;
    let selectors = r#"."places"[0]."longitude""#;

    let jql_groups = jql::selectors_parser(selectors).unwrap();
    let value: String = apply_jql(json, &jql_groups).unwrap();

    assert_eq!("-118.4065", value);
}

#[test]
fn test_apply_jql_bool() {
    let json = r#"{"post code": "90210", "country": "United States", "country abbreviation": "US", "places": [{"place name": "Beverly Hills", "longitude": -118.4065, "state": "California", "state abbreviation": "CA", "latitude": 34.0901, "expensive": true}]}"#;
    let selectors = r#"."places"[0]."expensive""#;

    let jql_groups = jql::selectors_parser(selectors).unwrap();
    let value: String = apply_jql(json, &jql_groups).unwrap();

    assert_eq!("true", value);
}

#[test]
fn test_apply_jql_null() {
    let json = r#"{"post code": "90210", "country": "United States", "country abbreviation": "US", "places": [{"place name": "Beverly Hills", "longitude": -118.4065, "state": "California", "state abbreviation": "CA", "latitude": 34.0901, "university":null}]}"#;
    let selectors = r#"."places"[0]."university""#;

    let jql_groups = jql::selectors_parser(selectors).unwrap();
    let value: String = apply_jql(json, &jql_groups).unwrap();

    assert_eq!("null", value);
}

#[test]
fn test_apply_jql_array() {
    let json = r#"{"post code": "90210", "country": "United States", "country abbreviation": "US", "places": [{"place name": "Beverly Hills", "longitude": -118.4065, "state": "California", "state abbreviation": "CA", "latitude": 34.0901}]}"#;
    let selectors = r#"."places"[0]."longitude",."places"[0]."latitude""#;

    let jql_groups = jql::selectors_parser(selectors).unwrap();
    let value: String = apply_jql(json, &jql_groups).unwrap();

    assert_eq!("-118.4065, 34.0901", value);
}

#[test]
fn test_root_out_of_bounds() {
    // test for out_of_bounds root element handling
    // see https://github.com/yamafaktory/jql/issues/129
    let json = r#"[{"page":1,"pages":1,"per_page":"50","total":1},[{"id":"BRA","iso2Code":"BR","name":"Brazil","region":{"id":"LCN","iso2code":"ZJ","value":"Latin America & Caribbean (all income levels)"},"adminregion":{"id":"LAC","iso2code":"XJ","value":"Latin America & Caribbean (developing only)"},"incomeLevel":{"id":"UMC","iso2code":"XT","value":"Upper middle income"},"lendingType":{"id":"IBD","iso2code":"XF","value":"IBRD"},"capitalCity":"Brasilia","longitude":"-47.9292","latitude":"-15.7801"}]]"#;
    let selectors = r#"[2].[0]."incomeLevel"."value"'"#;

    let jql_groups = jql::selectors_parser(selectors).unwrap();
    let value = apply_jql(json, &jql_groups).unwrap_err().to_string();

    assert_eq!(
        "Index [2] is out of bound, root element has a length of 2",
        value
    );
}
