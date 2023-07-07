use crate::workdir::Workdir;

// Providing an invalid API key with --openai-key without
// the environment variable set should result in an error
#[test]
fn describegpt_invalid_api_key() {
    let wrk = Workdir::new("describegpt");
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

// Valid use of describegpt
#[test]
#[ignore = "Requires environment variable to be set."]
fn describegpt_valid() {
    let wrk = Workdir::new("describegpt");

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
    cmd.arg("--all").arg("in.csv");

    // Check that the command ran successfully
    wrk.assert_success(&mut cmd);
}

// Valid use of describegpt with --json
#[test]
#[ignore = "Requires environment variable to be set."]
fn describegpt_valid_json() {
    let wrk = Workdir::new("describegpt");

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
    cmd.arg("--all").arg("--json").arg("in.csv");

    // Check that the output is valid JSON
    let got = wrk.stdout::<String>(&mut cmd);
    match serde_json::from_str::<serde_json::Value>(&got) {
        Ok(_) => (),
        Err(e) => assert!(false, "Error parsing JSON: {e}"),
    }

    // Check that the command ran successfully
    wrk.assert_success(&mut cmd);
}
