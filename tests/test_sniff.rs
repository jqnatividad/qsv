use newline_converter::dos2unix;

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
    wrk.create_with_delim("in.CSV", data(), b',');

    let mut cmd = wrk.command("sniff");
    cmd.arg("in.CSV");

    let got: String = wrk.stdout(&mut cmd);

    let expected_end = dos2unix(
        r#"Retrieved Size (bytes): 27
File Size (bytes): 27
Sampled Records: 2
Estimated: false
Num Records: 2
Avg Record Len (bytes): 6
Num Fields: 3
Stats Types: false
Fields:
    0:  Text      h1
    1:  Unsigned  h2
    2:  Text      h3"#,
    );

    assert!(dos2unix(&got).trim_end().ends_with(expected_end.trim_end()));
}

#[test]
fn sniff_stats_types() {
    let wrk = Workdir::new("sniff_stats_types");
    wrk.create_with_delim("in.CSV", data(), b',');

    let mut cmd = wrk.command("sniff");
    cmd.arg("--stats-types").arg("in.CSV");

    let got: String = wrk.stdout(&mut cmd);

    let expected_end = dos2unix(
        r#"Retrieved Size (bytes): 27
File Size (bytes): 27
Sampled Records: 2
Estimated: false
Num Records: 2
Avg Record Len (bytes): 6
Num Fields: 3
Stats Types: true
Fields:
    0:  String   h1
    1:  Integer  h2
    2:  String   h3"#,
    );

    assert!(dos2unix(&got).trim_end().ends_with(expected_end.trim_end()));
}

#[test]
fn sniff_url_notcsv() {
    let wrk = Workdir::new("sniff_url_notcsv");

    let mut cmd = wrk.command("sniff");
    cmd.arg("https://github.com/dathere/qsv/raw/master/resources/test/excel-xlsx.xlsx");

    let got_error = wrk.output_stderr(&mut cmd);

    let expected = "File is not a CSV file. Detected mime type: \
                    application/vnd.openxmlformats-officedocument.spreadsheetml.sheet";
    assert!(got_error.starts_with(expected));
}

#[test]
fn sniff_notcsv() {
    let wrk = Workdir::new("sniff_notcsv");

    let test_file = wrk.load_test_file("excel-xls.xls");

    let mut cmd = wrk.command("sniff");
    cmd.arg(test_file);

    let got_error = wrk.output_stderr(&mut cmd);

    let expected = "File is not a CSV file. Detected mime type: application/vnd.ms-excel";
    assert!(got_error.starts_with(expected));
}

#[test]
fn sniff_justmime() {
    let wrk = Workdir::new("sniff_justmime");

    let test_file = wrk.load_test_file("excel-xls.xls");

    let mut cmd = wrk.command("sniff");
    cmd.arg("--just-mime").arg(test_file);

    let got: String = wrk.stdout(&mut cmd);

    let expected = "Detected mime type: application/vnd.ms-excel";
    assert!(got.starts_with(expected));
}

#[test]
fn sniff_justmime_remote() {
    let wrk = Workdir::new("sniff_justmime_remote");

    let mut cmd = wrk.command("sniff");
    cmd.arg("--just-mime")
        .arg("https://github.com/dathere/qsv/raw/master/resources/test/excel-xls.xls");

    let got: String = wrk.stdout(&mut cmd);

    let expected = "Detected mime type: application/vnd.ms-excel";
    assert!(got.starts_with(expected));
}

#[test]
fn sniff_url_snappy() {
    let wrk = Workdir::new("sniff_url_snappy");

    let mut cmd = wrk.command("sniff");
    cmd.arg("https://github.com/dathere/qsv/raw/master/resources/test/boston311-100.csv.sz");

    wrk.assert_success(&mut cmd);

    let got: String = wrk.stdout(&mut cmd);

    let expected_end = r#"Sampled Records: 100
Estimated: false
Num Records: 100
Avg Record Len (bytes): 444
Num Fields: 29
Stats Types: false
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
    14:  NULL      closedphoto
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

    assert!(got.ends_with(expected_end));
}

#[test]
fn sniff_url_snappy_noinfer() {
    let wrk = Workdir::new("sniff_url_snappy_noinfer");

    let mut cmd = wrk.command("sniff");
    cmd.arg("https://github.com/dathere/qsv/raw/master/resources/test/boston311-100.csv.sz")
        .arg("--no-infer");

    let got: String = wrk.stdout(&mut cmd);

    let expected = "Detected mime type: application/x-snappy-framed";

    assert!(got.starts_with(expected));
}

#[test]
fn sniff_file_snappy() {
    let wrk = Workdir::new("sniff_file_snappy");

    let test_file = wrk.load_test_file("boston311-100.csv.sz");

    let mut cmd = wrk.command("sniff");
    cmd.arg(test_file);

    let got: String = wrk.stdout(&mut cmd);

    let expected_end = r#"Sampled Records: 100
Estimated: false
Num Records: 100
Avg Record Len (bytes): 444
Num Fields: 29
Stats Types: false
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
    14:  NULL      closedphoto
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

    assert!(dos2unix(&got).trim_end().ends_with(expected_end.trim_end()));
}

#[test]
fn sniff_tab() {
    let wrk = Workdir::new("sniff_tab");
    wrk.create_with_delim("in.TAB", data(), b'\t');

    let mut cmd = wrk.command("sniff");
    cmd.arg("in.TAB");

    let got: String = wrk.stdout(&mut cmd);

    let expected_end = r#"Delimiter: tab
Header Row: true
Preamble Rows: 0
Quote Char: none
Flexible: false
Is UTF8: true
Detected Mime Type: text/plain
Detected Kind: Other
Retrieved Size (bytes): 27
File Size (bytes): 27
Sampled Records: 2
Estimated: false
Num Records: 2
Avg Record Len (bytes): 6
Num Fields: 3
Stats Types: false
Fields:
    0:  Text      h1
    1:  Unsigned  h2
    2:  Text      h3"#;

    assert!(dos2unix(&got).trim_end().ends_with(expected_end));
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
    let expected_end: &str = r#"ampled_records":3,"estimated":false,"num_records":3,"avg_record_len":10,"num_fields":4,"stats_types":false,"fields":["h1","h2","h3","h4"],"types":["Text","Unsigned","Text","Float"]}"#;

    assert!(got.ends_with(expected_end));
}

#[test]
fn sniff_flexible_json() {
    let wrk = Workdir::new("sniff_flexible_json");
    let test_file = wrk.load_test_file("snifftest-flexible.csv");

    let mut cmd = wrk.command("sniff");
    cmd.arg("--json").arg(test_file);

    let got: String = wrk.stdout(&mut cmd);

    let expected_end = r#"sampled_records":5,"estimated":false,"num_records":5,"avg_record_len":8,"num_fields":4,"stats_types":false,"fields":["h1","h2","h3","h4"],"types":["Text","Unsigned","Text","Float"]}"#;

    assert!(got.ends_with(expected_end));
}

#[test]
fn sniff_pretty_json() {
    let wrk = Workdir::new("sniff_pretty_json");
    let test_file = wrk.load_test_file("snifftest.csv");

    let mut cmd = wrk.command("sniff");
    cmd.arg("--pretty-json").arg(test_file);

    let got: String = wrk.stdout(&mut cmd);

    let expected_end = dos2unix(
        r#"sampled_records": 3,
  "estimated": false,
  "num_records": 3,
  "avg_record_len": 10,
  "num_fields": 4,
  "stats_types": false,
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
}"#,
    );

    assert!(dos2unix(&got).trim_end().ends_with(expected_end.trim_end()));
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

    let expected_end = r#""sampled_records": 7,
  "estimated": false,
  "num_records": 15,
  "avg_record_len": 555,
  "num_fields": 32,
  "stats_types": false,
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
    "Boolean",
    "Boolean",
    "Boolean",
    "Boolean",
    "Boolean",
    "Boolean",
    "Boolean",
    "NULL",
    "NULL",
    "Text",
    "Text",
    "Text",
    "Text",
    "Text",
    "Text",
    "Unsigned",
    "NULL",
    "Text",
    "NULL",
    "NULL"
  ]
}"#;

    assert!(dos2unix(&got).trim_end().ends_with(expected_end.trim_end()));
}

#[test]
fn sniff_prefer_dmy() {
    let wrk = Workdir::new("sniff_prefer_dmy");
    let test_file = wrk.load_test_file("boston311-dmy-100.csv");

    let mut cmd = wrk.command("sniff");
    cmd.arg("--prefer-dmy").arg(test_file);

    let got: String = wrk.stdout(&mut cmd);

    let expected_end = r#"Sampled Records: 100
Estimated: false
Num Records: 100
Avg Record Len (bytes): 444
Num Fields: 29
Stats Types: false
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
    14:  NULL      closedphoto
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

    assert!(dos2unix(&got).trim_end().ends_with(expected_end.trim_end()));
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

#[test]
fn sniff_consistent_results_issue_956() {
    let wrk = Workdir::new("sniff_consistent_results_issue_956");

    let test_file = wrk.load_test_file("spendover25kdownloadSep.csv");
    let mut cmd = wrk.command("sniff");
    cmd.arg(test_file);
    wrk.assert_err(&mut cmd);

    let test_file = wrk.load_test_file("311011.csv");
    let mut cmd = wrk.command("sniff");
    cmd.arg(test_file);
    wrk.assert_err(&mut cmd);

    let test_file = wrk.load_test_file("FCOServices_TransparencySpend_May2011.csv");
    let mut cmd = wrk.command("sniff");
    cmd.arg(test_file);
    wrk.assert_err(&mut cmd);

    let test_file = wrk.load_test_file("iwfg09_Phos_river_200911.csv");
    let mut cmd = wrk.command("sniff");
    cmd.arg(test_file);
    wrk.assert_err(&mut cmd);

    let test_file = wrk.load_test_file("Inpatients_MHA_Machine_readable_dataset_1011.csv");
    let mut cmd = wrk.command("sniff");
    cmd.arg(test_file);
    wrk.assert_err(&mut cmd);
}
