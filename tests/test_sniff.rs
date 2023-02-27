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

    let expected_start = r#"Metadata
========
	Delimiter: ,
	Has header row?: true
	Number of preamble rows: 0
	Quote character: none
	Flexible: false
	Is utf-8 encoded?: true

Path: stdin
Sniffed:"#;

    let expected_end = r#"Retrieved size (bytes): 27
File size (bytes): 27
Sampled records: 2
Number of records: 2
Number of fields: 3
Fields:
    0:  Text      h1
    1:  Unsigned  h2
    2:  Text      h3"#;

    assert!(got.starts_with(expected_start));
    assert!(got.ends_with(expected_end));
}

#[test]
fn sniff_url() {
    let wrk = Workdir::new("sniff");

    let mut cmd = wrk.command("sniff");
    cmd.arg("https://github.com/jqnatividad/qsv/raw/master/resources/test/boston311-100.csv");

    let got: String = wrk.stdout(&mut cmd);

    let expected_start = r#"Metadata
========
	Delimiter: ,
	Has header row?: true
	Number of preamble rows: 0
	Quote character: none
	Flexible: false
	Is utf-8 encoded?: true

Path: https://github.com/jqnatividad/qsv/raw/master/resources/test/boston311-100.csv
Sniffed:"#;

    let expected_end = r#"Retrieved size (bytes): 47,702
File size (bytes): 47,702
Sampled records: 100
Number of records: 100
Number of fields: 29
Fields:
    0:   Unsigned  case_enquiry_id
    1:   DateTime  open_dt
    2:   DateTime  target_dt
    3:   DateTime  closed_dt
    4:   Text      ontime
    5:   Text      case_status
    6:   Text      closure_reason
    7:   Text      case_title
    8:   Text      subject
    9:   Text      reason
    10:  Text      type
    11:  Text      queue
    12:  Text      department
    13:  Text      submittedphoto
    14:  Boolean   closedphoto
    15:  Text      location
    16:  Unsigned  fire_district
    17:  Text      pwd_district
    18:  Unsigned  city_council_district
    19:  Text      police_district
    20:  Text      neighborhood
    21:  Unsigned  neighborhood_services_district
    22:  Text      ward
    23:  Unsigned  precinct
    24:  Text      location_street_name
    25:  Unsigned  location_zipcode
    26:  Float     latitude
    27:  Float     longitude
    28:  Text      source"#;

    assert!(got.starts_with(expected_start));
    assert!(got.ends_with(expected_end));
}

#[test]
fn sniff_tab() {
    let wrk = Workdir::new("sniff_tab");
    wrk.create_with_delim("in.file", data(), b'\t');

    let mut cmd = wrk.command("sniff");
    cmd.arg("in.file");

    let got: String = wrk.stdout(&mut cmd);

    let expected_start = r#"Metadata
========
	Delimiter: tab
	Has header row?: true
	Number of preamble rows: 0
	Quote character: none
	Flexible: false
	Is utf-8 encoded?: true

Path: stdin
Sniffed:"#;
    let expected_end = r#"Retrieved size (bytes): 27
File size (bytes): 27
Sampled records: 2
Number of records: 2
Number of fields: 3
Fields:
    0:  Text      h1
    1:  Unsigned  h2
    2:  Text      h3"#;
    assert!(got.starts_with(expected_start));
    assert!(got.ends_with(expected_end));
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

    let expected_start = r#"{"path":"stdin","sniff_timestamp":"#;
    let expected_end = r#""delimiter_char":",","header_row":true,"preamble_rows":3,"quote_char":"none","flexible":false,"is_utf8":true,"retrieved_size":116,"file_size":116,"sampled_records":3,"num_records":3,"num_fields":4,"fields":["h1","h2","h3","h4"],"types":["Text","Unsigned","Text","Float"]}"#;
    assert!(got.starts_with(expected_start));
    assert!(got.ends_with(expected_end));
}

#[test]
fn sniff_flexible_json() {
    let wrk = Workdir::new("sniff_flexible_json");
    let test_file = wrk.load_test_file("snifftest-flexible.csv");

    let mut cmd = wrk.command("sniff");
    cmd.arg("--json").arg(test_file);

    let got: String = wrk.stdout(&mut cmd);

    let expected_start = r#"{"path":"stdin","sniff_timestamp":"#;
    let expected_end = r#""delimiter_char":",","header_row":true,"preamble_rows":3,"quote_char":"none","flexible":true,"is_utf8":true,"retrieved_size":135,"file_size":135,"sampled_records":5,"num_records":5,"num_fields":4,"fields":["h1","h2","h3","h4"],"types":["Text","Unsigned","Text","Float"]}"#;
    assert!(got.starts_with(expected_start));
    assert!(got.ends_with(expected_end));
}

#[test]
fn sniff_pretty_json() {
    let wrk = Workdir::new("sniff_pretty_json");
    let test_file = wrk.load_test_file("snifftest.csv");

    let mut cmd = wrk.command("sniff");
    cmd.arg("--pretty-json").arg(test_file);

    let got: String = wrk.stdout(&mut cmd);

    let expected_start = r#"{
  "path": "stdin",
  "sniff_timestamp":"#;
    let expected_end = r#""delimiter_char": ",",
  "header_row": true,
  "preamble_rows": 3,
  "quote_char": "none",
  "flexible": false,
  "is_utf8": true,
  "retrieved_size": 116,
  "file_size": 116,
  "sampled_records": 3,
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

    assert!(got.starts_with(expected_start));
    assert!(got.ends_with(expected_end));
}

#[test]
fn sniff_sample() {
    let wrk = Workdir::new("sniff_sample");
    let test_file = wrk.load_test_file("adur-public-toilets.csv");

    let mut cmd = wrk.command("sniff");
    cmd.arg("--pretty-json")
        .arg("--sample")
        .arg("0.5")
        .arg(test_file);

    let got: String = wrk.stdout(&mut cmd);

    let expected_start = r#"{
  "path": "stdin",
  "sniff_timestamp":"#;
    let expected_end = r#""delimiter_char": ",",
  "header_row": true,
  "preamble_rows": 0,
  "quote_char": "none",
  "flexible": false,
  "is_utf8": true,
  "retrieved_size": 9246,
  "file_size": 9246,
  "sampled_records": 7,
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

    assert!(got.starts_with(expected_start));
    assert!(got.ends_with(expected_end));
}

#[test]
fn sniff_prefer_dmy() {
    let wrk = Workdir::new("sniff_prefer_dmy");
    let test_file = wrk.load_test_file("boston311-dmy-100.csv");

    let mut cmd = wrk.command("sniff");
    cmd.arg("--prefer-dmy").arg(test_file);

    let got: String = wrk.stdout(&mut cmd);

    let expected_start = r#"Metadata
========
	Delimiter: ,
	Has header row?: true
	Number of preamble rows: 0
	Quote character: none
	Flexible: false
	Is utf-8 encoded?: true

Path: stdin
Sniffed:"#;
    let expected_end = r#"Retrieved size (bytes): 47,702
File size (bytes): 47,702
Sampled records: 100
Number of records: 100
Number of fields: 29
Fields:
    0:   Unsigned  case_enquiry_id
    1:   DateTime  open_dt
    2:   DateTime  target_dt
    3:   DateTime  closed_dt
    4:   Text      ontime
    5:   Text      case_status
    6:   Text      closure_reason
    7:   Text      case_title
    8:   Text      subject
    9:   Text      reason
    10:  Text      type
    11:  Text      queue
    12:  Text      department
    13:  Text      submittedphoto
    14:  Boolean   closedphoto
    15:  Text      location
    16:  Unsigned  fire_district
    17:  Text      pwd_district
    18:  Unsigned  city_council_district
    19:  Text      police_district
    20:  Text      neighborhood
    21:  Unsigned  neighborhood_services_district
    22:  Text      ward
    23:  Unsigned  precinct
    24:  Text      location_street_name
    25:  Unsigned  location_zipcode
    26:  Float     latitude
    27:  Float     longitude
    28:  Text      source"#;

    assert!(got.starts_with(expected_start));
    assert!(got.ends_with(expected_end));
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
