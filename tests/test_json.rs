use crate::workdir::Workdir;

#[test]
fn json_simple() {
    let wrk = Workdir::new("json_simple");
    wrk.create_from_string(
        "data.json",
        r#"[{"id":1,"father":"Mark","mother":"Charlotte","oldest_child":"Tom","boy":true},
{"id":2,"father":"John","mother":"Ann","oldest_child":"Jessika","boy":false},
{"id":3,"father":"Bob","mother":"Monika","oldest_child":"Jerry","boy":true}]"#,
    );
    let mut cmd = wrk.command("json");
    cmd.arg("data.json");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["id", "father", "mother", "oldest_child", "boy"],
        svec!["1", "Mark", "Charlotte", "Tom", "true"],
        svec!["2", "John", "Ann", "Jessika", "false"],
        svec!["3", "Bob", "Monika", "Jerry", "true"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn json_fruits_stats() {
    let wrk = Workdir::new("json_fruits_stats");
    wrk.create_from_string(
        "data.json",
        r#"[{"field":"fruit","type":"String","is_ascii":true,"sum":null,"min":"apple","max":"strawberry","range":null,"min_length":5,"max_length":10,"mean":null,"stddev":null,"variance":null,"nullcount":0,"max_precision":null,"sparsity":0},{"field":"price","type":"Float","is_ascii":null,"sum":7,"min":"1.5","max":"3.0","range":1.5,"min_length":4,"max_length":4,"mean":2.3333,"stddev":0.6236,"variance":0.3889,"nullcount":0,"max_precision":1,"sparsity":0}]"#,
    );
    let mut cmd = wrk.command("json");
    cmd.arg("data.json");

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"field,type,is_ascii,sum,min,max,range,min_length,max_length,mean,stddev,variance,nullcount,max_precision,sparsity
fruit,String,true,,apple,strawberry,,5,10,,,,0,,0
price,Float,,7,1.5,3.0,1.5,4,4,2.3333,0.6236,0.3889,0,1,0"#.to_string();
    assert_eq!(got, expected);
}

#[test]
// Verify that qsv stats fruits.csv has the same content as
// qsv stats fruits.csv | qsv slice --json | qsv json
fn json_fruits_stats_slice_json() {
    let wrk = Workdir::new("json_fruits_stats_slice_json");
    let test_file = wrk.load_test_file("fruits.csv");

    // qsv stats fruits.csv
    let mut stats_cmd = wrk.command("stats");
    stats_cmd.arg(test_file);
    let stats_output: String = wrk.stdout(&mut stats_cmd);
    wrk.create_from_string("stats.csv", stats_output.as_str());

    // qsv slice --json
    let mut slice_cmd = wrk.command("slice");
    slice_cmd.arg("stats.csv");
    slice_cmd.arg("--json");
    let slice_output: String = wrk.stdout(&mut slice_cmd);
    wrk.create_from_string("slice.json", slice_output.as_str());

    // qsv json
    let mut json_cmd = wrk.command("json");
    json_cmd.arg("slice.json");
    let json_output: String = wrk.stdout(&mut json_cmd);

    assert_eq!(stats_output, json_output);
}

#[test]
// Verify that qsv stats House.csv has the same content as
// qsv stats House.csv | qsv slice --json | qsv json
fn json_house_stats_slice_json() {
    let wrk = Workdir::new("json_house_stats_slice_json");
    let test_file = wrk.load_test_file("House.csv");

    // qsv stats fruits.csv
    let mut stats_cmd = wrk.command("stats");
    stats_cmd.arg(test_file);
    let stats_output: String = wrk.stdout(&mut stats_cmd);
    wrk.create_from_string("stats.csv", stats_output.as_str());

    // qsv slice --json
    let mut slice_cmd = wrk.command("slice");
    slice_cmd.arg("stats.csv");
    slice_cmd.arg("--json");
    let slice_output: String = wrk.stdout(&mut slice_cmd);
    wrk.create_from_string("slice.json", slice_output.as_str());

    // qsv json
    let mut json_cmd = wrk.command("json");
    json_cmd.arg("slice.json");
    let json_output: String = wrk.stdout(&mut json_cmd);

    assert_eq!(stats_output, json_output);
}

#[test]
// Verify that House.csv has the same content as
// qsv slice House.csv --json | qsv json
// according to qsv diff
fn json_house_diff() {
    let wrk = Workdir::new("json_house_diff");
    let _ = wrk.load_test_file("House.csv");

    // qsv enum House.csv -o House_enum.csv
    let mut enum1_cmd = wrk.command("enum");
    enum1_cmd.arg("House.csv");
    let enum1_output: String = wrk.stdout(&mut enum1_cmd);
    wrk.create_from_string("House_enum.csv", enum1_output.as_str());

    // qsv slice --json
    let mut slice_cmd = wrk.command("slice");
    slice_cmd.arg("House.csv");
    slice_cmd.arg("--json");
    let slice_output: String = wrk.stdout(&mut slice_cmd);
    wrk.create_from_string("slice.json", slice_output.as_str());

    // qsv json
    let mut json_cmd = wrk.command("json");
    json_cmd.arg("slice.json");
    let json_output: String = wrk.stdout(&mut json_cmd);
    wrk.create_from_string("House2.csv", json_output.as_str());

    // qsv enum House2.csv -o House2_enum.csv
    let mut enum2_cmd = wrk.command("enum");
    enum2_cmd.arg("House2.csv");
    let enum2_output: String = wrk.stdout(&mut enum2_cmd);
    wrk.create_from_string("House2_enum.csv", enum2_output.as_str());

    // qsv diff House.csv House2.csv -k 2
    let mut diff_cmd = wrk.command("diff");
    diff_cmd.args(vec!["House.csv", "House2.csv", "-k", "2"]);
    let diff_output: String = wrk.stdout(&mut diff_cmd);

    assert!(diff_output.lines().count() == 1);
}
