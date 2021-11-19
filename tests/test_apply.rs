use crate::workdir::Workdir;

#[test]
fn apply_ops_upper() {
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
    cmd.arg("operations")
        .arg("upper")
        .arg("name")
        .arg("data.csv");

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
fn apply_ops_titlecase() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["description"],
            svec!["THE quick brown fox jumped over the lazy dog."],
            svec!["twinkle, twinkle little star, how I wonder what you are"],
            svec!["a simple title to capitalize: an example"],
            svec!["new york city police department"],
            svec!["department of human services"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("operations")
        .arg("titlecase")
        .arg("description")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["description"],
        svec!["THE Quick Brown Fox Jumped Over the Lazy Dog."],
        svec!["Twinkle, Twinkle Little Star, How I Wonder What You Are"],
        svec!["A Simple Title to Capitalize: An Example"],
        svec!["New York City Police Department"],
        svec!["Department of Human Services"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_ops_censor_check() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["description"],
            svec!["fuck"],
            svec!["FUCK"],
            svec!["fμ¢κ you!"],
            svec!["F_u c_K"],
            svec!["fuuuuuuuck"],
            svec!["fluff truck"],
            svec!["fukushima"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("operations")
        .arg("censor_check")
        .arg("description")
        .arg("--new-column")
        .arg("profanity_flag")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["description", "profanity_flag"],
        svec!["fuck", "true"],
        svec!["FUCK", "true"],
        svec!["fμ¢κ you!", "true"],
        svec!["F_u c_K", "true"],
        svec!["fuuuuuuuck", "true"],
        svec!["fluff truck", "false"],
        svec!["fukushima", "false"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_ops_censor() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["description"],
            svec!["fuck"],
            svec!["FUCK"],
            svec!["fμ¢κ that shit, faggot!"],
            svec!["F_u c_K that blowjoboobies"],
            svec!["fuuuuuuuck yooooouuuu"],
            svec!["kiss my ass!"],
            svec!["shittitties"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("operations")
        .arg("censor")
        .arg("description")
        .arg("--new-column")
        .arg("censored_text")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["description", "censored_text"],
        svec!["fuck", "****"],
        svec!["FUCK", "****"],
        svec!["fμ¢κ that shit, faggot!", "**** that ****, ******!"],
        svec!["F_u c_K that blowjoboobies", "*_* *_* that *************"],
        svec!["fuuuuuuuck yooooouuuu", "********** yooooouuuu"],
        svec!["kiss my ass!", "kiss my ***!"],
        svec!["shittitties", "***********"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_ops_replace() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["description"],
            svec!["THE quick brown fox jumped over the lazy dog."],
            svec!["twinkle, twinkle brownie star, how I wonder what you are"],
            svec!["a simple title to capitalize: an example"],
            svec!["Mr. Brown is not pleased."],
            svec!["this is a brownado car"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("operations")
        .arg("replace")
        .arg("description")
        .arg("--comparand")
        .arg("brown")
        .arg("--replacement")
        .arg("silver")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["description"],
        svec!["THE quick silver fox jumped over the lazy dog."],
        svec!["twinkle, twinkle silverie star, how I wonder what you are"],
        svec!["a simple title to capitalize: an example"],
        svec!["Mr. Brown is not pleased."],
        svec!["this is a silverado car"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_ops_regex_replace() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["description"],
            svec!["My SSN is 078-05-1120. Please do not share it."],
            svec!["twinkle, twinkle brownie star, how I wonder what you are"],
            svec!["Somebody from Nigeria called asked for my ssn - 987-65-4320."],
            svec!["Won't fall for that scam!"],
            svec!["Just enter 987-65-4329 when prompted. Also try 987-65-1234 if it doesn't work."],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("operations")
        .arg("regex_replace")
        .arg("description")
        .arg("--comparand")
        .arg("(?:\\d{3}-\\d{2}-\\d{4})")
        .arg("--replacement")
        .arg("SSN")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["description"],
        svec!["My SSN is SSN. Please do not share it."],
        svec!["twinkle, twinkle brownie star, how I wonder what you are"],
        svec!["Somebody from Nigeria called asked for my ssn - SSN."],
        svec!["Won't fall for that scam!"],
        svec!["Just enter SSN when prompted. Also try SSN if it doesn't work."],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_ops_mtrim() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["description"],
            svec!["(This is in parentheses)"],
            svec!["(This is in parentheses, but with a period)."],
            svec!["(Only left paren"],
            svec!["Only right paren)"],
            svec!["(((multiple parens)))"],
            svec!["Embedded (((multiple parens)))"],
            svec![")))reverse parens((("],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("operations")
        .arg("mtrim")
        .arg("description")
        .arg("--comparand")
        .arg("()")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["description"],
        svec!["This is in parentheses"],
        svec!["This is in parentheses, but with a period)."],
        svec!["Only left paren"],
        svec!["Only right paren"],
        svec!["multiple parens"],
        svec!["Embedded (((multiple parens"],
        svec!["reverse parens"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_ops_chain() {
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
    cmd.arg("operations")
        .arg("trim,upper")
        .arg("name")
        .arg("data.csv");

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
    cmd.arg("operations")
        .arg("trim,upper")
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
    cmd.arg("operations")
        .arg("upper")
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
    cmd.arg("operations")
        .arg("upper")
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
            svec!["$ 12 500.00"],
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
            svec!["₽234,567.89"],
            svec!["₪ 567.89"],
            svec!["₩ 567.89"],
            svec!["₩ 89,123.0"],
            svec!["ƒ 123,456.00"],
            svec!["฿ 789,123"],
            svec!["₫ 456"],
            svec!["123,456.00 $"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("operations")
        .arg("currencytonum")
        .arg("money")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["money"],
        svec!["10.00"],
        svec!["-10.00"],
        svec!["12500.00"],
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
        svec!["234567.89"],
        svec!["567.89"],
        svec!["567.89"],
        svec!["89123.00"],
        svec!["123456.00"],
        svec!["789123.00"],
        svec!["456.00"],
        svec!["123456.00"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_similarity() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["name"],
            svec!["John"],
            svec!["Jonathan"],
            svec!["Edna"],
            svec!["Larry"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("operations")
        .arg("simdln")
        .arg("name")
        .arg("--comparand")
        .arg("Joe")
        .arg("--new-column")
        .arg("name_sim_score")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["name", "name_sim_score"],
        svec!["John", "0.5"],
        svec!["Jonathan", "0.25"],
        svec!["Edna", "0"],
        svec!["Larry", "0"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_similarity_eudex() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["name"],
            svec!["John"],
            svec!["Jonathan"],
            svec!["Michelle"],
            svec!["Larry"],
            svec!["Joel"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("operations")
        .arg("lower,eudex")
        .arg("name")
        .arg("--comparand")
        .arg("michael")
        .arg("--new-column")
        .arg("eudex_flag")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["name", "eudex_flag"],
        svec!["John", "false"],
        svec!["Jonathan", "false"],
        svec!["Michelle", "true"],
        svec!["Larry", "false"],
        svec!["Joel", "false"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_similarity_more_eudex() {
    let wrk = Workdir::new("apply");
    wrk.create(
        "data.csv",
        vec![
            svec!["name"],
            svec!["Jeuses"],
            svec!["Josephina"],
            svec!["Juan"],
            svec!["Juanita"],
            svec!["Michael"],
            svec!["Jingjing"],
        ],
    );
    let mut cmd = wrk.command("apply");
    cmd.arg("operations")
        .arg("lower,eudex")
        .arg("name")
        .arg("--comparand")
        .arg("Jesus")
        .arg("--new-column")
        .arg("eudex_flag")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["name", "eudex_flag"],
        svec!["Jeuses", "true"],
        svec!["Josephina", "false"],
        svec!["Juan", "true"],
        svec!["Juanita", "true"],
        svec!["Michael", "false"],
        svec!["Jingjing", "false"],
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
fn apply_datefmt_fmtstring_with_literals() {
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
            svec!["(40.766672, -73.9568128)"],
            svec!["(  40.819342, -73.9532127    )"],
            svec!["< 40.819342,-73.9532127 >"],
            svec!["This is not a Location and it will not be geocoded"],
            svec!["The treasure is at these coordinates 40.66472342, -73.93867227. This should be geocoded."],
            svec!["95.213424, 190,1234565"], // invalid lat, long
            svec!["The coordinates are 40.66472342 latitude, -73.93867227 longitudue. This should NOT be geocoded."],
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
        svec!["Edgewater, New Jersey"],
        svec!["This is not a Location and it will not be geocoded"],
        svec!["Brooklyn, New York"],
        svec!["95.213424, 190,1234565"], // invalid lat, long
        svec!["The coordinates are 40.66472342 latitude, -73.93867227 longitudue. This should NOT be geocoded."],
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
            svec!["(40.766672, -73.9568128)"],
            svec!["(40.819342, -73.9532127)"],
            svec!["This is not a Location and it will not be geocoded"],
            svec!["95.213424, 190,1234565"], // invalid lat,long
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
        svec!["95.213424, 190,1234565"], // invalid lat,long
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_geocode_fmtstring_intl() {
    let wrk = Workdir::new("apply");
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
    let mut cmd = wrk.command("apply");
    cmd.arg("geocode")
        .arg("Location")
        .arg("--formatstr")
        .arg("city-admin1-country")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["Location"],
        svec!["Barcelona, Catalonia ES"],
        svec!["Amsterdam, North Holland NL"],
        svec!["Mitte, Berlin DE"],
        svec!["Makati City, Metro Manila PH"],
        svec!["This is not a Location and it will not be geocoded"],
        svec!["95.213424, 190,1234565"], // invalid lat,long
    ];
    assert_eq!(got, expected);
}
