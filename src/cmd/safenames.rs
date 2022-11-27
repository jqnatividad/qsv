static USAGE: &str = r#"
Modify headers of a CSV to only have "safe" names - guaranteed "database-ready" names
(optimized specifically for PostgreSQL column identifiers). 

Fold to lowercase. Trim leading & trailing whitespaces. Replace whitespace/non-alphanumeric
characters with _. If the first character is a digit, replace the digit with _.
If a header with the same name already exists, append a sequence suffix (e.g. c1, c1_2, c1_3).
Names are limited to 60 characters in length. Empty names are replaced with _.

In Always and Conditional mode, returns number of modified headers to stderr, and sends
CSV with safe headers output to stdout.

In Verify Mode, returns number of unsafe headers to stderr.
In Verbose Mode, returns number of headers; unsafe & safe headers; and duplicate count to stderr.
No stdout output is generated in Verify and Verbose mode.

Given data.csv:
 c1,12_col,Col with Embedded Spaces,,Column!@Invalid+Chars,c1
 1,a2,a3,a4,a5,a6

  $ qsv safenames data.csv
  c1,_2_col,col_with_embedded_spaces,_,column__invalid_chars,c1_2
  1,a2,a3,a4,a5,a6
  stderr: 5

  Conditionally rename headers, allowing "quoted identifiers":
  $ qsv safenames --mode c data.csv
  c1,_2_col,Col with Embedded Spaces,_,column__invalid_chars,c1_2
  1,a2,a3,a4,a5,a6
  stderr: 4

  Verify how many "unsafe" headers are found:
  $ qsv safenames --mode v data.csv
  stderr: 5

  Verbose mode:
  $ qsv safenames --mode V safenames.csv
  stderr: 6 header/s
  5 unsafe header/s: ["c1", "12_col", "Col with Embedded Spaces", "", "Column!@Invalid+Chars"]
  2 safe header/s: ["c1", "Col with Embedded Spaces"]
  1 duplicate/s found.

For more examples, see https://github.com/jqnatividad/qsv/blob/master/tests/test_safenames.rs.

Usage:
    qsv safenames [options] [<input>]
    qsv safenames --help

safenames options:
    --mode <c|a|v|V>       Rename header names to "safe" names - i.e.
                           guaranteed "database-ready" names.
                           It has four modes - conditional, always, verify & Verbose.
                           conditional (c) - check first before renaming and allow
                           "quoted identifiers" - mixed case with embedded spaces.
                           always (a) - goes ahead and renames all headers
                           without checking if they're already "safe".
                           verify (v) - count "unsafe" header names without
                           modifying them. Note that verify does not count
                           "quoted identifiers" as unsafe.
                           verbose (V) - like verify, but verbose, showing
                           total header count, unsafe headers & safe headers.
                           [default: Always]
Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
                           Note that no output is generated for Verify and
                           Verbose modes.
    -d, --delimiter <arg>  The field delimiter for reading CSV data.
                           Must be a single character. (default: ,)
"#;

use serde::Deserialize;

use crate::{
    config::{Config, Delimiter},
    util, CliResult,
};

#[derive(Deserialize)]
struct Args {
    arg_input:      Option<String>,
    flag_mode:      String,
    flag_output:    Option<String>,
    flag_delimiter: Option<Delimiter>,
}

#[derive(PartialEq)]
enum SafeNameMode {
    Always,
    Conditional,
    Verify,
    VerifyVerbose,
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
        _ => {
            return fail_clierror!("Invalid mode: {}", args.flag_mode);
        }
    };

    let rconfig = Config::new(&args.arg_input)
        .checkutf8(true)
        .delimiter(args.flag_delimiter);

    let mut rdr = rconfig.reader()?;
    let mut wtr = Config::new(&args.flag_output).writer()?;
    let old_headers = rdr.byte_headers()?;

    let mut headers = csv::StringRecord::from_byte_record_lossy(old_headers.clone());
    if let SafeNameMode::Conditional | SafeNameMode::Always = safenames_mode {
        let (safe_headers, changed_count) =
            util::safe_header_names(&headers, true, safenames_mode == SafeNameMode::Conditional);

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
            let unsafe_flag = if util::is_safe_name(header_name) {
                if !safenames_vec.contains(&header_name.to_string()) {
                    safenames_vec.push(header_name.to_string());
                }
                false
            } else {
                unsafenames_vec.push(header_name.to_string());
                unsafe_count += 1;
                true
            };

            // check for duplicate headers/columns
            // we use the unsafe_flag so we dont' double unsafe count
            // an already unsafe header that's also a duplicate
            if checkednames_vec.contains(&header_name.to_string()) {
                dupe_count += 1;
            } else {
                if !unsafe_flag {
                    unsafenames_vec.push(header_name.to_string());
                    unsafe_count += 1;
                }
                checkednames_vec.push(header_name.to_string());
            }
        }

        if safenames_mode == SafeNameMode::VerifyVerbose {
            eprintln!(
                "{} header/s\n{unsafe_count} unsafe header/s: {unsafenames_vec:?}\n{} safe \
                 header/s: {safenames_vec:?}\n{dupe_count} duplicate/s",
                headers.len(),
                safenames_vec.len()
            );
        } else {
            eprintln!("{unsafe_count}");
        }
    }

    Ok(())
}
