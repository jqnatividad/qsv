static USAGE: &str = r#"
Modify headers of a CSV to only have "safe" names - guaranteed "database-ready" names
(optimized specifically for PostgreSQL column identifiers). 

Fold to lowercase. Trim leading & trailing whitespaces. Replace whitespace/non-alphanumeric
characters with _. If name starts with a number & check_first_char is true, prepend the unsafe prefix.
If a header with the same name already exists, append a sequence suffix (e.g. col, col_2, col_3).
Names are limited to 60 characters in length. Empty names are replaced with the unsafe prefix.

In addition, specifically because of CKAN Datastore requirements:
- Headers with leading underscores are replaced with "unsafe_" prefix.
- Headers that are named "_id" are renamed to "reserved__id".

These CKAN Datastore options can be configured via the --prefix & --reserved options, respectively.

In Always (a) and Conditional (c) mode, returns number of modified headers to stderr,
and sends CSV with safe headers output to stdout.

In Verify (v) mode, returns number of unsafe headers to stderr.
In Verbose (V) mode, returns number of headers; duplicate count and unsafe & safe headers to stderr.
No stdout output is generated in Verify and Verbose mode.

In JSON (j) mode, returns Verbose mode info in minified JSON to stdout.
In Pretty JSON (J) mode, returns Verbose mode info in pretty printed JSON to stdout.

Given data.csv:
 c1,12_col,Col with Embedded Spaces,,Column!@Invalid+Chars,c1
 1,a2,a3,a4,a5,a6

  $ qsv safenames data.csv
  c1,unsafe_12_col,col_with_embedded_spaces,unsafe_,column__invalid_chars,c1_2
  1,a2,a3,a4,a5,a6
  stderr: 5

  Conditionally rename headers, allowing "quoted identifiers":
  $ qsv safenames --mode c data.csv
  c1,unsafe_12_col,Col with Embedded Spaces,unsafe_,column__invalid_chars,c1_2
  1,a2,a3,a4,a5,a6
  stderr: 4

  Verify how many "unsafe" headers are found:
  $ qsv safenames --mode v data.csv
  stderr: 4

  Verbose mode:
  $ qsv safenames --mode V data.csv
  stderr: 6 header/s
  1 duplicate/s: "c1:2"
  4 unsafe header/s: ["12_col", "Col with Embedded Spaces", "", "Column!@Invalid+Chars"]
  1 safe header/s: ["c1"]

Note that even if "Col with Embedded Spaces" is technically safe, it is generally discouraged.
Though it can be created as a "quoted identifier" in PostgreSQL, it is still marked "unsafe"
by default, unless mode is set to "conditional." 

It is discouraged because the embedded spaces can cause problems later on.
(see https://lerner.co.il/2013/11/30/quoting-postgresql/ for more info).

For more examples, see https://github.com/jqnatividad/qsv/blob/master/tests/test_safenames.rs.

Usage:
    qsv safenames [options] [<input>]
    qsv safenames --help

safenames options:
    --mode <c|a|v|V|j|J>   Rename header names to "safe" names - i.e.
                           guaranteed "database-ready" names.
                           It has six modes - conditional, always, verify, Verbose,
                           with Verbose having two submodes - JSON & pretty JSON.

                           conditional (c) - check first before renaming and allow
                           "quoted identifiers" - mixed case with embedded spaces.
                           always (a) - goes ahead and renames all headers
                           without checking if they're already "safe".

                           verify (v) - count "unsafe" header names without
                           modifying them. Note that verify does not count
                           "quoted identifiers" as unsafe.
                           verbose (V) - like verify, but verbose, showing
                           total header count, duplicates, unsafe headers & safe headers.

                           JSON (j) - similar to verbose in minified JSON.
                           pretty JSON (J) - verbose in pretty-printed JSON
                           [default: Always]
    --reserved <list>      Comma-delimited list of additional case-insensitive reserved names
                           that should be considered "unsafe." If a header name is found in 
                           the reserved list, it will be prefixed with "reserved_".
                           [default: _id]
    --prefix <string>      Certain systems do not allow header names to start with "_" (e.g. CKAN Datastore).
                           This option allows the specification of the unsafe prefix to use when a header
                           starts with "_". [default: unsafe_]

Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
                           Note that no output is generated for Verify and
                           Verbose modes.
    -d, --delimiter <arg>  The field delimiter for reading CSV data.
                           Must be a single character. (default: ,)
"#;

use std::collections::HashMap;

use ahash::RandomState;
use serde::{Deserialize, Serialize};

use crate::{
    config::{Config, Delimiter},
    util, CliResult,
};

#[derive(Deserialize)]
struct Args {
    arg_input:      Option<String>,
    flag_mode:      String,
    flag_reserved:  String,
    flag_prefix:    String,
    flag_output:    Option<String>,
    flag_delimiter: Option<Delimiter>,
}

#[derive(PartialEq)]
enum SafeNameMode {
    Always,
    Conditional,
    Verify,
    VerifyVerbose,
    VerifyVerboseJSON,
    VerifyVerbosePrettyJSON,
}

#[derive(Serialize, Deserialize)]
struct SafeNamesStruct {
    header_count:      usize,
    duplicate_count:   usize,
    duplicate_headers: Vec<String>,
    unsafe_headers:    Vec<String>,
    safe_headers:      Vec<String>,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;

    // set SafeNames Mode
    let first_letter = args.flag_mode.chars().next().unwrap_or_default();
    let safenames_mode = match first_letter {
        'c' | 'C' => SafeNameMode::Conditional,
        'a' | 'A' => SafeNameMode::Always,
        'v' => SafeNameMode::Verify,
        'V' => SafeNameMode::VerifyVerbose,
        'j' => SafeNameMode::VerifyVerboseJSON,
        'J' => SafeNameMode::VerifyVerbosePrettyJSON,
        _ => {
            return fail_clierror!("Invalid mode: {}", args.flag_mode);
        },
    };

    let reserved_names_vec: Vec<String> = args
        .flag_reserved
        .split(',')
        .map(str::to_lowercase)
        .collect();

    let rconfig = Config::new(&args.arg_input).delimiter(args.flag_delimiter);

    let mut rdr = rconfig.reader()?;
    let mut wtr = Config::new(&args.flag_output).writer()?;
    let old_headers = rdr.byte_headers()?;

    let mut headers = csv::StringRecord::from_byte_record_lossy(old_headers.clone());

    // trim enclosing quotes and spaces from headers as it messes up safenames
    // csv library will automatically add quotes when necessary when we write it
    let mut noquote_headers = csv::StringRecord::new();
    for header in &headers {
        noquote_headers.push_field(header.trim_matches(|c| c == '"' || c == ' '));
    }

    let (safe_headers, changed_count) = util::safe_header_names(
        &noquote_headers,
        true,
        safenames_mode == SafeNameMode::Conditional,
        Some(reserved_names_vec),
        &args.flag_prefix,
        false,
    );
    if let SafeNameMode::Conditional | SafeNameMode::Always = safenames_mode {
        headers.clear();
        for header_name in safe_headers {
            headers.push_field(&header_name);
        }

        // write CSV with safe headers
        wtr.write_record(headers.as_byte_record())?;
        let mut record = csv::ByteRecord::new();
        while rdr.read_byte_record(&mut record)? {
            wtr.write_record(&record)?;
        }
        wtr.flush()?;

        eprintln!("{changed_count}");
    } else {
        // Verify or VerifyVerbose Mode
        let mut safenames_vec: Vec<String> = Vec::new();
        let mut unsafenames_vec: Vec<String> = Vec::new();
        let mut checkednames_map: HashMap<String, u16, RandomState> = HashMap::default();
        let mut temp_string;

        for header_name in &headers {
            if safe_headers.contains(&header_name.to_string()) {
                if !safenames_vec.contains(&header_name.to_string()) {
                    safenames_vec.push(header_name.to_string());
                }
            } else {
                unsafenames_vec.push(header_name.to_string());
            };

            temp_string = header_name.to_string();
            if let Some(count) = checkednames_map.get(&temp_string) {
                checkednames_map.insert(temp_string, count + 1);
            } else {
                checkednames_map.insert(temp_string, 1);
            };
        }

        let headers_count = headers.len();
        let dupe_count = checkednames_map.values().filter(|&&v| v > 1).count();
        let unsafe_count = unsafenames_vec.len();
        let safe_count = safenames_vec.len();

        let safenames_struct = SafeNamesStruct {
            header_count:      headers_count,
            duplicate_count:   dupe_count,
            duplicate_headers: checkednames_map
                .iter()
                .filter(|(_, &v)| v > 1)
                .map(|(k, v)| format!("{k}:{v}"))
                .collect(),
            unsafe_headers:    unsafenames_vec.clone(),
            safe_headers:      safenames_vec.clone(),
        };
        match safenames_mode {
            SafeNameMode::VerifyVerbose => {
                eprintln!(
                    r#"{num_headers} header/s
{dupe_count} duplicate/s: {dupe_headers:?}
{unsafe_count} unsafe header/s: {unsafenames_vec:?}
{num_safeheaders} safe header/s: {safenames_vec:?}"#,
                    dupe_headers = safenames_struct.duplicate_headers.join(", "),
                    num_headers = headers_count,
                    num_safeheaders = safe_count
                );
            },
            SafeNameMode::VerifyVerboseJSON | SafeNameMode::VerifyVerbosePrettyJSON => {
                if safenames_mode == SafeNameMode::VerifyVerbosePrettyJSON {
                    println!(
                        "{}",
                        serde_json::to_string_pretty(&safenames_struct).unwrap()
                    );
                } else {
                    println!("{}", serde_json::to_string(&safenames_struct).unwrap());
                };
            },
            _ => eprintln!("{unsafe_count}"),
        }
    }

    Ok(())
}
