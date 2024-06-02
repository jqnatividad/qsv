use std::fs;

use filetime::{set_file_times, FileTime};

use crate::workdir::Workdir;

#[test]
fn index_outdated_count() {
    let wrk = Workdir::new("index_outdated_count");
    wrk.create_indexed(
        "in.csv",
        vec![
            svec!["letter", "number"],
            svec!["a", "13"],
            svec!["b", "24"],
        ],
    );

    let md = fs::metadata(wrk.path("in.csv.idx")).unwrap();
    set_file_times(
        wrk.path("in.csv"),
        future_time(FileTime::from_last_modification_time(&md)),
        future_time(FileTime::from_last_access_time(&md)),
    )
    .unwrap();

    let mut cmd = wrk.command("count");
    cmd.arg("in.csv");

    // count works even if index is stale
    let expected_count = 2;
    let got_count: usize = wrk.stdout(&mut cmd);
    rassert_eq!(got_count, expected_count);
}

#[test]
fn index_outdated_stats() {
    let wrk = Workdir::new("index_outdated_stats");

    wrk.create_indexed(
        "in.csv",
        vec![
            svec!["letter", "number"],
            svec!["a", "1"],
            svec!["b", "2"],
            svec!["c", "3"],
        ],
    );

    let md = fs::metadata(wrk.path("in.csv.idx")).unwrap();
    set_file_times(
        wrk.path("in.csv"),
        future_time(FileTime::from_last_access_time(&md)),
        future_time(FileTime::from_last_modification_time(&md)),
    )
    .unwrap();

    // even if the index is stale, stats should succeed
    // as the index is automatically updated
    let mut cmd = wrk.command("stats");
    cmd.arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec![
            "field",
            "type",
            "is_ascii",
            "sum",
            "min",
            "max",
            "range",
            "min_length",
            "max_length",
            "mean",
            "sem",
            "stddev",
            "variance",
            "cv",
            "nullcount",
            "max_precision",
            "sparsity"
        ],
        svec![
            "letter", "String", "true", "", "a", "c", "", "1", "1", "", "", "", "", "", "0", "",
            "0"
        ],
        svec![
            "number", "Integer", "", "6", "1", "3", "2", "1", "1", "2", "0.4714", "0.8165",
            "0.6667", "40.8248", "0", "", "0"
        ],
    ];

    assert_eq!(got, expected);
}

#[test]
fn index_outdated_index() {
    let wrk = Workdir::new("index_outdated_index");

    wrk.create_indexed(
        "in.csv",
        vec![
            svec!["letter", "number"],
            svec!["a", "1"],
            svec!["b", "2"],
            svec!["c", "3"],
        ],
    );

    let md = fs::metadata(wrk.path("in.csv.idx")).unwrap();
    set_file_times(
        wrk.path("in.csv"),
        future_time(FileTime::from_last_access_time(&md)),
        future_time(FileTime::from_last_modification_time(&md)),
    )
    .unwrap();

    // slice should NOT fail if the index is stale
    // as stale indexes are automatically updated
    let mut cmd = wrk.command("slice");
    cmd.arg("-i").arg("2").arg("in.csv");

    wrk.assert_success(&mut cmd);
}

#[test]
fn index_autoindex_threshold_reached() {
    let wrk = Workdir::new("index_autoindex_threshold_reached");

    wrk.create(
        "in.csv",
        vec![
            svec!["letter", "number"],
            svec!["a", "1"],
            svec!["b", "2"],
            svec!["c", "3"],
            svec!["d", "4"],
        ],
    );

    // slice should automatically create an index
    // as the file size is greater than the QSV_AUTOINDEX_SIZE threshold
    let mut cmd = wrk.command("slice");
    cmd.env("QSV_AUTOINDEX_SIZE", "1")
        .arg("-i")
        .arg("2")
        .arg("in.csv");
    wrk.assert_success(&mut cmd);

    // index should be created
    assert!(wrk.path("in.csv.idx").exists());
}

#[test]
fn index_autoindex_threshold_not_reached() {
    let wrk = Workdir::new("index_autoindex_threshold_not_reached");

    wrk.create(
        "in.csv",
        vec![
            svec!["letter", "number"],
            svec!["a", "1"],
            svec!["b", "2"],
            svec!["c", "3"],
            svec!["d", "4"],
        ],
    );

    // slice will NOT automatically create an index
    // as the file size is less than the QSV_AUTOINDEX_SIZE threshold
    let mut cmd = wrk.command("slice");
    cmd.env("QSV_AUTOINDEX_SIZE", "10000000")
        .arg("-i")
        .arg("2")
        .arg("in.csv");
    wrk.assert_success(&mut cmd);

    // index should NOT be created
    assert!(!wrk.path("in.csv.idx").exists());
}

fn future_time(ft: FileTime) -> FileTime {
    let secs = ft.unix_seconds();
    FileTime::from_unix_time(secs + 10_000, 0)
}
