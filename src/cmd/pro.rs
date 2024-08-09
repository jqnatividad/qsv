static USAGE: &str = r#"
Interact with qsv pro API.

Notes:

- You must have an activated qsv pro instance (free trial or purchased) to use this command.
- Your device must be connected to the Internet (to verify activated instance of qsv pro).
- qsv pro must be running for this command to work as intended.
- You may learn more about qsv pro at: https://qsvpro.dathere.com.

The qsv pro command has subcommands:
    lens:     Run csvlens on a local file in a new Alacritty terminal window (Windows only).
    workflow: Import a local file into the qsv pro Workflow.

Usage:
    qsv pro lens [options] [<input>]
    qsv pro workflow [options] [<input>]
    qsv pro --help

pro arguments:
    <input>               The input file path to send to the qsv pro API.
                          This must be a local file path, not stdin.
                          Workflow supports: CSV, TSV, SSV, TAB, XLSX, XLS, XLSB, XLSM, ODS.

Common options:
    -h, --help             Display this message
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

    let endpoint = if args.cmd_lens { "lens" } else { "workflow" };
    let res = reqwest::blocking::Client::new()
        .post(format!("http://localhost:14462/api/v1/{}", endpoint))
        .json(&payload)
        .send()?;

    // Handle response from API
    #[derive(Deserialize)]
    struct Status {
        success: bool,
    }
    if res.json::<Status>()?.success {
        println!("Successfully interacted with qsv pro API.");
    } else {
        println!("Error while interacting with qsv pro API.");
    }

    Ok(())
}
