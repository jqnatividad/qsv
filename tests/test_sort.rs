use std::cmp;

use crate::{qcheck, workdir::Workdir, Csv, CsvData};

fn prop_sort(name: &str, rows: CsvData, headers: bool, faster: bool) -> bool {
    let wrk = Workdir::new(name);
    wrk.create("in.csv", rows.clone());

    let mut cmd = wrk.command("sort");
    cmd.arg("in.csv");
    if !headers {
        cmd.arg("--no-headers");
    }

    if faster {
        cmd.arg("--faster");
    }

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let mut expected = rows.to_vecs();
    let headers = if headers && !expected.is_empty() {
        expected.remove(0)
    } else {
        vec![]
    };
    expected.sort_by(|r1, r2| iter_cmp(r1.iter(), r2.iter()));
    if !headers.is_empty() {
        expected.insert(0, headers);
    }
    rassert_eq!(got, expected)
}

#[test]
fn prop_sort_headers() {
    fn p(rows: CsvData) -> bool {
        prop_sort("prop_sort_headers", rows, true, false)
    }
    qcheck(p as fn(CsvData) -> bool);
}

#[test]
fn prop_sort_headers_faster() {
    fn p(rows: CsvData) -> bool {
        prop_sort("prop_sort_headers", rows, true, true)
    }
    qcheck(p as fn(CsvData) -> bool);
}

#[test]
fn prop_sort_no_headers() {
    fn p(rows: CsvData) -> bool {
        prop_sort("prop_sort_no_headers", rows, false, false)
    }
    qcheck(p as fn(CsvData) -> bool);
}

#[test]
fn prop_sort_no_headers_faster() {
    fn p(rows: CsvData) -> bool {
        prop_sort("prop_sort_no_headers", rows, false, true)
    }
    qcheck(p as fn(CsvData) -> bool);
}

#[test]
fn sort_select() {
    let wrk = Workdir::new("sort_select");
    wrk.create("in.csv", vec![svec!["1", "b"], svec!["2", "a"]]);

    let mut cmd = wrk.command("sort");
    cmd.arg("--no-headers")
        .args(["--select", "2"])
        .arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["2", "a"], svec!["1", "b"]];
    assert_eq!(got, expected);
}

#[test]
fn sort_numeric() {
    let wrk = Workdir::new("sort_numeric");
    wrk.create(
        "in.csv",
        vec![
            svec!["N", "S"],
            svec!["10", "a"],
            svec!["LETTER", "b"],
            svec!["2", "c"],
            svec!["1", "d"],
        ],
    );

    let mut cmd = wrk.command("sort");
    cmd.arg("-N").arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected =
        vec![
            svec!["N", "S"],
            //Non-numerics should be put first
            svec!["LETTER", "b"],
            svec!["1", "d"],
            svec!["2", "c"],
            svec!["10", "a"],
        ];
    assert_eq!(got, expected);
}

#[test]
fn sort_numeric_faster() {
    let wrk = Workdir::new("sort_numeric_faster");
    wrk.create(
        "in.csv",
        vec![
            svec!["N", "S"],
            svec!["10", "a"],
            svec!["LETTER", "b"],
            svec!["2", "c"],
            svec!["1", "d"],
        ],
    );

    let mut cmd = wrk.command("sort");
    cmd.arg("-N").arg("--faster").arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected =
        vec![
            svec!["N", "S"],
            //Non-numerics should be put first
            svec!["LETTER", "b"],
            svec!["1", "d"],
            svec!["2", "c"],
            svec!["10", "a"],
        ];
    assert_eq!(got, expected);
}

#[test]
fn sort_numeric_non_natural() {
    let wrk = Workdir::new("sort_numeric_non_natural");
    wrk.create(
        "in.csv",
        vec![
            svec!["N", "S"],
            svec!["8.33", "a"],
            svec!["5", "b"],
            svec!["LETTER", "c"],
            svec!["7.4", "d"],
            svec!["3.33", "e"],
        ],
    );

    let mut cmd = wrk.command("sort");
    cmd.arg("-N").arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["N", "S"],
        //Non-numerics should be put first
        svec!["LETTER", "c"],
        svec!["3.33", "e"],
        svec!["5", "b"],
        svec!["7.4", "d"],
        svec!["8.33", "a"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sort_numeric_non_natural_faster() {
    let wrk = Workdir::new("sort_numeric_non_natural_faster");
    wrk.create(
        "in.csv",
        vec![
            svec!["N", "S"],
            svec!["8.33", "a"],
            svec!["5", "b"],
            svec!["LETTER", "c"],
            svec!["7.4", "d"],
            svec!["3.33", "e"],
        ],
    );

    let mut cmd = wrk.command("sort");
    cmd.arg("-N").arg("--faster").arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["N", "S"],
        //Non-numerics should be put first
        svec!["LETTER", "c"],
        svec!["3.33", "e"],
        svec!["5", "b"],
        svec!["7.4", "d"],
        svec!["8.33", "a"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sort_case_insensitive() {
    let wrk = Workdir::new("sort_case_insensitive");
    wrk.create(
        "in.csv",
        vec![
            svec!["col1", "col2"],
            svec!["n", "s"],
            svec!["Alpha", "baBa"],
            svec!["aLPHA", "BABA"],
            svec!["N", "S"],
            svec!["n", "S"],
        ],
    );

    let mut cmd = wrk.command("sort");
    cmd.arg("--ignore-case").arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["col1", "col2"],
        svec!["Alpha", "baBa"],
        svec!["aLPHA", "BABA"],
        svec!["n", "s"],
        svec!["N", "S"],
        svec!["n", "S"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sort_case_insensitive_faster() {
    let wrk = Workdir::new("sort_case_insensitive_faster");
    wrk.create(
        "in.csv",
        vec![
            svec!["col1", "col2"],
            svec!["n", "s"],
            svec!["Alpha", "baBa"],
            svec!["aLPHA", "BABA"],
            svec!["N", "S"],
            svec!["n", "S"],
        ],
    );

    let mut cmd = wrk.command("sort");
    cmd.arg("--ignore-case").arg("--faster").arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["col1", "col2"],
        svec!["Alpha", "baBa"],
        svec!["aLPHA", "BABA"],
        svec!["n", "s"],
        svec!["N", "S"],
        svec!["n", "S"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sort_case_sensitive() {
    let wrk = Workdir::new("sort_case_sensitive");
    wrk.create(
        "in.csv",
        vec![
            svec!["col1", "col2"],
            svec!["n", "s"],
            svec!["Alpha", "baBa"],
            svec!["aLPHA", "BABA"],
            svec!["N", "S"],
            svec!["n", "S"],
        ],
    );

    let mut cmd = wrk.command("sort");
    cmd.arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["col1", "col2"],
        svec!["Alpha", "baBa"],
        svec!["N", "S"],
        svec!["aLPHA", "BABA"],
        svec!["n", "S"],
        svec!["n", "s"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sort_case_sensitive_faster() {
    let wrk = Workdir::new("sort_case_sensitive_faster");
    wrk.create(
        "in.csv",
        vec![
            svec!["col1", "col2"],
            svec!["n", "s"],
            svec!["Alpha", "baBa"],
            svec!["aLPHA", "BABA"],
            svec!["N", "S"],
            svec!["n", "S"],
        ],
    );

    let mut cmd = wrk.command("sort");
    cmd.arg("--faster").arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["col1", "col2"],
        svec!["Alpha", "baBa"],
        svec!["N", "S"],
        svec!["aLPHA", "BABA"],
        svec!["n", "S"],
        svec!["n", "s"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sort_reverse() {
    let wrk = Workdir::new("sort_reverse");
    wrk.create(
        "in.csv",
        vec![svec!["R", "S"], svec!["1", "b"], svec!["2", "a"]],
    );

    let mut cmd = wrk.command("sort");
    cmd.arg("-R").arg("--no-headers").arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["R", "S"], svec!["2", "a"], svec!["1", "b"]];
    assert_eq!(got, expected);
}

#[test]
fn sort_reverse_faster() {
    let wrk = Workdir::new("sort_reverse_faster");
    wrk.create(
        "in.csv",
        vec![svec!["R", "S"], svec!["1", "b"], svec!["2", "a"]],
    );

    let mut cmd = wrk.command("sort");
    cmd.arg("-R")
        .arg("--no-headers")
        .arg("--faster")
        .arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["R", "S"], svec!["2", "a"], svec!["1", "b"]];
    assert_eq!(got, expected);
}

#[test]
fn sort_uniq() {
    let wrk = Workdir::new("sort_unique");
    wrk.create(
        "in.csv",
        vec![
            svec!["number", "letter"],
            svec!["2", "c"],
            svec!["1", "a"],
            svec!["3", "f"],
            svec!["2", "b"],
            svec!["1", "d"],
            svec!["2", "e"],
        ],
    );

    let mut cmd = wrk.command("sort");
    cmd.arg("-u").args(["-s", "number"]).arg("-N").arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected =
        vec![
            svec!["number", "letter"],
            svec!["1", "a"],
            svec!["2", "c"],
            svec!["3", "f"],
        ];
    assert_eq!(got, expected);
}

#[test]
fn sort_uniq_faster() {
    let wrk = Workdir::new("sort_unique_faster");
    wrk.create(
        "in.csv",
        vec![
            svec!["number", "letter"],
            svec!["2", "c"],
            svec!["1", "a"],
            svec!["3", "f"],
            svec!["2", "b"],
            svec!["1", "d"],
            svec!["2", "e"],
        ],
    );

    let mut cmd = wrk.command("sort");
    cmd.arg("-u")
        .args(["-s", "number"])
        .arg("-N")
        .arg("--faster")
        .arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected =
        vec![
            svec!["number", "letter"],
            svec!["1", "a"],
            svec!["2", "c"],
            svec!["3", "f"],
        ];
    assert_eq!(got, expected);
}

#[test]
fn sort_random() {
    let wrk = Workdir::new("sort_random");
    wrk.create(
        "in.csv",
        vec![
            svec!["R", "S"],
            svec!["1", "b"],
            svec!["2", "a"],
            svec!["3", "d"],
            svec!["4", "c"],
            svec!["5", "f"],
            svec!["6", "e"],
        ],
    );

    let mut cmd = wrk.command("sort");
    cmd.arg("--random").args(["--seed", "42"]).arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected =
        vec![
            svec!["R", "S"],
            svec!["1", "b"],
            svec!["5", "f"],
            svec!["6", "e"],
            svec!["3", "d"],
            svec!["2", "a"],
            svec!["4", "c"],
        ];
    assert_eq!(got, expected);
}

#[test]
fn sort_random_faster() {
    let wrk = Workdir::new("sort_random_faster");
    wrk.create(
        "in.csv",
        vec![
            svec!["R", "S"],
            svec!["1", "b"],
            svec!["2", "a"],
            svec!["3", "d"],
            svec!["4", "c"],
            svec!["5", "f"],
            svec!["6", "e"],
        ],
    );

    let mut cmd = wrk.command("sort");
    cmd.arg("--random")
        .args(["--seed", "42"])
        .arg("--faster")
        .arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected =
        vec![
            svec!["R", "S"],
            svec!["1", "b"],
            svec!["2", "a"],
            svec!["6", "e"],
            svec!["4", "c"],
            svec!["5", "f"],
            svec!["3", "d"],
        ];
    assert_eq!(got, expected);
}

/// Order `a` and `b` lexicographically using `Ord`
pub fn iter_cmp<A, L, R>(mut a: L, mut b: R) -> cmp::Ordering
where
    A: Ord,
    L: Iterator<Item = A>,
    R: Iterator<Item = A>,
{
    loop {
        match (a.next(), b.next()) {
            (None, None) => return cmp::Ordering::Equal,
            (None, _) => return cmp::Ordering::Less,
            (_, None) => return cmp::Ordering::Greater,
            (Some(x), Some(y)) => match x.cmp(&y) {
                cmp::Ordering::Equal => (),
                non_eq => return non_eq,
            },
        }
    }
}
