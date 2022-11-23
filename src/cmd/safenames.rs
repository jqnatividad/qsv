static USAGE: &str = r#"
Modify headers of a CSV to only have "safe" names - guaranteed "database-ready" names. 

Trim leading & trailing whitespaces. Replace whitespace/non-alphanumeric characters with _.
If a column with the same name already exists, append a sequence suffix (e.g. col1, col1_2, col1_3).
If the first character is a digit, replace the digit with _.
Names are limited to 60 characters in length.
Empty names are replaced with _ as well.

In Always and Conditional mode, returns number of modified headers to stderr.
In Verify Mode, returns number of unsafe headers to stderr.

  Change the name of the columns:
  $ qsv safenames data.csv

For more examples, see https://github.com/jqnatividad/qsv/blob/master/tests/test_safenames.rs.

Usage:
    qsv safenames [options] [<input>]
    qsv safenames --help

safenames options:
    --mode <a|c|v>         Rename header names to "safe" names - i.e.
                           guaranteed "database-ready" names.
                           It has three modes - Always, Conditional & Verify.
                           Always - goes ahead and renames all headers
                           without checking if they're already "safe".
                           Conditional - check first before renaming.
                           Verify - count "unsafe" header names without
                           modifying them.
                           [default: conditional]
Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
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

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;

    #[derive(PartialEq)]
    enum SafeNameMode {
        Always,
        Conditional,
        Verify,
    }

    // set SafeNames Mode
    let mut first_letter = args.flag_mode.chars().next().unwrap_or_default();
    first_letter.make_ascii_lowercase();
    let safenames_mode = match first_letter {
        'c' => SafeNameMode::Conditional,
        'a' => SafeNameMode::Always,
        'v' => SafeNameMode::Verify,
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
        // Verify Mode
        let mut checkednames_vec: Vec<String> = Vec::with_capacity(old_headers.len());
        let mut unsafe_flag;
        let mut unsafe_count = 0_u16;

        for header_name in headers.iter() {
            unsafe_flag = false;
            if !util::is_safe_name(header_name) {
                unsafe_count += 1;
                unsafe_flag = true;
            }

            // check for duplicate headers/columns
            // we use the unsafe_flag so we dont' double unsafe count
            // an already unsafe header that's also a duplicate
            if !unsafe_flag && checkednames_vec.contains(&header_name.to_string()) {
                unsafe_count += 1;
            } else {
                checkednames_vec.push(header_name.to_string());
            }
        }

        eprintln!("{unsafe_count}");
    }

    Ok(())
}
