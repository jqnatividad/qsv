use crate::workdir::Workdir;

macro_rules! joinp_test {
    ($name:ident, $fun:expr) => {
        mod $name {
            use std::process;

            #[allow(unused_imports)]
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

macro_rules! joinp_test_comments {
    ($name2:ident, $fun:expr) => {
        mod $name2 {
            use std::process;

            #[allow(unused_imports)]
            use super::{make_rows, setup};
            use crate::workdir::Workdir;

            #[test]
            fn headers() {
                let wrk = setup(stringify!($name2));
                let mut cmd = wrk.command("joinp");
                cmd.args(&["city", "cities_comments.csv", "city", "places.csv"]);
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
    let cities_comments = vec![
        svec!["#this is a comment", ""],
        svec!["city", "state"],
        svec!["Boston", "MA"],
        svec!["New York", "NY"],
        svec!["#Washington", "DC"],
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
    wrk.create("cities_comments.csv", cities_comments);
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

joinp_test_comments!(
    joinp_inner_comments,
    |wrk: Workdir, mut cmd: process::Command| {
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

joinp_test_comments!(
    joinp_outer_left_comments,
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

joinp_test!(
    joinp_outer_left_filter_left,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.arg("--left").args(["--filter-left", "city = 'Boston'"]);
        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let expected = make_rows(
            false,
            vec![
                svec!["Boston", "MA", "Logan Airport"],
                svec!["Boston", "MA", "Boston Garden"],
            ],
        );
        assert_eq!(got, expected);
    }
);

joinp_test_comments!(
    joinp_outer_left_filter_left_comments,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.arg("--left").args(["--filter-left", "city = 'Boston'"]);
        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let expected = make_rows(
            false,
            vec![
                svec!["Boston", "MA", "Logan Airport"],
                svec!["Boston", "MA", "Boston Garden"],
            ],
        );
        assert_eq!(got, expected);
    }
);

joinp_test!(
    joinp_inner_filter_right,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.args(["--filter-right", "place ~* 'w'"]);
        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let expected = make_rows(false, vec![svec!["Buffalo", "NY", "Ralph Wilson Stadium"]]);
        assert_eq!(got, expected);
    }
);

joinp_test_comments!(
    joinp_inner_filter_right_comments,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.args(["--filter-right", "place ~* 'w'"]);
        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let expected = make_rows(false, vec![svec!["Buffalo", "NY", "Ralph Wilson Stadium"]]);
        assert_eq!(got, expected);
    }
);

joinp_test!(
    joinp_outer_left_validate_none,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.arg("--left").args(["--validate", "none"]);
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

joinp_test_comments!(
    joinp_outer_left_validate_none_comments,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.arg("--left").args(["--validate", "none"]);
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

// joinp_test!(
//     joinp_outer_left_validate_manytoone,
//     |wrk: Workdir, mut cmd: process::Command| {
//         cmd.arg("--left").args(["--validate", "onetomany"]).arg("--low-memory");
//         let got: String = wrk.output_stderr(&mut cmd);
//         assert_eq!(
//             got,
//             "Polars error: ComputeError(ErrString(\"the join keys did not fulfil m:1 \
//              validation\"))\n"
//         );
//         wrk.assert_err(&mut cmd);
//     }
// );

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

joinp_test_comments!(
    joinp_full_comments,
    |wrk: Workdir, mut cmd: process::Command| {
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
    }
);

joinp_test!(
    joinp_left_semi,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.arg("--left-semi");
        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let expected = make_rows(true, vec![svec!["Boston", "MA"], svec!["Buffalo", "NY"]]);
        assert_eq!(got, expected);
    }
);

joinp_test_comments!(
    joinp_left_semi_comments,
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

joinp_test_comments!(
    joinp_left_anti_comments,
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
fn joinp_asof_dat_comments() {
    let wrk = Workdir::new("join_asof_date_comments");
    wrk.create(
        "gdp.csv",
        vec![
            svec!["#comment", "here"],
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
            svec!["#comment", "in the middle"],
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
fn joinp_asofby_1() {
    let wrk = Workdir::new("join_asofby_timeseries");
    wrk.create(
        "trades.csv",
        vec![
            svec!["time", "ticker", "groups_numeric", "bid"],
            svec!["2016-01-01 12:23:00", "MSFT", "1", "51.95"],
            svec!["2016-01-01 12:38:00", "MSFT", "1", "51.95"],
            svec!["2016-01-01 12:48:00", "GOOG", "2", "720.77"],
            svec!["2016-01-01 12:48:00", "GOOG", "2", "720.92"],
            svec!["2016-01-01 12:48:00", "AAPL", "3", "98.0"],
        ],
    );
    wrk.create(
        "quotes.csv",
        vec![
            svec!["time", "ticker", "groups_numeric", "bid"],
            svec!["2016-01-01 12:23:00", "GOOG", "2", "720.50"],
            svec!["2016-01-01 12:23:00", "MSFT", "1", "51.95"],
            svec!["2016-01-01 12:30:00", "MSFT", "1", "51.97"],
            svec!["2016-01-01 12:41:00", "MSFT", "1", "51.99"],
            svec!["2016-01-01 12:48:00", "GOOG", "2", "720.50"],
            svec!["2016-01-01 12:49:00", "AAPL", "3", "97.99"],
            svec!["2016-01-01 12:52:00", "GOOG", "2", "720.50"],
            svec!["2016-01-01 12:55:00", "MSFT", "1", "52.01"],
        ],
    );

    let mut cmd = wrk.command("joinp");
    cmd.arg("--asof")
        .args(["time", "trades.csv", "time", "quotes.csv"])
        .args(["--left_by", "ticker"])
        .args(["--right_by", "ticker"])
        .args(["--datetime-format", "%Y-%m-%d %H:%M"]);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec![
            "time",
            "ticker",
            "groups_numeric",
            "bid",
            "ticker_right",
            "groups_numeric_right",
            "bid_right"
        ],
        svec![
            "2016-01-01 12:23",
            "MSFT",
            "1",
            "51.95",
            "MSFT",
            "1",
            "51.95"
        ],
        svec![
            "2016-01-01 12:38",
            "MSFT",
            "1",
            "51.95",
            "MSFT",
            "1",
            "51.97"
        ],
        svec![
            "2016-01-01 12:48",
            "GOOG",
            "2",
            "720.77",
            "GOOG",
            "2",
            "720.5"
        ],
        svec![
            "2016-01-01 12:48",
            "GOOG",
            "2",
            "720.92",
            "GOOG",
            "2",
            "720.5"
        ],
        svec![
            "2016-01-01 12:48",
            "AAPL",
            "3",
            "98.0",
            "GOOG",
            "2",
            "720.5"
        ],
    ];
    assert_eq!(got, expected);
}

#[test]
fn joinp_asofby_left_place_date() {
    let wrk = Workdir::new("join_asofby_left_place_date");
    wrk.create(
        "gdp.csv",
        vec![
            svec!["date", "gdp", "place"],
            svec!["2016-01-01", "4164", "US"],
            svec!["2017-01-01", "4411", "US"],
            svec!["2018-01-01", "4566", "Asia"],
            svec!["2019-01-01", "4696", "EU"],
        ],
    );
    wrk.create(
        "population.csv",
        vec![
            svec!["date", "population", "place"],
            svec!["2016-05-12", "82.19", "US"],
            svec!["2017-05-12", "82.66", "US"],
            svec!["2018-05-12", "83.12", "Asia"],
            svec!["2018-05-12", "84.12", "Asia"],
            svec!["2019-05-12", "83.52", "EU"],
        ],
    );

    let mut cmd = wrk.command("joinp");
    cmd.arg("--asof")
        .args(["date", "population.csv", "date", "gdp.csv"])
        .args(["--left_by", "place"]);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["date", "population", "place", "gdp", "place_right"],
        svec!["2016-05-12", "82.19", "US", "4164", "US"],
        svec!["2017-05-12", "82.66", "US", "4411", "US"],
        svec!["2018-05-12", "83.12", "Asia", "4566", "Asia"],
        svec!["2018-05-12", "84.12", "Asia", "4566", "Asia"],
        svec!["2019-05-12", "83.52", "EU", "4696", "EU"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn joinp_asofby_right_place_date() {
    let wrk = Workdir::new("join_asofby_right_place_date");
    wrk.create(
        "gdp.csv",
        vec![
            svec!["date", "gdp", "place"],
            svec!["2016-01-01", "4164", "US"],
            svec!["2017-01-01", "4411", "US"],
            svec!["2018-01-01", "4566", "Asia"],
            svec!["2019-01-01", "4696", "EU"],
        ],
    );
    wrk.create(
        "population.csv",
        vec![
            svec!["date", "population", "place"],
            svec!["2016-05-12", "82.19", "US"],
            svec!["2017-05-12", "82.66", "US"],
            svec!["2018-05-12", "83.12", "Asia"],
            svec!["2018-05-12", "84.12", "Asia"],
            svec!["2019-05-12", "83.52", "EU"],
        ],
    );

    let mut cmd = wrk.command("joinp");
    cmd.arg("--asof")
        .args(["date", "population.csv", "date", "gdp.csv"])
        .args(["--right_by", "place"]);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["date", "population", "place", "gdp", "place_right"],
        svec!["2016-05-12", "82.19", "US", "4164", "US"],
        svec!["2017-05-12", "82.66", "US", "4411", "US"],
        svec!["2018-05-12", "83.12", "Asia", "4566", "Asia"],
        svec!["2018-05-12", "84.12", "Asia", "4566", "Asia"],
        svec!["2019-05-12", "83.52", "EU", "4696", "EU"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn joinp_asofby_leftright_place_date() {
    let wrk = Workdir::new("join_asofby_leftright_place_date");
    wrk.create(
        "gdp.csv",
        vec![
            svec!["date", "gdp", "place"],
            svec!["2016-01-01", "4164", "US"],
            svec!["2017-01-01", "4411", "US"],
            svec!["2018-01-01", "4566", "Asia"],
            svec!["2019-01-01", "4696", "EU"],
        ],
    );
    wrk.create(
        "population.csv",
        vec![
            svec!["date", "population", "other_place"],
            svec!["2016-05-12", "82.19", "US"],
            svec!["2017-05-12", "82.66", "US"],
            svec!["2018-05-12", "83.12", "Asia"],
            svec!["2018-05-12", "84.12", "Asia"],
            svec!["2019-05-12", "83.52", "EU"],
        ],
    );

    let mut cmd = wrk.command("joinp");
    cmd.arg("--asof")
        .args(["date", "population.csv", "date", "gdp.csv"])
        .args(["--left_by", "place"])
        .args(["--right_by", "other_place"]);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["date", "population", "other_place", "gdp", "place"],
        svec!["2016-05-12", "82.19", "US", "4164", "US"],
        svec!["2017-05-12", "82.66", "US", "4411", "US"],
        svec!["2018-05-12", "83.12", "Asia", "4566", "Asia"],
        svec!["2018-05-12", "84.12", "Asia", "4566", "Asia"],
        svec!["2019-05-12", "83.52", "EU", "4696", "EU"],
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

#[test]
fn joinp_asof_date_diffcolnames_sqlfilter() {
    let wrk = Workdir::new("join_asof_date_diffcolnames_sqlfilter");
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
        .args(["pop_date", "population.csv", "gdp_date", "gdp.csv"])
        .args([
            "--sql-filter",
            "select pop_date, gdp from join_result where gdp > 4500",
        ]);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["pop_date", "gdp"],
        svec!["2018-05-12", "4566"],
        svec!["2019-05-12", "4696"],
    ];
    assert_eq!(got, expected);
}
