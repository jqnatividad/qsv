use crate::workdir::Workdir;
use crate::{qcheck, CsvData};

/// This tests whether `qsv count` gets the right answer.
///
/// It does some simple case analysis to handle whether we want to test counts
/// in the presence of headers and/or indexes.
fn prop_count_len(name: &str, rows: CsvData, headers: bool, idx: bool, noheaders_env: bool) -> bool {
    let mut expected_count = rows.len();
    if headers && expected_count > 0 {
        expected_count -= 1;
    }

    let wrk = Workdir::new(name);
    if idx {
        wrk.create_indexed("in.csv", rows);
    } else {
        wrk.create("in.csv", rows);
    }

    let mut cmd = wrk.command("count");
    if !headers {
        cmd.arg("--no-headers");
    }
    if noheaders_env {
        cmd.env("QSV_NO_HEADERS", "1");
    }
    cmd.arg("in.csv");

    let got_count: usize = wrk.stdout(&mut cmd);
    rassert_eq!(got_count, expected_count)
}

#[test]
fn prop_count() {
    fn p(rows: CsvData) -> bool {
        prop_count_len("prop_count", rows, false, false, false)
    }
    qcheck(p as fn(CsvData) -> bool);
}

#[test]
fn prop_count_headers() {
    fn p(rows: CsvData) -> bool {
        prop_count_len("prop_count_headers", rows, true, false, false)
    }
    qcheck(p as fn(CsvData) -> bool);
}

#[test]
fn prop_count_indexed() {
    fn p(rows: CsvData) -> bool {
        prop_count_len("prop_count_indexed", rows, false, true, false)
    }
    qcheck(p as fn(CsvData) -> bool);
}

#[test]
fn prop_count_indexed_headers() {
    fn p(rows: CsvData) -> bool {
        prop_count_len("prop_count_indexed_headers", rows, true, true, false)
    }
    qcheck(p as fn(CsvData) -> bool);
}

#[test]
fn prop_count_noheaders_env() {
    fn p(rows: CsvData) -> bool {
        prop_count_len("prop_count_noheaders_env", rows, false, false, true)
    }
    qcheck(p as fn(CsvData) -> bool);
}

#[test]
fn prop_count_noheaders_indexed_env() {
    fn p(rows: CsvData) -> bool {
        prop_count_len("prop_count_noheaders_indexed_env", rows, false, true, true)
    }
    qcheck(p as fn(CsvData) -> bool);
}
