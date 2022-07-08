#![allow(dead_code)]
use crate::config::{Config, Delimiter};
use crate::select::SelectColumns;
use crate::CliError;
use crate::CliResult;
use crate::{regex_once_cell, util};
use cached::proc_macro::{cached, io_cached};
use cached::{RedisCache, Return};
use dynfmt::Format;
use governor::{
    clock::DefaultClock, middleware::NoOpMiddleware, state::direct::NotKeyed, state::InMemoryState,
};
use indicatif::{ProgressBar, ProgressDrawTarget};
use log::{debug, error, info};
use once_cell::sync::{Lazy, OnceCell};
use rand::Rng;
use redis;
use regex::Regex;
use serde::Deserialize;
use serde_json::json;
use std::fs;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::{thread, time};
use url::Url;

// NOTE: when using the examples with jql, DO NOT USE the example here as rendered in
// source code, use the example as rendered by "qsv fetch --help".
// the source code below has addl escape characters for the jql examples,
// so cutting and pasting it into the command line will not work.
static USAGE: &str = r#"
Fetches HTML/data from web pages or web services for every row.

Fetch is integrated with `jql` to directly parse out values from an API JSON response.

The URL column needs to be a fully qualified URL path. Alternatively, you can dynamically
construct URLs for each CSV record with the --url-template option (see Examples below).

To use a proxy, please set env vars HTTP_PROXY and HTTPS_PROXY
(e.g. export HTTPS_PROXY=socks5://127.0.0.1:1086).

Fetch caches responses to minimize traffic and maximize performance. By default, it uses
a non-persistent memoized cache for each fetch session.

For persistent, inter-session caching, Redis is supported with the --redis flag. 
By default, it will connect to a local Redis instance at redis://127.0.0.1:6379,
with a cache expiry Time-to-Live (TTL) of 2,419,200 seconds (28 days),
and cache hits NOT refreshing the TTL of cached values.

Set the env vars QSV_REDIS_CONNECTION_STRING, QSV_REDIS_TTL_SECONDS and 
QSV_REDIS_TTL_REFRESH to change default Redis settings.

EXAMPLES USING THE COLUMN ARGUMENT:

data.csv
  URL
  https://api.zippopotam.us/us/90210
  https://api.zippopotam.us/us/94105
  https://api.zippopotam.us/us/92802

Given the data.csv above, fetch the JSON response.

  $ qsv fetch URL data.csv 

Note the output will be a JSONL file - with a minified JSON response per line,
not a CSV file.

Now, if we want to generate a CSV file with the parsed City and State, we use the 
new-column and jql options.

$ qsv fetch URL --new-column CityState --jql '"places"[0]."place name","places"[0]."state abbreviation"' 
  data.csv > datatest.csv

data_with_CityState.csv
  URL, CityState,
  https://api.zippopotam.us/us/90210, "Beverly Hills, CA"
  https://api.zippopotam.us/us/94105, "San Francisco, CA"
  https://api.zippopotam.us/us/92802, "Anaheim, CA"

As you can see, entering jql selectors can quickly become cumbersome. Alternatively,
the jql selector can be saved and loaded from a file using the --jqlfile option.

  $ qsv fetch URL --new-column CityState --jqlfile places.jql data.csv > datatest.csv

EXAMPLES USING THE --URL-TEMPLATE OPTION:

Geocode addresses in addr_data.csv, pass the latitude and longitude fields and store
the response in a new column called response into enriched_addr_data.csv.

$ qsv fetch --url-template "https://geocode.test/api/lookup.json?lat={latitude}&long={longitude}" 
  addr_data.csv -c response > enriched_addr_data.csv

Geocode addresses in addr_data.csv, pass the "street address" and "zip-code" fields
and use jql to parse CityState from the JSON response into a new column in enriched.csv.
Note how field name non-alphanumeric characters in the url-template were replace with _.

$ qsv fetch --jql '"places"[0]."place name","places"[0]."state abbreviation"' 
  addr_data.csv -c CityState --url-template "https://geocode.test/api/addr.json?addr={street_address}&zip={zip_code}"
  > enriched.csv

USING THE -HTTP-HEADER OPTION:

The --http-header option allows you to append arbitrary key value pairs (a k-v pair is separated by a :) 
to the HTTP header (to authenticate against an API, pass custom header fields, etc.). Note that you can 
pass as many key-value pairs by using --http-header option repeatedly. For example:

$ qsv fetch "https://httpbin.org/get" --http-header " X-Api-Key:TEST_KEY --http-header "X-Api-Secret:ABC123XYZ" data.csv


Usage:
    qsv fetch [<column> | --url-template <template>] [--jql <selector> | --jqlfile <file>] [--http-header <k:v>...] [options] [<input>]

Fetch options:
    --url-template <template>  URL template to use. Use column names enclosed with
                               curly braces to insert the CSV data for a record.
    -c, --new-column <name>    Put the fetched values in a new column instead.
    --jql <selector>           Apply jql selector to API returned JSON value.
    --jqlfile <file>           Load jql selector from file instead.
    --pretty                   Prettify JSON responses. Otherwise, they're minified.
                               If the response is not in JSON format, it's passed through.
    --rate-limit <qps>         Rate Limit in Queries Per Second (max: 1000). Note that fetch
                               dynamically throttles as well based on rate-limit and
                               retry-after response headers.
                               Set to zero (0) to go as fast as possible, automatically
                               down-throttling as required.
                               CAUTION: Only use zero for APIs that use RateLimit headers,
                               otherwise your fetch job may look like a Denial Of Service attack.
                               [default: 25]
    --timeout <seconds>        Timeout for each URL GET. [default: 30 ]
    --http-header <key:value>  Append custom header(s) to the server. Pass multiple key-value pairs
                               by adding this option multiple times, once for each pair.
    --max-errors <count>       Maximum number of errors before aborting.
                               Set to zero (0) to continue despite errors.
                               [default: 0 ]
    --store-error              On error, store error code/message instead of blank value.
    --cookies                  Allow cookies.
    --redis                    Use Redis to cache responses. It connects to "redis://127.0.0.1:6379"
                               with a TTL of 28 days, and a cache hit NOT renewing an entry's TTL.
                               Adjust the QSV_REDIS_CONNECTION_STRING, QSV_REDIS_TTL_SECONDS &
                               QSV_REDIS_TTL_REFRESH respectively to change Redis settings.

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
"#;

#[derive(Deserialize, Debug)]
struct Args {
    flag_url_template: Option<String>,
    flag_new_column: Option<String>,
    flag_jql: Option<String>,
    flag_jqlfile: Option<String>,
    flag_pretty: bool,
    flag_rate_limit: Option<u32>,
    flag_timeout: u64,
    flag_http_header: Vec<String>,
    flag_max_errors: usize,
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
static DEFAULT_REDIS_TTL_SECS: u64 = 60 * 60 * 24 * 28; // 28 days in seconds
static DEFAULT_REDIS_POOL_SIZE: u32 = 20;
static TIMEOUT_SECS: OnceCell<u64> = OnceCell::new();

// prioritize compression schemes. Brotli first, then gzip, then deflate, and * last
static DEFAULT_ACCEPT_ENCODING: &str = "br;q=1.0, gzip;q=0.6, deflate;q=0.4, *;q=0.2";

impl From<reqwest::Error> for CliError {
    fn from(err: reqwest::Error) -> CliError {
        CliError::Other(err.to_string())
    }
}

static GLOBAL_ERROR_COUNT: AtomicUsize = AtomicUsize::new(0);

struct RedisConfig {
    conn_str: String,
    max_pool_size: u32,
    ttl_secs: u64,
    ttl_refresh: bool,
}
impl RedisConfig {
    fn load() -> Self {
        Self {
            conn_str: std::env::var("QSV_REDIS_CONNECTION_STRING")
                .unwrap_or_else(|_| DEFAULT_REDIS_CONN_STR.to_string()),
            max_pool_size: std::env::var("QSV_REDIS_MAX_POOL_SIZE")
                .unwrap_or_else(|_| DEFAULT_REDIS_POOL_SIZE.to_string())
                .parse()
                .unwrap_or(DEFAULT_REDIS_POOL_SIZE),
            ttl_secs: std::env::var("QSV_REDIS_TTL_SECS")
                .unwrap_or_else(|_| DEFAULT_REDIS_TTL_SECS.to_string())
                .parse()
                .unwrap_or(DEFAULT_REDIS_TTL_SECS),
            ttl_refresh: std::env::var("QSV_REDIS_TTL_REFRESH").is_ok(),
        }
    }
}

static REDISCONFIG: Lazy<RedisConfig> = Lazy::new(RedisConfig::load);
static JQL_GROUPS: once_cell::sync::OnceCell<Vec<jql::Group>> = OnceCell::new();

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;

    if args.flag_redis {
        // check if redis connection is valid
        let conn_str = &REDISCONFIG.conn_str;
        let redis_client = redis::Client::open(conn_str.to_string()).unwrap();
        let redis_conn = redis_client.get_connection();
        if redis_conn.is_err() {
            return fail!(format!("Cannot connect to Redis using \"{conn_str}\"."));
        }
    }

    if args.flag_timeout > 3_600 {
        return fail!("Timeout cannot be more than one hour");
    }

    info!("TIMEOUT: {} secs", args.flag_timeout);
    TIMEOUT_SECS.set(args.flag_timeout).unwrap();

    let mut rconfig = Config::new(&args.arg_input)
        .delimiter(args.flag_delimiter)
        .no_headers(args.flag_no_headers);

    let mut rdr = rconfig.reader()?;
    let mut wtr = if args.flag_new_column.is_some() {
        // when adding a new column for the response, the output
        // is a regular CSV file
        Config::new(&args.flag_output).writer()?
    } else {
        // otherwise, the output is a JSONL file. So we need to configure
        // the csv writer so it doesn't double double quote the JSON response
        // and its flexible (i.e. "column counts are different row to row")
        Config::new(&args.flag_output)
            .quote_style(csv::QuoteStyle::Never)
            .flexible(true)
            .writer()?
    };

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

    let mut dynfmt_url_template = String::from("");
    if let Some(ref url_template) = args.flag_url_template {
        if args.flag_no_headers {
            return fail!("--url-template option requires headers.");
        }
        let str_headers = rdr.headers()?.clone();
        let mut dynfmt_fields = Vec::with_capacity(10); // 10 is a reasonable default to save allocs

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
                dynfmt_url_template = dynfmt_url_template.replace(&field_with_curly, &field_index);
            }
        }
        debug!("dynfmt_fields: {dynfmt_fields:?}  url_template: {dynfmt_url_template}");
    }

    use std::num::NonZeroU32;
    let rate_limit = if let Some(qps) = args.flag_rate_limit {
        match qps {
            0 => NonZeroU32::new(u32::MAX).unwrap(),
            1..=1000 => NonZeroU32::new(qps).unwrap(),
            _ => return fail!("Rate Limit should be between 1 to 1000 queries per second."),
        }
    } else {
        // default rate limit is actually set via docopt, so init below is just to satisfy compiler
        NonZeroU32::new(u32::MAX).unwrap()
    };
    info!("RATE LIMIT: {rate_limit}");

    use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

    let http_headers: HeaderMap = {
        let mut map = HeaderMap::with_capacity(args.flag_http_header.len() + 1);
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

        map.append(
            reqwest::header::ACCEPT_ENCODING,
            HeaderValue::from_str(DEFAULT_ACCEPT_ENCODING).unwrap(),
        );
        map
    };

    use reqwest::blocking::Client;

    let client_timeout = time::Duration::from_secs(*TIMEOUT_SECS.get().unwrap_or(&30));
    let client = Client::builder()
        .user_agent(util::DEFAULT_USER_AGENT)
        .default_headers(http_headers)
        .cookie_store(args.flag_cookies)
        .brotli(true)
        .gzip(true)
        .deflate(true)
        .http2_adaptive_window(true)
        .timeout(client_timeout)
        .build()?;

    use governor::{Quota, RateLimiter};

    let limiter =
        RateLimiter::direct(Quota::per_second(rate_limit).allow_burst(NonZeroU32::new(5).unwrap()));

    let include_existing_columns = if let Some(name) = args.flag_new_column {
        // write header with new column
        headers.push_field(name.as_bytes());
        wtr.write_byte_record(&headers)?;
        true
    } else {
        false
    };

    // prep progress bar
    let progress = ProgressBar::new(0);
    let mut record_count = 0;
    if args.flag_quiet {
        progress.set_draw_target(ProgressDrawTarget::hidden());
    } else {
        record_count = util::count_rows(&rconfig)?;
        util::prep_progress(&progress, record_count);
    }
    // do a progress update every 3 seconds
    // for very large jobs, so the job doesn't look frozen
    if record_count > 100_000 {
        progress.enable_steady_tick(3_000);
    }
    let not_quiet = !args.flag_quiet;

    let jql_selector: Option<String> = if let Some(jql_file) = args.flag_jqlfile {
        Some(fs::read_to_string(jql_file)?)
    } else {
        args.flag_jql.as_ref().map(std::string::ToString::to_string)
    };

    // amortize memory allocations
    // why optimize for mem & speed, when we're just doing single-threaded, throttled URL fetches?
    // we still optimize since fetch is backed by a memoized cache
    // (in memory or Redis, when --redis is used),
    // so we want to return responses as fast as possible as we bypass the network fetch
    // with a cache hit
    #[allow(unused_assignments)]
    let mut record = csv::ByteRecord::new();
    #[allow(unused_assignments)]
    let mut output_record = csv::ByteRecord::new();
    #[allow(unused_assignments)]
    let mut url = String::with_capacity(100);
    #[allow(unused_assignments)]
    let mut record_vec: Vec<String> = Vec::with_capacity(headers.len());
    let mut redis_cache_hits: u64 = 0;
    #[allow(unused_assignments)]
    let mut intermediate_value: Return<String> = Return {
        was_cached: false,
        value: String::with_capacity(150),
    };
    #[allow(unused_assignments)]
    let mut final_value = String::with_capacity(150);
    #[allow(unused_assignments)]
    let mut str_value = String::with_capacity(100);

    while rdr.read_byte_record(&mut record)? {
        if not_quiet {
            progress.inc(1);
        }

        if args.flag_url_template.is_some() {
            // we're using a URL template.
            // let's dynamically construct the URL with it
            record_vec.clear();
            for field in &record {
                str_value = unsafe { std::str::from_utf8_unchecked(field).trim().to_owned() };
                record_vec.push(str_value);
            }
            if let Ok(formatted) =
                dynfmt::SimpleCurlyFormat.format(&dynfmt_url_template, &*record_vec)
            {
                url = formatted.into_owned();
            }
        } else if let Ok(s) = std::str::from_utf8(&record[column_index]) {
            // we're not using a URL template,
            // just use the field as is as the URL
            url = s.to_owned();
        }

        final_value = if url.is_empty() {
            "".to_string()
        } else if args.flag_redis {
            intermediate_value = get_redis_response(
                &url,
                &client,
                &limiter,
                &jql_selector,
                args.flag_store_error,
                args.flag_pretty,
                include_existing_columns,
            )
            .unwrap();
            if intermediate_value.was_cached {
                redis_cache_hits += 1;
            }
            intermediate_value.value
        } else {
            get_cached_response(
                &url,
                &client,
                &limiter,
                &jql_selector,
                args.flag_store_error,
                args.flag_pretty,
                include_existing_columns,
            )
        };

        if include_existing_columns {
            record.push_field(final_value.as_bytes());
            wtr.write_byte_record(&record)?;
        } else {
            output_record.clear();
            if final_value.is_empty() {
                output_record.push_field(b"{}");
            } else {
                output_record.push_field(final_value.as_bytes());
            }
            wtr.write_byte_record(&output_record)?;
        }

        if args.flag_max_errors > 0
            && GLOBAL_ERROR_COUNT.load(Ordering::Relaxed) >= args.flag_max_errors
        {
            let abort_msg = format!("{} max errors. Fetch aborted.", args.flag_max_errors);
            info!("{abort_msg}");
            eprintln!("{abort_msg}");
            break;
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

    Ok(wtr.flush()?)
}

// we only need url and flag_jql in the cache key
// as this is an in-memory cache that is only used
// for one qsv session
#[cached(
    size = 2_000_000,
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
    include_existing_columns: bool,
) -> String {
    get_response(
        url,
        client,
        limiter,
        flag_jql,
        flag_store_error,
        flag_pretty,
        include_existing_columns,
    )
}

// get_redis_response needs a longer key as its a persistent cache
// and and the values of flag_store_error, flag_pretty and include_existing_columns
// may change between sessions
#[io_cached(
    type = "cached::RedisCache<String, String>",
    key = "String",
    convert = r#"{ format!("{}{:?}{}{}{}", url, flag_jql, flag_store_error, flag_pretty, include_existing_columns) }"#,
    create = r##" {
        RedisCache::new("f", REDISCONFIG.ttl_secs)
            .set_namespace("q")
            .set_refresh(REDISCONFIG.ttl_refresh)
            .set_connection_string(&REDISCONFIG.conn_str)
            .set_connection_pool_max_size(REDISCONFIG.max_pool_size)
            .build()
            .expect("error building redis cache")
    } "##,
    map_error = r##"|e| CliError::Other(format!("Redis Error: {:?}", e))"##,
    with_cached_flag = true
)]
fn get_redis_response(
    url: &str,
    client: &reqwest::blocking::Client,
    limiter: &governor::RateLimiter<NotKeyed, InMemoryState, DefaultClock, NoOpMiddleware>,
    flag_jql: &Option<String>,
    flag_store_error: bool,
    flag_pretty: bool,
    include_existing_columns: bool,
) -> Result<cached::Return<String>, CliError> {
    Ok(Return::new(get_response(
        url,
        client,
        limiter,
        flag_jql,
        flag_store_error,
        flag_pretty,
        include_existing_columns,
    )))
}

#[inline]
fn get_response(
    url: &str,
    client: &reqwest::blocking::Client,
    limiter: &governor::RateLimiter<NotKeyed, InMemoryState, DefaultClock, NoOpMiddleware>,
    flag_jql: &Option<String>,
    flag_store_error: bool,
    flag_pretty: bool,
    include_existing_columns: bool,
) -> String {
    // validate the URL
    let valid_url = match Url::parse(url) {
        Ok(valid) => valid.to_string(),
        Err(e) => {
            let url_invalid_err = if flag_store_error {
                if include_existing_columns {
                    // the output is a CSV
                    format!("Invalid URL: {e}")
                } else {
                    // the output is a JSONL file, so return the error
                    // in a JSON API compliant format
                    let json_error = json!({
                        "errors": [{
                            "title": "Invalid URL",
                            "detail": e.to_string()
                        }]
                    });
                    format!("{json_error}")
                }
            } else {
                "".to_string()
            };
            debug!("Invalid URL: Store_error: {flag_store_error} - {url_invalid_err}");
            return url_invalid_err;
        }
    };
    info!("Fetching URL: {valid_url}");

    // wait until RateLimiter gives Okay or we timeout
    const MINIMUM_WAIT_MS: u64 = 10;
    const MIN_WAIT: time::Duration = time::Duration::from_millis(MINIMUM_WAIT_MS);
    let mut limiter_total_wait = 0;
    let governor_timeout_ms = unsafe { *TIMEOUT_SECS.get_unchecked() * 1_000 };
    while limiter.check().is_err() {
        limiter_total_wait += MINIMUM_WAIT_MS;
        thread::sleep(MIN_WAIT);
        if limiter_total_wait > governor_timeout_ms {
            info!("rate limit timeout");
            break;
        } else if limiter_total_wait == 1 {
            info!("throttling...");
        }
    }
    if limiter_total_wait > 0 && limiter_total_wait <= governor_timeout_ms {
        info!("throttled for {limiter_total_wait} ms");
    }

    let resp: reqwest::blocking::Response = match client.get(&valid_url).send() {
        Ok(response) => response,
        Err(error) => {
            error!("Cannot fetch url: {valid_url:?}, error: {error:?}");
            if flag_store_error {
                return error.to_string();
            }
            return String::default();
        }
    };
    debug!("response: {resp:?}");

    let api_respheader = resp.headers().clone();
    let api_status = resp.status();
    let api_value: String = resp.text().unwrap_or_default();
    debug!("api value: {api_value}");

    let final_value: String;
    let mut error_flag = false;

    if api_status.is_client_error() || api_status.is_server_error() {
        error!(
            "HTTP error. url: {valid_url:?}, error: {:?}",
            api_status.canonical_reason().unwrap_or("unknown error")
        );
        error_flag = true;

        if flag_store_error {
            final_value = format!(
                "HTTP ERROR {} - {}",
                api_status.as_str(),
                api_status.canonical_reason().unwrap_or("unknown error")
            );
        } else {
            final_value = String::new();
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
                        final_value = String::new();
                    }
                    error_flag = true;
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

    // check if the API has ratelimits and we need to do dynamic throttling to respect the limits or
    // the API status code is not 200 (most likely 503-service not available or 493-too many requests)
    if api_status != 200
        || api_respheader.contains_key("ratelimit-limit")
        || api_respheader.contains_key("x-ratelimit-limit")
    {
        let mut ratelimit_remaining = api_respheader.get("ratelimit-remaining");
        let temp_var = api_respheader.get("x-ratelimit-remaining");
        if temp_var.is_some() {
            ratelimit_remaining = temp_var;
        }
        let mut ratelimit_reset = api_respheader.get("ratelimit-reset");
        let temp_var = api_respheader.get("x-ratelimit-reset");
        if temp_var.is_some() {
            ratelimit_reset = temp_var;
        }
        // some APIs add the "-second" suffix to ratelimit fields
        let mut ratelimit_remaining_sec = api_respheader.get("ratelimit-remaining-second");
        let temp_var = api_respheader.get("x-ratelimit-remaining-second");
        if temp_var.is_some() {
            ratelimit_remaining_sec = temp_var;
        }
        let mut ratelimit_reset_sec = api_respheader.get("ratelimit-reset-second");
        let temp_var = api_respheader.get("x-ratelimit-reset-second");
        if temp_var.is_some() {
            ratelimit_reset_sec = temp_var;
        }
        let retry_after = api_respheader.get("retry-after");

        debug!("api_status:{api_status:?} rate_limit_remaining:{ratelimit_remaining:?} {ratelimit_remaining_sec:?} \
ratelimit_reset:{ratelimit_reset:?} {ratelimit_reset_sec:?} retry_after:{retry_after:?}");

        // if there's a ratelimit_remaining field in the response header, get it
        // otherwise, set remaining to sentinel value 9999
        let mut remaining = ratelimit_remaining.map_or(9999_u64, |ratelimit_remaining| {
            let remaining_str = ratelimit_remaining.to_str().unwrap();
            remaining_str.parse::<u64>().unwrap_or_default()
        });
        if let Some(ratelimit_remaining_sec) = ratelimit_remaining_sec {
            let remaining_sec_str = ratelimit_remaining_sec.to_str().unwrap();
            remaining = remaining_sec_str.parse::<u64>().unwrap_or_default();
        }

        // if there's a ratelimit_reset field in the response header, get it
        // otherwise, set reset to sentinel value 1
        let mut reset = ratelimit_reset.map_or(1_u64, |ratelimit_reset| {
            let reset_str = ratelimit_reset.to_str().unwrap();
            reset_str.parse::<u64>().unwrap_or_default()
        });
        if let Some(ratelimit_reset_sec) = ratelimit_reset_sec {
            let reset_sec_str = ratelimit_reset_sec.to_str().unwrap();
            reset = reset_sec_str.parse::<u64>().unwrap_or_default();
        }
        // if there's a retry_after field in the response header, get it
        // and set reset to it
        if let Some(retry_after) = retry_after {
            let retry_str = retry_after.to_str().unwrap();
            reset = retry_str.parse::<u64>().unwrap_or_default();
        }

        // if there is only one more remaining call per our ratelimit quota or
        // reset is greater than 1, dynamically throttle and sleep for ~reset seconds
        if remaining <= 1 || reset > 1 {
            // we add a small random delta to how long fetch sleeps
            // as we need to add a little jitter as per the spec
            // https://tools.ietf.org/id/draft-polli-ratelimit-headers-00.html#rfc.section.7.5
            let rand_addl_sleep = (reset * 1000) + rand::thread_rng().gen_range(10..30);

            info!(
                "sleeping for {rand_addl_sleep} milliseconds until ratelimit is reset or retry_after has elapsed"
            );

            // sleep for reset seconds + rand_addl_sleep milliseconds
            thread::sleep(time::Duration::from_millis(rand_addl_sleep));
        }
    }

    if error_flag {
        GLOBAL_ERROR_COUNT.fetch_add(1, Ordering::SeqCst);
    }

    if include_existing_columns {
        final_value
    } else if final_value.starts_with("HTTP ERROR ") && flag_store_error {
        let json_error = json!({
            "errors": [{
                "title": "HTTP ERROR",
                "detail": final_value
            }]
        });
        format!("{json_error}")
    } else {
        final_value
    }
}

use jql::groups_walker;
use serde_json::{Deserializer, Value};

use anyhow::{anyhow, Result};

#[inline]
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
