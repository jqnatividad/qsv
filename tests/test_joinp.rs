use crate::workdir::Workdir;

macro_rules! joinp_test {
    ($name:ident, $fun:expr) => {
        mod $name {
            use std::process;

            use super::{make_rows, setup};
            use crate::workdir::Workdir;

            #[test]
            fn headers() {
                let wrk = setup(stringify!($name));
                let mut cmd = wrk.command("joinp");
                cmd.args(&["city", "cities.csv", "city", "places.csv"]);
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

joinp_test!(joinp_inner, |wrk: Workdir, mut cmd: process::Command| {
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
});

joinp_test!(
    joinp_outer_left,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.arg("--left");
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

joinp_test!(joinp_full, |wrk: Workdir, mut cmd: process::Command| {
    cmd.arg("--full");
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
});

joinp_test!(
    joinp_left_semi,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.arg("--left-semi");
        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let expected = make_rows(true, vec![svec!["Boston", "MA"], svec!["Buffalo", "NY"]]);
        assert_eq!(got, expected);
    }
);

joinp_test!(
    joinp_left_anti,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.arg("--left-anti");
        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let expected = make_rows(
            true,
            vec![svec!["New York", "NY"], svec!["San Francisco", "CA"]],
        );
        assert_eq!(got, expected);
    }
);

#[test]
fn joinp_cross() {
    let wrk = Workdir::new("join_cross");
    wrk.create(
        "letters.csv",
        vec![svec!["h1", "h2"], svec!["a", "b"], svec!["c", "d"]],
    );
    wrk.create(
        "numbers.csv",
        vec![svec!["h3", "h4"], svec!["1", "2"], svec!["3", "4"]],
    );

    let mut cmd = wrk.command("joinp");
    cmd.arg("--cross").args(["letters.csv", "numbers.csv"]);

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
fn joinp_asof_date() {
    let wrk = Workdir::new("join_asof_date");
    wrk.create(
        "gdp.csv",
        vec![
            svec!["date", "gdp"],
            svec!["2016-01-01", "4164"],
            svec!["2017-01-01", "4411"],
            svec!["2018-01-01", "4566"],
            svec!["2019-01-01", "4696"],
        ],
    );
    wrk.create(
        "population.csv",
        vec![
            svec!["date", "population"],
            svec!["2016-05-12", "82.19"],
            svec!["2017-05-12", "82.66"],
            svec!["2018-05-12", "83.12"],
            svec!["2019-05-12", "83.52"],
        ],
    );

    let mut cmd = wrk.command("joinp");
    cmd.arg("--asof")
        .args(["date", "population.csv", "date", "gdp.csv"]);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["date", "population", "gdp"],
        svec!["2016-05-12", "82.19", "4164"],
        svec!["2017-05-12", "82.66", "4411"],
        svec!["2018-05-12", "83.12", "4566"],
        svec!["2019-05-12", "83.52", "4696"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn joinp_asof_nearest_date() {
    let wrk = Workdir::new("join_asof_nearest_date");
    wrk.create(
        "gdp.csv",
        vec![
            svec!["date", "gdp"],
            svec!["2016-01-01", "4164"],
            svec!["2017-01-22", "4422"],
            svec!["2017-01-10", "4410"],
            svec!["2018-01-01", "4501"],
            svec!["2018-01-05", "4505"],
            svec!["2018-01-14", "4514"],
            svec!["2019-01-01", "4696"],
        ],
    );
    wrk.create(
        "population.csv",
        vec![
            svec!["date", "population"],
            svec!["2016-05-12", "82.19"],
            svec!["2017-05-12", "82.66"],
            svec!["2018-05-12", "83.12"],
            svec!["2019-05-12", "83.52"],
        ],
    );

    let mut cmd = wrk.command("joinp");
    cmd.arg("--asof").args(["--strategy", "nearest"]).args([
        "date",
        "population.csv",
        "date",
        "gdp.csv",
    ]);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["date", "population", "gdp"],
        svec!["2016-05-12", "82.19", "4164"],
        svec!["2017-05-12", "82.66", "4422"],
        svec!["2018-05-12", "83.12", "4514"],
        svec!["2019-05-12", "83.52", "4696"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn joinp_asof_date_diffcolnames() {
    let wrk = Workdir::new("join_asof_date_diffcolnames");
    wrk.create(
        "gdp.csv",
        vec![
            svec!["gdp_date", "gdp"],
            svec!["2016-01-01", "4164"],
            svec!["2017-01-01", "4411"],
            svec!["2018-01-01", "4566"],
            svec!["2019-01-01", "4696"],
        ],
    );
    wrk.create(
        "population.csv",
        vec![
            svec!["pop_date", "population"],
            svec!["2016-05-12", "82.19"],
            svec!["2017-05-12", "82.66"],
            svec!["2018-05-12", "83.12"],
            svec!["2019-05-12", "83.52"],
        ],
    );

    let mut cmd = wrk.command("joinp");
    cmd.arg("--asof")
        .args(["pop_date", "population.csv", "gdp_date", "gdp.csv"]);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["pop_date", "population", "gdp_date", "gdp"],
        svec!["2016-05-12", "82.19", "2016-01-01", "4164"],
        svec!["2017-05-12", "82.66", "2017-01-01", "4411"],
        svec!["2018-05-12", "83.12", "2018-01-01", "4566"],
        svec!["2019-05-12", "83.52", "2019-01-01", "4696"],
    ];
    assert_eq!(got, expected);
}
