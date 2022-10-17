use crate::workdir::Workdir;

#[test]
fn tojsonl_simple() {
    let wrk = Workdir::new("tojsonl_simple");
    wrk.create(
        "in.csv",
        vec![
            svec!["id", "father", "mother", "oldest_child", "boy", "weight"],
            svec!["1", "Mark", "Charlotte", "Tom", "true", "150.2"],
            svec!["2", "John", "Ann", "Jessika", "false", "175.5"],
            svec!["3", "Bob", "Monika", "Jerry", "true", "199.5"],
        ],
    );

    let mut cmd = wrk.command("tojsonl");
    cmd.arg("in.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"{"id":1,"father":"Mark","mother":"Charlotte","oldest_child":"Tom","boy":true,"weight":150.2}
{"id":2,"father":"John","mother":"Ann","oldest_child":"Jessika","boy":false,"weight":175.5}
{"id":3,"father":"Bob","mother":"Monika","oldest_child":"Jerry","boy":true,"weight":199.5}"#;
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
            svec![
                "4",
                "John\nSmith",
                "Jane \"Smiley\" Doe",
                "\"Jack\",\"Jill\r\n \"Climber\""
            ],
        ],
    );

    let mut cmd = wrk.command("tojsonl");
    cmd.arg("in.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"{"id":1,"father":"Mark","mother":"Charlotte","children":"\"Tom\""}
{"id":2,"father":"John","mother":"Ann","children":"\"Jessika\",\"Antony\",\"Jack\""}
{"id":3,"father":"Bob","mother":"Monika","children":"\"Jerry\",\"Karol\""}
{"id":4,"father":"John\nSmith","mother":"Jane \"Smiley\" Doe","children":"\"Jack\",\"Jill\r\n \"Climber\""}"#;

    assert_eq!(got, expected);
}
