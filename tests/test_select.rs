use crate::workdir::Workdir;

macro_rules! select_test {
    ($name:ident, $select:expr, $select_no_headers:expr,
     $expected_headers:expr, $expected_rows:expr) => {
        mod $name {
            use super::data;
            use crate::workdir::Workdir;

            #[test]
            fn headers() {
                let wrk = Workdir::new(stringify!($name));
                wrk.create("data.csv", data(true));
                let mut cmd = wrk.command("select");
                cmd.arg("--").arg($select).arg("data.csv");
                let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

                let expected = vec![
                    $expected_headers
                        .iter()
                        .map(|s| s.to_string())
                        .collect::<Vec<String>>(),
                    $expected_rows
                        .iter()
                        .map(|s| s.to_string())
                        .collect::<Vec<String>>(),
                ];
                assert_eq!(got, expected);
            }

            #[test]
            fn no_headers() {
                let wrk = Workdir::new(stringify!($name));
                wrk.create("data.csv", data(false));
                let mut cmd = wrk.command("select");
                cmd.arg("--no-headers")
                    .arg("--")
                    .arg($select_no_headers)
                    .arg("data.csv");
                let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

                let expected = vec![$expected_rows
                    .iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()];
                assert_eq!(got, expected);
            }
        }
    };
}

macro_rules! select_test_err {
    ($name:ident, $select:expr) => {
        #[test]
        fn $name() {
            let wrk = Workdir::new(stringify!($name));
            wrk.create("data.csv", data(true));
            let mut cmd = wrk.command("select");
            cmd.arg($select).arg("data.csv");
            wrk.assert_err(&mut cmd);
        }
    };
}

fn header_row() -> Vec<String> {
    svec!["h1", "h2", "h[]3", "h4", "h1"]
}

fn data(headers: bool) -> Vec<Vec<String>> {
    let mut rows = vec![svec!["a", "b", "c", "d", "e"]];
    if headers {
        rows.insert(0, header_row())
    }
    rows
}

select_test!(select_simple, "h1", "1", ["h1"], ["a"]);
select_test!(select_simple_idx, "h1[0]", "1", ["h1"], ["a"]);
select_test!(select_simple_idx_2, "h1[1]", "5", ["h1"], ["e"]);

select_test!(select_quoted, r#""h[]3""#, "3", ["h[]3"], ["c"]);
select_test!(select_quoted_idx, r#""h[]3"[0]"#, "3", ["h[]3"], ["c"]);

select_test!(
    select_range,
    "h1-h4",
    "1-4",
    ["h1", "h2", "h[]3", "h4"],
    ["a", "b", "c", "d"]
);

select_test!(
    select_range_multi,
    r#"h1-h2,"h[]3"-h4"#,
    "1-2,3-4",
    ["h1", "h2", "h[]3", "h4"],
    ["a", "b", "c", "d"]
);
select_test!(
    select_range_multi_idx,
    r#"h1-h2,"h[]3"[0]-h4"#,
    "1-2,3-4",
    ["h1", "h2", "h[]3", "h4"],
    ["a", "b", "c", "d"]
);

select_test!(
    select_reverse,
    "h1[1]-h1[0]",
    "5-1",
    ["h1", "h4", "h[]3", "h2", "h1"],
    ["e", "d", "c", "b", "a"]
);

select_test!(
    select_not,
    r#"!"h[]3"[0]"#,
    "!3",
    ["h1", "h2", "h4", "h1"],
    ["a", "b", "d", "e"]
);
select_test!(select_not_range, "!h1[1]-h2", "!5-2", ["h1"], ["a"]);

select_test!(select_duplicate, "h1,h1", "1,1", ["h1", "h1"], ["a", "a"]);
select_test!(
    select_duplicate_range,
    "h1-h2,h1-h2",
    "1-2,1-2",
    ["h1", "h2", "h1", "h2"],
    ["a", "b", "a", "b"]
);
select_test!(
    select_duplicate_range_reverse,
    "h1-h2,h2-h1",
    "1-2,2-1",
    ["h1", "h2", "h2", "h1"],
    ["a", "b", "b", "a"]
);

select_test!(select_range_no_end, "h4-", "4-", ["h4", "h1"], ["d", "e"]);
select_test!(select_range_no_start, "-h2", "-2", ["h1", "h2"], ["a", "b"]);
select_test!(
    select_range_no_end_cat,
    "h4-,h1",
    "4-,1",
    ["h4", "h1", "h1"],
    ["d", "e", "a"]
);
select_test!(
    select_range_no_start_cat,
    "-h2,h1[1]",
    "-2,5",
    ["h1", "h2", "h1"],
    ["a", "b", "e"]
);

select_test!(
    select_regex,
    "/h[1-3]/",
    "1,2,5",
    ["h1", "h2", "h1"],
    ["a", "b", "e"]
);

select_test!(
    select_not_regex,
    "!/h1|h2/",
    "3,4",
    ["h[]3", "h4"],
    ["c", "d"]
);

select_test!(
    select_regex_digit,
    r#"/h\d/"#,
    "1,2,4,5",
    ["h1", "h2", "h4", "h1"],
    ["a", "b", "d", "e"]
);

select_test!(
    select_reverse_sentinel,
    r#"9999-1"#,
    "5-1",
    ["h1", "h4", "h[]3", "h2", "h1"],
    ["e", "d", "c", "b", "a"]
);

select_test_err!(select_err_unknown_header, "done");
select_test_err!(select_err_oob_low, "0");
select_test_err!(select_err_oob_high, "6");
select_test_err!(select_err_idx_as_name, "1[0]");
select_test_err!(select_err_idx_oob_high, "h1[2]");
select_test_err!(select_err_idx_not_int, "h1[2.0]");
select_test_err!(select_err_idx_not_int_2, "h1[a]");
select_test_err!(select_err_unclosed_quote, r#""h1"#);
select_test_err!(select_err_unclosed_bracket, r#""h1"[1"#);
select_test_err!(select_err_expected_end_of_field, "a-b-");
select_test_err!(select_err_single_slash, "/");
select_test_err!(select_err_regex_nomatch, "/nomatch/");
select_test_err!(select_err_regex_invalid, "/?/");
select_test_err!(select_err_regex_empty, "//");
select_test_err!(select_err_regex_triple_slash, "///");

fn unsorted_data(headers: bool) -> Vec<Vec<String>> {
    let mut rows = vec![
        svec![
            "value1", "value2", "value3", "value4", "value5", "value6", "value7", "value8",
            "value9", "value10"
        ],
        svec!["1", "2", "3", "4", "5", "6", "7", "8", "9", "10"],
        svec![
            "value10", "value9", "value8", "value7", "value6", "value5", "value4", "value3",
            "value2", "value1"
        ],
    ];
    if headers {
        rows.insert(
            0,
            svec![
                "Günther", "Alice", "Çemil", "Đan", "Fátima", "Héctor", "İbrahim", "Bob", "Jürgen",
                "Élise"
            ],
        );
    }
    rows
}

#[test]
fn test_select_sort() {
    let wrk = Workdir::new("test_select_sort");
    wrk.create("data.csv", unsorted_data(true));
    let mut cmd = wrk.command("select");
    cmd.arg("1").arg("--sort").arg("data.csv");
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    let expected = vec![
        svec![
            "Alice", "Bob", "Fátima", "Günther", "Héctor", "Jürgen", "Çemil", "Élise", "Đan",
            "İbrahim"
        ],
        svec![
            "value2", "value8", "value5", "value1", "value6", "value9", "value3", "value10",
            "value4", "value7"
        ],
        svec!["2", "8", "5", "1", "6", "9", "3", "10", "4", "7"],
        svec![
            "value9", "value3", "value6", "value10", "value5", "value2", "value8", "value1",
            "value7", "value4"
        ],
    ];
    assert_eq!(got, expected);
}

#[test]
fn test_select_random_seeded() {
    let wrk = Workdir::new("test_select_random_seeded");
    wrk.create("data.csv", unsorted_data(true));
    let mut cmd = wrk.command("select");
    cmd.arg("1")
        .arg("--random")
        .args(["--seed", "42"])
        .arg("data.csv");
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    let expected = vec![
        svec![
            "Bob", "Đan", "Élise", "Héctor", "Günther", "Jürgen", "İbrahim", "Fátima", "Çemil",
            "Alice"
        ],
        svec![
            "value8", "value4", "value10", "value6", "value1", "value9", "value7", "value5",
            "value3", "value2"
        ],
        svec!["8", "4", "10", "6", "1", "9", "7", "5", "3", "2"],
        svec![
            "value3", "value7", "value1", "value5", "value10", "value2", "value4", "value6",
            "value8", "value9"
        ],
    ];
    assert_eq!(got, expected);
}
