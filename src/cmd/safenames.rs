static USAGE: &str = r#"
Modify headers of a CSV to only have "safe" names - guaranteed "database-ready" names
(optimized specifically for PostgreSQL column identifiers). 

Fold to lowercase. Trim leading & trailing whitespaces. Replace whitespace/non-alphanumeric
characters with _. If the first character is a digit, replace the digit with the unsafe prefix.
If a header with the same name already exists, append a sequence suffix (e.g. c1, c1_2, c1_3).
Names are limited to 60 characters in length. Empty names are replaced with "unsafe_".

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
  c1,_2_col,Col with Embedded Spaces,_blank,column__invalid_chars,c1_2
  1,a2,a3,a4,a5,a6
  stderr: 5

  Conditionally rename headers, allowing "quoted identifiers":
  $ qsv safenames --mode c data.csv
  c1,_2_col,Col with Embedded Spaces,_blank,column__invalid_chars,c1_2
  1,a2,a3,a4,a5,a6
  stderr: 4

  Verify how many "unsafe" headers are found:
  $ qsv safenames --mode v data.csv
  stderr: 3

  Verbose mode:
  $ qsv safenames --mode V safenames.csv
  stderr: 6 header/s
  1 duplicate/s
  3 unsafe header/s: ["12_col", "", "Column!@Invalid+Chars"]
  2 safe header/s: ["c1", "Col with Embedded Spaces"]

Though "Col with Embedded Spaces" is safe, it is generally discouraged. It can be created "safely"
as a "quoted identifier" in PostgreSQL. However, it is also discouraged because the embedded
spaces can cause problems later on (see https://lerner.co.il/2013/11/30/quoting-postgresql/).

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
                           This option allows the specification of a prefix to use when a header starts with "_".
                           [default: unsafe_]

Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
                           Note that no output is generated for Verify and
                           Verbose modes.
    -d, --delimiter <arg>  The field delimiter for reading CSV data.
                           Must be a single character. (default: ,)
"#;

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
    header_count:    usize,
    duplicate_count: u16,
    unsafe_headers:  Vec<String>,
    safe_headers:    Vec<String>,
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
        }
    };

    let reserved_names_vec: Vec<String> = args
        .flag_reserved
        .split(',')
        .map(str::to_lowercase)
        .collect();

    let rconfig = Config::new(&args.arg_input)
        .checkutf8(true)
        .delimiter(args.flag_delimiter);

    let mut rdr = rconfig.reader()?;
    let mut wtr = Config::new(&args.flag_output).writer()?;
    let old_headers = rdr.byte_headers()?;

    let mut headers = csv::StringRecord::from_byte_record_lossy(old_headers.clone());
    if let SafeNameMode::Conditional | SafeNameMode::Always = safenames_mode {
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
            &reserved_names_vec,
            &args.flag_prefix,
        );

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
        let mut checkednames_vec: Vec<String> = Vec::with_capacity(old_headers.len());
        let mut safenames_vec: Vec<String> = Vec::new();
        let mut unsafenames_vec: Vec<String> = Vec::new();
        let mut unsafe_count = 0_u16;
        let mut dupe_count = 0_u16;

        for header_name in headers.iter() {
            let safe_flag = util::is_safe_name(header_name);
            if safe_flag {
                if !safenames_vec.contains(&header_name.to_string()) {
                    safenames_vec.push(header_name.to_string());
                }
            } else {
                unsafenames_vec.push(header_name.to_string());
                unsafe_count += 1;
            };

            // check for duplicate headers/columns
            if checkednames_vec.contains(&header_name.to_string()) {
                dupe_count += 1;
            } else {
                checkednames_vec.push(header_name.to_string());
            }
        }

        match safenames_mode {
            SafeNameMode::VerifyVerbose => {
                eprintln!(
                    "{num_headers} header/s\n{dupe_count} duplicate/s\n{unsafe_count} unsafe \
                     header/s: {unsafenames_vec:?}\n{num_safeheaders} safe header/s: \
                     {safenames_vec:?}",
                    num_headers = headers.len(),
                    num_safeheaders = safenames_vec.len()
                );
            }
            SafeNameMode::VerifyVerboseJSON | SafeNameMode::VerifyVerbosePrettyJSON => {
                let safenames_struct = SafeNamesStruct {
                    header_count:    headers.len(),
                    duplicate_count: dupe_count,
                    unsafe_headers:  unsafenames_vec,
                    safe_headers:    safenames_vec,
                };
                // its OK to have unwrap here because safenames_struct is always valid
                if safenames_mode == SafeNameMode::VerifyVerbosePrettyJSON {
                    println!(
                        "{}",
                        serde_json::to_string_pretty(&safenames_struct).unwrap()
                    );
                } else {
                    println!("{}", serde_json::to_string(&safenames_struct).unwrap());
                };
            }
            _ => eprintln!("{unsafe_count}"),
        }
    }

    Ok(())
}
