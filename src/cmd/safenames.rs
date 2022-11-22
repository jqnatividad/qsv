static USAGE: &str = r#"
Modify headers of a CSV to only have "safe" names - guaranteed "database-ready" names. 

Trim leading & trailing whitespaces. Replace whitespace/non-alphanumeric characters with _.
If a column with the same name already exists, append a sequence suffix (e.g. _n).
If name starts with a number, replace it with an _ as well.
Finally, names are limited to 60 characters in length.

Returns exitcode 0 when headers are modified, returning number of modified headers to stderr.
Returns exitcode 1 when no headers are modified.

  Change the name of the columns:
  $ qsv safenames data.csv

For more examples, see https://github.com/jqnatividad/qsv/blob/master/tests/test_safenames.rs.

Usage:
    qsv safenames [options] [<input>]
    qsv safenames --help

safenames options:
    --mode <a|c>           Rename header names to "safe" names - i.e.
                           guaranteed "database-ready" names.
                           It has two modes - Always & Conditional.
                           Always - goes ahead and renames all headers
                           without checking if they're already "safe".
                           Conditional - check first before renaming.
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
        None,
    }

    // set Safe Name Mode
    let first_letter = args.flag_mode.to_lowercase();
    let saferename = if first_letter.starts_with('a') {
        // if it starts with a, Always mode
        SafeNameKind::Always
    } else if first_letter.starts_with('c') {
        // if it starts with c, its Conditional
        SafeNameKind::Conditional
    } else {
        SafeNameKind::None
    };

    let rconfig = Config::new(&args.arg_input)
        .checkutf8(false)
        .delimiter(args.flag_delimiter);

    let mut rdr = rconfig.reader()?;
    let mut wtr = Config::new(&args.flag_output).writer()?;
    let old_headers = rdr.byte_headers()?;
    let mut changed_count = 0_u16;

    let mut record = csv::StringRecord::from_byte_record_lossy(old_headers.clone());
    if saferename != SafeNameKind::None {
        let (safe_headers, changed) =
            util::safe_header_names(&record, true, saferename == SafeNameKind::Conditional);
        changed_count = changed;
        record.clear();
        for header_name in safe_headers {
            record.push_field(&header_name);
        }
    }

    wtr.write_record(record.as_byte_record())?;

    let mut record = csv::ByteRecord::new();
    while rdr.read_byte_record(&mut record)? {
        wtr.write_record(&record)?;
    }

    wtr.flush()?;

    if changed_count == 0 {
        return fail!("No headers modified.");
    }

    eprintln!("{changed_count}");
    Ok(())
}
