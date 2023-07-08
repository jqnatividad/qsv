static USAGE: &str = r#"
Infers extended metadata about a CSV using summary statistics.

Note that this command uses OpenAI's LLMs for inferencing and is therefore prone to
inaccurate information being produced. Verify output results before using them.

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
    --openai-key <key>     The OpenAI API key to use.
                           If the QSV_OPENAI_KEY envvar is set, it will be used instead.                           
    --max-tokens <value>   Limits the number of generated tokens in the output.
                           [default: 50]
    --json                 Return results in JSON format.
    --model <model>        The model to use for inferencing.
                           [default: gpt-3.5-turbo-16k]
    --timeout <secs>       Timeout for OpenAI completions in seconds.
                           [default: 60]
    --user-agent <agent>   Specify custom user agent. It supports the following variables -
                           $QSV_VERSION, $QSV_TARGET, $QSV_BIN_NAME, $QSV_KIND and $QSV_COMMAND.
                           Try to follow the syntax here -
                           https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/User-Agent

Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
"#;

use std::{env, fs, io::Write, path::PathBuf, process::Command, time::Duration};

use log::log_enabled;
use reqwest::blocking::Client;
use serde::Deserialize;
use serde_json::json;

use crate::{util, util::process_input, CliResult};

#[derive(Deserialize)]
struct Args {
    arg_input:        Option<String>,
    flag_all:         bool,
    flag_description: bool,
    flag_dictionary:  bool,
    flag_tags:        bool,
    flag_openai_key:  Option<String>,
    flag_max_tokens:  u16,
    flag_model:       Option<String>,
    flag_json:        bool,
    flag_user_agent:  Option<String>,
    flag_timeout:     u16,
    flag_output:      Option<String>,
}

const OPENAI_KEY_ERROR: &str = "Error: QSV_OPENAI_KEY environment variable not found.\nNote that \
                                this command uses OpenAI's LLMs for inferencing and is therefore \
                                prone to inaccurate information being produced. Verify output \
                                results before using them.";

fn create_client(args: &Args) -> CliResult<Client> {
    // Create client with timeout
    let timeout_duration = Duration::from_secs(args.flag_timeout.into());
    let client = Client::builder()
        .user_agent(util::set_user_agent(args.flag_user_agent.clone())?)
        .brotli(true)
        .gzip(true)
        .deflate(true)
        .use_rustls_tls()
        .http2_adaptive_window(true)
        .connection_verbose(log_enabled!(log::Level::Debug) || log_enabled!(log::Level::Trace))
        .timeout(timeout_duration)
        .build()?;
    Ok(client)
}

// Send an HTTP request using a client to a URL
// Optionally include an API key and request data
fn send_request(
    client: &Client,
    api_key: Option<&str>,
    request_data: Option<&serde_json::Value>,
    method: &str,
    url: &str,
) -> CliResult<reqwest::blocking::Response> {
    // Send request to OpenAI API
    let mut request = match method {
        "GET" => client.get(url),
        "POST" => client.post(url).body(request_data.unwrap().to_string()),
        _ => {
            // If --json is used, fail! with error message in JSON format
            if let Some(_) = request_data {
                return fail_clierror!("Error: Invalid HTTP method: {method}");
            }
            // If --json is not used, fail! with error message in plaintext
            else {
                return fail!("Error: Invalid HTTP method: {method}");
            }
        }
    };

    // If API key is provided, add it to the request header
    if let Some(key) = api_key {
        request = request.header("Authorization", format!("Bearer {}", key));
    }
    // If request data is provided, add it to the request header
    if let Some(data) = request_data {
        request = request
            .header("Content-Type", "application/json")
            .body(data.to_string());
    }

    // Get response
    let response = request.send()?;

    // If response is an error, return response
    if !response.status().is_success() {
        let output = response.text()?;
        return fail_clierror!("Error response when making request: {output}");
    }

    Ok(response)
}

// Check if model is valid, including the default model https://api.openai.com/v1/models
fn is_valid_model(client: &Client, api_key: Option<&str>, args: &Args) -> CliResult<bool> {
    let response = send_request(
        &client,
        api_key,
        None,
        "GET",
        "https://api.openai.com/v1/models",
    );

    // If response is an error, return the error with fail!
    if let Err(e) = response {
        return fail_clierror!(
            "Error while requesting OpenAI models: {e}",
        );
    }

    // Verify model is valid from response {"data": [{"id": "model-id", ...}, ...]
    let response_json: serde_json::Value = response.unwrap().json().unwrap();
    let models = response_json["data"].as_array().unwrap();
    for model in models {
        if model["id"].as_str().unwrap() == args.flag_model.clone().unwrap() {
            return Ok(true);
        }
    }

    // If model is not valid, return false
    Ok(false)
}

fn get_completion(api_key: &str, messages: &serde_json::Value, args: &Args) -> CliResult<String> {
    // Create client with timeout
    let client = create_client(args)?;

    // Verify model is valid
    if !is_valid_model(&client, Some(api_key), args)? {
        return fail!("Error: Invalid model.");
    }

    // Create request data
    let request_data = json!({
        "model": args.flag_model,
        "max_tokens": args.flag_max_tokens,
        "messages": messages
    });

    // Get response from POST request to OpenAI Chat Completions API
    let response = send_request(
        &client,
        Some(api_key),
        Some(&request_data),
        "POST",
        "https://api.openai.com/v1/chat/completions",
    )?;

    // Parse response as JSON
    let response_json: serde_json::Value = response.json()?;
    // If response is an error, print error message
    if let serde_json::Value::Object(ref map) = response_json {
        if map.contains_key("error") {
            return fail_clierror!("OpenAI API Error: {}", map["error"]);
        }
    }

    // Get completion from response
    let completion = response_json["choices"][0]["message"]["content"].as_str().unwrap();

    Ok(completion.to_string())

}

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
        "\nHere are the columns for each field in a data dictionary:\n\n- Type: the data type of \
         this column\n- Label: a human-friendly label for this column\n- Description: a full \
         description for this column (can be multiple sentences)\n\nGenerate a data dictionary{} \
         as aforementioned where each field has Name, Type, Label, and Description (so four \
         columns in total) based on the {}",
        if json_add.is_empty() {
            " (as a table with elastic tabstops)"
        } else {
            " (in JSON format)"
        },
        if stats.is_some() && frequency.is_some() {
            format!(
                "following summary statistics and frequency data from a CSV file.\n\nSummary \
                 Statistics:\n\n{}\n\nFrequency:\n\n{}",
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
        "\nGenerate only a description that is within 8 sentences{} about the entire dataset \
         based on the {}",
        json_add,
        if stats.is_some() && frequency.is_some() {
            format!(
                "following summary statistics and frequency data derived from the CSV file it \
                 came from.\n\nSummary Statistics:\n\n{}\n\nFrequency:\n\n{}",
                stats.unwrap(),
                frequency.unwrap()
            )
        } else {
            "dataset.".to_string()
        }
    );
    prompt.push_str(
        " Do not output the summary statistics for each field. Do not output the frequency for \
         each field. Do not output data about each field individually, but instead output about \
         the dataset as a whole in one 1-8 sentence description.",
    );
    prompt
}

// --tags
fn get_tags_prompt(stats: Option<&str>, frequency: Option<&str>, flag_json: bool) -> String {
    let json_add = json_addition(flag_json);
    let prompt = format!(
        "\nA tag is a keyword or label that categorizes datasets with other, similar datasets. \
         Using the right tags makes it easier for others to find and use datasets.\n\nGenerate \
         single-word tags{} about the dataset (lowercase only and remove all whitespace) based on \
         the {}",
        json_add,
        if stats.is_some() && frequency.is_some() {
            format!(
                "following summary statistics and frequency data from a CSV file.\n\nSummary \
                 Statistics:\n\n{}\n\nFrequency:\n\n{}",
                stats.unwrap(),
                frequency.unwrap()
            )
        } else {
            "dataset.".to_string()
        }
    );
    prompt
}

// Replace escaped characters with normal characters
fn replace_escape_chars(str: &str) -> String {
    str.replace("\\n", "\n")
        .replace("\\t", "\t")
        .replace("\\\"", "\"")
        .replace("\\'", "'")
        .replace("\\`", "`")
}

// Generates output for all inference options
fn run_inference_options(
    args: &Args,
    api_key: &str,
    stats_str: Option<&str>,
    frequency_str: Option<&str>,
) -> CliResult<()> {
    // Add --dictionary output as context if it is not empty
    fn get_messages(prompt: &str, dictionary_completion: &str) -> serde_json::Value {
        if dictionary_completion.is_empty() {
            json!([{"role": "user", "content": prompt}])
        } else {
            json!([{"role": "assistant", "content": dictionary_completion}, {"role": "user", "content": prompt}])
        }
    }
    // Generate the plaintext and/or JSON output of an inference option
    fn process_output(
        option: &str,
        output: &str,
        total_json_output: &mut serde_json::Value,
        args: &Args,
    ) -> CliResult<()> {
        // If --json is used, expect JSON
        if args.flag_json {
            // Parse the completion JSON
            let completion_json: serde_json::Value = match serde_json::from_str(output) {
                // Output is valid JSON
                Ok(val) => val,
                // Output is not valid JSON
                Err(_) => {
                    // Default error message in JSON format
                    let error_message = format!("Error: Invalid JSON output for {option}.");
                    let error_json = json!({"error": error_message});
                    // Print error message in JSON format
                    eprintln!("{error_json}");
                    error_json
                }
            };
            total_json_output[option] = completion_json;
        }
        // If --json is not used, expect plaintext
        else {
            let formatted_output = replace_escape_chars(output);
            println!("{formatted_output}");
            // If --output is used, append plaintext to file, do not overwrite
            if let Some(output) = args.flag_output.clone() {
                fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(output)?
                    .write_all(formatted_output.as_bytes())?;
            }
        }
        Ok(())
    }

    // Get completion from OpenAI API
    eprintln!("Interacting with OpenAI API...\n");

    let args_json = args.flag_json;
    let mut total_json_output: serde_json::Value = json!({});
    let mut prompt: String;
    let mut messages: serde_json::Value;
    let mut completion: String = String::new();
    let mut dictionary_completion = String::new();
    if args.flag_dictionary || args.flag_all {
        prompt = get_dictionary_prompt(stats_str, frequency_str, args_json);
        eprintln!("Generating data dictionary from OpenAI API...");
        messages = get_messages(&prompt, &dictionary_completion);
        dictionary_completion = get_completion(api_key, &messages, args)?;
        eprintln!("Received dictionary completion.");
        process_output(
            "dictionary",
            &dictionary_completion,
            &mut total_json_output,
            args,
        )?;
    }

    if args.flag_description || args.flag_all {
        prompt = if args.flag_dictionary {
            get_description_prompt(None, None, args_json)
        } else {
            get_description_prompt(stats_str, frequency_str, args_json)
        };
        messages = get_messages(&prompt, &dictionary_completion);
        eprintln!("Generating description from OpenAI API...");
        completion = get_completion(api_key, &messages, args)?;
        eprintln!("Received description completion.");
        process_output(
            "description",
            &completion,
            &mut total_json_output,
            args,
        )?;
    }
    if args.flag_tags || args.flag_all {
        prompt = if args.flag_dictionary {
            get_tags_prompt(None, None, args_json)
        } else {
            get_tags_prompt(stats_str, frequency_str, args_json)
        };
        messages = get_messages(&prompt, &dictionary_completion);
        eprintln!("Generating tags from OpenAI API...");
        completion = get_completion(api_key, &messages, args)?;
        eprintln!("Received tags completion.");
        process_output("tags", &completion, &mut total_json_output, args)?;
    }

    if args.flag_json {
        // Print all JSON output
        let formatted_output =
            replace_escape_chars(&serde_json::to_string_pretty(&total_json_output).unwrap());
        println!("{formatted_output}");
        // If --output is used, write JSON to file
        if let Some(output) = args.flag_output.clone() {
            fs::write(output, formatted_output)?;
        }
    }

    Ok(())
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;

    // Check for QSV_OPENAI_KEY in environment variables
    let api_key = match env::var("QSV_OPENAI_KEY") {
        Ok(val) => {
            if val.is_empty() {
                return fail!("Error: QSV_OPENAI_KEY environment variable is empty.");
            }
            val
        }
        Err(_) => {
            // Check if the --key flag is present
            if let Some(api_key) = args.flag_openai_key.clone() {
                if api_key.is_empty() {
                    return fail!(OPENAI_KEY_ERROR);
                }
                api_key
            } else {
                return fail!(OPENAI_KEY_ERROR);
            }
        }
    };

    // Check if user gives arg_input
    if args.arg_input.is_none() {
        return fail!("Error: No input file specified.");
    }

    // Process input file
    // support stdin and auto-decompress snappy file
    // stdin/decompressed file is written to a temporary file in tmpdir
    // which is automatically deleted after the command finishes
    let tmpdir = tempfile::tempdir()?;
    let work_input = process_input(
        vec![PathBuf::from(
            // if no input file is specified, read from stdin "-"
            args.arg_input.clone().unwrap_or("-".to_string()),
        )],
        &tmpdir,
        "No data on stdin. Please provide at least one input file or pipe data to stdin.",
    )?;
    // safety: we just checked that there is at least one input file
    let arg_input = work_input[0]
        .canonicalize()?
        .clone()
        .into_os_string()
        .into_string()
        .unwrap();

    // If no inference flags specified, print error message.
    if !args.flag_all && !args.flag_dictionary && !args.flag_description && !args.flag_tags {
        return fail!("Error: No inference options specified.");
    // If --all flag is specified, but other inference flags are also set, print error message.
    } else if args.flag_all && (args.flag_dictionary || args.flag_description || args.flag_tags) {
        return fail!("Error: --all option cannot be specified with other inference flags.");
    }

    // Get qsv executable's path
    let root = env::current_exe().unwrap();

    // Get stats from qsv stats on input file with --everything flag
    eprintln!("Generating stats from {arg_input} using qsv stats --everything...");
    let Ok(stats) = Command::new(root.clone())
        .arg("stats")
        .arg("--everything")
        .arg(arg_input.clone())
        .output()
    else {
        return fail!("Error: Error while generating stats.");
    };

    // Parse the stats as &str
    let Ok(stats_str) = std::str::from_utf8(&stats.stdout) else {
        return fail!("Error: Unable to parse stats as &str.");
    };

    // Get frequency from qsv frequency on input file
    eprintln!("Generating frequency from {arg_input} using qsv frequency...");
    let Ok(frequency) = Command::new(root).arg("frequency").arg(arg_input).output() else {
        return fail!("Error: Error while generating frequency.");
    };

    // Parse the frequency as &str
    let Ok(frequency_str) = std::str::from_utf8(&frequency.stdout) else {
        return fail!("Error: Unable to parse frequency as &str.");
    };

    // Run inference options
    run_inference_options(&args, &api_key, Some(stats_str), Some(frequency_str))?;

    Ok(())
}
