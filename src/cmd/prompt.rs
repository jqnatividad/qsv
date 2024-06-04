static USAGE: &str = r#"
Open a file dialog to pick a file as input.

For example to pick a single file as input to qsv stats using a file dialog, we can
pipe into qsv stats using qsv prompt:

qsv prompt | qsv stats

For examples, see https://github.com/jqnatividad/qsv/blob/master/tests/test_prompt.rs.

Usage:
    qsv prompt [options]
    qsv prompt --help

    Common options:
    -h, --help             Display this message
    -n, --no-headers       When set, does not include headers in output.
"#;

use rfd::FileDialog;

use crate::{config::Config, util, CliResult, Deserialize};

#[derive(Deserialize)]
struct Args {
    flag_no_headers: bool,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;
    if let Some(input_path) = FileDialog::new().set_directory("/").pick_file() {
        if let Some(input_path_str) = input_path.to_str() {
            let rconfig =
                Config::new(&Some(input_path_str.to_string())).no_headers(args.flag_no_headers);
            let mut rdr = rconfig.reader()?;
            let mut wtr = Config::new(&None).writer()?;
            if !rconfig.no_headers {
                rconfig.write_headers(&mut rdr, &mut wtr)?;
            }
            let mut record = csv::ByteRecord::new();

            while rdr.read_byte_record(&mut record)? {
                wtr.write_byte_record(&record)?;
            }

            wtr.flush()?;
        } else {
            return fail_clierror!(
                "Error while running qsv prompt. Perhaps the path to the file is not valid \
                 unicode?"
            );
        };
    } else {
        return fail_clierror!(
            "Error while running qsv prompt. Perhaps you did not select a file?"
        );
    };
    Ok(())
}
