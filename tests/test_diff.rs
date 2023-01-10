use crate::workdir::Workdir;

#[test]
fn simple_diff() {
    let wrk = Workdir::new("diff");
    let test_file = wrk.load_test_file("boston311-100.csv");
    let test_file2 = wrk.load_test_file("boston311-100-diff.csv");

    let mut cmd = wrk.command("diff");
    cmd.arg(test_file).arg(test_file2);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    wrk.create("in2.csv", got);

    // sort on the 1st column, case_enquiry_id
    // --select is set to 2 coz `diff` prepends
    // a "diffresult" column
    let mut cmd = wrk.command("sort");
    cmd.arg("--select").arg("2").arg("in2.csv");

    let got2: String = wrk.stdout(&mut cmd);
    let expected2 = wrk.load_test_resource("boston311-100-diffresult.csv");

    assert_eq!(got2, expected2.replace("\r\n", "\n").trim_end());
}
