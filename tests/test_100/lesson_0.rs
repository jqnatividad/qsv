// Lesson 0: Exploring qsv help messages and syntax
// https://100.dathere.com/lessons/0

use std::process;

use crate::workdir::Workdir;

fn setup(name: &str, command_str: &str, args: Vec<&str>) -> (Workdir, process::Command) {
    let wrk = Workdir::new(name);
    wrk.create(
        "fruits.csv",
        vec![
            svec!["fruit", "price"],
            svec!["apple", "2.50"],
            svec!["banana", "3.00"],
            svec!["strawberry", "1.50"],
        ],
    );

    let mut cmd = wrk.command(command_str);
    cmd.args(args);

    (wrk, cmd)
}

// https://100.dathere.com/lessons/0/#displaying-headers-of-a-csv
#[test]
fn fruits_headers() {
    let name = "fruits_headers";
    let command_str = "headers";
    let args = vec!["fruits.csv"];
    let (wrk, mut cmd) = setup(name, command_str, args);

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"1   fruit
2   price"#;
    assert_eq!(got, expected);
}

// https://100.dathere.com/lessons/0/#exercise-0-total-rows
#[test]
fn fruits_count_total() {
    let name = "fruits_count";
    let command_str = "count";
    let args = vec!["fruits.csv", "--no-headers"];
    let (wrk, mut cmd) = setup(name, command_str, args);

    let got: String = wrk.stdout(&mut cmd);
    let expected = "4";
    assert_eq!(got, expected);
}
#[test]
fn fruits_count() {
    let name = "fruits_count";
    let command_str = "count";
    let args = vec!["fruits.csv"];
    let (wrk, mut cmd) = setup(name, command_str, args);

    let got: String = wrk.stdout(&mut cmd);
    let expected = "3";
    assert_eq!(got, expected);
}
