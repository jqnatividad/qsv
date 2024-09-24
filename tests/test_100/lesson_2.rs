// Lesson 2: Piping commands
// https://100.dathere.com/lessons/2

use std::{io::Write, process};

use crate::workdir::Workdir;

// https://100.dathere.com/lessons/2/index.html#selecting-the-columns
#[test]
fn fruits_extended_headers() {
    let wrk = Workdir::new("fruits_extended_headers");
    let test_file = wrk.load_test_file("fruits_extended.csv");
    let mut cmd = process::Command::new(wrk.qsv_bin());
    cmd.args(vec!["headers", test_file.as_str()]);

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"1   fruit
2   price
3   size
4   availability"#;
    assert_eq!(got, expected);
}

// https://100.dathere.com/lessons/2/index.html#selecting-the-columns
#[test]
fn fruits_extended_select_1_4() {
    let wrk = Workdir::new("fruits_extended_select_1_4");
    let test_file = wrk.load_test_file("fruits_extended.csv");
    let mut cmd = process::Command::new(wrk.qsv_bin());
    cmd.args(vec!["select", "1,4", test_file.as_str()]);

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"fruit,availability
apple,available
banana,available
strawberry,available
orange,out of stock
pineapple,available
grape,out of stock
mango,available
watermelon,available
pear,out of stock"#;
    assert_eq!(got, expected);
}

// https://100.dathere.com/lessons/2/index.html#command-redirection
#[test]
#[cfg(not(feature = "datapusher_plus"))]
fn fruits_extended_select_1_4_table() {
    let wrk = Workdir::new("fruits_extended_select_1_4_table");
    let test_file = wrk.load_test_file("fruits_extended.csv");

    let mut select_cmd = process::Command::new(wrk.qsv_bin());
    select_cmd.args(vec!["select", "1,4", test_file.as_str()]);
    let select_stdout: String = wrk.stdout(&mut select_cmd);

    let mut table_child = process::Command::new(wrk.qsv_bin())
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .args(vec!["table"])
        .spawn()
        .unwrap();
    let mut table_stdin = table_child.stdin.take().unwrap();
    std::thread::spawn(move || {
        table_stdin.write_all(select_stdout.as_bytes()).unwrap();
    });
    let output = table_child.wait_with_output().unwrap();
    let got = String::from_utf8_lossy(&output.stdout);

    let expected = r#"fruit       availability
apple       available
banana      available
strawberry  available
orange      out of stock
pineapple   available
grape       out of stock
mango       available
watermelon  available
pear        out of stock
"#;
    assert_eq!(got, expected);
}

// https://100.dathere.com/lessons/2/index.html#exercise-2-piping-commands-example
#[test]
#[cfg(not(feature = "datapusher_plus"))]
fn fruits_extended_select_1_2_table() {
    let wrk = Workdir::new("fruits_extended_select_1_2_table");
    let test_file = wrk.load_test_file("fruits_extended.csv");

    let mut select_cmd = process::Command::new(wrk.qsv_bin());
    select_cmd.args(vec!["select", "1,2", test_file.as_str()]);
    let select_stdout: String = wrk.stdout(&mut select_cmd);

    let mut table_child = process::Command::new(wrk.qsv_bin())
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .args(vec!["table"])
        .spawn()
        .unwrap();
    let mut table_stdin = table_child.stdin.take().unwrap();
    std::thread::spawn(move || {
        table_stdin.write_all(select_stdout.as_bytes()).unwrap();
    });
    let output = table_child.wait_with_output().unwrap();
    let got = String::from_utf8_lossy(&output.stdout);

    let expected = r#"fruit       price
apple       2.50
banana      3.00
strawberry  1.50
orange      2.00
pineapple   3.50
grape       4.00
mango       1.80
watermelon  6.00
pear        2.20
"#;
    assert_eq!(got, expected);
}

// https://100.dathere.com/lessons/2/index.html#exercise-2-piping-commands-example
#[test]
#[cfg(not(feature = "datapusher_plus"))]
fn fruits_extended_select_1_2_transpose_table() {
    let wrk = Workdir::new("fruits_extended_select_1_2_transpose_table");
    let test_file = wrk.load_test_file("fruits_extended.csv");

    let mut select_cmd = process::Command::new(wrk.qsv_bin());
    select_cmd.args(vec!["select", "1,2", test_file.as_str()]);
    let select_stdout: String = wrk.stdout(&mut select_cmd);

    let mut transpose_child = process::Command::new(wrk.qsv_bin())
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .args(vec!["transpose"])
        .spawn()
        .unwrap();
    let mut transpose_stdin = transpose_child.stdin.take().unwrap();
    std::thread::spawn(move || {
        transpose_stdin.write_all(select_stdout.as_bytes()).unwrap();
    });
    let transpose_stdout = transpose_child.wait_with_output().unwrap().stdout;

    let mut table_child = process::Command::new(wrk.qsv_bin())
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .args(vec!["table"])
        .spawn()
        .unwrap();
    let mut table_stdin = table_child.stdin.take().unwrap();
    std::thread::spawn(move || {
        table_stdin.write_all(&transpose_stdout).unwrap();
    });
    let output = table_child.wait_with_output().unwrap();
    let got = String::from_utf8_lossy(&output.stdout);

    let expected = r#"fruit  apple  banana  strawberry  orange  pineapple  grape  mango  watermelon  pear
price  2.50   3.00    1.50        2.00    3.50       4.00   1.80   6.00        2.20
"#;
    assert_eq!(got, expected);
}
