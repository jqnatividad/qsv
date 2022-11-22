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
    enum SafeNameKind {
        Always,
        Conditional,
        Verify,
    }

    // set SafeNames Mode
    let mut first_letter = args.flag_mode.chars().next().unwrap_or_default();
    first_letter.make_ascii_lowercase();
    let safenames_mode = match first_letter {
        'a' => SafeNameKind::Always,
        'c' => SafeNameKind::Conditional,
        'v' => SafeNameKind::Verify,
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
    let mut changed_count = 0_u16;
    let mut unsafe_count = 0_u16;

    let mut record = csv::StringRecord::from_byte_record_lossy(old_headers.clone());
    match safenames_mode {
        SafeNameKind::Always | SafeNameKind::Conditional => {
            let (safe_headers, changed) =
                util::safe_header_names(&record, true, safenames_mode == SafeNameKind::Conditional);
            changed_count = changed;
            record.clear();
            for header_name in safe_headers {
                record.push_field(&header_name);
            }
        }
        SafeNameKind::Verify => {
            let mut checkednames_vec: Vec<String> = Vec::with_capacity(record.len());
            let mut unsafe_flag;
            for header_name in record.iter() {
                unsafe_flag = false;
                if !util::is_safe_name(header_name) {
                    unsafe_count += 1;
                    unsafe_flag = true;
                }
                if !unsafe_flag && checkednames_vec.contains(&header_name.to_string()) {
                    unsafe_count += 1;
                } else {
                    checkednames_vec.push(header_name.to_string());
                }
            }
        }
    }

    wtr.write_record(record.as_byte_record())?;

    let mut record = csv::ByteRecord::new();
    while rdr.read_byte_record(&mut record)? {
        wtr.write_record(&record)?;
    }

    wtr.flush()?;

    if safenames_mode == SafeNameKind::Verify {
        eprintln!("{unsafe_count}");
    } else {
        eprintln!("{changed_count}");
    }

    Ok(())
}
