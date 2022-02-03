use crate::workdir::Workdir;
use assert_json_diff::assert_json_eq;
use serde_json::Value;

#[test]
fn infer_schema_for_adur_public_toilets_dataset() {

    // Adur Public Toilets Dataset
    // https://datasets.opendata.esd.org.uk/details?submissionId=45
    // Some errors were manually thrown in:
    // row 1: missing values for ExtractDate and OrganisationLabel
    // row 3: wrong value for CoordinateReferenceSystem and Category
    let csv = r#"ExtractDate,OrganisationURI,OrganisationLabel,ServiceTypeURI,ServiceTypeLabel,LocationText,CoordinateReferenceSystem,GeoX,GeoY,GeoPointLicensingURL,Category,AccessibleCategory,RADARKeyNeeded,BabyChange,FamilyToilet,ChangingPlace,AutomaticPublicConvenience,FullTimeStaffing,PartOfCommunityScheme,CommunitySchemeName,ChargeAmount,InfoURL,OpeningHours,ManagedBy,ReportEmail,ReportTel,Notes,UPRN,Postcode,StreetAddress,GeoAreaURI,GeoAreaLabel
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
07/07/2014 00:00,http://opendatacommunities.org/id/district-council/adur,Adur,http://id.esd.org.uk/service/579,Public toilets,PUBLIC CONVENIENCES CIVIC CENTRE HAM ROAD SHOREHAM-BY-SEA,OSGB36,522083,105168,http://www.ordnancesurvey.co.uk/business-and-government/help-and-support/public-sector/guidance/derived-data-exemptions.html,Female and male,Unisex,Yes,No,No,No,No,No,No,,,http://www.adur-worthing.gov.uk/streets-and-travel/public-toilets/,09.00 - 17.00,ADC,surveyors@adur-worthing.gov.uk,01903 221471,,60034215,,PUBLIC CONVENIENCES CIVIC CENTRE HAM ROAD SHOREHAM-BY-SEA,,
"#;

    let expected_schema = r#"
    {
        "$schema": "https://json-schema.org/draft-07/schema",
        "title": "JSON Schema for adur-public-toilets.csv",
        "description": "Inferred JSON Schema from QSV schema command",
        "type": "object",
        "properties": {
          "ExtractDate": {
            "type": [
              "string",
              "null"
            ],
            "description": "ExtractDate column from adur-public-toilets.csv"
          },
          "OrganisationURI": {
            "type": [
              "string"
            ],
            "description": "OrganisationURI column from adur-public-toilets.csv"
          },
          "OrganisationLabel": {
            "type": [
              "string",
              "null"
            ],
            "description": "OrganisationLabel column from adur-public-toilets.csv"
          },
          "ServiceTypeURI": {
            "type": [
              "string"
            ],
            "description": "ServiceTypeURI column from adur-public-toilets.csv"
          },
          "ServiceTypeLabel": {
            "type": [
              "string"
            ],
            "description": "ServiceTypeLabel column from adur-public-toilets.csv"
          },
          "LocationText": {
            "type": [
              "string"
            ],
            "description": "LocationText column from adur-public-toilets.csv"
          },
          "CoordinateReferenceSystem": {
            "type": [
              "string"
            ],
            "description": "CoordinateReferenceSystem column from adur-public-toilets.csv"
          },
          "GeoX": {
            "type": [
              "integer"
            ],
            "description": "GeoX column from adur-public-toilets.csv"
          },
          "GeoY": {
            "type": [
              "integer"
            ],
            "description": "GeoY column from adur-public-toilets.csv"
          },
          "GeoPointLicensingURL": {
            "type": [
              "string"
            ],
            "description": "GeoPointLicensingURL column from adur-public-toilets.csv"
          },
          "Category": {
            "type": [
              "string"
            ],
            "description": "Category column from adur-public-toilets.csv"
          },
          "AccessibleCategory": {
            "type": [
              "string"
            ],
            "description": "AccessibleCategory column from adur-public-toilets.csv"
          },
          "RADARKeyNeeded": {
            "type": [
              "string"
            ],
            "description": "RADARKeyNeeded column from adur-public-toilets.csv"
          },
          "BabyChange": {
            "type": [
              "string"
            ],
            "description": "BabyChange column from adur-public-toilets.csv"
          },
          "FamilyToilet": {
            "type": [
              "string"
            ],
            "description": "FamilyToilet column from adur-public-toilets.csv"
          },
          "ChangingPlace": {
            "type": [
              "string"
            ],
            "description": "ChangingPlace column from adur-public-toilets.csv"
          },
          "AutomaticPublicConvenience": {
            "type": [
              "string"
            ],
            "description": "AutomaticPublicConvenience column from adur-public-toilets.csv"
          },
          "FullTimeStaffing": {
            "type": [
              "string"
            ],
            "description": "FullTimeStaffing column from adur-public-toilets.csv"
          },
          "PartOfCommunityScheme": {
            "type": [
              "string"
            ],
            "description": "PartOfCommunityScheme column from adur-public-toilets.csv"
          },
          "CommunitySchemeName": {
            "type": [
              "null"
            ],
            "description": "CommunitySchemeName column from adur-public-toilets.csv"
          },
          "ChargeAmount": {
            "type": [
              "null"
            ],
            "description": "ChargeAmount column from adur-public-toilets.csv"
          },
          "InfoURL": {
            "type": [
              "string"
            ],
            "description": "InfoURL column from adur-public-toilets.csv"
          },
          "OpeningHours": {
            "type": [
              "string",
              "null"
            ],
            "description": "OpeningHours column from adur-public-toilets.csv"
          },
          "ManagedBy": {
            "type": [
              "string",
              "null"
            ],
            "description": "ManagedBy column from adur-public-toilets.csv"
          },
          "ReportEmail": {
            "type": [
              "string"
            ],
            "description": "ReportEmail column from adur-public-toilets.csv"
          },
          "ReportTel": {
            "type": [
              "string"
            ],
            "description": "ReportTel column from adur-public-toilets.csv"
          },
          "Notes": {
            "type": [
              "string",
              "null"
            ],
            "description": "Notes column from adur-public-toilets.csv"
          },
          "UPRN": {
            "type": [
              "integer"
            ],
            "description": "UPRN column from adur-public-toilets.csv"
          },
          "Postcode": {
            "type": [
              "null"
            ],
            "description": "Postcode column from adur-public-toilets.csv"
          },
          "StreetAddress": {
            "type": [
              "string"
            ],
            "description": "StreetAddress column from adur-public-toilets.csv"
          },
          "GeoAreaURI": {
            "type": [
              "null"
            ],
            "description": "GeoAreaURI column from adur-public-toilets.csv"
          },
          "GeoAreaLabel": {
            "type": [
              "null"
            ],
            "description": "GeoAreaLabel column from adur-public-toilets.csv"
          }
        }
      }
    "#;

    let wrk = Workdir::new("schema").flexible(true);

    wrk.create_from_string("adur-public-toilets.csv", csv);

    let mut cmd = wrk.command("schema");
    cmd.arg("adur-public-toilets.csv");

    wrk.output(&mut cmd);

    // load output schema file
    let output_schema_string: String =
        wrk.from_str(&wrk.path("adur-public-toilets.csv.schema.json"));

    let output_schema_json: Value = serde_json::from_str(&output_schema_string).unwrap();
    let expected_schema_json: Value = serde_json::from_str(&expected_schema.to_string()).unwrap();

    // diff output json with expected json
    assert_json_eq!(
        expected_schema_json,
        output_schema_json
    );
}
