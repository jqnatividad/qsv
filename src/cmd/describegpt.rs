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
use serde_json::json;
use std::process::Command;

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

fn get_completion(api_key: &str, content: &str) -> Result<String, reqwest::Error> {
    let mut client = Client::new();

    let request_data = json!({
        "model": "gpt-3.5-turbo-16k",
        "messages": [{"role": "user", "content": content}],
        "temperature": 0.7
    });

    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .body(request_data.to_string())
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
    println!("Note that this command uses a LLM for inference and is therefore prone to inaccurate\ninformation being produced. Ensure verification of output results before using them.\n");

    // Get stats from qsv stats on input file with --everything flag
    let stats = Command::new("qsv")
        .arg("stats")
        .arg("--everything")
        .arg(args.arg_input.clone().unwrap())
        .output()
        .expect("Error: Unable to get stats from qsv.");

    // Parse the stats as &str
    let stats_str = match std::str::from_utf8(&stats.stdout) {
        Ok(val) => val,
        Err(_) => {
            eprintln!("Error: Unable to parse stats as &str.");
            std::process::exit(1);
        }
    };

    // Get frequency from qsv frequency on input file
    let frequency = Command::new("qsv")
        .arg("frequency")
        .arg(args.arg_input.unwrap())
        .output()
        .expect("Error: Unable to get frequency from qsv.");

    // Parse the frequency as &str
    let frequency_str = match std::str::from_utf8(&frequency.stdout) {
        Ok(val) => val,
        Err(_) => {
            eprintln!("Error: Unable to parse frequency as &str.");
            std::process::exit(1);
        }
    };
    
    // println!("Stats:\n{}", stats_str);
    // println!("Frequency:\n{}", frequency_str);

    fn json_addition(flag_json: bool) -> String {
        if flag_json {
            " in JSON format".to_string()
        } else {
            String::new()
        }
    }
    
    fn get_dictionary_prompt(stats: Option<&str>, frequency: Option<&str>, flag_json: bool) -> String {
        let json_add = json_addition(flag_json);
        let mut prompt = format!(
            "\nHere are the columns for each field in a data dictionary:\n\n\
            - Type: the data type of this column\n\
            - Label: a human-friendly label for this column\n\
            - Description: a full description for this column (can be multiple sentences)\n\n\
            Generate a data dictionary{}{} as aforementioned where each field has Name, Type, Label, and Description (so four columns in total) based on the {}",
            json_add,
            if json_add.is_empty() { "" } else { " (as a table with elastic tabstops)" },
            if stats.is_some() && frequency.is_some() {
                format!(
                    "following summary statistics and frequency data from a CSV file.\n\n\
                    Summary Statistics:\n\n\
                    {}\n\n\
                    Frequency:\n\n\
                    {}",
                    stats.unwrap(),
                    frequency.unwrap()
                )
            } else {
                "dataset.".to_string()
            }
        );
        prompt
    }
    
    fn get_description_prompt(stats: Option<&str>, frequency: Option<&str>, flag_json: bool) -> String {
        let json_add = json_addition(flag_json);
        let mut prompt = format!(
            "\nGenerate only a description that is within 8 sentences{} about the entire dataset based on the {}",
            json_add,
            if stats.is_some() && frequency.is_some() {
                format!(
                    "following summary statistics and frequency data derived from the CSV file it came from.\n\n\
                    Summary Statistics:\n\n\
                    {}\n\n\
                    Frequency:\n\n\
                    {}",
                    stats.unwrap(),
                    frequency.unwrap()
                )
            } else {
                "dataset.".to_string()
            }
        );
        prompt.push_str(" Do not output the summary statistics for each field. Do not output the frequency for each field. Do not output data about each field individually, but instead output about the dataset as a whole in one 1-8 sentence description.");
        prompt
    }
    
    fn get_tags_prompt(stats: Option<&str>, frequency: Option<&str>, flag_json: bool) -> String {
        let json_add = json_addition(flag_json);
        let mut prompt = format!(
            "\nA tag is a keyword or label that categorizes datasets with other, similar datasets. Using the right tags makes it easier for others to find and use datasets.\n\n\
            Generate single-word tags{} about the dataset (lowercase only and remove all whitespace) based on the {}",
            json_add,
            if stats.is_some() && frequency.is_some() {
                format!(
                    "following summary statistics and frequency data from a CSV file.\n\n\
                    Summary Statistics:\n\n\
                    {}\n\n\
                    Frequency:\n\n\
                    {}",
                    stats.unwrap(),
                    frequency.unwrap()
                )
            } else {
                "dataset.".to_string()
            }
        );
        prompt
    }

    // Set prompt based on flags, if no --description, --dictionary, or --tags flags, then default to --all
    // Which gets all three
    let prompt = if args.flag_description.is_some() {
        get_description_prompt(Some(stats_str), Some(frequency_str), args.flag_json.unwrap_or(false))
    } else if args.flag_dictionary.is_some() {
        get_dictionary_prompt(Some(stats_str), Some(frequency_str), args.flag_json.unwrap_or(false))
    } else if args.flag_tags.is_some() {
        get_tags_prompt(Some(stats_str), Some(frequency_str), args.flag_json.unwrap_or(false))
    } else {
        get_description_prompt(Some(stats_str), Some(frequency_str), args.flag_json.unwrap_or(false))
    };

    // Run the async function get_completion with Result
    let completion = match get_completion(&api_key, &prompt) {
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
