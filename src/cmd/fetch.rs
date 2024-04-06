#![allow(unused_assignments)]
static USAGE: &str = r#"
Fetches data from web services for every row using HTTP Get.

Fetch is integrated with `jql` to directly parse out values from an API JSON response.

CACHE OPTIONS:
Fetch caches responses to minimize traffic and maximize performance. It has four
mutually-exclusive caching options:

1. In memory cache (the default)
2. Disk cache
3. Redis cache
4. No cache

In memory Cache:
In memory cache is the default and is used if no caching option is set.
It uses a non-persistent, in-memory, 2 million entry Least Recently Used (LRU)
cache for each fetch session. To change the maximum number of entries in the cache,
set the --mem-cache-size option.

Disk Cache:
For persistent, inter-session caching, a DiskCache can be enabled with the --disk-cache flag.
By default, it will store the cache in the directory ~/.qsv/cache/fetch, with a cache expiry
Time-to-Live (TTL) of 2,419,200 seconds (28 days), and cache hits NOT refreshing the TTL
of cached values.

Set the --disk-cache-dir option and the environment variables QSV_DISKCACHE_TTL_SECS and
QSV_DISKCACHE_TTL_REFRESH to change default DiskCache settings.

Redis Cache:
Another persistent, inter-session cache option is a Redis cache enabled with the --redis flag. 
By default, it will connect to a local Redis instance at redis://127.0.0.1:6379/1,
with a cache expiry Time-to-Live (TTL) of 2,419,200 seconds (28 days),
and cache hits NOT refreshing the TTL of cached values.

Set the environment variables QSV_REDIS_CONNSTR, QSV_REDIS_TTL_SECONDS and 
QSV_REDIS_TTL_REFRESH to change default Redis settings.

If you don't want responses to be cached at all, use the --no-cache flag.

NETWORK OPTIONS:
Fetch recognizes RateLimit and Retry-After headers and dynamically throttles requests
to be as fast as allowed. The --rate-limit option sets the maximum number of queries per second
(QPS) to be made. The default is 0, which means to go as fast as possible,
automatically throttling as required.

To use a proxy, set the environment variables HTTP_PROXY, HTTPS_PROXY or ALL_PROXY
(e.g. export HTTPS_PROXY=socks5://127.0.0.1:1086).

qsv fetch supports brotli, gzip and deflate automatic decompression for improved throughput
and performance, preferring brotli over gzip over deflate.

It automatically upgrades its connection to the much faster and more efficient HTTP/2 protocol
with adaptive flow control if the server supports it.
See https://www.cloudflare.com/learning/performance/http2-vs-http1.1/ and
https://medium.com/coderscorner/http-2-flow-control-77e54f7fd518 for more info.

URL OPTIONS:
<url-column> needs to be a fully qualified URL path. Alternatively, you can dynamically
construct URLs for each CSV record with the --url-template option (see Examples below).

EXAMPLES USING THE URL-COLUMN ARGUMENT:

data.csv
  URL
  https://api.zippopotam.us/us/90210
  https://api.zippopotam.us/us/94105
  https://api.zippopotam.us/us/92802

Given the data.csv above, fetch the JSON response.

  $ qsv fetch URL data.csv 

Note the output will be a JSONL file - with a minified JSON response per line, not a CSV file.

Now, if we want to generate a CSV file with the parsed City and State, we use the 
new-column and jql options. (See https://github.com/yamafaktory/jql#%EF%B8%8F-usage 
for more info on how to use the jql JSON Query Language)

$ qsv fetch URL --new-column CityState --jql '"places"[0]"place name","places"[0]"state abbreviation"' 
  data.csv > data_with_CityState.csv

data_with_CityState.csv
  URL, CityState,
  https://api.zippopotam.us/us/90210, "[\"Beverly Hills\",\"CA\"]"
  https://api.zippopotam.us/us/94105, "[\"San Francisco\",\"CA\"]"
  https://api.zippopotam.us/us/92802, "[\"Anaheim\",\"CA\"]"

As you can see, entering jql selectors on the command line is error prone and can quickly become cumbersome.
Alternatively, the jql selector can be saved and loaded from a file using the --jqlfile option.

  $ qsv fetch URL --new-column CityState --jqlfile places.jql data.csv > datatest.csv

EXAMPLES USING THE --URL-TEMPLATE OPTION:

Instead of using hardcoded URLs, you can also dynamically construct the URL for each CSV row using CSV column
values in that row.

Exanple 1:
For example, we have a CSV with four columns and we want to geocode against the geocode.earth API that expects 
latitude and longitude passed as URL parameters.

addr_data.csv
  location, description, latitude, longitude
  Home, "house is not a home when there's no one there", 40.68889829703977, -73.99589368107037
  X, "marks the spot", 40.78576117777992, -73.96279560368552
  work, "moolah", 40.70692672280804, -74.0112264146281
  school, "exercise brain", 40.72916494539206, -73.99624185993626
  gym, "exercise muscles", 40.73947342617386, -73.99039923885411

Geocode addresses in addr_data.csv, pass the latitude and longitude fields and store
the response in a new column called response into enriched_addr_data.csv.

$ qsv fetch --url-template "https://api.geocode.earth/v1/reverse?point.lat={latitude}&point.lon={longitude}" 
  addr_data.csv -c response > enriched_addr_data.csv

Example 2:
Geocode addresses in addresses.csv, pass the "street address" and "zip-code" fields
and use jql to parse placename from the JSON response into a new column in addresses_with_placename.csv.
Note how field name non-alphanumeric characters (space and hyphen) in the url-template were replaced with _.

$ qsv fetch --jql '"features"[0]"properties","name"' addresses.csv -c placename --url-template 
  "https://api.geocode.earth/v1/search/structured?address={street_address}&postalcode={zip_code}"
  > addresses_with_placename.csv

USING THE HTTP-HEADER OPTION:

The --http-header option allows you to append arbitrary key value pairs (a valid pair is a key and value
separated by a colon) to the HTTP header (to authenticate against an API, pass custom header fields, etc.).
Note that you can pass as many key-value pairs by using --http-header option repeatedly. For example:

$ qsv fetch URL data.csv --http-header "X-Api-Key:TEST_KEY" -H "X-Api-Secret:ABC123XYZ" -H "Accept-Language: fr-FR"

For more extensive examples, see https://github.com/jqnatividad/qsv/blob/master/tests/test_fetch.rs.

Usage:
    qsv fetch [<url-column> | --url-template <template>] [--jql <selector> | --jqlfile <file>] [--http-header <k:v>...] [options] [<input>]
    qsv fetch --help

Fetch options:
    <url-column>               Name of the column with the URL.
                               Mutually exclusive with --url-template.
    --url-template <template>  URL template to use. Use column names enclosed with
                               curly braces to insert the CSV data for a record.
                               Mutually exclusive with url-column.
    -c, --new-column <name>    Put the fetched values in a new column. Specifying this option
                               results in a CSV. Otherwise, the output is in JSONL format.
    --jql <selector>           Apply jql selector to API returned JSON value.
                               Mutually exclusive with --jqlfile,
    --jqlfile <file>           Load jql selector from file instead.
                               Mutually exclusive with --jql.
    --pretty                   Prettify JSON responses. Otherwise, they're minified.
                               If the response is not in JSON format, it's passed through.
                               Note that --pretty requires the --new-column option.
    --rate-limit <qps>         Rate Limit in Queries Per Second (max: 1000). Note that fetch
                               dynamically throttles as well based on rate-limit and
                               retry-after response headers.
                               Set to 0 to go as fast as possible, automatically throttling as required.
                               CAUTION: Only use zero for APIs that use RateLimit and/or Retry-After headers,
                               otherwise your fetch job may look like a Denial Of Service attack.
                               Even though zero is the default, this is mitigated by --max-errors having a
                               default of 10.
                               [default: 0 ]
    --timeout <seconds>        Timeout for each URL request.
                               [default: 30 ]
    -H, --http-header <k:v>    Append custom header(s) to the HTTP header. Pass multiple key-value pairs
                               by adding this option multiple times, once for each pair. The key and value 
                               should be separated by a colon.
    --max-retries <count>      Maximum number of retries per record before an error is raised.
                               [default: 5]
    --max-errors <count>       Maximum number of errors before aborting.
                               Set to zero (0) to continue despite errors.
                               [default: 10 ]
    --store-error              On error, store error code/message instead of blank value.
    --cookies                  Allow cookies.
    --user-agent <agent>       Specify custom user agent. It supports the following variables -
                               $QSV_VERSION, $QSV_TARGET, $QSV_BIN_NAME, $QSV_KIND and $QSV_COMMAND.
                               Try to follow the syntax here -
                               https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/User-Agent
    --report <d|s>             Creates a report of the fetch job. The report has the same name as the input file
                               with the ".fetch-report" suffix. 
                               There are two kinds of report - d for "detailed" & s for "short". The detailed
                               report has the same columns as the input CSV with six additional columns - 
                               qsv_fetch_url, qsv_fetch_status, qsv_fetch_cache_hit, qsv_fetch_retries, 
                               qsv_fetch_elapsed_ms & qsv_fetch_response.
                               The short report only has the six columns without the "qsv_fetch_" prefix.
                               [default: none]

                               CACHING OPTIONS:
    --no-cache                 Do not cache responses.
    
    --mem-cache-size <count>   Maximum number of entries in the in-memory LRU cache.
                               [default: 2000000]
    
    --disk-cache               Use a persistent disk cache for responses. The cache is stored in the directory
                               specified by --disk-cache-dir. If the directory does not exist, it will be
                               created. If the directory exists, it will be used as is.
                               It has a default Time To Live (TTL)/lifespan of 28 days and cache hits do not
                               refresh the TTL of cached values.
                               Adjust the QSV_DISKCACHE_TTL_SECS & QSV_DISKCACHE_TTL_REFRESH env vars
                               to change DiskCache settings.
    --disk-cache-dir <dir>     The directory <dir> to store the disk cache. Note that if the directory
                               does not exist, it will be created. If the directory exists, it will be used as is,
                               and will not be flushed. This option allows you to maintain several disk caches
                               for different fetch jobs (e.g. one for geocoding, another for weather, etc.)
                               [default: ~/.qsv/cache/fetch]

    --redis-cache              Use Redis to cache responses. It connects to "redis://127.0.0.1:6379/1"
                               with a connection pool size of 20, with a TTL of 28 days, and a cache hit 
                               NOT renewing an entry's TTL.
                               Adjust the QSV_REDIS_CONNSTR, QSV_REDIS_MAX_POOL_SIZE, QSV_REDIS_TTL_SECONDS & 
                               QSV_REDIS_TTL_REFRESH env vars respectively to change Redis settings.
                               This option is ignored if the --disk-cache option is enabled.

    --cache-error              Cache error responses even if a request fails. If an identical URL is requested,
                               the cached error is returned. Otherwise, the fetch is attempted again 
                               for --max-retries.
    --flush-cache              Flush all the keys in the current cache on startup. This only applies to
                               Disk and Redis caches.

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

use std::{fs, num::NonZeroU32, sync::OnceLock, thread, time};

use cached::{
    proc_macro::{cached, io_cached},
    stores::DiskCacheBuilder,
    Cached, IOCached, RedisCache, Return, SizedCache,
};
use dynfmt::Format;
use governor::{
    clock::DefaultClock,
    middleware::NoOpMiddleware,
    state::{direct::NotKeyed, InMemoryState},
    Quota, RateLimiter,
};
use indicatif::{HumanCount, MultiProgress, ProgressBar, ProgressDrawTarget};
use log::{
    debug, error, info, log_enabled, warn,
    Level::{Debug, Trace, Warn},
};
use rand::Rng;
use regex::Regex;
use reqwest::{
    blocking::Client,
    header::{HeaderMap, HeaderName, HeaderValue},
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use simdutf8::basic::from_utf8;
use simple_expand_tilde::expand_tilde;
use url::Url;

use crate::{
    config::{Config, Delimiter},
    regex_oncelock,
    select::SelectColumns,
    util, CliError, CliResult,
};

#[derive(Deserialize)]
struct Args {
    arg_url_column:      SelectColumns,
    arg_input:           Option<String>,
    flag_url_template:   Option<String>,
    flag_new_column:     Option<String>,
    flag_jql:            Option<String>,
    flag_jqlfile:        Option<String>,
    flag_pretty:         bool,
    flag_rate_limit:     u32,
    flag_timeout:        u16,
    flag_http_header:    Vec<String>,
    flag_max_retries:    u8,
    flag_max_errors:     u64,
    flag_store_error:    bool,
    flag_cookies:        bool,
    flag_user_agent:     Option<String>,
    flag_report:         String,
    flag_no_cache:       bool,
    flag_mem_cache_size: usize,
    flag_disk_cache:     bool,
    flag_disk_cache_dir: Option<String>,
    flag_redis_cache:    bool,
    flag_cache_error:    bool,
    flag_flush_cache:    bool,
    flag_output:         Option<String>,
    flag_no_headers:     bool,
    flag_delimiter:      Option<Delimiter>,
    flag_progressbar:    bool,
}

// set memcache size - the default is 2 million entries
// and is set through the docopt usage text
static MEM_CACHE_SIZE: OnceLock<usize> = OnceLock::new();

static QSV_REDIS_CONNSTR_ENV: &str = "QSV_REDIS_CONNSTR";
static QSV_REDIS_MAX_POOL_SIZE_ENV: &str = "QSV_REDIS_MAX_POOL_SIZE";
static QSV_REDIS_TTL_SECS_ENV: &str = "QSV_REDIS_TTL_SECS";
static QSV_REDIS_TTL_REFRESH_ENV: &str = "QSV_REDIS_TTL_REFRESH";
static DEFAULT_REDIS_CONN_STRING: OnceLock<String> = OnceLock::new();
static DEFAULT_REDIS_TTL_SECS: u64 = 60 * 60 * 24 * 28; // 28 days in seconds
static DEFAULT_REDIS_POOL_SIZE: u32 = 20;

// disk cache TTL is also 28 days by default
static DEFAULT_DISKCACHE_TTL_SECS: u64 = 60 * 60 * 24 * 28;

static TIMEOUT_SECS: OnceLock<u64> = OnceLock::new();

const FETCH_REPORT_PREFIX: &str = "qsv_fetch_";
const FETCH_REPORT_SUFFIX: &str = ".fetch-report.tsv";

// prioritize compression schemes. Brotli first, then gzip, then deflate, and * last
pub static DEFAULT_ACCEPT_ENCODING: &str = "br;q=1.0, gzip;q=0.6, deflate;q=0.4, *;q=0.2";

// for governor/ratelimiter
const MINIMUM_WAIT_MS: u64 = 10;
const MIN_WAIT: time::Duration = time::Duration::from_millis(MINIMUM_WAIT_MS);

// for --report option
#[derive(PartialEq)]
pub enum ReportKind {
    Detailed,
    Short,
    None,
}

#[derive(Debug)]
pub struct RedisConfig {
    pub conn_str:      String,
    pub max_pool_size: u32,
    pub ttl_secs:      u64,
    pub ttl_refresh:   bool,
}
impl RedisConfig {
    pub fn new() -> RedisConfig {
        Self {
            conn_str:      std::env::var(QSV_REDIS_CONNSTR_ENV)
                .unwrap_or_else(|_| DEFAULT_REDIS_CONN_STRING.get().unwrap().to_string()),
            max_pool_size: std::env::var(QSV_REDIS_MAX_POOL_SIZE_ENV)
                .unwrap_or_else(|_| DEFAULT_REDIS_POOL_SIZE.to_string())
                .parse()
                .unwrap_or(DEFAULT_REDIS_POOL_SIZE),
            ttl_secs:      std::env::var(QSV_REDIS_TTL_SECS_ENV)
                .unwrap_or_else(|_| DEFAULT_REDIS_TTL_SECS.to_string())
                .parse()
                .unwrap_or(DEFAULT_REDIS_TTL_SECS),
            ttl_refresh:   util::get_envvar_flag(QSV_REDIS_TTL_REFRESH_ENV),
        }
    }
}

#[derive(Debug)]
pub struct DiskCacheConfig {
    pub ttl_secs:    u64,
    pub ttl_refresh: bool,
}
impl DiskCacheConfig {
    pub fn new() -> DiskCacheConfig {
        Self {
            ttl_secs:    std::env::var("QSV_DISKCACHE_TTL_SECS")
                .unwrap_or_else(|_| DEFAULT_DISKCACHE_TTL_SECS.to_string())
                .parse()
                .unwrap_or(DEFAULT_DISKCACHE_TTL_SECS),
            ttl_refresh: util::get_envvar_flag("QSV_DISKCACHE_TTL_REFRESH"),
        }
    }
}

#[derive(Debug, Default, PartialEq)]
pub enum CacheType {
    #[default]
    None,
    InMemory,
    Disk,
    Redis,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct FetchResponse {
    pub response:    String,
    pub status_code: u16,
    pub retries:     u8,
}

static DISKCACHE_DIR: OnceLock<String> = OnceLock::new();
static REDISCONFIG: OnceLock<RedisConfig> = OnceLock::new();
static DISKCACHECONFIG: OnceLock<DiskCacheConfig> = OnceLock::new();

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;

    // connect to Redis at localhost, using database 1 by default when --redis-cache is enabled
    // fetch uses database 1 by default, as opposed to the database 2 with fetchpost
    DEFAULT_REDIS_CONN_STRING
        .set("redis://127.0.0.1:6379/1".to_string())
        .unwrap();

    // set memcache size
    MEM_CACHE_SIZE.set(args.flag_mem_cache_size).unwrap();

    // set timeout
    TIMEOUT_SECS
        .set(util::timeout_secs(args.flag_timeout)?)
        .unwrap();

    // setup diskcache dir response caching
    let diskcache_dir = if let Some(dir) = &args.flag_disk_cache_dir {
        if dir.starts_with('~') {
            // expand the tilde
            let expanded_dir = expand_tilde(dir).unwrap();
            expanded_dir.to_string_lossy().to_string()
        } else {
            dir.to_string()
        }
    } else {
        String::new()
    };

    let cache_type = if args.flag_no_cache {
        CacheType::None
    } else if args.flag_disk_cache {
        // if --flush-cache is set, flush the cache directory first if it exists
        if args.flag_flush_cache
            && !diskcache_dir.is_empty()
            && fs::metadata(&diskcache_dir).is_ok()
        {
            if let Err(e) = fs::remove_dir_all(&diskcache_dir) {
                return fail_clierror!(r#"Cannot remove cache directory "{diskcache_dir}": {e:?}"#);
            }
            info!("flushed DiskCache directory: {diskcache_dir}");
        }
        // check if the cache directory exists, if it doesn't, create it
        if !diskcache_dir.is_empty() {
            if let Err(e) = fs::create_dir_all(&diskcache_dir) {
                return fail_clierror!(r#"Cannot create cache directory "{diskcache_dir}": {e:?}"#);
            }
        }
        DISKCACHE_DIR.set(diskcache_dir).unwrap();
        // initialize DiskCache Config
        DISKCACHECONFIG.set(DiskCacheConfig::new()).unwrap();
        CacheType::Disk
    } else if args.flag_redis_cache {
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

        if args.flag_flush_cache {
            redis::cmd("FLUSHDB").execute(&mut redis_conn);
            info!("flushed Redis database.");
        }
        CacheType::Redis
    } else {
        CacheType::InMemory
    };
    log::info!("Cache Type: {cache_type:?}");

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

    let mut column_index = 0_usize;
    if args.flag_url_template.is_none() {
        rconfig = rconfig.select(args.arg_url_column);
        let sel = rconfig.selection(&headers)?;
        column_index = *sel.iter().next().unwrap();
        if sel.len() != 1 {
            return fail!("Only a single URL column may be selected.");
        }
    }

    let mut dynfmt_url_template = String::new();
    if let Some(ref url_template) = args.flag_url_template {
        if args.flag_no_headers {
            return fail_incorrectusage_clierror!("--url-template option requires column headers.");
        }
        let str_headers = rdr.headers()?.clone();
        let mut dynfmt_fields = Vec::with_capacity(10); // 10 is a reasonable default to save allocs

        dynfmt_url_template = url_template.to_string();
        // first, get the fields used in the url template
        let (safe_headers, _) =
            util::safe_header_names(&str_headers, false, false, None, "", false);
        let formatstr_re: &'static Regex = regex_oncelock!(r"\{(?P<key>\w+)?\}");
        for format_fields in formatstr_re.captures_iter(url_template) {
            dynfmt_fields.push(format_fields.name("key").unwrap().as_str());
        }
        // we sort the fields so we can do binary_search
        dynfmt_fields.sort_unstable();
        // now, get the indices of the columns for the lookup vec
        for (i, field) in safe_headers.iter().enumerate() {
            if dynfmt_fields.binary_search(&field.as_str()).is_ok() {
                let field_with_curly = format!("{{{field}}}");
                let field_index = format!("{{{i}}}");
                dynfmt_url_template = dynfmt_url_template.replace(&field_with_curly, &field_index);
            }
        }
        debug!("dynfmt_fields: {dynfmt_fields:?}  url_template: {dynfmt_url_template}");
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
    debug!("RATE LIMIT: {rate_limit}");

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
        map
    };
    debug!("HTTP Header: {http_headers:?}");

    let client_timeout = time::Duration::from_secs(*TIMEOUT_SECS.get().unwrap_or(&30));
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
        error_progress.set_message(format!(
            " of {} max errors",
            HumanCount(args.flag_max_errors)
        ));
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

        report_wtr = Config::new(&Some(report_path.clone() + FETCH_REPORT_SUFFIX))
            .delimiter(Some(Delimiter(b'\t')))
            .writer()?;
        let mut report_headers = if report == ReportKind::Detailed {
            headers.clone()
        } else {
            csv::ByteRecord::new()
        };
        let rptcol_prefix = if report == ReportKind::Detailed {
            FETCH_REPORT_PREFIX
        } else {
            ""
        };
        // the fetch report has the following columns:
        // url - URL used, status - HTTP status code, cache_hit - cache hit flag,
        // retries - retry attempts, elapsed - elapsed time (milliseconds) & response.
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
    let mut record_vec: Vec<String> = Vec::with_capacity(headers.len());
    let mut redis_cache_hits: u64 = 0;
    let mut disk_cache_hits: u64 = 0;
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
    let mut final_response = FetchResponse {
        response:    String::new(),
        status_code: 0_u16,
        retries:     0_u8,
    };
    let empty_response = FetchResponse {
        response:    String::new(),
        status_code: 0_u16,
        retries:     0_u8,
    };
    let mut running_error_count = 0_u64;
    let mut running_success_count = 0_u64;
    let mut was_cached;
    let mut now = time::Instant::now();

    while rdr.read_byte_record(&mut record)? {
        if show_progress {
            progress.inc(1);
        }

        if report != ReportKind::None {
            now = time::Instant::now();
        };

        if args.flag_url_template.is_some() {
            // we're using a URL template.
            // let's dynamically construct the URL with it
            record_vec.clear();
            for field in &record {
                record_vec.push(from_utf8(field).unwrap_or_default().to_owned());
            }
            if let Ok(formatted) =
                dynfmt::SimpleCurlyFormat.format(&dynfmt_url_template, &*record_vec)
            {
                url = formatted.into_owned();
            }
        } else if let Ok(s) = from_utf8(&record[column_index]) {
            // we're not using a URL template,
            // just use the field as-is as the URL
            s.clone_into(&mut url);
        } else {
            url = String::new();
        }

        if url.is_empty() {
            final_response.clone_from(&empty_response);
            was_cached = false;
        } else {
            match cache_type {
                CacheType::InMemory => {
                    intermediate_value = get_cached_response(
                        &url,
                        &client,
                        &limiter,
                        &jql_selector,
                        args.flag_store_error,
                        args.flag_pretty,
                        include_existing_columns,
                        args.flag_max_retries,
                    );
                    final_response = intermediate_value.value;
                    was_cached = intermediate_value.was_cached;
                    if !args.flag_cache_error && final_response.status_code != 200 {
                        let mut cache = GET_CACHED_RESPONSE.lock().unwrap();
                        cache.cache_remove(&url);
                    }
                },
                CacheType::Disk => {
                    intermediate_value = get_diskcache_response(
                        &url,
                        &client,
                        &limiter,
                        &jql_selector,
                        args.flag_store_error,
                        args.flag_pretty,
                        include_existing_columns,
                        args.flag_max_retries,
                    )?;
                    final_response = intermediate_value.value;
                    was_cached = intermediate_value.was_cached;
                    if was_cached {
                        disk_cache_hits += 1;
                        // log::debug!("Disk cache hit for {url} hit: {disk_cache_hits}");
                    }
                    if !args.flag_cache_error && final_response.status_code != 200 {
                        let _ = GET_DISKCACHE_RESPONSE.cache_remove(&url);
                        // log::debug!("Removed Disk cache for {url}");
                    }
                },
                CacheType::Redis => {
                    intermediate_redis_value = get_redis_response(
                        &url,
                        &client,
                        &limiter,
                        &jql_selector,
                        args.flag_store_error,
                        args.flag_pretty,
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
                                "Cannot deserialize Redis cache value. Try flushing the Redis \
                                 cache with --flushdb: {e}"
                            )
                        },
                    };
                    if !args.flag_cache_error && final_response.status_code != 200 {
                        let key = format!(
                            "{}{:?}{}{}{}",
                            url,
                            jql_selector,
                            args.flag_store_error,
                            args.flag_pretty,
                            include_existing_columns
                        );

                        if GET_REDIS_RESPONSE.cache_remove(&key).is_err() && log_enabled!(Warn) {
                            // failure to remove cache keys is non-fatal. Continue, but log it.
                            wwarn!(r#"Cannot remove Redis key "{key}""#);
                        };
                    }
                },
                CacheType::None => {
                    final_response = get_response(
                        &url,
                        &client,
                        &limiter,
                        &jql_selector,
                        args.flag_store_error,
                        args.flag_pretty,
                        include_existing_columns,
                        args.flag_max_retries,
                    );
                    was_cached = false;
                },
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
        match cache_type {
            CacheType::InMemory => {
                util::update_cache_info!(progress, GET_CACHED_RESPONSE);
            },
            CacheType::Disk => {
                util::update_cache_info!(progress, disk_cache_hits, record_count);
            },
            CacheType::Redis => {
                util::update_cache_info!(progress, redis_cache_hits, record_count);
            },
            CacheType::None => (),
        }
        util::finish_progress(&progress);

        if running_error_count == 0 {
            error_progress.finish_and_clear();
        } else if running_error_count >= args.flag_max_errors {
            error_progress.finish();
            // sleep so we can dependably write eprintln without messing up progress bars
            thread::sleep(time::Duration::from_nanos(10));
            let abort_msg = format!(
                "{} max errors. Fetch aborted.",
                HumanCount(args.flag_max_errors)
            );
            winfo!("{abort_msg}");
        } else {
            error_progress.abandon();
        }
    }

    let mut end_msg = format!(
        "{} records successfully fetched as {}. {} errors.",
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
            " {} report created: \"{}{FETCH_REPORT_SUFFIX}\"",
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
    type = "SizedCache<String, Return<FetchResponse>>",
    create = r##"{
        let cache_size = MEM_CACHE_SIZE.get().unwrap();
        let memcache = SizedCache::with_size(*cache_size);
        log::info!("In Memory cache created - size: {cache_size} entries");
        memcache
    }"##,
    key = "String",
    convert = r#"{ format!("{}", url) }"#,
    with_cached_flag = true
)]
fn get_cached_response(
    url: &str,
    client: &reqwest::blocking::Client,
    limiter: &governor::RateLimiter<NotKeyed, InMemoryState, DefaultClock, NoOpMiddleware>,
    flag_jql: &Option<String>,
    flag_store_error: bool,
    flag_pretty: bool,
    include_existing_columns: bool,
    flag_max_retries: u8,
) -> cached::Return<FetchResponse> {
    Return::new(get_response(
        url,
        client,
        limiter,
        flag_jql,
        flag_store_error,
        flag_pretty,
        include_existing_columns,
        flag_max_retries,
    ))
}

// this is a disk cache that can be used across qsv sessions
// so we need to include the values of flag_jql, flag_store_error, flag_pretty and
// include_existing_columns in the cache key
#[io_cached(
    disk = true,
    type = "cached::DiskCache<String, FetchResponse>",
    cache_prefix_block = r##"{ "dc_" }"##,
    key = "String",
    convert = r##"{ format!("{}{:?}{}{}{}", url, flag_jql, flag_store_error, flag_pretty, include_existing_columns) }"##,
    create = r##"{
        let cache_dir = DISKCACHE_DIR.get().unwrap();
        let diskcache_config = DISKCACHECONFIG.get().unwrap();
        let diskcache = DiskCacheBuilder::new("fetch")
            .set_disk_directory(cache_dir)
            .set_lifespan(diskcache_config.ttl_secs)
            .set_refresh(diskcache_config.ttl_refresh)
            .build()
            .expect("error building disk cache");
        log::info!("Disk cache created - dir: {cache_dir} - ttl: {ttl_secs}",
            ttl_secs = diskcache_config.ttl_secs);
        diskcache
    }"##,
    map_error = r##"|e| CliError::Other(format!("Diskcache Error: {:?}", e))"##,
    with_cached_flag = true
)]
fn get_diskcache_response(
    url: &str,
    client: &reqwest::blocking::Client,
    limiter: &governor::RateLimiter<NotKeyed, InMemoryState, DefaultClock, NoOpMiddleware>,
    flag_jql: &Option<String>,
    flag_store_error: bool,
    flag_pretty: bool,
    include_existing_columns: bool,
    flag_max_retries: u8,
) -> Result<cached::Return<FetchResponse>, CliError> {
    Ok(Return::new({
        get_response(
            url,
            client,
            limiter,
            flag_jql,
            flag_store_error,
            flag_pretty,
            include_existing_columns,
            flag_max_retries,
        )
    }))
}

// get_redis_response needs a longer key as its a persistent cache and the
// values of flag_jql, flag_store_error, flag_pretty and include_existing_columns
// may change between sessions
#[io_cached(
    type = "cached::RedisCache<String, String>",
    key = "String",
    convert = r##"{ format!("{}{:?}{}{}{}", url, flag_jql, flag_store_error, flag_pretty, include_existing_columns) }"##,
    create = r##" {
        let redis_config = REDISCONFIG.get().unwrap();
        let rediscache = RedisCache::new("f", redis_config.ttl_secs)
            .set_namespace("q")
            .set_refresh(redis_config.ttl_refresh)
            .set_connection_string(&redis_config.conn_str)
            .set_connection_pool_max_size(redis_config.max_pool_size)
            .build()
            .expect("error building redis cache");
        log::info!("Redis cache created - conn_str: {conn_str} - refresh: {ttl_refresh} - ttl: {ttl_secs} - pool_size: {pool_size}",
            conn_str = redis_config.conn_str,
            ttl_refresh = redis_config.ttl_refresh,
            ttl_secs = redis_config.ttl_secs,
            pool_size = redis_config.max_pool_size);
        rediscache
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
    flag_max_retries: u8,
) -> Result<cached::Return<String>, CliError> {
    Ok(Return::new({
        serde_json::to_string(&get_response(
            url,
            client,
            limiter,
            flag_jql,
            flag_store_error,
            flag_pretty,
            include_existing_columns,
            flag_max_retries,
        ))
        .unwrap()
    }))
}

/// Get the value of a header from the ratelimit API response
/// given its name or its x-name
pub fn get_ratelimit_header_value<'a>(
    api_respheader: &'a HeaderMap,
    header: &'a str,
    x_header: &'a str,
) -> Option<&'a HeaderValue> {
    api_respheader
        .get(header)
        .or_else(|| api_respheader.get(x_header))
}

/// Parse the value of a header from the ratelimit API response
/// return sentinel value if the header is not found or if the value is not a valid u64
/// return 1 if the value is 0
pub fn parse_ratelimit_header_value(value: Option<&HeaderValue>, sentinel_value: u64) -> u64 {
    value.map_or(sentinel_value, |v| {
        atoi_simd::parse_pos::<u64>(v.to_str().unwrap().as_bytes()).unwrap_or(1)
    })
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
    debug!("Using URL: {valid_url}");

    // wait until RateLimiter gives Okay or we timeout
    let mut limiter_total_wait: u64;
    let timeout_secs = *TIMEOUT_SECS.get().unwrap_or(&30_u64);
    let governor_timeout_ms = timeout_secs * 1_000;

    let mut retries = 0_u8;
    let mut error_flag;
    let mut final_value = String::new();
    let mut api_status;
    let mut api_respheader = HeaderMap::new();

    let debug_flag = log_enabled!(Debug);

    // request with --max-retries
    'retry: loop {
        // check the rate-limiter
        limiter_total_wait = 0;
        while limiter.check().is_err() {
            limiter_total_wait += MINIMUM_WAIT_MS;
            thread::sleep(MIN_WAIT);
            if limiter_total_wait > governor_timeout_ms {
                debug!("rate limit timed out after {limiter_total_wait} ms");
                break;
            } else if limiter_total_wait == MINIMUM_WAIT_MS {
                debug!("throttling...");
            }
        }
        if debug_flag && limiter_total_wait > 0 && limiter_total_wait <= governor_timeout_ms {
            debug!("throttled for {limiter_total_wait} ms");
        }

        // send the actual request
        if let Ok(resp) = client.get(&valid_url).send() {
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
                    let api_value_json: serde_json::Value = serde_json::from_str(&api_value).unwrap();
                    if let Ok(pretty_json) = serde_json::to_string_pretty(&api_value_json) {
                        final_value = pretty_json;
                    } else {
                        final_value = api_value;
                    }
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
            let ratelimit_remaining = get_ratelimit_header_value(
                &api_respheader,
                "ratelimit-remaining",
                "x-ratelimit-remaining",
            );

            let ratelimit_reset =
                get_ratelimit_header_value(&api_respheader, "ratelimit-reset", "x-ratelimit-reset");

            // some APIs add the "-second" suffix to ratelimit fields
            let ratelimit_remaining_sec = get_ratelimit_header_value(
                &api_respheader,
                "ratelimit-remaining-second",
                "x-ratelimit-remaining-second",
            );

            let ratelimit_reset_sec = get_ratelimit_header_value(
                &api_respheader,
                "ratelimit-reset-second",
                "x-ratelimit-reset-second",
            );

            let retry_after = api_respheader.get("retry-after");

            if debug_flag {
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
            let remaining =
                parse_ratelimit_header_value(ratelimit_remaining.or(ratelimit_remaining_sec), 9999);

            // if there's a ratelimit_reset field in the response header, get it
            // otherwise, set reset to sentinel value 0
            let mut reset_secs =
                parse_ratelimit_header_value(ratelimit_reset.or(ratelimit_reset_sec), 0);

            // if there's a retry_after field in the response header, get it
            // and set reset to it
            if let Some(retry_after) = retry_after {
                let retry_str = retry_after.to_str().unwrap();
                // if we cannot parse its value as u64, the retry after value
                // is most likely an rfc2822 date and not number of seconds to
                // wait before retrying, which is a valid value
                // however, we don't want to do date-parsing here, so we just
                // wait timeout_secs seconds before retrying
                reset_secs =
                    atoi_simd::parse_pos::<u64>(retry_str.as_bytes()).unwrap_or(timeout_secs);
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

                debug!(
                    "sleeping for {pause_time} ms until ratelimit is reset/retry_after has elapsed"
                );
                thread::sleep(time::Duration::from_millis(pause_time));
            }

            if retries >= flag_max_retries {
                wwarn!("{flag_max_retries} max-retries reached.");
                break 'retry;
            }
            retries += 1;
            debug!("retrying {retries}...");
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

impl From<jql_runner::errors::JqlRunnerError> for CliError {
    fn from(err: jql_runner::errors::JqlRunnerError) -> CliError {
        CliError::Other(format!("jql runner error: {err:?}"))
    }
}

#[cached(
    size = 2_000_000,
    key = "String",
    convert = r#"{ format!("{}-{}", json, query) }"#,
    result = true
)]
pub fn process_jql(json: &str, query: &str) -> CliResult<String> {
    let mut deserializer = serde_json::Deserializer::from_str(json);

    deserializer.disable_recursion_limit();

    let deserializer = serde_stacker::Deserializer::new(&mut deserializer);

    let value = match serde_json::Value::deserialize(deserializer) {
        Ok(valid_value) => valid_value,
        Err(e) => return fail_clierror!("Failed to deserialize the JSON data: {e:?}"),
    };
    let result: Value = jql_runner::runner::raw(query, &value)?;

    Ok(serde_json::to_string(&result)?)
}

// TODO: use this in the future to process JQL with pre-parsed tokens
// pub fn process_json_with_tokens(
//     json: &str,
//     tokens_vec: &Vec<jql_parser::tokens::Token>,
// ) -> CliResult<String> { if let Err(error) = serde_json::from_str::<Value>(json) { return
//   fail_clierror!("Invalid json: {error:?}"); }

//     let mut deserializer = serde_json::Deserializer::from_str(json);

//     deserializer.disable_recursion_limit();

//     let deserializer = serde_stacker::Deserializer::new(&mut deserializer);

//     let value = match serde_json::Value::deserialize(deserializer) {
//         Ok(valid_value) => valid_value,
//         Err(e) => return fail_clierror!("Failed to deserialize the JSON data: {e:?}"),
//     };
//     let result: Value = jql_runner::runner::token(tokens_vec, &value)?;

//     Ok(serde_json::to_string(&result)?)
// }

#[test]
fn test_apply_jql_invalid_json() {
    let json =
        r#"<!doctype html><html lang="en"><meta charset=utf-8><title>shortest html5</title>"#;
    let selectors = r#"."places"[0]."place name""#;

    let value = process_jql(json, selectors).unwrap_err().to_string();

    assert_eq!(
        "Failed to deserialize the JSON data: Error(\"expected value\", line: 1, column: 1)",
        value
    );
}

#[test]
fn test_apply_jql_invalid_selector() {
    let json = r#"{"post code": "90210", "country": "United States", "country abbreviation": "US", "places": [{"place name": "Beverly Hills", "longitude": "-118.4065", "state": "California", "state abbreviation": "CA", "latitude": "34.0901"}]}"#;
    let selectors = r#"."place"[0]."place name""#;

    let value = process_jql(json, selectors).unwrap_err().to_string();

    assert_eq!(
        r#"jql runner error: ParsingError(ParsingError { tokens: "", unparsed: ".\"place\"[0].\"place name\"" })"#,
        value
    );
}

#[test]
fn test_apply_jql_string() {
    let json = r#"{"post code": "90210", "country": "United States", "country abbreviation": "US", "places": [{"place name": "Beverly Hills", "longitude": "-118.4065", "state": "California", "state abbreviation": "CA", "latitude": "34.0901"}]}"#;
    let selectors = r#""places"[0]"place name""#;

    let value = process_jql(json, selectors).unwrap();

    assert_eq!(r#""Beverly Hills""#, value);
}

// #[test]
// fn test_apply_jql_string_with_tokens_vec() {
//     let json = r#"{"post code": "90210", "country": "United States", "country abbreviation":
// "US", "places": [{"place name": "Beverly Hills", "longitude": "-118.4065", "state": "California",
// "state abbreviation": "CA", "latitude": "34.0901"}]}"#;     let selectors = r#""places"[0]"place
// name""#;
//     let token_vec = jql_parser::parser::parse(&selectors).unwrap();
//     let value = process_json_with_token(json, &token_vec).unwrap();
//     assert_eq!(r#""Beverly Hills""#, value);
// }

#[test]
fn test_apply_jql_number() {
    let json = r#"{"post code": "90210", "country": "United States", "country abbreviation": "US", "places": [{"place name": "Beverly Hills", "longitude": -118.4065, "state": "California", "state abbreviation": "CA", "latitude": 34.0901}]}"#;
    let selectors = r#""places"[0]"longitude""#;

    let value = process_jql(json, selectors).unwrap();

    assert_eq!("-118.4065", value);
}

#[test]
fn test_apply_jql_bool() {
    let json = r#"{"post code": "90210", "country": "United States", "country abbreviation": "US", "places": [{"place name": "Beverly Hills", "longitude": -118.4065, "state": "California", "state abbreviation": "CA", "latitude": 34.0901, "expensive": true}]}"#;
    let selectors = r#""places"[0]"expensive""#;

    let value = process_jql(json, selectors).unwrap();

    assert_eq!("true", value);
}

#[test]
fn test_apply_jql_null() {
    let json = r#"{"post code": "90210", "country": "United States", "country abbreviation": "US", "places": [{"place name": "Beverly Hills", "longitude": -118.4065, "state": "California", "state abbreviation": "CA", "latitude": 34.0901, "university":null}]}"#;
    let selectors = r#""places"[0]"university""#;

    let value = process_jql(json, selectors).unwrap();

    assert_eq!("null", value);
}

#[test]
fn test_apply_jql_array() {
    let json = r#"{"post code": "90210", "country": "United States", "country abbreviation": "US", "places": [{"place name": "Beverly Hills", "longitude": -118.4065, "state": "California", "state abbreviation": "CA", "latitude": 34.0901}]}"#;
    let selectors = r#""places"[0]"longitude","places"[0]"latitude""#;

    let value = process_jql(json, selectors).unwrap();

    assert_eq!("[-118.4065,34.0901]", value);
}

#[test]
fn test_root_out_of_bounds() {
    // test for out_of_bounds root element handling
    // see https://github.com/yamafaktory/jql/issues/129
    let json = r#"[{"page":1,"pages":1,"per_page":"50","total":1},[{"id":"BRA","iso2Code":"BR","name":"Brazil","region":{"id":"LCN","iso2code":"ZJ","value":"Latin America & Caribbean (all income levels)"},"adminregion":{"id":"LAC","iso2code":"XJ","value":"Latin America & Caribbean (developing only)"},"incomeLevel":{"id":"UMC","iso2code":"XT","value":"Upper middle income"},"lendingType":{"id":"IBD","iso2code":"XF","value":"IBRD"},"capitalCity":"Brasilia","longitude":"-47.9292","latitude":"-15.7801"}]]"#;
    let selectors = r#"[2][0]"incomeLevel""value"'"#;

    let value = process_jql(json, selectors).unwrap_err().to_string();

    assert_eq!(
        r#"jql runner error: ParsingError(ParsingError { tokens: "Array Index Selector [Index (2)], Array Index Selector [Index (0)], Key Selector \"incomeLevel\", Key Selector \"value\"", unparsed: "'" })"#,
        value
    );
}
