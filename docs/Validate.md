# Validate command


## Usecases

* validate according to json schema
* validate according to [RFC 4180](https://www.loc.gov/preservation/digital/formats/fdd/fdd000323.shtml)

## Notes

* json validator: https://github.com/Stranger6667/jsonschema-rs
* schema generator from code: didn't find existing rust library
  ** https://json-schema.org/implementations.html#from-data
* example validator https://github.com/Data-Liberation-Front/csvlint.io
### validate with existing jsonschema

[X] write docopt for command
[ ] POC using jsonschema
  [X] construct JSONSchema from URL
  [X] convert CSV record into JSON
    [X] Need support non-String types. Currently everything converted to JSON String, which fails with Schema requries Integer.
  [X] validate JSON via JSONSchema
[ ] write integrate test for valid and invalid cases
  * reference ruby project: https://github.com/Data-Liberation-Front/csvlint.rb/pull/38/files
  * example schemas: 
    ** https://json-schema.org/learn/examples/geographical-location.schema.json
    ** https://json-schema.org/learn/miscellaneous-examples.html
[ ] adopt jasonschema-rs to work for CSV data









