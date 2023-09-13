# Environment Variables

| Variable | Description |
| --- | --- |
| `QSV_DEFAULT_DELIMITER` | single ascii character to use as delimiter.  Overrides `--delimiter` option. Defaults to "," (comma) for CSV files & "\t" (tab) for TSV files when not set. Note that this will also set the delimiter for qsv's output to stdout.<br>However, using the `--output` option, regardless of this environment variable, will automatically change the delimiter used in the generated file based on the file extension - i.e. comma for `.csv`, tab for `.tsv` & `.tab` files. |
| `QSV_SNIFF_DELIMITER` | if set, the delimiter is automatically detected. Overrides `QSV_DEFAULT_DELIMITER` & `--delimiter` option. Note that this does not work with stdin. |
| `QSV_NO_HEADERS` | if set, the first row will **NOT** be interpreted as headers. Supersedes `QSV_TOGGLE_HEADERS`. |
| `QSV_TOGGLE_HEADERS` | if set to `1`, toggles header setting - i.e. inverts qsv header behavior, with no headers being the default, & setting `--no-headers` will actually mean headers will not be ignored. |
| `QSV_AUTOINDEX_SIZE` | if set, specifies the minimum file size of a CSV file before an index is automatically created. Note that stale indices are automatically updated regardless of this setting. |
| `QSV_CACHE_DIR` | The directory to use for caching downloaded lookup_table resources using the `luau` qsv_register_lookup() helper function. |
| `QSV_CKAN_API` | The CKAN Action API endpoint to use with the `luau` qsv_register_lookup() helper function when using the "ckan://" scheme. |
| `QSV_CKAN_TOKEN`| The CKAN token to use with the `luau` qsv_register_lookup() helper function when using the "ckan://" scheme. Only required to access private resources. |
| `QSV_OPENAI_KEY` | The OpenAI API key to use with the `describegpt` command. |
| `QSV_COMMENT_CHAR` | set to an ascii character. If set, any lines(including the header) that start with this character are ignored. |
| `QSV_MAX_JOBS` | number of jobs to use for multithreaded commands (currently `apply`, `applydp`, `dedup`, `diff`, `extsort`, `frequency`, `joinp`, `schema`, `snappy`, `sort`, `split`, `stats`, `to`, `tojsonl` & `validate`). If not set, max_jobs is set to the detected number of logical processors.  See [Multithreading](docs/PERFORMANCE.md#multithreading) for more info. |
| `QSV_NO_UPDATE` | if set, prohibit self-update version check for the latest qsv release published on GitHub. |
| `QSV_PREFER_DMY` | if set, date parsing will use DMY format. Otherwise, use MDY format (used with `apply datefmt`, `schema`, `sniff` & `stats` commands). |
| `QSV_REGEX_UNICODE` | if set, makes `search`, `searchset` & `replace` commands unicode-aware. For increased performance, these commands are not unicode-aware by default & will ignore unicode values when matching & will abort when unicode characters are used in the regex. Note that the `apply operations regex_replace` operation is always unicode-aware. |
| `QSV_RDR_BUFFER_CAPACITY` | reader buffer size (default (bytes): 16384) |
| `QSV_WTR_BUFFER_CAPACITY` | writer buffer size (default (bytes): 65536) |
| `QSV_FREEMEMORY_HEADROOM_PCT` | the percentage of free available memory required when running qsv in "non-streaming" mode (i.e. the entire file needs to be loaded into memory). If the incoming file is greater than the available memory after the headroom is subtracted, qsv will not proceed. See [Memory Management](#memory-management) for more info. (default: (percent) 20 ) |
| `QSV_MEMORY_CHECK` | if set, check if input file size < AVAILABLE memory - HEADROOM (CONSERVATIVE mode) when running in "non-streaming" mode. Otherwise, qsv will only check if the input file size < TOTAL memory - HEADROOM (NORMAL mode). This is done to prevent Out-of-Memory errors. See [Memory Management](#memory-management) for more info. |
| `QSV_LOG_LEVEL` | desired level (default - off; `error`, `warn`, `info`, `trace`, `debug`). |
| `QSV_LOG_DIR` | when logging is enabled, the directory where the log files will be stored. If the specified directory does not exist, qsv will attempt to create it. If not set, the log files are created in the directory where qsv was started. See [Logging](docs/Logging.md#logging) for more info. |
| `QSV_LOG_UNBUFFERED` | if set, log messages are written directly to disk, without buffering. Otherwise, log messages are buffered before being written to the log file (8k buffer, flushing every second). See [flexi_logger](https://docs.rs/flexi_logger/latest/flexi_logger/enum.WriteMode.html) for details. |
| `QSV_PROGRESSBAR` | if set, enable the --progressbar option on the `apply`, `fetch`, `fetchpost`, `foreach`, `luau`, `py`, `replace`, `search`, `searchset`, `sortcheck` & `validate` commands.  |
| `QSV_REDIS_CONNSTR` | the `fetch` command can use [Redis](https://redis.io/) to cache responses. Set to connect to the desired Redis instance. (default: `redis:127.0.0.1:6379/1`). For more info on valid Redis connection string formats, click [here](https://docs.rs/redis/latest/redis/#connection-parameters). |
| `QSV_FP_REDIS_CONNSTR` | the `fetchpost` command can also use Redis to cache responses (default: `redis:127.0.0.1:6379/2`). Note that `fetchpost` connects to database 2, as opposed to `fetch` which connects to database 1. |
| `QSV_REDIS_MAX_POOL_SIZE` | the maximum Redis connection pool size. (default: 20). |
| `QSV_REDIS_TTL_SECONDS` | set time-to-live of Redis cached values (default (seconds): 2419200 (28 days)). |
| `QSV_REDIS_TTL_REFRESH`| if set, enables cache hits to refresh TTL of cached values. |
| `QSV_TIMEOUT`| for commands with a --timeout option (`fetch`, `fetchpost`, `luau`, `sniff` and `validate`), the number of seconds before a web request times out (default: 30). |
| `QSV_USER_AGENT`| the user-agent to use for web requests. When specifying a custom user agent. It supports the following variables - $QSV_VERSION, $QSV_TARGET, $QSV_BIN_NAME and $QSV_KIND. Try to conform to the [IETF RFC 72321 standard](https://tools.ietf.org/html/rfc7231#section-5.5.3). See [here](https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/User-Agent) for examples.<br>(default: $QSV_BIN_NAME/$QSV_VERSION ($QSV_TARGET; $QSV_KIND; https://github.com/jqnatividad/qsv) - e.g.<br>`qsv/0.105.0 (x86_64-unknown-linux; prebuilt; https://github.com/jqnatividad/qsv)`).|

Several dependencies also have environment variables that influence qsv's performance & behavior:

* Memory Allocator   
  When incorporating qsv into a data pipeline that runs in batch mode, particularly with very large CSV files using qsv commands that load entire CSV files into memory, you can fine tune qsv's memory allocator run-time behavior using the environment variables for the allocator you're using:

  * [mimalloc](https://github.com/microsoft/mimalloc#environment-options)

  * [jemalloc](https://jemalloc.net/jemalloc.3.html#environment)
    
* Network Access ([reqwest](https://docs.rs/reqwest/latest/reqwest/))   
  qsv uses reqwest and will honor [proxy settings](https://docs.rs/reqwest/latest/reqwest/index.html#proxies) set through the `HTTP_PROXY`, `HTTPS_PROXY`, `ALL_PROXY` & `NO_PROXY` environment variables.
  
> ℹ️ **NOTE:** To get a list of all active qsv-relevant environment variables, run `qsv --envlist`.
Relevant env vars are defined as anything that starts with `QSV_`, `MIMALLOC_`, `JEMALLOC_`, `MALLOC_CONF` & the proxy variables listed above.

## .env File Support
qsv supports the use of `.env` files to set environment variables. The `.env` file is a simple text file that contains key-value pairs, one per line. 

It processes `.env` files as follows:

* Upon invocation, qsv will look for a file named `.env` in the current working directory. If one is found, it will be processed.
* If no `.env` file is not found in the current working directory, qsv will next look for an `.env` file with the same filestem as the binary in the directory where the binary is (e.g. if `qsv`/`qsvlite`/`qsvdp` is in `/usr/local/bin`, it will look for `/usr/local/bin/qsv.env`, `/usr/local/bin/qsvlite.env` or `/usr/local/bin/qsvdp.env` respectively).
* If no `.env` files are found, qsv will proceed with its default settings and the current environment variables, which may include "QSV_" variables.

When processing `.env` files, qsv will:
* overwrite any existing environment variables with the same name
* where multiple declarations of the same variable exist, the last one will be used
* ignore any lines that start with `#` (comments)

To facilitate the use of `.env` files, a [`dotenv.template.yaml`](../dotenv.template.yaml) file is included in the qsv distribution. This file contains all the environment variables that qsv recognizes, along with their default values. Copy the template to a file named '.env' and modify it to suit your needs.
