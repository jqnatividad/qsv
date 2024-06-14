// Lesson 1: Displaying file content with qsv table
// https://100.dathere.com/lessons/1

use std::process;

use crate::workdir::Workdir;

// https://100.dathere.com/lessons/1/#viewing-raw-file-content-in-the-terminal
#[test]
fn fruits_cat() {
    let wrk = Workdir::new("fruits_cat");
    let test_file = wrk.load_test_file("fruits.csv");
    let mut cmd = process::Command::new("cat");
    cmd.arg(test_file);

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"fruit,price
apple,2.50
banana,3.00
strawberry,1.50"#;
    assert_eq!(got, expected);
}

// https://100.dathere.com/lessons/1/#viewing-raw-file-content-in-the-terminal
#[test]
fn fruits_raw_select() {
    let wrk = Workdir::new("fruits_raw_select");
    let test_file = wrk.load_test_file("fruits.csv");
    let mut cmd = process::Command::new(wrk.qsv_bin());
    cmd.args(vec!["select", "1-", test_file.as_str()]);

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"fruit,price
apple,2.50
banana,3.00
strawberry,1.50"#;
    assert_eq!(got, expected);
}

// https://100.dathere.com/lessons/1/#viewing-raw-file-content-in-the-terminal
#[test]
#[cfg(not(feature = "datapusher_plus"))]
fn fruits_raw_fmt() {
    let wrk = Workdir::new("fruits_raw_fmt");
    let test_file = wrk.load_test_file("fruits.csv");
    let mut cmd = process::Command::new(wrk.qsv_bin());
    cmd.args(vec!["fmt", test_file.as_str()]);

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"fruit,price
apple,2.50
banana,3.00
strawberry,1.50"#;
    assert_eq!(got, expected);
}

// https://100.dathere.com/lessons/1/#viewing-raw-file-content-in-the-terminal
#[test]
fn fruits_raw_slice() {
    let wrk = Workdir::new("fruits_raw_slice");
    let test_file = wrk.load_test_file("fruits.csv");
    let mut cmd = process::Command::new(wrk.qsv_bin());
    cmd.args(vec!["slice", test_file.as_str()]);

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"fruit,price
apple,2.50
banana,3.00
strawberry,1.50"#;
    assert_eq!(got, expected);
}

// https://100.dathere.com/lessons/1/#viewing-raw-file-content-in-the-terminal
#[test]
#[cfg(feature = "polars")]
fn fruits_raw_sqlp() {
    let wrk = Workdir::new("fruits_raw_sqlp");
    let test_file = wrk.load_test_file("fruits.csv");
    let mut cmd = process::Command::new(wrk.qsv_bin());
    cmd.args(vec![
        "sqlp",
        test_file.as_str(),
        "SELECT * FROM fruits",
        "--float-precision",
        "2",
    ]);

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"fruit,price
apple,2.50
banana,3.00
strawberry,1.50"#;
    assert_eq!(got, expected);
}

// https://100.dathere.com/lessons/1/#increasing-readability-with-qsv-table
#[test]
#[cfg(not(feature = "datapusher_plus"))]
fn fruits_table() {
    let wrk = Workdir::new("fruits_table");
    let test_file = wrk.load_test_file("fruits.csv");
    let mut cmd = process::Command::new(wrk.qsv_bin());
    cmd.args(vec!["table", test_file.as_str()]);

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"fruit       price
apple       2.50
banana      3.00
strawberry  1.50"#;
    assert_eq!(got, expected);
}

// https://100.dathere.com/lessons/1/#exercise-1-viewing-file-content-with-tables
#[test]
#[cfg(not(feature = "datapusher_plus"))]
fn fruits_table_align_right() {
    let wrk = Workdir::new("fruits_table_align_right");
    let test_file = wrk.load_test_file("fruits.csv");
    let mut cmd = process::Command::new(wrk.qsv_bin());
    cmd.args(vec!["table", test_file.as_str(), "--align", "right"]);

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"     fruit  price
     apple  2.50
    banana  3.00
strawberry  1.50"#;
    assert_eq!(got, expected);
}

// https://100.dathere.com/lessons/1/#exercise-1-viewing-file-content-with-tables
#[test]
#[cfg(not(feature = "datapusher_plus"))]
fn fruits_extended_table() {
    let wrk = Workdir::new("fruits_extended_table");
    let test_file = wrk.load_test_file("fruits_extended.csv");
    let mut cmd = process::Command::new(wrk.qsv_bin());
    cmd.args(vec!["table", test_file.as_str()]);

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"fruit       price  size    availability
apple       2.50   medium  available
banana      3.00   medium  available
strawberry  1.50   small   available
orange      2.00   medium  out of stock
pineapple   3.50   large   available
grape       4.00   small   out of stock
mango       1.80   medium  available
watermelon  6.00   large   available
pear        2.20   medium  out of stock"#;
    assert_eq!(got, expected);
}

// https://100.dathere.com/lessons/1/#exercise-1-viewing-file-content-with-tables
#[test]
#[cfg(not(feature = "datapusher_plus"))]
fn fruits_extended_table_width() {
    let wrk = Workdir::new("fruits_extended_table_width");
    let test_file = wrk.load_test_file("fruits_extended.csv");
    let mut cmd = process::Command::new(wrk.qsv_bin());
    cmd.args(vec!["table", test_file.as_str(), "--width", "20"]);

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"fruit                 price                 size                  availability
apple                 2.50                  medium                available
banana                3.00                  medium                available
strawberry            1.50                  small                 available
orange                2.00                  medium                out of stock
pineapple             3.50                  large                 available
grape                 4.00                  small                 out of stock
mango                 1.80                  medium                available
watermelon            6.00                  large                 available
pear                  2.20                  medium                out of stock"#;
    assert_eq!(got, expected);
}

// https://100.dathere.com/lessons/1/#exercise-1-viewing-file-content-with-tables
#[test]
#[cfg(not(feature = "datapusher_plus"))]
fn fruits_extended_table_pad() {
    let wrk = Workdir::new("fruits_extended_table_pad");
    let test_file = wrk.load_test_file("fruits_extended.csv");
    let mut cmd = process::Command::new(wrk.qsv_bin());
    cmd.args(vec!["table", test_file.as_str(), "--pad", "20"]);

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"fruit                         price                    size                      availability
apple                         2.50                     medium                    available
banana                        3.00                     medium                    available
strawberry                    1.50                     small                     available
orange                        2.00                     medium                    out of stock
pineapple                     3.50                     large                     available
grape                         4.00                     small                     out of stock
mango                         1.80                     medium                    available
watermelon                    6.00                     large                     available
pear                          2.20                     medium                    out of stock"#;
    assert_eq!(got, expected);
}

// https://100.dathere.com/lessons/1/#exercise-1-viewing-file-content-with-tables
#[test]
#[cfg(not(feature = "datapusher_plus"))]
fn fruits_extended_table_condense() {
    let wrk = Workdir::new("fruits_extended_table_condense");
    let test_file = wrk.load_test_file("fruits_extended.csv");
    let mut cmd = process::Command::new(wrk.qsv_bin());
    cmd.args(vec!["table", test_file.as_str(), "--condense", "5"]);

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"fruit     price  size      avail...
apple     2.50   mediu...  avail...
banan...  3.00   mediu...  avail...
straw...  1.50   small     avail...
orang...  2.00   mediu...  out o...
pinea...  3.50   large     avail...
grape     4.00   small     out o...
mango     1.80   mediu...  avail...
water...  6.00   large     avail...
pear      2.20   mediu...  out o..."#;
    assert_eq!(got, expected);
}
