use crate::workdir::Workdir;

#[test]
fn comments() {
    let wrk = Workdir::new("comments");
    wrk.create(
        "comments.csv",
        vec![
            svec!["# test file to see how comments work", ""],
            svec!["column1", "column2"],
            svec!["a", "1"],
            svec!["#b", "2"],
            svec!["c", "3"],
        ],
    );
    let mut cmd = wrk.command("input");
    cmd.env("QSV_COMMENTS", "#");
    cmd.arg("comments.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["column1", "column2"],
        svec!["a", "1"],
        svec!["c", "3"],
    ];
    assert_eq!(got, expected);
}
