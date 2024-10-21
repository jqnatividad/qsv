use newline_converter::dos2unix;

use crate::workdir::Workdir;

#[test]
fn extsort_linemode() {
    let wrk = Workdir::new("extsort_linemode").flexible(true);
    wrk.clear_contents().unwrap();

    // copy csv file to workdir
    let unsorted_csv = wrk.load_test_resource("adur-public-toilets.csv");
    wrk.create_from_string("adur-public-toilets.csv", &unsorted_csv);

    let mut cmd = wrk.command("extsort");
    cmd.arg("adur-public-toilets.csv")
        .arg("adur-public-toilets-extsort-test.csv");
    wrk.output(&mut cmd);

    // load sorted output
    let sorted_output: String = wrk.from_str(&wrk.path("adur-public-toilets-extsort-test.csv"));

    let expected_csv = wrk.load_test_resource("adur-public-toilets-sorted.csv");
    wrk.create_from_string("adur-public-toilets-sorted.csv", &expected_csv);

    assert_eq!(dos2unix(&sorted_output), dos2unix(&expected_csv));
}

#[test]
fn extsort_csvmode() {
    let wrk = Workdir::new("extsort_csvmode").flexible(true);
    wrk.clear_contents().unwrap();

    // copy csv file to workdir
    let unsorted_csv = wrk.load_test_resource("adur-public-toilets.csv");
    wrk.create_from_string("adur-public-toilets.csv", &unsorted_csv);

    // set the environment variable to autoindex
    std::env::set_var("QSV_AUTOINDEX_SIZE", "1");

    let mut cmd = wrk.command("extsort");
    cmd.arg("adur-public-toilets.csv")
        .args(["--select", "OpeningHours,StreetAddress,LocationText"])
        .arg("adur-public-toilets-extsort-csvmode.csv");
    wrk.output(&mut cmd);
    // unset the environment variable
    std::env::remove_var("QSV_AUTOINDEX_SIZE");

    // load sorted output
    let sorted_output: String = wrk.from_str(&wrk.path("adur-public-toilets-extsort-csvmode.csv"));

    let expected_csv = wrk.load_test_resource("adur-public-toilets-extsorted-csvmode.csv");
    wrk.create_from_string("adur-public-toilets-extsorted-csvmode.csv", &expected_csv);

    assert_eq!(dos2unix(&sorted_output), dos2unix(&expected_csv));
}
