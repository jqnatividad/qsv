use crate::workdir::Workdir;
use assert_json_diff::assert_json_eq;
use serde_json::Value;
use crate::test_validate::ADUR_CSV;

#[test]
fn generate_schema_no_value_constraints() {

    let csv = ADUR_CSV;

    let expected_schema = r#"
    {
        "$schema": "https://json-schema.org/draft-07/schema",
        "title": "JSON Schema for adur-public-toilets.csv",
        "description": "Inferred JSON Schema from QSV schema command",
        "type": "object",
        "properties": {
          "ExtractDate": {
            "type": [
              "string"
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
        },
        "required": [
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
        ]
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
    assert!(output_schema_string.len() > 0);
    let output_schema_json: Value = serde_json::from_str(&output_schema_string).unwrap();

    // make sure it's a valid JSON Schema by compiling with jsonschema library 
    jsonschema::JSONSchema::options().compile(&output_schema_json).expect("valid JSON Schema");

    // diff output json with expected json
    let expected_schema_json: Value = serde_json::from_str(&expected_schema.to_string()).unwrap();
    assert_json_eq!(
        expected_schema_json,
        output_schema_json
    );


}

#[test]
fn generate_schema_with_value_constraints_then_feed_into_validate() {

    let csv = ADUR_CSV;
    let expected_schema = r#"
    {
        "$schema": "https://json-schema.org/draft-07/schema",
        "title": "JSON Schema for adur-public-toilets.csv",
        "description": "Inferred JSON Schema from QSV schema command",
        "type": "object",
        "properties": {
        "ExtractDate": {
            "description": "ExtractDate column from adur-public-toilets.csv",
            "minLength": 2,
            "maxLength": 16,
            "type": [
            "string"
            ]
        },
        "OrganisationURI": {
            "description": "OrganisationURI column from adur-public-toilets.csv",
            "minLength": 55,
            "maxLength": 55,
            "type": [
            "string"
            ]
        },
        "OrganisationLabel": {
            "description": "OrganisationLabel column from adur-public-toilets.csv",
            "minLength": 0,
            "maxLength": 4,
            "type": [
            "string",
            "null"
            ]
        },
        "ServiceTypeURI": {
            "description": "ServiceTypeURI column from adur-public-toilets.csv",
            "minLength": 32,
            "maxLength": 32,
            "type": [
            "string"
            ]
        },
        "ServiceTypeLabel": {
            "description": "ServiceTypeLabel column from adur-public-toilets.csv",
            "minLength": 14,
            "maxLength": 14,
            "type": [
            "string"
            ]
        },
        "LocationText": {
            "description": "LocationText column from adur-public-toilets.csv",
            "minLength": 40,
            "maxLength": 86,
            "type": [
            "string"
            ]
        },
        "CoordinateReferenceSystem": {
            "description": "CoordinateReferenceSystem column from adur-public-toilets.csv",
            "minLength": 5,
            "maxLength": 6,
            "type": [
            "string"
            ]
        },
        "GeoX": {
            "description": "GeoX column from adur-public-toilets.csv",
            "minimum": 518072,
            "maximum": 524401,
            "type": [
            "integer"
            ]
        },
        "GeoY": {
            "description": "GeoY column from adur-public-toilets.csv",
            "minimum": 103649,
            "maximum": 106062,
            "type": [
            "integer"
            ]
        },
        "GeoPointLicensingURL": {
            "description": "GeoPointLicensingURL column from adur-public-toilets.csv",
            "minLength": 124,
            "maxLength": 124,
            "type": [
            "string"
            ]
        },
        "Category": {
            "description": "Category column from adur-public-toilets.csv",
            "minLength": 4,
            "maxLength": 15,
            "type": [
            "string"
            ]
        },
        "AccessibleCategory": {
            "description": "AccessibleCategory column from adur-public-toilets.csv",
            "minLength": 4,
            "maxLength": 6,
            "type": [
            "string"
            ]
        },
        "RADARKeyNeeded": {
            "description": "RADARKeyNeeded column from adur-public-toilets.csv",
            "minLength": 2,
            "maxLength": 3,
            "type": [
            "string"
            ]
        },
        "BabyChange": {
            "description": "BabyChange column from adur-public-toilets.csv",
            "minLength": 2,
            "maxLength": 2,
            "type": [
            "string"
            ]
        },
        "FamilyToilet": {
            "description": "FamilyToilet column from adur-public-toilets.csv",
            "minLength": 2,
            "maxLength": 2,
            "type": [
            "string"
            ]
        },
        "ChangingPlace": {
            "description": "ChangingPlace column from adur-public-toilets.csv",
            "minLength": 2,
            "maxLength": 2,
            "type": [
            "string"
            ]
        },
        "AutomaticPublicConvenience": {
            "description": "AutomaticPublicConvenience column from adur-public-toilets.csv",
            "minLength": 2,
            "maxLength": 2,
            "type": [
            "string"
            ]
        },
        "FullTimeStaffing": {
            "description": "FullTimeStaffing column from adur-public-toilets.csv",
            "minLength": 2,
            "maxLength": 2,
            "type": [
            "string"
            ]
        },
        "PartOfCommunityScheme": {
            "description": "PartOfCommunityScheme column from adur-public-toilets.csv",
            "minLength": 2,
            "maxLength": 2,
            "type": [
            "string"
            ]
        },
        "CommunitySchemeName": {
            "description": "CommunitySchemeName column from adur-public-toilets.csv",
            "type": [
            "null"
            ]
        },
        "ChargeAmount": {
            "description": "ChargeAmount column from adur-public-toilets.csv",
            "type": [
            "null"
            ]
        },
        "InfoURL": {
            "description": "InfoURL column from adur-public-toilets.csv",
            "minLength": 66,
            "maxLength": 66,
            "type": [
            "string"
            ]
        },
        "OpeningHours": {
            "description": "OpeningHours column from adur-public-toilets.csv",
            "minLength": 0,
            "maxLength": 36,
            "type": [
            "string",
            "null"
            ]
        },
        "ManagedBy": {
            "description": "ManagedBy column from adur-public-toilets.csv",
            "minLength": 0,
            "maxLength": 3,
            "type": [
            "string",
            "null"
            ]
        },
        "ReportEmail": {
            "description": "ReportEmail column from adur-public-toilets.csv",
            "minLength": 30,
            "maxLength": 30,
            "type": [
            "string"
            ]
        },
        "ReportTel": {
            "description": "ReportTel column from adur-public-toilets.csv",
            "minLength": 12,
            "maxLength": 12,
            "type": [
            "string"
            ]
        },
        "Notes": {
            "description": "Notes column from adur-public-toilets.csv",
            "minLength": 0,
            "maxLength": 29,
            "type": [
            "string",
            "null"
            ]
        },
        "UPRN": {
            "description": "UPRN column from adur-public-toilets.csv",
            "minimum": 60001449,
            "maximum": 60034215,
            "type": [
            "integer"
            ]
        },
        "Postcode": {
            "description": "Postcode column from adur-public-toilets.csv",
            "type": [
            "null"
            ]
        },
        "StreetAddress": {
            "description": "StreetAddress column from adur-public-toilets.csv",
            "minLength": 40,
            "maxLength": 86,
            "type": [
            "string"
            ]
        },
        "GeoAreaURI": {
            "description": "GeoAreaURI column from adur-public-toilets.csv",
            "type": [
            "null"
            ]
        },
        "GeoAreaLabel": {
            "description": "GeoAreaLabel column from adur-public-toilets.csv",
            "type": [
            "null"
            ]
        }
        },
        "required": [
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
        ]
    }
    "#;

    // create worksapce and invoke schema command with value constraints flag
    let wrk = Workdir::new("schema").flexible(true);
    wrk.create_from_string("adur-public-toilets.csv", csv);
    let mut cmd = wrk.command("schema");
    cmd.arg("adur-public-toilets.csv");
    cmd.arg("--value-constraints");
    wrk.output(&mut cmd);

    // load output schema file
    let output_schema_string: String =
        wrk.from_str(&wrk.path("adur-public-toilets.csv.schema.json"));
    let output_schema_json = serde_json::from_str(&output_schema_string).expect("parse schema json");

    // make sure it's a valid JSON Schema by compiling with jsonschema library 
    jsonschema::JSONSchema::options().compile(&output_schema_json).expect("valid JSON Schema");

    // diff output json with expected json
    let expected_schema_json: Value = serde_json::from_str(&expected_schema.to_string()).unwrap();
    assert_json_eq!(
        expected_schema_json,
        output_schema_json
    );

    // invoke validate command from schema created above
    let mut cmd2 = wrk.command("validate");
    cmd2.arg("adur-public-toilets.csv");
    cmd2.arg("adur-public-toilets.csv.schema.json");
    wrk.output(&mut cmd2);

    // validation report
    let validation_errors_expected = 
r#"{"valid":false,"errors":[{"keywordLocation":"/properties/ExtractDate/type","instanceLocation":"/ExtractDate","error":"null is not of type \"string\""}],"row_index":1}
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
