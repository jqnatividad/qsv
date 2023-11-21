#![allow(unused_assignments)]
static USAGE: &str = r#"
Fetchpost fetches data from web services for every row using HTTP Post.
As opposed to fetch, which uses HTTP Get.

Fetchpost is integrated with `jql` to directly parse out values from an API JSON response.

<url-column> needs to be a fully qualified URL path. It can be specified as a column name
from which the URL value will be retrieved for each record, or as the URL literal itself.

To use a proxy, please set env vars HTTP_PROXY, HTTPS_PROXY or ALL_PROXY
(e.g. export HTTPS_PROXY=socks5://127.0.0.1:1086).

Fetchpost caches responses to minimize traffic and maximize performance. By default, it uses
a non-persistent memoized cache for each fetchpost session.

For persistent, inter-session caching, Redis is supported with the --redis flag. 
By default, it will connect to a local Redis instance at redis://127.0.0.1:6379/2,
with a cache expiry Time-to-Live (TTL) of 2,419,200 seconds (28 days),
and cache hits NOT refreshing the TTL of cached values.

Note that the default values are the same as the fetch command, except fetchpost creates the
cache at database 2, as opposed to database 1 with fetch.

If you don't want responses to be cached, use the --no-cache flag.

Set the environment variables QSV_FP_REDIS_CONNSTR, QSV_FP_REDIS_TTL_SECONDS and 
QSV_FP_REDIS_TTL_REFRESH respectively to change default Redis settings.

Supports brotli, gzip and deflate automatic decompression for improved throughput and
performance, preferring brotli over gzip over deflate.

Gzip compression of requests bodies is supported with the --compress flag. Note that
public APIs typically do not support gzip compression of request bodies because of the
"zip bomb" vulnerability. This option should only be used with private APIs where this
is not a concern.

Automatically upgrades its connection to HTTP/2 with adaptive flow control as well
if the server supports it.
See https://www.cloudflare.com/learning/performance/http2-vs-http1.1/ and
https://medium.com/coderscorner/http-2-flow-control-77e54f7fd518 for more info.

EXAMPLES:

data.csv
  URL, zipcode, country
  https://httpbin.org/post, 90210, USA
  https://httpbin.org/post, 94105, USA
  https://httpbin.org/post, 92802, USA

Given the data.csv above, fetch the JSON response.

  $ qsv fetchpost URL zipcode,country data.csv 

Note the output will be a JSONL file - with a minified JSON response per line, not a CSV file.

Now, if we want to generate a CSV file with a parsed response - getting only the "form" property,
we use the new-column and jql options. (See https://github.com/yamafaktory/jql#%EF%B8%8F-usage 
for more info on how to use the jql JSON Query Language)

$ qsv fetchpost URL zipcode,country --new-column form --jql '"form"' data.csv > data_with_response.csv

data_with_response.csv
  URL,zipcode,country,form
  https://httpbin.org/post,90210,USA,"{""country"": String(""USA""), ""zipcode"": String(""90210"")}"
  https://httpbin.org/post,94105,USA,"{""country"": String(""USA""), ""zipcode"": String(""94105"")}"
  https://httpbin.org/post,92802,USA,"{""country"": String(""USA""), ""zipcode"": String(""92802"")}"

Alternatively, since we're using the same URL for all the rows, we can just pass the url directly on the command-line.

  $ qsv fetchpost https://httpbin.org/post 2,3 --new-column form --jqlfile form.jql data.csv > data_with_formdata.csv

Also note that for the column-list argument, we used the column index (2,3 for second & third column)
instead of using the column names, and we loaded the jql selector from the form.jql file.

The form.jql file simply contains the string literal "form", including the enclosing double quotes:

form.jql
  "form"

USING THE HTTP-HEADER OPTION:

The --http-header option allows you to append arbitrary key value pairs (a valid pair is a key and value
separated by a colon) to the HTTP header (to authenticate against an API, pass custom header fields, etc.).
Note that you can pass as many key-value pairs by using --http-header option repeatedly. For example:

$ qsv fetchpost https://httpbin.org/post col1-col3 data.csv -H "X-Api-Key:TEST_KEY" -H "X-Api-Secret:ABC123XYZ"

For more extensive examples, see https://github.com/jqnatividad/qsv/blob/master/tests/test_fetch.rs.

Usage:
    qsv fetchpost (<url-column> <column-list>) [--jql <selector> | --jqlfile <file>] [--http-header <k:v>...] [options] [<input>]
    qsv fetchpost --help

Fetchpost options:
    <url-column>               Name of the column with the URL.
                               Otherwise, if the argument starts with `http`, the URL to use.
    <column-list>              Comma-delimited list of columns to insert into the HTTP Post body.
                               Uses `qsv select` syntax - i.e. Columns can be referenced by index or 
                               by name if there is a header row (duplicate column names can be disambiguated
                               with more indexing). Column ranges can also be specified. Finally, columns
                               can be selected using regular expressions.
                               See 'qsv select --help' for examples.
    -c, --new-column <name>    Put the fetched values in a new column. Specifying this option
                               results in a CSV. Otherwise, the output is in JSONL format.
    --jql <selector>           Apply jql selector to API returned JSON response.
                               Mutually exclusive with --jqlfile.
    --jqlfile <file>           Load jql selector from file instead.
                               Mutually exclusive with --jql.
    --pretty                   Prettify JSON responses. Otherwise, they're minified.
                               If the response is not in JSON format, it's passed through unchanged.
                               Note that --pretty requires the --new-column option.
    --rate-limit <qps>         Rate Limit in Queries Per Second (max: 1000). Note that fetch
                               dynamically throttles as well based on rate-limit and
                               retry-after response headers.
                               Set to 0 to go as fast as possible, automatically throttling as required.
                               CAUTION: Only use zero for APIs that use RateLimit and/or Retry-After headers,
                               otherwise your fetchpost job may look like a Denial Of Service attack.
                               Even though zero is the default, this is mitigated by --max-errors having a
                               default of 10.
                               [default: 0 ]
    --timeout <seconds>        Timeout for each URL request.
                               [default: 30 ]
    -H, --http-header <k:v>    Append custom header(s) to the HTTP header. Pass multiple key-value pairs
                               by adding this option multiple times, once for each pair. The key and value 
                               should be separated by a colon.
    --compress                 Compress the HTTP request body using gzip. Note that most servers do not support
                               compressed request bodies unless they are specifically configured to do so. This
                               should only be enabled for trusted scenarios where "zip bombs" are not a concern.
                               see https://github.com/postmanlabs/httpbin/issues/577#issuecomment-875814469
                               for more info.
    --max-retries <count>      Maximum number of retries per record before an error is raised.
                               [default: 5]
    --max-errors <count>       Maximum number of errors before aborting.
                               Set to zero (0) to continue despite errors.
                               [default: 10 ]
    --store-error              On error, store error code/message instead of blank value.
    --no-cache                 Do not cache responses.
    --cache-error              Cache error responses even if a request fails. If an identical URL is requested,
                               the cached error is returned. Otherwise, the fetch is attempted again
                               for --max-retries.
    --cookies                  Allow cookies.
    --user-agent <agent>       Specify custom user agent. It supports the following variables -
                               $QSV_VERSION, $QSV_TARGET, $QSV_BIN_NAME, $QSV_KIND and $QSV_COMMAND.
                               Try to follow the syntax here -
                               https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/User-Agent
    --report <d|s>             Creates a report of the fetchpost job. The report has the same name as the
                               input file with the ".fetchpost-report" suffix. 
                               There are two kinds of report - d for "detailed" & s for "short". The detailed
                               report has the same columns as the input CSV with seven additional columns - 
                               qsv_fetchp_url, qsv_fetchp_form, qsv_fetchp_status, qsv_fetchp_cache_hit,
                               qsv_fetchp_retries, qsv_fetchp_elapsed_ms & qsv_fetchp_response.
                               The short report only has the seven columns without the "qsv_fetchp_" prefix.
                               [default: none]
    --redis                    Use Redis to cache responses. It connects to "redis://127.0.0.1:6379/2"
                               with a connection pool size of 20, with a TTL of 28 days, and a cache hit 
                               NOT renewing an entry's TTL.
                               Adjust the QSV_FP_REDIS_CONNSTR, QSV_REDIS_MAX_POOL_SIZE, QSV_REDIS_TTL_SECONDS & 
                               QSV_REDIS_TTL_REFRESH respectively to change Redis settings.
    --flushdb                  Flush all the keys in the current Redis database on startup.
                               This option is ignored if the --redis option is NOT enabled.

Common options:
    -h, --help                 Display this message
    -o, --output <file>        Write output to <file> instead of stdout.
    -n, --no-headers           When set, the first row will not be interpreted
                               as headers. Namely, it will be sorted with the rest
                               of the rows. Otherwise, the first row will always
                               appear as the header row in the output.
    -d, --delimiter <arg>      The field delimiter for reading CSV data.
                               Must be a single character. (default: ,)
    -p, --progressbar          Show progress bars. Will also show the cache hit rate upon completion.
                               Not valid for stdin.
"#;

use std::{fs, io::Write, num::NonZeroU32, sync::OnceLock, thread, time};

use cached::{
    proc_macro::{cached, io_cached},
    Cached, IOCached, RedisCache, Return,
};
use flate2::{write::GzEncoder, Compression};
use governor::{
    clock::DefaultClock,
    middleware::NoOpMiddleware,
    state::{direct::NotKeyed, InMemoryState},
    Quota, RateLimiter,
};
use indicatif::{HumanCount, MultiProgress, ProgressBar, ProgressDrawTarget};
use log::{
    debug, error, info, log_enabled, warn,
    Level::{Debug, Info, Trace, Warn},
};
use rand::Rng;
use redis;
use regex::Regex;
use reqwest::{
    blocking::Client,
    header::{HeaderMap, HeaderName, HeaderValue},
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use serde_urlencoded;
use simdutf8::basic::from_utf8;
use url::Url;

use crate::{
    cmd::fetch::process_jql,
    config::{Config, Delimiter},
    select::SelectColumns,
    util, CliError, CliResult,
};

#[derive(Deserialize)]
struct Args {
    flag_new_column:  Option<String>,
    flag_jql:         Option<String>,
    flag_jqlfile:     Option<String>,
    flag_pretty:      bool,
    flag_rate_limit:  u32,
    flag_timeout:     u16,
    flag_http_header: Vec<String>,
    flag_compress:    bool,
    flag_max_retries: u8,
    flag_max_errors:  u64,
    flag_store_error: bool,
    flag_no_cache:    bool,
    flag_cache_error: bool,
    flag_cookies:     bool,
    flag_user_agent:  Option<String>,
    flag_report:      String,
    flag_redis:       bool,
    flag_flushdb:     bool,
    flag_output:      Option<String>,
    flag_no_headers:  bool,
    flag_delimiter:   Option<Delimiter>,
    flag_progressbar: bool,
    arg_url_column:   SelectColumns,
    arg_column_list:  SelectColumns,
    arg_input:        Option<String>,
}

// connect to Redis at localhost, using database 2 by default when --redis is enabled
static DEFAULT_FP_REDIS_CONN_STR: &str = "redis://127.0.0.1:6379/2";
static DEFAULT_FP_REDIS_TTL_SECS: u64 = 60 * 60 * 24 * 28; // 28 days in seconds
static DEFAULT_FP_REDIS_POOL_SIZE: u32 = 20;
static TIMEOUT_FP_SECS: OnceLock<u64> = OnceLock::new();

const FETCHPOST_REPORT_PREFIX: &str = "qsv_fetchp_";
const FETCHPOST_REPORT_SUFFIX: &str = ".fetchpost-report.tsv";

// prioritize decompression schemes. Brotli first, then gzip, then deflate, and * last
static DEFAULT_ACCEPT_ENCODING: &str = "br;q=1.0, gzip;q=0.6, deflate;q=0.4, *;q=0.2";

// for governor/ratelimiter
const MINIMUM_WAIT_MS: u64 = 10;
const MIN_WAIT: time::Duration = time::Duration::from_millis(MINIMUM_WAIT_MS);

// for --report option
#[derive(PartialEq)]
enum ReportKind {
    Detailed,
    Short,
    None,
}

#[derive(Debug)]
struct RedisConfig {
    conn_str:      String,
    max_pool_size: u32,
    ttl_secs:      u64,
    ttl_refresh:   bool,
}
impl RedisConfig {
    fn new() -> RedisConfig {
        Self {
            conn_str:      std::env::var("QSV_FP_REDIS_CONNSTR")
                .unwrap_or_else(|_| DEFAULT_FP_REDIS_CONN_STR.to_string()),
            max_pool_size: std::env::var("QSV_REDIS_MAX_POOL_SIZE")
                .unwrap_or_else(|_| DEFAULT_FP_REDIS_POOL_SIZE.to_string())
                .parse()
                .unwrap_or(DEFAULT_FP_REDIS_POOL_SIZE),
            ttl_secs:      std::env::var("QSV_REDIS_TTL_SECS")
                .unwrap_or_else(|_| DEFAULT_FP_REDIS_TTL_SECS.to_string())
                .parse()
                .unwrap_or(DEFAULT_FP_REDIS_TTL_SECS),
            ttl_refresh:   util::get_envvar_flag("QSV_REDIS_TTL_REFRESH"),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct FetchResponse {
    response:    String,
    status_code: u16,
    retries:     u8,
}

static REDISCONFIG: OnceLock<RedisConfig> = OnceLock::new();

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;

    TIMEOUT_FP_SECS
        .set(util::timeout_secs(args.flag_timeout)?)
        .unwrap();

    if args.flag_redis {
        // initialize Redis Config
        REDISCONFIG.set(RedisConfig::new()).unwrap();

        // check if redis connection is valid
        let conn_str = &REDISCONFIG.get().unwrap().conn_str;
        let redis_client = match redis::Client::open(conn_str.to_string()) {
            Ok(rc) => rc,
            Err(e) => {
                return fail_incorrectusage_clierror!(
                    r#"Invalid Redis connection string "{conn_str}": {e:?}"#
                )
            },
        };

        let mut redis_conn;
        match redis_client.get_connection() {
            Err(e) => {
                return fail_clierror!(r#"Cannot connect to Redis using "{conn_str}": {e:?}"#)
            },
            Ok(x) => redis_conn = x,
        }

        if args.flag_flushdb {
            redis::cmd("FLUSHDB").execute(&mut redis_conn);
            info!("flushed Redis database.");
        }
    }

    let mut rconfig = Config::new(&args.arg_input)
        .delimiter(args.flag_delimiter)
        .trim(csv::Trim::All)
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

    let include_existing_columns = if let Some(name) = args.flag_new_column {
        // write header with new column
        headers.push_field(name.as_bytes());
        wtr.write_byte_record(&headers)?;
        true
    } else {
        if args.flag_pretty {
            return fail_incorrectusage_clierror!(
                "The --pretty option requires the --new-column option."
            );
        }
        false
    };

    // validate column-list is a list of valid column names
    let cl_config = Config::new(&args.arg_input)
        .delimiter(args.flag_delimiter)
        .trim(csv::Trim::All)
        .no_headers(args.flag_no_headers)
        .select(args.arg_column_list.clone());
    let col_list = cl_config.selection(&headers)?;
    debug!("column-list: {col_list:?}");

    // check if the url_column arg was passed as a URL literal
    // or as a column selector
    let url_column_str = format!("{:?}", args.arg_url_column);
    let re = Regex::new(r"^IndexedName\((.*)\[0\]\)$").unwrap();
    let literal_url =
        if let Some(caps) = re.captures(&url_column_str) {
            caps[1].to_lowercase()
        } else {
            String::new()
        };
    let literal_url_used = literal_url.starts_with("http");

    let mut column_index = 0;
    if !literal_url_used {
        rconfig = rconfig.select(args.arg_url_column);
        let sel = rconfig.selection(&headers)?;
        column_index = *sel.iter().next().unwrap();
        if sel.len() != 1 {
            return fail!("Only a single URL column may be selected.");
        }
    }

    let rate_limit = match args.flag_rate_limit {
        0 => NonZeroU32::new(u32::MAX).unwrap(),
        1..=1000 => NonZeroU32::new(args.flag_rate_limit).unwrap(),
        _ => {
            return fail_incorrectusage_clierror!(
                "Rate Limit should be between 0 to 1000 queries per second."
            )
        },
    };
    info!("RATE LIMIT: {rate_limit}");

    let http_headers: HeaderMap = {
        let mut map = HeaderMap::with_capacity(args.flag_http_header.len() + 1);
        for header in args.flag_http_header {
            let vals: Vec<&str> = header.split(':').collect();

            if vals.len() != 2 {
                return fail_incorrectusage_clierror!(
                    "{vals:?} is not a valid key-value pair. Expecting a key and a value \
                     separated by a colon."
                );
            }

            // allocate new String for header key to put into map
            let k: String = String::from(vals[0].trim());
            let header_name: HeaderName =
                match HeaderName::from_lowercase(k.to_lowercase().as_bytes()) {
                    Ok(h) => h,
                    Err(e) => return fail_incorrectusage_clierror!("Invalid header name: {e}"),
                };

            // allocate new String for header value to put into map
            let v: String = String::from(vals[1].trim());
            let header_val: HeaderValue = match HeaderValue::from_str(v.as_str()) {
                Ok(v) => v,
                Err(e) => return fail_incorrectusage_clierror!("Invalid header value: {e}"),
            };

            map.append(header_name, header_val);
        }

        map.append(
            reqwest::header::ACCEPT_ENCODING,
            HeaderValue::from_str(DEFAULT_ACCEPT_ENCODING).unwrap(),
        );
        map.append(
            reqwest::header::CONTENT_TYPE,
            HeaderValue::from_str("application/x-www-form-urlencoded").unwrap(),
        );
        if args.flag_compress {
            map.append(
                reqwest::header::CONTENT_ENCODING,
                HeaderValue::from_str("gzip").unwrap(),
            );
        }
        map
    };
    debug!("HTTP Header: {http_headers:?}");

    let client_timeout = time::Duration::from_secs(*TIMEOUT_FP_SECS.get().unwrap_or(&30));
    let client = Client::builder()
        .user_agent(util::set_user_agent(args.flag_user_agent)?)
        .default_headers(http_headers)
        .cookie_store(args.flag_cookies)
        .brotli(true)
        .gzip(true)
        .deflate(true)
        .use_rustls_tls()
        .http2_adaptive_window(true)
        .connection_verbose(log_enabled!(Debug) || log_enabled!(Trace))
        .timeout(client_timeout)
        .build()?;

    // set rate limiter with allow_burst set to 1 - see https://github.com/antifuchs/governor/issues/39
    let limiter =
        RateLimiter::direct(Quota::per_second(rate_limit).allow_burst(NonZeroU32::new(1).unwrap()));

    // prep progress bars
    let show_progress =
        (args.flag_progressbar || util::get_envvar_flag("QSV_PROGRESSBAR")) && !rconfig.is_stdin();

    // create multi_progress to stderr with a maximum refresh of 5 per second
    let multi_progress = MultiProgress::with_draw_target(ProgressDrawTarget::stderr_with_hz(5));
    let progress = multi_progress.add(ProgressBar::new(0));
    let mut record_count = 0;

    let error_progress = multi_progress.add(ProgressBar::new(args.flag_max_errors));
    if args.flag_max_errors > 0 && show_progress {
        console::set_colors_enabled(true); // as error progress bar is red
        error_progress.set_style(
            indicatif::ProgressStyle::default_bar()
                .template("{bar:37.red/white} {percent}%{msg} ({per_sec:7})")
                .unwrap(),
        );
        error_progress.set_message(format!(" of {} max errors", HumanCount(args.flag_max_errors)));
    } else {
        error_progress.set_draw_target(ProgressDrawTarget::hidden());
    }

    if show_progress {
        record_count = util::count_rows(&rconfig)?;
        util::prep_progress(&progress, record_count);
    } else {
        multi_progress.set_draw_target(ProgressDrawTarget::hidden());
    }

    let jql_selector: Option<String> = match args.flag_jqlfile {
        Some(ref jql_file) => Some(fs::read_to_string(jql_file)?),
        None => args.flag_jql.as_ref().map(std::string::ToString::to_string),
    };

    // prepare report
    let report = if args.flag_report.to_lowercase().starts_with('d') {
        // if it starts with d, its a detailed report
        ReportKind::Detailed
    } else if args.flag_report.to_lowercase().starts_with('s') {
        // if it starts with s, its a short report
        ReportKind::Short
    } else {
        ReportKind::None
    };

    let mut report_wtr;
    let report_path;
    if report == ReportKind::None {
        // no report, point report_wtr to /dev/null (AKA sink)
        report_wtr = Config::new(&Some("sink".to_string())).writer()?;
        report_path = String::new();
    } else {
        report_path = args
            .arg_input
            .clone()
            .unwrap_or_else(|| "stdin.csv".to_string());

        report_wtr = Config::new(&Some(report_path.clone() + FETCHPOST_REPORT_SUFFIX)).writer()?;
        let mut report_headers =
            if report == ReportKind::Detailed {
                headers.clone()
            } else {
                csv::ByteRecord::new()
            };
        let rptcol_prefix =
            if report == ReportKind::Detailed {
                FETCHPOST_REPORT_PREFIX
            } else {
                ""
            };
        // the fetchpost report has the following columns:
        // url - URL used, form - form data sent, status - HTTP status code,
        // cache_hit - cache hit flag, retries - retry attempts,
        // elapsed - elapsed time (milliseconds) & response.
        let report_header_fields = vec![
            format!("{rptcol_prefix}url"),
            format!("{rptcol_prefix}status"),
            format!("{rptcol_prefix}cache_hit"),
            format!("{rptcol_prefix}retries"),
            format!("{rptcol_prefix}elapsed_ms"),
            format!("{rptcol_prefix}response"),
        ];
        report_headers = csv::ByteRecord::from(report_header_fields);

        report_wtr.write_byte_record(&report_headers)?;
    }

    // amortize memory allocations
    // why optimize for mem & speed, when we're just doing single-threaded, throttled URL fetches?
    // we still optimize since fetch is backed by a memoized cache (in memory or Redis, when --redis
    // is used), so we want to return responses as fast as possible as we bypass the network
    // request with a cache hit
    let mut record = csv::ByteRecord::new();
    let mut jsonl_record = csv::ByteRecord::new();
    let mut report_record = csv::ByteRecord::new();
    let mut url = String::with_capacity(100);
    let mut redis_cache_hits: u64 = 0;
    let mut intermediate_redis_value: Return<String> = Return {
        was_cached: false,
        value:      String::new(),
    };
    let mut intermediate_value: Return<FetchResponse> = Return {
        was_cached: false,
        value:      FetchResponse {
            response:    String::new(),
            status_code: 0_u16,
            retries:     0_u8,
        },
    };
    let mut final_value = String::with_capacity(150);
    let mut final_response =
        FetchResponse {
            response:    String::new(),
            status_code: 0_u16,
            retries:     0_u8,
        };
    let empty_response =
        FetchResponse {
            response:    String::new(),
            status_code: 0_u16,
            retries:     0_u8,
        };
    let mut running_error_count = 0_u64;
    let mut running_success_count = 0_u64;
    let mut was_cached;
    let mut now = time::Instant::now();
    let mut form_body_jsonmap = serde_json::map::Map::with_capacity(col_list.len());

    while rdr.read_byte_record(&mut record)? {
        if show_progress {
            progress.inc(1);
        }

        if report != ReportKind::None {
            now = time::Instant::now();
        };

        // construct body per the column-list
        form_body_jsonmap.clear();
        for col_idx in &*col_list {
            let header_key = String::from_utf8_lossy(headers.get(*col_idx).unwrap());
            let value_string = from_utf8(&record[*col_idx]).unwrap_or_default().to_string();
            form_body_jsonmap.insert(
                header_key.to_string(),
                serde_json::Value::String(value_string),
            );
        }
        if log::log_enabled!(Debug) {
            debug!("{form_body_jsonmap:?}");
        }

        if literal_url_used {
            url = literal_url.clone();
        } else if let Ok(s) = from_utf8(&record[column_index]) {
            s.clone_into(&mut url);
        } else {
            url = String::new();
        }

        if url.is_empty() {
            final_response.clone_from(&empty_response);
            was_cached = false;
        } else if args.flag_redis && !args.flag_no_cache {
            intermediate_redis_value = get_redis_response(
                &url,
                &form_body_jsonmap,
                &client,
                &limiter,
                &jql_selector,
                args.flag_store_error,
                args.flag_pretty,
                args.flag_compress,
                include_existing_columns,
                args.flag_max_retries,
            )?;
            was_cached = intermediate_redis_value.was_cached;
            if was_cached {
                redis_cache_hits += 1;
            }
            final_response = match serde_json::from_str(&intermediate_redis_value) {
                Ok(r) => r,
                Err(e) => {
                    return fail_clierror!(
                        "Cannot deserialize Redis cache value. Try flushing the Redis cache with \
                         --flushdb: {e}"
                    )
                },
            };
            if !args.flag_cache_error && final_response.status_code != 200 {
                let key = format!(
                    "{}{:?}{:?}{}{}{}{}",
                    url,
                    form_body_jsonmap,
                    args.flag_jql,
                    args.flag_store_error,
                    args.flag_pretty,
                    args.flag_compress,
                    include_existing_columns
                );

                if GET_REDIS_RESPONSE.cache_remove(&key).is_err() && log_enabled!(Warn) {
                    // failure to remove cache keys is non-fatal. Continue, but log it.
                    wwarn!(r#"Cannot remove Redis key "{key}""#);
                };
            }
        } else if args.flag_no_cache {
            final_response = get_response(
                &url,
                &form_body_jsonmap,
                &client,
                &limiter,
                &jql_selector,
                args.flag_store_error,
                args.flag_pretty,
                args.flag_compress,
                include_existing_columns,
                args.flag_max_retries,
            );
            was_cached = false;
        } else {
            intermediate_value = get_cached_response(
                &url,
                &form_body_jsonmap,
                &client,
                &limiter,
                &jql_selector,
                args.flag_store_error,
                args.flag_pretty,
                args.flag_compress,
                include_existing_columns,
                args.flag_max_retries,
            );
            final_response = intermediate_value.value;
            was_cached = intermediate_value.was_cached;
            if !args.flag_cache_error && final_response.status_code != 200 {
                let mut cache = GET_CACHED_RESPONSE.lock().unwrap();
                cache.cache_remove(&url);
            }
        };

        if final_response.status_code == 200 {
            running_success_count += 1;
        } else {
            running_error_count += 1;
            error_progress.inc(1);
        }

        final_value.clone_from(&final_response.response);

        if include_existing_columns {
            record.push_field(final_value.as_bytes());
            wtr.write_byte_record(&record)?;
        } else {
            jsonl_record.clear();
            if final_value.is_empty() {
                jsonl_record.push_field(b"{}");
            } else {
                jsonl_record.push_field(final_value.as_bytes());
            }
            wtr.write_byte_record(&jsonl_record)?;
        }

        if report != ReportKind::None {
            if report == ReportKind::Detailed {
                report_record.clone_from(&record);
            } else {
                report_record.clear();
            }
            report_record.push_field(url.as_bytes());
            report_record.push_field(format!("{form_body_jsonmap:?}").as_bytes());
            report_record.push_field(final_response.status_code.to_string().as_bytes());
            report_record.push_field(if was_cached { b"1" } else { b"0" });
            report_record.push_field(final_response.retries.to_string().as_bytes());
            report_record.push_field(now.elapsed().as_millis().to_string().as_bytes());
            if include_existing_columns {
                report_record.push_field(final_value.as_bytes());
            } else {
                report_record.push_field(jsonl_record.as_slice());
            }
            report_wtr.write_byte_record(&report_record)?;
        }

        if args.flag_max_errors > 0 && running_error_count >= args.flag_max_errors {
            break;
        }
    }

    report_wtr.flush()?;

    if show_progress {
        if args.flag_redis {
            util::update_cache_info!(progress, redis_cache_hits, record_count);
        } else {
            util::update_cache_info!(progress, GET_CACHED_RESPONSE);
        }
        util::finish_progress(&progress);

        if running_error_count == 0 {
            error_progress.finish_and_clear();
        } else if running_error_count >= args.flag_max_errors {
            error_progress.finish();
            // sleep so we can dependably write eprintln without messing up progress bars
            thread::sleep(time::Duration::from_nanos(10));
            let abort_msg = format!(
                "{} max errors. Fetchpost aborted.",
                HumanCount(args.flag_max_errors)
            );
            winfo!("{abort_msg}");
        } else {
            error_progress.abandon();
        }
    }

    let mut end_msg = format!(
        "{} records successfully fetchposted as {}. {} errors.",
        HumanCount(running_success_count),
        if include_existing_columns {
            "CSV"
        } else {
            "JSONL"
        },
        HumanCount(running_error_count)
    );
    if report != ReportKind::None {
        use std::fmt::Write;

        write!(
            &mut end_msg,
            " {} report created: \"{}{FETCHPOST_REPORT_SUFFIX}\"",
            if report == ReportKind::Detailed {
                "Detailed"
            } else {
                "Short"
            },
            report_path
        )
        .unwrap();
    }
    winfo!("{end_msg}");

    Ok(wtr.flush()?)
}

// we only need url in the cache key
// as this is an in-memory cache that is only used for one qsv session
#[cached(
    size = 2_000_000,
    key = "String",
    convert = r#"{ format!("{:?}", form_body_jsonmap) }"#,
    with_cached_flag = true
)]
fn get_cached_response(
    url: &str,
    form_body_jsonmap: &serde_json::Map<String, Value>,
    client: &reqwest::blocking::Client,
    limiter: &governor::RateLimiter<NotKeyed, InMemoryState, DefaultClock, NoOpMiddleware>,
    flag_jql: &Option<String>,
    flag_store_error: bool,
    flag_pretty: bool,
    flag_compress: bool,
    include_existing_columns: bool,
    flag_max_retries: u8,
) -> cached::Return<FetchResponse> {
    Return::new(get_response(
        url,
        form_body_jsonmap,
        client,
        limiter,
        flag_jql,
        flag_store_error,
        flag_pretty,
        flag_compress,
        include_existing_columns,
        flag_max_retries,
    ))
}

// get_redis_response needs a longer key as its a persistent cache and the
// values of flag_jql, flag_store_error, flag_pretty and include_existing_columns
// may change between sessions
#[io_cached(
    type = "cached::RedisCache<String, String>",
    key = "String",
    convert = r#"{ format!("{}{:?}{:?}{}{}{}{}", url, form_body_jsonmap, flag_jql, flag_store_error, flag_pretty, flag_compress, include_existing_columns) }"#,
    create = r##" {
        let redis_config = REDISCONFIG.get().unwrap();
        RedisCache::new("fp", redis_config.ttl_secs)
            .set_namespace("q")
            .set_refresh(redis_config.ttl_refresh)
            .set_connection_string(&redis_config.conn_str)
            .set_connection_pool_max_size(redis_config.max_pool_size)
            .build()
            .expect("error building redis cache")
    } "##,
    map_error = r##"|e| CliError::Other(format!("Redis Error: {:?}", e))"##,
    with_cached_flag = true
)]
fn get_redis_response(
    url: &str,
    form_body_jsonmap: &serde_json::Map<String, Value>,
    client: &reqwest::blocking::Client,
    limiter: &governor::RateLimiter<NotKeyed, InMemoryState, DefaultClock, NoOpMiddleware>,
    flag_jql: &Option<String>,
    flag_store_error: bool,
    flag_pretty: bool,
    flag_compress: bool,
    include_existing_columns: bool,
    flag_max_retries: u8,
) -> Result<cached::Return<String>, CliError> {
    Ok(Return::new({
        serde_json::to_string(&get_response(
            url,
            form_body_jsonmap,
            client,
            limiter,
            flag_jql,
            flag_store_error,
            flag_pretty,
            flag_compress,
            include_existing_columns,
            flag_max_retries,
        ))
        .unwrap()
    }))
}

#[allow(clippy::fn_params_excessive_bools)]
#[inline]
fn get_response(
    url: &str,
    form_body_jsonmap: &serde_json::Map<String, Value>,
    client: &reqwest::blocking::Client,
    limiter: &governor::RateLimiter<NotKeyed, InMemoryState, DefaultClock, NoOpMiddleware>,
    flag_jql: &Option<String>,
    flag_store_error: bool,
    flag_pretty: bool,
    flag_compress: bool,
    include_existing_columns: bool,
    flag_max_retries: u8,
) -> FetchResponse {
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
                String::new()
            };
            error!("Invalid URL: Store_error: {flag_store_error} - {url_invalid_err}");
            return FetchResponse {
                response:    url_invalid_err,
                status_code: reqwest::StatusCode::NOT_FOUND.as_u16(),
                retries:     0_u8,
            };
        },
    };
    info!("Using URL: {valid_url}");

    // wait until RateLimiter gives Okay or we timeout
    let mut limiter_total_wait: u64;
    let timeout_secs = *TIMEOUT_FP_SECS.get().unwrap_or(&30_u64);
    let governor_timeout_ms = timeout_secs * 1_000;

    let mut retries = 0_u8;
    let mut error_flag;
    let mut final_value = String::new();
    let mut api_status;
    let mut api_respheader = HeaderMap::new();

    // request with --max-retries
    'retry: loop {
        // check the rate-limiter
        limiter_total_wait = 0;
        while limiter.check().is_err() {
            limiter_total_wait += MINIMUM_WAIT_MS;
            thread::sleep(MIN_WAIT);
            if limiter_total_wait > governor_timeout_ms {
                info!("rate limit timed out after {limiter_total_wait} ms");
                break;
            } else if limiter_total_wait == MINIMUM_WAIT_MS {
                info!("throttling...");
            }
        }
        if log_enabled!(Info) && limiter_total_wait > 0 && limiter_total_wait <= governor_timeout_ms
        {
            info!("throttled for {limiter_total_wait} ms");
        }

        // send the actual request
        let form_body_raw =
            serde_urlencoded::to_string(form_body_jsonmap)
                .unwrap()
                .as_bytes()
                .to_owned();
        let resp_result = if flag_compress {
            // gzip the request body
            let mut gz_enc = GzEncoder::new(Vec::new(), Compression::default());
            gz_enc.write_all(&form_body_raw).unwrap();
            let gzipped_request_body = gz_enc.finish().unwrap();
            client.post(&valid_url).body(gzipped_request_body).send()
        } else {
            client.post(&valid_url).body(form_body_raw).send()
        };

        if let Ok(resp) = resp_result {
            // debug!("{resp:?}");
            api_respheader.clone_from(resp.headers());
            api_status = resp.status();
            let api_value: String = resp.text().unwrap_or_default();

            if api_status.is_client_error() || api_status.is_server_error() {
                error_flag = true;
                error!(
                    "HTTP error. url: {valid_url:?}, error: {:?}",
                    api_status.canonical_reason().unwrap_or("unknown error")
                );

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
                error_flag = false;
                // apply JQL selector if provided
                if let Some(selectors) = flag_jql {
                    match process_jql(&api_value, selectors) {
                        Ok(s) => {
                            final_value = s;
                        },
                        Err(e) => {
                            error!(
                                "jql error. json: {api_value:?}, selectors: {selectors:?}, error: \
                                 {e:?}"
                            );

                            if flag_store_error {
                                final_value = e.to_string();
                            } else {
                                final_value = String::new();
                            }
                            error_flag = true;
                        },
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
        } else {
            error_flag = true;
            api_respheader.clear();
            api_status = reqwest::StatusCode::BAD_REQUEST;
        }

        // debug!("final value: {final_value}");

        // check if there's an API error (likely 503-service not available or 493-too many requests)
        // or if the API has ratelimits and we need to do dynamic throttling to respect limits
        if error_flag
            || (!api_respheader.is_empty()
                && (api_respheader.contains_key("ratelimit-limit")
                    || api_respheader.contains_key("x-ratelimit-limit")
                    || api_respheader.contains_key("retry-after")))
        {
            let mut ratelimit_remaining = api_respheader.get("ratelimit-remaining");
            if ratelimit_remaining.is_none() {
                let temp_var = api_respheader.get("x-ratelimit-remaining");
                if temp_var.is_some() {
                    ratelimit_remaining = temp_var;
                }
            }
            let mut ratelimit_reset = api_respheader.get("ratelimit-reset");
            if ratelimit_reset.is_none() {
                let temp_var = api_respheader.get("x-ratelimit-reset");
                if temp_var.is_some() {
                    ratelimit_reset = temp_var;
                }
            }

            // some APIs add the "-second" suffix to ratelimit fields
            let mut ratelimit_remaining_sec = api_respheader.get("ratelimit-remaining-second");
            if ratelimit_remaining_sec.is_none() {
                let temp_var = api_respheader.get("x-ratelimit-remaining-second");
                if temp_var.is_some() {
                    ratelimit_remaining_sec = temp_var;
                }
            }
            let mut ratelimit_reset_sec = api_respheader.get("ratelimit-reset-second");
            if ratelimit_reset_sec.is_none() {
                let temp_var = api_respheader.get("x-ratelimit-reset-second");
                if temp_var.is_some() {
                    ratelimit_reset_sec = temp_var;
                }
            }

            let retry_after = api_respheader.get("retry-after");

            if log_enabled!(Debug) {
                let rapidapi_proxy_response = api_respheader.get("X-RapidAPI-Proxy-Response");

                debug!(
                    "api_status:{api_status:?} rate_limit_remaining:{ratelimit_remaining:?} \
                     {ratelimit_remaining_sec:?} ratelimit_reset:{ratelimit_reset:?} \
                     {ratelimit_reset_sec:?} retry_after:{retry_after:?} \
                     rapid_api_proxy_response:{rapidapi_proxy_response:?}"
                );
            }

            // if there's a ratelimit_remaining field in the response header, get it
            // otherwise, set remaining to sentinel value 9999
            let remaining = ratelimit_remaining.map_or_else(
                || {
                    if let Some(ratelimit_remaining_sec) = ratelimit_remaining_sec {
                        let remaining_sec_str = ratelimit_remaining_sec.to_str().unwrap();
                        remaining_sec_str.parse::<u64>().unwrap_or(1)
                    } else {
                        9999_u64
                    }
                },
                |ratelimit_remaining| {
                    let remaining_str = ratelimit_remaining.to_str().unwrap();
                    remaining_str.parse::<u64>().unwrap_or(1)
                },
            );

            // if there's a ratelimit_reset field in the response header, get it
            // otherwise, set reset to sentinel value 0
            let mut reset_secs = ratelimit_reset.map_or_else(
                || {
                    if let Some(ratelimit_reset_sec) = ratelimit_reset_sec {
                        let reset_sec_str = ratelimit_reset_sec.to_str().unwrap();
                        reset_sec_str.parse::<u64>().unwrap_or(1)
                    } else {
                        // sleep for at least 1 second if we get an API error,
                        // even if there is no ratelimit_reset header,
                        // otherwise return 0
                        u64::from(error_flag)
                    }
                },
                |ratelimit_reset| {
                    let reset_str = ratelimit_reset.to_str().unwrap();
                    reset_str.parse::<u64>().unwrap_or(1)
                },
            );

            // if there's a retry_after field in the response header, get it
            // and set reset to it
            if let Some(retry_after) = retry_after {
                let retry_str = retry_after.to_str().unwrap();
                // if we cannot parse its value as u64, the retry after value
                // is most likely an rfc2822 date and not number of seconds to
                // wait before retrying, which is a valid value
                // however, we don't want to do date-parsing here, so we just
                // wait timeout_secs seconds before retrying
                reset_secs = retry_str.parse::<u64>().unwrap_or(timeout_secs);
            }

            // if reset_secs > timeout, then just time out and skip the retries
            if reset_secs > timeout_secs {
                warn!("Reset_secs {reset_secs} > timeout_secs {timeout_secs}.");
                break 'retry;
            }

            // if there is only one more remaining call per our ratelimit quota or reset >= 1,
            // dynamically throttle and sleep for ~reset seconds
            if remaining <= 1 || reset_secs >= 1 {
                // we add a small random delta to how long fetch sleeps
                // as we need to add a little jitter as per the spec to avoid thundering herd issues
                // https://tools.ietf.org/id/draft-polli-ratelimit-headers-00.html#rfc.section.7.5
                // we multiply by retries as a simple backoff multiplier
                // we multiply reset_secs by 1001 instead of 1000 to give the server a teeny bit
                // more breathing room before we hit it again
                let pause_time =
                    (reset_secs * 1001) + (retries as u64 * rand::thread_rng().gen_range(10..30));
                info!(
                    "sleeping for {pause_time} ms until ratelimit is reset/retry_after has elapsed"
                );

                thread::sleep(time::Duration::from_millis(pause_time));
            }

            if retries >= flag_max_retries {
                wwarn!("{flag_max_retries} max-retries reached.");
                break 'retry;
            }
            retries += 1;
            info!("retrying {retries}...");
        } else {
            // there's no request error or ratelimits nor retry-after
            break 'retry;
        }
    } // end retry loop

    if error_flag {
        if flag_store_error && !include_existing_columns {
            let json_error = json!({
                "errors": [{
                    "title": "HTTP ERROR",
                    "detail": final_value
                }]
            });
            FetchResponse {
                response: format!("{json_error}"),
                status_code: api_status.as_u16(),
                retries,
            }
        } else {
            FetchResponse {
                response: String::new(),
                status_code: api_status.as_u16(),
                retries,
            }
        }
    } else {
        FetchResponse {
            response: final_value,
            status_code: api_status.as_u16(),
            retries,
        }
    }
}
