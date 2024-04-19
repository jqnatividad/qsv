static USAGE: &str = r#"
Infers extended metadata about a CSV using a large language model.

Note that this command uses LLMs for inferencing and is therefore prone to
inaccurate information being produced. Verify output results before using them.

For examples, see https://github.com/jqnatividad/qsv/blob/master/tests/test_describegpt.rs.

For more detailed info on how describegpt works and how to prepare a prompt file, 
see https://github.com/jqnatividad/qsv/blob/master/docs/Describegpt.md

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
    --api-key <key>        The API key to use. If using Ollama, set the key to ollama.
                           If the QSV_OPENAI_KEY envvar is set, it will be used instead.                           
    --max-tokens <value>   Limits the number of generated tokens in the output.
                           [default: 50]
    --json                 Return results in JSON format.
    --jsonl                Return results in JSON Lines format.
    --prompt-file <file>   The JSON file containing the prompts to use for inferencing.
                           If not specified, default prompts will be used.
    --base-url <url>       The endpoint for interacting with LLMs. Can be used to run local
                           local LLMs with Ollama, Jan, etc.
                           [default: https://api.openai.com/v1]
    --ollama               Required flag when using Ollama.
    --model <model>        The model to use for inferencing.
                           [default: gpt-3.5-turbo-16k]
    --timeout <secs>       Timeout for completions in seconds.
                           [default: 60]
    --user-agent <agent>   Specify custom user agent. It supports the following variables -
                           $QSV_VERSION, $QSV_TARGET, $QSV_BIN_NAME, $QSV_KIND and $QSV_COMMAND.
                           Try to follow the syntax here -
                           https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/User-Agent

Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
    -Q, --quiet            Do not print status messages to stderr.
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
    flag_api_key:     Option<String>,
    flag_max_tokens:  u16,
    flag_base_url:    String,
    flag_ollama:      bool,
    flag_model:       Option<String>,
    flag_json:        bool,
    flag_jsonl:       bool,
    flag_prompt_file: Option<String>,
    flag_user_agent:  Option<String>,
    flag_timeout:     u16,
    flag_output:      Option<String>,
    flag_quiet:       bool,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct PromptFile {
    name:               String,
    description:        String,
    author:             String,
    version:            String,
    tokens:             u16,
    dictionary_prompt:  String,
    description_prompt: String,
    tags_prompt:        String,
    json:               bool,
    jsonl:              bool,
}

const OPENAI_KEY_ERROR: &str = "Error: QSV_OPENAI_KEY environment variable not found.\nNote that \
                                this command uses LLMs for inferencing and is therefore prone to \
                                inaccurate information being produced. Verify output results \
                                before using them.";

fn print_status(args: &Args, msg: &str) {
    if !args.flag_quiet {
        eprintln!("{msg}");
    }
}

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
    // Send request to API
    let mut request = match method {
        "GET" => client.get(url),
        "POST" => client.post(url).body(request_data.unwrap().to_string()),
        other => {
            let error_json = json!({"Error: Unsupported HTTP method ": other});
            return fail_clierror!("{error_json}");
        },
    };

    // If API key is provided, add it to the request header
    if let Some(key) = api_key {
        request = request.header("Authorization", format!("Bearer {key}"));
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

// Check if model is valid, including the default model
fn is_valid_model(client: &Client, api_key: Option<&str>, args: &Args) -> CliResult<bool> {
    let models_endpoint = match args.flag_ollama {
        true => "/api/tags",
        false => "/models",
    };
    let response = send_request(
        client,
        api_key,
        None,
        "GET",
        format!("{0}{1}", args.flag_base_url, models_endpoint).as_str(),
    );

    // If response is an error, return the error with fail!
    if let Err(e) = response {
        return fail_clierror!("Error while requesting models: {e}",);
    }

    // Verify model is valid from response {"data": [{"id": "model-id", ...}, ...]
    let response_json: serde_json::Value = response.unwrap().json().unwrap();
    if args.flag_ollama {
        let models = response_json["models"].as_array().unwrap();
        for model in models {
            if model["name"].as_str().unwrap() == args.flag_model.clone().unwrap() {
                return Ok(true);
            }
        }
    } else {
        let models = response_json["data"].as_array().unwrap();
        for model in models {
            if model["id"].as_str().unwrap() == args.flag_model.clone().unwrap() {
                return Ok(true);
            }
        }
    }

    // If model is not valid, return false
    Ok(false)
}

fn get_prompt_file(args: &Args) -> CliResult<PromptFile> {
    // Get prompt file if --prompt-file is used
    let prompt_file = if let Some(prompt_file) = args.flag_prompt_file.clone() {
        // Read prompt file
        let prompt_file = fs::read_to_string(prompt_file)?;
        // Try to parse prompt file as JSON, if error then show it in JSON format
        let prompt_file: PromptFile = match serde_json::from_str(&prompt_file) {
            Ok(val) => val,
            Err(e) => {
                let error_json = json!({"error": e.to_string()});
                return fail_clierror!("{error_json}");
            },
        };
        prompt_file
    }
    // Otherwise, get default prompt file
    else {
        #[allow(clippy::let_and_return)]
        let default_prompt_file = PromptFile {
            name:               "My Prompt File".to_string(),
            description:        "My prompt file for qsv's describegpt command.".to_string(),
            author:             "My Name".to_string(),
            version:            "1.0.0".to_string(),
            tokens:             50,
            dictionary_prompt:  "Here are the columns for each field in a data dictionary:\n\n- \
                                 Type: the data type of this column\n- Label: a human-friendly \
                                 label for this column\n- Description: a full description for \
                                 this column (can be multiple sentences)\n\nGenerate a data \
                                 dictionary as aforementioned (in JSON output) where each field \
                                 has Name, Type, Label, and Description (so four columns in \
                                 total) based on the following summary statistics and frequency \
                                 data from a CSV file.\n\nSummary \
                                 Statistics:\n\n{stats}\n\nFrequency:\n\n{frequency}"
                .to_string(),
            description_prompt: "Generate only a description that is within 8 sentences about the \
                                 entire dataset{json_add} based on the following summary \
                                 statistics and frequency data derived from the CSV file it came \
                                 from.\n\nSummary \
                                 Statistics:\n\n{stats}\n\nFrequency:\n\n{frequency}\n\nDo not \
                                 output the summary statistics for each field. Do not output the \
                                 frequency for each field. Do not output data about each field \
                                 individually, but instead output about the dataset as a whole in \
                                 one 1-8 sentence description."
                .to_string(),
            tags_prompt:        "A tag is a keyword or label that categorizes datasets with \
                                 other, similar datasets. Using the right tags makes it easier \
                                 for others to find and use datasets.\n\nGenerate single-word \
                                 tags{json_add} about the dataset (lowercase only and remove all \
                                 whitespace) based on the following summary statistics and \
                                 frequency data from a CSV file.\n\nSummary \
                                 Statistics:\n\n{stats}\n\nFrequency:\n\n{frequency}"
                .to_string(),
            json:               true,
            jsonl:              false,
        };
        default_prompt_file
    };
    Ok(prompt_file)
}

// Generate prompt for prompt type based on either the prompt file (if given) or default prompts
fn get_prompt(
    prompt_type: &str,
    stats: Option<&str>,
    frequency: Option<&str>,
    args: &Args,
) -> CliResult<String> {
    // Get prompt file if --prompt-file is used, otherwise get default prompt file
    let prompt_file = get_prompt_file(args)?;

    // Get prompt from prompt file
    let prompt = match prompt_type {
        "dictionary_prompt" => prompt_file.dictionary_prompt,
        "description_prompt" => prompt_file.description_prompt,
        "tags_prompt" => prompt_file.tags_prompt,
        _ => {
            return fail_incorrectusage_clierror!("Error: Invalid prompt type: {prompt_type}");
        },
    };
    // Replace variable data in prompt
    let prompt = prompt
        .replace("{stats}", stats.unwrap_or(""))
        .replace("{frequency}", frequency.unwrap_or(""))
        .replace(
            "{json_add}",
            if prompt_file.json
                || prompt_file.jsonl
                || (args.flag_prompt_file.is_none() && (args.flag_json || args.flag_jsonl))
            {
                " (in JSON format)"
            } else {
                ""
            },
        );

    // Return prompt
    Ok(prompt)
}

fn get_completion(
    args: &Args,
    arg_is_some: impl Fn(&str) -> bool,
    api_key: &str,
    messages: &serde_json::Value,
) -> CliResult<String> {
    // Create client with timeout
    let client = create_client(args)?;

    // Verify model is valid
    if !is_valid_model(&client, Some(api_key), args)? {
        return fail!("Error: Invalid model.");
    }

    // If --max-tokens is specified, use it
    let max_tokens = if arg_is_some("--max-tokens") {
        args.flag_max_tokens
    }
    // If --prompt-file is used, use the tokens field from the prompt file
    else if args.flag_prompt_file.clone().is_some() {
        let prompt_file = get_prompt_file(args)?;
        prompt_file.tokens
    }
    // Else use the default max tokens value in USAGE
    else {
        args.flag_max_tokens
    };

    // Create request data
    let request_data = json!({
        "model": args.flag_model,
        "max_tokens": max_tokens,
        "messages": messages,
        "stream": false
    });

    // Get response from POST request to chat completions endpoint
    let completions_endpoint = match args.flag_ollama {
        true => "/api/chat",
        false => "/chat/completions",
    };
    let response = send_request(
        &client,
        Some(api_key),
        Some(&request_data),
        "POST",
        format!("{0}{1}", args.flag_base_url, completions_endpoint).as_str(),
    )?;

    // Parse response as JSON
    let response_json: serde_json::Value = response.json()?;
    // If response is an error, print error message
    if let serde_json::Value::Object(ref map) = response_json {
        if map.contains_key("error") {
            return fail_clierror!("API Error: {}", map["error"]);
        }
    }

    // Get completion from response
    match args.flag_ollama {
        true => {
            let completion = response_json["message"]["content"].as_str().unwrap();
            Ok(completion.to_string())
        },
        false => {
            let completion = response_json["choices"][0]["message"]["content"]
                .as_str()
                .unwrap();
            Ok(completion.to_string())
        },
    }
}

// Check if JSON output is expected
fn is_json_output(args: &Args) -> CliResult<bool> {
    // By default expect plaintext output
    let mut json_output = false;
    // Set expect_json to true if --prompt-file is used & the "json" field is true
    if args.flag_prompt_file.is_some() {
        let prompt_file = get_prompt_file(args)?;
        if prompt_file.json {
            json_output = true;
        }
    }
    // Set expect_json to true if --prompt-file is not used & --json is used
    else if args.flag_json {
        json_output = true;
    }
    Ok(json_output)
}
// Check if JSONL output is expected
fn is_jsonl_output(args: &Args) -> CliResult<bool> {
    // By default expect plaintext output
    let mut jsonl_output = false;
    // Set expect_jsonl to true if --prompt-file is used & the "jsonl" field is true
    if args.flag_prompt_file.is_some() {
        let prompt_file = get_prompt_file(args)?;
        if prompt_file.jsonl {
            jsonl_output = true;
        }
    }
    // Set expect_jsonl to true if --prompt-file is not used & --jsonl is used
    else if args.flag_jsonl {
        jsonl_output = true;
    }
    Ok(jsonl_output)
}

// Generates output for all inference options
fn run_inference_options(
    args: &Args,
    arg_is_some: impl Fn(&str) -> bool,
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
    // Format output by replacing escape characters
    fn format_output(str: &str) -> String {
        str.replace("\\n", "\n")
            .replace("\\t", "\t")
            .replace("\\\"", "\"")
            .replace("\\'", "'")
            .replace("\\`", "`")
    }
    // Generate the plaintext and/or JSON output of an inference option
    fn process_output(
        option: &str,
        output: &str,
        total_json_output: &mut serde_json::Value,
        args: &Args,
    ) -> CliResult<()> {
        // Process JSON output if expected or JSONL output is expected
        if is_json_output(args)? || is_jsonl_output(args)? {
            // Parse the completion JSON
            let completion_json: serde_json::Value = if let Ok(val) = serde_json::from_str(output) {
                // Output is valid JSON
                val
            } else {
                // Output is invalid JSON
                // Default error message in JSON format
                let error_message = format!("Error: Invalid JSON output for {option}.");
                let error_json = json!({"error": error_message});
                // Print error message in JSON format
                print_status(args, format!("{error_json}").as_str());
                print_status(args, format!("Output: {output}").as_str());
                error_json
            };
            total_json_output[option] = completion_json;
        }
        // Process plaintext output
        else {
            let formatted_output = format_output(output);
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

    // Get completion from API
    print_status(args, "Interacting with API...\n");

    let mut total_json_output: serde_json::Value = json!({});
    let mut prompt: String;
    let mut messages: serde_json::Value;
    let mut completion: String;
    let mut dictionary_completion = String::new();

    // Generate dictionary output
    if args.flag_dictionary || args.flag_all {
        prompt = get_prompt("dictionary_prompt", stats_str, frequency_str, args)?;
        print_status(args, "Generating data dictionary from API...");
        messages = get_messages(&prompt, &dictionary_completion);
        dictionary_completion = get_completion(args, &arg_is_some, api_key, &messages)?;
        print_status(args, "Received dictionary completion.");
        process_output(
            "dictionary",
            &dictionary_completion,
            &mut total_json_output,
            args,
        )?;
    }

    // Generate description output
    if args.flag_description || args.flag_all {
        prompt = if args.flag_dictionary {
            get_prompt("description_prompt", None, None, args)?
        } else {
            get_prompt("description_prompt", stats_str, frequency_str, args)?
        };
        messages = get_messages(&prompt, &dictionary_completion);
        print_status(args, "Generating description from API...");
        completion = get_completion(args, &arg_is_some, api_key, &messages)?;
        print_status(args, "Received description completion.");
        process_output("description", &completion, &mut total_json_output, args)?;
    }

    // Generate tags output
    if args.flag_tags || args.flag_all {
        prompt = if args.flag_dictionary {
            get_prompt("tags_prompt", None, None, args)?
        } else {
            get_prompt("tags_prompt", stats_str, frequency_str, args)?
        };
        messages = get_messages(&prompt, &dictionary_completion);
        print_status(args, "Generating tags from API...");
        completion = get_completion(args, &arg_is_some, api_key, &messages)?;
        print_status(args, "Received tags completion.");
        process_output("tags", &completion, &mut total_json_output, args)?;
    }

    // Expecting JSON output
    if is_json_output(args)? && !is_jsonl_output(args)? {
        // Format & print JSON output
        let formatted_output =
            format_output(&serde_json::to_string_pretty(&total_json_output).unwrap());
        println!("{formatted_output}");
        // Write to file if --output is used, or overwrite if already exists
        if let Some(output_file_path) = args.flag_output.clone() {
            fs::write(output_file_path, formatted_output)?;
        }
    }
    // Expecting JSONL output
    else if is_jsonl_output(args)? {
        // If --prompt-file is used, add prompt file name and timestamp to JSONL output
        if args.flag_prompt_file.clone().is_some() {
            let prompt_file = get_prompt_file(args)?;
            total_json_output["prompt_file"] = json!(prompt_file.name);
            total_json_output["timestamp"] = json!(chrono::offset::Utc::now().to_rfc3339());
        }
        // Format & print JSONL output
        let formatted_output = format_output(&serde_json::to_string(&total_json_output).unwrap());
        println!("{formatted_output}");
        // Write to file if --output is used, or append if already exists
        if let Some(output_file_path) = args.flag_output.clone() {
            fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(output_file_path)?
                .write_all(format!("\n{formatted_output}").as_bytes())?;
        }
    }

    Ok(())
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;
    // Closure to check if the user gives an argument
    let arg_is_some = |arg: &str| -> bool { argv.contains(&arg) };

    // Check for QSV_OPENAI_KEY in environment variables
    let api_key = match env::var("QSV_OPENAI_KEY") {
        Ok(val) => {
            if val.is_empty() {
                return fail!("Error: QSV_OPENAI_KEY environment variable is empty.");
            }
            val
        },
        Err(_) => {
            // Check if the --api-key flag is present
            if let Some(api_key) = args.flag_api_key.clone() {
                if api_key.is_empty() {
                    return fail!(OPENAI_KEY_ERROR);
                }
                api_key
            } else {
                return fail!(OPENAI_KEY_ERROR);
            }
        },
    };

    // Check if user gives arg_input
    if args.arg_input.is_none() {
        return fail_incorrectusage_clierror!("Error: No input file specified.");
    }

    // Process input file
    // support stdin and auto-decompress snappy file
    // stdin/decompressed file is written to a temporary file in tmpdir
    // which is automatically deleted after the command finishes
    let tmpdir = tempfile::tempdir()?;
    let work_input = process_input(
        vec![PathBuf::from(
            // if no input file is specified, read from stdin "-"
            args.arg_input.clone().unwrap_or_else(|| "-".to_string()),
        )],
        &tmpdir,
        "",
    )?;
    // safety: we just checked that there is at least one input file
    let input_path = work_input[0]
        .canonicalize()?
        .into_os_string()
        .into_string()
        .unwrap();

    // If no inference flags specified, print error message.
    if !args.flag_all && !args.flag_dictionary && !args.flag_description && !args.flag_tags {
        return fail_incorrectusage_clierror!("Error: No inference options specified.");
    // If --all flag is specified, but other inference flags are also set, print error message.
    } else if args.flag_all && (args.flag_dictionary || args.flag_description || args.flag_tags) {
        return fail_incorrectusage_clierror!(
            "Error: --all option cannot be specified with other inference flags."
        );
    }
    // If --prompt-file flag is specified but the prompt file does not exist, print error message.
    if let Some(prompt_file) = args.flag_prompt_file.clone() {
        if !PathBuf::from(prompt_file.clone()).exists() {
            return fail_incorrectusage_clierror!(
                "Error: Prompt file '{prompt_file}' does not exist."
            );
        }
    }
    // If --json and --jsonl flags are specified, print error message.
    if is_json_output(&args)? && is_jsonl_output(&args)? {
        return fail_incorrectusage_clierror!(
            "Error: --json and --jsonl options cannot be specified together."
        );
    }

    // Get qsv executable's path
    let qsv_path = env::current_exe().unwrap();
    // Get input file's name
    let input_filename = args.arg_input.clone().unwrap();

    // Get stats from qsv stats on input file with --everything flag
    print_status(
        &args,
        format!("Generating stats from {input_filename} using qsv stats --everything...").as_str(),
    );
    let Ok(stats) = Command::new(qsv_path.clone())
        .arg("stats")
        .arg("--everything")
        .arg(input_path.clone())
        .output()
    else {
        return fail!("Error: Error while generating stats.");
    };

    // Parse the stats as &str
    let Ok(stats_str) = std::str::from_utf8(&stats.stdout) else {
        return fail!("Error: Unable to parse stats as &str.");
    };

    // Get frequency from qsv frequency on input file
    print_status(
        &args,
        format!("Generating frequency from {input_filename} using qsv frequency...").as_str(),
    );
    let Ok(frequency) = Command::new(qsv_path)
        .arg("frequency")
        .arg(input_path)
        .output()
    else {
        return fail!("Error: Error while generating frequency.");
    };

    // Parse the frequency as &str
    let Ok(frequency_str) = std::str::from_utf8(&frequency.stdout) else {
        return fail!("Error: Unable to parse frequency as &str.");
    };

    // Run inference options
    run_inference_options(
        &args,
        arg_is_some,
        &api_key,
        Some(stats_str),
        Some(frequency_str),
    )?;

    Ok(())
}
