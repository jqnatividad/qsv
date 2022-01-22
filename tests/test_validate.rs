use crate::workdir::Workdir;

#[test]
fn validate_good_csv() {
    let wrk = Workdir::new("validate").flexible(true);
    wrk.create(
        "data.csv",
        vec![
            svec!["firstName", "lastName", "age"],
            svec!["Mickey", "Mouse", "10"],
            svec!["Donald", "Duck", "8"],
            svec!["Minie", "Mouse", "9"],
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
            svec!["firstName", "lastName", "age"],
            svec!["Mickey", "Mouse", "10"],
            svec!["Donald", "Duck"],
            svec!["Minie", "Mouse", "9"],
            ],
    );
    let mut cmd = wrk.command("validate");
    cmd.arg("data.csv");

    wrk.assert_err(&mut cmd);

}

#[test]
fn validate_with_json_schema() {

    // Public Toilets Schema
    // manually converted from JSON Table Schema
    // https://github.com/esd-org-uk/schemas/blob/master/PublicToilets/PublicToilets.json
    let schema = r#"
    {
        "$schema": "https://json-schema.org/draft/Draft-07",
        "$id": "https://example.com/public_toilets.schema.json",
        "title": "Public Toilets",
        "description": "List of Public Toilets",
        "type": "object",
        "properties": {
          "ExtractDate":
          {
            "description": "The date that the data was last extracted from its source database or manually updated",
            "type": ["string"]
           },
          "OrganisationURI":
          {
            "description": "URI from ODC of publishing organisation",
            "type": ["string", "null"]
          },
          "OrganisationLabel":
          {
            "description": "Label of publishing organisation. If empty uses this will be the same as the publishing organisation",
            "type": ["string"]
          },
          "ServiceTypeURI":
          {
            "description": "Service URI from http://id.esd.org.uk/list/services",
            "type": ["string", "null"]
          },
          "ServiceTypeLabel":
          {
            "description": "Label of the Service Type URI",
            "type": ["string"]
          },
          "LocationText":
          {
            "description": "Name of building, street, park etc.",
            "type": ["string"]
          },
          "CoordinateReferenceSystem":
          {
            "description": "‘WGS84’ (http://en.wikipedia.org/wiki/WGS84) or ‘OSGB36’ (http://en.wikipedia.org/wiki/Ordnance_Survey_National_Grid). WGS84 is the default.",
            "type": ["string", "null"]
          },
          "GeoX":
          {
            "description": "Longitude or east grid reference for centroid of application boundary",
            "type": "number"
          },
          "GeoY":
          {
            "description": "Latitude or north grid reference for centroid of application boundary",
            "type": "number"
          },
          "GeoPointLicensingURL":
          {
            "description": "URL of any page that describes any licensing restrictions on using the northing and easting.",
            "type": ["string", "null"]
          },
          "Category":
          {
            "description": "Use `None` if only accessible",
            "type": ["string", "null"]
          },
          "AccessibleCategory":
          {
            "description": "Use `None` if not accessible",
            "type": ["string", "null"]
          },
          "RADARKeyNeeded":
          {
            "description": "Flag to show whether a RADAR Key is needed",
            "type": ["string", "null"]
          },
          "BabyChange":
          {
            "description": "Flag to show whether there are baby changing facilities",
            "type": ["string", "null"]
          },
          "FamilyToilet":
          {
            "description": "Flag to show whether there is a family toilet",
            "type": ["string", "null"]
          },
          "ChangingPlace":
          {
            "description": "Flag to indicate whether the toilet includes changing facilities for people with profound and multiple learning disabilities and their carers, as well as many other disabled people. See http://www.changing-places.org/",
            "type": ["string", "null"]
          },
          "AutomaticPublicConvenience":
          {
            "description": "Flag to indicate whether this is an automatic public convenience (aka superloo)",
            "type": ["string", "null"]
          },
          "FullTimeStaffing":
          {
            "description": "Flag to indicate whether there is full time staffing",
            "type": ["string", "null"]
          },
          "PartOfCommunityScheme":
          {
            "description": "Flag to indicate whether it is part of a community scheme",
            "type": ["string", "null"]
          },
          "CommunitySchemeName":
          {
            "description": "The name of the community scheme",
            "type": ["string", "null"]
          },
          "ChargeAmount":
          {
            "description": "Charge amount in GBP to two decimal places. Omit the currency symbol",
            "type": ["number", "null"]
          },
          "InfoURL":
          {
            "description": "URL of web page describing the facilities",
            "type": ["string", "null"]
          },
          "OpeningHours":
          {
            "description": "Opening Hours",
            "type": ["string", "null"]
          },
          "ManagedBy":
          {
            "description": "Name of organisation managing the toilet",
            "type": ["string", "null"]
          },
          "ReportEmail":
          {
            "description": "Email address for reporting problems",
            "type": ["string", "null"]
          },
          "ReportTel":
          {
            "description": "Telephone number for reporting problems",
            "type": ["string", "null"]
          },
          "Notes":
          {
            "description": "Notes on reporting, accesibility, hours or anything else",
            "type": ["string", "null"]
          },
          "UPRN":
          {
            "description": "Unique Property Reference Number of the toilet",
            "type": ["string", "null"]
          },
          "Postcode":
          {
            "description": "Postcode of the toilet",
            "type": ["string", "null"]
          },
          "StreetAddress":
          {
            "description": "Street address of the toilet",
            "type": ["string", "null"]
          },
          "GeoAreaURI":
          {
            "description": "A predefined spatial area that the application is contained in.  Note that these can be derived from X/Y co-ordinates. Comma delimit in spreadsheet where there is more than one",
            "type": ["string", "null"]
          },
          "GeoAreaLabel":
          {
            "description": "Labels of the Geo Area URI. Comma delimit in spreadsheet where there is more than one",
            "type": ["string", "null"]
          }
        },
        "required": [ "ExtractDate", "OrganisationLabel", "ServiceTypeLabel", "LocationText", "GeoX", "GeoY" ]
      }
    "#;

    let csv =
r#"ExtractDate,OrganisationURI,OrganisationLabel,ServiceTypeURI,ServiceTypeLabel,LocationText,CoordinateReferenceSystem,GeoX,GeoY,GeoPointLicensingURL,Category,AccessibleCategory,RADARKeyNeeded,BabyChange,FamilyToilet,ChangingPlace,AutomaticPublicConvenience,FullTimeStaffing,PartOfCommunityScheme,CommunitySchemeName,ChargeAmount,InfoURL,OpeningHours,ManagedBy,ReportEmail,ReportTel,Notes,UPRN,Postcode,StreetAddress,GeoAreaURI,GeoAreaLabel
07/07/2014 00:00,http://opendatacommunities.org/id/district-council/adur,,http://id.esd.org.uk/service/579,Public toilets,BEACH GREEN PUBLIC CONVENIENCES BRIGHTON ROAD LANCING,OSGB36,518072,103649,http://www.ordnancesurvey.co.uk/business-and-government/help-and-support/public-sector/guidance/derived-data-exemptions.html,Female and male,Unisex,Yes,No,No,No,No,No,No,,,http://www.adur-worthing.gov.uk/streets-and-travel/public-toilets/,"S = 09:00 - 21:00 W = 09:00 - 17:00 ",ADC,surveyors@adur-worthing.gov.uk,01903 221471,,60001449,,BEACH GREEN PUBLIC CONVENIENCES BRIGHTON ROAD LANCING,,
07/07/2014 00:00,http://opendatacommunities.org/id/district-council/adur,Adur,http://id.esd.org.uk/service/579,Public toilets,PUBLIC CONVENIENCES MONKS RECREATION GROUND CRABTREE LANE LANCING,OSGB36,518225,104730,http://www.ordnancesurvey.co.uk/business-and-government/help-and-support/public-sector/guidance/derived-data-exemptions.html,Female and male,None,Yes,No,No,No,No,No,No,,,http://www.adur-worthing.gov.uk/streets-and-travel/public-toilets/,"S = 09:00 - 15:00 W = 09:00 - 15:00",ADC,surveyors@adur-worthing.gov.uk,01903 221471,,60002210,,PUBLIC CONVENIENCES MONKS RECREATION GROUND CRABTREE LANE LANCING,,
07/07/2014 00:00,http://opendatacommunities.org/id/district-council/adur,Adur,http://id.esd.org.uk/service/579,Public toilets,PUBLIC CONVENIENCES SHOPSDAM ROAD LANCING,OSGB36,518915,103795,http://www.ordnancesurvey.co.uk/business-and-government/help-and-support/public-sector/guidance/derived-data-exemptions.html,Female and male,Unisex,Yes,No,No,No,No,No,No,,,http://www.adur-worthing.gov.uk/streets-and-travel/public-toilets/,"S = 09:00 - 21:00 W = 09:00 - 17:00",ADC,surveyors@adur-worthing.gov.uk,01903 221471,,60007428,,PUBLIC CONVENIENCES SHOPSDAM ROAD LANCING,,
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

// missing value for 3rd column "OrganisationLabel"
// note: removed unnecessary quotes for string column "OpeningHours"
let invalid_expected =
r#"ExtractDate,OrganisationURI,OrganisationLabel,ServiceTypeURI,ServiceTypeLabel,LocationText,CoordinateReferenceSystem,GeoX,GeoY,GeoPointLicensingURL,Category,AccessibleCategory,RADARKeyNeeded,BabyChange,FamilyToilet,ChangingPlace,AutomaticPublicConvenience,FullTimeStaffing,PartOfCommunityScheme,CommunitySchemeName,ChargeAmount,InfoURL,OpeningHours,ManagedBy,ReportEmail,ReportTel,Notes,UPRN,Postcode,StreetAddress,GeoAreaURI,GeoAreaLabel
07/07/2014 00:00,http://opendatacommunities.org/id/district-council/adur,,http://id.esd.org.uk/service/579,Public toilets,BEACH GREEN PUBLIC CONVENIENCES BRIGHTON ROAD LANCING,OSGB36,518072,103649,http://www.ordnancesurvey.co.uk/business-and-government/help-and-support/public-sector/guidance/derived-data-exemptions.html,Female and male,Unisex,Yes,No,No,No,No,No,No,,,http://www.adur-worthing.gov.uk/streets-and-travel/public-toilets/,S = 09:00 - 21:00 W = 09:00 - 17:00 ,ADC,surveyors@adur-worthing.gov.uk,01903 221471,,60001449,,BEACH GREEN PUBLIC CONVENIENCES BRIGHTON ROAD LANCING,,
"#;

    let wrk = Workdir::new("validate").flexible(true);

    wrk.create_from_string("schema.json", schema);
    wrk.create_from_string("data.csv", csv);

    let mut cmd = wrk.command("validate");
    cmd.arg("data.csv").arg("schema.json");

    wrk.output(&mut cmd);

    let invalid_output: String = wrk.from_str(&wrk.path("data.csv.invalid"));

    assert_eq!(
        invalid_expected.to_string(),
        invalid_output
    );
}

