use newline_converter::dos2unix;

use crate::workdir::Workdir;

#[test]
fn extdedup_linemode() {
    let wrk = Workdir::new("extdedup_linemode").flexible(true);
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
fn extdedup_linemode_dupesoutput() {
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

#[test]
fn extdedupe_csvmode() {
    let wrk = Workdir::new("extdedup-csvmode").flexible(true);
    wrk.clear_contents().unwrap();

    let test_file = wrk.load_test_file("boston311-100-20dupes-random.csv");

    let mut cmd = wrk.command("extdedup");
    cmd.arg(test_file)
        .arg("boston311-100-extdeduped.csv")
        .args(["--select", "case_enquiry_id,open_dt,target_dt"]);
    wrk.output(&mut cmd);

    // load deduped output
    let deduped_output: String = wrk.from_str(&wrk.path("boston311-100-extdeduped.csv"));

    let expected_csv = wrk.load_test_resource("boston311-100-deduped.csv");
    wrk.create_from_string("boston311-100-deduped.csv", &expected_csv);

    assert_eq!(dos2unix(&deduped_output), dos2unix(&expected_csv));

    // Check that the correct number of rows were deduplicated
    let output = wrk.output(&mut cmd);

    // 20 duplicates should be removed
    assert!(String::from_utf8_lossy(&output.stderr).contains("20\n"));
}

#[test]
fn extdedupe_csvmode_dupesoutput() {
    let wrk = Workdir::new("extdedup-csvmode-dupesoutput").flexible(true);
    wrk.clear_contents().unwrap();

    let test_file = wrk.load_test_file("boston311-100-20dupes-random.csv");

    let mut cmd = wrk.command("extdedup");
    cmd.arg(test_file)
        .arg("boston311-100-extdeduped.csv")
        .args([
            "--select",
            "case_enquiry_id,open_dt,target_dt",
            "--dupes-output",
            "boston311-100-extdededuped-dupeoutput.csv",
        ]);
    wrk.output(&mut cmd);

    // load deduped output
    let deduped_output: String = wrk.from_str(&wrk.path("boston311-100-extdeduped.csv"));

    let expected_csv = wrk.load_test_resource("boston311-100-deduped.csv");
    wrk.create_from_string("boston311-100-deduped.csv", &expected_csv);

    assert_eq!(dos2unix(&deduped_output), dos2unix(&expected_csv));

    // load dupe-output txt
    let dupes_output: String = wrk.from_str(&wrk.path("boston311-100-extdededuped-dupeoutput.csv"));

    let expected_output = wrk.load_test_resource("boston311-extdedup-dupeoutput.csv");
    wrk.create_from_string("boston311-extdedup-dupeoutput.csv", &expected_output);

    assert_eq!(dos2unix(&dupes_output), dos2unix(&expected_output));

    // Check that the correct number of rows were deduplicated
    let output = wrk.output(&mut cmd);
    // 20 duplicates should be removed
    assert!(String::from_utf8_lossy(&output.stderr).contains("20\n"));
}

#[test]
fn extdedupe_csvmode_neighborhood() {
    let wrk = Workdir::new("extdedup-csvmode-neighborhood").flexible(true);
    wrk.clear_contents().unwrap();

    let test_file = wrk.load_test_file("boston311-100-20dupes-random.csv");

    let mut cmd = wrk.command("extdedup");
    cmd.arg(test_file)
        .arg("boston311-100-extdeduped.csv")
        .args(["--select", "neighborhood"]);
    wrk.output(&mut cmd);

    // load deduped output
    let deduped_output: String = wrk.from_str(&wrk.path("boston311-100-extdeduped.csv"));

    let expected_csv = wrk.load_test_resource("boston311-extdedup-neighborhood.csv");
    wrk.create_from_string("boston311-extdedup-neighborhood.csv", &expected_csv);

    assert_eq!(dos2unix(&deduped_output), dos2unix(&expected_csv));

    // Check that the correct number of rows were deduplicated
    let output = wrk.output(&mut cmd);

    // 81 duplicates should be removed
    assert!(String::from_utf8_lossy(&output.stderr).contains("81\n"));
}
