use crate::workdir::Workdir;

#[test]
fn edit_by_col_name() {
    let wrk = Workdir::new("edit_by_col_name");
    wrk.create(
        "data.csv",
        vec![svec!["letter", "number"], svec!["a", "1"], svec!["b", "2"]],
    );

    let mut cmd = wrk.command("edit");
    cmd.arg("data.csv");
    cmd.arg("number");
    cmd.arg("0");
    cmd.arg("3");

    let got: String = wrk.stdout(&mut cmd);
    let expected = "letter,number
a,3
b,2"
    .to_string();
    assert_eq!(got, expected);
}

#[test]
fn edit_by_col_index() {
    let wrk = Workdir::new("edit_by_col_index");
    wrk.create(
        "data.csv",
        vec![svec!["letter", "number"], svec!["a", "1"], svec!["b", "2"]],
    );

    let mut cmd = wrk.command("edit");
    cmd.arg("data.csv");
    cmd.arg("1");
    cmd.arg("0");
    cmd.arg("3");

    let got: String = wrk.stdout(&mut cmd);
    let expected = "letter,number
a,3
b,2"
    .to_string();
    assert_eq!(got, expected);
}

#[test]
fn edit_first_header() {
    let wrk = Workdir::new("edit_first_header");
    wrk.create_from_string(
        "data.csv",
        "letter,number
a,1
b,2",
    );

    let mut cmd = wrk.command("edit");
    cmd.arg("data.csv");
    cmd.arg("letter");
    cmd.arg("0");
    cmd.arg("character");
    cmd.arg("--no-headers");

    let got: String = wrk.stdout(&mut cmd);
    let expected = "character,number
a,1
b,2"
    .to_string();
    assert_eq!(got, expected);
}

#[test]
fn edit_column_index_priority() {
    let wrk = Workdir::new("edit_column_index_priority");
    wrk.create_from_string(
        "data.csv",
        "letter,number,0
a,1,x
b,2,y",
    );

    let mut cmd = wrk.command("edit");
    cmd.arg("data.csv");
    cmd.arg("0");
    cmd.arg("0");
    cmd.arg("z");

    let got: String = wrk.stdout(&mut cmd);
    let expected = "letter,number,0
z,1,x
b,2,y"
        .to_string();
    assert_eq!(got, expected);
}

#[test]
fn edit_by_col_underscore() {
    let wrk = Workdir::new("edit_by_col_underscore");
    wrk.create(
        "data.csv",
        vec![svec!["letter", "number"], svec!["a", "1"], svec!["b", "2"]],
    );

    let mut cmd = wrk.command("edit");
    cmd.arg("data.csv");
    cmd.arg("_");
    cmd.arg("0");
    cmd.arg("3");

    let got: String = wrk.stdout(&mut cmd);
    let expected = "letter,number
a,3
b,2"
    .to_string();
    assert_eq!(got, expected);
}
