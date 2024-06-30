use crate::workdir::Workdir;

#[test]
fn validate_good_csv() {
    let wrk = Workdir::new("validate").flexible(true);
    wrk.create(
        "data.csv",
        vec![
            svec!["title", "name", "age"],
            svec!["Professor", "Xaviers", "60"],
            svec!["Prisoner", "Magneto", "90"],
            svec!["First Class Student", "Iceman", "14"],
        ],
    );
    let mut cmd = wrk.command("validate");
    cmd.arg("data.csv");

    wrk.output(&mut cmd);
}

#[test]
fn validate_good_csv_msg() {
    let wrk = Workdir::new("validate_good_csv_msg").flexible(true);
    wrk.create(
        "data.csv",
        vec![
            svec!["title", "name", "real age (earth years)"],
            svec!["Professor", "Xaviers", "60"],
            svec!["Prisoner", "Magneto", "90"],
            svec!["First Class Student", "Iceman", "14"],
        ],
    );
    let mut cmd = wrk.command("validate");
    cmd.arg("data.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected =
        r#"Valid: 3 columns ("title", "name", "real age (earth years)") and 3 records detected."#;
    assert_eq!(got, expected);
}

#[test]
fn validate_empty_csv_msg() {
    let wrk = Workdir::new("validate_empty_csv_msg").flexible(true);
    wrk.create(
        "data.csv",
        vec![svec!["title", "name", "real age (earth years)"]],
    );
    let mut cmd = wrk.command("validate");
    cmd.arg("data.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected =
        r#"Valid: 3 columns ("title", "name", "real age (earth years)") and 0 records detected."#;
    assert_eq!(got, expected);
}

#[test]
fn validate_good_csv_pretty_json() {
    let wrk = Workdir::new("validate_good_csv_pretty_json").flexible(true);
    wrk.create(
        "data.csv",
        vec![
            svec!["title", "name", "real age (earth years)"],
            svec!["Professor", "Xaviers", "60"],
            svec!["Prisoner", "Magneto", "90"],
            svec!["First Class Student", "Iceman", "14"],
        ],
    );
    let mut cmd = wrk.command("validate");
    cmd.arg("--pretty-json").arg("data.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"{
  "delimiter_char": ",",
  "header_row": true,
  "quote_char": "\"",
  "num_records": 3,
  "num_fields": 3,
  "fields": [
    "title",
    "name",
    "real age (earth years)"
  ]
}"#;
    assert_eq!(got, expected);
}

#[test]
fn validate_good_csv_json() {
    let wrk = Workdir::new("validate_good_csv_json").flexible(true);
    wrk.create(
        "data.csv",
        vec![
            svec!["title", "name", "age"],
            svec!["Professor", "Xaviers", "60"],
            svec!["Prisoner", "Magneto", "90"],
            svec!["First Class Student", "Iceman", "14"],
        ],
    );
    let mut cmd = wrk.command("validate");
    cmd.arg("--json").arg("data.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"{"delimiter_char":",","header_row":true,"quote_char":"\"","num_records":3,"num_fields":3,"fields":["title","name","age"]}"#;
    assert_eq!(got, expected);
}

#[test]
fn validate_bad_csv() {
    let wrk = Workdir::new("validate").flexible(true);
    wrk.create(
        "data.csv",
        vec![
            svec!["title", "name", "age"],
            svec!["Professor", "Xaviers", "60"],
            svec!["Magneto", "90",],
            svec!["First Class Student", "Iceman", "14"],
        ],
    );
    let mut cmd = wrk.command("validate");
    cmd.arg("data.csv");

    let got: String = wrk.output_stderr(&mut cmd);
    let expected = r#"Validation error: CSV error: record 2 (line: 3, byte: 36): found record with 2 fields, but the previous record has 3 fields.
Use `qsv fixlengths` to fix record length issues.
"#;
    assert_eq!(got, expected);

    wrk.assert_err(&mut cmd);
}

#[test]
fn validate_bad_csv_first_record() {
    let wrk = Workdir::new("validate_bad_csv_first_record").flexible(true);
    wrk.create(
        "data.csv",
        vec![
            svec!["title", "name", "age"],
            svec!["Professor", "Xaviers",],
            svec!["Doctor", "Magneto", "90",],
            svec!["First Class Student", "Iceman", "14"],
        ],
    );
    let mut cmd = wrk.command("validate");
    cmd.arg("data.csv");

    let got: String = wrk.output_stderr(&mut cmd);
    let expected = r#"Validation error: CSV error: record 1 (line: 2, byte: 15): found record with 2 fields, but the previous record has 3 fields.
Use `qsv fixlengths` to fix record length issues.
"#;
    assert_eq!(got, expected);

    wrk.assert_err(&mut cmd);
}

#[test]
fn validate_bad_csv_last_record() {
    let wrk = Workdir::new("validate_bad_csv_last_record").flexible(true);
    wrk.create(
        "data.csv",
        vec![
            svec!["title", "name", "age"],
            svec!["Professor", "Xaviers", "60"],
            svec!["Doctor", "Magneto", "90"],
            svec!["First Class Student", "Iceman", "14", "extra field"],
        ],
    );
    let mut cmd = wrk.command("validate");
    cmd.arg("data.csv");

    let got: String = wrk.output_stderr(&mut cmd);
    let expected = r#"Validation error: CSV error: record 3 (line: 4, byte: 54): found record with 4 fields, but the previous record has 3 fields.
Use `qsv fixlengths` to fix record length issues.
"#;
    assert_eq!(got, expected);

    wrk.assert_err(&mut cmd);
}

#[test]
fn validate_bad_csv_prettyjson() {
    let wrk = Workdir::new("validate_bad_csv_prettyjson").flexible(true);
    wrk.create(
        "data.csv",
        vec![
            svec!["title", "name", "age"],
            svec!["Professor", "Xaviers", "60"],
            svec!["Magneto", "90",],
            svec!["First Class Student", "Iceman", "14"],
        ],
    );
    let mut cmd = wrk.command("validate");
    cmd.arg("--pretty-json").arg("data.csv");

    let got: String = wrk.output_stderr(&mut cmd);
    let expected = r#"{
  "errors": [
    {
      "title": "Validation error",
      "detail": "CSV error: record 2 (line: 3, byte: 36): found record with 2 fields, but the previous record has 3 fields",
      "meta": {
        "last_valid_record": "1"
      }
    }
  ]
}
"#;
    assert_eq!(got, expected);

    wrk.assert_err(&mut cmd);
}

fn adur_errors() -> &'static str {
    r#"row_number	field	error
1	ExtractDate	null is not of type "string"
1	OrganisationLabel	null is not of type "string"
3	CoordinateReferenceSystem	"OSGB3" does not match "(WGS84|OSGB36)"
3	Category	"Mens" does not match "(Female|Male|Female and Male|Unisex|Male urinal|Children only|None)"
"#
}

// invalid records with index from original csv
// row 1: missing values for ExtractDate and OrganisationLabel
// row 3: wrong value for CoordinateReferenceSystem and Category
// note: removed unnecessary quotes for string column "OpeningHours"
fn adur_invalids() -> &'static str {
    r#"ExtractDate,OrganisationURI,OrganisationLabel,ServiceTypeURI,ServiceTypeLabel,LocationText,CoordinateReferenceSystem,GeoX,GeoY,GeoPointLicensingURL,Category,AccessibleCategory,RADARKeyNeeded,BabyChange,FamilyToilet,ChangingPlace,AutomaticPublicConvenience,FullTimeStaffing,PartOfCommunityScheme,CommunitySchemeName,ChargeAmount,InfoURL,OpeningHours,ManagedBy,ReportEmail,ReportTel,Notes,UPRN,Postcode,StreetAddress,GeoAreaURI,GeoAreaLabel
,http://opendatacommunities.org/id/district-council/adur,,http://id.esd.org.uk/service/579,Public toilets,BEACH GREEN PUBLIC CONVENIENCES BRIGHTON ROAD LANCING,OSGB36,518072,103649,http://www.ordnancesurvey.co.uk/business-and-government/help-and-support/public-sector/guidance/derived-data-exemptions.html,Female and male,Unisex,Yes,No,No,No,No,No,No,,,http://www.adur-worthing.gov.uk/streets-and-travel/public-toilets/,S = 09:00 - 21:00 W = 09:00 - 17:00 ,ADC,surveyor_1@adur-worthing.gov.uk,01903 221471,,60001449,,BEACH GREEN PUBLIC CONVENIENCES BRIGHTON ROAD LANCING,,
2014-07-07 00:00,http://opendatacommunities.org/id/district-council/adur,Adur,http://id.esd.org.uk/service/579,Public toilets,PUBLIC CONVENIENCES SHOPSDAM ROAD LANCING,OSGB3,518915,103795,http://www.ordnancesurvey.co.uk/business-and-government/help-and-support/public-sector/guidance/derived-data-exemptions.html,Mens,Unisex,Yes,No,No,No,No,No,No,,,http://www.adur-worthing.gov.uk/streets-and-travel/public-toilets/,S = 09:00 - 21:00 W = 09:00 - 17:00,ADC,surveyor_3@adur-worthing.gov.uk,01903 221471,,60007428,,,,
"#
}

#[test]
fn validate_adur_public_toilets_dataset_with_json_schema() {
    let wrk = Workdir::new("validate").flexible(true);

    // copy schema file to workdir
    let schema: String = wrk.load_test_resource("public-toilets-schema.json");
    wrk.create_from_string("schema.json", &schema);

    // copy csv file to workdir
    let csv: String = wrk.load_test_resource("adur-public-toilets.csv");
    wrk.create_from_string("data.csv", &csv);

    // run validate command
    let mut cmd = wrk.command("validate");
    cmd.arg("data.csv").arg("schema.json");

    wrk.output(&mut cmd);

    // check invalid file output
    let invalid_output: String = wrk.from_str(&wrk.path("data.csv.invalid"));
    assert_eq!(adur_invalids().to_string(), invalid_output);

    // check validation error output

    let validation_error_output: String = wrk.from_str(&wrk.path("data.csv.validation-errors.tsv"));
    assert_eq!(adur_errors(), validation_error_output);
    wrk.assert_err(&mut cmd);
}

#[test]
fn validate_adur_public_toilets_dataset_with_json_schema_valid_output() {
    let wrk = Workdir::new("validate_valid_output").flexible(true);

    // copy schema file to workdir
    let schema: String = wrk.load_test_resource("public-toilets-schema.json");
    wrk.create_from_string("schema.json", &schema);

    // copy csv file to workdir
    let csv: String = wrk.load_test_resource("adur-public-toilets-valid.csv");
    wrk.create_from_string("data.csv", &csv);

    // run validate command
    let mut cmd = wrk.command("validate");
    cmd.arg("data.csv")
        .arg("schema.json")
        .args(["--valid-output", "-"]);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["ExtractDate", "OrganisationURI", "OrganisationLabel", "ServiceTypeURI", "ServiceTypeLabel", "LocationText", "CoordinateReferenceSystem", "GeoX", "GeoY", "GeoPointLicensingURL", "Category", "AccessibleCategory", "RADARKeyNeeded", "BabyChange", "FamilyToilet", "ChangingPlace", "AutomaticPublicConvenience", "FullTimeStaffing", "PartOfCommunityScheme", "CommunitySchemeName", "ChargeAmount", "InfoURL", "OpeningHours", "ManagedBy", "ReportEmail", "ReportTel", "Notes", "UPRN", "Postcode", "StreetAddress", "GeoAreaURI", "GeoAreaLabel"], 
        svec!["07/07/2014 00:00", "http://opendatacommunities.org/id/district-council/adur", "Adur", "http://id.esd.org.uk/service/579", "Public toilets", "PUBLIC CONVENIENCES MONKS RECREATION GROUND CRABTREE LANE LANCING", "OSGB36", "518225", "104730", "http://www.ordnancesurvey.co.uk/business-and-government/help-and-support/public-sector/guidance/derived-data-exemptions.html", "Female and male", "None", "Yes", "No", "No", "No", "No", "No", "No", "", "", "http://www.adur-worthing.gov.uk/streets-and-travel/public-toilets/", "S = 09:00 - 15:00 W = 09:00 - 15:00", "ADC", "surveyor_2@adur-worthing.gov.uk", "01903 221471", "", "60002210", "", "PUBLIC CONVENIENCES MONKS RECREATION GROUND CRABTREE LANE LANCING", "", ""], 
        svec!["07/07/2014 00:00", "http://opendatacommunities.org/id/district-council/adur", "Adur", "http://id.esd.org.uk/service/579", "Public toilets", "PUBLIC CONVENIENCES YEW TREE CLOSE LANCING", "OSGB36", "518222", "104168", "http://www.ordnancesurvey.co.uk/business-and-government/help-and-support/public-sector/guidance/derived-data-exemptions.html", "Female and male", "Unisex", "Yes", "No", "No", "No", "No", "No", "No", "", "", "http://www.adur-worthing.gov.uk/streets-and-travel/public-toilets/", "S = 09:00 - 21:00 W = 09:00 - 17:00", "ADC", "surveyor_4@adur-worthing.gov.uk", "01903 221471", "", "60008859", "", "PUBLIC CONVENIENCES YEW TREE CLOSE LANCING", "", ""], 
        svec!["07/07/2014 00:00", "http://opendatacommunities.org/id/district-council/adur", "Adur", "http://id.esd.org.uk/service/579", "Public toilets", "PUBLIC CONVENIENCES BEACH GREEN SHOREHAM-BY-SEA", "OSGB36", "521299", "104515", "http://www.ordnancesurvey.co.uk/business-and-government/help-and-support/public-sector/guidance/derived-data-exemptions.html", "Female and male", "Unisex", "Yes", "No", "No", "No", "No", "No", "No", "", "", "http://www.adur-worthing.gov.uk/streets-and-travel/public-toilets/", "S = 09:00 - 21:00 W = 09:00 - 17:00", "ADC", "surveyor_5@adur-worthing.gov.uk", "01903 221471", "", "60009402", "", "PUBLIC CONVENIENCES BEACH GREEN SHOREHAM-BY-SEA", "", ""], 
        svec!["07/07/2014 00:00", "http://opendatacommunities.org/id/district-council/adur", "Adur", "http://id.esd.org.uk/service/579", "Public toilets", "PUBLIC CONVENIENCES ADUR RECREATION GROUND BRIGHTON ROAD SHOREHAM-BY-SEA", "OSGB36", "521048", "104977", "http://www.ordnancesurvey.co.uk/business-and-government/help-and-support/public-sector/guidance/derived-data-exemptions.html", "Female and male", "Unisex", "Yes", "No", "No", "No", "No", "No", "No", "", "", "http://www.adur-worthing.gov.uk/streets-and-travel/public-toilets/", "S = 08:00 - 21:00 W = 08:00 - 17:00", "ADC", "surveyor_6@adur-worthing.gov.uk", "01903 221471", "", "60009666", "", "PUBLIC CONVENIENCES ADUR RECREATION GROUND BRIGHTON ROAD SHOREHAM-BY-SEA", "", ""], 
        svec!["07/07/2014 00:00", "http://opendatacommunities.org/id/district-council/adur", "Adur", "http://id.esd.org.uk/service/579", "Public toilets", "PUBLIC CONVENIENCES FORTHAVEN SHOREHAM-BY-SEA", "OSGB36", "523294", "104588", "http://www.ordnancesurvey.co.uk/business-and-government/help-and-support/public-sector/guidance/derived-data-exemptions.html", "Female and male", "Unisex", "Yes", "No", "No", "No", "No", "No", "No", "", "", "http://www.adur-worthing.gov.uk/streets-and-travel/public-toilets/", "S = 09:00 - 21:00 W = 09:00 - 17:00", "ADC", "surveyor_7@adur-worthing.gov.uk", "01903 221471", "", "60011970", "", "PUBLIC CONVENIENCES FORTHAVEN SHOREHAM-BY-SEA", "", ""], 
        svec!["07/07/2014 00:00", "http://opendatacommunities.org/id/district-council/adur", "Adur", "http://id.esd.org.uk/service/579", "Public toilets", "PUBLIC CONVENIENCES MIDDLE STREET SHOREHAM-BY-SEA", "OSGB36", "521515", "105083", "http://www.ordnancesurvey.co.uk/business-and-government/help-and-support/public-sector/guidance/derived-data-exemptions.html", "Female and male", "Unisex", "Yes", "No", "No", "No", "No", "No", "No", "", "", "http://www.adur-worthing.gov.uk/streets-and-travel/public-toilets/", "S = 09:00 - 21:00 W = 09:00 - 17:00", "ADC", "surveyor_8@adur-worthing.gov.uk", "01903 221471", "", "60014163", "", "PUBLIC CONVENIENCES MIDDLE STREET SHOREHAM-BY-SEA", "", ""], 
        svec!["07/07/2014 00:00", "http://opendatacommunities.org/id/district-council/adur", "Adur", "http://id.esd.org.uk/service/579", "Public toilets", "PUBLIC CONVENIENCES CEMETERY MILL LANE SHOREHAM-BY-SEA", "OSGB36", "521440", "105725", "http://www.ordnancesurvey.co.uk/business-and-government/help-and-support/public-sector/guidance/derived-data-exemptions.html", "Female and male", "None", "No", "No", "No", "No", "No", "No", "No", "", "", "http://www.adur-worthing.gov.uk/streets-and-travel/public-toilets/", "", "ADC", "surveyor_9@adur-worthing.gov.uk", "01903 221471", "Grounds staff only not public", "60014340", "", "PUBLIC CONVENIENCES CEMETERY MILL LANE SHOREHAM-BY-SEA", "", ""], 
        svec!["07/07/2014 00:00", "http://opendatacommunities.org/id/district-council/adur", "Adur", "http://id.esd.org.uk/service/579", "Public toilets", "PUBLIC CONVENIENCES SOUTH PAVILION BUCKINGHAM PARK UPPER SHOREHAM ROAD SHOREHAM-BY-SEA", "OSGB36", "522118", "105939", "http://www.ordnancesurvey.co.uk/business-and-government/help-and-support/public-sector/guidance/derived-data-exemptions.html", "Female and male", "None", "No", "No", "No", "No", "No", "No", "No", "", "", "http://www.adur-worthing.gov.uk/streets-and-travel/public-toilets/", "S = 09:00 - 21:00 W = 09:00 - 17:00", "ADC", "surveyor_10@adur-worthing.gov.uk", "01903 221471", "", "60017866", "", "PUBLIC CONVENIENCES SOUTH PAVILION BUCKINGHAM PARK UPPER SHOREHAM ROAD SHOREHAM-BY-SEA", "", ""], 
        svec!["07/07/2014 00:00", "http://opendatacommunities.org/id/district-council/adur", "Adur", "http://id.esd.org.uk/service/579", "Public toilets", "PUBLIC CONVENIENCE SOUTHWICK STREET SOUTHWICK", "OSGB36", "524401", "105405", "http://www.ordnancesurvey.co.uk/business-and-government/help-and-support/public-sector/guidance/derived-data-exemptions.html", "Female and male", "Unisex", "Yes", "No", "No", "No", "No", "No", "No", "", "", "http://www.adur-worthing.gov.uk/streets-and-travel/public-toilets/", "S = 08:00 - 21:00 W = 08:00 - 17:00", "ADC", "surveyor_11@adur-worthing.gov.uk", "01903 221471", "", "60026354", "", "PUBLIC CONVENIENCE SOUTHWICK STREET SOUTHWICK", "", ""], 
        svec!["07/07/2014 00:00", "http://opendatacommunities.org/id/district-council/adur", "Adur", "http://id.esd.org.uk/service/579", "Public toilets", "WEST BEACH PUBLIC CONVENIENCES WEST BEACH ROAD LANCING", "OSGB36", "520354", "104246", "http://www.ordnancesurvey.co.uk/business-and-government/help-and-support/public-sector/guidance/derived-data-exemptions.html", "Female and male", "Unisex", "Yes", "No", "No", "No", "No", "No", "No", "", "", "http://www.adur-worthing.gov.uk/streets-and-travel/public-toilets/", "S = 09:00 - 21:00 W = 09:00 - 17:00", "", "surveyor_12@adur-worthing.gov.uk", "01903 221471", "", "60028994", "", "WEST BEACH PUBLIC CONVENIENCES WEST BEACH ROAD LANCING", "", ""], 
        svec!["07/07/2014 00:00", "http://opendatacommunities.org/id/district-council/adur", "Adur", "http://id.esd.org.uk/service/579", "Public toilets", "BEACH TOILETS BASIN ROAD SOUTH SOUTHWICK", "OSGB36", "524375", "104753", "http://www.ordnancesurvey.co.uk/business-and-government/help-and-support/public-sector/guidance/derived-data-exemptions.html", "Female and male", "Unisex", "Yes", "No", "No", "No", "No", "No", "No", "", "", "http://www.adur-worthing.gov.uk/streets-and-travel/public-toilets/", "S = 09:00 - 21:00 W = 09:00 - 17:00", "ADC", "surveyor_13@adur-worthing.gov.uk", "01903 221471", "", "60029181", "", "BEACH TOILETS BASIN ROAD SOUTH SOUTHWICK", "", ""], 
        svec!["07/07/2014 00:00", "http://opendatacommunities.org/id/district-council/adur", "Adur", "http://id.esd.org.uk/service/579", "Public toilets", "BEACH TOILETS BASIN ROAD SOUTH SOUTHWICK", "OSGB36", "522007", "106062", "http://www.ordnancesurvey.co.uk/business-and-government/help-and-support/public-sector/guidance/derived-data-exemptions.html", "Female and male", "None", "No", "No", "No", "No", "No", "No", "No", "", "", "http://www.adur-worthing.gov.uk/streets-and-travel/public-toilets/", "", "ADC", "surveyor_14@adur-worthing.gov.uk", "01903 221471", "Grounds staff only not public", "60032527", "", "PUBLIC CONVENIENCE NORTH PAVILION BUCKINGHAM PARK UPPER SHOREHAM ROAD SHOREHAM-BY-SEA", "", ""], 
        svec!["07/07/2014 00:00", "http://opendatacommunities.org/id/district-council/adur", "Adur", "http://id.esd.org.uk/service/579", "Public toilets", "BEACH TOILETS BASIN ROAD SOUTH SOUTHWICK", "OSGB36", "522083", "105168", "http://www.ordnancesurvey.co.uk/business-and-government/help-and-support/public-sector/guidance/derived-data-exemptions.html", "Female and male", "Unisex", "Yes", "No", "No", "No", "No", "No", "No", "", "", "http://www.adur-worthing.gov.uk/streets-and-travel/public-toilets/", "09.00 - 17.00", "ADC", "surveyor_15@adur-worthing.gov.uk", "01903 221471", "", "60034215", "", "PUBLIC CONVENIENCES CIVIC CENTRE HAM ROAD SHOREHAM-BY-SEA", "", ""]    
    ];
    assert_eq!(got, expected);

    let num_recs = wrk.output_stderr(&mut cmd);
    let num_recs_expected = "13".to_string();
    assert_eq!(num_recs.trim_end(), num_recs_expected);

    wrk.assert_err(&mut cmd);
}

#[test]
fn validate_with_schema_noheader() {
    let wrk = Workdir::new("validate_with_schema_noheader").flexible(true);

    // copy schema file to workdir
    let schema: String = wrk.load_test_resource("public-toilets-schema.json");
    wrk.create_from_string("schema.json", &schema);

    // copy csv file to workdir
    let csv: String = wrk.load_test_resource("adur-public-toilets-valid.csv");
    wrk.create_from_string("data.csv", &csv);

    // run validate command
    let mut cmd = wrk.command("validate");
    cmd.arg("data.csv")
        .arg("schema.json")
        .arg("--no-headers")
        .args(["--valid-output", "-"]);

    let got = wrk.output_stderr(&mut cmd);
    let expected = "Cannot validate CSV without headers against a JSON Schema.\n".to_string();
    assert_eq!(got, expected);

    wrk.assert_err(&mut cmd);
}

#[test]
fn validate_adur_public_toilets_dataset_with_json_schema_url() {
    let wrk = Workdir::new("validate").flexible(true);

    // copy csv file to workdir
    let csv: String = wrk.load_test_resource("adur-public-toilets.csv");
    wrk.create_from_string("data.csv", &csv);

    // run validate command
    let mut cmd = wrk.command("validate");
    cmd.arg("data.csv").arg("https://raw.githubusercontent.com/jqnatividad/qsv/master/resources/test/public-toilets-schema.json");

    wrk.output(&mut cmd);

    let invalid_output: String = wrk.from_str(&wrk.path("data.csv.invalid"));
    assert_eq!(adur_invalids().to_string(), invalid_output);

    // check validation error output
    let validation_error_output: String = wrk.from_str(&wrk.path("data.csv.validation-errors.tsv"));
    assert_eq!(adur_errors(), validation_error_output);
    wrk.assert_err(&mut cmd);
}
