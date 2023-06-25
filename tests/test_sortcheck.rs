use crate::workdir::Workdir;

#[test]
fn sortcheck_select_notsorted() {
    let wrk = Workdir::new("sortcheck_select_notsorted");
    wrk.create(
        "in.csv",
        vec![
            svec!["1", "d"],
            svec!["2", "c"],
            svec!["3", "b"],
            svec!["4", "a"],
        ],
    );

    let mut cmd = wrk.command("sortcheck");
    cmd.arg("--no-headers")
        .args(["--select", "2"])
        .arg("in.csv");

    wrk.assert_err(&mut cmd);
}

#[test]
fn sortcheck_select_sorted() {
    let wrk = Workdir::new("sortcheck_select_sorted");
    wrk.create(
        "in.csv",
        vec![
            svec!["1", "d"],
            svec!["2", "c"],
            svec!["3", "b"],
            svec!["4", "a"],
        ],
    );

    let mut cmd = wrk.command("sortcheck");
    cmd.arg("--no-headers")
        .args(["--select", "1"])
        .arg("in.csv");

    wrk.assert_success(&mut cmd);
}

#[test]
fn sortcheck_select_unsorted() {
    let wrk = Workdir::new("sortcheck_select_sorted");
    wrk.create(
        "in.csv",
        vec![
            svec!["1", "d"],
            svec!["2", "c"],
            svec!["3", "b"],
            svec!["4", "a"],
        ],
    );

    let mut cmd = wrk.command("sortcheck");
    cmd.arg("--no-headers")
        .args(["--select", "2"])
        .arg("in.csv");

    wrk.assert_err(&mut cmd);
}

#[test]
fn sortcheck_simple_sorted() {
    let wrk = Workdir::new("sortcheck_simple_sorted");
    wrk.create(
        "in.csv",
        vec![
            svec!["1", "d"],
            svec!["2", "c"],
            svec!["3", "b"],
            svec!["4", "a"],
        ],
    );

    let mut cmd = wrk.command("sortcheck");
    cmd.arg("--no-headers").arg("in.csv");

    wrk.assert_success(&mut cmd);
}

#[test]
fn sortcheck_simple_unsorted() {
    let wrk = Workdir::new("sortcheck_simple_unsorted");
    wrk.create(
        "in.csv",
        vec![
            svec!["col11", "col2"],
            svec!["1", "d"],
            svec!["5", "c"],
            svec!["3", "b"],
            svec!["4", "a"],
        ],
    );

    let mut cmd = wrk.command("sortcheck");
    cmd.arg("in.csv");

    wrk.assert_err(&mut cmd);
}

#[test]
fn sortcheck_simple_all() {
    let wrk = Workdir::new("sortcheck_simple_all");
    wrk.create(
        "in.csv",
        vec![
            svec!["col11", "col2"],
            svec!["1", "d"],
            svec!["5", "c"],
            svec!["3", "b"],
            svec!["4", "a"],
            svec!["6", "a"],
            svec!["2", "y"],
            svec!["3", "z"],
        ],
    );

    let mut cmd = wrk.command("sortcheck");
    cmd.arg("--all").arg("in.csv");

    wrk.assert_err(&mut cmd);
}

#[test]
fn sortcheck_simple_all_json() {
    let wrk = Workdir::new("sortcheck_simple_all_json");
    wrk.create(
        "in.csv",
        vec![
            svec!["col11", "col2"],
            svec!["1", "d"],
            svec!["5", "c"],
            svec!["5", "c"],
            svec!["3", "b"],
            svec!["4", "a"],
            svec!["6", "a"],
            svec!["6", "a"],
            svec!["2", "y"],
            svec!["3", "z"],
        ],
    );

    let mut cmd = wrk.command("sortcheck");
    cmd.arg("--all").arg("--json").arg("in.csv");

    let output = cmd.output().unwrap();
    let got_stdout = std::str::from_utf8(&output.stdout).unwrap_or_default();

    assert_eq!(
        got_stdout,
        r#"{"sorted":false,"record_count":9,"unsorted_breaks":2,"dupe_count":-1}
"#
    );
    wrk.assert_err(&mut cmd);
}

#[test]
fn sortcheck_simple_json() {
    let wrk = Workdir::new("sortcheck_simple_json");
    wrk.create(
        "in.csv",
        vec![
            svec!["col11", "col2"],
            svec!["1", "d"],
            svec!["5", "c"],
            svec!["5", "c"],
            svec!["3", "b"],
            svec!["4", "a"],
            svec!["6", "a"],
            svec!["6", "a"],
            svec!["2", "y"],
            svec!["3", "z"],
        ],
    );

    let mut cmd = wrk.command("sortcheck");
    cmd.arg("--json").arg("in.csv");

    let output = cmd.output().unwrap();
    let got_stdout = std::str::from_utf8(&output.stdout).unwrap_or_default();

    assert_eq!(
        got_stdout,
        r#"{"sorted":false,"record_count":9,"unsorted_breaks":2,"dupe_count":-1}
"#
    );
    wrk.assert_err(&mut cmd);
}

#[test]
fn sortcheck_simple_all_json_progressbar() {
    let wrk = Workdir::new("sortcheck_simple_all_json_progressbar");
    wrk.create(
        "in.csv",
        vec![
            svec!["col11", "col2"],
            svec!["1", "d"],
            svec!["5", "c"],
            svec!["5", "c"],
            svec!["3", "b"],
            svec!["4", "a"],
            svec!["6", "a"],
            svec!["6", "a"],
            svec!["2", "y"],
            svec!["3", "z"],
        ],
    );

    let mut cmd = wrk.command("sortcheck");
    cmd.arg("--all")
        .arg("--json")
        .arg("--progressbar")
        .arg("in.csv");

    let output = cmd.output().unwrap();
    let got_stdout = std::str::from_utf8(&output.stdout).unwrap_or_default();

    assert_eq!(
        got_stdout,
        r#"{"sorted":false,"record_count":9,"unsorted_breaks":2,"dupe_count":-1}
"#
    );
    wrk.assert_err(&mut cmd);
}
