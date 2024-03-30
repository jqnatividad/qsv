use newline_converter::dos2unix;
use serial_test::serial;

use crate::workdir::Workdir;

#[test]
#[serial]
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
#[serial]
fn tojsonl_boolean() {
    let wrk = Workdir::new("tojsonl");
    wrk.create(
        "in.csv",
        vec![
            svec!["col1", "col2"],
            svec!["true", "Mark"],
            svec!["false", "John"],
            svec!["false", "Bob"],
        ],
    );

    let mut cmd = wrk.command("tojsonl");
    cmd.arg("in.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"{"col1":true,"col2":"Mark"}
{"col1":false,"col2":"John"}
{"col1":false,"col2":"Bob"}"#;
    assert_eq!(got, expected);
}

#[test]
#[serial]
fn tojsonl_boolean_tf() {
    let wrk = Workdir::new("tojsonl");
    wrk.create(
        "in.csv",
        vec![
            svec!["col1", "col2"],
            svec!["t", "Mark"],
            svec!["f", "John"],
            svec!["f", "Bob"],
        ],
    );

    let mut cmd = wrk.command("tojsonl");
    cmd.arg("in.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"{"col1":true,"col2":"Mark"}
{"col1":false,"col2":"John"}
{"col1":false,"col2":"Bob"}"#;
    assert_eq!(got, expected);
}

#[test]
#[serial]
fn tojsonl_boolean_upper_tf() {
    let wrk = Workdir::new("tojsonl");
    wrk.create(
        "in.csv",
        vec![
            svec!["col1", "col2"],
            svec!["T", "Mark"],
            svec!["F", "John"],
            svec!["F", "Bob"],
        ],
    );

    let mut cmd = wrk.command("tojsonl");
    cmd.arg("in.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"{"col1":true,"col2":"Mark"}
{"col1":false,"col2":"John"}
{"col1":false,"col2":"Bob"}"#;
    assert_eq!(got, expected);
}

#[test]
#[serial]
fn tojsonl_boolean_1or0() {
    let wrk = Workdir::new("tojsonl");
    wrk.create(
        "in.csv",
        vec![
            svec!["col1", "col2"],
            svec!["1", "Mark"],
            svec!["0", "John"],
            svec!["0", "Bob"],
        ],
    );

    let mut cmd = wrk.command("tojsonl");
    cmd.arg("in.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"{"col1":true,"col2":"Mark"}
{"col1":false,"col2":"John"}
{"col1":false,"col2":"Bob"}"#;
    assert_eq!(got, expected);
}
#[test]
#[serial]
fn tojsonl_noboolean_1or0() {
    let wrk = Workdir::new("tojsonl_noboolean_1or0");
    wrk.create(
        "in.csv",
        vec![
            svec!["col1", "col2"],
            svec!["1", "Mark"],
            svec!["0", "John"],
            svec!["0", "Bob"],
        ],
    );

    let mut cmd = wrk.command("tojsonl");
    cmd.arg("--no-boolean").arg("in.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"{"col1":1,"col2":"Mark"}
{"col1":0,"col2":"John"}
{"col1":0,"col2":"Bob"}"#;
    assert_eq!(got, expected);
}

#[test]
#[serial]
fn tojsonl_noboolean_tworecords() {
    let wrk = Workdir::new("tojsonl_noboolean_tworecords");
    wrk.create(
        "in.csv",
        vec![
            svec!["col1", "col2"],
            svec!["1", "Mark"],
            svec!["0", "John"],
        ],
    );

    let mut cmd = wrk.command("tojsonl");
    cmd.arg("in.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"{"col1":1,"col2":"Mark"}
{"col1":0,"col2":"John"}"#;
    assert_eq!(got, expected);
}

#[test]
#[serial]
fn tojsonl_boolean_1or0_false_positive_handling() {
    let wrk = Workdir::new("tojsonl");
    wrk.create(
        "in.csv",
        vec![
            svec!["col1", "col2"],
            svec!["15", "Mark"],
            svec!["02", "John"],
            svec!["02", "Bob"],
            svec!["15", "Mary"],
        ],
    );

    let mut cmd = wrk.command("tojsonl");
    cmd.arg("in.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"{"col1":"15","col2":"Mark"}
{"col1":"02","col2":"John"}
{"col1":"02","col2":"Bob"}
{"col1":"15","col2":"Mary"}"#;
    assert_eq!(got, expected);
}

#[test]
#[serial]
fn tojsonl_not_boolean_case_sensitive() {
    let wrk = Workdir::new("tojsonl");
    wrk.create(
        "in.csv",
        vec![
            svec!["col1", "col2"],
            svec!["True", "Mark"],
            svec!["False", "John"],
            svec!["false", "Bob"],
            svec!["TRUE", "Mary"],
        ],
    );

    let mut cmd = wrk.command("tojsonl");
    cmd.arg("in.csv");

    // properly treated as boolean since col1's domain has two values
    // case-insensitive, even though the enum for col1 is
    // True, False, false and TRUE
    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"{"col1":true,"col2":"Mark"}
{"col1":false,"col2":"John"}
{"col1":false,"col2":"Bob"}
{"col1":true,"col2":"Mary"}"#;
    assert_eq!(got, expected);
}

#[test]
#[serial]
fn tojsonl_is_boolean_case_sensitive() {
    let wrk = Workdir::new("tojsonl");
    wrk.create(
        "in.csv",
        vec![
            svec!["col1", "col2"],
            svec!["True", "Mark"],
            svec!["False", "John"],
            svec!["False", "Bob"],
        ],
    );

    let mut cmd = wrk.command("tojsonl");
    cmd.arg("in.csv");

    // this is treated as boolean since col1's domain has two values
    // True and False
    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"{"col1":true,"col2":"Mark"}
{"col1":false,"col2":"John"}
{"col1":false,"col2":"Bob"}"#;
    assert_eq!(got, expected);
}

#[test]
#[serial]
fn tojsonl_boolean_yes() {
    let wrk = Workdir::new("tojsonl");
    wrk.create(
        "in.csv",
        vec![
            svec!["col1", "col2"],
            svec!["yes", "Mark"],
            svec!["no", "John"],
            svec!["no", "Bob"],
        ],
    );

    let mut cmd = wrk.command("tojsonl");
    cmd.arg("in.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"{"col1":true,"col2":"Mark"}
{"col1":false,"col2":"John"}
{"col1":false,"col2":"Bob"}"#;
    assert_eq!(got, expected);
}

#[test]
#[serial]
fn tojsonl_boolean_null() {
    let wrk = Workdir::new("tojsonl");
    wrk.create(
        "in.csv",
        vec![
            svec!["col1", "col2"],
            svec!["true", "Mark"],
            svec!["", "John"],
            svec!["", "Bob"],
        ],
    );

    let mut cmd = wrk.command("tojsonl");
    cmd.arg("in.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"{"col1":true,"col2":"Mark"}
{"col1":false,"col2":"John"}
{"col1":false,"col2":"Bob"}"#;
    assert_eq!(got, expected);
}

#[test]
#[serial]
fn tojsonl_boolean_y_null() {
    let wrk = Workdir::new("tojsonl");
    wrk.create(
        "in.csv",
        vec![
            svec!["col1", "col2"],
            svec!["y", "Mark"],
            svec!["", "John"],
            svec!["", "Bob"],
            svec!["y", "Mary"],
        ],
    );

    let mut cmd = wrk.command("tojsonl");
    cmd.arg("in.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"{"col1":true,"col2":"Mark"}
{"col1":false,"col2":"John"}
{"col1":false,"col2":"Bob"}
{"col1":true,"col2":"Mary"}"#;
    assert_eq!(got, expected);
}

#[test]
#[serial]
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

#[test]
#[serial]
fn tojsonl_boston() {
    let wrk = Workdir::new("tojsonl");
    let test_file = wrk.load_test_file("boston311-100.csv");

    let mut cmd = wrk.command("tojsonl");
    cmd.arg(test_file);

    let got: String = wrk.stdout(&mut cmd);

    let expected = wrk.load_test_resource("boston311-100-untrimmed.jsonl");

    assert_eq!(dos2unix(&got), dos2unix(&expected).trim_end());
}

#[test]
#[serial]
fn tojsonl_boston_snappy() {
    let wrk = Workdir::new("tojsonl");
    let test_file = wrk.load_test_file("boston311-100.csv.sz");

    let mut cmd = wrk.command("tojsonl");
    cmd.arg(test_file);

    let got: String = wrk.stdout(&mut cmd);

    let expected = wrk.load_test_resource("boston311-100-untrimmed.jsonl");

    assert_eq!(dos2unix(&got), dos2unix(&expected).trim_end());
}

#[test]
#[serial]
fn tojsonl_boston_trim() {
    let wrk = Workdir::new("tojsonl");
    let test_file = wrk.load_test_file("boston311-100.csv");

    let mut cmd = wrk.command("tojsonl");
    cmd.arg(test_file).arg("--trim");

    let got: String = wrk.stdout(&mut cmd);

    let expected = wrk.load_test_resource("boston311-100.jsonl");

    assert_eq!(dos2unix(&got), dos2unix(&expected).trim_end());
}

#[test]
fn tojsonl_issue_1649_false_positive_tf() {
    let wrk = Workdir::new("tojsonl_issue_1649_false_positive_tf");
    wrk.create(
        "in.csv",
        vec![
            svec!["id", "name"],
            svec!["1", "François Hollande"],
            svec!["2", "Tarja Halonen"],
        ],
    );

    let mut cmd = wrk.command("tojsonl");
    cmd.arg("in.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"{"id":1,"name":"François Hollande"}
{"id":2,"name":"Tarja Halonen"}"#;

    assert_eq!(got, expected);
}

#[test]
fn tojsonl_issue_1649_false_positive_tf_3recs() {
    let wrk = Workdir::new("tojsonl_issue_1649_false_positive_tf_3_recs");
    wrk.create(
        "in.csv",
        vec![
            svec!["id", "name"],
            svec!["1", "Fanuel"],
            svec!["2", "Travis"],
            svec!["3", "Travis"],
        ],
    );

    let mut cmd = wrk.command("tojsonl");
    cmd.arg("in.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"{"id":1,"name":"Fanuel"}
{"id":2,"name":"Travis"}
{"id":3,"name":"Travis"}"#;

    assert_eq!(got, expected);
}
