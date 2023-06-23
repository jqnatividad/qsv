static USAGE: &str = r#"
Infers extended metadata about a CSV using summary statistics.

Note that this command uses a LLM for inference and is therefore prone to inaccurate
information being produced. Ensure verification of output results before using them.

For examples, see https://github.com/jqnatividad/qsv/blob/master/tests/test_describegpt.rs.

Usage:
    qsv describegpt [options] [<input>]
    qsv describegpt --help

describegpt options:
    -A, --all              Print all extended metadata options output.
    --description          Print a general description of the dataset.
    --dictionary           For each field, prints an inferred type, a 
                           human-readable label, a description, and stats.
    --tags                 Prints tags that categorize the dataset. Useful
                           for grouping datasets and filtering.
    --max-tokens           Limits the number of generated tokens in the
                           output.
    --json                 Return results in JSON format.

Common options:
    -h, --help             Display this message
"#;

use std::env;
use log::info;
use reqwest::blocking::{Client, RequestBuilder};
use serde::{Deserialize, Serialize};

use crate::{config::Config, util, CliResult};

#[derive(Deserialize)]
struct Args {
    arg_input:           Option<String>,
    flag_all:            Option<bool>,
    flag_dictionary:     Option<bool>,
    flag_description:    Option<bool>,
    flag_tags:           Option<bool>,
    flag_max_tokens:     Option<i32>,
    flag_json:           Option<bool>,
}

// Config
const MODEL: &str = "gpt-3.5-turbo-16k";

fn get_completion(api_key: &str) -> Result<String, reqwest::Error> {
    let mut client = Client::new();

    let request_data = r#"
        {
            "model": "gpt-3.5-turbo-16k",
            "messages": [{"role": "user", "content": "Hi, how are you?"}],
            "temperature": 0.7
        }
    "#;

    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .body(request_data)
        .send()?;

    let response_body = response.text()?;

    Ok(response_body)
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;

    // Check for OpenAI API Key in environment variables
    let api_key = match env::var("OPENAI_API_KEY") {
        Ok(val) => val,
        Err(_) => {
            eprintln!("Error: OPENAI_API_KEY environment variable not found.");
            std::process::exit(1);
        }
    };

    // Warning message
    println!("Note that this command uses a LLM for inference and is therefore prone to inaccurate\ninformation being produced. Ensure verification of output results before using them.");

    // Run the async function get_completion with Result
    let completion = match get_completion(&api_key) {
        Ok(val) => val,
        Err(_) => {
            eprintln!("Error: Unable to get completion from OpenAI API.");
            std::process::exit(1);
        }
    };

    // Parse the completion JSON
    let completion_json: serde_json::Value = match serde_json::from_str(&completion) {
        Ok(val) => val,
        Err(_) => {
            eprintln!("Error: Unable to parse completion JSON.");
            std::process::exit(1);
        }
    };  

    // If error, print error message
    match completion_json {
        serde_json::Value::Object(ref map) => {
            if map.contains_key("error") {
                eprintln!("Error: {}", map["error"]);
                std::process::exit(1);
            }
        }
        _ => {}
    }

    // Print the message content
    let message = &completion_json["choices"][0]["message"]["content"];
    println!("{}", message);

    Ok(())
}
