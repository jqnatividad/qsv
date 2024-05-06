use std::{borrow::ToOwned, cmp, process};

use newline_converter::dos2unix;

use crate::workdir::Workdir;

macro_rules! stats_tests {
    ($name:ident, $field:expr, $rows:expr, $expect:expr) => {
        stats_tests!($name, $field, $rows, $expect, false, true);
    };
    ($name:ident, $field:expr, $rows:expr, $expect:expr, $nulls:expr, $infer_dates:expr) => {
        mod $name {
            use super::test_stats;

            stats_test_headers!($name, $field, $rows, $expect, $nulls, $infer_dates);
            stats_test_no_headers!($name, $field, $rows, $expect, $nulls, $infer_dates);
        }
    };
}

macro_rules! stats_no_infer_dates_tests {
    ($name:ident, $field:expr, $rows:expr, $expect:expr) => {
        stats_tests!($name, $field, $rows, $expect, false, false);
    };
    ($name:ident, $field:expr, $rows:expr, $expect:expr, $nulls:expr, $infer_dates:expr) => {
        mod $name {
            use super::test_stats;

            stats_test_headers!($name, $field, $rows, $expect, $nulls, $infer_dates);
            stats_test_no_headers!($name, $field, $rows, $expect, $nulls, $infer_dates);
        }
    };
}

macro_rules! stats_test_headers {
    ($name:ident, $field:expr, $rows:expr, $expect:expr) => {
        stats_test_headers!($name, $field, $rows, $expect, false, true);
    };
    ($name:ident, $field:expr, $rows:expr, $expect:expr, $nulls:expr, $infer_dates:expr) => {
        #[test]
        fn headers_no_index() {
            let name = concat!(stringify!($name), "_headers_no_index");
            test_stats(
                name,
                $field,
                $rows,
                $expect,
                true,
                false,
                $nulls,
                $infer_dates,
            );
        }

        #[test]
        fn headers_index() {
            let name = concat!(stringify!($name), "_headers_index");
            test_stats(
                name,
                $field,
                $rows,
                $expect,
                true,
                true,
                $nulls,
                $infer_dates,
            );
        }
    };
}

macro_rules! stats_test_no_headers {
    ($name:ident, $field:expr, $rows:expr, $expect:expr) => {
        stats_test_no_headers!($name, $field, $rows, $expect, false, true);
    };
    ($name:ident, $field:expr, $rows:expr, $expect:expr, $nulls:expr, $infer_dates:expr) => {
        #[test]
        fn no_headers_no_index() {
            let name = concat!(stringify!($name), "_no_headers_no_index");
            test_stats(
                name,
                $field,
                $rows,
                $expect,
                false,
                false,
                $nulls,
                $infer_dates,
            );
        }

        #[test]
        fn no_headers_index() {
            let name = concat!(stringify!($name), "_no_headers_index");
            test_stats(
                name,
                $field,
                $rows,
                $expect,
                false,
                true,
                $nulls,
                $infer_dates,
            );
        }
    };
}

#[allow(clippy::too_many_arguments)]
fn test_stats<S>(
    name: S,
    field: &str,
    rows: &[&str],
    expected: &str,
    headers: bool,
    use_index: bool,
    nulls: bool,
    infer_dates: bool,
) where
    S: ::std::ops::Deref<Target = str>,
{
    let (wrk, mut cmd) = setup(name, rows, headers, use_index, nulls, infer_dates);
    let field_val = get_field_value(&wrk, &mut cmd, field);
    // Only compare the first few bytes since floating point arithmetic
    // can mess with exact comparisons.
    // when field = skewness, we're comparing a long sequence of the quartile columns,
    // that's why we use 40, if not, its a single column, and we need to compare only
    // the first 15 characters just in case its a float
    let len = cmp::min(
        if field == "skewness" { 40 } else { 15 },
        cmp::min(field_val.len(), expected.len()),
    );
    assert_eq!(&field_val[0..len], &expected[0..len]);
}

fn setup<S>(
    name: S,
    rows: &[&str],
    headers: bool,
    use_index: bool,
    nulls: bool,
    infer_dates: bool,
) -> (Workdir, process::Command)
where
    S: ::std::ops::Deref<Target = str>,
{
    let wrk = Workdir::new(&name);
    let mut data: Vec<Vec<String>> = rows.iter().map(|&s| vec![s.to_owned()]).collect();
    if headers {
        data.insert(0, svec!["header"]);
    }
    if use_index {
        wrk.create_indexed("in.csv", data);
    } else {
        wrk.create("in.csv", data);
    }

    let mut cmd = wrk.command("stats");
    cmd.arg("in.csv");
    if !headers {
        cmd.arg("--no-headers");
    }
    if nulls {
        cmd.arg("--nulls");
    }
    if infer_dates {
        cmd.arg("--infer-dates").arg("--dates-whitelist").arg("all");
    }

    (wrk, cmd)
}

fn get_field_value(wrk: &Workdir, cmd: &mut process::Command, field: &str) -> String {
    if field == "median" {
        cmd.arg("--median");
    }
    if field == "quartiles" {
        cmd.arg("--quartiles");
    }
    if field == "cardinality" {
        cmd.arg("--cardinality");
    }
    if field == "mode" || field == "antimode" {
        cmd.arg("--mode");
    }
    if field == "infer_dates" {
        cmd.arg("--infer-dates");
    }

    let mut rows: Vec<Vec<String>> = wrk.read_stdout(cmd);
    let headers = rows.remove(0);
    let mut sequence: Vec<&str> = vec![];
    for row in &rows {
        for (h, val) in headers.iter().zip(row.iter()) {
            match field {
                "quartiles" => match &**h {
                    "lower_outer_fence" | "lower_inner_fence" | "q1" | "q2_median" | "q3"
                    | "iqr" | "upper_inner_fence" | "upper_outer_fence" => {
                        sequence.push(val);
                    },
                    "skewness" => {
                        sequence.push(val);
                        return sequence.join(",");
                    },
                    _ => {},
                },
                _ => {
                    if &**h == field {
                        return val.clone();
                    }
                },
            }
        }
    }
    panic!("BUG: Could not find field '{field}' in headers '{headers:?}' for command '{cmd:?}'.");
}

stats_tests!(stats_infer_string, "type", &["a"], "String");
stats_tests!(stats_infer_int, "type", &["1"], "Integer");
stats_tests!(stats_infer_float, "type", &["1.2"], "Float");
stats_tests!(stats_infer_null, "type", &[""], "NULL");
stats_tests!(stats_infer_date, "type", &["1968-06-27"], "Date");
stats_no_infer_dates_tests!(stats_infer_nodate, "type", &["1968-06-27"], "String");
stats_tests!(
    stats_infer_datetime,
    "type",
    &["1968-06-27 12:30:01"],
    "DateTime"
);
stats_no_infer_dates_tests!(
    stats_infer_nodatetime,
    "type",
    &["1968-06-27 12:30:01"],
    "String"
);
stats_tests!(stats_infer_string_null, "type", &["a", ""], "String");
stats_tests!(stats_infer_int_null, "type", &["1", ""], "Integer");
stats_tests!(stats_infer_float_null, "type", &["1.2", ""], "Float");
stats_tests!(
    stats_infer_date_null,
    "type",
    &["June 27, 1968", ""],
    "Date"
);
stats_no_infer_dates_tests!(
    stats_infer_no_infer_dates_null,
    "type",
    &["June 27, 1968", ""],
    "String"
);
stats_tests!(
    stats_infer_datetime_null,
    "type",
    &["June 27, 1968 12:30:00 UTC", ""],
    "DateTime"
);
stats_no_infer_dates_tests!(
    stats_infer_nodatetime_null,
    "type",
    &["June 27, 1968 12:30:00 UTC", ""],
    "String"
);
stats_tests!(stats_infer_null_string, "type", &["", "a"], "String");
stats_tests!(stats_infer_null_int, "type", &["", "1"], "Integer");
stats_tests!(stats_infer_null_float, "type", &["", "1.2"], "Float");
stats_tests!(
    stats_infer_null_date,
    "type",
    &["", "September 17, 2012 at 10:09am PST"],
    "Date"
);
stats_no_infer_dates_tests!(
    stats_infer_null_nodate,
    "type",
    &["", "September 17, 2012 at 10:09am PST"],
    "String"
);
stats_tests!(
    stats_infer_date_datetime,
    "type",
    &["September 11, 2001", "September 17, 2012 at 10:09am PST"],
    "DateTime"
);
stats_no_infer_dates_tests!(
    stats_infer_nodate_nodatetime,
    "type",
    &["September 11, 2001", "September 17, 2012 at 10:09am PST"],
    "String"
);
stats_tests!(stats_infer_int_string, "type", &["1", "a"], "String");
stats_tests!(stats_infer_string_int, "type", &["a", "1"], "String");
stats_tests!(stats_infer_int_float, "type", &["1", "1.2"], "Float");
stats_tests!(stats_infer_float_int, "type", &["1.2", "1"], "Float");
stats_tests!(
    stats_infer_null_int_float_string,
    "type",
    &["", "1", "1.2", "a"],
    "String"
);
stats_tests!(
    stats_infer_date_string,
    "type",
    &["1968-06-27", "abcde"],
    "String"
);
stats_tests!(
    stats_infer_string_date,
    "type",
    &["wxyz", "1968-06-27"],
    "String"
);

stats_tests!(stats_no_mean, "mean", &["a"], "");
stats_tests!(stats_no_stddev, "stddev", &["a"], "");
stats_tests!(stats_no_variance, "variance", &["a"], "");
stats_tests!(stats_no_median, "median", &["a"], "");
stats_tests!(stats_no_quartiles, "quartiles", &["a"], ",,,,,");
stats_tests!(stats_no_mode, "mode", &["a", "b"], "N/A");
stats_tests!(
    stats_multiple_modes,
    "mode",
    &["a", "a", "b", "b", "c", "d", "e", "e"],
    "a,b,e,3,1"
);
stats_tests!(
    stats_multiple_modes_num,
    "mode",
    &["5", "5", "33", "33", "42", "17", "99", "99"],
    "33,5,99,3,1"
);
stats_tests!(
    stats_multiple_antimodes,
    "antimode",
    &["a", "a", "b", "b", "c", "d", "e", "e"],
    "c,d,2,1"
);
stats_tests!(
    stats_multiple_antimodes_num,
    "antimode",
    &["5", "5", "33", "33", "42", "17", "98", "99", "99"],
    "17,42,98,3,1"
);
stats_tests!(
    stats_range,
    "range",
    &["a", "a", "b", "b", "c", "d", "e", "e"],
    ""
);
stats_tests!(
    stats_range_num,
    "range",
    &["5", "5", "33", "33", "42", "17", "98", "99", "99"],
    "94"
);
stats_tests!(
    stats_sparsity,
    "sparsity",
    &["5", "5", "33", "33", "42", "17", "98", "99", "99", ""],
    "0.1"
);

stats_tests!(stats_null_mean, "mean", &[""], "");
stats_tests!(stats_null_stddev, "stddev", &[""], "");
stats_tests!(stats_null_variance, "variance", &[""], "");
stats_tests!(stats_null_median, "median", &[""], "");
stats_tests!(stats_null_quartiles, "quartiles", &[""], ",,,,,");
stats_tests!(stats_null_mode, "mode", &[""], "N/A");
stats_tests!(stats_null_antimode, "antimode", &[""], "*ALL");
stats_tests!(stats_null_range, "range", &[""], "N/A");
stats_tests!(stats_null_sparsity, "sparsity", &[""], "1.0");

stats_tests!(stats_includenulls_null_mean, "mean", &[""], "", true, false);
stats_tests!(
    stats_includenulls_null_stddev,
    "stddev",
    &[""],
    "",
    true,
    false
);
stats_tests!(
    stats_includenulls_null_variance,
    "variance",
    &[""],
    "",
    true,
    false
);
stats_tests!(
    stats_includenulls_null_median,
    "median",
    &[""],
    "",
    true,
    false
);
stats_tests!(
    stats_includenulls_null_quartiles,
    "quartiles",
    &[""],
    ",,,,,",
    true,
    false
);
stats_tests!(
    stats_includenulls_null_mode,
    "mode",
    &[""],
    "N/A",
    true,
    false
);
stats_tests!(
    stats_includenulls_null_antimode,
    "antimode",
    &[""],
    "*ALL",
    true,
    false
);
stats_tests!(
    stats_includenulls_null_range,
    "range",
    &[""],
    "N/A",
    true,
    false
);
stats_tests!(
    stats_includenulls_null_sparsity,
    "sparsity",
    &[""],
    "1.0",
    true,
    false
);

stats_tests!(
    stats_includenulls_mean,
    "mean",
    &["5", "", "15", "10"],
    "7.5",
    true,
    false
);

stats_tests!(stats_sum_integers, "sum", &["1", "2"], "3");
stats_tests!(stats_sum_floats, "sum", &["1.5", "2.8"], "4.3");
stats_tests!(stats_sum_mixed1, "sum", &["1.5", "2"], "3.5");
stats_tests!(stats_sum_mixed2, "sum", &["2", "1.5"], "3.5");
stats_tests!(stats_sum_mixed3, "sum", &["1.5", "hi", "2.8"], "4.3");
stats_tests!(stats_sum_nulls1, "sum", &["1", "", "2"], "3");
stats_tests!(stats_sum_nulls2, "sum", &["", "1", "2"], "3");
stats_tests!(
    stats_sum_overflow,
    "sum",
    &[
        {
            let i = i64::MAX;
            &i.to_string()
        },
        "1",
        "2"
    ],
    "OVERFLOW"
);
stats_tests!(
    stats_sum_negative_overflow,
    "sum",
    &[
        {
            let i = i64::MIN;
            &i.to_string()
        },
        "-1",
        "-2"
    ],
    "UNDERFLOW"
);

stats_tests!(stats_min, "min", &["2", "1.1"], "1.1");
stats_tests!(stats_max, "max", &["2", "1.1"], "2");
stats_tests!(stats_min_mix, "min", &["2", "a", "1.1"], "1.1");
stats_tests!(stats_max_mix, "max", &["2", "a", "1.1"], "a");
stats_tests!(stats_min_null, "min", &["", "2", "1.1"], "1.1");
stats_tests!(stats_max_null, "max", &["2", "1.1", ""], "2");

stats_tests!(stats_len_min, "min_length", &["aa", "a"], "1");
stats_tests!(stats_len_max, "max_length", &["a", "aa"], "2");
stats_tests!(stats_len_min_null, "min_length", &["", "aa", "a"], "0");
stats_tests!(stats_len_max_null, "max_length", &["a", "aa", ""], "2");

stats_tests!(stats_mean, "mean", &["5", "15", "10"], "10");
stats_tests!(stats_stddev, "stddev", &["1", "2", "3"], "0.8165");
stats_tests!(stats_variance, "variance", &["3", "5", "7", "9", "11"], "8");
stats_tests!(stats_mean_null, "mean", &["", "5", "15", "10"], "10");
stats_tests!(stats_stddev_null, "stddev", &["1", "2", "3", ""], "0.8165");
stats_tests!(
    stats_variance_null,
    "variance",
    &["3", "5", "7", "9", "", "10"],
    "6"
);
stats_tests!(stats_mean_mix, "mean", &["5", "15.1", "9.9"], "10");
stats_tests!(stats_stddev_mix, "stddev", &["1", "2.1", "2.9"], "0.7789");
stats_tests!(
    stats_variance_mix,
    "variance",
    &["1.5", "2", "2.5", "3"],
    "0.3125"
);

stats_tests!(stats_cardinality, "cardinality", &["a", "b", "a"], "2");
stats_tests!(stats_mode, "mode", &["a", "b", "a"], "a,1,2");
stats_tests!(stats_mode_null, "mode", &["", "a", "b", "a"], "a,1,2");
stats_tests!(stats_antimode, "antimode", &["a", "b", "a"], "b,1,1");
stats_tests!(
    stats_antimode_null,
    "antimode",
    &["", "a", "b", "a"],
    "NULL,b,2,1"
);
stats_tests!(stats_median, "median", &["1", "2", "3"], "2");
stats_tests!(stats_median_null, "median", &["", "1", "2", "3"], "2");
stats_tests!(stats_median_even, "median", &["1", "2", "3", "4"], "2.5");
stats_tests!(
    stats_median_even_null,
    "median",
    &["", "1", "2", "3", "4"],
    "2.5"
);
stats_tests!(stats_median_mix, "median", &["1", "2.5", "3"], "2.5");
stats_tests!(
    stats_quartiles,
    "quartiles",
    &["1", "2", "3"],
    "-5,-2,1,2,3,2,6,9"
);
stats_tests!(
    stats_quartiles_null,
    "quartiles",
    &["", "1", "2", "3"],
    "-5,-2,1,2,3,2,6,9"
);
stats_tests!(
    stats_quartiles_even,
    "quartiles",
    &["1", "2", "3", "4"],
    "-4.5,-1.5,1.5,2.5,3.5,2,6.5,9.5"
);
stats_tests!(
    stats_quartiles_even_null,
    "quartiles",
    &["", "1", "2", "3", "4"],
    "-4.5,-1.5,1.5,2.5,3.5,2,6.5,9.5"
);
stats_tests!(
    stats_quartiles_mix,
    "quartiles",
    &["1", "2.0", "3", "4"],
    "-4.5,-1.5,1.5,2.5,3.5,2,6.5,9.5"
);
stats_tests!(stats_quartiles_null_empty, "quartiles", &[""], "");

stats_tests!(stats_nullcount, "nullcount", &["", "1", "2"], "1");
stats_tests!(stats_nullcount_none, "nullcount", &["a", "1", "2"], "0");
stats_tests!(
    stats_nullcount_spacenotnull,
    "nullcount",
    &[" ", "1", "2"],
    "0"
);
stats_tests!(stats_nullcount_all, "nullcount", &["", "", ""], "3");
stats_no_infer_dates_tests!(
    stats_noinfer_null_nodate2,
    "type",
    &["", "September 17, 2012 at 10:09am PST"],
    "String",
    true,
    false
);
stats_no_infer_dates_tests!(
    stats_infer_null_nodate2,
    "type",
    &["", "September 17, 2012 at 10:09am PST"],
    "DateTime",
    true,
    true
);
stats_tests!(
    stats_infer_date_datetime2,
    "type",
    &["September 11, 2001", "September 17, 2012 at 10:09am PST"],
    "DateTime",
    false,
    true
);
stats_no_infer_dates_tests!(
    stats_infer_date_datetime3,
    "type",
    &["9-11", "September 17, 2012 at 10:09am PST"],
    "String",
    false,
    true
);

#[test]
fn stats_prefer_dmy() {
    let wrk = Workdir::new("stats_prefer_dmy");
    let test_file = wrk.load_test_file("boston311-dmy-100.csv");

    let mut cmd = wrk.command("stats");
    cmd.arg("--infer-dates")
        .arg("--prefer-dmy")
        .arg("--dates-whitelist")
        .arg("_dT")
        .arg(test_file);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    wrk.create("in2.csv", got);

    // removed variance & stddev columns as its causing flaky CI test for float values
    let mut cmd = wrk.command("select");
    cmd.arg("1-9,12-").arg("in2.csv");

    let got2: String = wrk.stdout(&mut cmd);
    let expected2 = wrk.load_test_resource("boston311-100-stats.csv");

    assert_eq!(dos2unix(&got2), dos2unix(&expected2).trim_end());
}

#[test]
fn stats_prefer_mdy() {
    let wrk = Workdir::new("stats_prefer_mdy");
    let test_file = wrk.load_test_file("boston311-100.csv");

    let mut cmd = wrk.command("stats");
    cmd.arg("--infer-dates")
        .arg("--dates-whitelist")
        .arg("_dt")
        .arg(test_file);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    wrk.create("in2.csv", got);

    // removed variance & stddev columns as its causing flaky CI test for float values
    let mut cmd = wrk.command("select");
    cmd.arg("1-9,12-").arg("in2.csv");

    let got2: String = wrk.stdout(&mut cmd);

    let expected2 = wrk.load_test_resource("boston311-100-stats.csv");

    assert_eq!(dos2unix(&got2), dos2unix(&expected2).trim_end());
}

#[test]
fn stats_rounding() {
    let wrk = Workdir::new("stats_rounding");
    let test_file = wrk.load_test_file("boston311-100.csv");

    let mut cmd = wrk.command("stats");
    cmd.arg("--everything")
        .args(["--round", "8"])
        .arg(test_file);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    wrk.create("in2.csv", got);

    // removed variance & stddev columns as its causing flaky CI test for float values
    let mut cmd = wrk.command("select");
    cmd.arg("1-9,12-").arg("in2.csv");

    let got2: String = wrk.stdout(&mut cmd);
    let expected2 = wrk.load_test_resource("boston311-100-everything-8places-stats.csv");

    assert_eq!(dos2unix(&got2), dos2unix(&expected2).trim_end());
}

#[test]
fn stats_no_date_inference() {
    let wrk = Workdir::new("stats_no_date_inference");
    let test_file = wrk.load_test_file("boston311-100.csv");

    let mut cmd = wrk.command("stats");
    cmd.arg("--everything").arg(test_file);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    wrk.create("in2.csv", got);

    // removed variance & stddev columns as its causing flaky CI test for float values
    let mut cmd = wrk.command("select");
    cmd.arg("1-9,12-").arg("in2.csv");

    let got2: String = wrk.stdout(&mut cmd);
    let expected2 = wrk.load_test_resource("boston311-100-everything-nodate-stats.csv");

    assert_eq!(dos2unix(&got2), dos2unix(&expected2).trim_end());
}

#[test]
fn stats_with_date_inference() {
    let wrk = Workdir::new("stats_with_date_inference");
    let test_file = wrk.load_test_file("boston311-100.csv");

    let mut cmd = wrk.command("stats");
    cmd.arg("--everything")
        .arg(test_file)
        .arg("--infer-dates")
        .arg("--dates-whitelist")
        .arg("all");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    wrk.create("in2.csv", got);

    // removed variance & stddev columns as its causing flaky CI test for float values
    let mut cmd = wrk.command("select");
    cmd.arg("1-9,12-").arg("in2.csv");

    let got2: String = wrk.stdout(&mut cmd);
    let expected2 = wrk.load_test_resource("boston311-100-everything-date-stats.csv");

    assert_eq!(dos2unix(&got2), dos2unix(&expected2).trim_end());
}

#[test]
fn stats_with_date_inference_default_whitelist() {
    let wrk = Workdir::new("stats_with_date_inference_default_whitelist");
    let test_file = wrk.load_test_file("boston311-100.csv");

    let mut cmd = wrk.command("stats");
    cmd.arg("--everything").arg(test_file).arg("--infer-dates");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    wrk.create("in2.csv", got);

    // removed variance & stddev columns as its causing flaky CI test for float values
    let mut cmd = wrk.command("select");
    cmd.arg("!/variance|stddev/").arg("in2.csv");

    let got2: String = wrk.stdout(&mut cmd);
    let expected2 =
        wrk.load_test_resource("boston311-100-everything-inferdates-defaultwhitelist-stats.csv");

    assert_eq!(dos2unix(&got2), dos2unix(&expected2).trim_end());
}

#[test]
fn stats_with_date_inference_variance_stddev() {
    let wrk = Workdir::new("stats_with_date_inference_variance_stddev");
    let test_file = wrk.load_test_file("boston311-100.csv");

    let mut cmd = wrk.command("stats");
    cmd.arg("--everything")
        .arg(test_file)
        .arg("--infer-dates")
        .arg("--dates-whitelist")
        .arg("aLL");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    wrk.create("in2.csv", got);

    let got2: String = wrk.stdout(&mut cmd);
    let expected2 =
        wrk.load_test_resource("boston311-100-everything-date-stats-variance-stddev.csv");

    assert_eq!(dos2unix(&got2), dos2unix(&expected2).trim_end());
}

#[test]
fn stats_with_date_type() {
    let wrk = Workdir::new("stats_with_date_type");
    let test_file = wrk.load_test_file("boston311-100-notime.csv");

    let mut cmd = wrk.command("stats");
    cmd.arg("--everything")
        .arg(test_file)
        .arg("--infer-dates")
        .arg("--dates-whitelist")
        .arg("all");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    wrk.create("in2.csv", got);

    // removed variance & stddev columns as its causing flaky CI test for float values
    let mut cmd = wrk.command("select");
    cmd.arg("1-9,12-").arg("in2.csv");

    let got2: String = wrk.stdout(&mut cmd);
    let expected2 = wrk.load_test_resource("boston311-100-everything-datenotime-stats.csv");

    assert_eq!(dos2unix(&got2), dos2unix(&expected2).trim_end());
}

#[test]
fn stats_typesonly() {
    let wrk = Workdir::new("stats_typesonly");
    let test_file = wrk.load_test_file("boston311-100.csv");

    let mut cmd = wrk.command("stats");
    cmd.arg("--typesonly").arg(test_file);

    let got: String = wrk.stdout(&mut cmd);

    let expected = wrk.load_test_resource("boston311-100-typesonly-stats.csv");

    assert_eq!(dos2unix(&got), dos2unix(&expected).trim_end());
}

#[test]
fn stats_typesonly_with_dates() {
    let wrk = Workdir::new("stats_typesonly_with_dates");
    let test_file = wrk.load_test_file("boston311-100.csv");

    let mut cmd = wrk.command("stats");
    cmd.arg("--typesonly")
        .arg("--infer-dates")
        .arg("--dates-whitelist")
        .arg("all")
        .arg(test_file);

    let got: String = wrk.stdout(&mut cmd);

    let expected = wrk.load_test_resource("boston311-100-typesonly-withdates-stats.csv");

    assert_eq!(dos2unix(&got), dos2unix(&expected).trim_end());
}

#[test]
fn stats_typesonly_cache_threshold_zero() {
    use std::path::Path;

    let wrk = Workdir::new("stats_typesonly_cache_threshold_zero");
    let test_file = wrk.load_test_file("boston311-100.csv");

    let mut cmd = wrk.command("stats");
    cmd.arg("--typesonly")
        .arg("--infer-dates")
        .arg("--dates-whitelist")
        .arg("all")
        .args(["--cache-threshold", "0"])
        .arg(test_file);

    let got: String = wrk.stdout(&mut cmd);

    let expected = wrk.load_test_resource("boston311-100-typesonly-withdates-stats.csv");

    assert_eq!(dos2unix(&got), dos2unix(&expected).trim_end());

    // check that the stats cache files were NOT created
    assert!(!Path::new(&wrk.path("boston311-100.stats.csv")).exists());
    assert!(!Path::new(&wrk.path("boston311-100.stats.csv.json")).exists());
}

#[test]
fn stats_typesonly_cache() {
    use std::path::Path;

    let wrk = Workdir::new("stats_typesonly_cache");
    let test_file = wrk.load_test_file("boston311-100.csv");

    let mut cmd = wrk.command("stats");
    cmd.arg("--typesonly")
        .arg("--infer-dates")
        .arg("--dates-whitelist")
        .arg("all")
        .arg(test_file);

    let got: String = wrk.stdout(&mut cmd);

    let expected = wrk.load_test_resource("boston311-100-typesonly-withdates-stats.csv");

    assert_eq!(dos2unix(&got), dos2unix(&expected).trim_end());

    // check that the stats cache files were created
    assert!(Path::new(&wrk.path("boston311-100.stats.csv")).exists());
    assert!(Path::new(&wrk.path("boston311-100.stats.csv.json")).exists());
}

#[test]
fn stats_cache() {
    use std::path::Path;

    let wrk = Workdir::new("stats_cache");
    let test_file = wrk.load_test_file("boston311-100.csv");

    let mut cmd = wrk.command("stats");
    cmd.arg("--infer-dates")
        .arg("--dates-whitelist")
        .arg("all")
        // set cache threshold to 1 byte to force cache creation
        .args(["--cache-threshold", "1"])
        .arg(test_file);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);

    wrk.create("in2.csv", got);

    // removed variance & stddev columns as its causing flaky CI test for float values
    let mut cmd = wrk.command("select");
    cmd.arg("1-9,12-").arg("in2.csv");

    let got2: String = wrk.stdout(&mut cmd);
    let expected2 = wrk.load_test_resource("boston311-100-stats.csv");

    assert_eq!(dos2unix(&got2), dos2unix(&expected2).trim_end());

    // check that the stats cache files were created
    assert!(Path::new(&wrk.path("boston311-100.stats.csv")).exists());
    assert!(Path::new(&wrk.path("boston311-100.stats.csv.json")).exists());
}

#[test]
fn stats_infer_boolean_1_0() {
    let wrk = Workdir::new("stats_infer_boolean_1_0");
    let test_file = wrk.load_test_file("boston311-10-boolean-1or0.csv");

    let mut cmd = wrk.command("stats");
    cmd.arg("--infer-boolean").arg(test_file);

    let got: String = wrk.stdout(&mut cmd);

    let expected = wrk.load_test_resource("boston311-10-boolean-1or0-stats.csv");

    assert_eq!(dos2unix(&got), dos2unix(&expected).trim_end());
}

#[test]
fn stats_infer_boolean_t_f() {
    let wrk = Workdir::new("stats_infer_boolean_t_f");
    let test_file = wrk.load_test_file("boston311-10-boolean-tf.csv");

    let mut cmd = wrk.command("stats");
    cmd.arg("--infer-boolean").arg(test_file);

    let got: String = wrk.stdout(&mut cmd);

    let expected = wrk.load_test_resource("boston311-10-boolean-tf-stats.csv");

    assert_eq!(dos2unix(&got), dos2unix(&expected).trim_end());
}

#[test]
fn stats_typesonly_infer_boolean_t_f() {
    let wrk = Workdir::new("stats_typesonly_infer_boolean_t_f");
    let test_file = wrk.load_test_file("boston311-10-boolean-tf.csv");

    let mut cmd = wrk.command("stats");
    cmd.arg("--typesonly").arg("--infer-boolean").arg(test_file);

    let got: String = wrk.stdout(&mut cmd);

    let expected = wrk.load_test_resource("boston311-10-typesonly-boolean-tf-stats.csv");

    assert_eq!(dos2unix(&got), dos2unix(&expected).trim_end());
}

#[test]
fn stats_everything_utf8_japanese_issue817() {
    let wrk = Workdir::new("stats_everything_utf8_japanese");
    let test_file = wrk.load_test_file("utf8-japanesedata.csv");

    let mut cmd = wrk.command("stats");
    cmd.arg("--everything").arg(test_file);

    wrk.assert_success(&mut cmd);
    // TODO: for now, let's just make sure it doesn't crash
    // comparing utf8 output is a bit tricky, with git line endings
    // and other things

    // let got: String = wrk.stdout(&mut cmd);
    // let expected = wrk.load_test_resource("utf8-japanesedata-stats-everything.csv");
    // assert_eq!(dos2unix(&got).trim_end(), dos2unix(&expected).trim_end());
}

#[test]
fn stats_leading_zero_handling() {
    let wrk = Workdir::new("stats_leading_zero_handling");

    wrk.create(
        "data.csv",
        vec![
            svec!["col1", "col2", "col3"],
            svec!["1", "4321", "01"],
            svec!["2", "3210", "02"],
            svec!["3", "2101", "03"],
            svec!["4", "1012", "04"],
            svec!["5", "0", "10"],
        ],
    );

    let mut cmd = wrk.command("stats");
    cmd.arg("--typesonly").arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["field", "type"],
        svec!["col1", "Integer"],
        svec!["col2", "Integer"],
        svec!["col3", "String"],
    ];
    assert_eq!(got, expected);
}

mod stats_infer_nothing {
    // Only test CSV data with headers.
    // Empty CSV data with no headers won't produce any statistical analysis.
    use super::test_stats;
    stats_test_headers!(stats_infer_nothing, "type", &[], "NULL");
}

mod stats_zero_cardinality {
    use super::test_stats;
    stats_test_headers!(stats_zero_cardinality, "cardinality", &[], "0");
}

mod stats_zero_mode {
    use super::test_stats;
    stats_test_headers!(stats_zero_mode, "mode", &[], "N/A");
}

mod stats_zero_mean {
    use super::test_stats;
    stats_test_headers!(stats_zero_mean, "mean", &[], "");
}

mod stats_zero_median {
    use super::test_stats;
    stats_test_headers!(stats_zero_median, "median", &[], "");
}

mod stats_zero_quartiles {
    use super::test_stats;
    stats_test_headers!(stats_zero_quartiles, "quartiles", &[], ",,,,,");
}

mod stats_header_fields {
    use super::test_stats;
    stats_test_headers!(stats_header_field_name, "field", &["a"], "header");
    stats_test_no_headers!(stats_header_no_field_name, "field", &["a"], "0");
}
