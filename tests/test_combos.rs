// this test module is for combinations of different qsv commands
// to ensure that they behave in a predictable manner
use crate::workdir::Workdir;

#[test]
fn combo_sort_dedup() {
    let wrk = Workdir::new("combo_sort_dedup");
    wrk.create(
        "in.csv",
        vec![
            svec!["id", "timestamp", "h3"],
            svec!["1", "2021-04-26 00:02:18", "a"],
            svec!["2", "2021-04-26 19:22:26", "b"],
            svec!["30", "2021-04-26 11:44:13", "c"],
            svec!["4", "2021-04-26 14:37:03", "d"],
            svec!["2", "2021-04-26 20:22:26", "e"],
            svec!["5", "2021-04-26 19:29:26", "f"],
            svec!["60", "2021-04-26 04:52:46", "g"],
            svec!["2", "2021-04-26 19:12:26", "h"],
            svec!["30", "2021-04-26 10:44:13", "i"],
            svec!["30", "2021-04-26 09:44:13", "j"],
            svec!["1", "2021-04-26 01:02:18", "k"],
        ],
    );

    let mut cmd = wrk.command("sort");
    cmd.arg("-s").arg("timestamp").arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["id", "timestamp", "h3"],
        svec!["1", "2021-04-26 00:02:18", "a"],
        svec!["1", "2021-04-26 01:02:18", "k"],
        svec!["60", "2021-04-26 04:52:46", "g"],
        svec!["30", "2021-04-26 09:44:13", "j"],
        svec!["30", "2021-04-26 10:44:13", "i"],
        svec!["30", "2021-04-26 11:44:13", "c"],
        svec!["4", "2021-04-26 14:37:03", "d"],
        svec!["2", "2021-04-26 19:12:26", "h"],
        svec!["2", "2021-04-26 19:22:26", "b"],
        svec!["5", "2021-04-26 19:29:26", "f"],
        svec!["2", "2021-04-26 20:22:26", "e"],
    ];
    assert_eq!(got, expected);

    wrk.create("in2.csv", expected);

    let mut cmd = wrk.command("dedup");
    cmd.arg("-s").arg("id").arg("in2.csv");

    let got2: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected2 = vec![
        svec!["id", "timestamp", "h3"],
        svec!["1", "2021-04-26 01:02:18", "k"],
        svec!["2", "2021-04-26 20:22:26", "e"],
        svec!["30", "2021-04-26 11:44:13", "c"],
        svec!["4", "2021-04-26 14:37:03", "d"],
        svec!["5", "2021-04-26 19:29:26", "f"],
        svec!["60", "2021-04-26 04:52:46", "g"],
    ];
    assert_eq!(got2, expected2);

    wrk.create("in3.csv", expected2);

    let mut cmd = wrk.command("sort");
    cmd.arg("--numeric").arg("in3.csv");

    let got3: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected3 = vec![
        svec!["id", "timestamp", "h3"],
        svec!["1", "2021-04-26 01:02:18", "k"],
        svec!["2", "2021-04-26 20:22:26", "e"],
        svec!["4", "2021-04-26 14:37:03", "d"],
        svec!["5", "2021-04-26 19:29:26", "f"],
        svec!["30", "2021-04-26 11:44:13", "c"],
        svec!["60", "2021-04-26 04:52:46", "g"],
    ];
    assert_eq!(got3, expected3);
}

#[test]
fn utf8_check_valid() {
    let wrk = Workdir::new("utf8_check_valid");

    let valid_file = wrk.load_test_file("adur-public-toilets.csv");

    let mut cmd = wrk.command("headers");
    cmd.arg(valid_file);

    let got: String = wrk.output_stderr(&mut cmd);
    assert!(got.contains("No error"));
}
