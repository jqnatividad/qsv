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
        svec!["URL", "City"],
        svec!["http://api.zippopotam.us/us/90210", "Beverly Hills"],
        svec!["http://api.zippopotam.us/us/94105", "San Francisco"],
        svec!["http://api.zippopotam.us/us/92802", "Anaheim"],
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
fn excel_sheet_name() {
    let wrk = Workdir::new("excel_sheet_name");

    let xls_file = wrk.load_test_file("excel-xls.xls");

    let mut cmd = wrk.command("excel");
    cmd.arg("--sheet").arg("Middle").arg(xls_file);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["Middle sheet col1", "Middle-2"],
        svec!["z", "3.14"],
        svec!["y", "42"],
        svec!["x", "33"],
        svec!["w", "7"],
        svec!["v", "8"],
    ];
    assert_eq!(got, expected);
}
