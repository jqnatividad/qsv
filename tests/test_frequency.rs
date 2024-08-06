use std::{borrow::ToOwned, collections::hash_map::Entry, process};

use ahash::AHashMap;
use serde::Deserialize;
use stats::Frequencies;

use crate::{qcheck_sized, workdir::Workdir, Csv, CsvData};

fn setup(name: &str) -> (Workdir, process::Command) {
    let rows = vec![
        svec!["h1", "h2"],
        svec!["a", "z"],
        svec!["a", "y"],
        svec!["a", "y"],
        svec!["b", "z"],
        svec!["a", "Y"],
        svec!["", "z"],
        svec!["(NULL)", "x"],
    ];

    let wrk = Workdir::new(name);
    wrk.create("in.csv", rows);

    let mut cmd = wrk.command("frequency");
    cmd.arg("in.csv");

    (wrk, cmd)
}

#[test]
fn frequency_no_headers() {
    let (wrk, mut cmd) = setup("frequency_no_headers");
    cmd.args(["--limit", "0"])
        .args(["--select", "1"])
        .arg("--no-headers");

    let mut got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    got = got.into_iter().skip(1).collect();
    got.sort_unstable();
    let expected = vec![
        svec!["1", "(NULL)", "1", "12.5"],
        svec!["1", "(NULL)", "1", "12.5"],
        svec!["1", "a", "4", "50"],
        svec!["1", "b", "1", "12.5"],
        svec!["1", "h1", "1", "12.5"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn frequency_casesensitive() {
    let (wrk, mut cmd) = setup("frequency_casesensitive");
    cmd.args(["--limit", "0"]).args(["--select", "h2"]);

    let mut got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    got.sort_unstable();
    let expected = vec![
        svec!["field", "value", "count", "percentage"],
        svec!["h2", "Y", "1", "14.28571"],
        svec!["h2", "x", "1", "14.28571"],
        svec!["h2", "y", "2", "28.57143"],
        svec!["h2", "z", "3", "42.85714"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn frequency_ignorecase() {
    let (wrk, mut cmd) = setup("frequency_ignorecase");
    cmd.arg("--ignore-case")
        .args(["--limit", "0"])
        .args(["--select", "h2"]);

    let mut got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    got.sort_unstable();
    let expected = vec![
        svec!["field", "value", "count", "percentage"],
        svec!["h2", "x", "1", "14.28571"],
        svec!["h2", "y", "3", "42.85714"],
        svec!["h2", "z", "3", "42.85714"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn frequency_trim() {
    let wrk = Workdir::new("frequency_trim");

    let rows = vec![
        svec!["h1", "h2"],
        svec!["a", "z"],
        svec!["a", "y"],
        svec!["a", "y"],
        svec!["b", "z"],
        svec!["a", "Y"],
        svec!["", "z"],
        svec!["(NULL)", "x"],
        svec!["a ", " z"],
        svec!["     A", "  Z   "],
        svec!["  a  ", " Y "],
        svec![" A     ", "y "],
        svec!["a", "y "],
        svec!["b", "y "],
        svec!["b", "  Z   "],
    ];

    wrk.create("in.csv", rows);

    let mut cmd = wrk.command("frequency");
    cmd.arg("in.csv")
        .args(["--limit", "0"])
        .args(["--select", "h2"]);

    wrk.assert_success(&mut cmd);

    let mut got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    got.sort_unstable();
    let expected = vec![
        svec!["field", "value", "count", "percentage"],
        svec!["h2", "Y", "2", "14.28571"],
        svec!["h2", "Z", "2", "14.28571"],
        svec!["h2", "x", "1", "7.14286"],
        svec!["h2", "y", "5", "35.71429"],
        svec!["h2", "z", "4", "28.57143"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn frequency_no_trim() {
    let wrk = Workdir::new("frequency_no_trim");

    let rows = vec![
        svec!["h1", "h2"],
        svec!["a", "z"],
        svec!["a", "y"],
        svec!["a", "y"],
        svec!["b", "z"],
        svec!["a", "Y"],
        svec!["", "z"],
        svec!["(NULL)", "x"],
        svec!["a ", " z"],
        svec!["     A", "  Z   "],
        svec!["  a  ", " Y "],
        svec![" A     ", "y "],
        svec!["a", "y "],
        svec!["b", "y "],
        svec!["b", "  Z   "],
    ];

    wrk.create("in.csv", rows);

    let mut cmd = wrk.command("frequency");
    cmd.arg("in.csv")
        .args(["--limit", "0"])
        .args(["--select", "h2"])
        .arg("--no-trim");

    let mut got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    got.sort_unstable();
    let expected = vec![
        svec!["field", "value", "count", "percentage"],
        svec!["h2", "  Z   ", "2", "14.28571"],
        svec!["h2", " Y ", "1", "7.14286"],
        svec!["h2", " z", "1", "7.14286"],
        svec!["h2", "Y", "1", "7.14286"],
        svec!["h2", "x", "1", "7.14286"],
        svec!["h2", "y", "2", "14.28571"],
        svec!["h2", "y ", "3", "21.42857"],
        svec!["h2", "z", "3", "21.42857"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn frequency_no_nulls() {
    let (wrk, mut cmd) = setup("frequency_no_nulls");
    cmd.arg("--no-nulls")
        .args(["--limit", "0"])
        .args(["--select", "h1"]);

    let mut got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    got.sort_unstable();
    let expected = vec![
        svec!["field", "value", "count", "percentage"],
        svec!["h1", "(NULL)", "1", "16.66667"],
        svec!["h1", "a", "4", "66.66667"],
        svec!["h1", "b", "1", "16.66667"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn frequency_nulls() {
    let (wrk, mut cmd) = setup("frequency_nulls");
    cmd.args(["--limit", "0"]).args(["--select", "h1"]);

    let mut got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    got.sort_unstable();
    let expected = vec![
        svec!["field", "value", "count", "percentage"],
        svec!["h1", "(NULL)", "1", "14.28571"],
        svec!["h1", "(NULL)", "1", "14.28571"],
        svec!["h1", "a", "4", "57.14286"],
        svec!["h1", "b", "1", "14.28571"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn frequency_limit() {
    let (wrk, mut cmd) = setup("frequency_limit");
    cmd.args(["--limit", "1"]);

    let mut got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    got.sort_unstable();
    let expected = vec![
        svec!["field", "value", "count", "percentage"],
        svec!["h1", "Other (3)", "3", "42.85714"],
        svec!["h1", "a", "4", "57.14286"],
        svec!["h2", "Other (3)", "4", "57.14286"],
        svec!["h2", "z", "3", "42.85714"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn frequency_pct_dec_places() {
    let (wrk, mut cmd) = setup("frequency_pct_dec_places");
    cmd.args(["--limit", "1"]).args(["--pct-dec-places", "3"]);

    let mut got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    got.sort_unstable();
    let expected = vec![
        svec!["field", "value", "count", "percentage"],
        svec!["h1", "Other (3)", "3", "42.857"],
        svec!["h1", "a", "4", "57.143"],
        svec!["h2", "Other (3)", "4", "57.143"],
        svec!["h2", "z", "3", "42.857"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn frequency_neg_pct_dec_places() {
    let (wrk, mut cmd) = setup("frequency_neg_pct_dec_places");
    cmd.args(["--limit", "1"]).args(["--pct-dec-places", "-4"]);

    let mut got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    got.sort_unstable();
    let expected = vec![
        svec!["field", "value", "count", "percentage"],
        svec!["h1", "Other (3)", "3", "42.8571"],
        svec!["h1", "a", "4", "57.1429"],
        svec!["h2", "Other (3)", "4", "57.1429"],
        svec!["h2", "z", "3", "42.8571"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn frequency_limit_no_other() {
    let (wrk, mut cmd) = setup("frequency_limit_no_other");
    cmd.args(["--limit", "1"]).args(["--other-text", "<NONE>"]);

    let mut got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    got.sort_unstable();
    let expected = vec![
        svec!["field", "value", "count", "percentage"],
        svec!["h1", "a", "4", "57.14286"],
        svec!["h2", "z", "3", "42.85714"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn frequency_negative_limit() {
    let (wrk, mut cmd) = setup("frequency_negative_limit");
    cmd.args(["--limit", "-4"]);

    let mut got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    got.sort_unstable();
    let expected = vec![
        svec!["field", "value", "count", "percentage"],
        svec!["h1", "Other (3)", "3", "42.85714"],
        svec!["h1", "a", "4", "57.14286"],
        svec!["h2", "Other (4)", "7", "100"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn frequency_limit_threshold() {
    let (wrk, mut cmd) = setup("frequency_limit_threshold");
    cmd.args(["--limit", "-4"]).args(["--lmt-threshold", "4"]);

    let mut got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    got.sort_unstable();
    let expected = vec![
        svec!["field", "value", "count", "percentage"],
        svec!["h1", "Other (3)", "3", "42.85714"],
        svec!["h1", "a", "4", "57.14286"],
        svec!["h2", "Other (4)", "7", "100"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn frequency_limit_threshold_notmet() {
    let (wrk, mut cmd) = setup("frequency_limit_threshold_notmet");
    cmd.args(["--limit", "-2"]).args(["--lmt-threshold", "3"]);

    let mut got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    got.sort_unstable();
    let expected = vec![
        svec!["field", "value", "count", "percentage"],
        svec!["h1", "(NULL)", "1", "14.28571"],
        svec!["h1", "(NULL)", "1", "14.28571"],
        svec!["h1", "a", "4", "57.14286"],
        svec!["h1", "b", "1", "14.28571"],
        svec!["h2", "Y", "1", "14.28571"],
        svec!["h2", "x", "1", "14.28571"],
        svec!["h2", "y", "2", "28.57143"],
        svec!["h2", "z", "3", "42.85714"],
    ];
    assert_eq!(got, expected);
}
#[test]
fn frequency_asc() {
    let (wrk, mut cmd) = setup("frequency_asc");
    cmd.args(["--select", "h2"]).arg("--asc");

    let mut got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    got.sort_unstable();
    let expected = vec![
        svec!["field", "value", "count", "percentage"],
        svec!["h2", "Y", "1", "14.28571"],
        svec!["h2", "x", "1", "14.28571"],
        svec!["h2", "y", "2", "28.57143"],
        svec!["h2", "z", "3", "42.85714"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn frequency_asc_ignorecase() {
    let (wrk, mut cmd) = setup("frequency_asc_ignorecase");
    cmd.arg("--ignore-case")
        .args(["--select", "h2"])
        .arg("--asc");

    let mut got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    got.sort_unstable();
    let expected = vec![
        svec!["field", "value", "count", "percentage"],
        svec!["h2", "x", "1", "14.28571"],
        svec!["h2", "y", "3", "42.85714"],
        svec!["h2", "z", "3", "42.85714"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn frequency_custom_other_text() {
    let (wrk, mut cmd) = setup("frequency_custom_other_text");
    cmd.args(["--limit", "-4"])
        .args(["--lmt-threshold", "4"])
        .args(["--other-text", "其他"]);

    let mut got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    got.sort_unstable();
    let expected = vec![
        svec!["field", "value", "count", "percentage"],
        svec!["h1", "a", "4", "57.14286"],
        svec!["h1", "其他 (3)", "3", "42.85714"],
        svec!["h2", "其他 (4)", "7", "100"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn frequency_custom_other_text_sorted() {
    let (wrk, mut cmd) = setup("frequency_custom_other_text_sorted");
    cmd.args(["--limit", "-4"])
        .args(["--lmt-threshold", "4"])
        .args(["--other-text", "Ibang halaga"])
        .arg("--other-sorted");

    let mut got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    got.sort_unstable();
    let expected = vec![
        svec!["field", "value", "count", "percentage"],
        svec!["h1", "Ibang halaga (3)", "3", "42.85714"],
        svec!["h1", "a", "4", "57.14286"],
        svec!["h2", "Ibang halaga (4)", "7", "100"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn frequency_other_sorted() {
    let (wrk, mut cmd) = setup("frequency_other_sorted");
    cmd.args(["--limit", "-4"])
        .args(["--lmt-threshold", "4"])
        .arg("--other-sorted");

    let mut got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    got.sort_unstable();
    let expected = vec![
        svec!["field", "value", "count", "percentage"],
        svec!["h1", "Other (3)", "3", "42.85714"],
        svec!["h1", "a", "4", "57.14286"],
        svec!["h2", "Other (4)", "7", "100"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn frequency_other_text_none() {
    let (wrk, mut cmd) = setup("frequency_other_text_none");
    cmd.args(["--limit", "-4"])
        .args(["--lmt-threshold", "4"])
        .args(["--other-text", "<NONE>"]);

    let mut got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    got.sort_unstable();
    let expected = vec![
        svec!["field", "value", "count", "percentage"],
        svec!["h1", "a", "4", "57.14286"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn frequency_select() {
    let (wrk, mut cmd) = setup("frequency_select");
    cmd.args(["--limit", "0"]).args(["--select", "h2"]);

    let mut got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    got.sort_unstable();
    let expected = vec![
        svec!["field", "value", "count", "percentage"],
        svec!["h2", "Y", "1", "14.28571"],
        svec!["h2", "x", "1", "14.28571"],
        svec!["h2", "y", "2", "28.57143"],
        svec!["h2", "z", "3", "42.85714"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn frequency_all_unique() {
    let wrk = Workdir::new("frequency_all_unique");
    let testdata = wrk.load_test_file("boston311-100.csv");
    let mut cmd = wrk.command("frequency");
    cmd.args(["--select", "1"]).arg(testdata);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["field", "value", "count", "percentage"],
        svec!["case_enquiry_id", "101004113298", "1", "1"],
        svec!["case_enquiry_id", "101004113313", "1", "1"],
        svec!["case_enquiry_id", "101004113348", "1", "1"],
        svec!["case_enquiry_id", "101004113363", "1", "1"],
        svec!["case_enquiry_id", "101004113371", "1", "1"],
        svec!["case_enquiry_id", "101004113385", "1", "1"],
        svec!["case_enquiry_id", "101004113386", "1", "1"],
        svec!["case_enquiry_id", "101004113391", "1", "1"],
        svec!["case_enquiry_id", "101004113394", "1", "1"],
        svec!["case_enquiry_id", "101004113403", "1", "1"],
        svec!["case_enquiry_id", "Other (90)", "90", "90"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn frequency_all_unique_with_stats_cache() {
    let wrk = Workdir::new("frequency_all_unique_with_stats_cache");
    let testdata = wrk.load_test_file("boston311-100.csv");

    let mut stats_cmd = wrk.command("stats");
    stats_cmd
        .arg(testdata.clone())
        .arg("--cardinality")
        .arg("--stats-binout");

    wrk.assert_success(&mut stats_cmd);

    let mut cmd = wrk.command("frequency");
    cmd.args(["--select", "1"]).arg(testdata);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["field", "value", "count", "percentage"],
        svec!["case_enquiry_id", "ALL_UNIQUE", "100", "100"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn frequency_all_unique_force_stats_cache() {
    let wrk = Workdir::new("frequency_all_unique_force_stats_cache");
    let testdata = wrk.load_test_file("boston311-100.csv");

    let mut cmd = wrk.command("frequency");
    cmd.args(["--select", "1"])
        .args(["--stats-mode", "force"])
        .arg(testdata);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["field", "value", "count", "percentage"],
        svec!["case_enquiry_id", "ALL_UNIQUE", "100", "100"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn frequency_all_unique_stats_mode_none() {
    let wrk = Workdir::new("frequency_all_unique_stats_mode_none");
    let testdata = wrk.load_test_file("boston311-100.csv");

    // create stats cache
    let mut stats_cmd = wrk.command("stats");
    stats_cmd
        .arg(testdata.clone())
        .arg("--cardinality")
        .arg("--stats-binout");

    wrk.assert_success(&mut stats_cmd);

    // run frequency with stats-mode none, ignoring the stats cache
    let mut cmd = wrk.command("frequency");
    cmd.args(["--select", "1"])
        .args(["--stats-mode", "none"])
        .arg(testdata);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["field", "value", "count", "percentage"],
        svec!["case_enquiry_id", "101004113298", "1", "1"],
        svec!["case_enquiry_id", "101004113313", "1", "1"],
        svec!["case_enquiry_id", "101004113348", "1", "1"],
        svec!["case_enquiry_id", "101004113363", "1", "1"],
        svec!["case_enquiry_id", "101004113371", "1", "1"],
        svec!["case_enquiry_id", "101004113385", "1", "1"],
        svec!["case_enquiry_id", "101004113386", "1", "1"],
        svec!["case_enquiry_id", "101004113391", "1", "1"],
        svec!["case_enquiry_id", "101004113394", "1", "1"],
        svec!["case_enquiry_id", "101004113403", "1", "1"],
        svec!["case_enquiry_id", "Other (90)", "90", "90"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn frequency_issue1962() {
    let wrk = Workdir::new("frequency_1962");
    let testdata = wrk.load_test_file("data1962.csv");
    let mut cmd = wrk.command("frequency");
    cmd.args(["--limit", "15"]).arg(testdata.clone());

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["field", "value", "count", "percentage"],
        svec!["year", "2024", "24", "8"],
        svec!["year", "2023", "23", "7.66667"],
        svec!["year", "2022", "22", "7.33333"],
        svec!["year", "2021", "21", "7"],
        svec!["year", "2020", "20", "6.66667"],
        svec!["year", "2019", "19", "6.33333"],
        svec!["year", "2018", "18", "6"],
        svec!["year", "2017", "17", "5.66667"],
        svec!["year", "2016", "16", "5.33333"],
        svec!["year", "2015", "15", "5"],
        svec!["year", "2014", "14", "4.66667"],
        svec!["year", "2013", "13", "4.33333"],
        svec!["year", "2012", "12", "4"],
        svec!["year", "2011", "11", "3.66667"],
        svec!["year", "2010", "10", "3.33333"],
        svec!["year", "Other (9)", "45", "15"],
    ];
    assert_eq!(got, expected);

    let mut cmd = wrk.command("frequency");
    cmd.args(["--limit", "5"]).arg(testdata);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["field", "value", "count", "percentage"],
        svec!["year", "2024", "24", "8"],
        svec!["year", "2023", "23", "7.66667"],
        svec!["year", "2022", "22", "7.33333"],
        svec!["year", "2021", "21", "7"],
        svec!["year", "2020", "20", "6.66667"],
        svec!["year", "Other (19)", "190", "63.33333"],
    ];
    assert_eq!(got, expected);
}

// This tests that a frequency table computed by `qsv` is always the same
// as the frequency table computed in memory.
#[test]
fn prop_frequency() {
    fn p(rows: CsvData) -> bool {
        param_prop_frequency("prop_frequency", rows, false)
    }
    // Run on really small values because we are incredibly careless
    // with allocation.
    qcheck_sized(p as fn(CsvData) -> bool, 5);
}

// This tests that running the frequency command on a CSV file with these two
// rows does not burst in flames:
//
//     \u{FEFF}
//     ""
//
// In this case, the `param_prop_frequency` just ignores this particular test.
// Namely, \u{FEFF} is the UTF-8 BOM, which is ignored by the underlying CSV
// reader.
#[test]
fn frequency_bom() {
    let rows = CsvData {
        data: vec![
            crate::CsvRecord(vec!["\u{FEFF}".to_string()]),
            crate::CsvRecord(vec![String::new()]),
        ],
    };
    assert!(param_prop_frequency("prop_frequency", rows, false))
}

// This tests that a frequency table computed by `qsv` (with an index) is
// always the same as the frequency table computed in memory.
#[test]
fn prop_frequency_indexed() {
    fn p(rows: CsvData) -> bool {
        param_prop_frequency("prop_frequency_indxed", rows, true)
    }
    // Run on really small values because we are incredibly careless
    // with allocation.
    qcheck_sized(p as fn(CsvData) -> bool, 5);
}

fn param_prop_frequency(name: &str, rows: CsvData, idx: bool) -> bool {
    if !rows.is_empty() && rows[0][0].len() == 3 && rows[0][0] == "\u{FEFF}" {
        return true;
    }
    let wrk = Workdir::new(name);
    if idx {
        wrk.create_indexed("in.csv", rows.clone());
    } else {
        wrk.create("in.csv", rows.clone());
    }

    let mut cmd = wrk.command("frequency");
    cmd.arg("in.csv")
        .args(["-j", "4"])
        .args(["--limit", "0"])
        .args(["--unq-limit", "0"]);

    let stdout = wrk.stdout::<String>(&mut cmd);
    let got_ftables = ftables_from_csv_string(stdout);
    let expected_ftables = ftables_from_rows(rows);
    assert_eq_ftables(&got_ftables, &expected_ftables)
}

type FTables = AHashMap<String, Frequencies<String>>;

#[derive(Deserialize)]
struct FRow {
    field: String,
    value: String,
    count: usize,
}

fn ftables_from_rows<T: Csv>(rows: T) -> FTables {
    let mut rows = rows.to_vecs();
    if rows.len() <= 1 {
        return AHashMap::new();
    }

    let header = rows.remove(0);
    let mut ftables = AHashMap::new();
    for field in &header {
        ftables.insert(field.clone(), Frequencies::new());
    }
    for row in rows {
        for (i, mut field) in row.into_iter().enumerate() {
            field = field.trim().to_owned();
            if field.is_empty() {
                field = "(NULL)".to_owned();
            }
            ftables.get_mut(&header[i]).unwrap().add(field);
        }
    }
    ftables
}

fn ftables_from_csv_string(data: String) -> FTables {
    let mut rdr = csv::Reader::from_reader(data.as_bytes());
    let mut ftables = AHashMap::new();
    for frow in rdr.deserialize() {
        let frow: FRow = frow.unwrap();
        match ftables.entry(frow.field) {
            Entry::Vacant(v) => {
                let mut ftable = Frequencies::new();
                for _ in 0..frow.count {
                    ftable.add(frow.value.clone());
                }
                v.insert(ftable);
            },
            Entry::Occupied(mut v) => {
                for _ in 0..frow.count {
                    v.get_mut().add(frow.value.clone());
                }
            },
        }
    }
    ftables
}

fn freq_data<T>(ftable: &Frequencies<T>) -> Vec<(&T, u64)>
where
    T: ::std::hash::Hash + Ord + Clone,
{
    let (mut freqs, _) = ftable.most_frequent();
    freqs.sort_unstable();
    freqs
}

fn assert_eq_ftables(got: &FTables, expected: &FTables) -> bool {
    for (k, v) in got.iter() {
        assert_eq!(freq_data(v), freq_data(expected.get(k).unwrap()));
    }
    for (k, v) in expected.iter() {
        assert_eq!(freq_data(got.get(k).unwrap()), freq_data(v));
    }
    true
}
