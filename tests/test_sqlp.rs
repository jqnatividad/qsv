use crate::workdir::Workdir;

macro_rules! sqlp_test {
    ($name:ident, $fun:expr) => {
        mod $name {
            use std::process;

            use super::{make_rows, setup};
            use crate::workdir::Workdir;

            #[test]
            fn headers() {
                let wrk = setup(stringify!($name));
                let mut cmd = wrk.command("sqlp");
                cmd.args(&["cities.csv", "places.csv"]);
                $fun(wrk, cmd);
            }
        }
    };
}

fn setup(name: &str) -> Workdir {
    let cities = vec![
        svec!["city", "state"],
        svec!["Boston", "MA"],
        svec!["New York", "NY"],
        svec!["San Francisco", "CA"],
        svec!["Buffalo", "NY"],
    ];
    let places = vec![
        svec!["city", "place"],
        svec!["Boston", "Logan Airport"],
        svec!["Boston", "Boston Garden"],
        svec!["Buffalo", "Ralph Wilson Stadium"],
        svec!["Orlando", "Disney World"],
    ];

    let wrk = Workdir::new(name);
    wrk.create("cities.csv", cities);
    wrk.create("places.csv", places);
    wrk
}

fn make_rows(left_only: bool, rows: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let mut all_rows = vec![];
    if left_only {
        all_rows.push(svec!["city", "state"]);
    } else {
        all_rows.push(svec!["city", "state", "place"]);
    }
    all_rows.extend(rows.into_iter());
    all_rows
}

sqlp_test!(
    sqlp_join_inner,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.arg("select * from cities inner join places on cities.city = places.city");
        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let expected = make_rows(
            false,
            vec![
                svec!["Boston", "MA", "Logan Airport"],
                svec!["Boston", "MA", "Boston Garden"],
                svec!["Buffalo", "NY", "Ralph Wilson Stadium"],
            ],
        );
        assert_eq!(got, expected);
    }
);

sqlp_test!(
    sqlp_join_outer_left,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.arg("select * from cities left outer join places on cities.city = places.city");
        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let expected = make_rows(
            false,
            vec![
                svec!["Boston", "MA", "Logan Airport"],
                svec!["Boston", "MA", "Boston Garden"],
                svec!["New York", "NY", ""],
                svec!["San Francisco", "CA", ""],
                svec!["Buffalo", "NY", "Ralph Wilson Stadium"],
            ],
        );
        assert_eq!(got, expected);
    }
);

sqlp_test!(
    sqlp_join_outer_full,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.arg("select * from cities full outer join places on cities.city = places.city");
        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let expected1 = make_rows(
            false,
            vec![
                svec!["Boston", "MA", "Logan Airport"],
                svec!["Boston", "MA", "Boston Garden"],
                svec!["Buffalo", "NY", "Ralph Wilson Stadium"],
                svec!["Orlando", "", "Disney World"],
                svec!["San Francisco", "CA", ""],
                svec!["New York", "NY", ""],
            ],
        );
        let expected2 = make_rows(
            false,
            vec![
                svec!["Boston", "MA", "Logan Airport"],
                svec!["Boston", "MA", "Boston Garden"],
                svec!["Buffalo", "NY", "Ralph Wilson Stadium"],
                svec!["Orlando", "", "Disney World"],
                svec!["New York", "NY", ""],
                svec!["San Francisco", "CA", ""],
            ],
        );
        assert!(got == expected1 || got == expected2);
    }
);

#[test]
fn sqlp_join_cross() {
    let wrk = Workdir::new("join_cross");
    wrk.create(
        "letters.csv",
        vec![svec!["h1", "h2"], svec!["a", "b"], svec!["c", "d"]],
    );
    wrk.create(
        "numbers.csv",
        vec![svec!["h3", "h4"], svec!["1", "2"], svec!["3", "4"]],
    );

    let mut cmd = wrk.command("sqlp");
    cmd.args(["letters.csv", "numbers.csv"])
        .arg("select * from letters cross join numbers");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["h1", "h2", "h3", "h4"],
        svec!["a", "b", "1", "2"],
        svec!["a", "b", "3", "4"],
        svec!["c", "d", "1", "2"],
        svec!["c", "d", "3", "4"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sqlp_boston311_groupby_orderby() {
    let wrk = Workdir::new("sqlp_boston311_groupby_orderby");
    let test_file = wrk.load_test_file("boston311-100.csv");

    let mut cmd = wrk.command("sqlp");

    // we quote "boston311-100" as contains a hyphen in its name, which is a special character
    // in SQL, so we need to make it a quoted identifier
    cmd.arg(&test_file)
        .arg(r#"select ward, count(*) as cnt from "boston311-100" group by ward order by cnt desc, ward asc"#);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["ward", "cnt"],
        svec!["Ward 3", "10"],
        svec!["Ward 6", "7"],
        svec!["Ward 1", "6"],
        svec!["3", "5"],
        svec!["Ward 20", "5"],
        svec!["Ward 4", "5"],
        svec!["Ward 5", "5"],
        svec!["14", "4"],
        svec!["Ward 13", "4"],
        svec!["Ward 16", "4"],
        svec!["Ward 18", "3"],
        svec!["Ward 19", "3"],
        svec!["Ward 7", "3"],
        svec!["Ward 8", "3"],
        svec!["03", "2"],
        svec!["17", "2"],
        svec!["22", "2"],
        svec!["Ward 11", "2"],
        svec!["Ward 21", "2"],
        svec![" ", "1"],
        svec!["01", "1"],
        svec!["02", "1"],
        svec!["04", "1"],
        svec!["06", "1"],
        svec!["07", "1"],
        svec!["1", "1"],
        svec!["10", "1"],
        svec!["16", "1"],
        svec!["18", "1"],
        svec!["19", "1"],
        svec!["21", "1"],
        svec!["7", "1"],
        svec!["8", "1"],
        svec!["9", "1"],
        svec!["Ward 10", "1"],
        svec!["Ward 12", "1"],
        svec!["Ward 14", "1"],
        svec!["Ward 15", "1"],
        svec!["Ward 17", "1"],
        svec!["Ward 2", "1"],
        svec!["Ward 22", "1"],
        svec!["Ward 9", "1"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sqlp_boston311_groupby_orderby_with_table_alias() {
    let wrk = Workdir::new("sqlp_boston311_groupby_orderby");
    let test_file = wrk.load_test_file("boston311-100.csv");

    let mut cmd = wrk.command("sqlp");

    // we use _t_1 alias as "boston311-100" contains a hyphen in its name, which is a special
    // character in SQL
    cmd.arg(&test_file)
        .arg("select ward, count(*) as cnt from _t_1 group by ward order by cnt desc, ward asc");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["ward", "cnt"],
        svec!["Ward 3", "10"],
        svec!["Ward 6", "7"],
        svec!["Ward 1", "6"],
        svec!["3", "5"],
        svec!["Ward 20", "5"],
        svec!["Ward 4", "5"],
        svec!["Ward 5", "5"],
        svec!["14", "4"],
        svec!["Ward 13", "4"],
        svec!["Ward 16", "4"],
        svec!["Ward 18", "3"],
        svec!["Ward 19", "3"],
        svec!["Ward 7", "3"],
        svec!["Ward 8", "3"],
        svec!["03", "2"],
        svec!["17", "2"],
        svec!["22", "2"],
        svec!["Ward 11", "2"],
        svec!["Ward 21", "2"],
        svec![" ", "1"],
        svec!["01", "1"],
        svec!["02", "1"],
        svec!["04", "1"],
        svec!["06", "1"],
        svec!["07", "1"],
        svec!["1", "1"],
        svec!["10", "1"],
        svec!["16", "1"],
        svec!["18", "1"],
        svec!["19", "1"],
        svec!["21", "1"],
        svec!["7", "1"],
        svec!["8", "1"],
        svec!["9", "1"],
        svec!["Ward 10", "1"],
        svec!["Ward 12", "1"],
        svec!["Ward 14", "1"],
        svec!["Ward 15", "1"],
        svec!["Ward 17", "1"],
        svec!["Ward 2", "1"],
        svec!["Ward 22", "1"],
        svec!["Ward 9", "1"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sqlp_boston311_wnull_value() {
    let wrk = Workdir::new("sqlp_boston311_wnull_value");
    let test_file = wrk.load_test_file("boston311-100.csv");

    let mut cmd = wrk.command("sqlp");

    cmd.arg(&test_file)
        .args(["--wnull-value", "Not Specified"])
        .arg(
            "select location_street_name, location_zipcode from _t_1 where location_zipcode is \
             null order by location_street_name limit 5",
        );

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["location_street_name", "location_zipcode"],
        svec!["Not Specified", "Not Specified"],
        svec!["INTERSECTION Asticou Rd & Washington St", "Not Specified"],
        svec![
            "INTERSECTION Charles River Plz & Cambridge St",
            "Not Specified"
        ],
        svec!["INTERSECTION Columbia Rd & E Cottage St", "Not Specified"],
        svec!["INTERSECTION E Canton St & Albany St", "Not Specified"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sqlp_null_aware_equality_checks() {
    let wrk = Workdir::new("sqlp_null_aware_equality_checks");
    wrk.create(
        "test_null.csv",
        vec![
            svec!["a", "b"],
            svec!["1", "1"],
            svec!["", ""],
            svec!["3", "3"],
            svec!["6", "4"],
            svec!["5", ""],
        ],
    );

    let mut cmd = wrk.command("sqlp");

    cmd.arg("test_null.csv")
        .args(["--wnull-value", "NULL"])
        .arg(
            r#"SELECT (a = b) as "1_eq_unaware",
           (a != b) as "2_neq_unaware", 
           (a <=> b) as "3_eq_aware", 
           (a IS NOT DISTINCT FROM b) as "4_eq_aware", 
           (a IS DISTINCT FROM b) as "5_neq_aware" 
         FROM test_null"#,
        );

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec![
            "1_eq_unaware",
            "2_neq_unaware",
            "3_eq_aware",
            "4_eq_aware",
            "5_neq_aware"
        ],
        svec!["true", "false", "true", "true", "false"],
        svec!["NULL", "NULL", "true", "true", "false"],
        svec!["true", "false", "true", "true", "false"],
        svec!["false", "true", "false", "false", "true"],
        svec!["NULL", "NULL", "false", "false", "true"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sqlp_rnull_values() {
    let wrk = Workdir::new("sqlp_rnull_values");
    wrk.create(
        "test_null.csv",
        vec![
            svec!["a", "b"],
            svec!["1", "NULL"],
            svec!["2", "NA"],
            svec!["3", "Dunno"],
            svec!["4", "4"],
            svec!["5", ""],
            svec!("6", "6"),
        ],
    );

    let mut cmd = wrk.command("sqlp");

    cmd.arg("test_null.csv")
        .args(["--rnull-values", "NULL,NA,Dunno"])
        .arg("SELECT * FROM test_null");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["a", "b"],
        svec!["1", ""],
        svec!["2", ""],
        svec!["3", ""],
        svec!["4", "4"],
        svec!["5", ""],
        svec!["6", "6"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sqlp_regex_operators() {
    let wrk = Workdir::new("sqlp_regex_operators");
    wrk.create(
        "test_regex.csv",
        vec![
            svec!["n", "sval"],
            svec!["1", "ABC"],
            svec!["2", "abc"],
            svec!["3", "000"],
            svec!["4", "A0C"],
            svec!["5", "a0c"],
        ],
    );

    // ~ operator - contains pattern (case-sensitive)
    let mut cmd = wrk.command("sqlp");
    cmd.arg("test_regex.csv")
        .arg(r#"SELECT * FROM test_regex WHERE sval ~ '\d'"#);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["n", "sval"],
        svec!["3", "000"],
        svec!["4", "A0C"],
        svec!["5", "a0c"],
    ];
    assert_eq!(got, expected);

    // ~* operator - contains pattern (case-insensitive)
    let mut cmd = wrk.command("sqlp");
    cmd.arg("test_regex.csv")
        .arg(r#"SELECT * FROM test_regex WHERE sval ~* '^a0'"#);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["n", "sval"], svec!["4", "A0C"], svec!["5", "a0c"]];
    assert_eq!(got, expected);

    // !~ operator - does not contain pattern (case-sensitive)
    let mut cmd = wrk.command("sqlp");
    cmd.arg("test_regex.csv")
        .arg(r#"SELECT * FROM test_regex WHERE sval !~ '^a0'"#);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["n", "sval"],
        svec!["1", "ABC"],
        svec!["2", "abc"],
        svec!["3", "000"],
        svec!["4", "A0C"],
    ];
    assert_eq!(got, expected);

    // !~* operator - does not contain pattern (case-insensitive)
    let mut cmd = wrk.command("sqlp");
    cmd.arg("test_regex.csv")
        .arg(r#"SELECT * FROM test_regex WHERE sval !~* '^a0'"#);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["n", "sval"],
        svec!["1", "ABC"],
        svec!["2", "abc"],
        svec!["3", "000"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sqlp_regexp_like() {
    let wrk = Workdir::new("sqlp_regexp_like");
    wrk.create(
        "test_regexp_like.csv",
        vec![
            svec!["scol"],
            svec!["abcde"],
            svec!["abc"],
            svec!["a"],
            svec![""],
        ],
    );

    let mut cmd = wrk.command("sqlp");
    cmd.arg("test_regexp_like.csv")
        .arg(r#"SELECT scol FROM test_regexp_like where REGEXP_LIKE(scol,'(C|D)','i')"#);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["scol"], svec!["abcde"], svec!["abc"]];
    assert_eq!(got, expected);
}

#[test]
fn sqlp_string_functions() {
    let wrk = Workdir::new("sqlp_string_functions");
    wrk.create(
        "test_strings.csv",
        vec![
            svec!["scol"],
            svec!["abcdE"],
            svec!["abc"],
            svec!["    abc"],
            svec!["a"],
            svec!["b"],
        ],
    );

    // starts_with
    let mut cmd = wrk.command("sqlp");
    cmd.arg("test_strings.csv")
        .arg(r#"SELECT starts_with(scol, 'a') from test_strings"#);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["scol"],
        svec!["true"],
        svec!["true"],
        svec!["true"],
        svec!["true"],
        svec!["false"],
    ];
    assert_eq!(got, expected);

    // ends_with
    let mut cmd = wrk.command("sqlp");
    cmd.arg("test_strings.csv")
        .arg(r#"SELECT ends_with(scol, 'c') from test_strings"#);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["scol"],
        svec!["false"],
        svec!["true"],
        svec!["true"],
        svec!["false"],
        svec!["false"],
    ];
    assert_eq!(got, expected);

    // left
    let mut cmd = wrk.command("sqlp");
    cmd.arg("test_strings.csv")
        .arg(r#"SELECT left(scol, 3) from test_strings"#);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["scol"],
        svec!["abc"],
        svec!["abc"],
        svec!["abc"],
        svec!["a"],
        svec!["b"],
    ];
    assert_eq!(got, expected);

    // substr
    let mut cmd = wrk.command("sqlp");
    cmd.arg("test_strings.csv")
        .arg(r#"SELECT substr(scol, 2, 2) from test_strings"#);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["scol"],
        svec!["cd"],
        svec!["c"],
        svec!["c"],
        svec![""],
        svec![""],
    ];
    assert_eq!(got, expected);

    // upper
    let mut cmd = wrk.command("sqlp");
    cmd.arg("test_strings.csv")
        .arg(r#"SELECT upper(scol) from test_strings"#);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["scol"],
        svec!["ABCDE"],
        svec!["ABC"],
        svec!["ABC"],
        svec!["A"],
        svec!["B"],
    ];
    assert_eq!(got, expected);

    // lower
    let mut cmd = wrk.command("sqlp");
    cmd.arg("test_strings.csv")
        .arg(r#"SELECT lower(scol) from test_strings"#);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["scol"],
        svec!["abcde"],
        svec!["abc"],
        svec!["abc"],
        svec!["a"],
        svec!["b"],
    ];
    assert_eq!(got, expected);

    // length
    let mut cmd = wrk.command("sqlp");
    cmd.arg("test_strings.csv")
        .arg(r#"SELECT length(scol) from test_strings"#);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["scol"],
        svec!["5"],
        svec!["3"],
        svec!["3"],
        svec!["1"],
        svec!["1"],
    ];
    assert_eq!(got, expected);

    // octet_length
    let mut cmd = wrk.command("sqlp");
    cmd.arg("test_strings.csv")
        .arg(r#"SELECT octet_length(scol) from test_strings"#);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["scol"],
        svec!["5"],
        svec!["3"],
        svec!["3"],
        svec!["1"],
        svec!["1"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sqlp_boston311_try_parsedates() {
    let wrk = Workdir::new("sqlp_boston311_try_parsedates");
    let test_file = wrk.load_test_file("boston311-100.csv");

    let mut cmd = wrk.command("sqlp");
    cmd.arg(&test_file)
        .arg("--try-parsedates")
        .arg(
            "select ward, cast(avg(closed_dt - open_dt) as float) as avg_tat from _t_1 where \
             case_status = 'Closed' group by ward order by avg_tat desc, ward asc",
        )
        .arg("--ignore-errors");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["ward", "avg_tat"],
        svec!["Ward 11", "4847760000000.0"],
        svec!["01", "4818270000000.0"],
        svec!["Ward 13", "1518365700000.0"],
        svec!["Ward 15", "1278926000000.0"],
        svec!["Ward 21", "878446000000.0"],
        svec!["Ward 14", "618933000000.0"],
        svec!["Ward 3", "437691450000.0"],
        svec!["Ward 5", "411909500000.0"],
        svec!["Ward 20", "367233000000.0"],
        svec!["9", "353495000000.0"],
        svec!["Ward 18", "249882000000.0"],
        svec!["19", "212566000000.0"],
        svec!["Ward 4", "112872600000.0"],
        svec!["Ward 1", "107850670000.0"],
        svec!["Ward 10", "104110000000.0"],
        svec!["16", "93557000000.0"],
        svec!["Ward 19", "84164000000.0"],
        svec!["10", "79101000000.0"],
        svec!["21", "77717000000.0"],
        svec!["7", "74611000000.0"],
        svec!["17", "70117500000.0"],
        svec!["3", "68836600000.0"],
        svec!["Ward 9", "64097000000.0"],
        svec!["Ward 12", "62930000000.0"],
        svec!["Ward 6", "54770168000.0"],
        svec!["Ward 7", "38346334000.0"],
        svec!["Ward 8", "32767500000.0"],
        svec!["03", "29810500000.0"],
        svec!["07", "25328000000.0"],
        svec!["22", "23919000000.0"],
        svec!["14", "20786500000.0"],
        svec!["Ward 22", "13524000000.0"],
        svec!["1", "9469000000.0"],
        svec!["06", "5290000000.0"],
        svec!["Ward 16", "4533667000.0"],
        svec!["8", "1757000000.0"],
        svec!["02", "1650000000.0"],
        svec!["18", "507000000.0"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sqlp_boston311_try_parsedates_precision() {
    let wrk = Workdir::new("sqlp_boston311_try_parsedates_precision");
    let test_file = wrk.load_test_file("boston311-100.csv");

    let mut cmd = wrk.command("sqlp");
    cmd.arg(&test_file)
        .arg("--try-parsedates")
        .args(["--float-precision", "3"])
        .arg(
            "select ward, cast(avg(closed_dt - open_dt) as float) as avg_tat from _t_1 where \
             case_status = 'Closed' group by ward order by avg_tat desc, ward asc limit 5",
        )
        .arg("--ignore-errors");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["ward", "avg_tat"],
        svec!["Ward 11", "4847759785984.000"],
        svec!["01", "4818270158848.000"],
        svec!["Ward 13", "1518365704192.000"],
        svec!["Ward 15", "1278925996032.000"],
        svec!["Ward 21", "878445985792.000"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sqlp_boston311_try_parsedates_format() {
    let wrk = Workdir::new("sqlp_boston311_try_parsedates_format");
    let test_file = wrk.load_test_file("boston311-100.csv");

    let mut cmd = wrk.command("sqlp");
    cmd.arg(&test_file)
        .arg("--try-parsedates")
        .args(["--datetime-format", "%a %Y-%m-%d %H:%M:%S"])
        .arg("select closed_dt, open_dt from _t_1 where case_status = 'Closed' limit 5")
        .arg("--ignore-errors");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["closed_dt", "open_dt"],
        svec!["Wed 2022-01-19 11:42:16", "Wed 2022-01-19 11:18:00"],
        svec!["Sun 2022-01-09 06:43:06", "Sat 2022-01-08 12:54:49"],
        svec!["Mon 2022-01-10 08:42:23", "Sat 2022-01-01 00:16:00"],
        svec!["Thu 2022-01-20 08:45:03", "Thu 2022-01-20 08:07:49"],
        svec!["Thu 2022-01-20 08:45:12", "Thu 2022-01-20 08:15:45"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sqlp_comments() {
    let wrk = Workdir::new("sqlp_comments");
    // let test_file = wrk.load_test_file("inputcommenttest.csv");
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

    let mut cmd = wrk.command("sqlp");
    cmd.env("QSV_COMMENT_CHAR", "#");
    cmd.arg("comments.csv")
        .arg("select column1, column2 from comments order by column2 desc");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["column1", "column2"],
        svec!["e", "5"],
        svec!["c", "3"],
        svec!["a", "1"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sqlp_boston311_explain() {
    let wrk = Workdir::new("sqlp_boston311_explain");
    let test_file = wrk.load_test_file("boston311-100.csv");

    let mut cmd = wrk.command("sqlp");
    cmd.arg(&test_file).arg("--try-parsedates").arg(
        "explain select ward, cast(avg(closed_dt - open_dt) as float) as avg_tat from _t_1 where \
         case_status = 'Closed' group by ward order by avg_tat desc, ward asc",
    );

    let got: String = wrk.stdout(&mut cmd);
    let expected_begin = r#"Logical Plan
"SORT BY [col(""avg_tat""), col(""ward"")]"
"  FAST_PROJECT: [ward, avg_tat]"
    AGGREGATE
"    	[[(col(""closed_dt"")) - (col(""open_dt""))].mean().cast(Float32).alias(""avg_tat"")] BY [col(""ward"")] FROM""#;
    assert!(got.starts_with(expected_begin));

    let expected_end = r#"boston311-100.csv
        PROJECT 4/29 COLUMNS
"        SELECTION: [(col(""case_status"")) == (Utf8(Closed))]""#;
    assert!(got.ends_with(expected_end));
}

#[test]
fn sqlp_boston311_sql_script() {
    let wrk = Workdir::new("sqlp_boston311_sql_script");
    let test_file = wrk.load_test_file("boston311-100.csv");

    wrk.create_from_string(
        "test.sql",
        r#"create table temp_table as select * from "boston311-100" where ontime = 'OVERDUE';
create table temp_table2 as select * from temp_table limit 10;
select ward,count(*) as cnt from temp_table2 group by ward order by cnt desc, ward asc;"#,
    );

    let mut cmd = wrk.command("sqlp");
    cmd.arg(&test_file).arg("test.sql");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["ward", "cnt"],
        svec!["Ward 3", "2"],
        svec![" ", "1"],
        svec!["04", "1"],
        svec!["3", "1"],
        svec!["Ward 13", "1"],
        svec!["Ward 17", "1"],
        svec!["Ward 19", "1"],
        svec!["Ward 21", "1"],
        svec!["Ward 6", "1"],
    ];

    assert_eq!(got, expected);
}

#[test]
fn sqlp_boston311_sql_script_json() {
    let wrk = Workdir::new("sqlp_boston311_sql_script_json");
    let test_file = wrk.load_test_file("boston311-100.csv");

    wrk.create_from_string(
        "test.sql",
        r#"create table temp_table as select * from "boston311-100" where ontime = 'OVERDUE';
create table temp_table2 as select * from temp_table limit 10;
select ward,count(*) as cnt from temp_table2 group by ward order by cnt desc, ward asc;"#,
    );

    let mut cmd = wrk.command("sqlp");
    cmd.arg(&test_file)
        .arg("test.sql")
        .args(["--format", "json"]);

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"{"ward":"Ward 3","cnt":2}
{"ward":" ","cnt":1}
{"ward":"04","cnt":1}
{"ward":"3","cnt":1}
{"ward":"Ward 13","cnt":1}
{"ward":"Ward 17","cnt":1}
{"ward":"Ward 19","cnt":1}
{"ward":"Ward 21","cnt":1}
{"ward":"Ward 6","cnt":1}"#;

    assert_eq!(got, expected);
}

#[test]
fn sqlp_boston311_cte_script() {
    let wrk = Workdir::new("sqlp_boston311_cte");
    let test_file = wrk.load_test_file("boston311-100.csv");

    wrk.create_from_string(
        "test.sql",
        r#"with boston311_roxbury as (select * from "boston311-100" where neighborhood = 'Roxbury')
select ward,count(*) as cnt from boston311_roxbury group by ward order by cnt desc, ward asc;"#,
    );

    let mut cmd = wrk.command("sqlp");
    cmd.arg(&test_file).arg("test.sql");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["ward", "cnt"],
        svec!["Ward 11", "2"],
        svec!["Ward 13", "2"],
        svec!["Ward 8", "2"],
        svec!["14", "1"],
        svec!["Ward 12", "1"],
    ];

    assert_eq!(got, expected);
}

#[test]
fn sqlp_boston311_cte() {
    let wrk = Workdir::new("sqlp_boston311_cte");
    let test_file = wrk.load_test_file("boston311-100.csv");

    let mut cmd = wrk.command("sqlp");
    cmd.arg(&test_file).arg(
        r#"with boston311_roxbury as (select * from "boston311-100" where neighborhood = 'Roxbury')
    select ward,count(*) as cnt from boston311_roxbury group by ward order by cnt desc, ward asc;"#,
    );

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["ward", "cnt"],
        svec!["Ward 11", "2"],
        svec!["Ward 13", "2"],
        svec!["Ward 8", "2"],
        svec!["14", "1"],
        svec!["Ward 12", "1"],
    ];

    assert_eq!(got, expected);
}

#[test]
fn sqlp_boston311_case_expression() {
    let wrk = Workdir::new("sqlp_boston311_case_expression");
    let test_file = wrk.load_test_file("boston311-100.csv");

    let mut cmd = wrk.command("sqlp");
    cmd.arg(&test_file).arg(
        r#"SELECT case_enquiry_id, 
           CASE closed_dt is null and case_title ~* 'graffiti' 
              WHEN True THEN 'Yes' 
              WHEN False THEN 'No' 
              ELSE 'N/A'
           END as graffiti_related
           from _t_1
           where case_status = 'Open'"#,
    );

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["case_enquiry_id", "graffiti_related"],
        svec!["101004143000", "No"],
        svec!["101004155594", "No"],
        svec!["101004154423", "No"],
        svec!["101004141848", "No"],
        svec!["101004113313", "No"],
        svec!["101004113751", "Yes"],
        svec!["101004113902", "Yes"],
        svec!["101004113473", "No"],
        svec!["101004113604", "No"],
        svec!["101004114154", "Yes"],
        svec!["101004114383", "No"],
        svec!["101004114795", "Yes"],
        svec!["101004118346", "Yes"],
        svec!["101004115302", "No"],
        svec!["101004115066", "No"],
    ];

    assert_eq!(got, expected);
}

#[test]
fn sqlp_boston311_case() {
    let wrk = Workdir::new("sqlp_boston311_case");
    let test_file = wrk.load_test_file("boston311-100.csv");

    let mut cmd = wrk.command("sqlp");
    cmd.arg(&test_file).arg(
        r#"SELECT case_enquiry_id, 
           CASE 
              WHEN case_title ~* 'graffiti' THEN 'Graffitti' 
              WHEN case_title ~* 'vehicle' THEN 'Vehicle'
              WHEN case_title ~* 'sidewalk' THEN 'Sidewalk'
              ELSE 'Something else'
           END as topic
           from _t_1
           where case_status = 'Open'"#,
    );

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["case_enquiry_id", "topic"],
        svec!["101004143000", "Something else"],
        svec!["101004155594", "Something else"],
        svec!["101004154423", "Sidewalk"],
        svec!["101004141848", "Something else"],
        svec!["101004113313", "Something else"],
        svec!["101004113751", "Graffitti"],
        svec!["101004113902", "Graffitti"],
        svec!["101004113473", "Sidewalk"],
        svec!["101004113604", "Something else"],
        svec!["101004114154", "Graffitti"],
        svec!["101004114383", "Something else"],
        svec!["101004114795", "Graffitti"],
        svec!["101004118346", "Graffitti"],
        svec!["101004115302", "Vehicle"],
        svec!["101004115066", "Sidewalk"],
    ];

    assert_eq!(got, expected);
}
