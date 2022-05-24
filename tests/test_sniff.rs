use crate::workdir::Workdir;

static EXPECTED_TABLE: &str = "\
h1       h2  h3
abcdefg  1   a
a        2   z\
";

fn data() -> Vec<Vec<String>> {
    vec![
        svec!["h1", "h2", "h3"],
        svec!["abcdefg", "1", "a"],
        svec!["a", "2", "z"],
    ]
}

#[test]
fn sniff() {
    let wrk = Workdir::new("sniff");
    wrk.create_with_delim("in.file", data(), b',');

    let mut cmd = wrk.command("sniff");
    cmd.arg("in.file");

    let got: String = wrk.stdout(&mut cmd);

    let expected = r#"Metadata
========
	Delimiter: ,
	Has header row?: true
	Number of preamble rows: 0
	Quote character: none
	Flexible: false
	Is utf-8 encoded?: true

Number of records: 2
Number of fields: 3
Fields:
    0:  Text      h1
    1:  Unsigned  h2
    2:  Text      h3"#;

    assert_eq!(got, expected);
}

#[test]
fn sniff_tab() {
    let wrk = Workdir::new("sniff_tab");
    wrk.create_with_delim("in.file", data(), b'\t');

    let mut cmd = wrk.command("sniff");
    cmd.arg("in.file");

    let got: String = wrk.stdout(&mut cmd);

    let expected = r#"Metadata
========
	Delimiter: tab
	Has header row?: true
	Number of preamble rows: 0
	Quote character: none
	Flexible: false
	Is utf-8 encoded?: true

Number of records: 2
Number of fields: 3
Fields:
    0:  Text      h1
    1:  Unsigned  h2
    2:  Text      h3"#;

    assert_eq!(got, expected);
}

#[test]
fn qsv_sniff_pipe_delimiter_env() {
    let wrk = Workdir::new("qsv_sniff_pipe_delimiter_env");
    wrk.create_with_delim("in.file", data(), b'|');

    let mut cmd = wrk.command("table");
    cmd.env("QSV_SNIFF_DELIMITER", "1");
    cmd.arg("in.file");

    let got: String = wrk.stdout(&mut cmd);
    assert_eq!(&*got, EXPECTED_TABLE)
}

#[test]
fn qsv_sniff_semicolon_delimiter_env() {
    let wrk = Workdir::new("qsv_sniff_semicolon_delimiter_env");
    wrk.create_with_delim("in.file", data(), b';');

    let mut cmd = wrk.command("table");
    cmd.env("QSV_SNIFF_DELIMITER", "1");
    cmd.arg("in.file");

    let got: String = wrk.stdout(&mut cmd);
    assert_eq!(&*got, EXPECTED_TABLE)
}

#[test]
fn qsv_sniff_tab_delimiter_env() {
    let wrk = Workdir::new("qsv_sniff_tab_delimiter_env");
    wrk.create_with_delim("in.file", data(), b'\t');

    let mut cmd = wrk.command("table");
    cmd.env("QSV_SNIFF_DELIMITER", "1");
    cmd.arg("in.file");

    let got: String = wrk.stdout(&mut cmd);
    assert_eq!(&*got, EXPECTED_TABLE)
}

#[test]
fn sniff_json() {
    let wrk = Workdir::new("sniff_json");
    let test_file = wrk.load_test_file("snifftest.csv");

    let mut cmd = wrk.command("sniff");
    cmd.arg("--json").arg(test_file);

    let got: String = wrk.stdout(&mut cmd);

    let expected = r#"{"delimiter_char":",","header_row":true,"preamble_rows":3,"quote_char":"none","flexible":false,"is_utf8":true,"num_records":3,"num_fields":4,"fields":["h1","h2","h3","h4"],"types":["Text","Unsigned","Text","Float"]}"#;
    assert_eq!(got, expected);
}

#[test]
fn sniff_flexible_json() {
    let wrk = Workdir::new("sniff_flexible_json");
    let test_file = wrk.load_test_file("snifftest-flexible.csv");

    let mut cmd = wrk.command("sniff");
    cmd.arg("--json").arg(test_file);

    let got: String = wrk.stdout(&mut cmd);

    let expected = r#"{"delimiter_char":",","header_row":true,"preamble_rows":3,"quote_char":"none","flexible":true,"is_utf8":true,"num_records":5,"num_fields":4,"fields":["h1","h2","h3","h4"],"types":["Text","Unsigned","Text","Float"]}"#;
    assert_eq!(got, expected);
}

#[test]
fn sniff_pretty_json() {
    let wrk = Workdir::new("sniff_pretty_json");
    let test_file = wrk.load_test_file("snifftest.csv");

    let mut cmd = wrk.command("sniff");
    cmd.arg("--pretty-json").arg(test_file);

    let got: String = wrk.stdout(&mut cmd);

    let expected = r#"{
  "delimiter_char": ",",
  "header_row": true,
  "preamble_rows": 3,
  "quote_char": "none",
  "flexible": false,
  "is_utf8": true,
  "num_records": 3,
  "num_fields": 4,
  "fields": [
    "h1",
    "h2",
    "h3",
    "h4"
  ],
  "types": [
    "Text",
    "Unsigned",
    "Text",
    "Float"
  ]
}"#;

    assert_eq!(got, expected);
}


#[test]
fn sniff_sample() {
    let wrk = Workdir::new("sniff_sample");
    let test_file = wrk.load_test_file("adur-public-toilets.csv");

    let mut cmd = wrk.command("sniff");
    cmd.arg("--pretty-json").arg("--sample").arg("0.25").arg(test_file);

    let got: String = wrk.stdout(&mut cmd);

    let expected = r#"{
  "delimiter_char": ",",
  "header_row": true,
  "preamble_rows": 0,
  "quote_char": "none",
  "flexible": false,
  "is_utf8": true,
  "num_records": 15,
  "num_fields": 32,
  "fields": [
    "ExtractDate",
    "OrganisationURI",
    "OrganisationLabel",
    "ServiceTypeURI",
    "ServiceTypeLabel",
    "LocationText",
    "CoordinateReferenceSystem",
    "GeoX",
    "GeoY",
    "GeoPointLicensingURL",
    "Category",
    "AccessibleCategory",
    "RADARKeyNeeded",
    "BabyChange",
    "FamilyToilet",
    "ChangingPlace",
    "AutomaticPublicConvenience",
    "FullTimeStaffing",
    "PartOfCommunityScheme",
    "CommunitySchemeName",
    "ChargeAmount",
    "InfoURL",
    "OpeningHours",
    "ManagedBy",
    "ReportEmail",
    "ReportTel",
    "Notes",
    "UPRN",
    "Postcode",
    "StreetAddress",
    "GeoAreaURI",
    "GeoAreaLabel"
  ],
  "types": [
    "Text",
    "Text",
    "Text",
    "Text",
    "Text",
    "Text",
    "Text",
    "Unsigned",
    "Unsigned",
    "Text",
    "Text",
    "Text",
    "Text",
    "Text",
    "Text",
    "Text",
    "Text",
    "Text",
    "Text",
    "Boolean",
    "Boolean",
    "Text",
    "Text",
    "Text",
    "Text",
    "Text",
    "Text",
    "Unsigned",
    "Boolean",
    "Text",
    "Boolean",
    "Boolean"
  ]
}"#;

    assert_eq!(got, expected);
}
