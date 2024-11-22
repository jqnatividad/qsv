use crate::workdir::Workdir;

#[test]
fn replace() {
    let wrk = Workdir::new("replace");
    wrk.create(
        "data.csv",
        vec![
            svec!["identifier", "color"],
            svec!["164.0", "yellow"],
            svec!["165.0", "yellow"],
            svec!["166.0", "yellow"],
            svec!["167.0", "yellow.0"],
        ],
    );
    let mut cmd = wrk.command("replace");
    cmd.arg("\\.0$").arg("").arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["identifier", "color"],
        svec!["164", "yellow"],
        svec!["165", "yellow"],
        svec!["166", "yellow"],
        svec!["167", "yellow"],
    ];
    assert_eq!(got, expected);
    wrk.assert_success(&mut cmd);
}

#[test]
fn replace_regex_literal() {
    let wrk = Workdir::new("replace");
    wrk.create(
        "data.csv",
        vec![
            svec!["identifier", "color"],
            svec!["164.0", "yel$low^"],
            svec!["165.0", "yellow"],
            svec!["166.0", "yellow"],
            svec!["167.0", "yel$low^.0"],
        ],
    );
    let mut cmd = wrk.command("replace");
    cmd.arg("$low^").arg("low").arg("--literal").arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["identifier", "color"],
        svec!["164.0", "yellow"],
        svec!["165.0", "yellow"],
        svec!["166.0", "yellow"],
        svec!["167.0", "yellow.0"],
    ];
    assert_eq!(got, expected);
    wrk.assert_success(&mut cmd);
}

#[test]
fn replace_match() {
    let wrk = Workdir::new("replace_match");
    wrk.create(
        "data.csv",
        vec![
            svec!["identifier", "color"],
            svec!["164.0", "yellow"],
            svec!["165.0", "yellow"],
            svec!["166.0", "yellow"],
            svec!["167.0", "yellow.0"],
        ],
    );
    let mut cmd = wrk.command("replace");
    cmd.arg("\\.0$").arg("").arg("data.csv");

    wrk.assert_success(&mut cmd);
}

#[test]
fn replace_nomatch() {
    let wrk = Workdir::new("replace_nomatch");
    wrk.create(
        "data.csv",
        vec![
            svec!["identifier", "color"],
            svec!["164.5", "yellow"],
            svec!["165.6", "yellow"],
            svec!["166.7", "yellow"],
            svec!["167.8", "yellow.1"],
        ],
    );
    let mut cmd = wrk.command("replace");
    cmd.arg("\\.0$").arg("").arg("data.csv");

    wrk.assert_err(&mut cmd);
}

#[test]
fn replace_nomatch_notone() {
    let wrk = Workdir::new("replace_nomatch_notone");
    wrk.create(
        "data.csv",
        vec![
            svec!["identifier", "color"],
            svec!["164.5", "yellow"],
            svec!["165.6", "yellow"],
            svec!["166.7", "yellow"],
            svec!["167.8", "yellow.1"],
        ],
    );
    let mut cmd = wrk.command("replace");
    cmd.arg("\\.0$").arg("").arg("data.csv").arg("--not-one");

    wrk.assert_success(&mut cmd);
}

#[test]
fn replace_null() {
    let wrk = Workdir::new("replace_null");
    wrk.create(
        "data.csv",
        vec![
            svec!["identifier", "color"],
            svec!["164.0", "yellow"],
            svec!["165.0", "yellow"],
            svec!["166.0", "yellow"],
            svec!["167.0", "yellow.0"],
        ],
    );
    let mut cmd = wrk.command("replace");
    cmd.arg("\\.0$").arg("<NULL>").arg("data.csv");

    let got_err = wrk.output_stderr(&mut cmd);
    let expected_err = "5\n";
    assert_eq!(got_err, expected_err);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["identifier", "color"],
        svec!["164", "yellow"],
        svec!["165", "yellow"],
        svec!["166", "yellow"],
        svec!["167", "yellow"],
    ];
    assert_eq!(got, expected);

    wrk.assert_success(&mut cmd);
}

#[test]
fn replace_unicode() {
    let wrk = Workdir::new("replace");
    wrk.create(
        "data.csv",
        vec![
            svec!["identifier", "color"],
            svec!["164.0", "ŷellow"],
            svec!["165.0", "yellow"],
            svec!["166.0", "yellѳwish"],
            svec!["167.0", "yelloψ"],
            svec!["167.0", "belloψ"],
            svec!["167.0", "bellowish"],
        ],
    );
    let mut cmd = wrk.command("replace");
    cmd.arg("[\\s\\S]ell[\\s\\S]w")
        .arg("Ƀellow")
        .arg("--unicode")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["identifier", "color"],
        svec!["164.0", "Ƀellow"],
        svec!["165.0", "Ƀellow"],
        svec!["166.0", "Ƀellowish"],
        svec!["167.0", "yelloψ"],
        svec!["167.0", "belloψ"],
        svec!["167.0", "Ƀellowish"],
    ];
    assert_eq!(got, expected);

    let got_err = wrk.output_stderr(&mut cmd);
    let expected_err = "4\n";
    assert_eq!(got_err, expected_err);

    wrk.assert_success(&mut cmd);
}

#[test]
fn replace_unicode_envvar() {
    let wrk = Workdir::new("replace");
    wrk.create(
        "data.csv",
        vec![
            svec!["identifier", "color"],
            svec!["164.0", "ŷellow"],
            svec!["165.0", "yellow"],
            svec!["166.0", "yellѳwish"],
            svec!["167.0", "yelloψ"],
            svec!["167.0", "belloψ"],
            svec!["167.0", "bellowish"],
        ],
    );
    let mut cmd = wrk.command("replace");
    cmd.env("QSV_REGEX_UNICODE", "1");
    cmd.arg("[\\s\\S]ell[\\s\\S]w")
        .arg("Ƀellow")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["identifier", "color"],
        svec!["164.0", "Ƀellow"],
        svec!["165.0", "Ƀellow"],
        svec!["166.0", "Ƀellowish"],
        svec!["167.0", "yelloψ"],
        svec!["167.0", "belloψ"],
        svec!["167.0", "Ƀellowish"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn replace_no_headers() {
    let wrk = Workdir::new("replace");
    wrk.create(
        "data.csv",
        vec![
            svec!["164.0", "yellow"],
            svec!["165.0", "yellow"],
            svec!["166.0", "yellow"],
            svec!["167.0", "yellow.0"],
        ],
    );
    let mut cmd = wrk.command("replace");
    cmd.arg("\\.0$")
        .arg("")
        .arg("--no-headers")
        .arg("--select")
        .arg("1")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["164", "yellow"],
        svec!["165", "yellow"],
        svec!["166", "yellow"],
        svec!["167", "yellow.0"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn replace_select() {
    let wrk = Workdir::new("replace");
    wrk.create(
        "data.csv",
        vec![
            svec!["identifier", "color"],
            svec!["164.0", "yellow"],
            svec!["165.0", "yellow"],
            svec!["166.0", "yellow"],
            svec!["167.0", "yellow.0"],
        ],
    );
    let mut cmd = wrk.command("replace");
    cmd.arg("\\.0$")
        .arg("")
        .arg("--select")
        .arg("identifier")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["identifier", "color"],
        svec!["164", "yellow"],
        svec!["165", "yellow"],
        svec!["166", "yellow"],
        svec!["167", "yellow.0"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn replace_groups() {
    let wrk = Workdir::new("replace");
    wrk.create(
        "data.csv",
        vec![
            svec!["identifier", "color"],
            svec!["164.0", "yellow"],
            svec!["165.0", "yellow"],
            svec!["166.0", "yellow"],
            svec!["167.0", "yellow.0"],
        ],
    );
    let mut cmd = wrk.command("replace");
    cmd.arg("\\d+(\\d)\\.0$")
        .arg("$1")
        .arg("--select")
        .arg("identifier")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["identifier", "color"],
        svec!["4", "yellow"],
        svec!["5", "yellow"],
        svec!["6", "yellow"],
        svec!["7", "yellow.0"],
    ];
    assert_eq!(got, expected);
}
