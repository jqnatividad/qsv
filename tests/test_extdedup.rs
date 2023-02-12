use newline_converter::dos2unix;

use crate::workdir::Workdir;

#[test]
fn extdedup() {
    let wrk = Workdir::new("extdedup").flexible(true);
    wrk.clear_contents().unwrap();

    // copy csv file to workdir
    let withdupes_csv = wrk.load_test_resource("boston311-100-20dupes-random.csv");
    wrk.create_from_string("boston311-100-20dupes.csv", &withdupes_csv);

    let mut cmd = wrk.command("extdedup");
    cmd.arg("boston311-100-20dupes.csv")
        .arg("boston311-100-extdeduped.csv");
    wrk.output(&mut cmd);

    // load deduped output
    let deduped_output: String = wrk.from_str(&wrk.path("boston311-100-extdeduped.csv"));

    let expected_csv = wrk.load_test_resource("boston311-100-deduped.csv");
    wrk.create_from_string("boston311-100-deduped.csv", &expected_csv);

    assert_eq!(dos2unix(&deduped_output), dos2unix(&expected_csv));
}
