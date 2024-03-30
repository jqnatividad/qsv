use crate::workdir::Workdir;

#[test]
fn dedup_normal() {
    let wrk = Workdir::new("dedup_normal");
    wrk.create(
        "in.csv",
        vec![
            svec!["N", "S"],
            svec!["10", "a"],
            svec!["10", "a"],
            svec!["2", "b"],
            svec!["2", "B"],
        ],
    );

    let mut cmd = wrk.command("dedup");
    cmd.arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["N", "S"],
        svec!["10", "a"],
        svec!["2", "B"],
        svec!["2", "b"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn dedup_no_case() {
    let wrk = Workdir::new("dedup_no_case");
    wrk.create(
        "in.csv",
        vec![
            svec!["N", "S"],
            svec!["10", "a"],
            svec!["10", "a"],
            svec!["2", "b"],
            svec!["2", "B"],
        ],
    );

    let mut cmd = wrk.command("dedup");
    cmd.arg("-i").arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["N", "S"], svec!["10", "a"], svec!["2", "B"]];
    assert_eq!(got, expected);
}

#[test]
fn dedup_issue_1381() {
    let wrk = Workdir::new("dedup_issue_1381");
    wrk.create(
        "in.csv",
        vec![
            svec!["office"],
            svec!["Member of legislative assembly"],
            svec!["Member of Legislative Assembly"],
            svec!["Member of Tamil Nadu Legislative Assembly"],
        ],
    );

    let mut cmd = wrk.command("dedup");
    cmd.arg("-i").arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["office"],
        svec!["Member of Legislative Assembly"],
        svec!["Member of Tamil Nadu Legislative Assembly"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn dedup_issue_1665_numeric() {
    let wrk = Workdir::new("dedup_issue_1665_numeric");
    wrk.create(
        "in.csv",
        vec![
            svec!["data"],
            svec!["1"],
            svec!["3"],
            svec!["3"],
            svec!["5"],
            svec!["10"],
        ],
    );

    let mut cmd = wrk.command("dedup");
    cmd.arg("-N").arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["data"],
        svec!["1"],
        svec!["3"],
        svec!["5"],
        svec!["10"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn dedup_select() {
    let wrk = Workdir::new("dedup_select");
    wrk.create(
        "in.csv",
        vec![
            svec!["N", "S"],
            svec!["10", "a"],
            svec!["10", "a"],
            svec!["2", "b"],
            svec!["2", "B"],
        ],
    );

    let mut cmd = wrk.command("dedup");
    cmd.args(["-s", "N"]).arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["N", "S"], svec!["10", "a"], svec!["2", "B"]];
    assert_eq!(got, expected);
}

#[test]
fn dedup_select_issue774() {
    let wrk = Workdir::new("dedup_select_issue774");
    let test_file = wrk.load_test_file("dedup-test.csv");

    let mut cmd = wrk.command("dedup");
    cmd.args(["-s", "id"]).arg(test_file);

    let got: String = wrk.stdout(&mut cmd);
    let expected = wrk.load_test_resource("dedup-by-id-test-expected.csv");

    assert_eq!(got, expected);
}

#[test]
fn dedup_sorted() {
    let wrk = Workdir::new("dedup_sorted");
    wrk.create(
        "in.csv",
        vec![
            svec!["N", "S"],
            svec!["10", "a"],
            svec!["10", "a"],
            svec!["10", "b"],
            svec!["20", "B"],
            svec!["20", "b"],
            svec!["3", "c"],
            svec!["4", "d"],
        ],
    );

    let mut cmd = wrk.command("dedup");
    cmd.arg("--sorted").arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["N", "S"],
        svec!["10", "a"],
        svec!["10", "b"],
        svec!["20", "B"],
        svec!["20", "b"],
        svec!["3", "c"],
        svec!["4", "d"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn dedup_sorted_nocase() {
    let wrk = Workdir::new("dedup_sorted_nocase");
    wrk.create(
        "in.csv",
        vec![
            svec!["N", "S"],
            svec!["10", "a"],
            svec!["10", "A"],
            svec!["10", "a"],
            svec!["10", "A"],
            svec!["11", "c"],
            svec!["20", "b"],
            svec!["20", "b"],
            svec!["20", "B"],
            svec!["20", "B"],
            svec!["3", "c"],
            svec!["4", "d"],
        ],
    );

    let mut cmd = wrk.command("dedup");
    cmd.arg("--sorted").arg("--ignore-case").arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["N", "S"],
        svec!["10", "a"],
        svec!["11", "c"],
        svec!["20", "b"],
        svec!["3", "c"],
        svec!["4", "d"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn dedup_alreadysorted_nocase() {
    let wrk = Workdir::new("dedup_alreadysorted_nocase");
    wrk.create(
        "in.csv",
        vec![
            svec!["N", "S"],
            svec!["10", "a"],
            svec!["10", "a"],
            svec!["10", "a"],
            svec!["100", "a"],
            svec!["100", "a"],
            svec!["20", "b"],
            svec!["20", "b"],
            svec!["20", "B"],
            svec!["20", "B"],
            svec!["3", "c"],
            svec!["4", "d"],
        ],
    );

    let mut cmd = wrk.command("dedup");
    cmd.arg("--ignore-case").arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["N", "S"],
        svec!["10", "a"],
        svec!["100", "a"],
        svec!["20", "B"],
        svec!["3", "c"],
        svec!["4", "d"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn dedup_not_sorted() {
    let wrk = Workdir::new("dedup__not_sorted");
    wrk.create(
        "in.csv",
        vec![
            svec!["N", "S"],
            svec!["30", "c"],
            svec!["10", "a"],
            svec!["10", "a"],
            svec!["20", "b"],
            svec!["20", "B"],
        ],
    );

    let mut cmd = wrk.command("dedup");
    cmd.arg("--sorted").arg("in.csv");

    let got: String = wrk.output_stderr(&mut cmd);
    assert!(got.contains("Aborting! Input not sorted!"));
}

#[test]
fn dedup_not_sorted2() {
    let wrk = Workdir::new("dedup__not_sorted2");
    wrk.create(
        "in.csv",
        vec![
            svec!["N", "S"],
            svec!["10", "a"],
            svec!["10", "a"],
            svec!["20", "b"],
            svec!["20", "B"],
            svec!["1", "c"],
        ],
    );

    let mut cmd = wrk.command("dedup");
    cmd.arg("--sorted").arg("in.csv");

    let got: String = wrk.output_stderr(&mut cmd);
    assert!(got.contains("Aborting! Input not sorted!"));
}
