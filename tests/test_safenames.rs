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
            "unsafe_",
            // though this is "safe", it's generally discouraged
            // to have embedded spaces and mixed case column names
            // as you will have to use quotes to refer to these columns
            // in Postgres
            "this is already a Postgres Safe Column",
            // a column cannot start with a digit
            "unsafe_1starts_with_1",
            // duplicate cols are not allowed in one table in postgres
            "col1_2",
            "col1_3",
            "unsafe__2",
            "unsafe__3"
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
            "unsafe_",
            // we were using Always mode, so even though the
            // original header name was already valid,
            // we replaced spaces with _ regardless
            "this_is_already_a_postgres_safe_column",
            "unsafe_1starts_with_1",
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
    let expected_count = "6\n";
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

    let got_stderr = wrk.output_stderr(&mut cmd);

    // the order of the duplicate headers is not guaranteed as we use a HashMap
    // so we need to check for both possible orders
    let expected_stderr = r#"13 header/s
2 duplicate/s: "col1:5, :4"
8 unsafe header/s: [" This is a column with invalid chars!# and leading & trailing spaces ", "", "this is already a Postgres Safe Column", "1starts with 1", "", "", "", "_1"]
1 safe header/s: ["col1"]
"#;
    let expected_stderr2 = r#"13 header/s
2 duplicate/s: ":4, col1:5"
8 unsafe header/s: [" This is a column with invalid chars!# and leading & trailing spaces ", "", "this is already a Postgres Safe Column", "1starts with 1", "", "", "", "_1"]
1 safe header/s: ["col1"]
"#;

    assert!(got_stderr == expected_stderr || got_stderr == expected_stderr2);

    wrk.assert_success(&mut cmd);
}

#[test]
fn safenames_verify_verbose_pretty_json() {
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
    cmd.arg("--mode").arg("J").arg("in.csv");

    let got: String = wrk.stdout(&mut cmd);

    // the order of the duplicate headers is not guaranteed as we use a HashMap
    // so we need to check for both possible orders
    let expected = r#"{
  "header_count": 13,
  "duplicate_count": 2,
  "duplicate_headers": [
    ":4",
    "col1:5"
  ],
  "unsafe_headers": [
    " This is a column with invalid chars!# and leading & trailing spaces ",
    "",
    "this is already a Postgres Safe Column",
    "1starts with 1",
    "",
    "",
    "",
    "_1"
  ],
  "safe_headers": [
    "col1"
  ]
}"#;

    let expected2 = r#"{
  "header_count": 13,
  "duplicate_count": 2,
  "duplicate_headers": [
    "col1:5",
    ":4"
  ],
  "unsafe_headers": [
    " This is a column with invalid chars!# and leading & trailing spaces ",
    "",
    "this is already a Postgres Safe Column",
    "1starts with 1",
    "",
    "",
    "",
    "_1"
  ],
  "safe_headers": [
    "col1"
  ]
}"#;

    // assert_eq!(got, expected);
    assert!(got == expected || got == expected2);

    wrk.assert_success(&mut cmd);
}

#[test]
fn safenames_verify_verbose_json() {
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
    cmd.arg("--mode").arg("j").arg("in.csv");

    let got: String = wrk.stdout(&mut cmd);

    // the order of the duplicate headers is not guaranteed as we use a HashMap
    // so we need to check for both possible orders
    let expected = r#"{"header_count":13,"duplicate_count":2,"duplicate_headers":["col1:5",":4"],"unsafe_headers":[" This is a column with invalid chars!# and leading & trailing spaces ","","this is already a Postgres Safe Column","1starts with 1","","","","_1"],"safe_headers":["col1"]}"#;

    let expected2 = r#"{"header_count":13,"duplicate_count":2,"duplicate_headers":[":4","col1:5"],"unsafe_headers":[" This is a column with invalid chars!# and leading & trailing spaces ","","this is already a Postgres Safe Column","1starts with 1","","","","_1"],"safe_headers":["col1"]}"#;

    assert!(got == expected || got == expected2);

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

#[test]
fn safenames_reserved_names_default() {
    let wrk = Workdir::new("safenames");
    wrk.create(
        "in.csv",
        vec![
            svec![
                "col1",
                " This is a column with invalid chars!# and leading & trailing spaces ",
                "_id",
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
    cmd.arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec![
            "col1",
            "this_is_a_column_with_invalid_chars___and_leading___trailing",
            "reserved__id",
            "this_is_already_a_postgres_safe_column",
            "unsafe_1starts_with_1",
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
fn safenames_reserved_names_specified() {
    let wrk = Workdir::new("safenames");
    wrk.create(
        "in.csv",
        vec![
            svec![
                "col1",
                " This is a column with invalid chars!# and leading & trailing spaces ",
                "waldo",
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
    cmd.arg("--reserved").arg("waldo").arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec![
            "col1",
            "this_is_a_column_with_invalid_chars___and_leading___trailing",
            "reserved_waldo",
            "this_is_already_a_postgres_safe_column",
            "unsafe_1starts_with_1",
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
fn safenames_reserved_names_specified_case_insensitive() {
    let wrk = Workdir::new("safenames");
    wrk.create(
        "in.csv",
        vec![
            svec![
                "col1",
                " This is a column with invalid chars!# and leading & trailing spaces ",
                "WaLdO",
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
    cmd.arg("--reserved").arg("waldo").arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec![
            "col1",
            "this_is_a_column_with_invalid_chars___and_leading___trailing",
            "reserved_waldo",
            "this_is_already_a_postgres_safe_column",
            "unsafe_1starts_with_1",
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
