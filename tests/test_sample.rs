use crate::workdir::Workdir;

#[test]
fn sample_seed() {
    let wrk = Workdir::new("sample_seed");
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
    cmd.args(["--seed", "42"]).arg("5").arg("in.csv");

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

#[test]
fn sample_seed_delimiter() {
    let wrk = Workdir::new("sample_seed_delimiter");
    wrk.create_with_delim(
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
        b'|',
    );

    let mut cmd = wrk.command("sample");
    cmd.args(["--seed", "42"]).arg("5").arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["R|S"],
        svec!["1|b"],
        svec!["2|a"],
        svec!["3|d"],
        svec!["7|i"],
        svec!["5|f"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sample_seed_faster() {
    let wrk = Workdir::new("sample_seed_faster");
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
    cmd.arg("--faster")
        .args(["--seed", "42"])
        .arg("5")
        .arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["R", "S"],
        svec!["1", "b"],
        svec!["2", "a"],
        svec!["3", "d"],
        svec!["4", "c"],
        svec!["6", "e"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sample_seed_url() {
    let wrk = Workdir::new("sample_seed_url");

    let mut cmd = wrk.command("sample");
    cmd.args(["--seed", "42"])
        .arg("5")
        .arg("https://github.com/jqnatividad/qsv/raw/master/resources/test/aliases.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        ["position", "title"],
        ["Q107145064", "embajador de España en Macedonia del Norte"],
        ["Q107133795", "ambassadrice d'Espagne aux Palaos"],
        ["Q107126367", "ambassador to Mali"],
        ["Q106807027", "Minister of Industry, Trade and Tourism"],
        [
            "Q105325251",
            "Consejero de Sanidad, Trabajo y Seguridad Social",
        ],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sample_percentage_seed_no_index_percentage() {
    let wrk = Workdir::new("sample_percentage_seed_no_index_percentage");
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
            svec!["8", "h"],
        ],
    );

    let mut cmd = wrk.command("sample");
    cmd.args(["--seed", "42"]).arg("0.6").arg("in.csv");

    // no error since percentage sampling no longer requires an index
    // though note the results are different even with the same seed and
    // sample size. This is because we use sample_reservoir method, not
    // sample_random_access method
    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["R", "S"],
        svec!["8", "h"],
        svec!["2", "a"],
        svec!["3", "d"],
        svec!["7", "i"],
        svec!["5", "f"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sample_percentage_seed_indexed() {
    let wrk = Workdir::new("sample_indexed");
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
            svec!["8", "h"],
        ],
    );

    let mut cmd = wrk.command("sample");
    cmd.args(["--seed", "42"]).arg("0.4").arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected =
        vec![
            svec!["R", "S"],
            svec!["6", "e"],
            svec!["3", "d"],
            svec!["7", "i"],
        ];
    assert_eq!(got, expected);
}

#[test]
fn sample_percentage_seed_indexed_faster() {
    let wrk = Workdir::new("sample_indexed_faster");
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
            svec!["8", "h"],
        ],
    );

    let mut cmd = wrk.command("sample");
    cmd.arg("--faster")
        .args(["--seed", "42"])
        .arg("0.4")
        .arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected =
        vec![
            svec!["R", "S"],
            svec!["1", "b"],
            svec!["2", "a"],
            svec!["8", "h"],
        ];
    assert_eq!(got, expected);
}

#[test]
fn sample_indexed_random_access() {
    let wrk = Workdir::new("sample_indexed_random_access");
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
            svec!["9", "i"],
            svec!["10", "j"],
            svec!["11", "k"],
            svec!["12", "l"],
            svec!["13", "m"],
            svec!["14", "n"],
            svec!["15", "o"],
            svec!["16", "p"],
            svec!["17", "q"],
            svec!["18", "r"],
            svec!["19", "s"],
            svec!["20", "t"],
            svec!["21", "u"],
            svec!["22", "v"],
            svec!["23", "w"],
            svec!["24", "x"],
            svec!["25", "y"],
            svec!["26", "z"],
        ],
    );

    let mut cmd = wrk.command("sample");
    cmd.args(["--seed", "42"]).arg("4").arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["R", "S"],
        svec!["23", "w"],
        svec!["17", "q"],
        svec!["24", "x"],
        svec!["16", "p"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sample_indexed_random_access_faster() {
    let wrk = Workdir::new("sample_indexed_random_access_faster");
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
            svec!["9", "i"],
            svec!["10", "j"],
            svec!["11", "k"],
            svec!["12", "l"],
            svec!["13", "m"],
            svec!["14", "n"],
            svec!["15", "o"],
            svec!["16", "p"],
            svec!["17", "q"],
            svec!["18", "r"],
            svec!["19", "s"],
            svec!["20", "t"],
            svec!["21", "u"],
            svec!["22", "v"],
            svec!["23", "w"],
            svec!["24", "x"],
            svec!["25", "y"],
            svec!["26", "z"],
        ],
    );

    let mut cmd = wrk.command("sample");
    cmd.arg("--faster")
        .args(["--seed", "42"])
        .arg("4")
        .arg("in.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["R", "S"],
        svec!["19", "s"],
        svec!["15", "o"],
        svec!["17", "q"],
        svec!["26", "z"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn sample_percentage_negative_sample_size_error() {
    let wrk = Workdir::new("sample_negative");
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
            svec!["8", "h"],
        ],
    );

    let mut cmd = wrk.command("sample");
    cmd.args(["--seed", "42"]).arg("-5").arg("in.csv");

    wrk.assert_err(&mut cmd);
}
