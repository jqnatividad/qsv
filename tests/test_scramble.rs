use crate::workdir::Workdir;

#[test]
fn scramble_seed() {
    let wrk = Workdir::new("scramble_random");
    wrk.create_indexed(
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

    let mut cmd = wrk.command("scramble");
    cmd.args(&["--seed", "42"]).arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["R", "S"],
        svec!["7", "i"],
        svec!["3", "d"],
        svec!["6", "e"],
        svec!["1", "b"],
        svec!["5", "f"],
        svec!["8", "h"],
        svec!["4", "c"],
        svec!["2", "a"],
    ];
    assert_eq!(got, expected);
}
