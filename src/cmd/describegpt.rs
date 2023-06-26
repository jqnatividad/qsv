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
    --max-tokens <value>   Limits the number of generated tokens in the
                           output.
    --json                 Return results in JSON format.

Common options:
    -h, --help             Display this message
"#;

use crate::{util, CliResult};
use reqwest::blocking::Client;
use serde::Deserialize;
use serde_json::json;
use std::{
    env,
    process::Command,
    time::Duration
};

#[derive(Deserialize)]
struct Args {
    arg_input:           Option<String>,
    flag_all:            Option<bool>,
    flag_description:    Option<bool>,
    flag_dictionary:     Option<bool>,
    flag_tags:           Option<bool>,
    flag_max_tokens:     Option<i32>,
    flag_json:           Option<bool>,
}

fn get_completion(api_key: &str, content: &str, max_tokens: Option<i32>) -> String {
    // Create client with timeout
    let timeout_duration = Duration::from_secs(60);
    let client = Client::builder()
        .timeout(timeout_duration)
        .build()
        .unwrap();

        let mut request_data = json!({
            "model": "gpt-3.5-turbo-16k",
            "messages": [{"role": "user", "content": content}],
        });
        
        // If max_tokens is specified, add it to the request data
        if max_tokens.is_some() {
            request_data["max_tokens"] = json!(max_tokens.unwrap());
        }

    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .body(request_data.to_string())
        .send();

    // Get response from OpenAI API
    let response = match response {
        Ok(val) => val,
        Err(err) => {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        }
    };

    let response_body = response.text();

    // Return completion output
    match response_body {
        Ok(val) => val,
        Err(_) => {
            eprintln!("Error: Unable to get response body from OpenAI API.");
            std::process::exit(1);
        }
    }
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;

    // Check for OpenAI API Key in environment variables
    let api_key = match env::var("OPENAI_API_KEY") {
        Ok(val) => {
            if val.is_empty() {
                eprintln!("Error: OPENAI_API_KEY environment variable is empty.");
                std::process::exit(1);
            }
            val
        }
        Err(_) => {
            eprintln!("Error: OPENAI_API_KEY environment variable not found.");
            // Warning message for new command users
            eprintln!("Note that this command uses a LLM for inference and is therefore prone to inaccurate\ninformation being produced. Ensure verification of output results before using them.\n");
            std::process::exit(1);
        }
    };

    // Check for input file errors
    match args.arg_input {
        Some(ref val) => {
            // If input file is not a CSV, print error message
            if !val.ends_with(".csv") {
                eprintln!("Error: Input file must be a CSV.");
                std::process::exit(1);
            }
            // If input file does not exist, print error message
            if !std::path::Path::new(val).exists() {
                eprintln!("Error: Input file does not exist.");
                std::process::exit(1);
            }
        }
        // If no input file, print error message
        None => {
            eprintln!("Error: No input file specified.");
            std::process::exit(1);
        }
    }

    // --all is not currently supported.
    if args.flag_all.is_some() {
        eprintln!("Error: --all is not currently supported.");
        std::process::exit(1);
    }
    // If no inference flags specified, print error message.
    if args.flag_all.is_none() && args.flag_dictionary.is_none() && args.flag_description.is_none() && args.flag_tags.is_none() {
        eprintln!("Error: No inference options specified.");
        std::process::exit(1);
    // If --all flag is specified, but other inference flags are also specified, print error message.
    } else if args.flag_all.is_some() && (args.flag_dictionary.is_some() || args.flag_description.is_some() || args.flag_tags.is_some()) {
        eprintln!("Error: --all option cannot be specified with other inference flags.");
        std::process::exit(1);
    }
    // If --max-tokens is not specified, print warning message that maximum token limit will be used.
    if args.flag_max_tokens.is_none() {
        eprintln!("Warning: No --max-tokens specified. Defaulting to maximum token limit.");
    }
    // If --max-tokens is specified as 0 or less, print error message.
    if args.flag_max_tokens.is_some() && args.flag_max_tokens.unwrap() <= 0 {
        eprintln!("Error: --max-tokens must be greater than 0.");
        std::process::exit(1);
    }

    // Get stats from qsv stats on input file with --everything flag
    println!("Generating stats from {} using qsv stats --everything...", args.arg_input.clone().unwrap());
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
    println!("Generating frequency from {} using qsv frequency...", args.arg_input.clone().unwrap());
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

    // Get addition to prompt based on --json flag
    fn json_addition(flag_json: bool) -> String {
        if flag_json {
            " in JSON format".to_string()
        } else {
            String::new()
        }
    }
    
    // --dictionary
    fn get_dictionary_prompt(stats: Option<&str>, frequency: Option<&str>, flag_json: bool) -> String {
        let json_add = json_addition(flag_json);
        let prompt = format!(
            "\nHere are the columns for each field in a data dictionary:\n\n\
            - Type: the data type of this column\n\
            - Label: a human-friendly label for this column\n\
            - Description: a full description for this column (can be multiple sentences)\n\n\
            Generate a data dictionary{} as aforementioned where each field has Name, Type, Label, and Description (so four columns in total) based on the {}",
            if json_add.is_empty() { " (as a table with elastic tabstops)" } else { " (in JSON format)" },
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
    
    // --description
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
    
    // --tags
    fn get_tags_prompt(stats: Option<&str>, frequency: Option<&str>, flag_json: bool) -> String {
        let json_add = json_addition(flag_json);
        let prompt = format!(
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

    // If args.json is true, then set to true, else false
    let args_json = match args.flag_json {
        Some(true) => true,
        _ => false
    };

    // Set prompt based on flags where --all is not true, but --description, --dictionary, or --tags flags may be true
    // TODO: Allow for multiple true flags and --all
    let prompt = match (args.flag_description, args.flag_dictionary, args.flag_tags) {
        (Some(true), _, _) => get_description_prompt(Some(stats_str), Some(frequency_str), args_json),
        (_, Some(true), _) => get_dictionary_prompt(Some(stats_str), Some(frequency_str), args_json),
        (_, _, Some(true)) => get_tags_prompt(Some(stats_str), Some(frequency_str), args_json),
        _ => {
            eprintln!("Error: No options specified.");
            std::process::exit(1);
        }
    };

    // Get completion from OpenAI API
    println!("Requesting completion from OpenAI API...\n");
    let completion = get_completion(&api_key, &prompt, args.flag_max_tokens);

    // Parse the completion JSON
    let completion_json: serde_json::Value = match serde_json::from_str(&completion) {
        Ok(val) => val,
        Err(_) => {
            eprintln!("Error: Unable to parse completion JSON.");
            std::process::exit(1);
        }
    };
    // If OpenAI API returns error, print error message
    match completion_json {
        serde_json::Value::Object(ref map) => {
            if map.contains_key("error") {
                eprintln!("Error: {}", map["error"]);
                std::process::exit(1);
            }
        }
        _ => {}
    }

    // Set the completion output
    let message = &completion_json["choices"][0]["message"]["content"];
    // Convert escaped characters to normal characters
    let formatted_message = message.to_string().replace("\\n", "\n").replace("\\t", "\t").replace("\\\"", "\"").replace("\\'", "'").replace("\\`", "`");
    // Print the completion output
    println!("{}", formatted_message);

    Ok(())
}
