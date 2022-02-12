use crate::workdir::Workdir;
use assert_json_diff::assert_json_eq;
use serde_json::Value;

#[test]
fn generate_schema_with_value_constraints_then_feed_into_validate() {

    // create worksapce and invoke schema command with value constraints flag
    let wrk = Workdir::new("schema").flexible(true);

    // copy csv file to workdir
    let csv = wrk.load_test_resource("adur-public-toilets.csv");
    wrk.create_from_string("adur-public-toilets.csv", &csv);

    // run schema command with value constraints option
    let mut cmd = wrk.command("schema");
    cmd.arg("adur-public-toilets.csv");
    cmd.arg("--enum-threshold");
    cmd.arg("13");
    wrk.output(&mut cmd);

    // load output schema file
    let output_schema_string: String =
        wrk.from_str(&wrk.path("adur-public-toilets.csv.schema.json"));
    let output_schema_json =
        serde_json::from_str(&output_schema_string).expect("parse schema json");

    // make sure it's a valid JSON Schema by compiling with jsonschema library
    jsonschema::JSONSchema::options()
        .compile(&output_schema_json)
        .expect("valid JSON Schema");

    // diff output json with expected json
    let expected_schema: String = wrk.load_test_resource("adur-public-toilets.csv.schema-with-value-constraints.expected.json");
    let expected_schema_json: Value = serde_json::from_str(&expected_schema.to_string()).unwrap();
    assert_json_eq!(expected_schema_json, output_schema_json);

    // invoke validate command from schema created above
    let mut cmd2 = wrk.command("validate");
    cmd2.arg("adur-public-toilets.csv");
    cmd2.arg("adur-public-toilets.csv.schema.json");
    wrk.output(&mut cmd2);

    // validation report
    let validation_errors_expected = r#"{"valid":false,"errors":[{"keywordLocation":"/properties/ExtractDate/type","instanceLocation":"/ExtractDate","error":"null is not of type \"string\""},{"keywordLocation":"/properties/ExtractDate/enum","instanceLocation":"/ExtractDate","error":"null is not one of [\"07/07/2014 00:00\",\"2014-07-07 00:00\"]"}],"row_index":1}
"#;

    // check validation error output
    let validation_error_output: String =
        wrk.from_str(&wrk.path("adur-public-toilets.csv.validation-errors.jsonl"));

    assert!(validation_error_output.len() > 0);

    assert_eq!(
        validation_errors_expected.to_string(),
        validation_error_output
    );
}
