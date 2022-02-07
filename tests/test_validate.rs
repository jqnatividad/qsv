use crate::workdir::Workdir;
use std::path::PathBuf;

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

    wrk.assert_err(&mut cmd);
}
pub const ADUR_CSV: &str = r#"ExtractDate,OrganisationURI,OrganisationLabel,ServiceTypeURI,ServiceTypeLabel,LocationText,CoordinateReferenceSystem,GeoX,GeoY,GeoPointLicensingURL,Category,AccessibleCategory,RADARKeyNeeded,BabyChange,FamilyToilet,ChangingPlace,AutomaticPublicConvenience,FullTimeStaffing,PartOfCommunityScheme,CommunitySchemeName,ChargeAmount,InfoURL,OpeningHours,ManagedBy,ReportEmail,ReportTel,Notes,UPRN,Postcode,StreetAddress,GeoAreaURI,GeoAreaLabel
  ,http://opendatacommunities.org/id/district-council/adur,,http://id.esd.org.uk/service/579,Public toilets,BEACH GREEN PUBLIC CONVENIENCES BRIGHTON ROAD LANCING,OSGB36,518072,103649,http://www.ordnancesurvey.co.uk/business-and-government/help-and-support/public-sector/guidance/derived-data-exemptions.html,Female and male,Unisex,Yes,No,No,No,No,No,No,,,http://www.adur-worthing.gov.uk/streets-and-travel/public-toilets/,"S = 09:00 - 21:00 W = 09:00 - 17:00 ",ADC,surveyors@adur-worthing.gov.uk,01903 221471,,60001449,,BEACH GREEN PUBLIC CONVENIENCES BRIGHTON ROAD LANCING,,
07/07/2014 00:00,http://opendatacommunities.org/id/district-council/adur,Adur,http://id.esd.org.uk/service/579,Public toilets,PUBLIC CONVENIENCES MONKS RECREATION GROUND CRABTREE LANE LANCING,OSGB36,518225,104730,http://www.ordnancesurvey.co.uk/business-and-government/help-and-support/public-sector/guidance/derived-data-exemptions.html,Female and male,None,Yes,No,No,No,No,No,No,,,http://www.adur-worthing.gov.uk/streets-and-travel/public-toilets/,"S = 09:00 - 15:00 W = 09:00 - 15:00",ADC,surveyors@adur-worthing.gov.uk,01903 221471,,60002210,,PUBLIC CONVENIENCES MONKS RECREATION GROUND CRABTREE LANE LANCING,,
2014-07-07 00:00,http://opendatacommunities.org/id/district-council/adur,Adur,http://id.esd.org.uk/service/579,Public toilets,PUBLIC CONVENIENCES SHOPSDAM ROAD LANCING,OSGB3,518915,103795,http://www.ordnancesurvey.co.uk/business-and-government/help-and-support/public-sector/guidance/derived-data-exemptions.html,Mens,Unisex,Yes,No,No,No,No,No,No,,,http://www.adur-worthing.gov.uk/streets-and-travel/public-toilets/,"S = 09:00 - 21:00 W = 09:00 - 17:00",ADC,surveyors@adur-worthing.gov.uk,01903 221471,,60007428,,PUBLIC CONVENIENCES SHOPSDAM ROAD LANCING,,
07/07/2014 00:00,http://opendatacommunities.org/id/district-council/adur,Adur,http://id.esd.org.uk/service/579,Public toilets,PUBLIC CONVENIENCES YEW TREE CLOSE LANCING,OSGB36,518222,104168,http://www.ordnancesurvey.co.uk/business-and-government/help-and-support/public-sector/guidance/derived-data-exemptions.html,Female and male,Unisex,Yes,No,No,No,No,No,No,,,http://www.adur-worthing.gov.uk/streets-and-travel/public-toilets/,"S = 09:00 - 21:00 W = 09:00 - 17:00",ADC,surveyors@adur-worthing.gov.uk,01903 221471,,60008859,,PUBLIC CONVENIENCES YEW TREE CLOSE LANCING,,
07/07/2014 00:00,http://opendatacommunities.org/id/district-council/adur,Adur,http://id.esd.org.uk/service/579,Public toilets,PUBLIC CONVENIENCES BEACH GREEN SHOREHAM-BY-SEA,OSGB36,521299,104515,http://www.ordnancesurvey.co.uk/business-and-government/help-and-support/public-sector/guidance/derived-data-exemptions.html,Female and male,Unisex,Yes,No,No,No,No,No,No,,,http://www.adur-worthing.gov.uk/streets-and-travel/public-toilets/,"S = 09:00 - 21:00 W = 09:00 - 17:00",ADC,surveyors@adur-worthing.gov.uk,01903 221471,,60009402,,PUBLIC CONVENIENCES BEACH GREEN SHOREHAM-BY-SEA,,
07/07/2014 00:00,http://opendatacommunities.org/id/district-council/adur,Adur,http://id.esd.org.uk/service/579,Public toilets,PUBLIC CONVENIENCES ADUR RECREATION GROUND BRIGHTON ROAD SHOREHAM-BY-SEA,OSGB36,521048,104977,http://www.ordnancesurvey.co.uk/business-and-government/help-and-support/public-sector/guidance/derived-data-exemptions.html,Female and male,Unisex,Yes,No,No,No,No,No,No,,,http://www.adur-worthing.gov.uk/streets-and-travel/public-toilets/,"S = 08:00 - 21:00 W = 08:00 - 17:00",ADC,surveyors@adur-worthing.gov.uk,01903 221471,,60009666,,PUBLIC CONVENIENCES ADUR RECREATION GROUND BRIGHTON ROAD SHOREHAM-BY-SEA,,
07/07/2014 00:00,http://opendatacommunities.org/id/district-council/adur,Adur,http://id.esd.org.uk/service/579,Public toilets,PUBLIC CONVENIENCES FORTHAVEN SHOREHAM-BY-SEA,OSGB36,523294,104588,http://www.ordnancesurvey.co.uk/business-and-government/help-and-support/public-sector/guidance/derived-data-exemptions.html,Female and male,Unisex,Yes,No,No,No,No,No,No,,,http://www.adur-worthing.gov.uk/streets-and-travel/public-toilets/,"S = 09:00 - 21:00 W = 09:00 - 17:00",ADC,surveyors@adur-worthing.gov.uk,01903 221471,,60011970,,PUBLIC CONVENIENCES FORTHAVEN SHOREHAM-BY-SEA,,
07/07/2014 00:00,http://opendatacommunities.org/id/district-council/adur,Adur,http://id.esd.org.uk/service/579,Public toilets,PUBLIC CONVENIENCES MIDDLE STREET SHOREHAM-BY-SEA,OSGB36,521515,105083,http://www.ordnancesurvey.co.uk/business-and-government/help-and-support/public-sector/guidance/derived-data-exemptions.html,Female and male,Unisex,Yes,No,No,No,No,No,No,,,http://www.adur-worthing.gov.uk/streets-and-travel/public-toilets/,"S = 09:00 - 21:00 W = 09:00 - 17:00",ADC,surveyors@adur-worthing.gov.uk,01903 221471,,60014163,,PUBLIC CONVENIENCES MIDDLE STREET SHOREHAM-BY-SEA,,
07/07/2014 00:00,http://opendatacommunities.org/id/district-council/adur,Adur,http://id.esd.org.uk/service/579,Public toilets,PUBLIC CONVENIENCES CEMETERY MILL LANE SHOREHAM-BY-SEA,OSGB36,521440,105725,http://www.ordnancesurvey.co.uk/business-and-government/help-and-support/public-sector/guidance/derived-data-exemptions.html,Female and male,None,No,No,No,No,No,No,No,,,http://www.adur-worthing.gov.uk/streets-and-travel/public-toilets/,,ADC,surveyors@adur-worthing.gov.uk,01903 221471,Grounds staff only not public,60014340,,PUBLIC CONVENIENCES CEMETERY MILL LANE SHOREHAM-BY-SEA,,
07/07/2014 00:00,http://opendatacommunities.org/id/district-council/adur,Adur,http://id.esd.org.uk/service/579,Public toilets,PUBLIC CONVENIENCES SOUTH PAVILION BUCKINGHAM PARK UPPER SHOREHAM ROAD SHOREHAM-BY-SEA,OSGB36,522118,105939,http://www.ordnancesurvey.co.uk/business-and-government/help-and-support/public-sector/guidance/derived-data-exemptions.html,Female and male,None,No,No,No,No,No,No,No,,,http://www.adur-worthing.gov.uk/streets-and-travel/public-toilets/,"S = 09:00 - 21:00 W = 09:00 - 17:00",ADC,surveyors@adur-worthing.gov.uk,01903 221471,,60017866,,PUBLIC CONVENIENCES SOUTH PAVILION BUCKINGHAM PARK UPPER SHOREHAM ROAD SHOREHAM-BY-SEA,,
07/07/2014 00:00,http://opendatacommunities.org/id/district-council/adur,Adur,http://id.esd.org.uk/service/579,Public toilets,PUBLIC CONVENIENCE SOUTHWICK STREET SOUTHWICK,OSGB36,524401,105405,http://www.ordnancesurvey.co.uk/business-and-government/help-and-support/public-sector/guidance/derived-data-exemptions.html,Female and male,Unisex,Yes,No,No,No,No,No,No,,,http://www.adur-worthing.gov.uk/streets-and-travel/public-toilets/,"S = 08:00 - 21:00 W = 08:00 - 17:00",ADC,surveyors@adur-worthing.gov.uk,01903 221471,,60026354,,PUBLIC CONVENIENCE SOUTHWICK STREET SOUTHWICK,,
07/07/2014 00:00,http://opendatacommunities.org/id/district-council/adur,Adur,http://id.esd.org.uk/service/579,Public toilets,WEST BEACH PUBLIC CONVENIENCES WEST BEACH ROAD LANCING,OSGB36,520354,104246,http://www.ordnancesurvey.co.uk/business-and-government/help-and-support/public-sector/guidance/derived-data-exemptions.html,Female and male,Unisex,Yes,No,No,No,No,No,No,,,http://www.adur-worthing.gov.uk/streets-and-travel/public-toilets/,"S = 09:00 - 21:00 W = 09:00 - 17:00",,surveyors@adur-worthing.gov.uk,01903 221471,,60028994,,WEST BEACH PUBLIC CONVENIENCES WEST BEACH ROAD LANCING,,
07/07/2014 00:00,http://opendatacommunities.org/id/district-council/adur,Adur,http://id.esd.org.uk/service/579,Public toilets,BEACH TOILETS BASIN ROAD SOUTH SOUTHWICK,OSGB36,524375,104753,http://www.ordnancesurvey.co.uk/business-and-government/help-and-support/public-sector/guidance/derived-data-exemptions.html,Female and male,Unisex,Yes,No,No,No,No,No,No,,,http://www.adur-worthing.gov.uk/streets-and-travel/public-toilets/,"S = 09:00 - 21:00 W = 09:00 - 17:00",ADC,surveyors@adur-worthing.gov.uk,01903 221471,,60029181,,BEACH TOILETS BASIN ROAD SOUTH SOUTHWICK,,
07/07/2014 00:00,http://opendatacommunities.org/id/district-council/adur,Adur,http://id.esd.org.uk/service/579,Public toilets,PUBLIC CONVENIENCE NORTH PAVILION BUCKINGHAM PARK UPPER SHOREHAM ROAD SHOREHAM-BY-SEA,OSGB36,522007,106062,http://www.ordnancesurvey.co.uk/business-and-government/help-and-support/public-sector/guidance/derived-data-exemptions.html,Female and male,None,No,No,No,No,No,No,No,,,http://www.adur-worthing.gov.uk/streets-and-travel/public-toilets/,,ADC,surveyors@adur-worthing.gov.uk,01903 221471,Grounds staff only not public,60032527,,PUBLIC CONVENIENCE NORTH PAVILION BUCKINGHAM PARK UPPER SHOREHAM ROAD SHOREHAM-BY-SEA,,
07/07/2014 00:00,http://opendatacommunities.org/id/district-council/adur,Adur,http://id.esd.org.uk/service/579,Public toilets,PUBLIC CONVENIENCES CIVIC CENTRE HAM ROAD SHOREHAM-BY-SEA,OSGB36,522083,105168,http://www.ordnancesurvey.co.uk/business-and-government/help-and-support/public-sector/guidance/derived-data-exemptions.html,Female and male,Unisex,Yes,No,No,No,No,No,No,,,http://www.adur-worthing.gov.uk/streets-and-travel/public-toilets/,09.00 - 17.00,ADC,surveyors@adur-worthing.gov.uk,01903 221471,,60034215,,PUBLIC CONVENIENCES CIVIC CENTRE HAM ROAD SHOREHAM-BY-SEA,,"#;

#[test]
fn validate_adur_public_toilets_dataset_with_json_schema() {

    let wrk = Workdir::new("validate").flexible(true);

    // locate resources/test relative to crate base dir
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("resources/test");

    // copy schema file to workdir
    path.push("public-toilets-schema.json");
    let schema: String = wrk.from_str(path.as_path());
    wrk.create_from_string("schema.json", &schema);
    path.pop();

    // copy csv file to workdir
    path.push("adur-public-toilets.csv");
    let csv: String = wrk.from_str(path.as_path());
    wrk.create_from_string("data.csv", &csv);
    path.pop();

    // run validate command
    let mut cmd = wrk.command("validate");
    cmd.arg("data.csv").arg("schema.json");

    wrk.output(&mut cmd);

    // check invalid file output
    
    // invalid records with index from original csv
    // row 1: missing values for ExtractDate and OrganisationLabel
    // row 3: wrong value for CoordinateReferenceSystem and Category
    // note: removed unnecessary quotes for string column "OpeningHours"
    let invalid_expected = r#"ExtractDate,OrganisationURI,OrganisationLabel,ServiceTypeURI,ServiceTypeLabel,LocationText,CoordinateReferenceSystem,GeoX,GeoY,GeoPointLicensingURL,Category,AccessibleCategory,RADARKeyNeeded,BabyChange,FamilyToilet,ChangingPlace,AutomaticPublicConvenience,FullTimeStaffing,PartOfCommunityScheme,CommunitySchemeName,ChargeAmount,InfoURL,OpeningHours,ManagedBy,ReportEmail,ReportTel,Notes,UPRN,Postcode,StreetAddress,GeoAreaURI,GeoAreaLabel
  ,http://opendatacommunities.org/id/district-council/adur,,http://id.esd.org.uk/service/579,Public toilets,BEACH GREEN PUBLIC CONVENIENCES BRIGHTON ROAD LANCING,OSGB36,518072,103649,http://www.ordnancesurvey.co.uk/business-and-government/help-and-support/public-sector/guidance/derived-data-exemptions.html,Female and male,Unisex,Yes,No,No,No,No,No,No,,,http://www.adur-worthing.gov.uk/streets-and-travel/public-toilets/,S = 09:00 - 21:00 W = 09:00 - 17:00 ,ADC,surveyors@adur-worthing.gov.uk,01903 221471,,60001449,,BEACH GREEN PUBLIC CONVENIENCES BRIGHTON ROAD LANCING,,
2014-07-07 00:00,http://opendatacommunities.org/id/district-council/adur,Adur,http://id.esd.org.uk/service/579,Public toilets,PUBLIC CONVENIENCES SHOPSDAM ROAD LANCING,OSGB3,518915,103795,http://www.ordnancesurvey.co.uk/business-and-government/help-and-support/public-sector/guidance/derived-data-exemptions.html,Mens,Unisex,Yes,No,No,No,No,No,No,,,http://www.adur-worthing.gov.uk/streets-and-travel/public-toilets/,S = 09:00 - 21:00 W = 09:00 - 17:00,ADC,surveyors@adur-worthing.gov.uk,01903 221471,,60007428,,PUBLIC CONVENIENCES SHOPSDAM ROAD LANCING,,
"#;
    let invalid_output: String = wrk.from_str(&wrk.path("data.csv.invalid"));
    assert_eq!(invalid_expected.to_string(), invalid_output);

    // check validation error output

    let validation_errors_expected = r#"{"valid":false,"errors":[{"keywordLocation":"/properties/ExtractDate/type","instanceLocation":"/ExtractDate","absoluteKeywordLocation":"https://example.com/properties/ExtractDate/type","error":"null is not of type \"string\""},{"keywordLocation":"/properties/OrganisationLabel/type","instanceLocation":"/OrganisationLabel","absoluteKeywordLocation":"https://example.com/properties/OrganisationLabel/type","error":"null is not of type \"string\""}],"row_index":1}
{"valid":false,"errors":[{"keywordLocation":"/properties/CoordinateReferenceSystem/pattern","instanceLocation":"/CoordinateReferenceSystem","absoluteKeywordLocation":"https://example.com/properties/CoordinateReferenceSystem/pattern","error":"\"OSGB3\" does not match \"(WGS84|OSGB36)\""},{"keywordLocation":"/properties/Category/pattern","instanceLocation":"/Category","absoluteKeywordLocation":"https://example.com/properties/Category/pattern","error":"\"Mens\" does not match \"(Female|Male|Female and Male|Unisex|Male urinal|Children only|None)\""}],"row_index":3}
"#;
    let validation_error_output: String =
        wrk.from_str(&wrk.path("data.csv.validation-errors.jsonl"));
    assert_eq!(
        validation_errors_expected.to_string(),
        validation_error_output
    );
}
