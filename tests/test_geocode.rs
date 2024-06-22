// NOTE: these tests are meant for the cities15000 dataset
use serial_test::serial;

use crate::workdir::Workdir;

#[test]
fn geocode_suggest() {
    let wrk = Workdir::new("geocode_suggest");
    wrk.create(
        "data.csv",
        vec![
            svec!["Location"],
            svec!["Melrose, New York"],
            svec!["East Flatbush, New York"],
            svec!["Manhattan, New York"],
            svec!["Brooklyn, New York"],
            svec!["East Harlem, New York"],
            svec!["This is not a Location and it will not be geocoded"],
            svec!["Jersey City, New Jersey"],
            svec!["95.213424, 190,1234565"], // invalid lat, long
            svec!["Makati, Metro Manila, Philippines"],
        ],
    );
    let mut cmd = wrk.command("geocode");
    cmd.arg("suggest").arg("Location").arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["Location"],
        svec!["(41.90059, -87.85673)"],
        svec!["(28.11085, -82.69482)"],
        svec!["(40.71427, -74.00597)"],
        svec!["(45.09413, -93.35634)"],
        svec!["(40.79472, -73.9425)"],
        svec!["This is not a Location and it will not be geocoded"],
        svec!["(40.72816, -74.07764)"],
        svec!["95.213424, 190,1234565"], // suggest expects a city name, not lat, long
        svec!["(14.55027, 121.03269)"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn geocode_suggest_select() {
    let wrk = Workdir::new("geocode_suggest_select");
    wrk.create(
        "data.csv",
        vec![
            svec!["c1", "c2", "Location"],
            svec!["1", "2", "Melrose, New York"],
            svec!["3", "4", "East Flatbush, New York"],
            svec!["5", "6", "Manhattan, New York"],
            svec!["7", "8", "Brooklyn, New York"],
            svec!["9", "10", "East Harlem, New York"],
            svec![
                "11",
                "12",
                "This is not a Location and it will not be geocoded"
            ],
            svec!["13", "14", "Jersey City, New Jersey"],
            svec!["15", "16", "95.213424, 190,1234565"], // invalid lat, long
            svec!["17", "18", "Makati, Metro Manila, Philippines"],
        ],
    );
    let mut cmd = wrk.command("geocode");
    // use select syntax to select the last column
    cmd.arg("suggest").arg("_").arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["c1", "c2", "Location"],
        svec!["1", "2", "(41.90059, -87.85673)"],
        svec!["3", "4", "(28.11085, -82.69482)"],
        svec!["5", "6", "(40.71427, -74.00597)"],
        svec!["7", "8", "(45.09413, -93.35634)"],
        svec!["9", "10", "(40.79472, -73.9425)"],
        svec![
            "11",
            "12",
            "This is not a Location and it will not be geocoded"
        ],
        svec!["13", "14", "(40.72816, -74.07764)"],
        svec!["15", "16", "95.213424, 190,1234565"], // suggest expects a city name, not lat, long
        svec!["17", "18", "(14.55027, 121.03269)"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn geocode_suggestnow_default() {
    let wrk = Workdir::new("geocode_suggestnow_default");
    let mut cmd = wrk.command("geocode");
    cmd.arg("suggestnow").arg("Brooklyn");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["Location"],
        svec!["Brooklyn, New York US: 40.6501, -73.94958"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn geocode_suggestnow_formatstr_dyncols() {
    let wrk = Workdir::new("geocode_suggestnow_formatstr_dyncols");

    let mut cmd = wrk.command("geocode");
    cmd.arg("suggestnow").arg("Secaucus").args([
        "--formatstr",
        "%dyncols: {population:population}, {state:admin1}, {county:admin2}, \
         {state_fips:us_state_fips_code}, {county_fips:us_county_fips_code}, {timezone:timezone}",
    ]);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec![
            "Location",
            "population",
            "state",
            "county",
            "state_fips",
            "county_fips",
            "timezone",
        ],
        svec![
            "Secaucus",
            "19104",
            "New Jersey",
            "Hudson",
            "34",
            "017",
            "America/New_York"
        ],
    ];
    assert_eq!(got, expected);
}

#[test]
fn geocode_suggest_intl() {
    let wrk = Workdir::new("geocode_suggest_intl");
    wrk.create(
        "data.csv",
        vec![
            svec!["Location"],
            svec!["Paris"],
            svec!["Manila"],
            svec!["London"],
            svec!["Berlin"],
            svec!["Moscow"],
            svec!["This is not a Location and it will not be geocoded"],
            svec!["Brazil"],
            svec!["95.213424, 190,1234565"], // invalid lat, long
            svec!["Havana"],
        ],
    );
    let mut cmd = wrk.command("geocode");
    cmd.arg("suggest")
        .arg("Location")
        .args(["-f", "%city-admin1-country"])
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["Location"],
        svec!["Paris, Île-de-France Region FR"],
        svec!["Manila, National Capital Region PH"],
        svec!["London, England GB"],
        svec!["Berlin,  DE"],
        svec!["Moscow, Moscow RU"],
        svec!["This is not a Location and it will not be geocoded"],
        svec!["Brasília, Federal District BR"],
        svec!["95.213424, 190,1234565"],
        svec!["Havana, La Habana Province CU"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn geocode_suggest_intl_country_filter() {
    let wrk = Workdir::new("geocode_suggest_intl_country_filter");
    wrk.create(
        "data.csv",
        vec![
            svec!["Location"],
            svec!["Paris"],
            svec!["Manila"],
            svec!["London"],
            svec!["Berlin"],
            svec!["Moscow"],
            svec!["This is not a Location and it will not be geocoded"],
            svec!["Brazil"],
            svec!["95.213424, 190,1234565"], // invalid lat, long
            svec!["Havana"],
        ],
    );
    let mut cmd = wrk.command("geocode");
    cmd.arg("suggest")
        .arg("Location")
        .args(["--country", "US"])
        .args(["-f", "%city-admin1-country"])
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["Location"],
        svec!["Paris, Texas US"],
        svec!["Manteca, California US"],
        svec!["Sterling, Virginia US"],
        svec!["Burlington, North Carolina US"],
        svec!["Moscow, Idaho US"],
        svec!["This is not a Location and it will not be geocoded"],
        svec!["Brawley, California US"],
        svec!["95.213424, 190,1234565"],
        svec!["Savannah, Georgia US"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn geocode_suggest_intl_admin1_filter_error() {
    let wrk = Workdir::new("geocode_suggest_intl_admin1_filter_error");
    wrk.create(
        "data.csv",
        vec![
            svec!["Location"],
            svec!["Paris"],
            svec!["Manila"],
            svec!["London"],
        ],
    );
    let mut cmd = wrk.command("geocode");
    cmd.arg("suggest")
        .arg("Location")
        .args(["--admin1", "US"])
        .args(["-f", "%city-admin1-country"])
        .arg("data.csv");

    // admin1 requires a country filter
    wrk.assert_err(&mut cmd);
}

#[test]
fn geocode_suggestnow() {
    let wrk = Workdir::new("geocode_suggestnow");

    let mut cmd = wrk.command("geocode");
    cmd.arg("suggestnow")
        .arg("Paris")
        .args(["--country", "US"])
        .args(["-f", "%city-admin1-country"]);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["Location"], svec!["Paris, Texas US"]];
    assert_eq!(got, expected);
}

#[test]
fn geocode_reversenow() {
    let wrk = Workdir::new("geocode_reversenow");

    let mut cmd = wrk.command("geocode");
    cmd.arg("reversenow").arg("(40.67, -73.94)").args([
        "-f",
        "{name}, {admin2} County, {admin1} - {population} {timezone}",
    ]);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["Location"],
        svec!["Brownsville, Kings County, New York - 74497 America/New_York"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn geocode_reversenow_error() {
    let wrk = Workdir::new("geocode_reversenow_error");

    let mut cmd = wrk.command("geocode");
    cmd.arg("reversenow")
        .arg("(40.67, -73.94)")
        .args(["--admin1", "New York"])
        .args([
            "-f",
            "{name}, {admin2} County, {admin1} - {population} {timezone}",
        ]);

    // reversenow does not support admin1 filter
    wrk.assert_err(&mut cmd);
}

#[test]
fn geocode_suggest_intl_admin1_filter_country_inferencing() {
    let wrk = Workdir::new("geocode_suggest_intl_admin1_filter_country_inferencing");
    wrk.create(
        "data.csv",
        vec![
            svec!["Location"],
            svec!["Paris"],
            svec!["Manila"],
            svec!["London"],
        ],
    );
    let mut cmd = wrk.command("geocode");
    cmd.arg("suggest")
        .arg("Location")
        .args(["--admin1", "US.NJ,US.TX,US.NY"])
        .args(["-f", "%city-admin1-country"])
        .arg("data.csv");

    // admin1 requires a country filter
    // however, since all the admin1 filters are in the US
    // or more specifically, the admin1 filters have a prefix of "US."
    // the country filter is inferred to be "US"
    wrk.assert_success(&mut cmd);
}

#[test]
fn geocode_suggest_intl_multi_country_filter() {
    let wrk = Workdir::new("geocode_suggest_intl_multi_country_filter");
    wrk.create(
        "data.csv",
        vec![
            svec!["Location"],
            svec!["Paris"],
            svec!["Manila"],
            svec!["London"],
            svec!["Berlin"],
            svec!["Moscow"],
            svec!["This is not a Location and it will not be geocoded"],
            svec!["Brazil"],
            svec!["95.213424, 190,1234565"], // invalid lat, long
            svec!["Havana"],
        ],
    );
    let mut cmd = wrk.command("geocode");
    cmd.arg("suggest")
        .arg("Location")
        .args(["--country", "us,FR,ru"])
        .args(["-f", "%city-admin1-country"])
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["Location"],
        svec!["Paris, Île-de-France Region FR"],
        svec!["Manteca, California US"],
        svec!["Sterling, Virginia US"],
        svec!["Burlington, North Carolina US"],
        svec!["Moscow, Moscow RU"],
        svec!["This is not a Location and it will not be geocoded"],
        svec!["Brawley, California US"],
        svec!["95.213424, 190,1234565"],
        svec!["Savannah, Georgia US"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn geocode_suggest_filter_country_admin1() {
    let wrk = Workdir::new("geocode_suggest_filter_country_admin1");
    wrk.create(
        "data.csv",
        vec![
            svec!["Location"],
            svec!["Melrose, New York"],
            svec!["East Flatbush, New York"],
            svec!["Manhattan, New York"],
            svec!["Brooklyn, New York"],
            svec!["East Harlem, New York"],
            svec!["This is not a Location and it will not be geocoded"],
            svec!["Jersey City, New Jersey"],
            svec!["(41.90059, -87.85673)"],
            svec!["Makati, Metro Manila, Philippines"],
        ],
    );
    let mut cmd = wrk.command("geocode");
    cmd.arg("suggest")
        .arg("Location")
        .args(["-f", "{name}, {admin1}, {admin2} {country}"])
        .args(["--country", "US"])
        .args(["--admin1", "US.NY,New J,Metro Manila"])
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["Location"],
        svec!["Melrose, New York, Bronx County US"],
        svec!["Elmwood Park, New Jersey, Bergen County US"],
        svec!["New York, New York,  US"],
        svec!["Brooklyn, New York, Kings US"],
        svec!["East Harlem, New York, New York County US"],
        svec!["This is not a Location and it will not be geocoded"],
        // Jersey City matched as the admin1 filter included "New J"
        // which starts_with match "New Jersey"
        svec!["Jersey City, New Jersey, Hudson US"],
        // suggest expects a city name, not lat, long
        svec!["(41.90059, -87.85673)"],
        // Makati did not match, even with the Metro Manila admin1 filter
        // as the country filter was set to US
        // as a result, the country filter takes precedence over the admin1 filter
        // and the closest match for Makati in the US is McAllen in Texas
        svec!["McKinney, Texas, Collin US"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn geocode_suggest_invalid() {
    let wrk = Workdir::new("geocode_suggest_invalid");
    wrk.create(
        "data.csv",
        vec![
            svec!["Location"],
            svec!["Melrose, New York"],
            svec!["East Flatbush, New York"],
            svec!["Manhattan, New York"],
            svec!["East Harlem, New York"],
            svec!["Brooklyn, New York"],
            svec!["This is not a Location and it will not be geocoded"],
            svec!["Jersey City, New Jersey"],
            svec!["95.213424, 190,1234565"], // invalid lat, long
            svec!["Makati, Metro Manila, Philippines"],
        ],
    );
    let mut cmd = wrk.command("geocode");
    cmd.arg("suggest")
        .arg("Location")
        .args(["--invalid-result", "<ERROR>"])
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["Location"],
        svec!["(41.90059, -87.85673)"],
        svec!["(28.11085, -82.69482)"],
        svec!["(40.71427, -74.00597)"],
        svec!["(40.79472, -73.9425)"],
        svec!["(45.09413, -93.35634)"],
        svec!["<ERROR>"],
        svec!["(40.72816, -74.07764)"],
        svec!["<ERROR>"], // suggest expects a city name, not lat, long
        svec!["(14.55027, 121.03269)"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn geocode_suggest_dynfmt() {
    let wrk = Workdir::new("geocode_suggest_dynfmt");
    wrk.create(
        "data.csv",
        vec![
            svec!["Location"],
            svec!["Melrose, New York"],
            svec!["East Flatbush, New York"],
            svec!["Manhattan, New York"],
            svec!["East Harlem, New York"],
            svec!["This is not a Location and it will not be geocoded"],
            svec!["95.213424, 190,1234565"], // invalid lat, long
            svec!["Makati, Metro Manila, Philippines"],
        ],
    );
    let mut cmd = wrk.command("geocode");
    cmd.arg("suggest")
        .arg("Location")
        .arg("--formatstr")
        .arg(
            "{latitude}:{longitude} - {name}, {admin1}:{us_state_fips_code}-{us_county_fips_code} \
             {country} {continent} {currency_code} {neighbours}",
        )
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["Location"],
        svec!["41.90059:-87.85673 - Melrose Park, Illinois:17-031 US NA USD CA,MX,CU"],
        svec!["28.11085:-82.69482 - East Lake, Florida:12-003 US NA USD CA,MX,CU"],
        svec!["40.71427:-74.00597 - New York, New York:36- US NA USD CA,MX,CU"],
        svec!["40.79472:-73.9425 - East Harlem, New York:36-061 US NA USD CA,MX,CU"],
        svec!["This is not a Location and it will not be geocoded"],
        svec!["95.213424, 190,1234565"], // invalid lat, long
        svec!["14.55027:121.03269 - Makati City, National Capital Region:- PH AS PHP "],
    ];
    assert_eq!(got, expected);
}

#[test]
fn geocode_suggest_pretty_json() {
    let wrk = Workdir::new("geocode_suggest_pretty_json");
    wrk.create(
        "data.csv",
        vec![
            svec!["Location"],
            svec!["Melrose, New York"],
            svec!["East Flatbush, New York"],
            svec!["Manhattan, New York"],
            svec!["East Harlem, New York"],
            svec!["This is not a Location and it will not be geocoded"],
            svec!["95.213424, 190,1234565"], // invalid lat, long
            svec!["Makati, Metro Manila, Philippines"],
        ],
    );
    let mut cmd = wrk.command("geocode");
    cmd.arg("suggest")
        .arg("Location")
        .arg("--formatstr")
        .arg("%pretty-json")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["Location"],
        svec!["{\n  \"cityrecord\":{\n  \"id\": 4901868,\n  \"name\": \"Melrose Park\",\n  \"latitude\": 41.90059,\n  \"longitude\": -87.85673,\n  \"country\": {\n    \"id\": 6252001,\n    \"code\": \"US\",\n    \"name\": \"United States\"\n  },\n  \"admin_division\": {\n    \"id\": 4896861,\n    \"code\": \"US.IL\",\n    \"name\": \"Illinois\"\n  },\n  \"admin2_division\": {\n    \"id\": 4888671,\n    \"code\": \"US.IL.031\",\n    \"name\": \"Cook County\"\n  },\n  \"timezone\": \"America/Chicago\",\n  \"names\": {\n    \"en\": \"Melrose Park\"\n  },\n  \"country_names\": {\n    \"en\": \"United States\"\n  },\n  \"admin1_names\": {\n    \"en\": \"Illinois\"\n  },\n  \"admin2_names\": {\n    \"en\": \"Cook\"\n  },\n  \"population\": 25379\n},\n  \"countryrecord\":{\n  \"info\": {\n    \"iso\": \"US\",\n    \"iso3\": \"USA\",\n    \"iso_numeric\": \"840\",\n    \"fips\": \"US\",\n    \"name\": \"United States\",\n    \"capital\": \"Washington\",\n    \"area\": \"9629091\",\n    \"population\": 327167434,\n    \"continent\": \"NA\",\n    \"tld\": \".us\",\n    \"currency_code\": \"USD\",\n    \"currency_name\": \"Dollar\",\n    \"phone\": \"1\",\n    \"postal_code_format\": \"#####-####\",\n    \"postal_code_regex\": \"^\\\\d{5}(-\\\\d{4})?$\",\n    \"languages\": \"en-US,es-US,haw,fr\",\n    \"geonameid\": 6252001,\n    \"neighbours\": \"CA,MX,CU\",\n    \"equivalent_fips_code\": \"\"\n  },\n  \"names\": {\n    \"en\": \"United States\"\n  },\n  \"capital_names\": {\n    \"en\": \"Washington D.C.\"\n  }\n}\n \"us_fips_codes\":{\n  \"us_state_code\": \"IL\",\n  \"us_state_name\": \"Illinois\",\n  \"us_state_fips_code\": \"17\",\n  \"us_county\": \"Cook\",\n  \"us_county_fips_code\": \"031\"\n}\n}"], 
        svec!["{\n  \"cityrecord\":{\n  \"id\": 4154008,\n  \"name\": \"East Lake\",\n  \"latitude\": 28.11085,\n  \"longitude\": -82.69482,\n  \"country\": {\n    \"id\": 6252001,\n    \"code\": \"US\",\n    \"name\": \"United States\"\n  },\n  \"admin_division\": {\n    \"id\": 4155751,\n    \"code\": \"US.FL\",\n    \"name\": \"Florida\"\n  },\n  \"admin2_division\": {\n    \"id\": 4168618,\n    \"code\": \"US.FL.103\",\n    \"name\": \"Pinellas County\"\n  },\n  \"timezone\": \"America/New_York\",\n  \"names\": null,\n  \"country_names\": {\n    \"en\": \"United States\"\n  },\n  \"admin1_names\": {\n    \"en\": \"Florida\"\n  },\n  \"admin2_names\": {\n    \"en\": \"Pinellas\"\n  },\n  \"population\": 30962\n},\n  \"countryrecord\":{\n  \"info\": {\n    \"iso\": \"US\",\n    \"iso3\": \"USA\",\n    \"iso_numeric\": \"840\",\n    \"fips\": \"US\",\n    \"name\": \"United States\",\n    \"capital\": \"Washington\",\n    \"area\": \"9629091\",\n    \"population\": 327167434,\n    \"continent\": \"NA\",\n    \"tld\": \".us\",\n    \"currency_code\": \"USD\",\n    \"currency_name\": \"Dollar\",\n    \"phone\": \"1\",\n    \"postal_code_format\": \"#####-####\",\n    \"postal_code_regex\": \"^\\\\d{5}(-\\\\d{4})?$\",\n    \"languages\": \"en-US,es-US,haw,fr\",\n    \"geonameid\": 6252001,\n    \"neighbours\": \"CA,MX,CU\",\n    \"equivalent_fips_code\": \"\"\n  },\n  \"names\": {\n    \"en\": \"United States\"\n  },\n  \"capital_names\": {\n    \"en\": \"Washington D.C.\"\n  }\n}\n \"us_fips_codes\":{\n  \"us_state_code\": \"FL\",\n  \"us_state_name\": \"Florida\",\n  \"us_state_fips_code\": \"12\",\n  \"us_county\": \"Pinellas\",\n  \"us_county_fips_code\": \"003\"\n}\n}"], 
        svec!["{\n  \"cityrecord\":{\n  \"id\": 5128581,\n  \"name\": \"New York City\",\n  \"latitude\": 40.71427,\n  \"longitude\": -74.00597,\n  \"country\": {\n    \"id\": 6252001,\n    \"code\": \"US\",\n    \"name\": \"United States\"\n  },\n  \"admin_division\": {\n    \"id\": 5128638,\n    \"code\": \"US.NY\",\n    \"name\": \"New York\"\n  },\n  \"admin2_division\": null,\n  \"timezone\": \"America/New_York\",\n  \"names\": {\n    \"en\": \"New York\"\n  },\n  \"country_names\": {\n    \"en\": \"United States\"\n  },\n  \"admin1_names\": {\n    \"en\": \"New York\"\n  },\n  \"admin2_names\": null,\n  \"population\": 8804190\n},\n  \"countryrecord\":{\n  \"info\": {\n    \"iso\": \"US\",\n    \"iso3\": \"USA\",\n    \"iso_numeric\": \"840\",\n    \"fips\": \"US\",\n    \"name\": \"United States\",\n    \"capital\": \"Washington\",\n    \"area\": \"9629091\",\n    \"population\": 327167434,\n    \"continent\": \"NA\",\n    \"tld\": \".us\",\n    \"currency_code\": \"USD\",\n    \"currency_name\": \"Dollar\",\n    \"phone\": \"1\",\n    \"postal_code_format\": \"#####-####\",\n    \"postal_code_regex\": \"^\\\\d{5}(-\\\\d{4})?$\",\n    \"languages\": \"en-US,es-US,haw,fr\",\n    \"geonameid\": 6252001,\n    \"neighbours\": \"CA,MX,CU\",\n    \"equivalent_fips_code\": \"\"\n  },\n  \"names\": {\n    \"en\": \"United States\"\n  },\n  \"capital_names\": {\n    \"en\": \"Washington D.C.\"\n  }\n}\n \"us_fips_codes\":{\n  \"us_state_code\": \"NY\",\n  \"us_state_name\": \"New York\",\n  \"us_state_fips_code\": \"36\",\n  \"us_county\": \"\",\n  \"us_county_fips_code\": \"\"\n}\n}"], 
        svec!["{\n  \"cityrecord\":{\n  \"id\": 6332428,\n  \"name\": \"East Harlem\",\n  \"latitude\": 40.79472,\n  \"longitude\": -73.9425,\n  \"country\": {\n    \"id\": 6252001,\n    \"code\": \"US\",\n    \"name\": \"United States\"\n  },\n  \"admin_division\": {\n    \"id\": 5128638,\n    \"code\": \"US.NY\",\n    \"name\": \"New York\"\n  },\n  \"admin2_division\": {\n    \"id\": 5128594,\n    \"code\": \"US.NY.061\",\n    \"name\": \"New York County\"\n  },\n  \"timezone\": \"America/New_York\",\n  \"names\": {\n    \"en\": \"East Harlem\"\n  },\n  \"country_names\": {\n    \"en\": \"United States\"\n  },\n  \"admin1_names\": {\n    \"en\": \"New York\"\n  },\n  \"admin2_names\": {\n    \"en\": \"New York County\"\n  },\n  \"population\": 115921\n},\n  \"countryrecord\":{\n  \"info\": {\n    \"iso\": \"US\",\n    \"iso3\": \"USA\",\n    \"iso_numeric\": \"840\",\n    \"fips\": \"US\",\n    \"name\": \"United States\",\n    \"capital\": \"Washington\",\n    \"area\": \"9629091\",\n    \"population\": 327167434,\n    \"continent\": \"NA\",\n    \"tld\": \".us\",\n    \"currency_code\": \"USD\",\n    \"currency_name\": \"Dollar\",\n    \"phone\": \"1\",\n    \"postal_code_format\": \"#####-####\",\n    \"postal_code_regex\": \"^\\\\d{5}(-\\\\d{4})?$\",\n    \"languages\": \"en-US,es-US,haw,fr\",\n    \"geonameid\": 6252001,\n    \"neighbours\": \"CA,MX,CU\",\n    \"equivalent_fips_code\": \"\"\n  },\n  \"names\": {\n    \"en\": \"United States\"\n  },\n  \"capital_names\": {\n    \"en\": \"Washington D.C.\"\n  }\n}\n \"us_fips_codes\":{\n  \"us_state_code\": \"NY\",\n  \"us_state_name\": \"New York\",\n  \"us_state_fips_code\": \"36\",\n  \"us_county\": \"New York County\",\n  \"us_county_fips_code\": \"061\"\n}\n}"], 
        svec!["This is not a Location and it will not be geocoded"], 
        svec!["95.213424, 190,1234565"], 
        svec!["{\n  \"cityrecord\":{\n  \"id\": 1703417,\n  \"name\": \"Makati City\",\n  \"latitude\": 14.55027,\n  \"longitude\": 121.03269,\n  \"country\": {\n    \"id\": 1694008,\n    \"code\": \"PH\",\n    \"name\": \"Philippines\"\n  },\n  \"admin_division\": {\n    \"id\": 7521311,\n    \"code\": \"PH.NCR\",\n    \"name\": \"Metro Manila\"\n  },\n  \"admin2_division\": {\n    \"id\": 11395838,\n    \"code\": \"PH.NCR.137600000\",\n    \"name\": \"Southern Manila District\"\n  },\n  \"timezone\": \"Asia/Manila\",\n  \"names\": {\n    \"en\": \"Makati City\"\n  },\n  \"country_names\": {\n    \"en\": \"Philippines\"\n  },\n  \"admin1_names\": {\n    \"en\": \"National Capital Region\"\n  },\n  \"admin2_names\": null,\n  \"population\": 510383\n},\n  \"countryrecord\":{\n  \"info\": {\n    \"iso\": \"PH\",\n    \"iso3\": \"PHL\",\n    \"iso_numeric\": \"608\",\n    \"fips\": \"RP\",\n    \"name\": \"Philippines\",\n    \"capital\": \"Manila\",\n    \"area\": \"300000\",\n    \"population\": 106651922,\n    \"continent\": \"AS\",\n    \"tld\": \".ph\",\n    \"currency_code\": \"PHP\",\n    \"currency_name\": \"Peso\",\n    \"phone\": \"63\",\n    \"postal_code_format\": \"####\",\n    \"postal_code_regex\": \"^(\\\\d{4})$\",\n    \"languages\": \"tl,en-PH,fil,ceb,ilo,hil,war,pam,bik,bcl,pag,mrw,tsg,mdh,cbk,krj,sgd,msb,akl,ibg,yka,mta,abx\",\n    \"geonameid\": 1694008,\n    \"neighbours\": \"\",\n    \"equivalent_fips_code\": \"\"\n  },\n  \"names\": {\n    \"en\": \"Philippines\"\n  },\n  \"capital_names\": {\n    \"en\": \"Manila\"\n  }\n}\n \"us_fips_codes\":{\n  \"us_state_code\": \"\",\n  \"us_state_name\": \"National Capital Region\",\n  \"us_state_fips_code\": \"null\",\n  \"us_county\": \"\",\n  \"us_county_fips_code\": \"\"\n}\n}"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn geocode_suggest_invalid_dynfmt() {
    let wrk = Workdir::new("geocode_suggest_invalid_dynfmt");
    wrk.create(
        "data.csv",
        vec![
            svec!["Location"],
            svec!["Melrose, New York"],
            svec!["East Flatbush, New York"],
            svec!["This is not a Location and it will not be geocoded"],
            svec!["95.213424, 190,1234565"], // invalid lat, long
            svec!["Makati, Metro Manila, Philippines"],
        ],
    );
    let mut cmd = wrk.command("geocode");
    cmd.arg("suggest")
        .arg("Location")
        .arg("--formatstr")
        .arg("{latitude}:{longitude} - {name}, {admin1} {invalid_field}")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["Location"],
        svec!["Invalid dynfmt template."],
        svec!["Invalid dynfmt template."],
        svec!["This is not a Location and it will not be geocoded"],
        svec!["95.213424, 190,1234565"], // invalid lat, long
        svec!["Invalid dynfmt template."],
    ];
    assert_eq!(got, expected);
}

#[test]
fn geocode_suggest_fmt() {
    let wrk = Workdir::new("geocode_suggest_fmt");
    wrk.create(
        "data.csv",
        vec![
            svec!["Location"],
            svec!["Elmhurst, New York"],
            svec!["East Flatbush, New York"],
            svec!["Manhattan, New York"],
            svec!["East Harlem, New York"],
            svec!["This is not a Location and it will not be geocoded"],
            svec!["40.71427, -74.00597"],
            svec!["Makati, Metro Manila, Philippines"],
        ],
    );
    let mut cmd = wrk.command("geocode");
    cmd.arg("suggest")
        .arg("Location")
        .arg("--formatstr")
        .arg("%city-state-country")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["Location"],
        svec!["Elmhurst, New York US"],
        svec!["East Lake, Florida US"],
        svec!["New York, New York US"],
        svec!["East Harlem, New York US"],
        svec!["This is not a Location and it will not be geocoded"],
        svec!["40.71427, -74.00597"], // suggest doesn't work with lat, long
        svec!["Makati City, National Capital Region PH"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn geocode_suggest_fmt_json() {
    let wrk = Workdir::new("geocode_suggest_fmt");
    wrk.create(
        "data.csv",
        vec![
            svec!["Location"],
            svec!["Elmhurst, New York"],
            svec!["East Flatbush, New York"],
            svec!["Manhattan, New York"],
            svec!["East Harlem, New York"],
            svec!["This is not a Location and it will not be geocoded"],
            svec!["40.71427, -74.00597"],
            svec!["Makati, Metro Manila, Philippines"],
        ],
    );
    let mut cmd = wrk.command("geocode");
    cmd.arg("suggest")
        .arg("Location")
        .arg("--formatstr")
        .arg("%json")
        .arg("data.csv");

    let got: String = wrk.stdout(&mut cmd);

    let expected = r######"Location
"{""cityrecord"":{""id"":5116495,""name"":""Elmhurst"",""latitude"":40.73649,""longitude"":-73.87791,""country"":{""id"":6252001,""code"":""US"",""name"":""United States""},""admin_division"":{""id"":5128638,""code"":""US.NY"",""name"":""New York""},""admin2_division"":{""id"":5133268,""code"":""US.NY.081"",""name"":""Queens County""},""timezone"":""America/New_York"",""names"":{""en"":""Elmhurst""},""country_names"":{""en"":""United States""},""admin1_names"":{""en"":""New York""},""admin2_names"":{""en"":""Queens County""},""population"":113364}, ""countryrecord"":{""info"":{""iso"":""US"",""iso3"":""USA"",""iso_numeric"":""840"",""fips"":""US"",""name"":""United States"",""capital"":""Washington"",""area"":""9629091"",""population"":327167434,""continent"":""NA"",""tld"":"".us"",""currency_code"":""USD"",""currency_name"":""Dollar"",""phone"":""1"",""postal_code_format"":""#####-####"",""postal_code_regex"":""^\\d{5}(-\\d{4})?$"",""languages"":""en-US,es-US,haw,fr"",""geonameid"":6252001,""neighbours"":""CA,MX,CU"",""equivalent_fips_code"":""""},""names"":{""en"":""United States""},""capital_names"":{""en"":""Washington D.C.""}} ""us_fips_codes"":{""us_state_code"":""NY"",""us_state_name"":""New York"",""us_state_fips_code"":""36"",""us_county"":""Queens County"",""us_county_fips_code"":""081""}}"
"{""cityrecord"":{""id"":4154008,""name"":""East Lake"",""latitude"":28.11085,""longitude"":-82.69482,""country"":{""id"":6252001,""code"":""US"",""name"":""United States""},""admin_division"":{""id"":4155751,""code"":""US.FL"",""name"":""Florida""},""admin2_division"":{""id"":4168618,""code"":""US.FL.103"",""name"":""Pinellas County""},""timezone"":""America/New_York"",""names"":null,""country_names"":{""en"":""United States""},""admin1_names"":{""en"":""Florida""},""admin2_names"":{""en"":""Pinellas""},""population"":30962}, ""countryrecord"":{""info"":{""iso"":""US"",""iso3"":""USA"",""iso_numeric"":""840"",""fips"":""US"",""name"":""United States"",""capital"":""Washington"",""area"":""9629091"",""population"":327167434,""continent"":""NA"",""tld"":"".us"",""currency_code"":""USD"",""currency_name"":""Dollar"",""phone"":""1"",""postal_code_format"":""#####-####"",""postal_code_regex"":""^\\d{5}(-\\d{4})?$"",""languages"":""en-US,es-US,haw,fr"",""geonameid"":6252001,""neighbours"":""CA,MX,CU"",""equivalent_fips_code"":""""},""names"":{""en"":""United States""},""capital_names"":{""en"":""Washington D.C.""}} ""us_fips_codes"":{""us_state_code"":""FL"",""us_state_name"":""Florida"",""us_state_fips_code"":""12"",""us_county"":""Pinellas"",""us_county_fips_code"":""003""}}"
"{""cityrecord"":{""id"":5128581,""name"":""New York City"",""latitude"":40.71427,""longitude"":-74.00597,""country"":{""id"":6252001,""code"":""US"",""name"":""United States""},""admin_division"":{""id"":5128638,""code"":""US.NY"",""name"":""New York""},""admin2_division"":null,""timezone"":""America/New_York"",""names"":{""en"":""New York""},""country_names"":{""en"":""United States""},""admin1_names"":{""en"":""New York""},""admin2_names"":null,""population"":8804190}, ""countryrecord"":{""info"":{""iso"":""US"",""iso3"":""USA"",""iso_numeric"":""840"",""fips"":""US"",""name"":""United States"",""capital"":""Washington"",""area"":""9629091"",""population"":327167434,""continent"":""NA"",""tld"":"".us"",""currency_code"":""USD"",""currency_name"":""Dollar"",""phone"":""1"",""postal_code_format"":""#####-####"",""postal_code_regex"":""^\\d{5}(-\\d{4})?$"",""languages"":""en-US,es-US,haw,fr"",""geonameid"":6252001,""neighbours"":""CA,MX,CU"",""equivalent_fips_code"":""""},""names"":{""en"":""United States""},""capital_names"":{""en"":""Washington D.C.""}} ""us_fips_codes"":{""us_state_code"":""NY"",""us_state_name"":""New York"",""us_state_fips_code"":""36"",""us_county"":"""",""us_county_fips_code"":""""}}"
"{""cityrecord"":{""id"":6332428,""name"":""East Harlem"",""latitude"":40.79472,""longitude"":-73.9425,""country"":{""id"":6252001,""code"":""US"",""name"":""United States""},""admin_division"":{""id"":5128638,""code"":""US.NY"",""name"":""New York""},""admin2_division"":{""id"":5128594,""code"":""US.NY.061"",""name"":""New York County""},""timezone"":""America/New_York"",""names"":{""en"":""East Harlem""},""country_names"":{""en"":""United States""},""admin1_names"":{""en"":""New York""},""admin2_names"":{""en"":""New York County""},""population"":115921}, ""countryrecord"":{""info"":{""iso"":""US"",""iso3"":""USA"",""iso_numeric"":""840"",""fips"":""US"",""name"":""United States"",""capital"":""Washington"",""area"":""9629091"",""population"":327167434,""continent"":""NA"",""tld"":"".us"",""currency_code"":""USD"",""currency_name"":""Dollar"",""phone"":""1"",""postal_code_format"":""#####-####"",""postal_code_regex"":""^\\d{5}(-\\d{4})?$"",""languages"":""en-US,es-US,haw,fr"",""geonameid"":6252001,""neighbours"":""CA,MX,CU"",""equivalent_fips_code"":""""},""names"":{""en"":""United States""},""capital_names"":{""en"":""Washington D.C.""}} ""us_fips_codes"":{""us_state_code"":""NY"",""us_state_name"":""New York"",""us_state_fips_code"":""36"",""us_county"":""New York County"",""us_county_fips_code"":""061""}}"
This is not a Location and it will not be geocoded
"40.71427, -74.00597"
"{""cityrecord"":{""id"":1703417,""name"":""Makati City"",""latitude"":14.55027,""longitude"":121.03269,""country"":{""id"":1694008,""code"":""PH"",""name"":""Philippines""},""admin_division"":{""id"":7521311,""code"":""PH.NCR"",""name"":""Metro Manila""},""admin2_division"":{""id"":11395838,""code"":""PH.NCR.137600000"",""name"":""Southern Manila District""},""timezone"":""Asia/Manila"",""names"":{""en"":""Makati City""},""country_names"":{""en"":""Philippines""},""admin1_names"":{""en"":""National Capital Region""},""admin2_names"":null,""population"":510383}, ""countryrecord"":{""info"":{""iso"":""PH"",""iso3"":""PHL"",""iso_numeric"":""608"",""fips"":""RP"",""name"":""Philippines"",""capital"":""Manila"",""area"":""300000"",""population"":106651922,""continent"":""AS"",""tld"":"".ph"",""currency_code"":""PHP"",""currency_name"":""Peso"",""phone"":""63"",""postal_code_format"":""####"",""postal_code_regex"":""^(\\d{4})$"",""languages"":""tl,en-PH,fil,ceb,ilo,hil,war,pam,bik,bcl,pag,mrw,tsg,mdh,cbk,krj,sgd,msb,akl,ibg,yka,mta,abx"",""geonameid"":1694008,""neighbours"":"""",""equivalent_fips_code"":""""},""names"":{""en"":""Philippines""},""capital_names"":{""en"":""Manila""}} ""us_fips_codes"":{""us_state_code"":"""",""us_state_name"":""National Capital Region"",""us_state_fips_code"":""null"",""us_county"":"""",""us_county_fips_code"":""""}}""######;

    assert_eq!(got, expected);
}

#[test]
fn geocode_suggest_fmt_cityrecord() {
    let wrk = Workdir::new("geocode_suggest_fmt_cityrecord");
    wrk.create(
        "data.csv",
        vec![
            svec!["Location"],
            svec!["Elmhurst, New York"],
            svec!["East Flatbush, New York"],
            svec!["Manhattan, New York"],
            svec!["East Harlem, New York"],
            svec!["This is not a Location and it will not be geocoded"],
            svec!["40.71427, -74.00597"],
            svec!["Makati, Metro Manila, Philippines"],
        ],
    );
    let mut cmd = wrk.command("geocode");
    cmd.arg("suggest")
        .arg("Location")
        .arg("--formatstr")
        .arg("%cityrecord")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["Location"],
        svec![
            "CitiesRecord { id: 5116495, name: \"Elmhurst\", latitude: 40.73649, longitude: \
             -73.87791, country: Some(Country { id: 6252001, code: \"US\", name: \"United \
             States\" }), admin_division: Some(AdminDivision { id: 5128638, code: \"US.NY\", \
             name: \"New York\" }), admin2_division: Some(AdminDivision { id: 5133268, code: \
             \"US.NY.081\", name: \"Queens County\" }), timezone: \"America/New_York\", names: \
             Some({\"en\": \"Elmhurst\"}), country_names: Some({\"en\": \"United States\"}), \
             admin1_names: Some({\"en\": \"New York\"}), admin2_names: Some({\"en\": \"Queens \
             County\"}), population: 113364 }"
        ],
        svec![
            "CitiesRecord { id: 4154008, name: \"East Lake\", latitude: 28.11085, longitude: \
             -82.69482, country: Some(Country { id: 6252001, code: \"US\", name: \"United \
             States\" }), admin_division: Some(AdminDivision { id: 4155751, code: \"US.FL\", \
             name: \"Florida\" }), admin2_division: Some(AdminDivision { id: 4168618, code: \
             \"US.FL.103\", name: \"Pinellas County\" }), timezone: \"America/New_York\", names: \
             None, country_names: Some({\"en\": \"United States\"}), admin1_names: Some({\"en\": \
             \"Florida\"}), admin2_names: Some({\"en\": \"Pinellas\"}), population: 30962 }"
        ],
        svec![
            "CitiesRecord { id: 5128581, name: \"New York City\", latitude: 40.71427, longitude: \
             -74.00597, country: Some(Country { id: 6252001, code: \"US\", name: \"United \
             States\" }), admin_division: Some(AdminDivision { id: 5128638, code: \"US.NY\", \
             name: \"New York\" }), admin2_division: None, timezone: \"America/New_York\", names: \
             Some({\"en\": \"New York\"}), country_names: Some({\"en\": \"United States\"}), \
             admin1_names: Some({\"en\": \"New York\"}), admin2_names: None, population: 8804190 }"
        ],
        svec![
            "CitiesRecord { id: 6332428, name: \"East Harlem\", latitude: 40.79472, longitude: \
             -73.9425, country: Some(Country { id: 6252001, code: \"US\", name: \"United States\" \
             }), admin_division: Some(AdminDivision { id: 5128638, code: \"US.NY\", name: \"New \
             York\" }), admin2_division: Some(AdminDivision { id: 5128594, code: \"US.NY.061\", \
             name: \"New York County\" }), timezone: \"America/New_York\", names: Some({\"en\": \
             \"East Harlem\"}), country_names: Some({\"en\": \"United States\"}), admin1_names: \
             Some({\"en\": \"New York\"}), admin2_names: Some({\"en\": \"New York County\"}), \
             population: 115921 }"
        ],
        svec!["This is not a Location and it will not be geocoded"],
        svec!["40.71427, -74.00597"],
        svec![
            "CitiesRecord { id: 1703417, name: \"Makati City\", latitude: 14.55027, longitude: \
             121.03269, country: Some(Country { id: 1694008, code: \"PH\", name: \"Philippines\" \
             }), admin_division: Some(AdminDivision { id: 7521311, code: \"PH.NCR\", name: \
             \"Metro Manila\" }), admin2_division: Some(AdminDivision { id: 11395838, code: \
             \"PH.NCR.137600000\", name: \"Southern Manila District\" }), timezone: \
             \"Asia/Manila\", names: Some({\"en\": \"Makati City\"}), country_names: \
             Some({\"en\": \"Philippines\"}), admin1_names: Some({\"en\": \"National Capital \
             Region\"}), admin2_names: None, population: 510383 }"
        ],
    ];
    assert_eq!(got, expected);
}

#[test]
fn geocode_reverse() {
    let wrk = Workdir::new("geocode_reverse");
    wrk.create(
        "data.csv",
        vec![
            svec!["Location"],
            svec!["40.812126, -73.9041813"],
            svec!["40.66472342, -73.93867227"],
            svec!["(40.766672, -73.9568128)"],
            svec!["(  40.819342, -73.9532127    )"],
            svec!["< 40.819342,-73.9532127 >"],
            svec!["This is not a Location and it will not be geocoded"],
            svec![
                "The treasure is at these coordinates 40.66472342, -73.93867227. This should be \
                 geocoded."
            ],
            svec!["95.213424, 190,1234565"], // invalid lat, long
            svec![
                "The coordinates are 40.66472342 latitude, -73.93867227 longitudue. This should \
                 NOT be geocoded."
            ],
        ],
    );
    let mut cmd = wrk.command("geocode");
    cmd.arg("reverse").arg("Location").arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["Location"],
        svec!["Melrose, New York US"],
        svec!["Brooklyn, New York US"],
        svec!["Manhattan, New York US"],
        svec!["East Harlem, New York US"],
        svec!["East Harlem, New York US"],
        svec!["This is not a Location and it will not be geocoded"],
        svec!["Brooklyn, New York US"],
        svec!["95.213424, 190,1234565"],
        svec![
            "The coordinates are 40.66472342 latitude, -73.93867227 longitudue. This should NOT \
             be geocoded."
        ],
    ];
    assert_eq!(got, expected);
}

#[test]
fn geocode_reverse_fmtstring() {
    let wrk = Workdir::new("geocode_reverse_fmtstring");
    wrk.create(
        "data.csv",
        vec![
            svec!["Location"],
            svec!["40.812126, -73.9041813"],
            svec!["40.66472342, -73.93867227"],
            svec!["(40.766672, -73.9568128)"],
            svec!["(40.819342, -73.9532127)"],
            svec!["This is not a Location and it will not be geocoded"],
            svec!["95.213424, 190,1234565"], // invalid lat,long
        ],
    );
    let mut cmd = wrk.command("geocode");
    cmd.arg("reverse")
        .arg("Location")
        .arg("--formatstr")
        .arg("%city-state-country")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["Location"],
        svec!["Melrose, New York US"],
        svec!["Brooklyn, New York US"],
        svec!["Manhattan, New York US"],
        svec!["East Harlem, New York US"],
        svec!["This is not a Location and it will not be geocoded"],
        svec!["95.213424, 190,1234565"], // invalid lat,long
    ];
    assert_eq!(got, expected);
}

#[test]
fn geocode_reverse_fmtstring_intl() {
    let wrk = Workdir::new("geocode_reverse_fmtstring_intl");
    wrk.create(
        "data.csv",
        vec![
            svec!["Location"],
            svec!["41.390205, 2.154007"],
            svec!["52.371807, 4.896029"],
            svec!["(52.520008, 13.404954)"],
            svec!["(14.55027,121.03269)"],
            svec!["This is not a Location and it will not be geocoded"],
            svec!["95.213424, 190,1234565"], // invalid lat,long
        ],
    );
    let mut cmd = wrk.command("geocode");
    cmd.arg("reverse")
        .arg("Location")
        .arg("--formatstr")
        .arg("%city-admin1-country")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["Location"],
        svec!["Barcelona, Catalonia ES"],
        svec!["Amsterdam, North Holland NL"],
        svec!["Berlin,  DE"],
        svec!["Makati City, National Capital Region PH"],
        svec!["This is not a Location and it will not be geocoded"],
        svec!["95.213424, 190,1234565"], // invalid lat,long
    ];
    assert_eq!(got, expected);
}

#[test]
fn geocode_reverse_fmtstring_intl_dynfmt() {
    let wrk = Workdir::new("geocode_reverse_fmtstring_intl_dynfmt");
    wrk.create(
        "data.csv",
        vec![
            svec!["Location"],
            svec!["41.390205, 2.154007"],
            svec!["52.371807, 4.896029"],
            svec!["(52.520008, 13.404954)"],
            svec!["(14.55027,121.03269)"],
            svec!["This is not a Location and it will not be geocoded"],
            svec!["95.213424, 190,1234565"], // invalid lat,long
        ],
    );
    let mut cmd = wrk.command("geocode");
    cmd.arg("reverse")
        .arg("Location")
        .arg("--formatstr")
        .arg("pop: {population} tz: {timezone}")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["Location"],
        svec!["pop: 1620343 tz: Europe/Madrid"],
        svec!["pop: 741636 tz: Europe/Amsterdam"],
        svec!["pop: 3426354 tz: Europe/Berlin"],
        svec!["pop: 510383 tz: Asia/Manila"],
        svec!["This is not a Location and it will not be geocoded"],
        svec!["95.213424, 190,1234565"], // invalid lat,long
    ];
    assert_eq!(got, expected);
}

#[test]
fn geocode_reverse_fmtstring_intl_invalid_dynfmt() {
    let wrk = Workdir::new("geocode_reverse_fmtstring_intl_invalid_dynfmt");
    wrk.create(
        "data.csv",
        vec![
            svec!["Location"],
            svec!["41.390205, 2.154007"],
            svec!["52.371807, 4.896029"],
            svec!["(52.520008, 13.404954)"],
            svec!["(14.55027,121.03269)"],
            svec!["This is not a Location and it will not be geocoded"],
            svec!["95.213424, 190,1234565"], // invalid lat,long
        ],
    );
    let mut cmd = wrk.command("geocode");
    cmd.arg("reverse")
        .arg("Location")
        .arg("--formatstr")
        .arg("pop: {population} tz: {timezone} {doesnotexistfield}")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["Location"],
        svec!["Invalid dynfmt template."],
        svec!["Invalid dynfmt template."],
        svec!["Invalid dynfmt template."],
        svec!["Invalid dynfmt template."],
        svec!["This is not a Location and it will not be geocoded"],
        svec!["95.213424, 190,1234565"], // invalid lat,long
    ];
    assert_eq!(got, expected);
}

#[test]
fn geocode_suggest_dyncols_fmt() {
    let wrk = Workdir::new("geocode_suggest_dyncols_fmt");
    wrk.create(
        "data.csv",
        vec![
            svec!["Location"],
            svec!["Melrose, New York"],
            svec!["East Flatbush, New York"],
            svec!["Manhattan, New York"],
            svec!["Brooklyn, New York"],
            svec!["East Harlem, New York"],
            svec!["This is not a Location and it will not be geocoded"],
            svec!["Jersey City, New Jersey"],
            svec!["95.213424, 190,1234565"], // invalid lat, long
            svec!["Makati, Metro Manila, Philippines"],
        ],
    );
    let mut cmd = wrk.command("geocode");
    cmd.arg("suggest")
        .arg("Location")
        .args([
            "-f",
            "%dyncols: {city_col:name}, {state_col:admin1}, {county_col:admin2}, \
             {country_col:country}, {continent_col:continent}, {currency_col:currency_code}",
        ])
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec![
            "Location",
            "city_col",
            "state_col",
            "county_col",
            "country_col",
            "continent_col",
            "currency_col"
        ],
        svec![
            "Melrose, New York",
            "Melrose Park",
            "Illinois",
            "Cook",
            "US",
            "NA",
            "USD"
        ],
        svec![
            "East Flatbush, New York",
            "East Lake",
            "Florida",
            "Pinellas",
            "US",
            "NA",
            "USD"
        ],
        svec![
            "Manhattan, New York",
            "New York",
            "New York",
            "",
            "US",
            "NA",
            "USD"
        ],
        svec![
            "Brooklyn, New York",
            "Brooklyn Park",
            "Minnesota",
            "Hennepin",
            "US",
            "NA",
            "USD"
        ],
        svec![
            "East Harlem, New York",
            "East Harlem",
            "New York",
            "New York County",
            "US",
            "NA",
            "USD"
        ],
        svec![
            "This is not a Location and it will not be geocoded",
            "",
            "",
            "",
            "",
            "",
            ""
        ],
        svec![
            "Jersey City, New Jersey",
            "Jersey City",
            "New Jersey",
            "Hudson",
            "US",
            "NA",
            "USD"
        ],
        svec!["95.213424, 190,1234565", "", "", "", "", "", ""],
        svec![
            "Makati, Metro Manila, Philippines",
            "Makati City",
            "National Capital Region",
            "",
            "PH",
            "AS",
            "PHP"
        ],
    ];
    assert_eq!(got, expected);
}

#[test]
#[serial]
fn geocode_reverse_dyncols_fmt() {
    let wrk = Workdir::new("geocode_reverse_dyncols_fmt");
    wrk.create(
        "data.csv",
        vec![
            svec!["Location"],
            svec!["40.812126, -73.9041813"],
            svec!["40.66472342, -73.93867227"],
            svec!["(40.766672, -73.9568128)"],
            svec!["(  40.819342, -73.9532127    )"],
            svec!["< 40.819342,-73.9532127 >"],
            svec!["This is not a Location and it will not be geocoded"],
            svec![
                "The treasure is at these coordinates 40.66472342, -73.93867227. This should be \
                 geocoded."
            ],
            svec!["95.213424, 190,1234565"], // invalid lat, long
            svec![
                "The coordinates are 40.66472342 latitude, -73.93867227 longitudue. This should \
                 NOT be geocoded."
            ],
        ],
    );
    let mut cmd = wrk.command("geocode");
    cmd.arg("reverse")
        .arg("Location")
        .args([
            "-f",
            "%dyncols: {city_col:name}, {tz_col:timezone}, {capital_col:capital}, \
             {pop_col:population}",
        ])
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["Location", "city_col", "tz_col", "capital_col", "pop_col"],
        svec![
            "40.812126, -73.9041813",
            "Melrose",
            "America/New_York",
            "Washington",
            "22470"
        ],
        svec![
            "40.66472342, -73.93867227",
            "Brooklyn",
            "America/New_York",
            "Washington",
            "2736074"
        ],
        svec![
            "(40.766672, -73.9568128)",
            "Manhattan",
            "America/New_York",
            "Washington",
            "1487536"
        ],
        svec![
            "(  40.819342, -73.9532127    )",
            "East Harlem",
            "America/New_York",
            "Washington",
            "115921"
        ],
        svec![
            "< 40.819342,-73.9532127 >",
            "East Harlem",
            "America/New_York",
            "Washington",
            "115921"
        ],
        svec![
            "This is not a Location and it will not be geocoded",
            "",
            "",
            "",
            ""
        ],
        svec![
            "The treasure is at these coordinates 40.66472342, -73.93867227. This should be \
             geocoded.",
            "Brooklyn",
            "America/New_York",
            "Washington",
            "2736074"
        ],
        svec!["95.213424, 190,1234565", "", "", "", ""],
        svec![
            "The coordinates are 40.66472342 latitude, -73.93867227 longitudue. This should NOT \
             be geocoded.",
            "",
            "",
            "",
            ""
        ],
    ];
    assert_eq!(got, expected);
}

#[test]
#[serial]
fn geocode_countryinfo() {
    let wrk = Workdir::new("geocode_countryinfo");
    wrk.create(
        "data.csv",
        vec![
            svec!["Country"],
            svec!["US"],
            svec!["CA"],
            svec!["MX"],
            svec!["us"],
            svec!["Cn"],
            svec!["This is not a country and it will not be geocoded"],
            svec!["PH"],
            svec!["95.213424, 190,1234565"],
            svec!["Germany"],
        ],
    );
    let mut cmd = wrk.command("geocode");
    cmd.arg("countryinfo").arg("Country").arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["Country"],
        svec!["United States"],
        svec!["Canada"],
        svec!["Mexico"],
        svec!["United States"],
        svec!["China"],
        svec!["This is not a country and it will not be geocoded"],
        svec!["Philippines"],
        svec!["95.213424, 190,1234565"],
        svec!["Germany"], // passed thru as its not a valid country code
    ];
    assert_eq!(got, expected);
}

#[test]
#[serial]
fn geocode_countryinfo_formatstr() {
    let wrk = Workdir::new("geocode_countryinfo_formatstr");
    wrk.create(
        "data.csv",
        vec![
            svec!["Country"],
            svec!["US"],
            svec!["CA"],
            svec!["MX"],
            svec!["us"],
            svec!["Cn"],
            svec!["This is not a country and it will not be geocoded"],
            svec!["PH"],
            svec!["95.213424, 190,1234565"],
            svec!["Germany"],
        ],
    );
    let mut cmd = wrk.command("geocode");
    cmd.arg("countryinfo")
        .arg("Country")
        .args([
            "--formatstr",
            "{country_name} Pop: {country_population} in {continent} using {currency_name} all in \
             {area} square kms.",
        ])
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["Country"],
        svec!["United States Pop: 327167434 in NA using Dollar all in 9629091 square kms."],
        svec!["Canada Pop: 37058856 in NA using Dollar all in 9984670 square kms."],
        svec!["Mexico Pop: 126190788 in NA using Peso all in 1972550 square kms."],
        svec!["United States Pop: 327167434 in NA using Dollar all in 9629091 square kms."],
        svec!["China Pop: 1411778724 in AS using Yuan Renminbi all in 9596960 square kms."],
        svec!["This is not a country and it will not be geocoded"],
        svec!["Philippines Pop: 106651922 in AS using Peso all in 300000 square kms."],
        svec!["95.213424, 190,1234565"],
        svec!["Germany"],
    ];
    assert_eq!(got, expected);
}

#[test]
#[serial]
fn geocode_countryinfo_formatstr_pretty_json() {
    let wrk = Workdir::new("geocode_countryinfo_formatstr_pretty_json");
    wrk.create(
        "data.csv",
        vec![
            svec!["Country"],
            svec!["US"],
            svec!["CA"],
            svec!["MX"],
            svec!["us"],
            svec!["Cn"],
            svec!["This is not a country and it will not be geocoded"],
            svec!["PH"],
            svec!["95.213424, 190,1234565"],
            svec!["Germany"],
        ],
    );
    let mut cmd = wrk.command("geocode");
    cmd.arg("countryinfo")
        .arg("Country")
        .args(["--formatstr", "%pretty-json"])
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["Country"],
        svec![
            r######"{
  "info": {
    "iso": "US",
    "iso3": "USA",
    "iso_numeric": "840",
    "fips": "US",
    "name": "United States",
    "capital": "Washington",
    "area": "9629091",
    "population": 327167434,
    "continent": "NA",
    "tld": ".us",
    "currency_code": "USD",
    "currency_name": "Dollar",
    "phone": "1",
    "postal_code_format": "#####-####",
    "postal_code_regex": "^\\d{5}(-\\d{4})?$",
    "languages": "en-US,es-US,haw,fr",
    "geonameid": 6252001,
    "neighbours": "CA,MX,CU",
    "equivalent_fips_code": ""
  },
  "names": {
    "en": "United States"
  },
  "capital_names": {
    "en": "Washington D.C."
  }
}"######
        ],
        svec![
            r#"{
  "info": {
    "iso": "CA",
    "iso3": "CAN",
    "iso_numeric": "124",
    "fips": "CA",
    "name": "Canada",
    "capital": "Ottawa",
    "area": "9984670",
    "population": 37058856,
    "continent": "NA",
    "tld": ".ca",
    "currency_code": "CAD",
    "currency_name": "Dollar",
    "phone": "1",
    "postal_code_format": "@#@ #@#",
    "postal_code_regex": "^([ABCEGHJKLMNPRSTVXY]\\d[ABCEGHJKLMNPRSTVWXYZ]) ?(\\d[ABCEGHJKLMNPRSTVWXYZ]\\d)$ ",
    "languages": "en-CA,fr-CA,iu",
    "geonameid": 6251999,
    "neighbours": "US",
    "equivalent_fips_code": ""
  },
  "names": {
    "en": "Canada"
  },
  "capital_names": {
    "en": "Ottawa"
  }
}"#
        ],
        svec![
            r######"{
  "info": {
    "iso": "MX",
    "iso3": "MEX",
    "iso_numeric": "484",
    "fips": "MX",
    "name": "Mexico",
    "capital": "Mexico City",
    "area": "1972550",
    "population": 126190788,
    "continent": "NA",
    "tld": ".mx",
    "currency_code": "MXN",
    "currency_name": "Peso",
    "phone": "52",
    "postal_code_format": "#####",
    "postal_code_regex": "^(\\d{5})$",
    "languages": "es-MX",
    "geonameid": 3996063,
    "neighbours": "GT,US,BZ",
    "equivalent_fips_code": ""
  },
  "names": {
    "en": "Mexico"
  },
  "capital_names": {
    "en": "Mexico City"
  }
}"######
        ],
        svec![
            r######"{
  "info": {
    "iso": "US",
    "iso3": "USA",
    "iso_numeric": "840",
    "fips": "US",
    "name": "United States",
    "capital": "Washington",
    "area": "9629091",
    "population": 327167434,
    "continent": "NA",
    "tld": ".us",
    "currency_code": "USD",
    "currency_name": "Dollar",
    "phone": "1",
    "postal_code_format": "#####-####",
    "postal_code_regex": "^\\d{5}(-\\d{4})?$",
    "languages": "en-US,es-US,haw,fr",
    "geonameid": 6252001,
    "neighbours": "CA,MX,CU",
    "equivalent_fips_code": ""
  },
  "names": {
    "en": "United States"
  },
  "capital_names": {
    "en": "Washington D.C."
  }
}"######
        ],
        svec![
            r#######"{
  "info": {
    "iso": "CN",
    "iso3": "CHN",
    "iso_numeric": "156",
    "fips": "CH",
    "name": "China",
    "capital": "Beijing",
    "area": "9596960",
    "population": 1411778724,
    "continent": "AS",
    "tld": ".cn",
    "currency_code": "CNY",
    "currency_name": "Yuan Renminbi",
    "phone": "86",
    "postal_code_format": "######",
    "postal_code_regex": "^(\\d{6})$",
    "languages": "zh-CN,yue,wuu,dta,ug,za",
    "geonameid": 1814991,
    "neighbours": "LA,BT,TJ,KZ,MN,AF,NP,MM,KG,PK,KP,RU,VN,IN",
    "equivalent_fips_code": ""
  },
  "names": {
    "en": "China"
  },
  "capital_names": {
    "en": "Beijing"
  }
}"#######
        ],
        svec!["This is not a country and it will not be geocoded"],
        svec![
            r#####"{
  "info": {
    "iso": "PH",
    "iso3": "PHL",
    "iso_numeric": "608",
    "fips": "RP",
    "name": "Philippines",
    "capital": "Manila",
    "area": "300000",
    "population": 106651922,
    "continent": "AS",
    "tld": ".ph",
    "currency_code": "PHP",
    "currency_name": "Peso",
    "phone": "63",
    "postal_code_format": "####",
    "postal_code_regex": "^(\\d{4})$",
    "languages": "tl,en-PH,fil,ceb,ilo,hil,war,pam,bik,bcl,pag,mrw,tsg,mdh,cbk,krj,sgd,msb,akl,ibg,yka,mta,abx",
    "geonameid": 1694008,
    "neighbours": "",
    "equivalent_fips_code": ""
  },
  "names": {
    "en": "Philippines"
  },
  "capital_names": {
    "en": "Manila"
  }
}"#####
        ],
        svec!["95.213424, 190,1234565"],
        svec!["Germany"],
    ];
    assert_eq!(got, expected);
}

#[test]
#[serial]
fn geocode_countryinfonow() {
    let wrk = Workdir::new("geocode_countryinfonow");
    let mut cmd = wrk.command("geocode");
    cmd.arg("countryinfonow").arg("US");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["Location"], svec!["United States"]];
    assert_eq!(got, expected);
}

#[test]
#[serial]
fn geocode_countryinfonow_formatstr() {
    let wrk = Workdir::new("geocode_countryinfonow_formatstr");

    let mut cmd = wrk.command("geocode");
    cmd.arg("countryinfonow").arg("cA").args([
        "--formatstr",
        "{country_name} Pop: {country_population} in {continent} using {currency_name} all in \
         {area} square kms.",
    ]);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["Location"],
        svec!["Canada Pop: 37058856 in NA using Dollar all in 9984670 square kms."],
    ];
    assert_eq!(got, expected);
}

#[test]
#[serial]
fn geocode_countryinfonow_formatstr_pretty_json() {
    let wrk = Workdir::new("geocode_countryinfonow_formatstr_pretty_json");
    let mut cmd = wrk.command("geocode");
    cmd.arg("countryinfonow")
        .arg("mx")
        .args(["--formatstr", "%pretty-json"]);

    let got: String = wrk.stdout(&mut cmd);
    let expected = r######"{
  "info": {
    "iso": "MX",
    "iso3": "MEX",
    "iso_numeric": "484",
    "fips": "MX",
    "name": "Mexico",
    "capital": "Mexico City",
    "area": "1972550",
    "population": 126190788,
    "continent": "NA",
    "tld": ".mx",
    "currency_code": "MXN",
    "currency_name": "Peso",
    "phone": "52",
    "postal_code_format": "#####",
    "postal_code_regex": "^(\\d{5})$",
    "languages": "es-MX",
    "geonameid": 3996063,
    "neighbours": "GT,US,BZ",
    "equivalent_fips_code": ""
  },
  "names": {
    "en": "Mexico"
  },
  "capital_names": {
    "en": "Mexico City"
  }
}"######;
    assert_eq!(got, expected);
}
