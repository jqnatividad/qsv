use crate::workdir::Workdir;

#[test]
fn rename() {
    let wrk = Workdir::new("rename");
    wrk.create(
        "in.csv",
        vec![
            svec!["R", "S"],
            svec!["1", "b"],
            svec!["2", "a"],
            svec!["3", "d"],
        ],
    );

    let mut cmd = wrk.command("rename");
    cmd.arg("cola,colb").arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["cola", "colb"],
        svec!["1", "b"],
        svec!["2", "a"],
        svec!["3", "d"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn rename_generic() {
    let wrk = Workdir::new("rename_generic");
    wrk.create(
        "in.csv",
        vec![
            svec!["R", "S"],
            svec!["1", "b"],
            svec!["2", "a"],
            svec!["3", "d"],
        ],
    );

    let mut cmd = wrk.command("rename");
    cmd.arg("_all_generic").arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["_col_1", "_col_2"],
        svec!["1", "b"],
        svec!["2", "a"],
        svec!["3", "d"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn rename_noheaders() {
    let wrk = Workdir::new("rename_noheaders");
    wrk.create(
        "in.csv",
        vec![svec!["1", "b"], svec!["2", "a"], svec!["3", "d"]],
    );

    let mut cmd = wrk.command("rename");
    cmd.arg("cola,colb").arg("--no-headers").arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["cola", "colb"],
        svec!["1", "b"],
        svec!["2", "a"],
        svec!["3", "d"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn rename_noheaders_generic() {
    let wrk = Workdir::new("rename_noheaders_generic");
    wrk.create(
        "in.csv",
        vec![svec!["1", "b"], svec!["2", "a"], svec!["3", "d"]],
    );

    let mut cmd = wrk.command("rename");
    cmd.arg("_ALL_Generic").arg("--no-headers").arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["_col_1", "_col_2"],
        svec!["1", "b"],
        svec!["2", "a"],
        svec!["3", "d"],
    ];
    assert_eq!(got, expected);
}
