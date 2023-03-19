use crate::workdir::Workdir;

#[test]
fn applydp_ops_unknown_operation() {
    let wrk = Workdir::new("unknown_op");
    wrk.create(
        "data.csv",
        vec![svec!["letter", "number"], svec!["a", "1"], svec!["b", "2"]],
    );
    let mut cmd = wrk.command("applydp");
    cmd.arg("operations")
        .arg("obfuscate")
        .arg("letter")
        .arg("data.csv");

    let got: String = wrk.output_stderr(&mut cmd);
    assert_eq!(&*got, "Unknown 'obfuscate' operation\n")
}

#[test]
fn applydp_ops_upper() {
    let wrk = Workdir::new("applydp");
    wrk.create(
        "data.csv",
        vec![
            svec!["name", "surname"],
            svec!["John", "Cena"],
            svec!["Mary", "Jane"],
            svec!["Sue", "Bird"],
            svec!["Hopkins", "Jade"],
        ],
    );
    let mut cmd = wrk.command("applydp");
    cmd.arg("operations")
        .arg("upper")
        .arg("name,surname")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["name", "surname"],
        svec!["JOHN", "CENA"],
        svec!["MARY", "JANE"],
        svec!["SUE", "BIRD"],
        svec!["HOPKINS", "JADE"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn applydp_ops_escape() {
    let wrk = Workdir::new("applydp");
    wrk.create(
        "data.csv",
        vec![
            svec!["name", "surname"],
            svec!["John😁", "😡Cena"],
            svec!["Mary☎", "Janë"],
            svec!["Sue", "Bird🐦"],
            svec!["Hopękins", "Jæde"],
        ],
    );
    let mut cmd = wrk.command("applydp");
    cmd.arg("operations")
        .arg("escape")
        .arg("name,surname")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["name", "surname"],
        svec!["John\\u{1f601}", "\\u{1f621}Cena"],
        svec!["Mary\\u{260e}", "Jan\\u{eb}"],
        svec!["Sue", "Bird\\u{1f426}"],
        svec!["Hop\\u{119}kins", "J\\u{e6}de"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn applydp_ops_upper_rename() {
    let wrk = Workdir::new("applydp");
    wrk.create(
        "data.csv",
        vec![
            svec!["name", "surname"],
            svec!["John", "Cena"],
            svec!["Mary", "Jane"],
            svec!["Sue", "Bird"],
            svec!["Hopkins", "Jade"],
        ],
    );
    let mut cmd = wrk.command("applydp");
    cmd.arg("operations")
        .arg("upper")
        .arg("name,surname")
        .arg("--rename")
        .arg("uname,usurname")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["uname", "usurname"],
        svec!["JOHN", "CENA"],
        svec!["MARY", "JANE"],
        svec!["SUE", "BIRD"],
        svec!["HOPKINS", "JADE"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn applydp_ops_upper_rename_invalid() {
    let wrk = Workdir::new("applydp");
    wrk.create(
        "data.csv",
        vec![
            svec!["name", "surname"],
            svec!["John", "Cena"],
            svec!["Mary", "Jane"],
            svec!["Sue", "Bird"],
            svec!["Hopkins", "Jade"],
        ],
    );
    let mut cmd = wrk.command("applydp");
    cmd.arg("operations")
        .arg("upper")
        .arg("name,surname")
        .arg("--rename")
        .arg("uname")
        .arg("data.csv");

    let got: String = wrk.output_stderr(&mut cmd);
    assert_eq!(
        got,
        "Number of new columns does not match input column selection.\n"
    );
}

#[test]
fn applydp_ops_upper_index_params() {
    let wrk = Workdir::new("applydp");
    wrk.create(
        "data.csv",
        vec![
            svec!["name", "surname"],
            svec!["John", "Cena"],
            svec!["Mary", "Jane"],
            svec!["Sue", "Bird"],
            svec!["Hopkins", "Jade"],
        ],
    );
    let mut cmd = wrk.command("applydp");
    cmd.arg("operations")
        .arg("upper")
        .arg("1,2")
        .arg("--rename")
        .arg("uname,usurname")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["uname", "usurname"],
        svec!["JOHN", "CENA"],
        svec!["MARY", "JANE"],
        svec!["SUE", "BIRD"],
        svec!["HOPKINS", "JADE"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn applydp_dynfmt() {
    let wrk = Workdir::new("applydp");
    wrk.create(
        "data.csv",
        vec![
            svec![
                "qty-fruit/day",
                "1fruit",
                "another col",
                "unit cost usd",
                "and another one"
            ],
            svec!["20.5", "mangoes", "a", "5", "z"],
            svec!["10", "bananas", "b", "20", "y"],
            svec!["3", "strawberries", "c", "3.50", "x"],
        ],
    );
    let mut cmd = wrk.command("applydp");
    cmd.arg("dynfmt")
        .arg("--formatstr")
        .arg(
            "{qty_fruit_day} helpings of {1fruit} is good for you, even if it costs \
             ${unit_cost_usd} each. {1fruit}, all {qty_fruit_day} - is just worth it!",
        )
        .arg("--new-column")
        .arg("saying")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec![
            "qty-fruit/day",
            "1fruit",
            "another col",
            "unit cost usd",
            "and another one",
            "saying"
        ],
        svec![
            "20.5",
            "mangoes",
            "a",
            "5",
            "z",
            "20.5 helpings of mangoes is good for you, even if it costs $5 each. mangoes, all \
             20.5 - is just worth it!"
        ],
        svec![
            "10",
            "bananas",
            "b",
            "20",
            "y",
            "10 helpings of bananas is good for you, even if it costs $20 each. bananas, all 10 - \
             is just worth it!"
        ],
        svec![
            "3",
            "strawberries",
            "c",
            "3.50",
            "x",
            "3 helpings of strawberries is good for you, even if it costs $3.50 each. \
             strawberries, all 3 - is just worth it!"
        ],
    ];
    assert_eq!(got, expected);
}

#[test]
fn applydp_ops_empty_shortcircuit() {
    let wrk = Workdir::new("applydp");
    wrk.create(
        "data.csv",
        vec![
            svec!["name"],
            svec!["John"],
            svec![""],
            svec![""],
            svec!["Hopkins"],
        ],
    );
    let mut cmd = wrk.command("applydp");
    cmd.arg("operations").arg("len").arg("name").arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["name"],
        svec!["4"],
        svec!["0"],
        svec!["0"],
        svec!["7"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn applydp_ops_replace() {
    let wrk = Workdir::new("applydp");
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
    let mut cmd = wrk.command("applydp");
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
fn applydp_ops_replace_validation_error() {
    let wrk = Workdir::new("applydp");
    wrk.create(
        "data.csv",
        vec![
            svec!["description"],
            svec!["THE quick brown fox jumped over the lazy dog."],
            svec!["twinkle, twinkle brownie star, how I wonder what you are"],
        ],
    );
    let mut cmd = wrk.command("applydp");
    cmd.arg("operations")
        .arg("replace")
        .arg("description")
        .arg("--replacement")
        .arg("silver")
        .arg("data.csv");

    let got = wrk.output_stderr(&mut cmd);
    assert_eq!(
        got,
        "--comparand (-C) and --replacement (-R) are required for replace operation.\n"
    );
    wrk.assert_err(&mut cmd);
}

#[test]
fn applydp_ops_regex_replace() {
    let wrk = Workdir::new("applydp");
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
    let mut cmd = wrk.command("applydp");
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
fn applydp_ops_regex_replace_validation_error() {
    let wrk = Workdir::new("applydp");
    wrk.create(
        "data.csv",
        vec![
            svec!["description"],
            svec!["My SSN is 078-05-1120. Please do not share it."],
            svec!["twinkle, twinkle brownie star, how I wonder what you are"],
        ],
    );
    let mut cmd = wrk.command("applydp");
    cmd.arg("operations")
        .arg("regex_replace")
        .arg("description")
        .arg("--comparand")
        .arg("(?:\\d{3}-\\d{2}-\\d{4})")
        .arg("data.csv");

    let got = wrk.output_stderr(&mut cmd);
    assert_eq!(
        got,
        "--comparand (-C) and --replacement (-R) are required for regex_replace operation.\n"
    );
    wrk.assert_err(&mut cmd);
}

#[test]
fn applydp_ops_regex_replace_error() {
    let wrk = Workdir::new("applydp");
    wrk.create(
        "data.csv",
        vec![
            svec!["description"],
            svec!["My SSN is 078-05-1120. Please do not share it."],
        ],
    );
    let mut cmd = wrk.command("applydp");
    cmd.arg("operations")
        .arg("regex_replace")
        .arg("description")
        .arg("--comparand")
        .arg("(?:?)") // invalid regular expression
        .arg("--replacement")
        .arg("SSN")
        .arg("data.csv");

    wrk.assert_err(&mut cmd);

    let got: String = wrk.output_stderr(&mut cmd);
    assert!(got.starts_with("regex_replace expression error"));
}

#[test]
fn applydp_ops_mtrim() {
    let wrk = Workdir::new("applydp");
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
    let mut cmd = wrk.command("applydp");
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
fn applydp_ops_round() {
    let wrk = Workdir::new("applydp");
    wrk.create(
        "data.csv",
        vec![
            svec!["number"],
            svec!["123456789"],
            svec!["123456789.12345678"],
            svec!["123456789.0"],
            svec!["123456789.123"],
            svec!["123456789.12398"],
            svec!["0"],
            svec!["5"],
            svec!["not a number, should be ignored"],
        ],
    );
    let mut cmd = wrk.command("applydp");
    cmd.arg("operations")
        .arg("round")
        .arg("number")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["number"],
        svec!["123456789"],
        svec!["123456789.123"],
        svec!["123456789"],
        svec!["123456789.123"],
        svec!["123456789.124"],
        svec!["0"],
        svec!["5"],
        svec!["not a number, should be ignored"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn applydp_ops_round_5places() {
    let wrk = Workdir::new("applydp");
    wrk.create(
        "data.csv",
        vec![
            svec!["number"],
            svec!["123456789"],
            svec!["123456789.12345678"],
            svec!["123456789.0"],
            svec!["123456789.123"],
            svec!["123456789.1239876"],
            svec!["123456789.1239844"],
            svec!["0"],
            svec!["5"],
            svec!["not a number, should be ignored"],
        ],
    );
    let mut cmd = wrk.command("applydp");
    cmd.arg("operations")
        .arg("round")
        .args(["--formatstr", "5"])
        .arg("number")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["number"],
        svec!["123456789"],
        svec!["123456789.12346"],
        svec!["123456789"],
        svec!["123456789.123"],
        svec!["123456789.12399"],
        svec!["123456789.12398"],
        svec!["0"],
        svec!["5"],
        svec!["not a number, should be ignored"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn applydp_ops_chain() {
    let wrk = Workdir::new("applydp");
    wrk.create(
        "data.csv",
        vec![
            svec!["name"],
            svec!["   John       Paul   "],
            svec!["Mary"],
            svec!["  Mary    Sue"],
            svec!["Hopkins"],
        ],
    );
    let mut cmd = wrk.command("applydp");
    cmd.arg("operations")
        .arg("trim,upper,squeeze")
        .arg("name")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["name"],
        svec!["JOHN PAUL"],
        svec!["MARY"],
        svec!["MARY SUE"],
        svec!["HOPKINS"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn applydp_ops_chain_validation_error() {
    let wrk = Workdir::new("applydp");
    wrk.create(
        "data.csv",
        vec![svec!["name"], svec!["   John       Paul   "], svec!["Mary"]],
    );
    let mut cmd = wrk.command("applydp");
    cmd.arg("operations")
        .arg("trim,strip_prefix,upper,squeeze,strip_suffix")
        .arg("name")
        .arg("--comparand")
        .arg("Joe")
        .arg("-c")
        .arg("new_column")
        .arg("data.csv");

    let got = wrk.output_stderr(&mut cmd);
    assert_eq!(
        got,
        "you can only use copy(0), regex_replace(0), replace(0), and strip(2) ONCE per operation \
         series.\n"
    );
    wrk.assert_err(&mut cmd);
}

#[test]
fn applydp_ops_chain_validation_error_missing_comparand() {
    let wrk = Workdir::new("applydp");
    wrk.create(
        "data.csv",
        vec![svec!["name"], svec!["   John       Paul   "], svec!["Mary"]],
    );
    let mut cmd = wrk.command("applydp");
    cmd.arg("operations")
        .arg("trim,upper,squeeze,strip_prefix")
        .arg("name")
        .arg("-c")
        .arg("new_column")
        .arg("data.csv");

    let got = wrk.output_stderr(&mut cmd);
    assert_eq!(got, "--comparand (-C) is required for strip operations.\n");
    wrk.assert_err(&mut cmd);
}

#[test]
fn applydp_ops_chain_squeeze0() {
    let wrk = Workdir::new("applydp");
    wrk.create(
        "data.csv",
        vec![
            svec!["name"],
            svec!["   John       Paul   "],
            svec!["Mary"],
            svec!["  Mary    Sue"],
            svec!["Hopkins"],
        ],
    );
    let mut cmd = wrk.command("applydp");
    cmd.arg("operations")
        .arg("trim,upper,squeeze0")
        .arg("name")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["name"],
        svec!["JOHNPAUL"],
        svec!["MARY"],
        svec!["MARYSUE"],
        svec!["HOPKINS"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn applydp_ops_squeeze0() {
    let wrk = Workdir::new("applydp");
    wrk.create(
        "data.csv",
        vec![
            svec!["name"],
            svec!["   John   \t    Paul   "],
            svec!["    Mary \t   "],
            svec!["  Mary    \n  Sue"],
            svec!["John\r\nHopkins"],
        ],
    );
    let mut cmd = wrk.command("applydp");
    cmd.arg("operations")
        .arg("squeeze0")
        .arg("name")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["name"],
        svec!["JohnPaul"],
        svec!["Mary"],
        svec!["MarySue"],
        svec!["JohnHopkins"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn applydp_ops_chain_strip() {
    let wrk = Workdir::new("applydp");
    wrk.create(
        "data.csv",
        vec![
            svec!["name"],
            svec!["Doctor   John       Paul   "],
            svec!["DocTor Mary"],
            svec!["  Mary    Sue"],
            svec!["doctor Hopkins"],
        ],
    );
    let mut cmd = wrk.command("applydp");
    cmd.arg("operations")
        .arg("squeeze,upper,strip_prefix,trim")
        .arg("name")
        .arg("--comparand")
        .arg("DOCTOR")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["name"],
        svec!["JOHN PAUL"],
        svec!["MARY"],
        svec!["MARY SUE"],
        svec!["HOPKINS"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn applydp_ops_mixed_case_chain() {
    let wrk = Workdir::new("applydp");
    wrk.create(
        "data.csv",
        vec![
            svec!["name"],
            svec!["   John       Paul   "],
            svec!["Mary"],
            svec!["  Mary    Sue"],
            svec!["Hopkins"],
        ],
    );
    let mut cmd = wrk.command("applydp");
    cmd.arg("operations")
        .arg("Trim,UPPER,squEeZe")
        .arg("name")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["name"],
        svec!["JOHN PAUL"],
        svec!["MARY"],
        svec!["MARY SUE"],
        svec!["HOPKINS"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn applydp_no_headers() {
    let wrk = Workdir::new("applydp");
    wrk.create(
        "data.csv",
        vec![
            svec!["John   "],
            svec!["Mary"],
            svec!["  Sue"],
            svec!["Hopkins"],
        ],
    );
    let mut cmd = wrk.command("applydp");
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
fn applydp_rename() {
    let wrk = Workdir::new("applydp");
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
    let mut cmd = wrk.command("applydp");
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
fn applydp_new_column() {
    let wrk = Workdir::new("applydp");
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
    let mut cmd = wrk.command("applydp");
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
fn applydp_emptyreplace() {
    let wrk = Workdir::new("applydp");
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
    let mut cmd = wrk.command("applydp");
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
fn apply_emptyreplace_multiple_columns() {
    let wrk = Workdir::new("apply_emptyreplace_multiple_columns");
    wrk.create(
        "data.csv",
        vec![
            svec!["name", "age", "city"],
            svec!["John", "30", "New York"],
            svec![" ", " ", "      "],
            svec!["Sue", " ", "Boston"],
            svec!["Hopkins", "40", ""],
        ],
    );
    let mut cmd = wrk.command("applydp");
    cmd.arg("emptyreplace")
        .arg("--replacement")
        .arg("NA")
        .arg("name,age,city")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["name", "age", "city"],
        svec!["John", "30", "New York"],
        svec!["NA", "NA", "NA"],
        svec!["Sue", "NA", "Boston"],
        svec!["Hopkins", "40", "NA"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn apply_emptyreplace_all_columns() {
    let wrk = Workdir::new("apply_emptyreplace_all_columns");
    wrk.create(
        "data.csv",
        vec![
            svec!["name", "age", "city"],
            svec!["John", "30", "New York"],
            svec![" ", " ", "      "],
            svec!["Sue", " ", "Boston"],
            svec!["Hopkins", "40", ""],
        ],
    );
    let mut cmd = wrk.command("applydp");
    cmd.arg("emptyreplace")
        .arg("--replacement")
        .arg("NA")
        .arg("1-")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["name", "age", "city"],
        svec!["John", "30", "New York"],
        svec!["NA", "NA", "NA"],
        svec!["Sue", "NA", "Boston"],
        svec!["Hopkins", "40", "NA"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn applydp_datefmt() {
    let wrk = Workdir::new("applydp");
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
    let mut cmd = wrk.command("applydp");
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
fn applydp_datefmt_keep_zero_time() {
    let wrk = Workdir::new("applydp");
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
    let mut cmd = wrk.command("applydp");
    cmd.arg("datefmt")
        .arg("Created Date")
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
fn applydp_datefmt_multiple_cols() {
    let wrk = Workdir::new("applydp");
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
    let mut cmd = wrk.command("applydp");
    cmd.arg("datefmt")
        .arg("Created Date,End Date")
        .arg("data.csv");

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
fn applydp_datefmt_multiple_cols_keep_zero_time() {
    let wrk = Workdir::new("applydp");
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
    let mut cmd = wrk.command("applydp");
    cmd.arg("datefmt")
        .arg("Created Date,End Date")
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
fn applydp_datefmt_multiple_cols_rename() {
    let wrk = Workdir::new("applydp");
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
    let mut cmd = wrk.command("applydp");
    cmd.arg("datefmt")
        .arg("Created Date,End Date")
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
fn applydp_datefmt_prefer_dmy() {
    let wrk = Workdir::new("apply_dmy");
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
    let mut cmd = wrk.command("applydp");
    cmd.arg("datefmt")
        .arg("Created Date")
        .arg("--prefer-dmy")
        .arg("data.csv");

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
fn applydp_datefmt_prefer_dmy_env() {
    let wrk = Workdir::new("apply_prefer_dmy_env");
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
    let mut cmd = wrk.command("applydp");
    cmd.env("QSV_PREFER_DMY", "1");
    cmd.arg("datefmt").arg("Created Date").arg("data.csv");

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
fn applydp_datefmt_fmtstring() {
    let wrk = Workdir::new("applydp");
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
    let mut cmd = wrk.command("applydp");
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
fn applydp_datefmt_fmtstring_with_literals() {
    let wrk = Workdir::new("applydp");
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
    let mut cmd = wrk.command("applydp");
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
fn applydp_datefmt_fmtstring_notime() {
    let wrk = Workdir::new("applydp");
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
    let mut cmd = wrk.command("applydp");
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
