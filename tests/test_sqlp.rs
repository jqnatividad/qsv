use crate::workdir::Workdir;

macro_rules! sqlp_test {
    ($name:ident, $fun:expr) => {
        mod $name {
            use std::process;

            #[allow(unused_imports)]
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
        all_rows.push(svec!["city", "state", "city:places", "place"]);
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
                svec!["Boston", "MA", "Boston", "Logan Airport"],
                svec!["Boston", "MA", "Boston", "Boston Garden"],
                svec!["Buffalo", "NY", "Buffalo", "Ralph Wilson Stadium"],
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
                svec!["Boston", "MA", "Boston", "Logan Airport"],
                svec!["Boston", "MA", "Boston", "Boston Garden"],
                svec!["New York", "NY", "", ""],
                svec!["San Francisco", "CA", "", ""],
                svec!["Buffalo", "NY", "Buffalo", "Ralph Wilson Stadium"],
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
        let expected1 = vec![
            svec!["city", "state", "city:places", "place"],
            svec!["Boston", "MA", "Boston", "Logan Airport"],
            svec!["Boston", "MA", "Boston", "Boston Garden"],
            svec!["Buffalo", "NY", "Buffalo", "Ralph Wilson Stadium"],
            svec!["", "", "Orlando", "Disney World"],
            svec!["New York", "NY", "", ""],
            svec!["San Francisco", "CA", "", ""],
        ];
        let expected2 = vec![
            svec!["city", "state", "city:places", "place"],
            svec!["Boston", "MA", "Boston", "Logan Airport"],
            svec!["Boston", "MA", "Boston", "Boston Garden"],
            svec!["Buffalo", "NY", "Buffalo", "Ralph Wilson Stadium"],
            svec!["", "", "Orlando", "Disney World"],
            svec!["San Francisco", "CA", "", ""],
            svec!["New York", "NY", "", ""],
        ];
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
fn sqlp_join_same_colname_1820() {
    let wrk = Workdir::new("sqlp_join_same_colname_1820");
    wrk.create("one.csv", vec![svec!["id", "data"], svec!["1", "open"]]);
    wrk.create("two.csv", vec![svec!["id", "data"], svec!["1", "closed"]]);

    let mut cmd = wrk.command("sqlp");
    cmd.args(["one.csv", "two.csv"])
        .arg("SELECT _t_1.id, _t_2.data FROM _t_1 JOIN _t_2 ON _t_1.id = _t_2.id");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["id", "data"], svec!["1", "closed"]];
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
fn sqlp_boston311_groupby_orderby_all() {
    let wrk = Workdir::new("sqlp_boston311_groupby_orderby_all");
    let test_file = wrk.load_test_file("boston311-100.csv");

    let mut cmd = wrk.command("sqlp");

    cmd.arg(&test_file)
        .arg(r#"select ward, count(*) as cnt from "boston311-100" group by ward order by all"#);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["ward", "cnt"],
        svec![" ", "1"],
        svec!["01", "1"],
        svec!["02", "1"],
        svec!["03", "2"],
        svec!["04", "1"],
        svec!["06", "1"],
        svec!["07", "1"],
        svec!["1", "1"],
        svec!["10", "1"],
        svec!["14", "4"],
        svec!["16", "1"],
        svec!["17", "2"],
        svec!["18", "1"],
        svec!["19", "1"],
        svec!["21", "1"],
        svec!["22", "2"],
        svec!["3", "5"],
        svec!["7", "1"],
        svec!["8", "1"],
        svec!["9", "1"],
        svec!["Ward 1", "6"],
        svec!["Ward 10", "1"],
        svec!["Ward 11", "2"],
        svec!["Ward 12", "1"],
        svec!["Ward 13", "4"],
        svec!["Ward 14", "1"],
        svec!["Ward 15", "1"],
        svec!["Ward 16", "4"],
        svec!["Ward 17", "1"],
        svec!["Ward 18", "3"],
        svec!["Ward 19", "3"],
        svec!["Ward 2", "1"],
        svec!["Ward 20", "5"],
        svec!["Ward 21", "2"],
        svec!["Ward 22", "1"],
        svec!["Ward 3", "10"],
        svec!["Ward 4", "5"],
        svec!["Ward 5", "5"],
        svec!["Ward 6", "7"],
        svec!["Ward 7", "3"],
        svec!["Ward 8", "3"],
        svec!["Ward 9", "1"],
    ];
    assert_eq!(got, expected);
}

#[test]
// #[ignore = "temporarily disable due to a bug in polars aliasing"]
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
        svec!["INTERSECTION Asticou Rd & Washington St", "Not Specified"],
        svec![
            "INTERSECTION Charles River Plz & Cambridge St",
            "Not Specified"
        ],
        svec!["INTERSECTION Columbia Rd & E Cottage St", "Not Specified"],
        svec!["INTERSECTION E Canton St & Albany St", "Not Specified"],
        svec![
            "INTERSECTION Gallivan Blvd & Washington St",
            "Not Specified"
        ],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sqlp_decimal_comma_issue_1050() {
    let wrk = Workdir::new("sqlp_decimal_comma_issue_1050");
    let test_file = wrk.load_test_file("progetti_sample_10.csv");

    let mut cmd = wrk.command("sqlp");

    cmd.arg(&test_file)
        .arg("--decimal-comma")
        .args(["--delimiter", ";"])
        .arg("select COD_LOCALE_PROGETTO, FINANZ_UE from _t_1");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["COD_LOCALE_PROGETTO;FINANZ_UE"],
        svec!["1AGCOE1627;6328008.44"],
        svec!["1AGCOE2113;344203.2"],
        svec!["1AGCOE645;491260.0"],
        svec!["1AGCOE705;522811.32"],
        svec!["1AGCOE706;460463.85"],
        svec!["1ANPALINP-001;141566507.74"],
        svec!["1ANPALINP-CLP-00002;47325000.0"],
        svec!["1ANPALINP-CLP-00003;78450000.0"],
        svec!["1ANPALINP-CLP-00004;67500000.0"],
        svec!["1ANPALVAO1C001;6416.62"],
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
fn sqlp_rnull_values_wnull_value() {
    let wrk = Workdir::new("sqlp_rnull_values_wnull_value");
    wrk.create(
        "test_null.csv",
        vec![
            svec!["a", "b"],
            svec!["1", "Nothing"],
            svec!["2", "NA"],
            svec!["3", "Dunno"],
            svec!["4", "4"],
            svec!["5", ""],
            svec!("6", "6"),
            svec!("7", "DUNNO"),
        ],
    );

    let mut cmd = wrk.command("sqlp");

    cmd.arg("test_null.csv")
        .args(["--rnull-values", "Nothing,NA,<empty string>,Dunno"])
        .args(["--wnull-value", "NULL"])
        .arg("SELECT * FROM test_null");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["a", "b"],
        svec!["1", "NULL"],
        svec!["2", "NULL"],
        svec!["3", "NULL"],
        svec!["4", "4"],
        svec!["5", "NULL"],
        svec!["6", "6"],
        svec!["7", "DUNNO"],
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
        svec!["false"],
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
        svec!["   "],
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
        svec!["bc"],
        svec!["bc"],
        svec!["  "],
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
        svec!["    ABC"],
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
        svec!["    abc"],
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
        svec!["7"],
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
        svec!["7"],
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
        svec!["Ward 13", "1518365750000.0"],
        svec!["Ward 15", "1278926000000.0"],
        svec!["Ward 21", "878446000000.0"],
        svec!["Ward 14", "618933000000.0"],
        svec!["Ward 3", "437691444444.0"],
        svec!["Ward 5", "411909500000.0"],
        svec!["Ward 20", "367233000000.0"],
        svec!["9", "353495000000.0"],
        svec!["Ward 18", "249882000000.0"],
        svec!["19", "212566000000.0"],
        svec!["Ward 4", "112872600000.0"],
        svec!["Ward 1", "107850666666.0"],
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
        svec!["Ward 6", "54770166666.0"],
        svec!["Ward 7", "38346333333.0"],
        svec!["Ward 8", "32767500000.0"],
        svec!["03", "29810500000.0"],
        svec!["07", "25328000000.0"],
        svec!["22", "23919000000.0"],
        svec!["14", "20786500000.0"],
        svec!["Ward 22", "13524000000.0"],
        svec!["1", "9469000000.0"],
        svec!["06", "5290000000.0"],
        svec!["Ward 16", "4533666666.0"],
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
        svec!["Ward 11", "4847760000000.000"],
        svec!["01", "4818270000000.000"],
        svec!["Ward 13", "1518365750000.000"],
        svec!["Ward 15", "1278926000000.000"],
        svec!["Ward 21", "878446000000.000"],
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
fn sqlp_compress() {
    let wrk = Workdir::new("sqlp_compress");
    wrk.create(
        "data.csv",
        vec![
            svec!["column1", "column2"],
            svec!["a", "1"],
            svec!["c", "3"],
            svec!["e", "5"],
        ],
    );

    let out_file = wrk.path("out.csv.sz").to_string_lossy().to_string();

    let mut cmd = wrk.command("sqlp");
    cmd.arg("data.csv")
        .arg("select column1, column2 from data order by column2 desc")
        .args(["-o", &out_file]);

    wrk.assert_success(&mut cmd);

    let mut cmd2 = wrk.command("snappy"); // DevSkim: ignore DS126858
    cmd2.arg("decompress").arg(out_file); // DevSkim: ignore DS126858

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd2); // DevSkim: ignore DS126858
    let expected = vec![
        svec!["column1", "column2"],
        svec!["e", "5"],
        svec!["c", "3"],
        svec!["a", "1"],
    ];

    assert_eq!(got, expected);
}

// disable this test on windows as it fails as the expected output is different
// due to the different line endings
#[cfg(not(target_os = "windows"))]
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
  AGGREGATE
"  	[[(col(""closed_dt"")) - (col(""open_dt""))].mean().strict_cast(Float64).alias(""avg_tat"")] BY [col(""ward"")] FROM"
    Csv SCAN"#;
    assert!(got.starts_with(expected_begin));

    let expected_end = r#"boston311-100.csv]
    PROJECT 4/29 COLUMNS
"    SELECTION: [(col(""case_status"")) == (String(Closed))]""#;
    assert!(got.ends_with(expected_end));
}

#[test]
// #[ignore = "temporarily disable due to a bug in polars aliasing"]
fn sqlp_boston311_sql_script() {
    let wrk = Workdir::new("sqlp_boston311_sql_script");
    let test_file = wrk.load_test_file("boston311-100.csv");

    wrk.create_from_string(
        "test.sql",
        r#"create table temp_table as select * from "boston311-100" where ontime = 'OVERDUE';
create table temp_table2 as select * from temp_table limit 10;
-- we already got what we needed from temp_table into temp_table2
-- so we can truncate temp_table. Otherwise, the memory taken by temp_table
-- won't be released until the end of the script
truncate temp_table;
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
// #[ignore = "temporarily disable due to a bug in polars aliasing"]
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
    let expected = r#"[{"ward":"Ward 3","cnt":2},{"ward":" ","cnt":1},{"ward":"04","cnt":1},{"ward":"3","cnt":1},{"ward":"Ward 13","cnt":1},{"ward":"Ward 17","cnt":1},{"ward":"Ward 19","cnt":1},{"ward":"Ward 21","cnt":1},{"ward":"Ward 6","cnt":1}]"#;

    assert_eq!(got, expected);
}

#[test]
// #[ignore = "temporarily disable due to a bug in polars aliasing"]
fn sqlp_boston311_sql_script_jsonl() {
    let wrk = Workdir::new("sqlp_boston311_sql_script_jsonl");
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
        .args(["--format", "jsonl"]);

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
// #[ignore = "temporarily disable due to a bug in polars aliasing"]
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
// #[ignore = "temporarily disable due to a bug in polars aliasing"]
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

#[test]
fn sqlp_literal_pattern_match() {
    let wrk = Workdir::new("sqlp_literal_pattern_match");
    wrk.create(
        "test.csv",
        vec![
            svec!["idx", "val"],
            svec!["0", "ABC"],
            svec!["1", "abc"],
            svec!["2", "000"],
            svec!["3", "A0C"],
            svec!["4", "a0c"],
        ],
    );

    let mut cmd = wrk.command("sqlp");
    cmd.arg("test.csv")
        .arg(r#"SELECT * FROM test WHERE val NOT REGEXP '.*c$'"#);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["idx", "val"],
        svec!["0", "ABC"],
        svec!["2", "000"],
        svec!["3", "A0C"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sqlp_expression_pattern_match() {
    let wrk = Workdir::new("sqlp_expression_pattern_match");
    wrk.create(
        "test.csv",
        vec![
            svec!["idx", "val", "pat"],
            svec!["0", "ABC", "^A"],
            svec!["1", "abc", "^A"],
            svec!["2", "000", "^A"],
            svec!["3", "A0C", r#"[AB]\d.*$"#,],
            svec!["4", "a0c", ".*xxx$"],
        ],
    );

    let mut cmd = wrk.command("sqlp");
    cmd.arg("test.csv")
        .arg("SELECT idx, val FROM test WHERE val REGEXP pat");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["idx", "val"], svec!["0", "ABC"], svec!["3", "A0C"]];
    assert_eq!(got, expected);
}

#[test]
fn sqlp_sql_join_on_subquery() {
    let wrk = Workdir::new("sqlp_sql_join_on_subquery");
    wrk.create(
        "test.csv",
        vec![
            svec!["idx", "val"],
            svec!["0", "ABC"],
            svec!["1", "abc"],
            svec!["2", "000"],
            svec!["3", "A0C"],
            svec!["4", "a0c"],
        ],
    );

    wrk.create(
        "test2.csv",
        vec![
            svec!["idx", "val"],
            svec!["0", "ABC"],
            svec!["1", "abc"],
            svec!["2", "000"],
            svec!["3", "A0C"],
            svec!["4", "a0c"],
        ],
    );

    let mut cmd = wrk.command("sqlp");
    cmd.arg("test.csv").arg("test2.csv").arg(
        "SELECT * FROM test t1 JOIN (SELECT idx, val FROM test2 WHERE idx > 2) t2 ON t1.idx = \
         t2.idx",
    );

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["idx", "val", "idx:t2", "val:t2"],
        svec!["3", "A0C", "3", "A0C"],
        svec!["4", "a0c", "4", "a0c"],
    ];

    assert_eq!(got, expected);
}

#[test]
fn sqlp_sql_from_subquery() {
    let wrk = Workdir::new("sqlp_sql_from_subquery");
    wrk.create(
        "test.csv",
        vec![
            svec!["idx", "val"],
            svec!["0", "ABC"],
            svec!["1", "abc"],
            svec!["2", "000"],
            svec!["3", "A0C"],
            svec!["4", "a0c"],
        ],
    );

    wrk.create(
        "test2.csv",
        vec![
            svec!["idx", "val"],
            svec!["0", "ABC"],
            svec!["1", "abc"],
            svec!["2", "000"],
            svec!["3", "A0C"],
            svec!["4", "a0c"],
        ],
    );

    let mut cmd = wrk.command("sqlp");
    cmd.arg("test.csv").arg("test2.csv").arg(
        "SELECT * FROM (SELECT idx, val FROM test WHERE idx > 2) t1 JOIN test2 t2 ON t1.idx = \
         t2.idx",
    );

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["idx", "val", "idx:t2", "val:t2"],
        svec!["3", "A0C", "3", "A0C"],
        svec!["4", "a0c", "4", "a0c"],
    ];

    assert_eq!(got, expected);
}

#[test]
fn sqlp_sql_tsv() {
    let wrk = Workdir::new("sqlp_sql_tsv");
    wrk.create(
        "test.csv",
        vec![
            svec!["idx", "val"],
            svec!["0", "ABC"],
            svec!["1", "abc"],
            svec!["2", "000"],
            svec!["3", "A0C"],
            svec!["4", "a0c"],
        ],
    );

    let output_file = wrk.path("output.tsv").to_string_lossy().to_string();

    let mut cmd = wrk.command("sqlp");
    cmd.arg("test.csv")
        .arg("SELECT * FROM test")
        .args(["--output", &output_file]);

    wrk.assert_success(&mut cmd);

    let got = wrk.read_to_string(&output_file);

    let expected = "idx\tval\n0\tABC\n1\tabc\n2\t000\n3\tA0C\n4\ta0c\n";

    assert_eq!(got, expected);
}

#[test]
fn sqlp_binary_functions() {
    let wrk = Workdir::new("sqlp_sql_binary_functions");
    wrk.create("dummy.csv", vec![svec!["dummy"], svec!["0"]]);

    let mut cmd = wrk.command("sqlp");
    cmd.arg("dummy.csv")
        .arg(
            r#"
        SELECT *,
          -- bit strings
          b''                 AS b0,
          b'1001'             AS b1,
          b'11101011'         AS b2,
          b'1111110100110010' AS b3,
          -- hex strings
          x''                 AS x0,
          x'FF'               AS x1,
          x'4142'             AS x2,
          x'DeadBeef'         AS x3,
        FROM dummy
        "#,
        )
        .args(["--format", "parquet"]);

    wrk.assert_success(&mut cmd);
}

#[test]
fn sqlp_length_fns() {
    let wrk = Workdir::new("sqlp_sql_length_fns");
    wrk.create(
        "test.csv",
        vec![svec!["words"], svec!["Cafe"], svec![""], svec!["東京"]],
    );

    let mut cmd = wrk.command("sqlp");
    cmd.arg("test.csv").arg(
        r#"
        SELECT
              words,
              LENGTH(words) AS n_chrs1,
              CHAR_LENGTH(words) AS n_chrs2,
              CHARACTER_LENGTH(words) AS n_chrs3,
              OCTET_LENGTH(words) AS n_bytes,
              BIT_LENGTH(words) AS n_bits
            FROM test
"#,
    );

    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["words", "n_chrs1", "n_chrs2", "n_chrs3", "n_bytes", "n_bits"],
        svec!["Cafe", "4", "4", "4", "4", "32"],
        svec!["", "", "", "", "", ""],
        svec!["東京", "2", "2", "2", "6", "48"],
    ];

    assert_eq!(got, expected);
}

#[test]
fn sqlp_control_flow() {
    let wrk = Workdir::new("sqlp_control_flow");
    wrk.create(
        "test.csv",
        vec![
            svec!["x", "y", "z"],
            svec!["1", "5", "3"],
            svec!["", "4", "4"],
            svec!["2", "", ""],
            svec!["3", "3", "3"],
            svec!["", "", "6"],
            svec!["4", "2", ""],
        ],
    );

    let mut cmd = wrk.command("sqlp");
    cmd.arg("test.csv").arg(
        r#"
        SELECT
          COALESCE(x,y,z) as "coalsc",
          NULLIF(x, y) as "nullif x_y",
          NULLIF(y, z) as "nullif y_z",
          IFNULL(x, y) as "ifnull x_y",
          IFNULL(y,-1) as "inullf y_z",
          COALESCE(x, NULLIF(y,z)) as "both",
          IF(x = y, 'eq', 'ne') as "x_eq_y",
        FROM test
"#,
    );

    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec![
            "coalsc",
            "nullif x_y",
            "nullif y_z",
            "ifnull x_y",
            "inullf y_z",
            "both",
            "x_eq_y"
        ],
        svec!["1", "1", "5", "1", "5", "1", "ne"],
        svec!["4", "", "", "4", "4", "", "ne"],
        svec!["2", "2", "", "2", "-1", "2", "ne"],
        svec!["3", "", "", "3", "3", "3", "eq"],
        svec!["6", "", "", "", "-1", "", "ne"],
        svec!["4", "4", "2", "4", "2", "4", "ne"],
    ];

    assert_eq!(got, expected);
}

#[test]
fn sqlp_div_sign() {
    let wrk = Workdir::new("sqlp_div_sign");
    wrk.create(
        "test.csv",
        vec![
            svec!["a", "b"],
            svec!["10.0", "-100.5"],
            svec!["20.0", "7.0"],
            svec!["30.0", "2.5"],
            svec!["40.0", ""],
            svec!["50.0", "-3.14"],
        ],
    );

    let mut cmd = wrk.command("sqlp");
    cmd.arg("test.csv").arg(
        r#"
        SELECT
            a / b AS a_div_b,
            a // b AS a_floordiv_b,
            SIGN(b) AS b_sign,
        FROM test
"#,
    );

    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["a_div_b", "a_floordiv_b", "b_sign"],
        svec!["-0.09950248756218906", "-1", "-1"],
        svec!["2.857142857142857", "2", "1"],
        svec!["12.0", "12", "1"],
        svec!["", "", ""],
        svec!["-15.92356687898089", "-16", "-1"],
    ];

    assert_eq!(got, expected);
}

#[test]
fn sqlp_string_replace() {
    let wrk = Workdir::new("sqlp_string_replace");
    wrk.create(
        "test.csv",
        vec![
            svec!["words"],
            svec!["Yemeni coffee is the best coffee"],
            svec![""],
        ],
    );

    let mut cmd = wrk.command("sqlp");
    cmd.arg("test.csv").arg(
        r#"
        SELECT
        REPLACE(
          REPLACE(words, 'coffee', 'tea'),
          'Yemeni',
          'English breakfast'
        )
        FROM test
"#,
    );

    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["words"],
        svec!["English breakfast tea is the best tea"],
    ];

    assert_eq!(got, expected);
}

#[test]
fn sqlp_compound_join_basic() {
    let wrk = Workdir::new("sqlp_compound_join_basic");
    wrk.create(
        "test1.csv",
        vec![
            svec!["a", "b"],
            svec!["1", "1"],
            svec!["2", "3"],
            svec!["3", "4"],
            svec!["4", "4"],
            svec!["5", "5"],
        ],
    );

    wrk.create(
        "test2.csv",
        vec![
            svec!["a", "b"],
            svec!["1", "0"],
            svec!["2", "3"],
            svec!["3", "4"],
            svec!["4", "5"],
            svec!["5", "6"],
        ],
    );

    let mut cmd = wrk.command("sqlp");
    cmd.args(["test1.csv", "test2.csv"])
        .arg(
            r#"
        SELECT * FROM test1
         INNER JOIN test2 ON test1.a = test2.a AND test1.b = test2.b
"#,
        )
        .arg("--truncate-ragged-lines");

    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["a", "b", "a:test2", "b:test2"],
        svec!["2", "3", "2", "3"],
        svec!["3", "4", "3", "4"],
    ];

    assert_eq!(got, expected);
}

#[test]
fn sqlp_compound_join_diff_colnames() {
    let wrk = Workdir::new("sqlp_compound_join_diff_colnames");
    wrk.create(
        "test1.csv",
        vec![
            svec!["a", "b"],
            svec!["1", "1"],
            svec!["2", "2"],
            svec!["3", "3"],
            svec!["4", "4"],
            svec!["5", "5"],
        ],
    );

    wrk.create(
        "test2.csv",
        vec![
            svec!["a", "b", "c"],
            svec!["0", "1", "7"],
            svec!["2", "2", "8"],
            svec!["3", "3", "9"],
            svec!["4", "5", "10"],
            svec!["5", "6", "11"],
        ],
    );

    let mut cmd = wrk.command("sqlp");
    cmd.args(["test1.csv", "test2.csv"])
        .arg(
            r#"
        SELECT * FROM test1
         INNER JOIN test2 ON test1.a = test2.b AND test1.b = test2.a
"#,
        )
        .arg("--truncate-ragged-lines");

    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["a", "b", "a:test2", "b:test2", "c"],
        svec!["2", "2", "2", "2", "8"],
        svec!["3", "3", "3", "3", "9"],
    ];

    assert_eq!(got, expected);
}

#[test]
fn sqlp_compound_join_three_tables() {
    let wrk = Workdir::new("sqlp_compound_join_three_tables");
    wrk.create(
        "test1.csv",
        vec![
            svec!["a", "b"],
            svec!["1", "1"],
            svec!["2", "3"],
            svec!["3", "4"],
            svec!["4", "4"],
            svec!["5", "5"],
        ],
    );

    wrk.create(
        "test2.csv",
        vec![
            svec!["a", "b"],
            svec!["1", "0"],
            svec!["2", "3"],
            svec!["3", "4"],
            svec!["4", "5"],
            svec!["5", "6"],
        ],
    );

    wrk.create(
        "test3.csv",
        vec![
            svec!["a", "b", "c"],
            svec!["1", "0", "0"],
            svec!["2", "3", "3"],
            svec!["3", "4", "4"],
            svec!["4", "5", "5"],
            svec!["5", "6", "6"],
        ],
    );

    let mut cmd = wrk.command("sqlp");
    cmd.args(["test1.csv", "test2.csv", "test3.csv"])
        .arg(
            r#"
            SELECT * FROM test1
            INNER JOIN test2
                ON test1.a = test2.a AND test1.b = test2.b
            INNER JOIN test3
                ON test1.a = test3.a AND test1.b = test3.b
"#,
        )
        .arg("--truncate-ragged-lines");

    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["a", "b", "a:test2", "b:test2", "a:test3", "b:test3", "c"],
        svec!["2", "3", "2", "3", "2", "3", "3"],
        svec!["3", "4", "3", "4", "3", "4", "4"],
    ];

    assert_eq!(got, expected);
}

#[test]
fn sqlp_string_concat() {
    let wrk = Workdir::new("sqlp_string_concat");
    wrk.create(
        "test.csv",
        vec![
            svec!["x", "y", "z"],
            svec!["a", "d", "1"],
            svec!["", "e", "2"],
            svec!["c", "f", "3"],
        ],
    );

    let mut cmd = wrk.command("sqlp");
    cmd.arg("test.csv").arg(
        r#"
        SELECT
           ("x" || "x" || "y")           AS c0,
           ("x" || "y" || "z")           AS c1,
           CONCAT(("x" + "x"), "y")      AS c2,
           CONCAT("x", "x", "y")         AS c3,
           CONCAT("x", "y", ("z" * 2))   AS c4,
           CONCAT_WS(':', "x", "y", "z") AS c5,
           CONCAT_WS('!', "x", "y", "z") AS c6,
        FROM test
"#,
    );

    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["c0", "c1", "c2", "c3", "c4", "c5", "c6"],
        svec!["aad", "ad1", "aad", "aad", "ad2", "a:d:1", "a!d!1"],
        svec!["", "", "e", "e", "e4", "e:2", "e!2"],
        svec!["ccf", "cf3", "ccf", "ccf", "cf6", "c:f:3", "c!f!3"],
    ];

    assert_eq!(got, expected);
}

#[test]
fn sqlp_string_right_reverse() {
    let wrk = Workdir::new("sqlp_string_right_reverse");
    wrk.create(
        "test.csv",
        vec![
            svec!["txt"],
            svec!["abcde"],
            svec!["abc"],
            svec!["a"],
            svec![""],
        ],
    );

    let mut cmd = wrk.command("sqlp");
    cmd.arg("test.csv").arg(
        r#"
        SELECT
           LEFT(txt,2) AS "l",
           RIGHT(txt,2) AS "r",
           REVERSE(txt) AS "rev"
        FROM test
"#,
    );

    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["l", "r", "rev"],
        svec!["ab", "de", "edcba"],
        svec!["ab", "bc", "cba"],
        svec!["a", "a", "a"],
        svec!["", "", ""],
    ];

    assert_eq!(got, expected);
}

#[test]
fn sqlp_modulo() {
    let wrk = Workdir::new("sqlp_modulo");
    wrk.create(
        "test.csv",
        vec![
            svec!["a", "b", "c", "d"],
            svec!["1.5", "6", "11", "16.5"],
            svec!["", "7", "12", "17.0"],
            svec!["3.0", "8", "13", "18.5"],
            svec!["4333333333", "9", "14", ""],
            svec!["5.0", "10", "15", "20.0"],
        ],
    );

    let mut cmd = wrk.command("sqlp");
    cmd.arg("test.csv").arg(
        r#"
        SELECT
            cast(a as float) % 2 AS a2,
            cast(b as float)  % 3 AS b3,
            MOD(cast(c as float), 4) AS c4,
            MOD(cast(d as float), 5.5) AS d55
        FROM test
"#,
    );

    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["a2", "b3", "c4", "d55"],
        svec!["1.5", "0.0", "3.0", "0.0"],
        svec!["", "1.0", "0.0", "0.5"],
        svec!["1.0", "2.0", "1.0", "2.0"],
        svec!["1.0", "0.0", "2.0", ""],
        svec!["1.0", "1.0", "3.0", "3.5"],
    ];

    assert_eq!(got, expected);
}

#[test]
fn sqlp_try_cast() {
    let wrk = Workdir::new("sqlp_try_cast");
    wrk.create(
        "test.csv",
        vec![
            svec!["foo", "bar"],
            svec!["65432", "1999-12-31"],
            svec!["101010", "N/A"],
            svec!["-3333", "2024-01-01"],
        ],
    );

    let mut cmd = wrk.command("sqlp");
    cmd.arg("test.csv").arg(
        r#"
        SELECT
            try_cast(foo as uint2),
            try_cast(bar as DATE)
        FROM test
"#,
    );

    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["foo", "bar"],
        svec!["65432", "1999-12-31"],
        svec!["", ""],
        svec!["", "2024-01-01"],
    ];

    assert_eq!(got, expected);
}

#[test]
fn sqlp_stddev_variance() {
    let wrk = Workdir::new("sqlp_stddev_variance");
    wrk.create(
        "test.csv",
        vec![
            svec!["v1", "v2", "v3", "v4"],
            svec!["-1.0", "5.5", "-10", "-100"],
            svec!["0.0", "0.0", "", "0.0"],
            svec!["1.0", "3.0", "10", "-50.0"],
        ],
    );

    let mut cmd = wrk.command("sqlp");
    cmd.arg("test.csv").arg(
        r#"
        SELECT
            STDEV(v1) AS "v1_std",
            STDDEV(v2) AS "v2_std",
            STDEV_SAMP(v3) AS "v3_std",
            STDDEV_SAMP(v4) AS "v4_std",
            VAR(v1) AS "v1_var",
            VARIANCE(v2) AS "v2_var",
            VARIANCE(v3) AS "v3_var",
            VAR_SAMP(v4) AS "v4_var"
        FROM test
"#,
    );

    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["v1_std", "v2_std", "v3_std", "v4_std", "v1_var", "v2_var", "v3_var", "v4_var"],
        svec![
            "1.0",
            "2.753785273643051",
            "14.142135623730951",
            "50.0",
            "1.0",
            "7.583333333333334",
            "200.0",
            "2500.0"
        ],
    ];

    assert_eq!(got, expected);
}

#[test]
fn sqlp_string_position() {
    let wrk = Workdir::new("sqlp_string_position");
    wrk.create(
        "cities.csv",
        vec![
            svec!["city"],
            svec!["Dubai"],
            svec!["Abu Dhabi"],
            svec!["Sharjah"],
            svec!["Al Ain"],
            svec!["Ajman"],
            svec!["Ras Al Khaimah"],
        ],
    );

    let mut cmd = wrk.command("sqlp");
    cmd.arg("cities.csv").arg(
        r#"
        SELECT
        POSITION('a' IN city) AS a_lc1,
        POSITION('A' IN city) AS a_uc1,
        STRPOS(city,'a') AS a_lc2,
        STRPOS(city,'A') AS a_uc2,
      FROM cities
"#,
    );

    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["a_lc1", "a_uc1", "a_lc2", "a_uc2"],
        svec!["4", "0", "4", "0"],
        svec!["7", "1", "7", "1"],
        svec!["3", "0", "3", "0"],
        svec!["0", "1", "0", "1"],
        svec!["4", "1", "4", "1"],
        svec!["2", "5", "2", "5"],
    ];

    assert_eq!(got, expected);
}

#[test]
fn sqlp_date_part() {
    let wrk = Workdir::new("sqlp_date_part");
    wrk.create(
        "datestbl.csv",
        vec![svec!["datecol"], svec!["20-01-2012 10:30:20"]],
    );

    let mut cmd = wrk.command("sqlp");
    cmd.arg("datestbl.csv")
        .arg(
            r#"
        SELECT 
            DATE_PART('isoyear', datecol) as c1,
            DATE_PART('month', datecol) as c2,
            DATE_PART('day', datecol) as c3,
            DATE_PART('hour', datecol) as c4,
            DATE_PART('minute', datecol) as c5,
            DATE_PART('second', datecol) as c6,
            DATE_PART('millisecond', datecol) as c61,
            DATE_PART('microsecond', datecol) as c62,
            DATE_PART('nanosecond', datecol) as c63,
            DATE_PART('isoweek', datecol) as c7,
            DATE_PART('dayofyear', datecol) as c8,
            DATE_PART('dayofweek', datecol) as c9,
            DATE_PART('time', datecol) as c10,
            DATE_PART('decade', datecol) as c11,
            DATE_PART('century', datecol) as c12,
            DATE_PART('millennium', datecol) as c13,
            DATE_PART('quarter', datecol) as c14,
      FROM datestbl
"#,
        )
        .arg("--try-parsedates");

    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec![
            "c1", "c2", "c3", "c4", "c5", "c6", "c61", "c62", "c63", "c7", "c8", "c9", "c10",
            "c11", "c12", "c13", "c14"
        ],
        svec![
            "2012",
            "1",
            "20",
            "10",
            "30",
            "20",
            "20000.0",
            "20000000.0",
            "20000000000.0",
            "3",
            "20",
            "5",
            "10:30:20.000000000",
            "201",
            "21",
            "3",
            "1"
        ],
    ];

    assert_eq!(got, expected);
}

#[test]
fn sqlp_date_part_tz() {
    let wrk = Workdir::new("sqlp_date_part_tz");
    wrk.create(
        "datestbl.csv",
        vec![
            svec!["datecol"],
            svec!["2012-01-20T10:30:20-05:00"],
            svec!["2012-11-05T10:30:20.1234Z"],
            svec!["1999-03-15T15:30:20.1234+01:00"],
            svec!["1978-08-05T23:30:20.1234-01:00"],
        ],
    );

    let mut cmd = wrk.command("sqlp");
    cmd.arg("datestbl.csv")
        .arg(
            r#"
        SELECT
            DATE_PART('isoyear', datecol) as c1,
            DATE_PART('month', datecol) as c2,
            DATE_PART('day', datecol) as c3,
            DATE_PART('hour', datecol) as c4,
            DATE_PART('minute', datecol) as c5,
            DATE_PART('second', datecol) as c6,
            DATE_PART('millisecond', datecol) as c61,
            DATE_PART('microsecond', datecol) as c62,
            DATE_PART('nanosecond', datecol) as c63,
            DATE_PART('isoweek', datecol) as c7,
            DATE_PART('dayofyear', datecol) as c8,
            DATE_PART('dayofweek', datecol) as c9,
            DATE_PART('time', datecol) as c10,
            DATE_PART('decade', datecol) as c11,
            DATE_PART('century', datecol) as c12,
            DATE_PART('millennium', datecol) as c13,
            DATE_PART('quarter', datecol) as c14,
            DATE_PART('timezone', datecol) as c15,
            DATE_PART('epoch', datecol) as c16,
      FROM datestbl
"#,
        )
        .arg("--try-parsedates");

    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec![
            "c1", "c2", "c3", "c4", "c5", "c6", "c61", "c62", "c63", "c7", "c8", "c9", "c10",
            "c11", "c12", "c13", "c14", "c15", "c16"
        ],
        svec![
            "2012",
            "1",
            "20",
            "15",
            "30",
            "20",
            "20000.0",
            "20000000.0",
            "20000000000.0",
            "3",
            "20",
            "5",
            "15:30:20.000000000",
            "201",
            "21",
            "3",
            "1",
            "0",
            "1327073420.0"
        ],
        svec![
            "2012",
            "11",
            "5",
            "10",
            "30",
            "20",
            "20123.4",
            "20123400.0",
            "20123400000.0",
            "45",
            "310",
            "1",
            "10:30:20.123400000",
            "201",
            "21",
            "3",
            "4",
            "0",
            "1352111420.1234"
        ],
        svec![
            "1999",
            "3",
            "15",
            "14",
            "30",
            "20",
            "20123.4",
            "20123400.0",
            "20123400000.0",
            "11",
            "74",
            "1",
            "14:30:20.123400000",
            "199",
            "20",
            "2",
            "1",
            "0",
            "921508220.1234"
        ],
        svec![
            "1978",
            "8",
            "6",
            "0",
            "30",
            "20",
            "20123.4",
            "20123400.0",
            "20123400000.0",
            "31",
            "218",
            "0",
            "00:30:20.123400000",
            "197",
            "20",
            "2",
            "3",
            "0",
            "271211420.1234"
        ],
    ];

    assert_eq!(got, expected);
}

#[test]
fn sqlp_date() {
    let wrk = Workdir::new("sqlp_date");

    let mut cmd = wrk.command("sqlp");
    cmd.arg("SKIP_INPUT").arg(
        r#"
        SELECT 
            DATE('2021-03-15') as c1,
            DATE('2021-03-15 10:30:20', '%Y-%m-%d %H:%M:%S') as c2,
            DATE('03-15-2021 10:30:20 AM EST', '%m-%d-%Y %I:%M:%S %p %Z') as c3"#,
    );

    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    // this is the documented behavior of the date function
    // use STRFTIME and STRPTIME for more control
    let expected = vec![
        svec!["c1", "c2", "c3"],
        svec!["2021-03-15", "2021-03-15", "2021-03-15"],
    ];

    assert_eq!(got, expected);
}

#[test]
fn sqlp_date_strftime() {
    let wrk = Workdir::new("sqlp_date_strftime");

    wrk.create(
        "data.csv",
        vec![
            svec!["dtm", "dt", "tm"],
            svec!["1972-03-06 23:50:03", "1978-07-05", "10:10:10"],
            svec!["1980-09-30 01:25:50", "1969-12-31", "22:33:55"],
        ],
    );

    let mut cmd = wrk.command("sqlp");
    cmd.arg("data.csv")
        .arg(
            r#"
        SELECT
      STRFTIME(dtm,'%m.%d.%Y/%T') AS s_dtm,
      STRFTIME(dt,'%B %d, %Y') AS s_dt,
      STRFTIME(tm,'%S.%M.%H') AS s_tm,
    FROM data"#,
        )
        .arg("--try-parsedates");

    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["s_dtm", "s_dt", "s_tm"],
        svec!["03.06.1972/23:50:03", "July 05, 1978", "10.10.10"],
        svec!["09.30.1980/01:25:50", "December 31, 1969", "55.33.22"],
    ];

    assert_eq!(got, expected);
}

#[test]
fn sqlp_read_jsonl() {
    let wrk = Workdir::new("sqlp_read_jsonl");

    let test_jsonl = wrk.load_test_file("boston311-10.jsonl");

    let mut cmd = wrk.command("sqlp");
    cmd.arg("SKIP_INPUT").arg(
        format!(
            "SELECT * FROM read_json('{}') WHERE source = 'City Worker App'",
            test_jsonl
        )
        .as_str(),
    );

    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec![
            "case_enquiry_id",
            "open_dt",
            "target_dt",
            "closed_dt",
            "ontime",
            "case_status",
            "closure_reason",
            "case_title",
            "subject",
            "reason",
            "type",
            "queue",
            "department",
            "submittedphoto",
            "closedphoto",
            "location",
            "fire_district",
            "pwd_district",
            "city_council_district",
            "police_district",
            "neighborhood",
            "neighborhood_services_district",
            "ward",
            "precinct",
            "location_street_name",
            "location_zipcode",
            "latitude",
            "longitude",
            "source"
        ],
        svec![
            "101004120108",
            "2022-01-08 12:54:49",
            "2022-01-11 08:30:00",
            "2022-01-09 06:43:06",
            "ONTIME",
            "Closed",
            "Case Closed. Closed date : Sun Jan 09 06:43:06 EST 2022 Noted ",
            "CE Collection",
            "Public Works Department",
            "Street Cleaning",
            "CE Collection",
            "PWDx_District 1C: Downtown",
            "PWDx",
            "",
            "",
            "198 W Springfield St  Roxbury  MA  02118",
            "4",
            "1C",
            "7",
            "D4",
            "South End",
            "6",
            "Ward 9",
            "0902",
            "198 W Springfield St",
            "2118",
            "42.3401",
            "-71.0803",
            "City Worker App"
        ],
        svec![
            "101004141354",
            "2022-01-20 08:07:49",
            "2022-01-21 08:30:00",
            "2022-01-20 08:45:03",
            "ONTIME",
            "Closed",
            "Case Closed. Closed date : Thu Jan 20 08:45:03 EST 2022 Noted ",
            "CE Collection",
            "Public Works Department",
            "Street Cleaning",
            "CE Collection",
            "PWDx_District 1B: North End",
            "PWDx",
            "",
            "",
            "21-23 Temple St  Boston  MA  02114",
            "3",
            "1B",
            "1",
            "A1",
            "Beacon Hill",
            "3",
            "Ward 3",
            "0306",
            "21-23 Temple St",
            "2114",
            "42.3606",
            "-71.0638",
            "City Worker App"
        ],
        svec![
            "101004141367",
            "2022-01-20 08:15:45",
            "2022-01-21 08:30:00",
            "2022-01-20 08:45:12",
            "ONTIME",
            "Closed",
            "Case Closed. Closed date : Thu Jan 20 08:45:12 EST 2022 Noted ",
            "CE Collection",
            "Public Works Department",
            "Street Cleaning",
            "CE Collection",
            "PWDx_District 1B: North End",
            "PWDx",
            "",
            "",
            "12 Derne St  Boston  MA  02114",
            "3",
            "1B",
            "1",
            "A1",
            "Beacon Hill",
            "3",
            "Ward 3",
            "0306",
            "12 Derne St",
            "2114",
            "42.3596",
            "-71.0634",
            "City Worker App"
        ],
    ];

    assert_eq!(got, expected);
}

#[test]
fn sqlp_string_like_ops() {
    let wrk = Workdir::new("sqlp_string_like_ops");

    wrk.create(
        "likedata.csv",
        vec![
            svec!["x", "y"],
            svec!["aaa", "abc"],
            svec!["bbb", "b"],
            svec!["a", "aa"],
        ],
    );

    let mut cmd = wrk.command("sqlp");
    cmd.arg("likedata.csv").arg(
        r#"
        SELECT
            x,
            x ^@ 'a' AS x_starts_with_a,
            x ~~* '%B' AS x_ends_with_b_case_insensitive,
            x ^@ y AS x_starts_with_y,
            x ~~ '%a' AS x_ends_with_a
        FROM likedata"#,
    );

    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec![
            "x",
            "x_starts_with_a",
            "x_ends_with_b_case_insensitive",
            "x_starts_with_y",
            "x_ends_with_a"
        ],
        svec!["aaa", "true", "false", "false", "true"],
        svec!["bbb", "false", "true", "true", "false"],
        svec!["a", "true", "false", "false", "true"],
    ];

    assert_eq!(got, expected);
}

#[test]
fn sqlp_star_ilike() {
    let wrk = Workdir::new("sqlp_star_ilike");

    wrk.create(
        "starlikedata.csv",
        vec![
            svec!["ID", "FirstName", "LastName", "Address", "City"],
            svec!["333", "Bruce", "Wayne", "The Batcave", "Gotham"],
            svec!["666", "Diana", "Prince", "Paradise Island", "Themyscira"],
            svec!["999", "Clark", "Kent", "Fortress of Solitude", "Metropolis"],
        ],
    );

    let mut cmd = wrk.command("sqlp");
    cmd.arg("starlikedata.csv").arg(
        r#"
        SELECT * ILIKE '%a%e%'
  FROM starlikedata
  ORDER BY FirstName"#,
    );

    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["FirstName", "LastName", "Address"],
        svec!["Bruce", "Wayne", "The Batcave"],
        svec!["Clark", "Kent", "Fortress of Solitude"],
        svec!["Diana", "Prince", "Paradise Island"],
    ];

    assert_eq!(got, expected);

    let mut cmd = wrk.command("sqlp");
    cmd.arg("starlikedata.csv").arg(
        r#"
        SELECT * ILIKE '%I%' RENAME (FirstName AS Name) 
  FROM starlikedata
  ORDER BY 3 DESC"#,
    );

    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["ID", "Name", "City"],
        svec!["666", "Diana", "Themyscira"],
        svec!["999", "Clark", "Metropolis"],
        svec!["333", "Bruce", "Gotham"],
    ];

    assert_eq!(got, expected);

    let mut cmd = wrk.command("sqlp");
    cmd.arg("starlikedata.csv").arg(
        r#"
        SELECT * EXCLUDE (ID, City, LastName) RENAME FirstName AS Name
  FROM starlikedata
  ORDER BY Name"#,
    );

    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["Name", "Address"],
        svec!["Bruce", "The Batcave"],
        svec!["Clark", "Fortress of Solitude"],
        svec!["Diana", "Paradise Island"],
    ];

    assert_eq!(got, expected);
}

#[test]
fn sqlp_skip_input() {
    let wrk = Workdir::new("sqlp_skip_input");

    let mut cmd = wrk.command("sqlp");
    cmd.arg("SKIP_INPUT")
        .arg("SELECT 1 AS one, '2' AS two, 3.0 AS three");

    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["one", "two", "three"], svec!["1", "2", "3.0"]];

    assert_eq!(got, expected);
}

#[test]
fn sqlp_autotab_delim() {
    let wrk = Workdir::new("sqlp_autotab_delim");
    wrk.create_with_delim(
        "cities_array.tsv",
        vec![
            svec!["countryid", "cities"],
            svec!["DB", "Dubai,Abu Dhabi,Sharjah"],
            svec!["IN", "Mumbai,Delhi,Bangalore"],
            svec!["US", "New York,Los Angeles,Chicago"],
            svec!["UK", "London,Birmingham,Manchester"],
            svec!["CN", "Beijing"],
            svec!["RU", "Moscow"],
            svec!["NL", "Amsterdam"],
            svec!["IT", "Rome,Milan,Turin,Naples,Venice"],
        ],
        b'\t',
    );

    let output_file = wrk.path("output.tsv").to_string_lossy().to_string();

    let mut cmd = wrk.command("sqlp");
    cmd.arg("cities_array.tsv")
        .arg(
            r#"
            SELECT 
                countryid, cities
            FROM cities_array
    "#,
        )
        .args(["--output", &output_file]);

    wrk.assert_success(&mut cmd);

    let got = wrk.read_to_string(&output_file);
    let expected = r#"countryid	cities
DB	Dubai,Abu Dhabi,Sharjah
IN	Mumbai,Delhi,Bangalore
US	New York,Los Angeles,Chicago
UK	London,Birmingham,Manchester
CN	Beijing
RU	Moscow
NL	Amsterdam
IT	Rome,Milan,Turin,Naples,Venice
"#;

    assert_eq!(got, expected);
}
