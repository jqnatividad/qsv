use crate::workdir::Workdir;

#[test]
fn jsonp_simple() {
    let wrk = Workdir::new("jsonp_simple");
    wrk.create_from_string(
        "data.json",
        r#"[{"id":1,"father":"Mark","mother":"Charlotte","oldest_child":"Tom","boy":true},
{"id":2,"father":"John","mother":"Ann","oldest_child":"Jessika","boy":false},
{"id":3,"father":"Bob","mother":"Monika","oldest_child":"Jerry","boy":true}]"#,
    );
    let mut cmd = wrk.command("jsonp");
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
fn jsonp_fruits_stats() {
    let wrk = Workdir::new("jsonp_fruits_stats");
    wrk.create_from_string(
        "data.json",
        r#"[{"field":"fruit","type":"String","is_ascii":true,"sum":null,"min":"apple","max":"strawberry","range":null,"min_length":5,"max_length":10,"mean":null,"stddev":null,"variance":null,"nullcount":0,"max_precision":null,"sparsity":0},{"field":"price","type":"Float","is_ascii":null,"sum":7,"min":"1.5","max":"3.0","range":1.5,"min_length":4,"max_length":4,"mean":2.3333,"stddev":0.6236,"variance":0.3889,"nullcount":0,"max_precision":1,"sparsity":0}]"#,
    );
    let mut cmd = wrk.command("jsonp");
    cmd.arg("data.json");

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"field,type,is_ascii,sum,min,max,range,min_length,max_length,mean,stddev,variance,nullcount,max_precision,sparsity
fruit,String,true,,apple,strawberry,,5,10,,,,0,,0
price,Float,,7,1.5,3.0,1.5,4,4,2.3333,0.6236,0.3889,0,1,0"#.to_string();
    assert_eq!(got, expected);
}

#[test]
fn jsonp_fruits_stats_fp_2() {
    let wrk = Workdir::new("jsonp_fruits_stats_fp_2");
    wrk.create_from_string(
        "data.json",
        r#"[{"field":"fruit","type":"String","is_ascii":true,"sum":null,"min":"apple","max":"strawberry","range":null,"min_length":5,"max_length":10,"mean":null,"stddev":null,"variance":null,"nullcount":0,"max_precision":null,"sparsity":0},{"field":"price","type":"Float","is_ascii":null,"sum":7,"min":"1.5","max":"3.0","range":1.5,"min_length":4,"max_length":4,"mean":2.3333,"stddev":0.6236,"variance":0.3889,"nullcount":0,"max_precision":1,"sparsity":0}]"#,
    );
    let mut cmd = wrk.command("jsonp");
    cmd.arg("data.json");
    cmd.args(&["--float-precision", "2"]);

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"field,type,is_ascii,sum,min,max,range,min_length,max_length,mean,stddev,variance,nullcount,max_precision,sparsity
fruit,String,true,,apple,strawberry,,5,10,,,,0,,0
price,Float,,7,1.5,3.0,1.50,4,4,2.33,0.62,0.39,0,1,0"#.to_string();
    assert_eq!(got, expected);
}

#[test]
// Verify that qsv stats fruits.csv has the same content as
// qsv stats fruits.csv | qsv slice --json | qsv jsonp
fn jsonp_fruits_stats_slice_jsonp() {
    let wrk = Workdir::new("jsonp_fruits_stats_slice_jsonp");
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

    // qsv jsonp
    let mut jsonp_cmd = wrk.command("jsonp");
    jsonp_cmd.arg("slice.json");
    let jsonp_output: String = wrk.stdout(&mut jsonp_cmd);

    assert_eq!(stats_output, jsonp_output);
}
