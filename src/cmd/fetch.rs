use std::str::from_utf8;

use crate::config::{Config, Delimiter};
use crate::util;
use crate::CliResult;
use crate::select::SelectColumns;
use serde::Deserialize;
use log::{debug, info};

static USAGE: &str = "
Fetch values via an URL column, and optionally store them in a new column.

This command fetches HTML/data from web pages or web services for every row in the URL column, 
and optionally stores them in a new column.

URL column must contain full and valid URL path, which can be constructed via the 'lua' command.

To set proxy, please set env var HTTP_PROXY and HTTPS_PROXY (eg export HTTPS_PROXY=socks5://127.0.0.1:1086)

Usage:
    qsv fetch [options] [<column>] [<input>]

fetch options:
    --new-column=<name>        Put the fetched values in a new column instead.
    --threads=<value>          Number of threads for concurrent requests.                 
    --throttle-delay=<ms>      Set delay between requests in milliseconds. Recommend 1000 ms or greater (default: 5000 ms).
    --http-header-file=<file>  File containing additional HTTP Request Headers. Useful for setting Authorization or overriding User Agent.
    --cache-responses          Cache HTTP Responses to increase throughput.
    --on-error-store-error     On error, store HTTP error instead of blank value.
    --store-and-send-cookies   Automatically store and send cookies. Useful for authenticated sessions.

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
    flag_threads: Option<u8>,
    flag_throttle_delay: Option<usize>,
    flag_http_header_file: Option<String>,
    flag_cache_responses: bool,
    flag_on_error_store_error: bool,
    flag_store_and_send_cookies: bool,
    flag_output: Option<String>,
    flag_no_headers: bool,
    flag_delimiter: Option<Delimiter>,
    flag_quiet: bool,
    arg_column: SelectColumns,
    arg_input: Option<String>
}



pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;

    debug!("url column: {:?}, 
            input: {:?}, 
            new column: {:?}, 
            threads: {:?},
            throttle delay: {:?},
            http header file: {:?},
            cache response: {:?},
            store error: {:?},
            store cookie: {:?},
            output: {:?}, 
            no_header: {:?}, 
            delimiter: {:?}, 
            quiet: {:?}", 
            (&args.arg_column).clone(),
            (&args.arg_input).clone().unwrap(),
            &args.flag_new_column,
            &args.flag_threads,
            &args.flag_throttle_delay,
            &args.flag_http_header_file,
            &args.flag_cache_responses,
            &args.flag_on_error_store_error,
            &args.flag_store_and_send_cookies,
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
    let headers = rdr.byte_headers()?.clone();
    let sel = rconfig.selection(&headers)?;
    let _column_index = *sel.iter().next().unwrap();    

    use reqwest::blocking::Client;
    let client = Client::new();

    for row in rdr.byte_records() {
        let row = row?;
        for (_i, field) in sel.select(&row).enumerate() {
            let url = String::from_utf8_lossy(field).to_string();
            debug!("Fetching URL: {:?}", url);
            let resp = client.get(url).send().unwrap();
            let value = resp.text().unwrap().replace(",",";");
            println!("{}", value);
        }
    }

    Ok(())
}
