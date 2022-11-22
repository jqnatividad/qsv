use crate::workdir::Workdir;

#[test]
fn safenames() {
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
                "col1",
                "",
                "",
            ],
            svec!["1", "b", "33", "1", "b", "33", "34", "z", "42"],
            svec!["2", "c", "34", "3", "d", "31", "3", "y", "3.14"],
        ],
    );

    let mut cmd = wrk.command("safenames");
    cmd.arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec![
            "col1",
            "This_is_a_column_with_invalid_chars___and_leading___trailing",
            // null column names are not allowed in postgres
            "_",
            "this is already a postgres safe column",
            // a column cannot start with a digit
            "_starts_with_1",
            // duplicate cols are not allowed in one table in postgres
            "col1_2",
            "col1_3",
            "__2",
            "__3"
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
    cmd.arg("--mode").arg("always").arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec![
            "col1",
            "This_is_a_column_with_invalid_chars___and_leading___trailing",
            "_",
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
fn safenames_ignore_invalid_mode() {
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

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    // if the mode is invalid, its a noop and we do not modify the header names
    let expected = vec![
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
    ];
    assert_eq!(got, expected);

    wrk.assert_err(&mut cmd);
}
