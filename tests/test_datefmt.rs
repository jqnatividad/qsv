use crate::workdir::Workdir;

#[test]
fn datefmt() {
    let wrk = Workdir::new("datefmt");
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
            svec!["1511648546"],
            svec!["-770172300"],
            svec!["1671673426.123456789"],
            // svec!["-770172300"],
        ],
    );
    let mut cmd = wrk.command("datefmt");
    cmd.arg("Created Date").arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["Created Date"],
        svec!["2012-09-17T15:09:00+00:00"],
        svec!["2021-06-02T06:31:39+00:00"],
        svec!["2009-01-20T10:00:00+00:00"],
        svec!["2005-07-04"],
        svec!["2021-05-01T01:17:02.604456+00:00"],
        svec!["This is not a date and it will not be reformatted"],
        svec!["2017-11-25T22:22:26+00:00"],
        svec!["1945-08-05T23:15:00+00:00"],
        svec!["2022-12-22T01:43:46.123456768+00:00"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn datefmt_to_unixtime() {
    let wrk = Workdir::new("datefmt_to_unixtime");
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
            svec!["1511648546"],
            svec!["1620021848429"],
            svec!["1620024872717915000"],
            svec!["1945-08-06T06:54:32.717915+00:00"],
        ],
    );
    let mut cmd = wrk.command("datefmt");
    cmd.arg("Created Date")
        .arg("--formatstr")
        .arg("%s")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["Created Date"],
        svec!["1347894540"],
        svec!["1622615499"],
        svec!["1232445600"],
        svec!["1120435200"],
        svec!["1619831822"],
        svec!["This is not a date and it will not be reformatted"],
        // %s formatstr can only do unixtime in seconds, that's why there's rounding here
        svec!["1511648546"],
        svec!["9223372036"],
        svec!["9223372036"],
        svec!["-770144728"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn datefmt_keep_zero_time() {
    let wrk = Workdir::new("datefmt_keep_zero_time");
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
    let mut cmd = wrk.command("datefmt");
    cmd.arg("Created Date")
        .arg("--keep-zero-time")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["Created Date"],
        svec!["2012-09-17T15:09:00+00:00"],
        svec!["2021-06-02T06:31:39+00:00"],
        svec!["2009-01-20T10:00:00+00:00"],
        svec!["2005-07-04T00:00:00+00:00"],
        svec!["2021-05-01T01:17:02.604456+00:00"],
        svec!["This is not a date and it will not be reformatted"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn datefmt_multiple_cols() {
    let wrk = Workdir::new("datefmt_multiple_cols");
    wrk.create(
        "data.csv",
        vec![
            svec!["Created Date", "End Date"],
            svec![
                "September 17, 2012 10:09am EST",
                "September 18, 2012 10:09am EST"
            ],
            svec![
                "Wed, 02 Jun 2021 06:31:39 GMT",
                "Wed, 02 Jun 2021 08:31:39 GMT"
            ],
            svec!["2009-01-20 05:00 EST", "2009-01-21 05:00 EST"],
            svec!["July 4, 2005", "July 5, 2005"],
            svec!["2021-05-01T01:17:02.604456Z", "2021-05-02T01:17:02.604456Z"],
            svec![
                "This is not a date and it will not be reformatted",
                "This is not a date and it will not be reformatted"
            ],
        ],
    );
    let mut cmd = wrk.command("datefmt");
    cmd.arg("Created Date,End Date").arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["Created Date", "End Date"],
        svec!["2012-09-17T15:09:00+00:00", "2012-09-18T15:09:00+00:00"],
        svec!["2021-06-02T06:31:39+00:00", "2021-06-02T08:31:39+00:00"],
        svec!["2009-01-20T10:00:00+00:00", "2009-01-21T10:00:00+00:00"],
        svec!["2005-07-04", "2005-07-05"],
        svec![
            "2021-05-01T01:17:02.604456+00:00",
            "2021-05-02T01:17:02.604456+00:00"
        ],
        svec![
            "This is not a date and it will not be reformatted",
            "This is not a date and it will not be reformatted"
        ],
    ];
    assert_eq!(got, expected);
}

#[test]
fn datefmt_multiple_cols_keep_zero_time() {
    let wrk = Workdir::new("datefmt_multiple_cols_keep_zero_time");
    wrk.create(
        "data.csv",
        vec![
            svec!["Created Date", "End Date"],
            svec![
                "September 17, 2012 10:09am EST",
                "September 18, 2012 10:09am EST"
            ],
            svec![
                "Wed, 02 Jun 2021 06:31:39 GMT",
                "Wed, 02 Jun 2021 08:31:39 GMT"
            ],
            svec!["2009-01-20 05:00 EST", "2009-01-21 05:00 EST"],
            svec!["July 4, 2005", "July 5, 2005"],
            svec!["2021-05-01T01:17:02.604456Z", "2021-05-02T01:17:02.604456Z"],
            svec![
                "This is not a date and it will not be reformatted",
                "This is not a date and it will not be reformatted"
            ],
        ],
    );
    let mut cmd = wrk.command("datefmt");
    cmd.arg("Created Date,End Date")
        .arg("--keep-zero-time")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["Created Date", "End Date"],
        svec!["2012-09-17T15:09:00+00:00", "2012-09-18T15:09:00+00:00"],
        svec!["2021-06-02T06:31:39+00:00", "2021-06-02T08:31:39+00:00"],
        svec!["2009-01-20T10:00:00+00:00", "2009-01-21T10:00:00+00:00"],
        svec!["2005-07-04T00:00:00+00:00", "2005-07-05T00:00:00+00:00"],
        svec![
            "2021-05-01T01:17:02.604456+00:00",
            "2021-05-02T01:17:02.604456+00:00"
        ],
        svec![
            "This is not a date and it will not be reformatted",
            "This is not a date and it will not be reformatted"
        ],
    ];
    assert_eq!(got, expected);
}

#[test]
fn datefmt_multiple_cols_rename() {
    let wrk = Workdir::new("datefmt_multiple_cols_rename");
    wrk.create(
        "data.csv",
        vec![
            svec!["Created Date", "End Date"],
            svec![
                "September 17, 2012 10:09am EST",
                "September 18, 2012 10:09am EST"
            ],
            svec![
                "Wed, 02 Jun 2021 06:31:39 GMT",
                "Wed, 02 Jun 2021 08:31:39 GMT"
            ],
            svec!["2009-01-20 05:00 EST", "2009-01-21 05:00 EST"],
            svec!["July 4, 2005", "July 5, 2005"],
            svec!["2021-05-01T01:17:02.604456Z", "2021-05-02T01:17:02.604456Z"],
            svec![
                "This is not a date and it will not be reformatted",
                "This is not a date and it will not be reformatted"
            ],
        ],
    );
    let mut cmd = wrk.command("datefmt");
    cmd.arg("Created Date,End Date")
        .arg("--formatstr")
        .arg("%u")
        .arg("--rename")
        .arg("Created Weekday,End Weekday")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["Created Weekday", "End Weekday"],
        svec!["1", "2"],
        svec!["3", "3"],
        svec!["2", "3"],
        svec!["1", "2"],
        svec!["6", "7"],
        svec![
            "This is not a date and it will not be reformatted",
            "This is not a date and it will not be reformatted"
        ],
    ];
    assert_eq!(got, expected);
}

#[test]
fn datefmt_prefer_dmy() {
    let wrk = Workdir::new("datefmt_prefer_dmy");
    wrk.create(
        "data.csv",
        vec![
            svec!["Created Date"],
            svec!["September 17, 2012 10:09am EST"],
            svec!["02/06/2021"],
            svec!["2009-01-20 05:00 EST"],
            svec!["July 4, 2005"],
            svec!["2021-05-01T01:17:02.604456Z"],
            svec!["10/05/71"],
            svec!["12/31/71"],
            svec!["This is not a date and it will not be reformatted"],
        ],
    );
    let mut cmd = wrk.command("datefmt");
    cmd.arg("Created Date").arg("--prefer-dmy").arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["Created Date"],
        svec!["2012-09-17T15:09:00+00:00"],
        svec!["2021-06-02"],
        svec!["2009-01-20T10:00:00+00:00"],
        svec!["2005-07-04"],
        svec!["2021-05-01T01:17:02.604456+00:00"],
        svec!["1971-05-10"],
        svec!["1971-12-31"], /* will still parse obviously valid mdy dates that are not valid as
                              * dmy */
        svec!["This is not a date and it will not be reformatted"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn datefmt_prefer_dmy_env() {
    let wrk = Workdir::new("datefmt_prefer_dmy_env");
    wrk.create(
        "data.csv",
        vec![
            svec!["Created Date"],
            svec!["September 17, 2012 10:09am EST"],
            svec!["02/06/2021"],
            svec!["2009-01-20 05:00 EST"],
            svec!["July 4, 2005"],
            svec!["2021-05-01T01:17:02.604456Z"],
            svec!["10/05/71"],
            svec!["12/31/71"],
            svec!["This is not a date and it will not be reformatted"],
        ],
    );
    let mut cmd = wrk.command("datefmt");
    cmd.env("QSV_PREFER_DMY", "1");
    cmd.arg("Created Date").arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["Created Date"],
        svec!["2012-09-17T15:09:00+00:00"],
        svec!["2021-06-02"],
        svec!["2009-01-20T10:00:00+00:00"],
        svec!["2005-07-04"],
        svec!["2021-05-01T01:17:02.604456+00:00"],
        svec!["1971-05-10"],
        svec!["1971-12-31"], /* will still parse obviously valid mdy dates that are not valid as
                              * dmy */
        svec!["This is not a date and it will not be reformatted"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn datefmt_fmtstring() {
    let wrk = Workdir::new("datefmt_fmtstring");
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
    let mut cmd = wrk.command("datefmt");
    cmd.arg("Created Date")
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
fn datefmt_fmtstring_with_literals() {
    let wrk = Workdir::new("datefmt_fmtstring_with_literals");
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
    let mut cmd = wrk.command("datefmt");
    cmd.arg("Created Date")
        .arg("--formatstr")
        .arg("%c is day %j, week %V of %G")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["Created Date"],
        svec!["Mon Sep 17 15:09:00 2012 is day 261, week 38 of 2012"],
        svec!["Wed Jun  2 06:31:39 2021 is day 153, week 22 of 2021"],
        svec!["Tue Jan 20 10:00:00 2009 is day 020, week 04 of 2009"],
        svec!["Wed Sep 30 18:48:56 2015 is day 273, week 40 of 2015"],
        svec!["This is not a date and it will not be reformatted"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn datefmt_fmtstring_notime() {
    let wrk = Workdir::new("datefmt_fmtstring_notime");
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
    let mut cmd = wrk.command("datefmt");
    cmd.arg("Created Date")
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
