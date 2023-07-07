use crate::{qcheck, workdir::Workdir, CsvData};

// Providing an invalid API key with --openai-key without
// the environment variable set should result in an error
#[test]
fn invalid_api_key() {
    let wrk = Workdir::new("invalid_api_key");
    // Create a CSV file with sample data
    wrk.create_indexed(
        "in.csv",
        vec![
            svec!["letter", "number"],
            svec!["alpha", "13"],
            svec!["beta", "24"],
            svec!["gamma", "37"],
        ],
    );

    // Run the command
    let mut cmd = wrk.command("describegpt");
    cmd.arg("--all")
        .arg("--json")
        .args(["--openai-key", "INVALIDKEY"])
        .arg("in.csv");

    // Error message
    let got_stderr = wrk.output_stderr(&mut cmd);
    // Check that we receive the correct error message
    assert!(got_stderr.contains("Incorrect API key provided: INVALIDKEY"));
}
