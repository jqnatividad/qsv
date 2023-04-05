use crate::workdir::Workdir;

// This macro takes *two* identifiers: one for the test with headers
// and another for the test without headers.
macro_rules! exclude_test {
    ($name:ident, $fun:expr) => {
        mod $name {
            use std::process;

            use super::{make_rows, setup};
            use crate::workdir::Workdir;

            #[test]
            fn headers() {
                let wrk = setup(stringify!($name), true);
                let mut cmd = wrk.command("exclude");
                cmd.args(&["city", "cities.csv", "city", "places.csv"]);
                $fun(wrk, cmd, true);
            }

            #[test]
            fn no_headers() {
                let n = stringify!(concat_idents!($name, _no_headers));
                let wrk = setup(n, false);
                let mut cmd = wrk.command("exclude");
                cmd.arg("--no-headers");
                cmd.args(&["1", "cities.csv", "1", "places.csv"]);
                $fun(wrk, cmd, false);
            }
        }
    };
}

fn setup(name: &str, headers: bool) -> Workdir {
    let mut cities = vec![
        svec!["Boston", "MA"],
        svec!["New York", "NY"],
        svec!["San Francisco", "CA"],
        svec!["Buffalo", "NY"],
    ];
    let mut places = vec![
        svec!["Boston", "Logan Airport"],
        svec!["Boston", "Boston Garden"],
        svec!["Buffalo", "Ralph Wilson Stadium"],
        svec!["Orlando", "Disney World"],
    ];
    if headers {
        cities.insert(0, svec!["city", "state"]);
    }
    if headers {
        places.insert(0, svec!["city", "place"]);
    }

    let wrk = Workdir::new(name);
    wrk.create("cities.csv", cities);
    wrk.create("places.csv", places);
    wrk
}

fn make_rows(headers: bool, rows: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let mut all_rows = vec![];
    if headers {
        all_rows.push(svec!["city", "state"]);
    }
    all_rows.extend(rows.into_iter());
    all_rows
}

exclude_test!(exclude, |wrk: Workdir,
                        mut cmd: process::Command,
                        headers: bool| {
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = make_rows(
        headers,
        vec![svec!["New York", "NY"], svec!["San Francisco", "CA"]],
    );
    assert_eq!(got, expected);
});

exclude_test!(include, |wrk: Workdir,
                        mut cmd: process::Command,
                        headers: bool| {
    cmd.arg("-v");
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = make_rows(headers, vec![svec!["Boston", "MA"], svec!["Buffalo", "NY"]]);
    assert_eq!(got, expected);
});

#[test]
fn exclude_utf8_issue778_aliases_posiions() {
    let wrk = Workdir::new("exclude_utf8_issue778_aliases_posiions");
    let aliases_file = wrk.load_test_file("aliases.csv");
    let positions_file = wrk.load_test_file("positions.csv");

    let mut cmd = wrk.command("exclude");
    cmd.arg("position")
        .arg(aliases_file)
        .arg("position")
        .arg(positions_file);

    let got: String = wrk.stdout(&mut cmd);
    let expected = wrk.load_test_resource("exclude/aliases-positions-expected.csv");

    assert_eq!(got, expected.trim_end());
}

#[test]
fn exclude_utf8_issue778_positions_aliases() {
    let wrk = Workdir::new("exclude_utf8_issue778_aliases_posiions_aliases");
    let aliases_file = wrk.load_test_file("aliases.csv");
    let positions_file = wrk.load_test_file("positions.csv");

    let mut cmd = wrk.command("exclude");
    cmd.arg("position")
        .arg(positions_file)
        .arg("position")
        .arg(aliases_file);

    let got: String = wrk.stdout(&mut cmd);
    let expected = wrk.load_test_resource("exclude/positions-aliases-expected.csv");

    assert_eq!(got, expected.trim_end());
}
