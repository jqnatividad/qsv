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
        "price": 2.50
    },
    {
        "fruit": "banana",
        "price": 3.00
    }
]

To convert it to CSV format run:

qsv json fruits.json

And the following is printed to the terminal:

fruit,price
apple,2.5
banana,3.0

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
    --jaq <filter>         Filter JSON data using jaq syntax (https://github.com/01mf02/jaq).

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

use crate::{util, CliError, CliResult};

#[derive(Deserialize)]
struct Args {
    arg_input:   Option<String>,
    flag_jaq:    Option<String>,
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
        let reader = std::io::BufReader::new(file);

        // Return the JSON contents of the file as serde_json::Value
        match serde_json::from_reader(reader) {
            Ok(value) => Ok(value),
            Err(err) => fail_clierror!("Failed to parse JSON from file: {err}"),
        }
    }

    let args: Args = util::get_args(USAGE, argv)?;

    let flattener = Flattener::new();
    let mut output = Vec::<u8>::new();
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
            .filter_map(|val| val.ok());

        let jaq_value = serde_json::Value::from_iter(out);

        // from_iter creates a Value::Array even if the JSON data is an array,
        // so we unwrap this generated Value::Array to get the actual filtered output.
        // This allows the user to filter with '.data' for {"data": [...]} instead of not being able
        // to use '.data'. Both '.data' and '.data[]' should work with this implementation.
        value = if jaq_value
            .as_array()
            .is_some_and(|arr| arr.first().is_some_and(|f| f.is_array()))
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
    let mut headers: Vec<&str> = Vec::new();
    for key in first_dict.keys() {
        headers.push(key.as_str());
    }

    let empty_values = vec![serde_json::Value::Null; 1];
    let values = if value.is_array() {
        value.as_array().unwrap_or(&empty_values)
    } else {
        &vec![value.clone()]
    };

    let csv_writer = csv::WriterBuilder::new().from_writer(&mut output);
    Json2Csv::new(flattener).convert_from_array(values, csv_writer)?;

    // Use qsv select to reorder headers to first dict's keys order
    let mut select_child = std::process::Command::new(env::current_exe()?)
        .arg("select")
        .arg(headers.join(","))
        .stdin(std::process::Stdio::piped())
        .spawn()?;
    let mut stdin = select_child.stdin.take().expect("Failed to open stdin");
    std::thread::spawn(move || {
        stdin.write_all(&output).expect("Failed to write to stdin");
    });

    let select_output = select_child
        .wait_with_output()
        .map_err(|_| CliError::Other("Failed to read stdout".to_string()))?;

    if let Some(output_path) = args.flag_output {
        let mut file = std::fs::File::create(output_path)?;
        let buf = select_output.stdout;
        file.write_all(&buf)?;
        file.flush()?;
    } else {
        print!("{}", String::from_utf8_lossy(&select_output.stdout));
    }

    Ok(())
}
