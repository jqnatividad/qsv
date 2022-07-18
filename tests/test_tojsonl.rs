use crate::workdir::Workdir;

#[test]
fn tojsonl_simple() {
    let wrk = Workdir::new("tojsonl_simple");
    wrk.create(
        "in.csv",
        vec![
            svec!["id", "father", "mother", "oldest_child", "boy"],
            svec!["1", "Mark", "Charlotte", "Tom", "true"],
            svec!["2", "John", "Ann", "Jessika", "false"],
            svec!["3", "Bob", "Monika", "Jerry", "true"],
        ],
    );

    let mut cmd = wrk.command("tojsonl");
    cmd.arg("in.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"{"id":"1","father":"Mark","mother":"Charlotte","oldest_child":"Tom","boy":"true"}
{"id":"2","father":"John","mother":"Ann","oldest_child":"Jessika","boy":"false"}
{"id":"3","father":"Bob","mother":"Monika","oldest_child":"Jerry","boy":"true"}"#;
    assert_eq!(got, expected);
}

#[test]
fn tojsonl_nested() {
    let wrk = Workdir::new("tojsonl_nested");
    wrk.create(
        "in.csv",
        vec![
            svec!["id", "father", "mother", "children"],
            svec!["1", "Mark", "Charlotte", "\"Tom\""],
            svec!["2", "John", "Ann", "\"Jessika\",\"Antony\",\"Jack\""],
            svec!["3", "Bob", "Monika", "\"Jerry\",\"Karol\""],
        ],
    );

    let mut cmd = wrk.command("tojsonl");
    cmd.arg("in.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"{"id":"1","father":"Mark","mother":"Charlotte","children":"\"Tom\""}
{"id":"2","father":"John","mother":"Ann","children":"\"Jessika\",\"Antony\",\"Jack\""}
{"id":"3","father":"Bob","mother":"Monika","children":"\"Jerry\",\"Karol\""}"#;

    assert_eq!(got, expected);
}
