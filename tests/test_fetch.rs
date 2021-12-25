use crate::workdir::Workdir;

#[test]
fn fetch_simple() {
    let wrk = Workdir::new("fetch");
    wrk.create(
        "data.csv",
        vec![
            svec!["URL"],
            svec!["http://api.zippopotam.us/us/90210"],
            svec!["https://api.zippopotam.us/us/94105"],
            svec!["http://api.zippopotam.us/us/92802"],
            svec!["https://query.wikidata.org/sparql?query=SELECT%20?dob%20WHERE%20{wd:Q42%20wdt:P569%20?dob.}&format=json"],
        ],
    );
    let mut cmd = wrk.command("fetch");
    cmd.arg("URL").arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec![
            r#"{"post code": "90210", "country": "United States", "country abbreviation": "US", "places": [{"place name": "Beverly Hills", "longitude": "-118.4065", "state": "California", "state abbreviation": "CA", "latitude": "34.0901"}]}"#
        ],
        svec![
            r#"{"post code": "94105", "country": "United States", "country abbreviation": "US", "places": [{"place name": "San Francisco", "longitude": "-122.3892", "state": "California", "state abbreviation": "CA", "latitude": "37.7864"}]}"#
        ],
        svec![
            r#"{"post code": "92802", "country": "United States", "country abbreviation": "US", "places": [{"place name": "Anaheim", "longitude": "-117.9228", "state": "California", "state abbreviation": "CA", "latitude": "33.8085"}]}"#
        ],
        svec![
            r#"{
  "head" : {
    "vars" : [ "dob" ]
  },
  "results" : {
    "bindings" : [ {
      "dob" : {
        "datatype" : "http://www.w3.org/2001/XMLSchema#dateTime",
        "type" : "literal",
        "value" : "1952-03-11T00:00:00Z"
      }
    } ]
  }
}"#
        ],
    ];
    assert_eq!(got, expected);
}

#[test]
fn fetch_jql() {
    let wrk = Workdir::new("fetch");
    wrk.create(
        "data.csv",
        vec![
            svec!["URL"],
            svec!["http://api.zippopotam.us/us/90210"],
            svec!["http://api.zippopotam.us/us/94105"],
            svec!["https://api.zippopotam.us/us/92802"],
        ],
    );
    let mut cmd = wrk.command("fetch");
    cmd.arg("URL")
        .arg("--new-column")
        .arg("City")
        .arg("--jql")
        .arg(r#"."places"[0]."place name""#)
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["URL", "City"],
        svec!["http://api.zippopotam.us/us/90210", "Beverly Hills"],
        svec!["http://api.zippopotam.us/us/94105", "San Francisco"],
        svec!["https://api.zippopotam.us/us/92802", "Anaheim"],
    ];
    assert_eq!(got, expected);
}
