use crate::workdir::Workdir;

// Basic output assuming the user successfully chose a file (did not cancel).
#[test]
#[ignore = "Requires GUI to test."]
fn prompt_success() {
    let wrk = Workdir::new("prompt");
    // Create a CSV file with sample data
    wrk.create_indexed(
        "fruits.csv",
        vec![
            svec!["fruit", "price"],
            svec!["apple", "2.50"],
            svec!["banana", "3.00"],
            svec!["strawberry", "1.50"],
        ],
    );

    // Run the command
    let mut cmd = wrk.command("prompt");
    let got: String = wrk.stdout(&mut cmd);
    let expected = r"fruit,price
apple,2.50
banana,3.00
strawberry,1.50";
    // Check that we receive the correct output
    assert_eq!(got, expected.to_string());
}
