use crate::{qcheck, workdir::Workdir, Csv, CsvData};

// note that these tests sometimes fail because of the property-based testing
// strategy, not because the implementation is incorrect
// The failures are usually because the randomly generated CSV data sometimes
// have BOMs, which are not handled by the reverse command
fn prop_reverse(name: &str, rows: CsvData, headers: bool) -> bool {
    let wrk = Workdir::new(name);
    wrk.create("in.csv", rows.clone());

    let mut cmd = wrk.command("reverse");
    cmd.arg("in.csv");
    if !headers {
        cmd.arg("--no-headers");
    }

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let mut expected = rows.to_vecs();
    let headers = if headers && !expected.is_empty() {
        expected.remove(0)
    } else {
        vec![]
    };
    expected.reverse();
    if !headers.is_empty() {
        expected.insert(0, headers);
    }
    rassert_eq!(got, expected)
}

#[test]
fn prop_reverse_headers() {
    fn p(rows: CsvData) -> bool {
        prop_reverse("prop_reverse_headers", rows, true)
    }
    qcheck(p as fn(CsvData) -> bool);
}

#[test]
fn prop_reverse_no_headers() {
    fn p(rows: CsvData) -> bool {
        prop_reverse("prop_reverse_no_headers", rows, false)
    }
    qcheck(p as fn(CsvData) -> bool);
}

fn prop_reverse_indexed(name: &str, rows: CsvData, headers: bool) -> bool {
    let wrk = Workdir::new(name);
    wrk.create_indexed("in.csv", rows.clone());

    let mut cmd = wrk.command("reverse");
    cmd.arg("in.csv");
    if !headers {
        cmd.arg("--no-headers");
    }

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let mut expected = rows.to_vecs();
    let headers = if headers && !expected.is_empty() {
        expected.remove(0)
    } else {
        vec![]
    };
    expected.reverse();
    if !headers.is_empty() {
        expected.insert(0, headers);
    }
    rassert_eq!(got, expected)
}

#[test]
fn prop_reverse_headers_indexed() {
    fn p(rows: CsvData) -> bool {
        prop_reverse_indexed("prop_reverse_headers_indexed", rows, true)
    }
    qcheck(p as fn(CsvData) -> bool);
}

#[test]
fn prop_reverse_no_headers_indexed() {
    fn p(rows: CsvData) -> bool {
        prop_reverse_indexed("prop_reverse_no_headers_indexed", rows, false)
    }
    qcheck(p as fn(CsvData) -> bool);
}
