static USAGE: &str = r#"
Interact with qsv pro API. Learn more about qsv pro at: https://qsvpro.dathere.com.

- qsv pro must be running for this command to work as described.
- Some features of this command require a paid plan of qsv pro and may require an Internet connection.

The qsv pro command has subcommands:
    lens:     Run csvlens on a local file in a new Alacritty terminal emulator window (Windows only).
    workflow: Import a local file into the qsv pro Workflow (Workflow must be open).

Usage:
    qsv pro lens [options] [<input>]
    qsv pro workflow [options] [<input>]
    qsv pro --help

pro arguments:
    <input>               The input file path to send to the qsv pro API.
                          This must be a local file path, not stdin.
                          Workflow supports: CSV, TSV, SSV, TAB, XLSX, XLS, XLSB, XLSM, ODS.

Common options:
    -h, --help            Display this message
"#;

use std::path::PathBuf;

use serde::Deserialize;
use serde_json::json;

use crate::{util, CliResult};

#[allow(dead_code)]
#[derive(Deserialize)]
struct Args {
    arg_input:    PathBuf,
    cmd_lens:     bool,
    cmd_workflow: bool,
}

#[derive(Deserialize)]
struct Status {
    success: bool,
    message: Option<String>,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;

    if !cfg!(windows) && args.cmd_lens {
        println!("Cannot use the lens subcommand on a non-Windows device.");
        return Ok(());
    }

    // Get the absolute file path and send it to the API
    let file_path = args.arg_input.canonicalize()?.to_string_lossy().to_string();
    let payload = json!({
        "file_path": file_path
    });

    // Handle response from API
    let endpoint = if args.cmd_lens { "lens" } else { "workflow" };
    let res = reqwest::blocking::Client::new()
        .post(format!("http://localhost:14462/api/v1/{endpoint}"))
        .json(&payload)
        .send()?;

    let status = res.json::<Status>()?;

    if status.success {
        println!("Successfully interacted with qsv pro API.");
    } else {
        println!("Error while interacting with qsv pro API.");
    }

    if let Some(message) = status.message {
        println!("{message}");
    }

    Ok(())
}
