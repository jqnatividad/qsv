use crate::workdir::Workdir;

fn data(headers: bool) -> Vec<Vec<String>> {
    let mut rows = vec![
        svec!["foobar", "barfoo"],
        svec!["a", "b"],
        svec!["barfoo", "foobar"],
        svec!["is waldo here", "spot"],
        svec!["Ḟooƀar", "ḃarḟoo"],
        svec!["bleh", "no, Waldo is there"],
    ];
    if headers {
        rows.insert(0, svec!["h1", "h2"]);
    }
    rows
}

fn regexset_file() -> Vec<Vec<String>> {
    let rows = vec![svec!["^foo"], svec!["bar$"], svec!["waldo"]];
    rows
}

fn regexset_unicode_file() -> Vec<Vec<String>> {
    let rows = vec![svec!["^foo"], svec!["bar$"], svec!["waldo"], svec!["^Ḟoo"]];
    rows
}

fn empty_regexset_file() -> Vec<Vec<String>> {
    let rows = vec![svec![""]];
    rows
}

#[test]
fn searchset() {
    let wrk = Workdir::new("searchset");
    wrk.create("data.csv", data(true));
    wrk.create("regexset.txt", regexset_file());
    let mut cmd = wrk.command("searchset");
    cmd.arg("regexset.txt").arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["h1", "h2"],
        svec!["foobar", "barfoo"],
        svec!["barfoo", "foobar"],
        svec!["is waldo here", "spot"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn searchset_unicode() {
    let wrk = Workdir::new("searchset");
    wrk.create("data.csv", data(true));
    wrk.create("regexset_unicode.txt", regexset_unicode_file());
    let mut cmd = wrk.command("searchset");
    cmd.arg("regexset_unicode.txt").arg("data.csv");
    cmd.arg("--unicode");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["h1", "h2"],
        svec!["foobar", "barfoo"],
        svec!["barfoo", "foobar"],
        svec!["is waldo here", "spot"],
        svec!["Ḟooƀar", "ḃarḟoo"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn searchset_unicode_envvar() {
    let wrk = Workdir::new("searchset");
    wrk.create("data.csv", data(true));
    wrk.create("regexset_unicode.txt", regexset_unicode_file());
    let mut cmd = wrk.command("searchset");
    cmd.env("QSV_REGEX_UNICODE", "1");
    cmd.arg("regexset_unicode.txt").arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["h1", "h2"],
        svec!["foobar", "barfoo"],
        svec!["barfoo", "foobar"],
        svec!["is waldo here", "spot"],
        svec!["Ḟooƀar", "ḃarḟoo"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn searchset_empty() {
    let wrk = Workdir::new("searchset");
    wrk.create("data.csv", data(true));
    wrk.create("emptyregexset.txt", empty_regexset_file());
    let mut cmd = wrk.command("searchset");
    cmd.arg("emptyregexset.txt").arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["h1", "h2"]];
    assert_eq!(got, expected);
}

#[test]
fn searchset_empty_no_headers() {
    let wrk = Workdir::new("searchset");
    wrk.create("data.csv", data(true));
    wrk.create("emptyregexset.txt", empty_regexset_file());
    let mut cmd = wrk.command("searchset");
    cmd.arg("emptyregexset.txt").arg("data.csv");
    cmd.arg("--no-headers");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected: Vec<Vec<String>> = vec![];
    assert_eq!(got, expected);
}

#[test]
fn searchset_ignore_case() {
    let wrk = Workdir::new("searchset");
    wrk.create("data.csv", data(true));
    wrk.create("regexset.txt", regexset_file());
    let mut cmd = wrk.command("searchset");
    cmd.arg("regexset.txt").arg("data.csv");
    cmd.arg("--ignore-case");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["h1", "h2"],
        svec!["foobar", "barfoo"],
        svec!["barfoo", "foobar"],
        svec!["is waldo here", "spot"],
        svec!["bleh", "no, Waldo is there"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn searchset_no_headers() {
    let wrk = Workdir::new("searchset_no_headers");
    wrk.create("data.csv", data(false));
    wrk.create("regexset.txt", regexset_file());
    let mut cmd = wrk.command("searchset");
    cmd.arg("regexset.txt").arg("data.csv");
    cmd.arg("--no-headers");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["foobar", "barfoo"],
        svec!["barfoo", "foobar"],
        svec!["is waldo here", "spot"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn searchset_select() {
    let wrk = Workdir::new("searchset_select");
    wrk.create("data.csv", data(true));
    wrk.create("regexset.txt", regexset_file());
    let mut cmd = wrk.command("searchset");
    cmd.arg("regexset.txt").arg("data.csv");
    cmd.arg("--select").arg("h2");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["h1", "h2"], svec!["barfoo", "foobar"]];
    assert_eq!(got, expected);
}

#[test]
fn searchset_select_no_headers() {
    let wrk = Workdir::new("searchset_select_no_headers");
    wrk.create("data.csv", data(false));
    wrk.create("regexset.txt", regexset_file());
    let mut cmd = wrk.command("searchset");
    cmd.arg("regexset.txt").arg("data.csv");
    cmd.arg("--select").arg("2");
    cmd.arg("--no-headers");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["barfoo", "foobar"]];
    assert_eq!(got, expected);
}

#[test]
fn searchset_invert_match() {
    let wrk = Workdir::new("searchset_invert_match");
    wrk.create("data.csv", data(false));
    wrk.create("regexset.txt", regexset_file());
    let mut cmd = wrk.command("searchset");
    cmd.arg("regexset.txt").arg("data.csv");
    cmd.arg("--invert-match");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["foobar", "barfoo"],
        svec!["a", "b"],
        svec!["Ḟooƀar", "ḃarḟoo"],
        svec!["bleh", "no, Waldo is there"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn searchset_invert_match_no_headers() {
    let wrk = Workdir::new("searchset_invert_match");
    wrk.create("data.csv", data(false));
    wrk.create("regexset.txt", regexset_file());
    let mut cmd = wrk.command("searchset");
    cmd.arg("regexset.txt").arg("data.csv");
    cmd.arg("--invert-match");
    cmd.arg("--no-headers");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["a", "b"],
        svec!["Ḟooƀar", "ḃarḟoo"],
        svec!["bleh", "no, Waldo is there"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn searchset_flag() {
    let wrk = Workdir::new("searchset_flag");
    wrk.create("data.csv", data(false));
    wrk.create("regexset.txt", regexset_file());
    let mut cmd = wrk.command("searchset");
    cmd.arg("regexset.txt")
        .arg("data.csv")
        .args(&["--flag", "flagged"]);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["foobar", "barfoo", "flagged"],
        svec!["a", "b", "0"],
        svec!["barfoo", "foobar", "3;[1, 2]"],
        svec!["is waldo here", "spot", "4;[3]"],
        svec!["Ḟooƀar", "ḃarḟoo", "0"],
        svec!["bleh", "no, Waldo is there", "0"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn searchset_flag_invert_match() {
    let wrk = Workdir::new("searchset_flag");
    wrk.create("regexset.txt", regexset_file());
    wrk.create("data.csv", data(false));
    let mut cmd = wrk.command("searchset");
    cmd.arg("regexset.txt")
        .arg("data.csv")
        .args(&["--flag", "flagged"]);
    cmd.arg("--invert-match");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["foobar", "barfoo", "flagged"],
        svec!["a", "b", "2"],
        svec!["barfoo", "foobar", "0"],
        svec!["is waldo here", "spot", "0"],
        svec!["Ḟooƀar", "ḃarḟoo", "5"],
        svec!["bleh", "no, Waldo is there", "6"],
    ];
    assert_eq!(got, expected);
}
