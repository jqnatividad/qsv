use crate::workdir::Workdir;

static EXPECTED_CSV: &str = "\
h1,h2,h3
abcdefg,1,a
a,2,z";

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

    let mut cmd = wrk.command("input");
    cmd.env("QSV_SNIFF_DELIMITER", "1");
    cmd.arg("in.file");

    let got: String = wrk.stdout(&mut cmd);
    assert_eq!(&*got, EXPECTED_CSV)
}

#[test]
fn qsv_sniff_semicolon_delimiter_env() {
    let wrk = Workdir::new("qsv_sniff_semicolon_delimiter_env");
    wrk.create_with_delim("in.file", data(), b';');

    let mut cmd = wrk.command("input");
    cmd.env("QSV_SNIFF_DELIMITER", "1");
    cmd.arg("in.file");

    let got: String = wrk.stdout(&mut cmd);
    assert_eq!(&*got, EXPECTED_CSV)
}

#[test]
fn qsv_sniff_tab_delimiter_env() {
    let wrk = Workdir::new("qsv_sniff_tab_delimiter_env");
    wrk.create_with_delim("in.file", data(), b'\t');

    let mut cmd = wrk.command("input");
    cmd.env("QSV_SNIFF_DELIMITER", "1");
    cmd.arg("in.file");

    let got: String = wrk.stdout(&mut cmd);
    assert_eq!(&*got, EXPECTED_CSV)
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
    cmd.arg("--pretty-json")
        .arg("--sample")
        .arg("0.25")
        .arg(test_file);

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
    "Date",
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

#[test]
fn sniff_prefer_dmy() {
    let wrk = Workdir::new("sniff_prefer_dmy");
    let test_file = wrk.load_test_file("boston311-dmy-100.csv");

    let mut cmd = wrk.command("sniff");
    cmd.arg("--prefer-dmy").arg(test_file);

    let got: String = wrk.stdout(&mut cmd);

    let expected = "Metadata\n========\n\tDelimiter: ,\n\tHas header row?: true\n\tNumber of preamble rows: 0\n\tQuote character: none\n\tFlexible: false\n\tIs utf-8 encoded?: true\n\nNumber of records: 100\nNumber of fields: 29\nFields:\n    0:   Unsigned  case_enquiry_id\n    1:   DateTime  open_dt\n    2:   DateTime  target_dt\n    3:   DateTime  closed_dt\n    4:   Text      ontime\n    5:   Text      case_status\n    6:   Text      closure_reason\n    7:   Text      case_title\n    8:   Text      subject\n    9:   Text      reason\n    10:  Text      type\n    11:  Text      queue\n    12:  Text      department\n    13:  Text      submittedphoto\n    14:  Boolean   closedphoto\n    15:  Text      location\n    16:  Unsigned  fire_district\n    17:  Text      pwd_district\n    18:  Unsigned  city_council_district\n    19:  Text      police_district\n    20:  Text      neighborhood\n    21:  Unsigned  neighborhood_services_district\n    22:  Text      ward\n    23:  Unsigned  precinct\n    24:  Text      location_street_name\n    25:  Unsigned  location_zipcode\n    26:  Float     latitude\n    27:  Float     longitude\n    28:  Text      source";

    assert_eq!(got, expected);
}

#[test]
fn sniff_flaky_delimiter_guess() {
    let wrk = Workdir::new("sniff_flaky_delimiter_guess");
    let test_file = wrk.load_test_file("test_sniff_delimiter.csv");

    let mut cmd = wrk.command("sniff");
    cmd.arg("--delimiter").arg(",").arg(test_file);

    // this should  ALWAYS succeed since we explicitly set the delimiter to ','
    // about 40% OF the time for this specific file, the delimiter guesser will
    // guess the wrong delimiter if we don't explicitly set it.
    wrk.assert_success(&mut cmd);
}
