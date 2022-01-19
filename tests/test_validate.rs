use crate::workdir::Workdir;

#[test]
fn validate_good_csv() {
    let wrk = Workdir::new("fetch").flexible(true);;
    wrk.create(
        "data.csv",
        vec![
            svec!["firstName", "lastName", "age"],
            svec!["Mickey", "Mouse", "10"],
            svec!["Donald", "Duck", "8"],
            svec!["Minie", "Mouse", "9"],
            ],
    );
    let mut cmd = wrk.command("validate");

    wrk.output(&mut cmd);
}

#[test]
fn validate_bad_csv() {
    let wrk = Workdir::new("fetch").flexible(true);
    wrk.create(
        "data.csv",
        vec![
            svec!["firstName", "lastName", "age"],
            svec!["Mickey", "Mouse", "10"],
            svec!["Donald", "Duck"],
            svec!["Minie", "Mouse", "9"],
            ],
    );
    let mut cmd = wrk.command("validate");
    // quiet flag required to avoid progress counter from panic due to using csv::reader without flexible flag
    cmd.arg("--quiet");

    // for some reason asserting error doesn't work here, but on cmd `echo $?` shows 1
    // this fails
    wrk.assert_err(&mut cmd);

    // but this passes
    // wrk.output(&mut cmd);
}

#[test]
fn validate_with_json_schema() {

}

