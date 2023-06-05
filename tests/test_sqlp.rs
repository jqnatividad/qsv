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
fn sqlp_boston311_try_parsedates() {
    let wrk = Workdir::new("sqlp_boston311_try_parsedates");
    let test_file = wrk.load_test_file("boston311-100.csv");

    let mut cmd = wrk.command("sqlp");
    cmd.arg(&test_file).arg("--try-parsedates").arg(
        "select ward, cast(avg(closed_dt - open_dt) as float) as avg_tat from _t_1 where \
         case_status = 'Closed' group by ward order by avg_tat desc, ward asc",
    );

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["ward", "avg_tat"],
        svec!["Ward 11", "4.84776e12"],
        svec!["01", "4.81827e12"],
        svec!["Ward 13", "1.5183657e12"],
        svec!["Ward 15", "1.278926e12"],
        svec!["Ward 21", "8.78446e11"],
        svec!["Ward 14", "6.18933e11"],
        svec!["Ward 3", "4.3769145e11"],
        svec!["Ward 5", "4.119095e11"],
        svec!["Ward 20", "3.67233e11"],
        svec!["9", "3.53495e11"],
        svec!["Ward 18", "2.49882e11"],
        svec!["19", "2.12566e11"],
        svec!["Ward 4", "1.128726e11"],
        svec!["Ward 1", "1.0785067e11"],
        svec!["Ward 10", "1.0411e11"],
        svec!["16", "9.3557e10"],
        svec!["Ward 19", "8.4164e10"],
        svec!["10", "7.9101e10"],
        svec!["21", "7.7717e10"],
        svec!["7", "7.4611e10"],
        svec!["17", "7.01175e10"],
        svec!["3", "6.88366e10"],
        svec!["Ward 9", "6.4097e10"],
        svec!["Ward 12", "6.293e10"],
        svec!["Ward 6", "5.4770168e10"],
        svec!["Ward 7", "3.8346334e10"],
        svec!["Ward 8", "3.27675e10"],
        svec!["03", "2.98105e10"],
        svec!["07", "2.5328001e10"],
        svec!["22", "2.3919e10"],
        svec!["14", "2.07865e10"],
        svec!["Ward 22", "1.3524e10"],
        svec!["1", "9469000000.0"],
        svec!["06", "5290000000.0"],
        svec!["Ward 16", "4533667000.0"],
        svec!["8", "1757000000.0"],
        svec!["02", "1650000000.0"],
        svec!["18", "507000000.0"],
    ];
    assert_eq!(got, expected);
}
