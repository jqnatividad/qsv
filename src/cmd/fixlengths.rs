static USAGE: &str = r#"
Transforms CSV data so that all records have the same length. The length is
the length of the longest record in the data (not counting trailing empty fields,
but at least 1). Records with smaller lengths are padded with empty fields.

This requires two complete scans of the CSV data: one for determining the
record size and one for the actual transform. Because of this, the input
given must be a file and not stdin.

Alternatively, if --length is set, then all records are forced to that length.
This requires a single pass and can be done with stdin.

Usage:
    qsv fixlengths [options] [<input>]
    qsv fixlengths --help

fixlengths options:
    -l, --length <arg>     Forcefully set the length of each record. If a
                           record is not the size given, then it is truncated
                           or expanded as appropriate.
    -i, --insert <pos>     If empty fields need to be inserted, insert them
                           at <pos>. If <pos> is zero, then it is inserted
                           at the end of each record. If <pos> is negative, it
                           is inserted from the END of each record going backwards.
                           If <pos> is positive, it is inserted from the BEGINNING
                           of each record going forward. [default: 0]

Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
    -d, --delimiter <arg>  The field delimiter for reading CSV data.
                           Must be a single character. (default: ,)
"#;

use std::cmp;

use serde::Deserialize;

use crate::{
    config::{Config, Delimiter},
    util, CliResult,
};

#[derive(Deserialize)]
struct Args {
    arg_input:      Option<String>,
    flag_length:    Option<usize>,
    flag_insert:    i16,
    flag_output:    Option<String>,
    flag_delimiter: Option<Delimiter>,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;
    let config = Config::new(&args.arg_input)
        .delimiter(args.flag_delimiter)
        .no_headers(true)
        .flexible(true);
    let length = if let Some(length) = args.flag_length {
        if length == 0 {
            return fail_incorrectusage_clierror!("Length must be greater than 0.");
        }
        length
    } else {
        if config.is_stdin() {
            return fail_incorrectusage_clierror!(
                "<stdin> cannot be used in this command. Please specify a file path."
            );
        }
        let mut maxlen = 0_usize;
        let mut rdr = config.reader()?;
        let mut record = csv::ByteRecord::new();
        while rdr.read_byte_record(&mut record)? {
            let mut nonempty_count = 0;
            for (index, field) in record.iter().enumerate() {
                if index == 0 || !field.is_empty() {
                    nonempty_count = index + 1;
                }
            }
            maxlen = cmp::max(maxlen, nonempty_count);
        }
        maxlen
    };

    let mut rdr = config.reader()?;
    let mut wtr = Config::new(&args.flag_output).writer()?;
    let mut record = csv::ByteRecord::new();
    let mut record_work = csv::ByteRecord::new();
    #[allow(unused_assignments)]
    let mut field_idx = 1_i16;

    let insert_pos = if args.flag_insert < 0 {
        length as i16 + args.flag_insert
    } else {
        args.flag_insert
    };
    log::debug!("length: {length} insert_pos: {insert_pos}");

    while rdr.read_byte_record(&mut record)? {
        if length >= record.len() {
            if args.flag_insert == 0 {
                for _ in record.len()..length {
                    record.push_field(b"");
                }
            } else {
                record_work.clear();
                field_idx = 1_i16;
                for field in &record {
                    if field_idx == insert_pos {
                        // insert all the empty fields at the insert position
                        for _ in record.len()..length {
                            record_work.push_field(b"");
                        }
                    }
                    record_work.push_field(field);

                    field_idx += 1;
                }
                if record_work.len() <= length {
                    // insert all the empty fields at the end
                    for _ in record_work.len()..length {
                        record_work.push_field(b"");
                    }
                }
                record = record_work.clone();
            }
        } else {
            record.truncate(length);
        }
        wtr.write_byte_record(&record)?;
    }
    wtr.flush()?;
    Ok(())
}
