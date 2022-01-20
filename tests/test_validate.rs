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
    cmd.arg("data.csv");

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
    cmd.arg("data.csv");

    wrk.assert_err(&mut cmd);

}

#[test]
fn validate_with_json_schema() {

}

