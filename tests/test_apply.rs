use crate::workdir::Workdir;

#[test]
fn apply() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["name"],
            svec!["John"],
            svec!["Mary"],
            svec!["Sue"],
            svec!["Hopkins"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("upper").arg("name").arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["name"],
        svec!["JOHN"],
        svec!["MARY"],
        svec!["SUE"],
        svec!["HOPKINS"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_chain() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["name"],
            svec!["John   "],
            svec!["Mary"],
            svec!["  Sue"],
            svec!["Hopkins"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("trim,upper").arg("name").arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["name"],
        svec!["JOHN"],
        svec!["MARY"],
        svec!["SUE"],
        svec!["HOPKINS"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_no_headers() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["John   "],
            svec!["Mary"],
            svec!["  Sue"],
            svec!["Hopkins"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("trim,upper")
        .arg("1")
        .arg("--no-headers")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![svec!["JOHN"], svec!["MARY"], svec!["SUE"], svec!["HOPKINS"]];
    assert_eq!(got, expected);
}

#[test]
fn apply_rename() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["name"],
            svec!["John"],
            svec!["Mary"],
            svec!["Sue"],
            svec!["Hopkins"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("upper")
        .arg("name")
        .arg("--rename")
        .arg("upper_name")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["upper_name"],
        svec!["JOHN"],
        svec!["MARY"],
        svec!["SUE"],
        svec!["HOPKINS"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_new_column() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["name"],
            svec!["John"],
            svec!["Mary"],
            svec!["Sue"],
            svec!["Hopkins"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("upper")
        .arg("name")
        .arg("--new-column")
        .arg("upper_name")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["name", "upper_name"],
        svec!["John", "JOHN"],
        svec!["Mary", "MARY"],
        svec!["Sue", "SUE"],
        svec!["Hopkins", "HOPKINS"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_currencytonum() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["money"],
            svec!["$10.00"],
            svec!["$-10.00"],
            svec!["$5"],
            svec!["0"],
            svec!["5"],
            svec!["$0.25"],
            svec!["$ 10.05"],
            svec!["¥10,000,000.00"],
            svec!["£423.56"],
            svec!["€120.00"],
            svec!["֏99,999.50"],
            svec!["€300 999,55"],
            svec!["This is not money. Leave untouched."],
            svec!["₱1,234,567.89"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("currencytonum").arg("money").arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["money"],
        svec!["10.00"],
        svec!["-10.00"],
        svec!["5.00"],
        svec!["0"],
        svec!["5.00"],
        svec!["0.25"],
        svec!["10.05"],
        svec!["10000000.00"],
        svec!["423.56"],
        svec!["120.00"],
        svec!["99999.50"],
        svec!["300999.55"],
        svec!["This is not money. Leave untouched."],
        svec!["1234567.89"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_emptyreplace() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["name"],
            svec!["John"],
            svec![" "],
            svec!["Sue"],
            svec!["Hopkins"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("emptyreplace").arg("name").arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["name"],
        svec!["John"],
        svec!["None"],
        svec!["Sue"],
        svec!["Hopkins"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_emptyreplace_parameter() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["name"],
            svec!["John"],
            svec![" "],
            svec!["Sue"],
            svec!["Hopkins"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("emptyreplace")
        .arg("--replacement")
        .arg("NA")
        .arg("name")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["name"],
        svec!["John"],
        svec!["NA"],
        svec!["Sue"],
        svec!["Hopkins"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_datefmt() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["Created Date"],
            svec!["September 17, 2012 10:09am EST"],
            svec!["Wed, 02 Jun 2021 06:31:39 GMT"],
            svec!["2009-01-20 05:00 EST"],
            svec!["July 4, 2005"],            
            svec!["2021-05-01T01:17:02.604456Z"],
            svec!["This is not a date and it will not be reformatted"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("datefmt").arg("Created Date").arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["Created Date"],
        svec!["2012-09-17T15:09:00+00:00"],
        svec!["2021-06-02T06:31:39+00:00"],
        svec!["2009-01-20T10:00:00+00:00"],
        svec!["2005-07-04"],            
        svec!["2021-05-01T01:17:02.604456+00:00"],
        svec!["This is not a date and it will not be reformatted"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_datefmt_fmtstring() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["Created Date"],
            svec!["September 17, 2012 10:09am EST"],
            svec!["Wed, 02 Jun 2021 06:31:39 GMT"],
            svec!["2009-01-20 05:00 EST"],
            svec!["2015-09-30 18:48:56.35272715 UTC"],
            svec!["This is not a date and it will not be reformatted"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("datefmt")
        .arg("Created Date")
        .arg("--formatstr")
        .arg("%a %b %e %T %Y %z")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["Created Date"],
        svec!["Mon Sep 17 15:09:00 2012 +0000"],
        svec!["Wed Jun  2 06:31:39 2021 +0000"],
        svec!["Tue Jan 20 10:00:00 2009 +0000"],
        svec!["Wed Sep 30 18:48:56 2015 +0000"],
        svec!["This is not a date and it will not be reformatted"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_datefmt_fmtstring_notime() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["Created Date"],
            svec!["September 17, 2012 10:09am EST"],
            svec!["Wed, 02 Jun 2021 06:31:39 GMT"],
            svec!["2009-01-20 05:00 EST"],
            svec!["4/8/2014 14:13"],
            svec!["This is not a date and it will not be reformatted"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("datefmt")
        .arg("Created Date")
        .arg("--formatstr")
        .arg("%Y-%m-%d")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["Created Date"],
        svec!["2012-09-17"],
        svec!["2021-06-02"],
        svec!["2009-01-20"],
        svec!["2014-04-08"],
        svec!["This is not a date and it will not be reformatted"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_geocode() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["Location"],
            svec!["40.812126, -73.9041813"],
            svec!["40.66472342, -73.93867227"],
            svec!["( 40.766672, -73.9568128 )"],
            svec!["(40.819342, -73.9532127)"],
            svec!["This is not a Location and it will not be geocoded"],
            svec!["95.213424, 190,1234565"], // invalid lat, long
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("geocode").arg("Location").arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["Location"],
        svec!["The Bronx, New York"],
        svec!["Brooklyn, New York"],
        svec!["Manhattan, New York"],
        svec!["Edgewater, New Jersey"],
        svec!["This is not a Location and it will not be geocoded"],
        svec!["95.213424, 190,1234565"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_geocode_fmtstring() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["Location"],
            svec!["40.812126, -73.9041813"],
            svec!["40.66472342, -73.93867227"],
            svec!["( 40.766672, -73.9568128 )"],
            svec!["(40.819342, -73.9532127)"],
            svec!["This is not a Location and it will not be geocoded"],
            svec!["95.213424, 190,1234565"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("geocode")
        .arg("Location")
        .arg("--formatstr")
        .arg("county-country")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["Location"],
        svec!["Bronx, US"],
        svec!["Kings County, US"],
        svec!["New York County, US"],
        svec!["Bergen County, US"],
        svec!["This is not a Location and it will not be geocoded"],
        svec!["95.213424, 190,1234565"],
    ];
    assert_eq!(got, expected);
}
