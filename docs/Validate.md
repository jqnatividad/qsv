# Validate command

Validates CSV against [JSON Schema](https://json-schema.org/), or just against [RFC 4180](https://www.loc.gov/preservation/digital/formats/fdd/fdd000323.shtml).

Uses [Stranger6667/jsonschema-rs](https://github.com/Stranger6667/jsonschema-rs) as validation engine, and supports JSON Schema Draft 4, 6, and 7.

## Usage Examples

### Simple Validation

people_schema.json
```
{
    "$id": "https://example.com/person.schema.json",
    "$schema": "https://json-schema.org/draft/2020-12/schema",
    "title": "Person",
    "type": "object",
    "properties": {
      "firstName": {
        "type": "string",
        "description": "The person's first name."
      },
      "lastName": {
        "type": "string",
        "description": "The person's last name.",
        "minLength": 2
      },
      "age": {
        "description": "Age in years which must be equal to or greater than 18.",
        "type": "integer",
        "minimum": 18
      }
    }
}
```

people.csv
```
firstName,lastName,age
John,Doe,21
Mickey,Mouse,10
Little,A,16
```

Example run
```
$ qsv validate people.csv people_schema.json  --quiet
$ ls people.csv*
people.csv  people.csv.invalid  people.csv.valid
$ cat people.csv.invalid 
firstName,lastName,age
Mickey,Mouse,10
Little,A,16
$ cat people.csv.valid
firstName,lastName,age
John,Doe,21
```

### Validation of the [Adur Public Toilets Dataset](https://datasets.opendata.esd.org.uk/details?submissionId=45)

adur-public-toilets.csv, with bad data manually inserted into rows 1 and 3
```
ExtractDate,OrganisationURI,OrganisationLabel,ServiceTypeURI,ServiceTypeLabel,LocationText,CoordinateReferenceSystem,GeoX,GeoY,GeoPointLicensingURL,Category,AccessibleCategory,RADARKeyNeeded,BabyChange,FamilyToilet,ChangingPlace,AutomaticPublicConvenience,FullTimeStaffing,PartOfCommunityScheme,CommunitySchemeName,ChargeAmount,InfoURL,OpeningHours,ManagedBy,ReportEmail,ReportTel,Notes,UPRN,Postcode,StreetAddress,GeoAreaURI,GeoAreaLabel
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

```

Manually crafted JSON Schema [public_toilets.schema.json](https://gist.githubusercontent.com/mhuang74/076c0cfdfaf14b05638569839744565a/raw/8f693f353201d1ab2c2255ed987b0b58a23dd4b9/public_toilets.schema.json) JSON Table Schema format, with a few validation rules ported as illustsration:
* 6 mandatory fields (ExtractDate, OrganisationLabel, ServiceTypeLabel, LocationText, GeoX, GeoY)
* ExtractDate: date format
* Category: allowed values
* CoordinateReferenceSystem: allowed values

example run:

```
$ QSV_LOG_LEVEL=debug qsvlite validate adur-public-toilets.csv https://gist.githubusercontent.com/mhuang74/076c0cfdfaf14b05638569839744565a/raw/8f693f353201d1ab2c2255ed987b0b58a23dd4b9/public_toilets.schema.json --quiet
2 out of 15 records invalid.

$ wc -l adur-public-toilets.csv*
   16 adur-public-toilets.csv
    3 adur-public-toilets.csv.invalid
   14 adur-public-toilets.csv.valid
    2 adur-public-toilets.csv.validation-errors.jsonl
```

adur-public-toilets.csv.validation-errors.jsonl shows 2 rows failed validation, with 4 rules violated.
```
{"valid":false,"errors":[{"keywordLocation":"/properties/ExtractDate/type","instanceLocation":"/ExtractDate","absoluteKeywordLocation":"https://example.com/properties/ExtractDate/type","error":"null is not of type \"string\""},{"keywordLocation":"/properties/OrganisationLabel/type","instanceLocation":"/OrganisationLabel","absoluteKeywordLocation":"https://example.com/properties/OrganisationLabel/type","error":"null is not of type \"string\""}],"row_index":1}
{"valid":false,"errors":[{"keywordLocation":"/properties/CoordinateReferenceSystem/pattern","instanceLocation":"/CoordinateReferenceSystem","absoluteKeywordLocation":"https://example.com/properties/CoordinateReferenceSystem/pattern","error":"\"OSGB3\" does not match \"(WGS84|OSGB36)\""},{"keywordLocation":"/properties/Category/pattern","instanceLocation":"/Category","absoluteKeywordLocation":"https://example.com/properties/Category/pattern","error":"\"Mens\" does not match \"(Female|Male|Female and Male|Unisex|Male urinal|Children only|None)\""}],"row_index":3}
```


