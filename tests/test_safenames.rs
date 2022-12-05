use crate::workdir::Workdir;

#[test]
fn safenames_conditional() {
    let wrk = Workdir::new("safenames");
    wrk.create(
        "in.csv",
        vec![
            svec![
                "col1",
                " This is a column with invalid chars!# and leading & trailing spaces ",
                "",
                "this is already a Postgres Safe Column",
                "1starts with 1",
                "col1",
                "col1",
                "",
                "",
            ],
            svec!["1", "b", "33", "1", "b", "33", "34", "z", "42"],
            svec!["2", "c", "34", "3", "d", "31", "3", "y", "3.14"],
        ],
    );

    let mut cmd = wrk.command("safenames");
    cmd.arg("--mode").arg("c").arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec![
            "col1",
            "this_is_a_column_with_invalid_chars___and_leading___trailing",
            // null column names are not allowed in postgres
            "_blank",
            // though this is "safe", it's generally discouraged
            // to have embedded spaces and mixed case column names
            // as you will have to use quotes to refer to these columns
            // in Postgres
            "this is already a Postgres Safe Column",
            // a column cannot start with a digit
            "_starts_with_1",
            // duplicate cols are not allowed in one table in postgres
            "col1_2",
            "col1_3",
            "_blank_2",
            "_blank_3"
        ],
        svec!["1", "b", "33", "1", "b", "33", "34", "z", "42"],
        svec!["2", "c", "34", "3", "d", "31", "3", "y", "3.14"],
    ];
    assert_eq!(got, expected);

    let changed_headers = wrk.output_stderr(&mut cmd);
    let expected_count = "7\n";
    assert_eq!(changed_headers, expected_count);
}

#[test]
fn safenames_always() {
    let wrk = Workdir::new("safenames");
    wrk.create(
        "in.csv",
        vec![
            svec![
                "col1",
                // not valid in postgres
                " This is a column with invalid chars!# and leading & trailing spaces ",
                "",
                // postgres allows for embedded spaces
                "this is already a Postgres Safe Column",
                "1starts with 1",
                "col1",
                "col1"
            ],
            svec!["1", "b", "33", "1", "b", "33", "34"],
            svec!["2", "c", "34", "3", "d", "31", "3"],
        ],
    );

    let mut cmd = wrk.command("safenames");
    cmd.arg("--mode").arg("always").arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec![
            "col1",
            "this_is_a_column_with_invalid_chars___and_leading___trailing",
            "_blank",
            // we were using Always mode, so even though the
            // original header name was already valid,
            // we replaced spaces with _ regardless
            "this_is_already_a_postgres_safe_column",
            "_starts_with_1",
            "col1_2",
            "col1_3"
        ],
        svec!["1", "b", "33", "1", "b", "33", "34"],
        svec!["2", "c", "34", "3", "d", "31", "3"],
    ];
    assert_eq!(got, expected);

    let changed_headers = wrk.output_stderr(&mut cmd);
    let expected_count = "6\n";
    assert_eq!(changed_headers, expected_count);
}

#[test]
fn safenames_verify() {
    let wrk = Workdir::new("safenames");
    wrk.create(
        "in.csv",
        vec![
            svec![
                "col1",
                " This is a column with invalid chars!# and leading & trailing spaces ",
                "",
                "this is already a Postgres Safe Column",
                "1starts with 1",
                "col1",
                "col1",
                "",
                "",
            ],
            svec!["1", "b", "33", "1", "b", "33", "34", "z", "42"],
            svec!["2", "c", "34", "3", "d", "31", "3", "y", "3.14"],
        ],
    );

    let mut cmd = wrk.command("safenames");
    cmd.arg("--mode").arg("verify").arg("in.csv");

    let changed_headers = wrk.output_stderr(&mut cmd);
    let expected_count = "7\n";
    assert_eq!(changed_headers, expected_count);

    wrk.assert_success(&mut cmd);
}

#[test]
fn safenames_verify_verbose() {
    let wrk = Workdir::new("safenames");
    wrk.create(
        "in.csv",
        vec![
            svec![
                "col1",
                " This is a column with invalid chars!# and leading & trailing spaces ",
                "",
                "this is already a Postgres Safe Column",
                "1starts with 1",
                "col1",
                "col1",
                "col1",
                "",
                "",
                "",
                "col1",
                "_1",
            ],
            svec!["1", "b", "33", "1", "b", "33", "34", "z", "42", "3", "2", "1", "0"],
        ],
    );

    let mut cmd = wrk.command("safenames");
    cmd.arg("--mode").arg("V").arg("in.csv");

    let changed_headers = wrk.output_stderr(&mut cmd);
    let expected_count = r#"13 header/s
9 unsafe header/s: ["col1", " This is a column with invalid chars!# and leading & trailing spaces ", "", "this is already a Postgres Safe Column", "1starts with 1", "", "", "", "_1"]
2 safe header/s: ["col1", "this is already a Postgres Safe Column"]
7 duplicate/s
"#;
    assert_eq!(changed_headers, expected_count);

    wrk.assert_success(&mut cmd);
}

#[test]
fn safenames_invalid_mode() {
    let wrk = Workdir::new("safenames");
    wrk.create(
        "in.csv",
        vec![
            svec![
                "col1",
                " This is a column with invalid chars!# and leading & trailing spaces ",
                "",
                "this is already a postgres safe column",
                "1starts with 1",
                "col1",
                "col1"
            ],
            svec!["1", "b", "33", "1", "b", "33", "34"],
            svec!["2", "c", "34", "3", "d", "31", "3"],
        ],
    );

    let mut cmd = wrk.command("safenames");
    cmd.arg("--mode").arg("invalidmode").arg("in.csv");

    wrk.assert_err(&mut cmd);
}
