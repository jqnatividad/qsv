static USAGE: &str = r#"
Add a new column enumerating the lines of a CSV file. This can be useful to keep
track of a specific line order, give a unique identifier to each line or even
make a copy of the contents of a column.

The enum function has four modes of operation:

  1. INCREMENT. Add an incremental identifier to each of the lines:
    $ qsv enum file.csv

  2. UUID4. Add a uuid v4 to each of the lines:
    $ qsv enum --uuid4 file.csv

  3. UUID7. Add a uuid v7 to each of the lines:
    $ qsv enum --uuid7 file.csv

  3. CONSTANT. Create a new column filled with a given value:
    $ qsv enum --constant 0

  4. COPY. Copy the contents of a column to a new one:
    $ qsv enum --copy names

  5. HASH. Create a new column with the deterministic hash of the given column/s.
     The hash uses the xxHash algorithm and is platform-agnostic.
     (see https://github.com/DoumanAsh/xxhash-rust for more information):
    $ qsv enum --hash 1- // hash all columns, auto-ignores existing "hash" column
    $ qsv enum --hash col2,col3,col4 // hash specific columns
    $ qsv enum --hash col2 // hash a single column
    $ qsv enum --hash /record_id|name|address/ // hash columns that match a regex
    $ qsv enum --hash !/record_id/ // hash all columns except the record_id column

  Finally, you should also be able to shuffle the lines of a CSV file by sorting
  on the generated uuid4s:
    $ qsv enum --uuid4 file.csv | qsv sort -s uuid > shuffled.csv

  This will shuffle the lines of the file.csv file as uuids generated using the v4
  specification are random and for practical purposes, are unique (1 in 2^122).
  See https://en.wikipedia.org/wiki/Universally_unique_identifier#Collisions

  However, sorting on uuid7 identifiers will not work as they are time-based
  and monotonically increasing, and will not shuffle the lines.

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
    --uuid4                  When set, the column will be populated with
                             uuids (v4) instead of the incremental identifier.
                             Changes the default column name to "uuid4".
    --uuid7                  When set, the column will be populated with
                             uuids (v7) instead of the incremental identifier.
                             uuid v7 is a time-based uuid and is monotonically increasing.
                             See https://buildkite.com/blog/goodbye-integers-hello-uuids
                             Changes the default column name to "uuid7".
    --hash <columns>         Create a new column filled with the hash of the
                             given column/s. Use "1-" to hash all columns.
                             Changes the default column name to "hash".
                             Will remove an existing "hash" column if it exists.

                             The <columns> argument specify the columns to use
                             in the hash. Columns can be referenced by name or index,
                             starting at 1. Specify multiple columns by separating
                             them with a comma. Specify a range of columns with `-`.
                             (See 'qsv select --help' for the full syntax.)

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
use xxhash_rust::xxh3::xxh3_64;

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
    flag_uuid4:      bool,
    flag_uuid7:      bool,
    flag_hash:       Option<SelectColumns>,
    flag_output:     Option<String>,
    flag_no_headers: bool,
    flag_delimiter:  Option<Delimiter>,
}

enum EnumOperation {
    Increment,
    Uuid4,
    Uuid7,
    Constant,
    Copy,
    Hash,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;
    let mut rconfig = Config::new(&args.arg_input)
        .delimiter(args.flag_delimiter)
        .no_headers(args.flag_no_headers);

    let mut rdr = rconfig.reader()?;
    let mut wtr = Config::new(&args.flag_output).writer()?;

    let mut headers = rdr.byte_headers()?.clone();
    let mut hash_index = None;

    let mut copy_index = 0;
    let mut copy_operation = false;

    if let Some(column_name) = args.flag_copy {
        rconfig = rconfig.select(column_name);
        let sel = rconfig.selection(&headers)?;
        copy_index = *sel.iter().next().unwrap();
        copy_operation = true;
    }

    let mut hash_sel = None;
    let mut hash_operation = false;

    if let Some(hash_columns) = &args.flag_hash {
        // get the index of the column named "hash", if it exists
        hash_index = headers.iter().position(|col| col == b"hash");

        // get the original selection
        rconfig = rconfig.select(hash_columns.clone());
        let original_selection = rconfig
            .clone()
            .select(hash_columns.clone())
            .selection(&headers)?;

        // Filter out the "hash" column from the original selection, if it exists
        let filtered_selection = original_selection
            .iter()
            .filter(|&&index| index != hash_index.unwrap_or(usize::MAX))
            .collect::<Vec<_>>();

        // Construct selection string without "hash" column
        let selection_string = filtered_selection
            .iter()
            .map(|&&index| (index + 1).to_string())
            .collect::<Vec<String>>()
            .join(",");

        // Parse the new selection without "hash" column
        let no_hash_column_selection = SelectColumns::parse(&selection_string)?;

        // Update the configuration with the new selection
        rconfig = rconfig.select(no_hash_column_selection);
        hash_sel = Some(rconfig.selection(&headers)?);

        hash_operation = true;
    }

    if !rconfig.no_headers {
        if let Some(column_name) = &args.flag_new_column {
            headers.push_field(column_name.as_bytes());
        } else if args.flag_uuid4 {
            headers.push_field(b"uuid4");
        } else if args.flag_uuid7 {
            headers.push_field(b"uuid7");
        } else if args.flag_constant.is_some() {
            headers.push_field(b"constant");
        } else if copy_operation {
            let current_header = match simdutf8::compat::from_utf8(&headers[copy_index]) {
                Ok(s) => s,
                Err(e) => return fail_clierror!("Could not parse header as utf-8!: {e}"),
            };
            headers.push_field(format!("{current_header}_copy").as_bytes());
        } else if hash_operation {
            // Remove an existing "hash" column from the header, if it exists
            headers = if let Some(hash_index) = hash_index {
                headers
                    .into_iter()
                    .enumerate()
                    .filter_map(|(i, field)| if i == hash_index { None } else { Some(field) })
                    .collect()
            } else {
                headers
            };
            headers.push_field(b"hash");
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
    } else if args.flag_uuid4 {
        EnumOperation::Uuid4
    } else if args.flag_uuid7 {
        EnumOperation::Uuid7
    } else if copy_operation {
        EnumOperation::Copy
    } else if args.flag_hash.is_some() {
        EnumOperation::Hash
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
    let mut hash_string = String::new();
    let mut hash;
    let uuid7_ctxt = uuid::ContextV7::new();
    let mut uuid;

    while rdr.read_byte_record(&mut record)? {
        match enum_operation {
            EnumOperation::Increment => {
                record.push_field(itoa_buffer.format(counter).as_bytes());
                counter += increment;
            },
            EnumOperation::Uuid4 => {
                uuid = Uuid::new_v4();
                record.push_field(
                    uuid.as_hyphenated()
                        .encode_lower(&mut Uuid::encode_buffer())
                        .as_bytes(),
                );
            },
            EnumOperation::Uuid7 => {
                uuid = Uuid::new_v7(uuid::Timestamp::now(&uuid7_ctxt));
                record.push_field(
                    uuid.as_hyphenated()
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
            EnumOperation::Hash => {
                hash_string.clear();

                // build the hash string from the filtered selection
                if let Some(ref sel) = hash_sel {
                    sel.iter().for_each(|i| {
                        hash_string
                            .push_str(simdutf8::basic::from_utf8(&record[*i]).unwrap_or_default());
                    });
                }
                hash = xxh3_64(hash_string.as_bytes());

                // Optionally remove the "hash" column if it already exists from the output
                record = if let Some(hash_index) = hash_index {
                    record
                        .into_iter()
                        .enumerate()
                        .filter_map(|(i, field)| if i == hash_index { None } else { Some(field) })
                        .collect()
                } else {
                    record
                };
                record.push_field(hash.to_string().as_bytes());
            },
        }

        wtr.write_byte_record(&record)?;
    }
    Ok(wtr.flush()?)
}
