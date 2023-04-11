use std::path::Path;

use crate::workdir::{is_same_file, Workdir};

#[test]
fn to_xlsx_roundtrip() {
    let wrk = Workdir::new("to_xlsx");

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

    let xlsx_file = wrk.path("testxlsx.xlsx").to_string_lossy().to_string();
    log::info!("xlsx_file: {}", xlsx_file);

    let mut cmd = wrk.command("to");
    cmd.arg("xlsx").arg(xlsx_file.clone()).arg("in.csv");

    wrk.assert_success(&mut cmd);

    let mut cmd2 = wrk.command("excel");
    cmd2.arg(xlsx_file);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd2);
    assert_eq!(got, thedata);

    wrk.assert_success(&mut cmd2);
}

#[test]
fn to_datapackage() {
    let wrk = Workdir::new("to_datapackage");

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

    let testdp_json_file = wrk.path("testdp.json");
    let testdp_json_filename = testdp_json_file.to_string_lossy().to_string();

    let mut cmd = wrk.command("to");
    cmd.arg("datapackage")
        .arg(testdp_json_filename.clone())
        .arg("in.csv");

    wrk.assert_success(&mut cmd);

    let got: String = wrk.stdout(&mut cmd);
    let expected: String = r#"Table 'in' (8 rows)

Field Name   Field Type  Field Format
Col1         integer     integer
Description  string      string"#
        .to_string();
    assert_eq!(got, expected);

    let expected = wrk.load_test_file("testdp.json");
    let expected_path = Path::new(&expected);

    assert!(is_same_file(&testdp_json_file, expected_path).unwrap());
}
