use crate::workdir::Workdir;

#[test]
fn luau_map() {
    let wrk = Workdir::new("luau");
    wrk.create(
        "data.csv",
        vec![
            svec!["letter", "number"],
            svec!["a", "13"],
            svec!["b", "24"],
            svec!["c", "72"],
            svec!["d", "7"],
        ],
    );
    let mut cmd = wrk.command("luau");
    cmd.arg("map").arg("inc").arg("number + 1").arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["letter", "number", "inc"],
        svec!["a", "13", "14"],
        svec!["b", "24", "25"],
        svec!["c", "72", "73"],
        svec!["d", "7", "8"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn luau_map_idx() {
    let wrk = Workdir::new("luau");
    wrk.create(
        "data.csv",
        vec![
            svec!["letter", "number"],
            svec!["a", "13"],
            svec!["b", "24"],
            svec!["c", "72"],
            svec!["d", "7"],
        ],
    );
    let mut cmd = wrk.command("luau");
    cmd.arg("map")
        .arg("inc")
        .arg("number * _idx")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["letter", "number", "inc"],
        svec!["a", "13", "13"],
        svec!["b", "24", "48"],
        svec!["c", "72", "216"],
        svec!["d", "7", "28"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn luau_aggregation() {
    let wrk = Workdir::new("luau");
    wrk.create(
        "data.csv",
        vec![
            svec!["letter", "Amount"],
            svec!["a", "13"],
            svec!["b", "24"],
            svec!["c", "72"],
            svec!["d", "7"],
        ],
    );
    let mut cmd = wrk.command("luau");
    cmd.arg("map")
        .arg("Total")
        .arg("-x")
        .arg("tot = (tot or 0) + Amount; return tot")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["letter", "Amount", "Total"],
        svec!["a", "13", "13"],
        svec!["b", "24", "37"],
        svec!["c", "72", "109"],
        svec!["d", "7", "116"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn luau_aggregation_with_begin() {
    let wrk = Workdir::new("luau");
    wrk.create(
        "data.csv",
        vec![
            svec!["letter", "Amount"],
            svec!["a", "13"],
            svec!["b", "24"],
            svec!["c", "72"],
            svec!["d", "7"],
        ],
    );
    let mut cmd = wrk.command("luau");
    cmd.arg("map")
        .arg("Total")
        .arg("--begin")
        .arg("tot = 0")
        .arg("-x")
        .arg("tot = tot + Amount; return tot")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["letter", "Amount", "Total"],
        svec!["a", "13", "13"],
        svec!["b", "24", "37"],
        svec!["c", "72", "109"],
        svec!["d", "7", "116"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn luau_aggregation_with_begin_end() {
    let wrk = Workdir::new("luau");
    wrk.create(
        "data.csv",
        vec![
            svec!["letter", "Amount"],
            svec!["a", "13"],
            svec!["b", "24"],
            svec!["c", "72"],
            svec!["d", "7"],
        ],
    );
    let mut cmd = wrk.command("luau");
    cmd.arg("map")
        .arg("Total")
        .arg("--begin")
        .arg("tot = 0; gtotal = 0; amt_array = {}")
        .arg("-x")
        .arg("amt_array[_idx] = Amount; tot = tot + Amount; gtotal = gtotal + tot; return tot")
        .arg("--end")
        .arg(r#"return ("Min/Max: " .. math.min(unpack(amt_array)) .. "/" .. math.max(unpack(amt_array)) .. " Grand total of " .. _rowcount .. " rows: " .. gtotal)"#)
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["letter", "Amount", "Total"],
        svec!["a", "13", "13"],
        svec!["b", "24", "37"],
        svec!["c", "72", "109"],
        svec!["d", "7", "116"],
    ];
    assert_eq!(got, expected);

    let end = wrk.output_stderr(&mut cmd);
    let expected_end = "Min/Max: 7/72 Grand total of 4 rows: 275\n".to_string();
    assert_eq!(end, expected_end);

    wrk.assert_success(&mut cmd);
}

#[test]
fn luau_aggregation_with_embedded_begin_end() {
    let wrk = Workdir::new("luau_embedded_begin_end");
    wrk.create(
        "data.csv",
        vec![
            svec!["letter", "Amount"],
            svec!["a", "13"],
            svec!["b", "24"],
            svec!["c", "72"],
            svec!["d", "7"],
        ],
    );
    let mut cmd = wrk.command("luau");
    cmd.arg("map")
        .arg("Total")
        .arg("-x")
        .arg(
            "BEGIN {tot = 0; gtotal = 0; amt_array = {}}! amt_array[_idx] = Amount; tot = tot + \
             Amount; gtotal = gtotal + tot; return tot END {return (\"Min/Max: \" .. \
             math.min(unpack(amt_array)) .. \"/\" .. math.max(unpack(amt_array)) .. \" Grand \
             total of \" .. _rowcount .. \" rows: \" .. gtotal)}!",
        )
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["letter", "Amount", "Total"],
        svec!["a", "13", "13"],
        svec!["b", "24", "37"],
        svec!["c", "72", "109"],
        svec!["d", "7", "116"],
    ];
    assert_eq!(got, expected);

    let end = wrk.output_stderr(&mut cmd);
    let expected_end = "Min/Max: 7/72 Grand total of 4 rows: 275\n".to_string();
    assert_eq!(end, expected_end);

    wrk.assert_success(&mut cmd);
}

#[test]
fn luau_aggregation_with_embedded_begin_end_using_file() {
    let wrk = Workdir::new("luau_embedded_begin_end");
    wrk.create(
        "data.csv",
        vec![
            svec!["letter", "Amount"],
            svec!["a", "13"],
            svec!["b", "24"],
            svec!["c", "72"],
            svec!["d", "7"],
        ],
    );

    wrk.create_from_string(
        "testbeginend.luau",
        r#"
BEGIN {
    -- this is the BEGIN block
    -- where we typically initialize variables
    running_total = 0;
    grand_total = 0;
    amt_array = {};
}!

-- this is the main script, which is executed for each row
-- note that we use the _idx special variable to get the row index
amt_array[_idx] = Amount;
running_total = running_total + Amount;
grand_total = grand_total + running_total;
-- running_total is the value we "map" to the "Running Total" column of each row
return running_total;

END {
    -- and this is the end block
    -- note that we use the _rowcount special variable to get the number of rows
    return ("Min/Max: " .. math.min(unpack(amt_array)) .. "/" .. math.max(unpack(amt_array)) .. " Grand total of " .. _rowcount .. " rows: " .. grand_total);
}!        
"#,
    );

    let mut cmd = wrk.command("luau");
    cmd.arg("map")
        .arg("Running Total")
        .arg("-x")
        .arg("file:testbeginend.luau")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["letter", "Amount", "Running Total"],
        svec!["a", "13", "13"],
        svec!["b", "24", "37"],
        svec!["c", "72", "109"],
        svec!["d", "7", "116"],
    ];
    assert_eq!(got, expected);

    let end = wrk.output_stderr(&mut cmd);
    let expected_end = "Min/Max: 7/72 Grand total of 4 rows: 275\n".to_string();
    assert_eq!(end, expected_end);

    wrk.assert_success(&mut cmd);
}

#[test]
fn luau_aggregation_with_embedded_begin_end_and_beginend_options() {
    // when a main script has BEGIN/END blocks, and --begin/--end options are also specified,
    // the --begin/--end options take precedence and the embedded BEGIN/END blocks are ignored.
    let wrk = Workdir::new("luau_embedded_begin_end_options");
    wrk.create(
        "data.csv",
        vec![
            svec!["letter", "Amount"],
            svec!["a", "13"],
            svec!["b", "24"],
            svec!["c", "72"],
            svec!["d", "7"],
        ],
    );
    let mut cmd = wrk.command("luau");
    cmd.arg("map")
        .arg("Total")
        .arg("--begin")
        .arg("tot = 1; gtotal = 0; amt_array = {}")
        .arg("-x")
        .arg(
            "BEGIN {tot = 0; gtotal = 0; amt_array = {}}! amt_array[_idx] = Amount; tot = tot + \
             Amount; gtotal = gtotal + tot; return tot END {return (\"Min/Max: \" .. \
             math.min(unpack(amt_array)) .. \"/\" .. math.max(unpack(amt_array)) .. \" Grand \
             total of \" .. _rowcount .. \" rows: \" .. gtotal)}!",
        )
        .arg("--end")
        .arg(
            "return (\"Minimum/Maximum: \" .. math.min(unpack(amt_array)) .. \"/\" .. \
             math.max(unpack(amt_array)) .. \" Grand total of \" .. _rowcount .. \" rows: \" .. \
             gtotal)",
        )
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["letter", "Amount", "Total"],
        svec!["a", "13", "14"],
        svec!["b", "24", "38"],
        svec!["c", "72", "110"],
        svec!["d", "7", "117"],
    ];
    assert_eq!(got, expected);

    let end = wrk.output_stderr(&mut cmd);
    let expected_end = "Minimum/Maximum: 7/72 Grand total of 4 rows: 279\n".to_string();
    assert_eq!(end, expected_end);

    wrk.assert_success(&mut cmd);
}

#[test]
fn luau_aggregation_with_begin_end_and_luau_syntax() {
    let wrk = Workdir::new("luau");
    wrk.create(
        "data.csv",
        vec![
            svec!["letter", "Amount"],
            svec!["a", "13"],
            svec!["b", "-24"],
            svec!["c", "-72"],
            svec!["d", "7"],
        ],
    );
    let mut cmd = wrk.command("luau");
    cmd.arg("map")
        .arg("Total")
        .arg("--begin")
        .arg("tot = 0; gtotal = 0; amt_array = {}")
        .arg("-x")
        .arg("amt_array[_idx] = Amount; tot += if tonumber(Amount) < 0 then Amount * -1 else Amount; gtotal += tot; return tot")
        .arg("--end")
        .arg(r#"return ("Min/Max: " .. math.min(unpack(amt_array)) .. "/" .. math.max(unpack(amt_array)) .. " Grand total of " .. _rowcount .. " rows: " .. gtotal)"#)
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["letter", "Amount", "Total"],
        svec!["a", "13", "13"],
        svec!["b", "-24", "37"],
        svec!["c", "-72", "109"],
        svec!["d", "7", "116"],
    ];
    assert_eq!(got, expected);

    let end = wrk.output_stderr(&mut cmd);
    let expected_end = "Min/Max: -72/13 Grand total of 4 rows: 275\n".to_string();
    assert_eq!(end, expected_end);

    wrk.assert_success(&mut cmd);
}

#[test]
fn luau_map_math() {
    let wrk = Workdir::new("luau");
    wrk.create(
        "data.csv",
        vec![
            svec!["letter", "number"],
            svec!["a", "13"],
            svec!["b", "24"],
            svec!["c", "72"],
            svec!["d", "7"],
        ],
    );
    let mut cmd = wrk.command("luau");
    cmd.arg("map")
        .arg("div")
        .arg("math.floor(number / 2)")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["letter", "number", "div"],
        svec!["a", "13", "6"],
        svec!["b", "24", "12"],
        svec!["c", "72", "36"],
        svec!["d", "7", "3"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn luau_map_require_luadate() {
    let wrk = Workdir::new("luau");
    wrk.create(
        "data.csv",
        vec![
            svec!["letter", "number", "date_col"],
            svec!["a", "13", "2001-09-11"],
            svec!["b", "24", "1989-11-09"],
            svec!["c", "72", "2008-11-04"],
            svec!["d", "7", "2020-03-11"],
        ],
    );
    let mut cmd = wrk.command("luau");
    cmd.arg("map")
        .arg("days_added")
        .arg("-x")
        .arg(r#"local date = require "date";local t_date = date(date_col):adddays(tonumber(number)); return tostring(t_date:fmt("${iso}"))"#)
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["letter", "number", "date_col", "days_added"],
        svec!["a", "13", "2001-09-11", "2001-09-24T00:00:00"],
        svec!["b", "24", "1989-11-09", "1989-12-03T00:00:00"],
        svec!["c", "72", "2008-11-04", "2009-01-15T00:00:00"],
        svec!["d", "7", "2020-03-11", "2020-03-18T00:00:00"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn luau_map_require() {
    let wrk = Workdir::new("luau");
    wrk.create(
        "data.csv",
        vec![
            svec!["letter", "number1", "number2"],
            svec!["a", "13", "43"],
            svec!["b", "24", "3.14"],
            svec!["c", "72", "42"],
            svec!["d", "7", "6.5"],
        ],
    );

    let mintest_luau = r#"
local mintest = {}

function mintest.mymin(n1, n2)
    if (tonumber(n1) < tonumber(n2)) then
        return n1;
    else
        return n2;
    end
end

return mintest"#;

    wrk.create_from_string("mintest.luau", mintest_luau);

    let mut cmd = wrk.command("luau");
    cmd.arg("map")
        .arg("min")
        .arg("-x")
        .arg(r#"local mintest = require "mintest";local t_min = mintest.mymin(number1,number2); return t_min"#)
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["letter", "number1", "number2", "min"],
        svec!["a", "13", "43", "13"],
        svec!["b", "24", "3.14", "3.14"],
        svec!["c", "72", "42", "42"],
        svec!["d", "7", "6.5", "6.5"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn luau_map_error() {
    let wrk = Workdir::new("luau");
    wrk.create(
        "data.csv",
        vec![
            svec!["letter", "number"],
            svec!["a", "13"],
            svec!["b", "24"],
            svec!["c", "72"],
            svec!["d", "7"],
        ],
    );
    let mut cmd = wrk.command("luau");
    cmd.arg("map")
        .arg("div")
        .arg("math.dancefloor(number / 2)")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["letter", "number", "div"],
        svec!["a", "13", "<ERROR>"],
        svec!["b", "24", "<ERROR>"],
        svec!["c", "72", "<ERROR>"],
        svec!["d", "7", "<ERROR>"],
    ];
    assert_eq!(got, expected);

    wrk.assert_err(&mut cmd);
    let stderr_string = wrk.output_stderr(&mut cmd);
    assert!(stderr_string.ends_with("Luau errors encountered: 4\n"));
}

#[test]
fn luau_map_header_with_nonalphanumeric_chars() {
    let wrk = Workdir::new("luau");
    wrk.create(
        "data.csv",
        vec![
            svec!["letter-column", "number column"],
            svec!["a", "13"],
            svec!["b", "24"],
            svec!["c", "72"],
            svec!["d", "7"],
        ],
    );
    let mut cmd = wrk.command("luau");
    cmd.arg("map")
        .arg(r#"div/column"#)
        .arg(r#"col["letter-column"] .. math.floor(col["number column"] / 2)"#)
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["letter-column", "number column", "div/column"],
        svec!["a", "13", "a6"],
        svec!["b", "24", "b12"],
        svec!["c", "72", "c36"],
        svec!["d", "7", "d3"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn luau_map_no_headers() {
    let wrk = Workdir::new("luau");
    wrk.create(
        "data.csv",
        vec![
            svec!["a", "13"],
            svec!["b", "24"],
            svec!["c", "72"],
            svec!["d", "7"],
        ],
    );
    let mut cmd = wrk.command("luau");
    cmd.arg("map")
        .arg("col[2] + 1")
        .arg("--no-headers")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["a", "13", "14"],
        svec!["b", "24", "25"],
        svec!["c", "72", "73"],
        svec!["d", "7", "8"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn luau_map_exec() {
    let wrk = Workdir::new("luau");
    wrk.create(
        "data.csv",
        vec![
            svec!["letter", "x"],
            svec!["a", "13"],
            svec!["b", "24"],
            svec!["c", "72"],
            svec!["d", "7"],
        ],
    );
    let mut cmd = wrk.command("luau");
    cmd.arg("map")
        .arg("running_total")
        .arg("-x")
        .arg("tot = (tot or 0) + x; return tot")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["letter", "x", "running_total"],
        svec!["a", "13", "13"],
        svec!["b", "24", "37"],
        svec!["c", "72", "109"],
        svec!["d", "7", "116"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn luau_map_no_globals() {
    let wrk = Workdir::new("luau");
    wrk.create(
        "data.csv",
        vec![
            svec!["y", "x"],
            svec!["1", "13"],
            svec!["2", "24"],
            svec!["3", "72"],
            svec!["4", "7"],
        ],
    );
    let mut cmd = wrk.command("luau");
    cmd.arg("map")
        .arg("z")
        .arg("-g")
        .arg("(x or col[1]) + 1")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["y", "x", "z"],
        svec!["1", "13", "2"],
        svec!["2", "24", "3"],
        svec!["3", "72", "4"],
        svec!["4", "7", "5"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn luau_map_boolean() {
    let wrk = Workdir::new("luau");
    wrk.create(
        "data.csv",
        vec![
            svec!["letter", "number"],
            svec!["a", "13"],
            svec!["b", "24"],
            svec!["c", "72"],
            svec!["d", "7"],
        ],
    );
    let mut cmd = wrk.command("luau");
    cmd.arg("map")
        .arg("test")
        .arg("tonumber(number) > 14")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["letter", "number", "test"],
        svec!["a", "13", "false"],
        svec!["b", "24", "true"],
        svec!["c", "72", "true"],
        svec!["d", "7", "false"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn luau_filter() {
    let wrk = Workdir::new("luau");
    wrk.create(
        "data.csv",
        vec![
            svec!["letter", "number"],
            svec!["a", "13"],
            svec!["b", "24"],
            svec!["c", "72"],
            svec!["d", "7"],
        ],
    );
    let mut cmd = wrk.command("luau");
    cmd.arg("filter")
        .arg("tonumber(number) > 14")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["letter", "number"],
        svec!["b", "24"],
        svec!["c", "72"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn luau_filter_error() {
    let wrk = Workdir::new("luau");
    wrk.create(
        "data.csv",
        vec![
            svec!["letter", "number"],
            svec!["a", "13"],
            svec!["b", "24"],
            svec!["c", "72"],
            svec!["d", "7"],
        ],
    );
    let mut cmd = wrk.command("luau");
    cmd.arg("filter")
        .arg("tothedancenumber(number) > 14")
        .arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["letter", "number"],
        svec!["a", "13"],
        svec!["b", "24"],
        svec!["c", "72"],
        svec!["d", "7"],
    ];
    assert_eq!(got, expected);
}

#[test]
fn luau_filter_num() {
    let wrk = Workdir::new("luau");
    wrk.create(
        "data.csv",
        vec![
            svec!["letter", "number"],
            svec!["a", "13"],
            svec!["b", "24"],
            svec!["c", "72"],
            svec!["d", "-7"],
            svec!["e", "0"],
            svec!["f", "42"],
            svec!["g", "0.0"],
            svec!["h", "3.14"],
            svec!["i", "0.000123"],
            svec!["j", "-7.01"],
            svec!["k", "0.0000"],
        ],
    );
    let mut cmd = wrk.command("luau");
    cmd.arg("filter").arg("tonumber(number)").arg("data.csv");

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["letter", "number"],
        svec!["a", "13"],
        svec!["b", "24"],
        svec!["c", "72"],
        svec!["d", "-7"],
        svec!["f", "42"],
        svec!["h", "3.14"],
        svec!["i", "0.000123"],
        svec!["j", "-7.01"],
    ];
    assert_eq!(got, expected);
}
