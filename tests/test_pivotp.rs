use crate::workdir::Workdir;

macro_rules! pivotp_test {
    ($name:ident, $fun:expr) => {
        mod $name {
            use std::process;

            #[allow(unused_imports)]
            use super::setup;
            use crate::workdir::Workdir;

            #[test]
            fn main() {
                let wrk = setup(stringify!($name));
                let cmd = wrk.command("pivotp");
                $fun(wrk, cmd);
            }
        }
    };
}

fn setup(name: &str) -> Workdir {
    // Sample data for testing pivot operations
    let sales = vec![
        svec!["date", "product", "region", "sales"],
        svec!["2023-01-01", "A", "North", "100"],
        svec!["2023-01-01", "B", "North", "150"],
        svec!["2023-01-01", "A", "South", "200"],
        svec!["2023-01-02", "B", "South", "250"],
        svec!["2023-01-02", "A", "North", "300"],
        svec!["2023-01-02", "B", "North", "350"],
    ];

    let wrk = Workdir::new(name);
    wrk.create("sales.csv", sales);
    wrk
}

// Test basic pivot with single index
pivotp_test!(pivotp_basic, |wrk: Workdir, mut cmd: process::Command| {
    cmd.args(&[
        "product",
        "--index",
        "date",
        "--values",
        "sales",
        "--agg",
        "first",
        "sales.csv",
    ]);

    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["date", "A", "B"],
        svec!["2023-01-01", "100", "150"],
        svec!["2023-01-02", "300", "250"],
    ];
    assert_eq!(got, expected);
});

// Test pivot with multiple index columns
pivotp_test!(
    pivotp_multi_index,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.args(&[
            "product",
            "--index",
            "date,region",
            "--values",
            "sales",
            "--agg",
            "sum",
            "sales.csv",
        ]);

        wrk.assert_success(&mut cmd);

        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let expected = vec![
            svec!["date", "region", "A", "B"],
            svec!["2023-01-01", "North", "100", "150"],
            svec!["2023-01-01", "South", "200", ""],
            svec!["2023-01-02", "South", "", "250"],
            svec!["2023-01-02", "North", "300", "350"],
        ];
        assert_eq!(got, expected);
    }
);

// Test pivot with sum aggregation
pivotp_test!(pivotp_sum_agg, |wrk: Workdir, mut cmd: process::Command| {
    cmd.args(&[
        "product",
        "--index",
        "region",
        "--values",
        "sales",
        "--agg",
        "sum",
        "sales.csv",
    ]);

    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["region", "A", "B"],
        svec!["North", "400", "500"],
        svec!["South", "200", "250"],
    ];
    assert_eq!(got, expected);
});

// Test pivot with mean aggregation
pivotp_test!(
    pivotp_mean_agg,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.args(&[
            "product",
            "--index",
            "region",
            "--values",
            "sales",
            "--agg",
            "mean",
            "sales.csv",
        ]);

        wrk.assert_success(&mut cmd);

        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let expected = vec![
            svec!["region", "A", "B"],
            svec!["North", "200.0", "250.0"],
            svec!["South", "200.0", "250.0"],
        ];
        assert_eq!(got, expected);
    }
);

// Test pivot with min aggregation
pivotp_test!(pivotp_min_agg, |wrk: Workdir, mut cmd: process::Command| {
    cmd.args(&[
        "product",
        "--index",
        "region",
        "--values",
        "sales",
        "--agg",
        "min",
        "sales.csv",
    ]);

    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["region", "A", "B"],
        svec!["North", "100", "150"],
        svec!["South", "200", "250"],
    ];
    assert_eq!(got, expected);
});

// Test pivot with max aggregation
pivotp_test!(pivotp_max_agg, |wrk: Workdir, mut cmd: process::Command| {
    cmd.args(&[
        "product",
        "--index",
        "region",
        "--values",
        "sales",
        "--agg",
        "max",
        "sales.csv",
    ]);

    wrk.assert_success(&mut cmd);

    let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
    let expected = vec![
        svec!["region", "A", "B"],
        svec!["North", "300", "350"],
        svec!["South", "200", "250"],
    ];
    assert_eq!(got, expected);
});

// Test pivot with median aggregation
pivotp_test!(
    pivotp_median_agg,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.args(&[
            "product",
            "--index",
            "region",
            "--values",
            "sales",
            "--agg",
            "median",
            "sales.csv",
        ]);

        wrk.assert_success(&mut cmd);

        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let expected = vec![
            svec!["region", "A", "B"],
            svec!["North", "200.0", "250.0"],
            svec!["South", "200.0", "250.0"],
        ];
        assert_eq!(got, expected);
    }
);

// Test pivot with count aggregation
pivotp_test!(
    pivotp_count_agg,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.args(&[
            "product",
            "--index",
            "region",
            "--values",
            "sales",
            "--agg",
            "count",
            "sales.csv",
        ]);

        wrk.assert_success(&mut cmd);

        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let expected = vec![
            svec!["region", "A", "B"],
            svec!["North", "2", "2"],
            svec!["South", "1", "1"],
        ];
        assert_eq!(got, expected);
    }
);

// Test pivot with last aggregation
pivotp_test!(
    pivotp_last_agg,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.args(&[
            "product",
            "--index",
            "region",
            "--values",
            "sales",
            "--agg",
            "last",
            "sales.csv",
        ]);

        wrk.assert_success(&mut cmd);

        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let expected = vec![
            svec!["region", "A", "B"],
            svec!["North", "300", "350"],
            svec!["South", "200", "250"],
        ];
        assert_eq!(got, expected);
    }
);

// Test pivot with sorted columns
pivotp_test!(
    pivotp_sort_columns,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.args(&[
            "product",
            "--index",
            "date",
            "--values",
            "sales",
            "--sort-columns",
            "--agg",
            "first",
            "sales.csv",
        ]);

        wrk.assert_success(&mut cmd);

        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let expected = vec![
            svec!["date", "A", "B"], // Columns will be sorted alphabetically
            svec!["2023-01-01", "100", "150"],
            svec!["2023-01-02", "300", "250"],
        ];
        assert_eq!(got, expected);
    }
);

// Test pivot with custom column separator
pivotp_test!(
    pivotp_col_separator,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.args(&[
            "product",
            "--index",
            "date",
            "--values",
            "sales",
            "--col-separator",
            "::",
            "--agg",
            "first",
            "sales.csv",
        ]);

        wrk.assert_success(&mut cmd);

        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let expected = vec![
            svec!["date", "A", "B"],
            svec!["2023-01-01", "100", "150"],
            svec!["2023-01-02", "300", "250"],
        ];
        assert_eq!(got, expected);
    }
);

// Test pivot with custom delimiter
pivotp_test!(
    pivotp_delimiter,
    |wrk: Workdir, mut cmd: process::Command| {
        // Create data with semicolon delimiter
        let sales = vec![
            svec!["date;product;region;sales"],
            svec!["2023-01-01;A;North;100"],
            svec!["2023-01-01;B;North;150"],
        ];
        wrk.create("sales_semicolon.csv", sales);

        cmd.args(&[
            "product",
            "--index",
            "date",
            "--values",
            "sales",
            "--delimiter",
            ";",
            "sales_semicolon.csv",
        ]);

        wrk.assert_success(&mut cmd);

        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let expected = vec![svec!["date;A;B"], svec!["2023-01-01;1;1"]];
        assert_eq!(got, expected);
    }
);

// Test pivot with no explicit index (uses remaining columns)
pivotp_test!(
    pivotp_no_index,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.args(&["product", "--values", "sales", "--agg", "sum", "sales.csv"]);

        wrk.assert_success(&mut cmd);

        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let expected = vec![
            svec!["date", "region", "A", "B"],
            svec!["2023-01-01", "North", "100", "150"],
            svec!["2023-01-01", "South", "200", ""],
            svec!["2023-01-02", "South", "", "250"],
            svec!["2023-01-02", "North", "300", "350"],
        ];
        assert_eq!(got, expected);
    }
);

// Test pivot with multiple on-cols
pivotp_test!(
    pivotp_multi_on_cols,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.args(&[
            "product,region", // Multiple on-cols
            "--index",
            "date",
            "--values",
            "sales",
            "--agg",
            "sum",
            "sales.csv",
        ]);

        wrk.assert_success(&mut cmd);

        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let expected = vec![
            svec![
                "date",
                "{\"A\",\"North\"}",
                "{\"B\",\"North\"}",
                "{\"A\",\"South\"}",
                "{\"B\",\"South\"}"
            ],
            svec!["2023-01-01", "100", "150", "200", ""],
            svec!["2023-01-02", "300", "350", "", "250"],
        ];
        assert_eq!(got, expected);
    }
);

// Test pivot with multiple value columns
pivotp_test!(
    pivotp_multi_values,
    |wrk: Workdir, mut cmd: process::Command| {
        // Create test data with multiple value columns
        let sales_multi = vec![
            svec!["date", "product", "region", "sales", "quantity"],
            svec!["2023-01-01", "A", "North", "100", "10"],
            svec!["2023-01-01", "B", "North", "150", "15"],
            svec!["2023-01-01", "A", "South", "200", "20"],
            svec!["2023-01-02", "B", "South", "250", "25"],
            svec!["2023-01-02", "A", "North", "300", "30"],
            svec!["2023-01-02", "B", "North", "350", "35"],
        ];
        wrk.create("sales_multi.csv", sales_multi);

        cmd.args(&[
            "product",
            "--index",
            "date",
            "--values",
            "sales,quantity", // Multiple value columns
            "--agg",
            "sum",
            "sales_multi.csv",
        ]);

        wrk.assert_success(&mut cmd);

        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let expected = vec![
            svec!["date", "sales_A", "sales_B", "quantity_A", "quantity_B"],
            svec!["2023-01-01", "300", "150", "30", "15"],
            svec!["2023-01-02", "300", "600", "30", "60"],
        ];
        assert_eq!(got, expected);
    }
);

pivotp_test!(
    pivotp_multi_values_custom_col_separator,
    |wrk: Workdir, mut cmd: process::Command| {
        let sales_multi = vec![
            svec!["date", "product", "region", "sales", "quantity"],
            svec!["2023-01-01", "A", "North", "100", "10"],
            svec!["2023-01-01", "B", "North", "150", "15"],
            svec!["2023-01-01", "A", "South", "200", "20"],
            svec!["2023-01-02", "B", "South", "250", "25"],
            svec!["2023-01-02", "A", "North", "300", "30"],
            svec!["2023-01-02", "B", "North", "350", "35"],
        ];
        wrk.create("sales_multi.csv", sales_multi);

        cmd.args(&[
            "product",
            "--index",
            "date",
            "--values",
            "sales,quantity",
            "--agg",
            "sum",
            "--col-separator",
            "<->",
            "sales_multi.csv",
        ]);

        wrk.assert_success(&mut cmd);

        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let expected = vec![
            svec![
                "date",
                "sales<->A",
                "sales<->B",
                "quantity<->A",
                "quantity<->B"
            ],
            svec!["2023-01-01", "300", "150", "30", "15"],
            svec!["2023-01-02", "300", "600", "30", "60"],
        ];
        assert_eq!(got, expected);
    }
);

// Test pivot with try-parsedates flag
pivotp_test!(
    pivotp_try_parsedates,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.args(&[
            "product",
            "--index",
            "date",
            "--values",
            "sales",
            "--try-parsedates",
            "--agg",
            "sum",
            "sales.csv",
        ]);

        wrk.assert_success(&mut cmd);

        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let expected = vec![
            svec!["date", "A", "B"],
            svec!["2023-01-01", "300", "150"],
            svec!["2023-01-02", "300", "600"],
        ];
        assert_eq!(got, expected);
    }
);

// Test pivot with decimal comma
pivotp_test!(
    pivotp_decimal_comma,
    |wrk: Workdir, mut cmd: process::Command| {
        // Create data with decimal commas
        let sales_decimal = vec![
            svec!["date", "product", "region", "sales"],
            svec!["2023-01-01", "A", "North", "100,50"],
            svec!["2023-01-01", "B", "North", "150,75"],
        ];
        wrk.create_with_delim("sales_decimal.csv", sales_decimal, b';');

        cmd.args(&[
            "product",
            "--index",
            "date",
            "--values",
            "sales",
            "--decimal-comma",
            "--delimiter",
            ";",
            "sales_decimal.csv",
        ]);

        wrk.assert_success(&mut cmd);

        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let expected = vec![svec!["date;A;B"], svec!["2023-01-01;1;1"]];
        assert_eq!(got, expected);
    }
);

// Test pivot with validation
pivotp_test!(
    pivotp_validate,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.args(&[
            "product",
            "--index",
            "date",
            "--values",
            "sales",
            "--validate",
            "sales.csv",
        ]);

        wrk.assert_success(&mut cmd);

        let msg = wrk.output_stderr(&mut cmd);
        let expected_msg = "Pivot column cardinalities:\n  product: 2\n(2, 3)\n";
        assert_eq!(msg, expected_msg);

        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let expected = vec![
            svec!["date", "A", "B"],
            svec!["2023-01-01", "2", "1"],
            svec!["2023-01-02", "1", "2"],
        ];
        assert_eq!(got, expected);
    }
);

// Test pivot with custom infer length
pivotp_test!(
    pivotp_infer_len,
    |wrk: Workdir, mut cmd: process::Command| {
        cmd.args(&[
            "product",
            "--index",
            "date",
            "--values",
            "sales",
            "--infer-len",
            "5",
            "sales.csv",
        ]);

        wrk.assert_success(&mut cmd);

        let got: Vec<Vec<String>> = wrk.read_stdout(&mut cmd);
        let expected = vec![
            svec!["date", "A", "B"],
            svec!["2023-01-01", "2", "1"],
            svec!["2023-01-02", "1", "2"],
        ];
        assert_eq!(got, expected);
    }
);
