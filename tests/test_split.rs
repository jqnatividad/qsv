use std::borrow::ToOwned;

use crate::workdir::Workdir;

macro_rules! split_eq {
    ($wrk:expr, $path:expr, $expected:expr) => {
        // assert_eq!($wrk.path($path).into_os_string().into_string().unwrap(),
        // $expected.to_owned());
        assert_eq!(
            $wrk.from_str::<String>(&$wrk.path($path)),
            $expected.to_owned()
        );
    };
}

fn data(headers: bool) -> Vec<Vec<String>> {
    let mut rows = vec![
        svec!["a", "b"],
        svec!["c", "d"],
        svec!["e", "f"],
        svec!["g", "h"],
        svec!["i", "j"],
        svec!["k", "l"],
    ];
    if headers {
        rows.insert(0, svec!["h1", "h2"]);
    }
    rows
}

#[test]
fn split_zero() {
    let wrk = Workdir::new("split_zero");
    wrk.create("in.csv", data(true));

    let mut cmd = wrk.command("split");
    cmd.args(["--size", "0"]).arg(&wrk.path(".")).arg("in.csv");
    wrk.assert_err(&mut cmd);
}

#[test]
fn split() {
    let wrk = Workdir::new("split");
    wrk.create("in.csv", data(true));

    let mut cmd = wrk.command("split");
    cmd.args(["--size", "2"]).arg(&wrk.path(".")).arg("in.csv");
    wrk.run(&mut cmd);

    split_eq!(
        wrk,
        "0.csv",
        "\
h1,h2
a,b
c,d
"
    );
    split_eq!(
        wrk,
        "2.csv",
        "\
h1,h2
e,f
g,h
"
    );
    split_eq!(
        wrk,
        "4.csv",
        "\
h1,h2
i,j
k,l
"
    );
    assert!(!wrk.path("6.csv").exists());
}

#[test]
fn split_a_lot() {
    let wrk = Workdir::new("split_a_lot");
    wrk.create("in.csv", data(true));

    let mut cmd = wrk.command("split");
    cmd.args(["--size", "1000"])
        .arg(&wrk.path("."))
        .arg("in.csv");
    wrk.run(&mut cmd);

    split_eq!(
        wrk,
        "0.csv",
        "\
h1,h2
a,b
c,d
e,f
g,h
i,j
k,l
"
    );
    assert!(!wrk.path("1.csv").exists());
}

#[test]
fn split_a_lot_indexed() {
    let wrk = Workdir::new("split_a_lot_indexed");
    wrk.create_indexed("in.csv", data(true));

    let mut cmd = wrk.command("split");
    cmd.args(["--size", "1000"])
        .arg(&wrk.path("."))
        .arg("in.csv");
    wrk.run(&mut cmd);

    split_eq!(
        wrk,
        "0.csv",
        "\
h1,h2
a,b
c,d
e,f
g,h
i,j
k,l
"
    );
    assert!(!wrk.path("1.csv").exists());
}

#[test]
fn split_padding() {
    let wrk = Workdir::new("split");
    wrk.create("in.csv", data(true));

    let mut cmd = wrk.command("split");
    cmd.args(["--size", "2"])
        .arg("--pad")
        .arg("4")
        .arg(&wrk.path("."))
        .arg("in.csv");
    wrk.run(&mut cmd);

    split_eq!(
        wrk,
        "0000.csv",
        "\
h1,h2
a,b
c,d
"
    );
    split_eq!(
        wrk,
        "0002.csv",
        "\
h1,h2
e,f
g,h
"
    );
    split_eq!(
        wrk,
        "0004.csv",
        "\
h1,h2
i,j
k,l
"
    );
    assert!(!wrk.path("0006.csv").exists());
}

#[test]
fn split_idx() {
    let wrk = Workdir::new("split_idx");
    wrk.create_indexed("in.csv", data(true));

    let mut cmd = wrk.command("split");
    cmd.args(["--size", "2"]).arg(&wrk.path(".")).arg("in.csv");
    wrk.run(&mut cmd);

    split_eq!(
        wrk,
        "0.csv",
        "\
h1,h2
a,b
c,d
"
    );
    split_eq!(
        wrk,
        "2.csv",
        "\
h1,h2
e,f
g,h
"
    );
    split_eq!(
        wrk,
        "4.csv",
        "\
h1,h2
i,j
k,l
"
    );
    assert!(!wrk.path("6.csv").exists());
}

#[test]
fn split_no_headers() {
    let wrk = Workdir::new("split_no_headers");
    wrk.create("in.csv", data(false));

    let mut cmd = wrk.command("split");
    cmd.args(["--no-headers", "--size", "2"])
        .arg(&wrk.path("."))
        .arg("in.csv");
    wrk.run(&mut cmd);

    split_eq!(
        wrk,
        "0.csv",
        "\
a,b
c,d
"
    );
    split_eq!(
        wrk,
        "2.csv",
        "\
e,f
g,h
"
    );
    split_eq!(
        wrk,
        "4.csv",
        "\
i,j
k,l
"
    );
}

#[test]
fn split_no_headers_idx() {
    let wrk = Workdir::new("split_no_headers_idx");
    wrk.create_indexed("in.csv", data(false));

    let mut cmd = wrk.command("split");
    cmd.args(["--no-headers", "--size", "2"])
        .arg(&wrk.path("."))
        .arg("in.csv");
    wrk.run(&mut cmd);

    split_eq!(
        wrk,
        "0.csv",
        "\
a,b
c,d
"
    );
    split_eq!(
        wrk,
        "2.csv",
        "\
e,f
g,h
"
    );
    split_eq!(
        wrk,
        "4.csv",
        "\
i,j
k,l
"
    );
}

#[test]
fn split_one() {
    let wrk = Workdir::new("split_one");
    wrk.create("in.csv", data(true));

    let mut cmd = wrk.command("split");
    cmd.args(["--size", "1"]).arg(&wrk.path(".")).arg("in.csv");
    wrk.run(&mut cmd);

    split_eq!(
        wrk,
        "0.csv",
        "\
h1,h2
a,b
"
    );
    split_eq!(
        wrk,
        "1.csv",
        "\
h1,h2
c,d
"
    );
    split_eq!(
        wrk,
        "2.csv",
        "\
h1,h2
e,f
"
    );
    split_eq!(
        wrk,
        "3.csv",
        "\
h1,h2
g,h
"
    );
    split_eq!(
        wrk,
        "4.csv",
        "\
h1,h2
i,j
"
    );
    split_eq!(
        wrk,
        "5.csv",
        "\
h1,h2
k,l
"
    );
}

#[test]
fn split_one_idx() {
    let wrk = Workdir::new("split_one_idx");
    wrk.create_indexed("in.csv", data(true));

    let mut cmd = wrk.command("split");
    cmd.args(["--size", "1"]).arg(&wrk.path(".")).arg("in.csv");
    wrk.run(&mut cmd);

    split_eq!(
        wrk,
        "0.csv",
        "\
h1,h2
a,b
"
    );
    split_eq!(
        wrk,
        "1.csv",
        "\
h1,h2
c,d
"
    );
    split_eq!(
        wrk,
        "2.csv",
        "\
h1,h2
e,f
"
    );
    split_eq!(
        wrk,
        "3.csv",
        "\
h1,h2
g,h
"
    );
    split_eq!(
        wrk,
        "4.csv",
        "\
h1,h2
i,j
"
    );
    split_eq!(
        wrk,
        "5.csv",
        "\
h1,h2
k,l
"
    );
}

#[test]
fn split_uneven() {
    let wrk = Workdir::new("split_uneven");
    wrk.create("in.csv", data(true));

    let mut cmd = wrk.command("split");
    cmd.args(["--size", "4"]).arg(&wrk.path(".")).arg("in.csv");
    wrk.run(&mut cmd);

    split_eq!(
        wrk,
        "0.csv",
        "\
h1,h2
a,b
c,d
e,f
g,h
"
    );
    split_eq!(
        wrk,
        "4.csv",
        "\
h1,h2
i,j
k,l
"
    );
}

#[test]
fn split_uneven_idx() {
    let wrk = Workdir::new("split_uneven_idx");
    wrk.create_indexed("in.csv", data(true));

    let mut cmd = wrk.command("split");
    cmd.args(["--size", "4"]).arg(&wrk.path(".")).arg("in.csv");
    wrk.run(&mut cmd);

    split_eq!(
        wrk,
        "0.csv",
        "\
h1,h2
a,b
c,d
e,f
g,h
"
    );
    split_eq!(
        wrk,
        "4.csv",
        "\
h1,h2
i,j
k,l
"
    );
}

#[test]
fn split_custom_filename() {
    let wrk = Workdir::new("split");
    wrk.create("in.csv", data(true));

    let mut cmd = wrk.command("split");
    cmd.args(["--size", "2"])
        .args(["--filename", "prefix-{}.csv"])
        .arg(&wrk.path("."))
        .arg("in.csv");
    wrk.run(&mut cmd);

    assert!(wrk.path("prefix-0.csv").exists());
    assert!(wrk.path("prefix-2.csv").exists());
    assert!(wrk.path("prefix-4.csv").exists());
}

#[test]
fn split_custom_filename_padded() {
    let wrk = Workdir::new("split");
    wrk.create("in.csv", data(true));

    let mut cmd = wrk.command("split");
    cmd.args(["--size", "2"])
        .arg("--pad")
        .arg("3")
        .args(["--filename", "prefix-{}.csv"])
        .arg(&wrk.path("."))
        .arg("in.csv");
    wrk.run(&mut cmd);

    assert!(wrk.path("prefix-000.csv").exists());
    assert!(wrk.path("prefix-002.csv").exists());
    assert!(wrk.path("prefix-004.csv").exists());
}

#[test]
fn split_nooutdir() {
    let wrk = Workdir::new("split_nooutdir");
    wrk.create("in.csv", data(true));

    let mut cmd = wrk.command("split");
    cmd.args(["--size", "2"]).arg("in.csv");
    wrk.run(&mut cmd);

    wrk.assert_err(&mut cmd);
    let got = wrk.output_stderr(&mut cmd);
    let expected = "usage error: <outdir> is not specified or is a file.\n";
    assert_eq!(got, expected);
}
