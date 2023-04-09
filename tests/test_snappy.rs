use std::path::Path;

use crate::workdir::{is_same_file, Workdir};

#[test]
fn snappy_roundtrip() {
    let wrk = Workdir::new("snappy_roundtrip");

    let thedata = vec![
        svec!["Col1", "Description"],
        svec![
            "1",
            "The quick brown fox jumped over the lazy dog by the zigzag quarry site."
        ],
        svec!["2", "Mary had a little lamb"],
        svec![
            "3",
            "I think that I shall never see a poem lovely as a tree."
        ],
        svec!["4", "I think, therefore I am."],
        svec!["5", "I am a leaf on the wind."],
        svec!["6", "Look at me, I'm the captain now."],
        svec!["7", "Bazinga!"],
        svec!["8", "I'm Batman."],
    ];
    wrk.create("in.csv", thedata.clone());

    let out_file = wrk.path("out.csv.sz").to_string_lossy().to_string();
    log::info!("out_file: {}", out_file);

    let mut cmd = wrk.command("snappy");
    cmd.arg("compress")
        .arg("in.csv")
        .args(["--output", &out_file]);

    wrk.assert_success(&mut cmd);

    let mut cmd2 = wrk.command("snappy");
    cmd2.arg("decompress").arg(out_file);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd2);
    assert_eq!(got, thedata);

    wrk.assert_success(&mut cmd2);
}

#[test]
fn snappy_decompress() {
    let wrk = Workdir::new("snappy_decompress");

    let test_file = wrk.load_test_file("boston311-100.csv.sz");

    let mut cmd = wrk.command("snappy");
    cmd.arg("decompress").arg(test_file);

    let got: String = wrk.stdout(&mut cmd);

    let expected = wrk.load_test_resource("boston311-100.csv");

    assert_eq!(got, expected.trim_end());

    wrk.assert_success(&mut cmd);
}

#[test]
fn snappy_compress() {
    let wrk = Workdir::new("snappy_compress");

    let test_file = wrk.load_test_file("boston311-100.csv");

    let mut cmd = wrk.command("snappy");
    cmd.arg("compress")
        .arg(test_file)
        .args(["--output", "out.csv.sz"]);

    wrk.assert_success(&mut cmd);

    let got_path = wrk.path("out.csv.sz");
    let expected = wrk.load_test_file("boston311-100.csv.sz");
    let expected_path = Path::new(&expected);

    assert!(is_same_file(&got_path, expected_path).unwrap());
}

#[test]
fn snappy_check() {
    let wrk = Workdir::new("snappy_check");

    let test_file = wrk.load_test_file("boston311-100.csv.sz");

    let mut cmd = wrk.command("snappy");
    cmd.arg("check").arg(test_file);

    wrk.assert_success(&mut cmd);
}

#[test]
fn snappy_check_invalid() {
    let wrk = Workdir::new("snappy_check_invalid");

    let test_file = wrk.load_test_file("boston311-100.csv");

    let mut cmd = wrk.command("snappy");
    cmd.arg("check").arg(test_file);

    wrk.assert_err(&mut cmd);
}

#[test]
fn snappy_automatic_decompression() {
    let wrk = Workdir::new("snappy_automatic_decompression");

    let test_file = wrk.load_test_file("boston311-100.csv.sz");

    let mut cmd = wrk.command("count");
    cmd.arg(test_file);

    let got: String = wrk.stdout(&mut cmd);
    let expected = "100";
    assert_eq!(got, expected);

    wrk.assert_success(&mut cmd);
}

#[test]
fn snappy_automatic_compression() {
    let wrk = Workdir::new("snappy_automatic_compression");

    let test_file = wrk.load_test_file("boston311-100.csv");

    let mut cmd = wrk.command("slice");
    cmd.args(["--len", "50"])
        .arg(test_file)
        .args(["--output", "out.csv.sz"]);

    wrk.assert_success(&mut cmd);

    let got_path = wrk.path("out.csv.sz");

    let mut cmd2 = wrk.command("count");
    cmd2.arg(got_path);

    wrk.assert_success(&mut cmd2);

    let got: String = wrk.stdout(&mut cmd2);
    let expected = "50";
    assert_eq!(got, expected);

    wrk.assert_success(&mut cmd2);
}
