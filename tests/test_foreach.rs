#![cfg(target_family = "unix")]
use crate::workdir::Workdir;

#[test]
fn foreach() {
    let wrk = Workdir::new("foreach");
    wrk.create(
        "data.csv",
        vec![svec!["name"], svec!["John"], svec!["Mary"]],
    );
    let mut cmd = wrk.command("foreach");
    cmd.arg("name")
        .arg("echo 'NAME = {}'")
        .arg("data.csv")
        .args(["--dry-run", "false"]);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["NAME = John"], svec!["NAME = Mary"]];
    assert_eq!(got, expected);
}

#[test]
fn foreach_dry_run() {
    let wrk = Workdir::new("foreach_dry_run");
    wrk.create(
        "data.csv",
        vec![svec!["name"], svec!["John"], svec!["Mary"]],
    );
    let mut cmd = wrk.command("foreach");
    cmd.arg("name").arg("echo 'NAME = {}'").arg("data.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"echo NAME = John
echo NAME = Mary"#;
    assert_eq!(got, expected);
}

#[test]
fn foreach_multiple_braces() {
    let wrk = Workdir::new("foreach");
    wrk.create(
        "data.csv",
        vec![svec!["name"], svec!["John"], svec!["Mary"]],
    );
    let mut cmd = wrk.command("foreach");
    cmd.arg("name")
        .arg("echo 'NAME = {}, {}, {}'")
        .arg("data.csv")
        .args(["--dry-run", "false"]);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["NAME = John", " John", " John"],
        svec!["NAME = Mary", " Mary", " Mary"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn foreach_multiple_braces_dry_run() {
    let wrk = Workdir::new("foreach");
    wrk.create(
        "data.csv",
        vec![svec!["name"], svec!["John"], svec!["Mary"]],
    );
    let mut cmd = wrk.command("foreach");
    cmd.arg("name")
        .arg("echo 'NAME = {}, {}, {}'")
        .arg("data.csv")
        .args(["--dry-run", "true"]);

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"echo NAME = John, John, John
echo NAME = Mary, Mary, Mary"#;
    assert_eq!(got, expected);
}

#[test]
fn foreach_special_chars_1171() {
    let wrk = Workdir::new("foreach_special_chars");
    wrk.create(
        "data.csv",
        vec![
            svec!["host"],
            svec!["omadhina.co.NA"],
            svec!["https://www.google.com"],
            svec!["www.apple.com"],
            svec!["https://civic-data-ecosystem.github.io"],
        ],
    );
    let mut cmd = wrk.command("foreach");
    cmd.arg("host")
        .arg("echo 'dig +short {} a'")
        .arg("data.csv")
        .args(["--dry-run", "false"]);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["dig +short omadhina.co.NA a"],
        svec!["dig +short https://www.google.com a"],
        svec!["dig +short www.apple.com a"],
        svec!["dig +short https://civic-data-ecosystem.github.io a"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn foreach_special_chars_1171_dry_run() {
    let wrk = Workdir::new("foreach_special_chars_dry_run");
    wrk.create(
        "data.csv",
        vec![
            svec!["host"],
            svec!["omadhina.co.NA"],
            svec!["https://www.google.com"],
            svec!["www.apple.com"],
            svec!["https://civic-data-ecosystem.github.io"],
        ],
    );
    let mut cmd = wrk.command("foreach");
    cmd.arg("host")
        .arg("echo 'dig +short {} a'")
        .arg("data.csv")
        .args(["--dry-run", "true"]);

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"echo dig +short omadhina.co.NA a
echo dig +short https://www.google.com a
echo dig +short www.apple.com a
echo dig +short https://civic-data-ecosystem.github.io a"#;

    assert_eq!(got, expected);
}

#[test]
fn foreach_unify() {
    let wrk = Workdir::new("foreach_unify");
    wrk.create(
        "data.csv",
        vec![svec!["name"], svec!["John"], svec!["Mary"]],
    );
    let mut cmd = wrk.command("foreach");
    cmd.arg("name")
        .arg("echo 'name,value\n{},1'")
        .arg("--unify")
        .arg("data.csv")
        .args(["--dry-run", "false"]);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["name", "value"],
        svec!["John", "1"],
        svec!["Mary", "1"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn foreach_new_column() {
    let wrk = Workdir::new("foreach_nc");
    wrk.create(
        "data.csv",
        vec![svec!["name"], svec!["John"], svec!["Mary"]],
    );
    let mut cmd = wrk.command("foreach");
    cmd.arg("name")
        .arg("echo 'name,value\n{},1'")
        .arg("--unify")
        .arg("--new-column")
        .arg("current_value")
        .arg("data.csv")
        .args(["--dry-run", "false"]);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["name", "value", "current_value"],
        svec!["John", "1", "John"],
        svec!["Mary", "1", "Mary"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn foreach_multiple_commands_with_shell_script() {
    let wrk = Workdir::new("foreach_multiple_commands_with_shell_script");
    wrk.create(
        "data.csv",
        vec![svec!["name"], svec!["John"], svec!["Mary"]],
    );
    wrk.create_from_string(
        "multiple_commands.sh",
        r#"REVERSED_NAME=$(echo $1 | rev)
echo $1 $REVERSED_NAME"#,
    );
    let mut cmd = wrk.command("foreach");
    cmd.arg("name")
        .arg("sh multiple_commands.sh {}")
        .arg("data.csv")
        .args(["--dry-run", "false"]);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["John nhoJ"], svec!["Mary yraM"]];
    assert_eq!(got, expected);
}

#[test]
fn foreach_multiple_commands_with_shell_script_dry_run() {
    let wrk = Workdir::new("foreach_multiple_commands_with_shell_script_dry_run");
    wrk.create(
        "data.csv",
        vec![svec!["name"], svec!["John"], svec!["Mary"]],
    );
    wrk.create_from_string(
        "multiple_commands.sh",
        r#"REVERSED_NAME=$(echo $1 | rev)
echo $1 $REVERSED_NAME"#,
    );
    let mut cmd = wrk.command("foreach");
    cmd.arg("name")
        .arg("sh multiple_commands.sh {}")
        .arg("data.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = "sh multiple_commands.sh John\nsh multiple_commands.sh Mary";
    assert_eq!(got, expected);
}
