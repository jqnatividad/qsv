use crate::workdir::Workdir;

#[test]
fn test_input_comment() {
    let wrk = Workdir::new("input_comment");

    let test_file = wrk.load_test_file("inputcommenttest.csv");

    let mut cmd = wrk.command("input");
    cmd.arg("--comment").arg("#").arg(test_file);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["column1", "column2", "column3"],
        svec!["a", "1", "alpha"],
        svec!["b", "2", "beta"],
        svec!["c", "3", "gamma"],
        svec!["d", "4", "delta"],
        svec!["e", "5", "epsilon"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn test_input_skiplines() {
    let wrk = Workdir::new("input_skiplines");
    wrk.create(
        "preamble.csv",
        vec![
            svec!["# test file to see how skiplines work", ""],
            svec!["! this is another comment before the header", ""],
            svec!["# DATA DICTIONARY", ""],
            svec!["! column1 - alphabetic; id of the column", ""],
            svec!["% column2 - numeric; just a number", ""],
            svec!["column1", "column2"],
            svec!["a", "1"],
            svec!["c", "3"],
            svec!["e", "5"],
        ],
    );
    let mut cmd = wrk.command("input");
    cmd.arg("--skip-lines").arg("5").arg("preamble.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["column1", "column2"],
        svec!["a", "1"],
        svec!["c", "3"],
        svec!["e", "5"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn test_input_autoskip() {
    let wrk = Workdir::new("input_autoskip");
    let test_file = wrk.load_test_file("snifftest.csv");

    let mut cmd = wrk.command("input");
    cmd.arg("--auto-skip").arg(test_file);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["h1", "h2", "h3", "h4"],
        svec!["abcdefg", "1", "a", "3.14"],
        svec!["a", "2", "z", "1.2020569"],
        svec!["c", "42", "x", "1.0"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn test_input_quotestyle_nonnumeric() {
    let wrk = Workdir::new("input_quotestyle_nonnumeric");
    wrk.create(
        "testdata.csv",
        vec![
            svec!["column1", "float column", "int column", "description"],
            svec!["a", "1.0", "1", "this is a string"],
            svec!["c", "3.5", "3", "this is another string"],
            svec!["e", "3.14", "42", "this is a third string"],
        ],
    );
    let mut cmd = wrk.command("input");
    cmd.args(["--quote-style", "nonnumeric"])
        .arg("testdata.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#""column1","float column","int column","description"
"a",1.0,1,"this is a string"
"c",3.5,3,"this is another string"
"e",3.14,42,"this is a third string""#;
    assert_eq!(got, expected);
}

#[test]
fn test_input_quotestyle_necessary() {
    let wrk = Workdir::new("input_quotestyle_necessary");
    wrk.create(
        "testdata.csv",
        vec![
            svec!["column1", "float column", "int column", "description"],
            svec!["a", "1.0", "1", "1,234,5678 - number with commas"],
            svec!["c", "3.5", "3", "this is another string"],
            svec!["e", "3.14", "42", "this is a third string"],
        ],
    );
    let mut cmd = wrk.command("input");
    cmd.args(["--quote-style", "necessary"]).arg("testdata.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"column1,float column,int column,description
a,1.0,1,"1,234,5678 - number with commas"
c,3.5,3,this is another string
e,3.14,42,this is a third string"#;
    assert_eq!(got, expected);
}

#[test]
fn test_input_quotestyle_all() {
    let wrk = Workdir::new("input_quotestyle_all");
    wrk.create(
        "testdata.csv",
        vec![
            svec!["column1", "float column", "int column", "description"],
            svec!["a", "1.0", "1", "1,234,5678 - number with commas"],
            svec!["c", "3.5", "3", "this is another string"],
            svec!["e", "3.14", "42", "this is a third string"],
        ],
    );
    let mut cmd = wrk.command("input");
    cmd.args(["--quote-style", "all"]).arg("testdata.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#""column1","float column","int column","description"
"a","1.0","1","1,234,5678 - number with commas"
"c","3.5","3","this is another string"
"e","3.14","42","this is a third string""#;
    assert_eq!(got, expected);
}

#[test]
fn test_input_skip_one_line() {
    let wrk = Workdir::new("input_skip_one_line");
    wrk.create(
        "preamble.csv",
        vec![
            svec!["# test file to see how skiplines work", ""],
            svec!["column1", "column2"],
            svec!["a", "1"],
            svec!["c", "3"],
            svec!["e", "5"],
        ],
    );
    let mut cmd = wrk.command("input");
    cmd.arg("--skip-lines").arg("1").arg("preamble.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["column1", "column2"],
        svec!["a", "1"],
        svec!["c", "3"],
        svec!["e", "5"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn test_input_skip_no_line() {
    let wrk = Workdir::new("input_skip_no_line");
    wrk.create(
        "preamble.csv",
        vec![
            svec!["column1", "column2"],
            svec!["a", "1"],
            svec!["c", "3"],
            svec!["e", "5"],
        ],
    );
    let mut cmd = wrk.command("input");
    cmd.arg("--skip-lines").arg("0").arg("preamble.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["column1", "column2"],
        svec!["a", "1"],
        svec!["c", "3"],
        svec!["e", "5"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn test_input_trim_headers() {
    let wrk = Workdir::new("input_trim_headers");
    wrk.create(
        "data.csv",
        vec![
            svec!["   column1   ", "  column2   "],
            svec!["  a", "1"],
            svec!["c  ", "3"],
            svec!["e", "5   "],
        ],
    );
    let mut cmd = wrk.command("input");
    cmd.arg("--trim-headers").arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["column1", "column2"],
        svec!["  a", "1"],
        svec!["c  ", "3"],
        svec!["e", "5   "],
    ];
    assert_eq!(got, expected);
}

#[test]
fn test_input_trim_fields() {
    let wrk = Workdir::new("input_trim_fields");
    wrk.create(
        "data.csv",
        vec![
            svec!["column1   ", "column2   "],
            svec!["   a", "  1"],
            svec!["c   ", "3  "],
            svec!["   e   ", "  5"],
        ],
    );
    let mut cmd = wrk.command("input");
    cmd.arg("--trim-fields").arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["column1   ", "column2   "],
        svec!["a", "1"],
        svec!["c", "3"],
        svec!["e", "5"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn test_input_trim_headers_fields() {
    let wrk = Workdir::new("input_trim_headers_fields");
    wrk.create(
        "data.csv",
        vec![
            svec!["   column1   ", "   column2   "],
            svec!["   a", "  1"],
            svec!["c   ", "3  "],
            svec!["   e   ", "  5"],
        ],
    );
    let mut cmd = wrk.command("input");
    cmd.arg("--trim-headers")
        .arg("--trim-fields")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["column1", "column2"],
        svec!["a", "1"],
        svec!["c", "3"],
        svec!["e", "5"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn test_input_skip_lastlines() {
    let wrk = Workdir::new("input_skip_lastlines");
    wrk.create(
        "data.csv",
        vec![
            svec!["column1", "column2"],
            svec!["a", "1"],
            svec!["b", "2"],
            svec!["c", "3"],
            svec!["d", "4"],
            svec!["e", "5"],
        ],
    );
    let mut cmd = wrk.command("input");
    cmd.arg("--skip-lastlines").arg("2").arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["column1", "column2"],
        svec!["a", "1"],
        svec!["b", "2"],
        svec!["c", "3"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn test_input_skip_lines_both() {
    let wrk = Workdir::new("input_skip_lines_both");
    wrk.create(
        "data.csv",
        vec![
            svec!["#column1", "column2"],
            svec!["! column1", "column2"],
            svec!["column1", "column2"],
            svec!["a", "1"],
            svec!["b", "2"],
            svec!["c", "3"],
            svec!["d", "4"],
            svec!["e", "5"],
        ],
    );
    let mut cmd = wrk.command("input");
    cmd.arg("--skip-lastlines")
        .arg("2")
        .arg("--skip-lines")
        .arg("2")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["column1", "column2"],
        svec!["a", "1"],
        svec!["b", "2"],
        svec!["c", "3"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn test_input_both_skip_flexible() {
    let wrk = Workdir::new("test_input_both_skip_flexible");

    let test_file = wrk.load_test_file("inputskiptest.csv");

    let mut cmd = wrk.command("input");
    cmd.args(["--skip-lastlines", "4"])
        .args(["--skip-lines", "5"])
        .arg(test_file);

    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["column1", "column2"],
        svec!["a", "1"],
        svec!["b", "2"],
        svec!["c", "3"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn input_noheadertrim() {
    let wrk = Workdir::new("input_noheadertrim");

    // headers taken from malformed CSV example - cities.csv at
    // https://people.sc.fsu.edu/~jburkardt/data/csv/csv.html
    wrk.create(
        "data.csv",
        vec![
            svec![
                "\"LatD\"",
                "\"LatM\"",
                "\"LatS\"",
                "\"NS\"",
                "\"LonD\"",
                "\"LonM\"",
                "\"LonS\"",
                "\"EW\"",
                "\"City\"",
                "\"State\""
            ],
            svec![
                "41",
                "5",
                "59",
                "N",
                "80",
                "39",
                "0",
                "W",
                "Youngstown",
                "OH"
            ],
        ],
    );

    let mut cmd = wrk.command("input");
    cmd.arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec![
            "\"LatD\"",
            "\"LatM\"",
            "\"LatS\"",
            "\"NS\"",
            "\"LonD\"",
            "\"LonM\"",
            "\"LonS\"",
            "\"EW\"",
            "\"City\"",
            "\"State\""
        ],
        svec![
            "41",
            "5",
            "59",
            "N",
            "80",
            "39",
            "0",
            "W",
            "Youngstown",
            "OH"
        ],
    ];
    assert_eq!(got, expected);
}

#[test]
fn input_headertrim() {
    let wrk = Workdir::new("input_headertrim");

    // headers taken from malformed CSV example - cities.csv at
    // https://people.sc.fsu.edu/~jburkardt/data/csv/csv.html
    wrk.create(
        "data.csv",
        vec![
            svec![
                "\"LatD\"",
                "\"LatM\"",
                "\"LatS\"",
                "\"NS\"",
                "\"LonD\"",
                "\"LonM\"",
                "\"LonS\"",
                "\"EW\"",
                "\"City\"",
                "\"State\""
            ],
            svec![
                "41",
                "5",
                "59",
                "N",
                "80",
                "39",
                "0",
                "W",
                "Youngstown",
                "OH"
            ],
        ],
    );

    let mut cmd = wrk.command("input");
    cmd.arg("--trim-headers").arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["LatD", "LatM", "LatS", "NS", "LonD", "LonM", "LonS", "EW", "City", "State"],
        svec![
            "41",
            "5",
            "59",
            "N",
            "80",
            "39",
            "0",
            "W",
            "Youngstown",
            "OH"
        ],
    ];
    assert_eq!(got, expected);
}
