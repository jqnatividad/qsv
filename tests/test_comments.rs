use crate::workdir::Workdir;

#[test]
fn comments() {
    let wrk = Workdir::new("comments");
    wrk.create(
        "comments.csv",
        vec![
            svec!["# test file to see how comments work", ""],
            svec!["# this is another comment before the header", ""],
            svec!["# DATA DICTIONARY", ""],
            svec!["# column1 - alphabetic; id of the column", ""],
            svec!["# column2 - numeric; just a number", ""],
            svec!["column1", "column2"],
            svec!["a", "1"],
            svec!["#b", "2"],
            svec!["c", "3"],
            svec!["#d - this row is corrupted skip", "extra col2"],
            svec!["e", "5"],
        ],
    );
    let mut cmd = wrk.command("input");
    cmd.env("QSV_COMMENT_CHAR", "#");
    cmd.arg("comments.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["column1", "column2"],
        svec!["a", "1"],
        svec!["c", "3"],
        svec!["e", "5"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn comments_long() {
    let wrk = Workdir::new("comments");
    wrk.create(
        "comments.csv",
        vec![
            svec!["# test file to see how comments work", ""],
            svec!["# this is another comment before the header", ""],
            svec!["# DATA DICTIONARY", ""],
            svec!["# column1 - alphabetic; id of the column", ""],
            svec!["# column2 - numeric; just a number", ""],
            svec!["column1", "column2"],
            svec!["a", "1"],
            svec!["#b", "2"],
            svec!["c", "3"],
            svec!["#d - this row is corrupted skip", "extra col2"],
            svec!["e", "5"],
        ],
    );
    let mut cmd = wrk.command("input");
    cmd.env("QSV_COMMENT_CHAR", "# is the comment character");
    cmd.arg("comments.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["column1", "column2"],
        svec!["a", "1"],
        svec!["c", "3"],
        svec!["e", "5"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn comments_unicode_supported() {
    let wrk = Workdir::new("comments");
    wrk.create(
        "comments.csv",
        vec![
            svec!["Ǽ test file to see how comments work", ""],
            svec!["Ǽ yet another comment", ""],
            svec!["column1", "column2"],
            svec!["a", "1"],
            svec!["Ǽb", "2"],
            svec!["c", "3"],
        ],
    );
    let mut cmd = wrk.command("input");
    cmd.env("QSV_COMMENT_CHAR", "Ǽ");
    cmd.arg("comments.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["column1", "column2"],
        svec!["a", "1"],
        svec!["c", "3"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn comments_count() {
    let wrk = Workdir::new("comments");
    wrk.create(
        "comments.csv",
        vec![
            svec!["Ǽ test file to see how comments work", ""],
            svec!["Ǽ yet another comment", ""],
            svec!["column1", "column2"],
            svec!["a", "1"],
            svec!["Ǽb", "2"],
            svec!["c", "3"],
        ],
    );
    let mut cmd = wrk.command("count");
    cmd.env("QSV_COMMENT_CHAR", "Ǽ");
    cmd.arg("comments.csv");

    let got_count: usize = wrk.stdout(&mut cmd);
    rassert_eq!(got_count, 2);
}

#[test]
fn comments_headers() {
    let wrk = Workdir::new("comments");
    wrk.create(
        "comments.csv",
        vec![
            svec!["// test file to see how comments work", ""],
            svec!["// yet another comment", ""],
            svec!["/ still a comment", ""],
            svec!["column1", "column2"],
            svec!["a", "1"],
            svec!["//b", "2"],
            svec!["c", "3"],
        ],
    );
    let mut cmd = wrk.command("headers");
    cmd.env("QSV_COMMENT_CHAR", "/");
    cmd.arg("comments.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = "\
1   column1
2   column2";
    assert_eq!(got, expected);
}

#[test]
fn envlist() {
    let wrk = Workdir::new("comments");
    let mut cmd = wrk.command("");
    cmd.env("QSV_ENVVAR", "#");
    cmd.env("MIMALLOC_ENVVAR", "1");
    cmd.arg("--envlist");

    let got_envlist: String = wrk.stdout(&mut cmd);
    assert_eq!(
        got_envlist,
        r#"MIMALLOC_ENVVAR: 1
QSV_ENVVAR: #"#
    );
    // unset it so we don't have side effects outside tests
    // as these env vars persists
    cmd.env("QSV_ENVVAR", "");
    cmd.env("MIMALLOC_ENVVAR", "");
}
