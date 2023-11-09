use std::process;

use crate::{qcheck, quickcheck::TestResult, workdir::Workdir, Csv, CsvData};

fn no_headers(cmd: &mut process::Command) {
    cmd.arg("--no-headers");
}

fn pad(cmd: &mut process::Command) {
    cmd.arg("--pad");
}

fn run_cat<X, Y, Z, F>(test_name: &str, which: &str, rows1: X, rows2: Y, modify_cmd: F) -> Z
where
    X: Csv,
    Y: Csv,
    Z: Csv,
    F: FnOnce(&mut process::Command),
{
    let wrk = Workdir::new(test_name);
    wrk.create("in1.csv", rows1);
    wrk.create("in2.csv", rows2);

    let mut cmd = wrk.command("cat");
    modify_cmd(cmd.arg(which).arg("in1.csv").arg("in2.csv"));
    wrk.read_stdout(&mut cmd)
}

#[test]
fn prop_cat_rows() {
    fn p(rows: CsvData) -> bool {
        let expected = rows.clone();
        let (rows1, rows2) = if rows.is_empty() {
            (vec![], vec![])
        } else {
            let (rows1, rows2) = rows.split_at(rows.len() / 2);
            (rows1.to_vec(), rows2.to_vec())
        };
        let got: CsvData = run_cat("cat_rows", "rows", rows1, rows2, no_headers);
        rassert_eq!(got, expected)
    }
    qcheck(p as fn(CsvData) -> bool);
}

#[test]
fn cat_rows_space() {
    let rows = vec![svec!["\u{0085}"]];
    let expected = rows.clone();
    let (rows1, rows2) = if rows.is_empty() {
        (vec![], vec![])
    } else {
        let (rows1, rows2) = rows.split_at(rows.len() / 2);
        (rows1.to_vec(), rows2.to_vec())
    };
    let got: Vec<Vec<String>> = run_cat("cat_rows_space", "rows", rows1, rows2, no_headers);
    assert_eq!(got, expected);
}

#[test]
fn cat_rows_headers() {
    let rows1 = vec![svec!["h1", "h2"], svec!["a", "b"]];
    let rows2 = vec![svec!["h1", "h2"], svec!["y", "z"]];

    let mut expected = rows1.clone();
    expected.extend(rows2.clone().into_iter().skip(1));

    let got: Vec<Vec<String>> = run_cat("cat_rows_headers", "rows", rows1, rows2, |_| ());
    assert_eq!(got, expected);
}

#[test]
fn cat_rowskey() {
    let wrk = Workdir::new("cat_rowskey");
    wrk.create(
        "in1.csv",
        vec![
            svec!["a", "b", "c"],
            svec!["1", "2", "3"],
            svec!["2", "3", "4"],
        ],
    );

    wrk.create(
        "in2.csv",
        vec![
            svec!["c", "a", "b"],
            svec!["3", "1", "2"],
            svec!["4", "2", "3"],
        ],
    );

    wrk.create(
        "in3.csv",
        vec![
            svec!["a", "b", "d", "c"],
            svec!["1", "2", "4", "3"],
            svec!["2", "3", "5", "4"],
            svec!["z", "y", "w", "x"],
        ],
    );

    let mut cmd = wrk.command("cat");
    cmd.arg("rowskey")
        .arg("in1.csv")
        .arg("in2.csv")
        .arg("in3.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["a", "b", "c", "d"],
        svec!["1", "2", "3", ""],
        svec!["2", "3", "4", ""],
        svec!["1", "2", "3", ""],
        svec!["2", "3", "4", ""],
        svec!["1", "2", "3", "4"],
        svec!["2", "3", "4", "5"],
        svec!["z", "y", "x", "w"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn cat_rows_flexible() {
    let wrk = Workdir::new("cat_rows_flexible");
    wrk.create(
        "in1.csv",
        vec![
            svec!["a", "b", "c"],
            svec!["1", "2", "3"],
            svec!["2", "3", "4"],
        ],
    );

    wrk.create(
        "in2.csv",
        vec![
            svec!["a", "b", "c"],
            svec!["3", "1", "2"],
            svec!["4", "2", "3"],
        ],
    );

    wrk.create(
        "in3.csv",
        vec![
            svec!["a", "b", "c", "d"],
            svec!["1", "2", "4", "3"],
            svec!["2", "3", "5", "4"],
            svec!["z", "y", "w", "x"],
        ],
    );

    let mut cmd = wrk.command("cat");
    cmd.arg("rows")
        .arg("--flexible")
        .arg("in1.csv")
        .arg("in2.csv")
        .arg("in3.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["a", "b", "c"],
        svec!["1", "2", "3"],
        svec!["2", "3", "4"],
        svec!["3", "1", "2"],
        svec!["4", "2", "3"],
        svec!["1", "2", "4", "3"],
        svec!["2", "3", "5", "4"],
        svec!["z", "y", "w", "x"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn cat_rowskey_grouping() {
    let wrk = Workdir::new("cat_rowskey_grouping");
    wrk.create(
        "in1.csv",
        vec![
            svec!["a", "b", "c"],
            svec!["1", "2", "3"],
            svec!["2", "3", "4"],
        ],
    );

    wrk.create(
        "in2.csv",
        vec![
            svec!["c", "a", "b"],
            svec!["3", "1", "2"],
            svec!["4", "2", "3"],
        ],
    );

    wrk.create(
        "in3.csv",
        vec![
            svec!["a", "b", "d", "c"],
            svec!["1", "2", "4", "3"],
            svec!["2", "3", "5", "4"],
            svec!["z", "y", "w", "x"],
        ],
    );

    let mut cmd = wrk.command("cat");
    cmd.arg("rowskey")
        .arg("--group")
        .arg("in1.csv")
        .arg("in2.csv")
        .arg("in3.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["file", "a", "b", "c", "d"],
        svec!["in1", "1", "2", "3", ""],
        svec!["in1", "2", "3", "4", ""],
        svec!["in2", "1", "2", "3", ""],
        svec!["in2", "2", "3", "4", ""],
        svec!["in3", "1", "2", "3", "4"],
        svec!["in3", "2", "3", "4", "5"],
        svec!["in3", "z", "y", "x", "w"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn cat_rowskey_grouping_customname() {
    let wrk = Workdir::new("cat_rowskey_grouping_customname");
    wrk.create(
        "in1.csv",
        vec![
            svec!["a", "b", "c"],
            svec!["1", "2", "3"],
            svec!["2", "3", "4"],
        ],
    );

    wrk.create(
        "in2.csv",
        vec![
            svec!["c", "a", "b"],
            svec!["3", "1", "2"],
            svec!["4", "2", "3"],
        ],
    );

    wrk.create(
        "in3.csv",
        vec![
            svec!["a", "b", "d", "c"],
            svec!["1", "2", "4", "3"],
            svec!["2", "3", "5", "4"],
            svec!["z", "y", "w", "x"],
        ],
    );

    let mut cmd = wrk.command("cat");
    cmd.arg("rowskey")
        .arg("--group")
        .args(&["--group-name", "file group label"])
        .arg("in1.csv")
        .arg("in2.csv")
        .arg("in3.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["file group label", "a", "b", "c", "d"],
        svec!["in1", "1", "2", "3", ""],
        svec!["in1", "2", "3", "4", ""],
        svec!["in2", "1", "2", "3", ""],
        svec!["in2", "2", "3", "4", ""],
        svec!["in3", "1", "2", "3", "4"],
        svec!["in3", "2", "3", "4", "5"],
        svec!["in3", "z", "y", "x", "w"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn cat_rowskey_insertion_order() {
    let wrk = Workdir::new("cat_rowskey_insertion_order");
    wrk.create(
        "in1.csv",
        vec![
            svec!["j", "b", "c"],
            svec!["1", "2", "3"],
            svec!["2", "3", "4"],
        ],
    );

    wrk.create(
        "in2.csv",
        vec![
            svec!["c", "j", "b"],
            svec!["3", "1", "2"],
            svec!["4", "2", "3"],
        ],
    );

    wrk.create(
        "in3.csv",
        vec![
            svec!["j", "b", "d", "c"],
            svec!["1", "2", "4", "3"],
            svec!["2", "3", "5", "4"],
            svec!["z", "y", "w", "x"],
        ],
    );

    let mut cmd = wrk.command("cat");
    cmd.arg("rowskey")
        .arg("in1.csv")
        .arg("in2.csv")
        .arg("in3.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["j", "b", "c", "d"],
        svec!["1", "2", "3", ""],
        svec!["2", "3", "4", ""],
        svec!["1", "2", "3", ""],
        svec!["2", "3", "4", ""],
        svec!["1", "2", "3", "4"],
        svec!["2", "3", "4", "5"],
        svec!["z", "y", "x", "w"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn prop_cat_cols() {
    fn p(rows1: CsvData, rows2: CsvData) -> TestResult {
        let got: Vec<Vec<String>> = run_cat(
            "cat_cols",
            "columns",
            rows1.clone(),
            rows2.clone(),
            no_headers,
        );

        let mut expected: Vec<Vec<String>> = vec![];
        let (rows1, rows2) = (rows1.to_vecs().into_iter(), rows2.to_vecs().into_iter());
        for (mut r1, r2) in rows1.zip(rows2) {
            r1.extend(r2.into_iter());
            expected.push(r1);
        }
        assert_eq!(got, expected);
        TestResult::passed()
    }
    qcheck(p as fn(CsvData, CsvData) -> TestResult);
}

#[test]
fn cat_cols_headers() {
    let rows1 = vec![svec!["h1", "h2"], svec!["a", "b"]];
    let rows2 = vec![svec!["h3", "h4"], svec!["y", "z"]];

    let expected = vec![svec!["h1", "h2", "h3", "h4"], svec!["a", "b", "y", "z"]];
    let got: Vec<Vec<String>> = run_cat("cat_cols_headers", "columns", rows1, rows2, |_| ());
    assert_eq!(got, expected);
}

#[test]
fn cat_cols_no_pad() {
    let rows1 = vec![svec!["a", "b"]];
    let rows2 = vec![svec!["y", "z"], svec!["y", "z"]];

    let expected = vec![svec!["a", "b", "y", "z"]];
    let got: Vec<Vec<String>> = run_cat("cat_cols_headers", "columns", rows1, rows2, no_headers);
    assert_eq!(got, expected);
}

#[test]
fn cat_cols_pad() {
    let rows1 = vec![svec!["a", "b"]];
    let rows2 = vec![svec!["y", "z"], svec!["y", "z"]];

    let expected = vec![svec!["a", "b", "y", "z"], svec!["", "", "y", "z"]];
    let got: Vec<Vec<String>> = run_cat("cat_cols_headers", "columns", rows1, rows2, pad);
    assert_eq!(got, expected);
}
