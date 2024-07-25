static USAGE: &str = r#"
Convert JSON to CSV.

The JSON data is expected to be non-empty and non-nested as either:

1. An array of objects where:
   A. All objects are non-empty and have the same keys.
   B. Values are not objects or arrays.
2. An object where values are not objects or arrays.

If your JSON data is not in the expected format and/or is nested or complex, try using
the --jaq option to pass a jq-like filter before parsing with the above constraints.

As an example, say we have the following JSON data in a file fruits.json:

[
    {
        "fruit": "apple",
        "price": 2.50,
        "calories": 95
    },
    {
        "fruit": "banana",
        "price": 3.00,
        "calories": 105
    }
]

To convert it to CSV format run:

qsv json fruits.json

And the following is printed to the terminal:

fruit,price,calories
apple,2.5,95
banana,3.0,105

The order of the columns in the CSV file will be the same as the order of the keys in the first JSON object.
The order of the rows in the CSV file will be the same as the order of the objects in the JSON array.

If you want to select specific columns in the final output, use the --select option, for example:

qsv json fruits.json --select price,fruit

And the following is printed to the terminal:

price,fruit
2.5,apple
3.0,banana

Note: Trailing zeroes in decimal numbers after the decimal are truncated (2.50 becomes 2.5).

If the JSON data was provided using stdin then either use - or do not provide a file path.
For example you may copy the JSON data above to your clipboard then run:

qsv clipboard | qsv json

When JSON data is nested or complex, try using the --jaq option and provide a filter value.
The --jaq option uses jaq (like jq). You may learn more here: https://github.com/01mf02/jaq

For example we have a .json file with a "data" key and the value being the same array as before:

{
    "data": [...]
}

We may run the following to select the JSON file and convert the nested array to CSV:

qsv prompt -F json | qsv json --jaq .data

For more examples, see https://github.com/jqnatividad/qsv/blob/master/tests/test_json.rs.

Usage:
    qsv json [options] [<input>]
    qsv json --help

json options:
    --jaq <filter>         Filter JSON data using jaq syntax (https://github.com/01mf02/jaq),
                           which is identical to the popular JSON command-line tool - jq.
                           https://jqlang.github.io/jq/
                           Note that the filter is applied BEFORE converting JSON to a
                           temporary intermediate CSV file.
    -s, --select <cols>    Select columns in the temporary intermediate CSV file in the order 
                           provided for final output. Otherwise, the order of the columns
                           will be the same as the first object's keys in the JSON data.
                           See 'qsv select --help' for the full syntax.

                           Note however that <cols> NEED to be a comma-delimited list of column NAMES
                           and NOT column INDICES as the order of the columns in the intermediate
                           CSV file is not guaranteed to be the same as the order of the
                           keys in the JSON object.

Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
"#;

use std::{
    env,
    io::{Read, Write},
};

use jaq_interpret::{Ctx, FilterT, ParseCtx, RcIter, Val};
use json_objects_to_csv::{flatten_json_object::Flattener, Json2Csv};
use serde::Deserialize;

use crate::{config, select::SelectColumns, util, CliError, CliResult};

#[derive(Deserialize)]
struct Args {
    arg_input:   Option<String>,
    flag_jaq:    Option<String>,
    flag_select: Option<SelectColumns>,
    flag_output: Option<String>,
}

impl From<json_objects_to_csv::Error> for CliError {
    fn from(err: json_objects_to_csv::Error) -> Self {
        match err {
            json_objects_to_csv::Error::Flattening(err) => {
                CliError::Other(format!("Flattening error: {err}"))
            },
            json_objects_to_csv::Error::FlattenedKeysCollision => {
                CliError::Other(format!("Flattening Key Collision error: {err}"))
            },
            json_objects_to_csv::Error::WrittingCSV(err) => {
                CliError::Other(format!("Writing CSV error: {err}"))
            },
            json_objects_to_csv::Error::ParsingJson(err) => {
                CliError::Other(format!("Parsing JSON error: {err}"))
            },
            json_objects_to_csv::Error::InputOutput(err) => CliError::Io(err),
            json_objects_to_csv::Error::IntoFile(err) => CliError::Io(err.into()),
        }
    }
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    fn get_value_from_stdin() -> CliResult<serde_json::Value> {
        // Create a buffer in memory for stdin
        let mut buffer: Vec<u8> = Vec::new();
        let stdin = std::io::stdin();
        let mut stdin_handle = stdin.lock();
        stdin_handle.read_to_end(&mut buffer)?;
        drop(stdin_handle);

        // Return the JSON contents of the buffer as serde_json::Value
        match serde_json::from_slice(&buffer) {
            Ok(value) => Ok(value),
            Err(err) => fail_clierror!("Failed to parse JSON from stdin: {err}"),
        }
    }

    fn get_value_from_path(path: String) -> CliResult<serde_json::Value> {
        // Open the file in read-only mode with buffer.
        let file = std::fs::File::open(path)?;
        let reader = std::io::BufReader::with_capacity(config::DEFAULT_RDR_BUFFER_CAPACITY, file);

        // Return the JSON contents of the file as serde_json::Value
        match serde_json::from_reader(reader) {
            Ok(value) => Ok(value),
            Err(err) => fail_clierror!("Failed to parse JSON from file: {err}"),
        }
    }

    let args: Args = util::get_args(USAGE, argv)?;

    let flattener = Flattener::new();
    let mut value = if let Some(path) = args.arg_input {
        get_value_from_path(path)?
    } else {
        get_value_from_stdin()?
    };

    if value.is_null() {
        return fail_clierror!("No JSON data found.");
    }

    if let Some(filter) = args.flag_jaq {
        // Parse jaq filter based on JSON input
        let mut defs = ParseCtx::new(Vec::new());
        let (f, _errs) = jaq_parse::parse(filter.as_str(), jaq_parse::main());
        let f = defs.compile(f.unwrap());
        let inputs = RcIter::new(core::iter::empty());
        let out = f
            .run((Ctx::new([], &inputs), Val::from(value.clone())))
            .filter_map(std::result::Result::ok);

        #[allow(clippy::from_iter_instead_of_collect)]
        let jaq_value = serde_json::Value::from_iter(out);

        // from_iter creates a Value::Array even if the JSON data is an array,
        // so we unwrap this generated Value::Array to get the actual filtered output.
        // This allows the user to filter with '.data' for {"data": [...]} instead of not being able
        // to use '.data'. Both '.data' and '.data[]' should work with this implementation.
        value = if jaq_value
            .as_array()
            .is_some_and(|arr| arr.first().is_some_and(serde_json::Value::is_array))
        {
            jaq_value.as_array().unwrap().first().unwrap().to_owned()
        } else {
            jaq_value
        };
    }

    if value.is_null() {
        return fail_clierror!("No JSON data found.");
    }

    let first_dict = if value.is_array() {
        value
            .as_array()
            .and_then(|arr| arr.first())
            .and_then(|val| val.as_object())
            .ok_or_else(|| CliError::Other("Expected an array of objects in JSON".to_string()))?
    } else {
        value
            .as_object()
            .ok_or_else(|| CliError::Other("Expected a JSON object".to_string()))?
    };
    if first_dict.is_empty() {
        return Err(CliError::Other(
            "Expected a non-empty JSON object".to_string(),
        ));
    }
    let mut first_dict_headers: Vec<&str> = Vec::new();
    for key in first_dict.keys() {
        first_dict_headers.push(key.as_str());
    }

    let empty_values = vec![serde_json::Value::Null; 1];
    let values = if value.is_array() {
        value.as_array().unwrap_or(&empty_values)
    } else {
        &vec![value.clone()]
    };

    // STEP 1: create an intermediate CSV tempfile from the JSON data
    // we need to do this so we can use qsv select to reorder headers to first dict's keys order
    // as the order of the headers in the CSV file is not guaranteed to be the same as the order of
    // the keys in the JSON object
    let temp_dir = env::temp_dir();
    let intermediate_csv = temp_dir.join("intermediate.csv");

    // convert JSON to CSV and store it in output_buf
    let mut output_buf = Vec::<u8>::new();
    let csv_buf_writer = csv::WriterBuilder::new().from_writer(&mut output_buf);
    Json2Csv::new(flattener).convert_from_array(values, csv_buf_writer)?;

    // now write output_buf to intermediate_csv
    let intermediate_csv_file = std::fs::File::create(&intermediate_csv)?;
    let mut intermediate_csv_writer = std::io::BufWriter::with_capacity(
        config::DEFAULT_WTR_BUFFER_CAPACITY,
        intermediate_csv_file,
    );
    intermediate_csv_writer.write_all(&output_buf)?;
    intermediate_csv_writer.flush()?;
    drop(output_buf);

    // STEP 2: select the columns to use in the final output
    // if --select is not specified, select in the order of the first dict's keys
    // safety: we checked that first_dict is not empty so headers is not empty
    let sel_cols = args
        .flag_select
        .unwrap_or_else(|| SelectColumns::parse(&first_dict_headers.join(",")).unwrap());

    let sel_rconfig = config::Config::new(&Some(intermediate_csv.to_string_lossy().into_owned()));
    let mut intermediate_csv_rdr = sel_rconfig.reader()?;
    let byteheaders = intermediate_csv_rdr.byte_headers()?;

    // and write the selected columns to the final CSV file
    let sel = sel_rconfig.select(sel_cols).selection(byteheaders)?;
    let mut record = csv::ByteRecord::new();
    let mut final_csv_wtr = config::Config::new(&args.flag_output).writer()?;
    final_csv_wtr.write_record(sel.iter().map(|&i| &byteheaders[i]))?;
    while intermediate_csv_rdr.read_byte_record(&mut record)? {
        final_csv_wtr.write_record(sel.iter().map(|&i| &record[i]))?;
    }

    Ok(final_csv_wtr.flush()?)
}
