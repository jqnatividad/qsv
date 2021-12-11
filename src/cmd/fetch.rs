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
    --jobs=<value>             Number of concurrent requests.
    --throttle=<ms>            Set throttle delay between requests in milliseconds. Recommend 1000 ms or greater (default: 5000 ms).
    --header=<file>            File containing additional HTTP Request Headers. Useful for setting Authorization or overriding User Agent.
    --cache                    Cache HTTP Responses to increase throughput.
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
    flag_jobs: Option<u8>,
    flag_throttle: Option<usize>,
    flag_header: Option<String>,
    flag_cache: bool,
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
            &args.flag_jobs,
            &args.flag_throttle,
            &args.flag_header,
            &args.flag_cache,
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
    let mut wtr = Config::new(&None).writer()?;

    let mut headers = rdr.byte_headers()?.clone();
    let sel = rconfig.selection(&headers)?;
    let _column_index = *sel.iter().next().unwrap();    

    assert!(sel.len() == 1, "Only one single URL column may be selected.");

    use reqwest::blocking::Client;
    let client = Client::new();

    if let Some(name) = &args.flag_new_column {

        // write header with new column 
        headers.push_field(name.as_bytes());
        wtr.write_byte_record(&headers)?;

        for row in rdr.byte_records() {
            let mut record = row?;
            for (_i, field) in sel.select(&record.clone()).enumerate() {
                let url = String::from_utf8_lossy(field).to_string();
                debug!("Fetching URL: {:?}", &url);
                let resp = client.get(url).send().unwrap();
                debug!("response: {:?}", &resp);
                let value = resp.text().unwrap();
                debug!("value: {:?}", &value);
                let safe_value = value.replace("\"","'");
                record.push_field(value.as_bytes());
            }

            wtr.write_byte_record(&record)?;
        }

    } else {

        // no valid new column name; only output fetched values 
        for row in rdr.byte_records() {
            let row = row?;
            for (_i, field) in sel.select(&row).enumerate() {
                let url = String::from_utf8_lossy(field).to_string();
                debug!("Fetching URL: {:?}", &url);
                let resp = client.get(url).send().unwrap();
                debug!("response: {:?}", &resp);
                let value = resp.text().unwrap();
                debug!("value: {:?}", &value);
                println!("\"{}\"", value.replace("\"","'"));
            }
        }

    } 


    Ok(())
}
