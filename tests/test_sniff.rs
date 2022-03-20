use crate::workdir::Workdir;

#[test]
fn sniff() {
    let wrk = Workdir::new("sniff");
    wrk.create(
        "in.csv",
        vec![
            svec!["id", "timestamp", "h3"],
            svec!["1", "2021-04-26 00:02:18", "a"],
            svec!["2", "2021-04-26 19:22:26", "b"],
            svec!["30", "2021-04-26 11:44:13", "c"],
            svec!["4", "2021-04-26 14:37:03", "d"],
            svec!["2", "2021-04-26 20:22:26", "e"],
            svec!["5", "2021-04-26 19:29:26", "f"],
            svec!["60", "2021-04-26 04:52:46", "g"],
            svec!["2", "2021-04-26 19:12:26", "h"],
            svec!["30", "2021-04-26 10:44:13", "i"],
            svec!["30", "2021-04-26 09:44:13", "j"],
            svec!["1", "2021-04-26 01:02:18", "k"],
        ],
    );

    let mut cmd = wrk.command("sniff");
    cmd.arg("in.csv");

    let got: String = wrk.stdout(&mut cmd);

    let expected = r#"Metadata
========
Dialect:
	Delimiter: ,
	Has header row?: true
	Number of preamble rows: 0
	Quote character: none
	Double-quote escapes?: true
	Escape character: none
	Comment character: none
	Flexible: false

Number of fields: 3
Types:
	0: Unsigned
	1: Text
	2: Text"#;

    assert_eq!(got, expected);
}
