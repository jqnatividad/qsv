# Validate command

Validates CSV against [JSON Schema](https://json-schema.org/), or just against [RFC 4180](https://www.loc.gov/preservation/digital/formats/fdd/fdd000323.shtml).
## example usage

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


