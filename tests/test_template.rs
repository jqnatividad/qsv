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

    let got = wrk.read_to_string(output_file);
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
    let file1 = wrk.read_to_string(&format!("{outdir}/1.txt"));
    let file2 = wrk.read_to_string(&format!("{outdir}/2.txt"));

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
    let file1 = wrk.read_to_string(&format!("{outdir}/John_greeting-1.txt"));
    let file2 = wrk.read_to_string(&format!("{outdir}/Jane_greeting-2.txt"));

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
    let file1 = wrk.read_to_string(&format!("{outdir}/1.txt"));
    let file2 = wrk.read_to_string(&format!("{outdir}/2.txt"));

    assert_eq!(file1, "Record: John - New York");
    assert_eq!(file2, "Record: Jane - Boston");
}
