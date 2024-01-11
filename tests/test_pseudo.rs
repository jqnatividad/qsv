use crate::workdir::Workdir;

#[test]
fn pseudo() {
    let wrk = Workdir::new("pseudo");
    wrk.create(
        "data.csv",
        vec![
            svec!["name", "colors"],
            svec!["Mary", "yellow"],
            svec!["John", "blue"],
            svec!["Mary", "purple"],
            svec!["Sue", "orange"],
            svec!["John", "magenta"],
            svec!["Mary", "cyan"],
        ],
    );
    let mut cmd = wrk.command("pseudo");
    cmd.arg("name").arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["name", "colors"],
        svec!["0", "yellow"],
        svec!["1", "blue"],
        svec!["0", "purple"],
        svec!["2", "orange"],
        svec!["1", "magenta"],
        svec!["0", "cyan"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn pseudo_no_headers() {
    let wrk = Workdir::new("pseudo");
    wrk.create(
        "data.csv",
        vec![
            svec!["Mary", "yellow"],
            svec!["John", "blue"],
            svec!["Mary", "purple"],
            svec!["Sue", "orange"],
            svec!["John", "magenta"],
            svec!["Mary", "cyan"],
        ],
    );
    let mut cmd = wrk.command("pseudo");
    cmd.arg("1").arg("--no-headers").arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["0", "yellow"],
        svec!["1", "blue"],
        svec!["0", "purple"],
        svec!["2", "orange"],
        svec!["1", "magenta"],
        svec!["0", "cyan"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn pseudo_formatstr() {
    let wrk = Workdir::new("pseudo_formatstr");
    wrk.create(
        "data.csv",
        vec![
            svec!["name", "colors"],
            svec!["Mary", "yellow"],
            svec!["John", "blue"],
            svec!["Mary", "purple"],
            svec!["Sue", "orange"],
            svec!["John", "magenta"],
            svec!["Mary", "cyan"],
        ],
    );
    let mut cmd = wrk.command("pseudo");
    cmd.arg("name")
        .args(["--formatstr", "ID-{}"])
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["name", "colors"],
        svec!["ID-0", "yellow"],
        svec!["ID-1", "blue"],
        svec!["ID-0", "purple"],
        svec!["ID-2", "orange"],
        svec!["ID-1", "magenta"],
        svec!["ID-0", "cyan"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn pseudo_formatstr_increment() {
    let wrk = Workdir::new("pseudo_formatstr_increment");
    wrk.create(
        "data.csv",
        vec![
            svec!["name", "colors"],
            svec!["Mary", "yellow"],
            svec!["John", "blue"],
            svec!["Mary", "purple"],
            svec!["Sue", "orange"],
            svec!["John", "magenta"],
            svec!["Mary", "cyan"],
        ],
    );
    let mut cmd = wrk.command("pseudo");
    cmd.arg("name")
        .args(["--formatstr", "ID-{}"])
        .args(["--increment", "5"])
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["name", "colors"],
        svec!["ID-0", "yellow"],
        svec!["ID-5", "blue"],
        svec!["ID-0", "purple"],
        svec!["ID-10", "orange"],
        svec!["ID-5", "magenta"],
        svec!["ID-0", "cyan"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn pseudo_formatstr_start_increment() {
    let wrk = Workdir::new("pseudo_formatstr_start_increment");
    wrk.create(
        "data.csv",
        vec![
            svec!["name", "colors"],
            svec!["Mary", "yellow"],
            svec!["John", "blue"],
            svec!["Mary", "purple"],
            svec!["Sue", "orange"],
            svec!["John", "magenta"],
            svec!["Mary", "cyan"],
        ],
    );
    let mut cmd = wrk.command("pseudo");
    cmd.arg("name")
        .args(["--start", "1000"])
        .args(["--formatstr", "ID-{}"])
        .args(["--increment", "5"])
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["name", "colors"],
        svec!["ID-1000", "yellow"],
        svec!["ID-1005", "blue"],
        svec!["ID-1000", "purple"],
        svec!["ID-1010", "orange"],
        svec!["ID-1005", "magenta"],
        svec!["ID-1000", "cyan"],
    ];
    assert_eq!(got, expected);
}
