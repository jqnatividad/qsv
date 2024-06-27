static USAGE: &str = r#"
Convert non-nested JSON to CSV.

You may provide JSON data either from stdin or a file path.
This command may not work with nested JSON data.

As a basic example, say we have a file fruits.json with contents:

[
    {
        "fruit": "apple",
        "price": 2.5
    },
    {
        "fruit": "banana",
        "price": 3.0
    }
]

To convert it to CSV format, run:

qsv json fruits.json

And the following is printed to the terminal:

fruit,price
apple,2.5
banana,3.0

If fruits.json was provided using stdin then either use - or do not provide a file path. For example:

cat fruits.json | qsv json -

For more examples, see https://github.com/jqnatividad/qsv/blob/master/tests/test_json.rs.

Usage:
    qsv json [options] [<input>]
    qsv json --help

Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
"#;

use std::{
    env,
    io::{Read, Write},
};

use json_objects_to_csv::{flatten_json_object::Flattener, Json2Csv};
use serde::Deserialize;

use crate::{util, CliError, CliResult};

#[derive(Deserialize)]
struct Args {
    arg_input:   Option<String>,
    flag_output: Option<String>,
}

impl From<json_objects_to_csv::Error> for CliError {
    fn from(err: json_objects_to_csv::Error) -> Self {
        match err {
            json_objects_to_csv::Error::Flattening(err) => {
                CliError::Other(format!("Flattening error: {err}"))
            },
            json_objects_to_csv::Error::FlattenedKeysCollision => {
                CliError::Other(format!("Flattening Key Collission error: {err}"))
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
    let value = match args.arg_input {
        Some(path) => get_value_from_path(path)?,
        _ => get_value_from_stdin()?,
    };
    let csv_writer = csv::WriterBuilder::new().from_writer(&mut output);

    if value.is_null() {
        return fail_clierror!("No JSON data found.");
    }
    // safety: value is not null
    let first_dict = value
        .as_array()
        .unwrap()
        .first()
        .unwrap()
        .as_object()
        .unwrap();
    let mut headers: Vec<&str> = Vec::new();
    for key in first_dict.keys() {
        headers.push(key.as_str());
    }

    let empty_values = vec![serde_json::Value::Null; 1];
    let values = value.as_array().unwrap_or(&empty_values);
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
        let mut file = std::fs::File::create(&output_path)?;
        let buf = select_output.stdout;
        file.write_all(&buf)?;
        file.flush()?;
    } else {
        print!("{}", String::from_utf8_lossy(&select_output.stdout));
    }

    Ok(())
}
