use crate::workdir::Workdir;

fn data(headers: bool) -> String {
    if headers {
        String::from("name,age,city\nJohn,30,New York\nJane,25,Boston\n")
    } else {
        String::from("John,30,New York\nJane,25,Boston\n")
    }
}

#[test]
fn template_basic() {
    let wrk = Workdir::new("template_basic");
    wrk.create_from_string("data.csv", &data(true));
    wrk.create_from_string("template.txt", "Hello {{name}} from {{city}}!\n\n");

    let mut cmd = wrk.command("template");
    cmd.arg("--template-file")
        .arg("template.txt")
        .arg("data.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = "Hello John from New York!\nHello Jane from Boston!";

    wrk.assert_success(&mut cmd);
    assert_eq!(got, expected);
}

#[test]
fn template_no_headers() {
    let wrk = Workdir::new("template_no_headers");
    wrk.create_from_string("data.csv", &data(true));
    wrk.create_from_string("template.txt", "Name: {{_c1}}, Age: {{_c2}}\n\n");

    let mut cmd = wrk.command("template");
    cmd.arg("--template-file")
        .arg("template.txt")
        .arg("data.csv")
        .arg("--no-headers");

    let got: String = wrk.stdout(&mut cmd);
    let expected = "Name: name, Age: age\nName: John, Age: 30\nName: Jane, Age: 25";

    wrk.assert_success(&mut cmd);
    assert_eq!(got, expected);
}

#[test]
fn template_string() {
    let wrk = Workdir::new("template_string");
    wrk.create_from_string("data.csv", &data(true));

    let mut cmd = wrk.command("template");
    cmd.arg("--template")
        .arg("{{name}} is {{age}} years old\n\n")
        .arg("data.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = "John is 30 years old\nJane is 25 years old";

    wrk.assert_success(&mut cmd);
    assert_eq!(got, expected);
}

#[test]
fn template_custom_delimiter() {
    let wrk = Workdir::new("template_custom_delimiter");
    wrk.create_from_string(
        "data.csv",
        "name;age;city\nJohn;30;New York\nJane;25;Boston\n",
    );
    wrk.create_from_string("template.txt", "Name: {{ name }}, Age: {{age}}\n\n");

    let mut cmd = wrk.command("template");
    cmd.arg("--template-file")
        .arg("template.txt")
        .arg("data.csv")
        .args(["--delimiter", ";"]);

    let got: String = wrk.stdout(&mut cmd);
    let expected = "Name: John, Age: 30\nName: Jane, Age: 25";

    wrk.assert_success(&mut cmd);
    assert_eq!(got, expected);
}

#[test]
fn template_with_filters() {
    let wrk = Workdir::new("template_filters");
    wrk.create_from_string("data.csv", "name,amount\nJohn,1234.5678\nJane,9876.54321\n");
    wrk.create_from_string(
        "template.txt",
        "{{ name }}: ${{ amount | float | round(2) }}\n\n",
    );

    let mut cmd = wrk.command("template");
    cmd.arg("--template-file")
        .arg("template.txt")
        .arg("data.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = "John: $1234.57\nJane: $9876.54";

    wrk.assert_success(&mut cmd);
    assert_eq!(got, expected);
}

#[test]
fn template_with_conditionals() {
    let wrk = Workdir::new("template_conditionals");
    wrk.create_from_string("data.csv", "name,age\nJohn,17\nJane,21\n");
    wrk.create_from_string(
        "template.txt",
        "{{ name }} is {% if age | int >= 18 %}an adult{% else %}a minor{% endif %}\n\n",
    );

    let mut cmd = wrk.command("template");
    cmd.arg("--template-file")
        .arg("template.txt")
        .arg("data.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = "John is a minor\nJane is an adult";

    wrk.assert_success(&mut cmd);
    assert_eq!(got, expected);
}

#[test]
fn template_missing_field() {
    let wrk = Workdir::new("template_missing_field");
    wrk.create_from_string("data.csv", "name,age\nJohn,30\nJane,25\n");
    wrk.create_from_string(
        "template.txt",
        "{{ name }} ({{ missing_field | default('N/A') }})\n\n",
    );

    let mut cmd = wrk.command("template");
    cmd.arg("--template-file")
        .arg("template.txt")
        .arg("data.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = "John (N/A)\nJane (N/A)";

    wrk.assert_success(&mut cmd);
    assert_eq!(got, expected);
}

#[test]
fn template_empty_input() {
    let wrk = Workdir::new("template_empty");
    wrk.create_from_string("data.csv", "name,age\n");
    wrk.create_from_string("template.txt", "Hello {{name}}!\n");

    let mut cmd = wrk.command("template");
    cmd.arg("--template-file")
        .arg("template.txt")
        .arg("data.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = "";

    wrk.assert_success(&mut cmd);
    assert_eq!(got, expected);
}

#[test]
fn template_with_loops() {
    let wrk = Workdir::new("template_loops");
    wrk.create_from_string(
        "data.csv",
        "name,hobbies\nJohn,\"reading,gaming,cooking\"\nJane,\"hiking,painting\"\n",
    );
    wrk.create_from_string(
        "template.txt",
        "{{ name }}'s hobbies: {% for hobby in hobbies | split(',') %}{{ hobby | trim }}{% if not \
         loop.last %}, {% endif %}{% endfor %}\n\n",
    );

    let mut cmd = wrk.command("template");
    cmd.arg("--template-file")
        .arg("template.txt")
        .arg("data.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = "John's hobbies: reading, gaming, cooking, \nJane's hobbies: hiking, painting, ";

    wrk.assert_success(&mut cmd);
    assert_eq!(got, expected);
}

#[test]
fn template_error_invalid_syntax() {
    let wrk = Workdir::new("template_invalid_syntax");
    wrk.create_from_string("data.csv", "name,age\nJohn,30\n");
    wrk.create_from_string("template.txt", "{{ name } }}\n"); // Invalid syntax

    let mut cmd = wrk.command("template");
    cmd.arg("--template-file")
        .arg("template.txt")
        .arg("data.csv");

    wrk.assert_err(&mut cmd);
}

#[test]
fn template_error_missing_template() {
    let wrk = Workdir::new("template_missing_template");
    wrk.create_from_string("data.csv", "name,age\nJohn,30\n");

    let mut cmd = wrk.command("template");
    cmd.arg("--template-file")
        .arg("nonexistent.txt")
        .arg("data.csv");

    wrk.assert_err(&mut cmd);
}

#[test]
fn template_with_whitespace_control() {
    let wrk = Workdir::new("template_whitespace");
    wrk.create_from_string("data.csv", "name,items\nJohn,\"a,b,c\"\n");
    wrk.create_from_string(
        "template.txt",
        "Items:{%- for item in items | split(',') %}\n  - {{ item }}{%- if not loop.last %}{%- \
         endif %}{%- endfor %}\n\n",
    );

    let mut cmd = wrk.command("template");
    cmd.arg("--template-file")
        .arg("template.txt")
        .arg("data.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = "Items:\n  - a\n  - b\n  - c";

    wrk.assert_success(&mut cmd);
    assert_eq!(got, expected);
}

#[test]
fn template_output_file() {
    let wrk = Workdir::new("template_output");
    wrk.create_from_string("data.csv", &data(true));
    wrk.create_from_string("template.txt", "{{name}},{{city}}\n\n");

    let output_file = "output.txt";
    let mut cmd = wrk.command("template");
    cmd.arg("--template-file")
        .arg("template.txt")
        .arg("--output")
        .arg(output_file)
        .arg("data.csv");

    wrk.assert_success(&mut cmd);

    let got = wrk.read_to_string(output_file).unwrap();
    let expected = "John,New York\nJane,Boston\n";
    assert_eq!(got, expected);
}

#[test]
fn template_output_directory() {
    let wrk = Workdir::new("template_output_dir");
    wrk.create_from_string("data.csv", &data(true));
    wrk.create_from_string("template.txt", "Hello {{name}} from {{city}}!\n");

    let outdir = "output_dir";
    let mut cmd = wrk.command("template");
    cmd.arg("--template-file")
        .arg("template.txt")
        .arg("data.csv")
        .arg(outdir);

    wrk.assert_success(&mut cmd);

    // Check that files were created with default ROWNO naming
    let file1 = wrk.read_to_string(&format!("{outdir}/0/1.txt")).unwrap();
    let file2 = wrk.read_to_string(&format!("{outdir}/0/2.txt")).unwrap();

    assert_eq!(file1, "Hello John from New York!");
    assert_eq!(file2, "Hello Jane from Boston!");
}

#[test]
fn template_output_custom_filename() {
    let wrk = Workdir::new("template_custom_filename");
    wrk.create_from_string("data.csv", &data(true));
    wrk.create_from_string("template.txt", "Greetings from {{city}}!\n");

    let outdir = "custom_output";
    let mut cmd = wrk.command("template");
    cmd.arg("--template-file")
        .arg("template.txt")
        .arg("--outfilename")
        .arg("{{name}}_greeting-{{ QSV_ROWNO }}.txt")
        .arg("data.csv")
        .arg(outdir);

    wrk.assert_success(&mut cmd);

    // Check that files were created with custom naming
    let file1 = wrk
        .read_to_string(&format!("{outdir}/0/John_greeting-1.txt"))
        .unwrap();
    let file2 = wrk
        .read_to_string(&format!("{outdir}/0/Jane_greeting-2.txt"))
        .unwrap();

    assert_eq!(file1, "Greetings from New York!");
    assert_eq!(file2, "Greetings from Boston!");
}

#[test]
fn template_output_directory_no_headers() {
    let wrk = Workdir::new("template_output_dir_no_headers");
    wrk.create_from_string("data.csv", &data(false));
    wrk.create_from_string("template.txt", "Record: {{_c1}} - {{_c3}}\n");

    let outdir = "no_headers_output";
    let mut cmd = wrk.command("template");
    cmd.arg("--template-file")
        .arg("template.txt")
        .arg("--no-headers")
        .arg("data.csv")
        .arg(outdir);

    wrk.assert_success(&mut cmd);

    // Check files with row numbers
    let file1 = wrk.read_to_string(&format!("{outdir}/0/1.txt")).unwrap();
    let file2 = wrk.read_to_string(&format!("{outdir}/0/2.txt")).unwrap();

    assert_eq!(file1, "Record: John - New York");
    assert_eq!(file2, "Record: Jane - Boston");
}

#[test]
fn template_custom_filters() {
    let wrk = Workdir::new("template_custom_filters");
    wrk.create(
        "data.csv",
        vec![
            svec!["name", "amount", "bytes", "score", "active"],
            svec![
                "John",
                "1234567",
                "1048576",
                "3.14159265358979323846",
                "yes"
            ],
            svec!["Jane", "7654321.04", "1073741824", "2.71828", "no"],
        ],
    );

    // Test all custom filters
    wrk.create_from_string(
        "template.txt",
        "Name: {{ name|substr(0,2) }}\nAmount: {{ amount|human_count }}\nBytes: {{ \
         bytes|float|filesizeformat }} {{bytes|float|filesizeformat(true) }}\nScore (2 decimals): \
         {{ score|format_float(2) }}\nScore (rounded): {{ score|round_banker(4) }} \
         {{score|float|round(4) }}\nActive: {{ active|to_bool }}\nFloat with commas: {{ \
         amount|human_float_count }}\n\n",
    );

    let mut cmd = wrk.command("template");
    cmd.arg("--template-file")
        .arg("template.txt")
        .arg("data.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"Name: Jo
Amount: 1,234,567
Bytes: 1.0 MB 1.0 MiB
Score (2 decimals): 3.14
Score (rounded): 3.1416 3.1416
Active: true
Float with commas: 1,234,567
Name: Ja
Amount: <FILTER_ERROR>: "7654321.04" is not an integer.
Bytes: 1.1 GB 1.0 GiB
Score (2 decimals): 2.72
Score (rounded): 2.7183 2.7183
Active: false
Float with commas: 7,654,321.04"#;
    assert_eq!(got, expected);
}

#[test]
fn template_inline() {
    let wrk = Workdir::new("template_inline");
    wrk.create(
        "data.csv",
        vec![
            svec!["name", "age"],
            svec!["Alice", "25"],
            svec!["Bob", "30"],
        ],
    );

    let mut cmd = wrk.command("template");
    cmd.arg("--template")
        .arg("Hello {{name}}, you are {{age}} years old!\n\n")
        .arg("data.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = "\
Hello Alice, you are 25 years old!
Hello Bob, you are 30 years old!";
    assert_eq!(got, expected);
}

#[test]
fn template_conditional() {
    let wrk = Workdir::new("template_conditional");
    wrk.create(
        "data.csv",
        vec![
            svec!["name", "age"],
            svec!["Alice", "17"],
            svec!["Bob", "21"],
        ],
    );

    wrk.create_from_string(
        "template.txt",
        "{{ name }} is {% if age|round_banker(0)|int >= 18 %}an adult{% else %}a minor{% endif \
         %}.\n\n",
    );

    let mut cmd = wrk.command("template");
    cmd.arg("--template-file")
        .arg("template.txt")
        .arg("data.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = "\
Alice is a minor.
Bob is an adult.";
    assert_eq!(got, expected);
}

#[test]
fn template_render_error() {
    let wrk = Workdir::new("template_render_error");
    wrk.create(
        "data.csv",
        vec![
            svec!["name", "age"],
            svec!["Alice", "25"],
            svec!["Bob", "30"],
        ],
    );

    // Test invalid template syntax with default error message
    let mut cmd = wrk.command("template");
    cmd.arg("--template")
        .arg("Hello {{name}, invalid syntax!")
        .arg("data.csv");

    wrk.assert_err(&mut *&mut cmd);
    let got: String = wrk.output_stderr(&mut cmd);
    let expected = "syntax error: unexpected `}`, expected end of variable block (in template:1)\n";
    assert_eq!(got, expected);
}

#[test]
fn template_filter_error() {
    let wrk = Workdir::new("template_filter_error");
    wrk.create(
        "data.csv",
        vec![
            svec!["name", "amount"],
            svec!["Alice", "not_a_number"],
            svec!["Bob", "123.45"],
        ],
    );

    // Test filter error with default error message
    let mut cmd = wrk.command("template");
    cmd.arg("--template")
        .arg("{{name}}: {{amount|format_float(2)}}\n\n")
        .arg("data.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = "Alice: <FILTER_ERROR>\nBob: 123.45";
    assert_eq!(got, expected);

    // Test custom filter error message
    let mut cmd = wrk.command("template");
    cmd.arg("--template")
        .arg("{{name}}: {{amount|format_float(2)}}\n\n")
        .arg("--customfilter-error")
        .arg("INVALID NUMBER")
        .arg("data.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = "Alice: INVALID NUMBER\nBob: 123.45";
    assert_eq!(got, expected);

    // Test empty string as filter error
    let mut cmd = wrk.command("template");
    cmd.arg("--template")
        .arg("{{name}}: {{amount|format_float(2)}}\n\n")
        .arg("--customfilter-error")
        .arg("<empty string>")
        .arg("data.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = "Alice: \nBob: 123.45";
    assert_eq!(got, expected);
}

#[test]
fn template_contrib_filters() {
    let wrk = Workdir::new("template_contrib_filters");
    wrk.create(
        "data.csv",
        vec![
            svec!["text", "num", "datalist", "url"],
            svec![
                "hello WORLD",
                "12345.6789",
                "a,b,c",
                "https://example.com/path?q=test&lang=en"
            ],
            svec![
                "Testing 123",
                "-98765.4321",
                "1,2,3",
                "http://localhost:8080/api"
            ],
        ],
    );

    // Test various minijinja_contrib filters
    let mut cmd = wrk.command("template");
    cmd.arg("--template")
        .arg(concat!(
            // String filters
            "capitalize: {{text|capitalize}}\n",
            "title: {{text|title}}\n",
            "upper: {{text|upper}}\n",
            "lower: {{text|lower}}\n",
            // URL encode
            "urlencode: {{text|urlencode}}\n",
            // List filters
            "split: {{datalist|split(',')|join('|')}}\n",
            "first: {{datalist|split(',')|first}}\n",
            "last: {{datalist|split(',')|last}}\n",
            // Add newline between records
            "\n"
        ))
        .arg("data.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = concat!(
        "capitalize: Hello world\n",
        "title: Hello World\n",
        "upper: HELLO WORLD\n",
        "lower: hello world\n",
        "urlencode: hello%20WORLD\n",
        "split: a|b|c\n",
        "first: a\n",
        "last: c\n",
        "capitalize: Testing 123\n",
        "title: Testing 123\n",
        "upper: TESTING 123\n",
        "lower: testing 123\n",
        "urlencode: Testing%20123\n",
        "split: 1|2|3\n",
        "first: 1\n",
        "last: 3",
    );
    assert_eq!(got, expected);
}

#[test]
fn template_contrib_functions() {
    let wrk = Workdir::new("template_contrib_functions");
    wrk.create(
        "data.csv",
        vec![
            svec!["num_messages", "date_col"],
            svec!["1", "2023-06-24T16:37:22+00:00"],
            svec!["2", "1999-12-24T16:37:22+12:00"],
        ],
    );

    // Test various minijinja_contrib functions
    let mut cmd = wrk.command("template");
    cmd.arg("--template")
        .arg(concat!(
            "pluralize: You have {{ num_messages }} message{{ num_messages|int|pluralize }}\n",
            "now: {{now()|datetimeformat|length > 2}}\n", // Just verify we get a non-empty string
            "dtformat: {{date_col|datetimeformat(format=\"long\", tz=\"EST\")}}\n",
            "\n\n"
        ))
        .arg("data.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = concat!(
        "pluralize: You have 1 message\n",
        "now: true\n",
        "dtformat: June 24 2023 11:37:22\n",
        "\n",
        "pluralize: You have 2 messages\n",
        "now: true\n",
        "dtformat: December 23 1999 23:37:22",
    );
    assert_eq!(got, expected);
}

#[test]
fn template_pycompat_filters() {
    let wrk = Workdir::new("template_pycompat_filters");
    wrk.create(
        "data.csv",
        vec![
            svec!["text", "num", "mixed"],
            svec!["Hello World!", "123", "ABC123xyz  "],
            svec!["TESTING", "abc", "  Hello  "],
        ],
    );

    let mut cmd = wrk.command("template");
    cmd.arg("--template")
        .arg(concat!(
            // Test string methods from Python compatibility
            "isascii: {{text.isascii()}}\n",
            "isdigit: {{num.isdigit()}}\n",
            "startswith: {{text.startswith('Hello')}}\n",
            "isnumeric: {{num.isnumeric()}}\n",
            "isupper: {{text.isupper()}}\n",
            "replace: {{mixed.replace('ABC', 'XYZ')}}\n",
            "rfind: {{mixed.rfind('xyz')}}\n",
            "rstrip: {{mixed.rstrip()}}\n",
            "\n"
        ))
        .arg("data.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = concat!(
        "isascii: true\n",
        "isdigit: true\n",
        "startswith: true\n",
        "isnumeric: true\n",
        "isupper: false\n",
        "replace: XYZ123xyz  \n",
        "rfind: 6\n",
        "rstrip: ABC123xyz\n",
        "isascii: true\n",
        "isdigit: false\n",
        "startswith: false\n",
        "isnumeric: false\n",
        "isupper: true\n",
        "replace:   Hello  \n",
        "rfind: -1\n",
        "rstrip:   Hello",
    );
    assert_eq!(got, expected);
}

#[test]
fn template_custom_filters_error_handling() {
    let wrk = Workdir::new("template_custom_filters_error");
    wrk.create(
        "data.csv",
        vec![
            svec!["value", "number"],
            svec!["abc", "1234567.890123"],
            svec!["def", "not_a_number"],
            svec!["ghi", "7654321.04"],
        ],
    );

    let mut cmd = wrk.command("template");
    cmd.arg("--template")
        .arg(concat!(
            "VALUE: {{value}}\n",
            "  format_float: {{number|format_float(2)}}\n",
            "  human_count: {{number|human_count}}\n",
            "  human_float_count: {{number|human_float_count}}\n",
            "  round_banker: {{number|round_banker(3)}}\n",
            "\n"
        ))
        .arg("--customfilter-error")
        .arg("ERROR")
        .arg("data.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"VALUE: abc
  format_float: 1234567.89
  human_count: ERROR: "1234567.890123" is not an integer.
  human_float_count: 1,234,567.8901
  round_banker: 1234567.89
VALUE: def
  format_float: ERROR
  human_count: ERROR: "not_a_number" is not an integer.
  human_float_count: ERROR: "not_a_number" is not a float.
  round_banker: ERROR: "not_a_number" is not a float.
VALUE: ghi
  format_float: 7654321.04
  human_count: ERROR: "7654321.04" is not an integer.
  human_float_count: 7,654,321.04
  round_banker: 7654321.04"#;
    assert_eq!(got, expected);

    let mut cmd = wrk.command("template");
    cmd.arg("--template")
        .arg(concat!(
            "VALUE: {{value}}\n",
            "  format_float: {{number|format_float(2)}}\n",
            "  human_count: {{number|human_count}}\n",
            "  human_float_count: {{number|human_float_count}}\n",
            "  round_banker: {{number|round_banker(3)}}\n",
            "\n"
        ))
        .arg("--customfilter-error")
        .arg("<empty string>")
        .arg("data.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"VALUE: abc
  format_float: 1234567.89
  human_count: 
  human_float_count: 1,234,567.8901
  round_banker: 1234567.89
VALUE: def
  format_float: 
  human_count: 
  human_float_count: 
  round_banker: 
VALUE: ghi
  format_float: 7654321.04
  human_count: 
  human_float_count: 7,654,321.04
  round_banker: 7654321.04"#;
    assert_eq!(got, expected);
}

#[test]
fn template_to_bool_filter() {
    let wrk = Workdir::new("template_to_bool");
    wrk.create(
        "data.csv",
        vec![
            svec!["value"],
            svec!["true"],
            svec!["yes"],
            svec!["1"],
            svec!["0"],
            svec!["false"],
            svec!["no"],
            svec!["42.032"],
            svec!["0.0"],
            svec!["kinda true"],
            svec!["kinda false"],
            svec!["dunno"],
        ],
    );

    let mut cmd = wrk.command("template");
    cmd.arg("--template")
        .arg("{% if value|to_bool %}true{% else %}false{% endif %}\n\n")
        .arg("data.csv");

    wrk.assert_success(&mut cmd);

    let got: String = wrk.stdout(&mut cmd);
    let expected = r#"true
true
true
false
false
false
true
false
false
false
false"#;
    assert_eq!(got, expected);
}

#[test]
fn template_substr_filter() {
    let wrk = Workdir::new("template_substr");
    wrk.create(
        "data.csv",
        vec![svec!["text"], svec!["Hello World"], svec!["Testing 123"]],
    );

    let mut cmd = wrk.command("template");
    cmd.arg("--template")
        .arg(concat!(
            "start_only: {{text|substr(0)}}\n",
            "start_end: {{text|substr(0,5)}}\n",
            "middle: {{text|substr(6,11)}}\n",
            "invalid: {{text|substr(100)}}\n",
            "\n"
        ))
        .arg("--customfilter-error")
        .arg("ERROR")
        .arg("data.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = concat!(
        "start_only: Hello World\n",
        "start_end: Hello\n",
        "middle: World\n",
        "invalid: ERROR\n",
        "start_only: Testing 123\n",
        "start_end: Testi\n",
        "middle: g 123\n",
        "invalid: ERROR"
    );
    assert_eq!(got, expected);
}

#[test]
fn template_lookup_filter_simple() {
    let wrk = Workdir::new("template_lookup");

    // Create a lookup table CSV
    wrk.create(
        "lookup.csv",
        vec![
            svec!["id", "name", "description"],
            svec!["1", "apple", "A red fruit"],
            svec!["2", "banana", "A yellow fruit"],
            svec!["3", "orange", "A citrus fruit"],
        ],
    );

    // Create main data CSV
    wrk.create(
        "data.csv",
        vec![
            svec!["product_id", "quantity"],
            svec!["1", "5"],
            svec!["2", "3"],
            svec!["4", "1"], // Invalid ID to test error handling
        ],
    );

    let mut cmd = wrk.command("template");
    cmd.arg("--template")
        .arg(concat!(
            "{% set result = register_lookup('products', 'lookup.csv') %}",
            "{% if result %}",
            "{{product_id}}: {{product_id|lookup('products', 'name')}} - \
             {{product_id|lookup('products', 'description')}}\n",
            "{% else %}",
            "Error: Failed to register lookup table 'products' {{ result.err }} \n",
            "{% endif %}"
        ))
        .arg("--customfilter-error")
        .arg("<not found>")
        .arg("data.csv");

    wrk.assert_success(&mut cmd);

    let got: String = wrk.stdout(&mut cmd);
    let expected = concat!(
        "1: apple - A red fruit\n",
        "2: banana - A yellow fruit\n",
        r#"4: <not found> - lookup: "products-name" not found for: "4" - <not found> - lookup: "products-description" not found for: "4""#
    );
    assert_eq!(got, expected);
}

#[test]
fn template_lookup_filter_invalid_field() {
    let wrk = Workdir::new("template_lookup");

    // Create a lookup table CSV
    wrk.create(
        "lookup.csv",
        vec![
            svec!["id", "name", "description"],
            svec!["1", "apple", "A red fruit"],
            svec!["2", "banana", "A yellow fruit"],
            svec!["3", "orange", "A citrus fruit"],
        ],
    );

    // Create main data CSV
    wrk.create(
        "data.csv",
        vec![
            svec!["product_id", "quantity"],
            svec!["1", "5"],
            svec!["2", "3"],
            svec!["4", "1"], // Invalid ID to test error handling
        ],
    );

    let mut cmd = wrk.command("template");
    cmd.arg("--template")
        .arg(concat!(
            "{% set result = register_lookup('products', 'lookup.csv') %}",
            "{% if result %}",
            "{{product_id}}: {{product_id|lookup('products', 'name')}} - \
             {{product_id|lookup('products', 'non_existent_column')}}\n",
            "{% else %}",
            "Error: Failed to register lookup table 'products' {{ result.err }} \n",
            "{% endif %}"
        ))
        .arg("--customfilter-error")
        .arg("<not found>")
        .arg("data.csv");

    wrk.assert_success(&mut cmd);

    let got: String = wrk.stdout(&mut cmd);
    let expected = concat!(
        r#"1: apple - <not found> - lookup: "products-non_existent_column" not found for: "1"
"#,
        r#"2: banana - <not found> - lookup: "products-non_existent_column" not found for: "2"
"#,
        r#"4: <not found> - lookup: "products-name" not found for: "4" - <not found> - lookup: "products-non_existent_column" not found for: "4""#
    );
    assert_eq!(got, expected);
}

#[test]
fn template_lookup_filter_errors() {
    let wrk = Workdir::new("template_lookup_errors");

    wrk.create("data.csv", vec![svec!["id"], svec!["1"]]);

    // Test missing lookup key
    let mut cmd = wrk.command("template");
    cmd.arg("--template")
        .arg("{{id|lookup('', 'name')}}")
        .arg("data.csv");

    wrk.assert_success(&mut cmd);

    let got: String = wrk.stdout(&mut cmd);
    assert!(got.contains("RENDERING ERROR"));

    // Test missing field name
    let mut cmd = wrk.command("template");
    cmd.arg("--template")
        .arg("{{id|lookup('id', '')}}")
        .arg("data.csv");

    let got: String = wrk.stdout(&mut cmd);
    assert!(got.contains("RENDERING ERROR"));

    // Test unregistered lookup table
    let mut cmd = wrk.command("template");
    cmd.arg("--template")
        .arg("{{id|lookup('non_existent lookup', 'name')}}")
        .arg("data.csv");

    wrk.assert_success(&mut cmd);

    let got: String = wrk.stdout(&mut cmd);
    assert_eq!(
        got,
        "<FILTER_ERROR> - lookup: \"non_existent lookup-name\" not found for: \"1\""
    );
}

#[test]
fn template_lookup_register_errors() {
    let wrk = Workdir::new("template_lookup_register");

    wrk.create("data.csv", vec![svec!["id"], svec!["1"]]);

    // Test non-existent lookup file
    let mut cmd = wrk.command("template");
    cmd.arg("--template")
        .arg(concat!(
            "{% if register_lookup('test', 'nonexistent.csv') %}\n",
            "success\n",
            "{% else %}\n",
            "failed\n",
            "{% endif %}"
        ))
        .arg("data.csv");

    wrk.assert_err(&mut cmd);

    let got: String = wrk.output_stderr(&mut cmd);
    assert!(got.contains(
        r#"invalid operation: failed to load lookup table "test": failed to open nonexistent.csv:"#
    ));
}
#[test]
fn template_lookup_case_sensitivity() {
    let wrk = Workdir::new("template_lookup_case");

    // Create lookup table with mixed case values
    wrk.create(
        "lookup.csv",
        vec![
            svec!["code", "value"],
            svec!["ABC", "first"],
            svec!["def", "second"],
            svec!["GHI", "third"],
        ],
    );

    // Create input data with different cases
    wrk.create(
        "data.csv",
        vec![svec!["code"], svec!["abc"], svec!["DEF"], svec!["ghi"]],
    );

    // Test case-sensitive lookup (default)
    let mut cmd = wrk.command("template");
    cmd.arg("--template")
        .arg(concat!(
            "{% if register_lookup('codes', 'lookup.csv') %}",
            "{{code|lookup('codes', 'value')}}\n",
            "{% endif %}"
        ))
        .arg("data.csv");

    let got: String = wrk.stdout(&mut cmd);
    assert_eq!(
        got,
        r#"<FILTER_ERROR> - lookup: "codes-value" not found for: "abc"
<FILTER_ERROR> - lookup: "codes-value" not found for: "DEF"
<FILTER_ERROR> - lookup: "codes-value" not found for: "ghi""#
    );

    // Test case-insensitive lookup
    let mut cmd = wrk.command("template");
    cmd.arg("--template")
        .arg(concat!(
            "{% if register_lookup('codes', 'lookup.csv') %}",
            "{{code|lookup('codes', 'value', false)}}\n",
            "{% endif %}"
        ))
        .arg("data.csv");

    let got: String = wrk.stdout(&mut cmd);
    assert_eq!(got, "first\nsecond\nthird");
}

#[test]
fn template_humanfloat_filter() {
    let wrk = Workdir::new("template_humanfloat");
    wrk.create(
        "data.csv",
        vec![
            svec!["number"],
            svec!["1234.5678"],
            svec!["1000000"],
            svec!["123456789.3145679"],
            svec!["0.0001"],
            svec!["not_a_number"],
        ],
    );

    let mut cmd = wrk.command("template");
    cmd.arg("--template")
        .arg("{{number|human_float_count}}\n\n")
        .arg("--customfilter-error")
        .arg("ERROR")
        .arg("data.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = concat!(
        "1,234.5678\n",
        "1,000,000\n",
        "123,456,789.3146\n",
        "0.0001\n",
        "ERROR: \"not_a_number\" is not a float."
    );
    assert_eq!(got, expected);
}

#[test]
fn template_round_banker_filter() {
    let wrk = Workdir::new("template_round_banker");
    wrk.create(
        "data.csv",
        vec![
            svec!["number"],
            svec!["1.234567"],
            svec!["2.5467"],
            svec!["3.5"],
            svec!["not_a_number"],
        ],
    );

    let mut cmd = wrk.command("template");
    cmd.arg("--template")
        .arg(concat!(
            "2 places: {{number|round_banker(2)}}\n",
            "0 places: {{number|round_banker(0)}}\n\n"
        ))
        .arg("--customfilter-error")
        .arg("ERROR")
        .arg("data.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = concat!(
        "2 places: 1.23\n",
        "0 places: 1\n",
        "2 places: 2.55\n",
        "0 places: 3\n",
        "2 places: 3.5\n",
        "0 places: 4\n",
        "2 places: ERROR: \"not_a_number\" is not a float.\n",
        "0 places: ERROR: \"not_a_number\" is not a float."
    );
    assert_eq!(got, expected);
}

#[test]
fn template_globals_json() {
    let wrk = Workdir::new("template_globals");
    wrk.create(
        "data.csv",
        vec![
            svec!["name", "score"],
            svec!["Alice", "85"],
            svec!["Bob", "92"],
        ],
    );

    wrk.create_from_string(
        "globals.json",
        r#"{
            "passing_score": 90,
            "school_name": "Test Academy",
            "year": 2023
        }"#,
    );

    let mut cmd = wrk.command("template");
    cmd.arg("--template")
        .arg(concat!(
            "School: {{qsv_g.school_name}}\n",
            "Year: {{qsv_g.year}}\n",
            "Student: {{name}}\n",
            "Score: {{score}}\n",
            "Status: {% if score|int >= qsv_g.passing_score %}PASS{% else %}FAIL{% endif %}\n\n\n"
        ))
        .arg("--globals-json")
        .arg("globals.json")
        .arg("data.csv");

    let got: String = wrk.stdout(&mut cmd);
    let expected = concat!(
        "School: Test Academy\n",
        "Year: 2023\n",
        "Student: Alice\n",
        "Score: 85\n",
        "Status: FAIL\n\n",
        "School: Test Academy\n",
        "Year: 2023\n",
        "Student: Bob\n",
        "Score: 92\n",
        "Status: PASS"
    );
    assert_eq!(got, expected);
}

#[test]
fn template_globals_json_invalid() {
    let wrk = Workdir::new("template_globals_invalid");
    wrk.create("data.csv", vec![svec!["name"], svec!["test"]]);

    wrk.create_from_string(
        "invalid.json",
        r#"{
            "bad_json": "missing_comma"
            "another_field": true
        }"#,
    );

    let mut cmd = wrk.command("template");
    cmd.arg("--template")
        .arg("{{name}}\n")
        .arg("--globals-json")
        .arg("invalid.json")
        .arg("data.csv");

    let got: String = wrk.output_stderr(&mut cmd);
    assert!(got.contains("Failed to parse globals JSON file"));
}
