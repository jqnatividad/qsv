use crate::workdir::Workdir;

#[test]
fn excel_open_xls() {
    let wrk = Workdir::new("excel_open_xls");

    let xls_file = wrk.load_test_file("excel-xls.xls");

    let mut cmd = wrk.command("excel");
    cmd.arg(xls_file);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["URL", "City"],
        svec!["http://api.zippopotam.us/us/90210", "Beverly Hills"],
        svec!["http://api.zippopotam.us/us/94105", "San Francisco"],
        svec!["http://api.zippopotam.us/us/92802", "Anaheim"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn excel_open_xlsx_readpassword() {
    let wrk = Workdir::new("excel_open_xlsx_readpassword");

    let xlsx_file = wrk.load_test_file("password-protected-password123.xlsx");

    let mut cmd = wrk.command("excel");
    cmd.arg(xlsx_file);

    let got = wrk.output_stderr(&mut cmd);
    assert!(got
        .matches("password-protected-password123.xlsx may be a password-protected workbook:")
        .min()
        .is_some());
    wrk.assert_err(&mut cmd);
}

#[test]
fn excel_open_flexible_xls() {
    let wrk = Workdir::new("excel_open_flexible_xls");

    let xls_file = wrk.load_test_file("excel-xls.xls");

    let mut cmd = wrk.command("excel");
    cmd.arg("--sheet")
        .arg("Flexibility Test")
        .arg("--flexible")
        .arg(xls_file);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["URL", "City", ""],
        svec!["http://api.zippopotam.us/us/90210", "Beverly Hills", ""],
        svec!["http://api.zippopotam.us/us/94105", "San Francisco", ""],
        svec!["http://api.zippopotam.us/us/07094", "Secaucus", "NJ"],
        svec!["http://api.zippopotam.us/us/92802", "Anaheim", ""],
        svec!["http://api.zippopotam.us/us/10001", "New York", ""],
    ];
    assert_eq!(got, expected);
}

#[test]
fn excel_trim_xls() {
    let wrk = Workdir::new("excel_trim_xls");

    let xls_file = wrk.load_test_file("excel-xls.xls");

    let mut cmd = wrk.command("excel");
    cmd.arg("--sheet")
        .arg("trim test")
        .arg("--trim")
        .arg(xls_file);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["col1", "col2", "col3"],
        svec!["a", "1", ""],
        svec!["b", "2", "white"],
        svec![
            "c",
            "3a",
            "the quick brown fox jumped over the lazy dog by the zigzag quarry site"
        ],
        svec!["d", "line1 line2 line3", "f"],
        svec!["e", "5c", "surrounded by en and em spaces"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn excel_date_xls() {
    let wrk = Workdir::new("excel_date_xls");

    let xls_file = wrk.load_test_file("excel-xls.xls");

    let mut cmd = wrk.command("excel");
    cmd.arg("--sheet").arg("date test").arg(xls_file);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["date_col", "num_col", "col_Petsa", "just another col"],
        svec!["2001-12-25", "1", "33423", "foo"],
        svec!["2001-09-11 08:30:00", "3", "44202", "bar"],
        svec![
            "This is not a date and will be passed through",
            "5",
            "37145",
            "was"
        ],
        svec!["1970-01-01", "7", "39834", "here"],
        svec!["1989-12-31", "11", "42461", "42"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn excel_date_xlsx() {
    let wrk = Workdir::new("excel_date_xlsx");

    let xlsx_file = wrk.load_test_file("excel-xlsx.xlsx");

    let mut cmd = wrk.command("excel");
    cmd.arg("--sheet").arg("date_test").arg(xlsx_file);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["date", "plaincol"],
        svec![
            "1980-12-25",
            "it will still parse the dates below as date even if plaincol is not in the default \
             --dates-whitelist because the cell format was set to date"
        ],
        svec!["2001-09-11 08:30:00", "2001-09-11"],
        svec!["not a date", "2001-09-11 08:30:00"],
        svec![
            "Wednesday, Mar-14-2012",
            "the date below is not parsed as a date coz we didn't explicitly set the cell format \
             to a date format and \"plaincol\" is not in the --dates-whitelist"
        ],
        svec!["2001-09-11", "9/11/01 8:30 am"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn excel_date_whitelist_xls() {
    let wrk = Workdir::new("excel_date_whitelist_xls");

    let xls_file = wrk.load_test_file("excel-xls.xls");

    let mut cmd = wrk.command("excel");
    cmd.arg("--sheet")
        .arg("date test")
        .args(["--dates-whitelist", "date,petsa"])
        .arg(xls_file);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["date_col", "num_col", "col_Petsa", "just another col"],
        svec!["2001-12-25", "1", "1991-07-04", "foo"],
        svec!["2001-09-11 08:30:00", "3", "2021-01-06", "bar"],
        svec![
            "This is not a date and will be passed through",
            "5",
            "2001-09-11",
            "was"
        ],
        svec!["1970-01-01", "7", "2009-01-21", "here"],
        svec!["1989-12-31", "11", "2016-04-01", "42"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn excel_date_whitelist_none_xls() {
    let wrk = Workdir::new("excel_date_whitelist_none_xls");

    let xls_file = wrk.load_test_file("excel-xls.xls");

    let mut cmd = wrk.command("excel");
    cmd.arg("--sheet")
        .arg("date test")
        .args(["--dates-whitelist", "none"])
        .arg(xls_file);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["date_col", "num_col", "col_Petsa", "just another col"],
        svec!["37250", "1", "33423", "foo"],
        svec!["37145.354166666664", "3", "44202", "bar"],
        svec![
            "This is not a date and will be passed through",
            "5",
            "37145",
            "was"
        ],
        svec!["25569", "7", "39834", "here"],
        svec!["32873", "11", "42461", "42"],
    ];

    assert_eq!(got, expected);
}

#[test]
fn excel_colidx_date_whitelist_xls() {
    let wrk = Workdir::new("excel_colidx_date_whitelist_xls");

    let xls_file = wrk.load_test_file("excel-xls.xls");

    let mut cmd = wrk.command("excel");
    cmd.arg("--sheet")
        .arg("date test")
        .args(["--dates-whitelist", "0,2"])
        .arg(xls_file);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["date_col", "num_col", "col_Petsa", "just another col"],
        svec!["2001-12-25", "1", "1991-07-04", "foo"],
        svec!["2001-09-11 08:30:00", "3", "2021-01-06", "bar"],
        svec![
            "This is not a date and will be passed through",
            "5",
            "2001-09-11",
            "was"
        ],
        svec!["1970-01-01", "7", "2009-01-21", "here"],
        svec!["1989-12-31", "11", "2016-04-01", "42"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn excel_open_ods() {
    let wrk = Workdir::new("excel_open_ods");

    let ods_file = wrk.load_test_file("excel-ods.ods");

    let mut cmd = wrk.command("excel");
    cmd.arg(ods_file);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["URL", "City"],
        svec!["http://api.zippopotam.us/us/90210", "Beverly Hills"],
        svec!["http://api.zippopotam.us/us/94105", "San Francisco"],
        svec!["http://api.zippopotam.us/us/92802", "Anaheim"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn excel_open_xlsx() {
    let wrk = Workdir::new("excel_open_xlsx");

    let xlsx_file = wrk.load_test_file("excel-xlsx.xlsx");

    let mut cmd = wrk.command("excel");
    cmd.arg(xlsx_file);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["URL", "City", "number", "date"],
        svec![
            "http://api.zippopotam.us/us/90210",
            "Beverly Hills",
            "42",
            "2001-09-11 08:30:00"
        ],
        svec![
            "http://api.zippopotam.us/us/94105",
            "San Francisco",
            "3.14",
            "not a date"
        ],
        svec![
            "http://api.zippopotam.us/us/92802",
            "Anaheim",
            "3.14159265358979",
            "2021-01-06"
        ],
        svec![
            "http://api.zippopotam.us/us/10013",
            "Manhattan",
            "1.5",
            "1900-05-02 10:48:00"
        ],
        svec![
            "google.com",
            "Mountain View",
            "20.02",
            "2021-07-04 22:02:59.999"
        ],
        svec!["apple.com", "Cupertino", "37", "Wednesday, March 14, 2012"],
        svec!["amazon.com", "Seattle", "14.23", "2012-03-14"],
        svec!["microsoft.com", "Redmond", "14.201", "2012-03-14 15:30:00"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn excel_xlsx_safe_header_name() {
    let wrk = Workdir::new("excel_xlsx_safe_header_name");

    let xlsx_file = wrk.load_test_file("excel-xlsx.xlsx");

    let mut cmd = wrk.command("excel");
    cmd.arg("--sheet")
        .arg("safe_header_name_test")
        .arg("--safe-header-names")
        .arg(xlsx_file);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec![
            "col1",
            "col_with_leading_and_trailing_spaces_",
            "_23_starts_with_123",
            "With_____special_____Characters_",
            "col1_1",
            "col1_2",
            "The_quick_BROWN_fox_with_a_very_long_column_name_is_now_jump",
            "___date___"
        ],
        svec![
            "1",
            "a",
            "a",
            "1.5",
            "5",
            "e",
            "   This is some text. With whitespaces.  ",
            "2001-09-11"
        ],
        svec![
            "2",
            "b",
            "ba",
            "2.3",
            "4",
            "d",
            "jumped over the lazy dog",
            "1968-07-04"
        ],
        svec![
            "3",
            "c",
            "ka",
            "3.4",
            "3",
            "c",
            "     by the zigzag\r\nquarry site.   ",
            "not a date"
        ],
        svec![
            "4",
            "d",
            "da",
            "3.14",
            "2",
            "b",
            "lorem ipsum dolorem",
            "1902-10-31"
        ],
        svec![
            "5",
            "e",
            "e",
            "0.00012",
            "1",
            "a",
            "Joel was here",
            "1901-10-15 08:09:36"
        ],
    ];
    assert_eq!(got, expected);
}

#[test]
fn excel_last_sheet() {
    let wrk = Workdir::new("excel_last_sheet");

    let xls_file = wrk.load_test_file("excel-xls.xls");

    let mut cmd = wrk.command("excel");
    cmd.arg("--sheet").arg("-1").arg(xls_file);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["Last sheet col1", "Last-2"],
        svec!["a", "5"],
        svec!["b", "4"],
        svec!["c", "3"],
        svec!["d", "2"],
        svec!["e", "1"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn excel_invalid_sheet_index() {
    let wrk = Workdir::new("excel_invalid_sheet_index");

    let xls_file = wrk.load_test_file("excel-xls.xls");

    let mut cmd = wrk.command("excel");
    cmd.arg("--sheet").arg("100").arg(xls_file);

    let got = wrk.output_stderr(&mut cmd);
    let expected = "sheet index 100 is greater than number of sheets 8\n".to_string();
    assert_eq!(got, expected);
    wrk.assert_err(&mut cmd);
}

#[test]
fn excel_invalid_sheet_neg_index() {
    let wrk = Workdir::new("excel_invalid_sheet_neg_index");

    let xls_file = wrk.load_test_file("excel-xls.xls");

    let mut cmd = wrk.command("excel");
    cmd.arg("--sheet").arg("-100").arg(xls_file);

    let got = wrk.output_stderr(&mut cmd);
    let expected = "5 2-column rows exported from \"Last\"\n".to_string();
    assert_eq!(got, expected);
}

#[test]
fn excel_sheet_name() {
    let wrk = Workdir::new("excel_sheet_name");

    let xls_file = wrk.load_test_file("excel-xls.xls");

    let mut cmd = wrk.command("excel");
    cmd.arg("--sheet").arg("Middle").arg(xls_file);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["Middle sheet col1", "Middle-2"],
        svec!["z", "3.14159265358979"],
        svec!["y", "42"],
        svec!["x", "33"],
        svec!["w", "7"],
        svec!["v", "3.14159265358979"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn excel_xls_float_handling_516() {
    let wrk = Workdir::new("excel_float_handling");

    let xls_file = wrk.load_test_file("testexcel-issue-516.xls");

    let mut cmd = wrk.command("excel");
    cmd.arg("--sheet").arg("Middle").arg(xls_file);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["id", "amount", "color"],
        svec!["1", "20.02", "green"],
        svec!["2", "37", "red"],
        svec!["3", "14.23", "blue"],
        svec!["4", "14.2", "pink"],
        svec!["5", "14.201", "grey"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn excel_case_insensitve_sheet_name() {
    let wrk = Workdir::new("excel_case_insensitive_sheet_name");

    let xls_file = wrk.load_test_file("excel-xls.xls");

    let mut cmd = wrk.command("excel");
    cmd.arg("--sheet").arg("miDDlE").arg(xls_file);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["Middle sheet col1", "Middle-2"],
        svec!["z", "3.14159265358979"],
        svec!["y", "42"],
        svec!["x", "33"],
        svec!["w", "7"],
        svec!["v", "3.14159265358979"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn excel_metadata() {
    let wrk = Workdir::new("excel_metadata");

    let xls_file = wrk.load_test_file("excel-xls.xls");

    let mut cmd = wrk.command("excel");
    cmd.arg("--metadata").arg(xls_file);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["index", "sheet_name", "headers", "num_columns", "num_rows"],
        svec!["0", "First", "URL;City", "2", "4"],
        svec!["1", "Flexibility Test", "URL;City;", "3", "6"],
        svec!["2", "Middle", "Middle sheet col1;Middle-2", "2", "6"],
        svec!["3", "Sheet1", "", "0", "0"],
        svec!["4", "trim test", "col1;   col2;col3", "3", "6"],
        svec![
            "5",
            "date test",
            "date_col;num_col;col_Petsa;just another col",
            "4",
            "6"
        ],
        svec!["6", "NoData", "col1;col2;col3;col4", "4", "1"],
        svec!["7", "Last", "Last sheet col1;Last-2", "2", "6"],
    ];
    assert_eq!(got, expected);
    wrk.assert_success(&mut cmd);
}

#[test]
fn excel_message() {
    let wrk = Workdir::new("excel_message");

    let xls_file = wrk.load_test_file("excel-xls.xls");

    let mut cmd = wrk.command("excel");
    cmd.arg("--sheet").arg("Middle").arg(xls_file);

    let got = wrk.output_stderr(&mut cmd);
    assert_eq!(got, "5 2-column rows exported from \"Middle\"\n");
}

#[test]
fn excel_empty_sheet_message() {
    let wrk = Workdir::new("excel_empty_sheet_message");

    let xls_file = wrk.load_test_file("excel-xls.xls");

    let mut cmd = wrk.command("excel");
    cmd.arg("--sheet").arg("nodata").arg(xls_file);

    let got = wrk.output_stderr(&mut cmd);
    assert_eq!(got, "0 4-column rows exported from \"NoData\"\n");
}

#[test]
fn excel_empty_sheet2_message() {
    let wrk = Workdir::new("excel_empty_sheet2_message");

    let xls_file = wrk.load_test_file("excel-xls.xls");

    let mut cmd = wrk.command("excel");
    cmd.arg("--sheet").arg("Sheet1").arg(xls_file);

    let got = wrk.output_stderr(&mut cmd);
    assert_eq!(got, "0 0-column rows exported from \"Sheet1\"\n");
}
