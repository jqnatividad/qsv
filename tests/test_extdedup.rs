use newline_converter::dos2unix;

use crate::workdir::Workdir;

#[test]
fn extdedup() {
    let wrk = Workdir::new("extdedup").flexible(true);
    wrk.clear_contents().unwrap();

    let test_file = wrk.load_test_file("boston311-100-20dupes-random.csv");

    let mut cmd = wrk.command("extdedup");
    cmd.arg(test_file).arg("boston311-100-extdeduped.csv");
    wrk.output(&mut cmd);

    // load deduped output
    let deduped_output: String = wrk.from_str(&wrk.path("boston311-100-extdeduped.csv"));

    let expected_csv = wrk.load_test_resource("boston311-100-deduped.csv");
    wrk.create_from_string("boston311-100-deduped.csv", &expected_csv);

    assert_eq!(dos2unix(&deduped_output), dos2unix(&expected_csv));
}

#[test]
fn extdedup_dupesoutput() {
    let wrk = Workdir::new("extdedup-dupes-output").flexible(true);
    wrk.clear_contents().unwrap();

    let test_file = wrk.load_test_file("boston311-100-20dupes-random.csv");

    let mut cmd = wrk.command("extdedup");
    cmd.arg(test_file)
        .arg("boston311-100-extdeduped.csv")
        .args([
            "--dupes-output",
            "boston311-100-extdededuped-dupeoutput.txt",
        ]);
    wrk.output(&mut cmd);

    // load deduped output
    let deduped_output: String = wrk.from_str(&wrk.path("boston311-100-extdeduped.csv"));

    let expected_csv = wrk.load_test_resource("boston311-100-deduped.csv");
    wrk.create_from_string("boston311-100-deduped.csv", &expected_csv);

    assert_eq!(dos2unix(&deduped_output), dos2unix(&expected_csv));

    // load dupe-output txt
    let dupes_output: String = wrk.from_str(&wrk.path("boston311-100-extdededuped-dupeoutput.txt"));

    let expected_output = wrk.load_test_resource("boston311-extdedup-dupeoutput.txt");
    wrk.create_from_string("boston311-extdedup-dupeoutput.txt", &expected_output);

    assert_eq!(dos2unix(&dupes_output), dos2unix(&expected_output));
}
