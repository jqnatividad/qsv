use newline_converter::dos2unix;

use crate::workdir::Workdir;

#[test]
fn extsort() {
    let wrk = Workdir::new("extsort").flexible(true);
    wrk.clear_contents().unwrap();

    // copy csv file to workdir
    let unsorted_csv = wrk.load_test_resource("adur-public-toilets.csv");
    wrk.create_from_string("adur-public-toilets.csv", &unsorted_csv);

    // run schema command with value constraints option
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
