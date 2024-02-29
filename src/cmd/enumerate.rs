static USAGE: &str = r#"
Add a new column enumerating the lines of a CSV file. This can be useful to keep
track of a specific line order, give a unique identifier to each line or even
make a copy of the contents of a column.

The enum function has four modes of operation:

  1. INCREMENT. Add an incremental identifier to each of the lines:
    $ qsv enum file.csv

  2. UUID. Add a uuid v4 to each of the lines:
    $ qsv enum --uuid file.csv

  3. CONSTANT. Create a new column filled with a given value:
    $ qsv enum --constant 0

  4. COPY. Copy the contents of a column to a new one:
    $ qsv enum --copy names

  Finally, note that you should also be able to shuffle the lines of a CSV file
  by sorting on the generated uuids:
    $ qsv enum --uuid file.csv | qsv sort -s uuid > shuffled.csv

Usage:
    qsv enum [options] [<input>]
    qsv enum --help

enum options:
    -c, --new-column <name>  Name of the column to create.
                             Will default to "index".
    --start <value>          The value to start the enumeration from.
                             Only applies in Increment mode.
                             (default: 0)
    --increment <value>      The value to increment the enumeration by.
                             Only applies in Increment mode.
                             (default: 1)
    --constant <value>       Fill a new column with the given value.
                             Changes the default column name to "constant".
                             To specify a null value, pass the literal "<NULL>".
    --copy <column>          Name of a column to copy.
                             Changes the default column name to "{column}_copy".
    --uuid                   When set, the column will be populated with
                             uuids (v4) instead of the incremental identifier.
                             Changes the default column name to "uuid".

Common options:
    -h, --help               Display this message
    -o, --output <file>      Write output to <file> instead of stdout.
    -n, --no-headers         When set, the first row will not be interpreted
                             as headers.
    -d, --delimiter <arg>    The field delimiter for reading CSV data.
                             Must be a single character. (default: ,)
"#;

use serde::Deserialize;
use uuid::Uuid;

use crate::{
    config::{Config, Delimiter},
    select::SelectColumns,
    util, CliResult,
};

const NULL_VALUE: &str = "<NULL>";

#[derive(Deserialize)]
struct Args {
    arg_input:       Option<String>,
    flag_new_column: Option<String>,
    flag_start:      u64,
    flag_increment:  Option<u64>,
    flag_constant:   Option<String>,
    flag_copy:       Option<SelectColumns>,
    flag_uuid:       bool,
    flag_output:     Option<String>,
    flag_no_headers: bool,
    flag_delimiter:  Option<Delimiter>,
}

enum EnumOperation {
    Increment,
    Uuid,
    Constant,
    Copy,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;
    let mut rconfig = Config::new(&args.arg_input)
        .delimiter(args.flag_delimiter)
        .no_headers(args.flag_no_headers);

    let mut rdr = rconfig.reader()?;
    let mut wtr = Config::new(&args.flag_output).writer()?;

    let mut headers = rdr.byte_headers()?.clone();

    let mut copy_index = 0;
    let mut copy_operation = false;

    if let Some(column_name) = args.flag_copy {
        rconfig = rconfig.select(column_name);
        let sel = rconfig.selection(&headers)?;
        copy_index = *sel.iter().next().unwrap();
        copy_operation = true;
    }

    if !rconfig.no_headers {
        if let Some(column_name) = &args.flag_new_column {
            headers.push_field(column_name.as_bytes());
        } else if args.flag_uuid {
            headers.push_field(b"uuid");
        } else if args.flag_constant.is_some() {
            headers.push_field(b"constant");
        } else if copy_operation {
            let current_header = match String::from_utf8(headers[copy_index].to_vec()) {
                Ok(s) => s,
                Err(e) => return fail_clierror!("Could not parse cell as utf-8!: {e}"),
            };
            headers.push_field(format!("{current_header}_copy").as_bytes());
        } else {
            headers.push_field(b"index");
        };

        wtr.write_record(&headers)?;
    }

    let constant_value = if args.flag_constant == Some(NULL_VALUE.to_string()) {
        b""
    } else {
        args.flag_constant.as_deref().unwrap_or("").as_bytes()
    };

    let enum_operation = if args.flag_constant.is_some() {
        EnumOperation::Constant
    } else if args.flag_uuid {
        EnumOperation::Uuid
    } else if copy_operation {
        EnumOperation::Copy
    } else {
        EnumOperation::Increment
    };

    // amortize allocations
    let mut record = csv::ByteRecord::new();
    let mut counter: u64 = args.flag_start;
    let mut itoa_buffer = itoa::Buffer::new();
    #[allow(unused_assignments)]
    let mut colcopy: Vec<u8> = Vec::with_capacity(20);
    let increment = args.flag_increment.unwrap_or(1);

    while rdr.read_byte_record(&mut record)? {
        match enum_operation {
            EnumOperation::Increment => {
                record.push_field(itoa_buffer.format(counter).as_bytes());
                counter += increment;
            },
            EnumOperation::Uuid => {
                let id = Uuid::new_v4();
                record.push_field(
                    id.as_hyphenated()
                        .encode_lower(&mut Uuid::encode_buffer())
                        .as_bytes(),
                );
            },
            EnumOperation::Constant => {
                record.push_field(constant_value);
            },
            EnumOperation::Copy => {
                colcopy = record[copy_index].to_vec();
                record.push_field(&colcopy);
            },
        }

        wtr.write_byte_record(&record)?;
    }
    Ok(wtr.flush()?)
}
