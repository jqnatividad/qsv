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
        svec!["(40.65371, -73.93042)"],
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
        svec!["(40.65371, -73.93042)"],
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
            svec!["East Harlem, New York"],
            svec!["This is not a Location and it will not be geocoded"],
            svec!["East Flatbush, New York"],
            svec!["95.213424, 190,1234565"], // invalid lat, long
            svec!["Makati, Metro Manila, Philippines"],
        ],
    );
    let mut cmd = wrk.command("geocode");
    cmd.arg("suggest")
        .arg("Location")
        .arg("--formatstr")
        .arg("{latitude}:{longitude} - {name}, {admin1}")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["Location"],
        svec!["41.90059:-87.85673 - Melrose Park, Illinois"],
        svec!["40.65371:-73.93042 - East Flatbush, New York"],
        svec!["40.71427:-74.00597 - New York City, New York"],
        svec!["40.79472:-73.9425 - East Harlem, New York"],
        svec!["40.79472:-73.9425 - East Harlem, New York"],
        svec!["This is not a Location and it will not be geocoded"],
        svec!["40.65371:-73.93042 - East Flatbush, New York"],
        svec!["95.213424, 190,1234565"], // invalid lat, long
        svec!["14.55027:121.03269 - Makati City, National Capital Region"],
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
        svec!["Melrose, New York"],
        svec!["East Flatbush, New York"],
        svec!["Manhattan, New York"],
        svec!["East Harlem, New York"],
        svec!["East Harlem, New York"],
        svec!["This is not a Location and it will not be geocoded"],
        svec!["East Flatbush, New York"],
        svec!["95.213424, 190,1234565"], // invalid lat, long
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
        .arg("%state-country")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["Location"],
        svec!["Melrose, New York United States"],
        svec!["East Flatbush, New York United States"],
        svec!["Manhattan, New York United States"],
        svec!["East Harlem, New York United States"],
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
        svec!["Barcelona, Catalonia Spain"],
        svec!["Amsterdam, North Holland Netherlands"],
        svec!["Berlin,  Germany"],
        svec!["Makati City, National Capital Region Philippines"],
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
