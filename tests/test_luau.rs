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
fn luau_aggregation_with_prologue() {
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
        .arg("--prologue")
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
fn luau_aggregation_with_prologue_epilogue() {
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
        .arg("--prologue")
        .arg("tot = 0; gtotal = 0; amt_array = {}")
        .arg("-x")
        .arg("amt_array[_idx] = Amount; tot = tot + Amount; gtotal = gtotal + tot; return tot")
        .arg("--epilogue")
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

    let epilogue = wrk.output_stderr(&mut cmd);
    let expected_epilogue = "Min/Max: 7/72 Grand total of 4 rows: 275\n".to_string();
    assert_eq!(epilogue, expected_epilogue);

    wrk.assert_success(&mut cmd);
}

#[test]
fn luau_aggregation_with_prologue_epilogue_and_luau_syntax() {
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
        .arg("--prologue")
        .arg("tot = 0; gtotal = 0; amt_array = {}")
        .arg("-x")
        .arg("amt_array[_idx] = Amount; tot += if tonumber(Amount) < 0 then Amount * -1 else Amount; gtotal += tot; return tot")
        .arg("--epilogue")
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

    let epilogue = wrk.output_stderr(&mut cmd);
    let expected_epilogue = "Min/Max: -72/13 Grand total of 4 rows: 275\n".to_string();
    assert_eq!(epilogue, expected_epilogue);

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
