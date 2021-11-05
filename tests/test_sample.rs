use crate::workdir::Workdir;

#[test]
fn sample_seed() {
    let wrk = Workdir::new("sample_random");
    wrk.create(
        "in.csv",
        vec![
            svec!["R", "S"],
            svec!["1", "b"],
            svec!["2", "a"],
            svec!["3", "d"],
            svec!["4", "c"],
            svec!["5", "f"],
            svec!["6", "e"],
            svec!["7", "i"],
            svec!["8", "h"],
        ],
    );

    let mut cmd = wrk.command("sample");
    cmd.args(&["--seed", "42"]).arg("5").arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["R", "S"],
        svec!["1", "b"],
        svec!["2", "a"],
        svec!["3", "d"],
        svec!["7", "i"],
        svec!["5", "f"],
    ];
    assert_eq!(got, expected);
}
