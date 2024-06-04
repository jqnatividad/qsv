static USAGE: &str = r#"
Open a file dialog to pick a file as input or save to an output file.

For example to pick a single file as input to qsv stats using a file dialog, we can
pipe into qsv stats using qsv prompt:

qsv prompt | qsv stats

If you want to save the output of a command to a file using a save file dialog, you
may pipe into qsv prompt but make sure you include the --fd-output (or -f) flag:

qsv prompt | qsv stats | qsv prompt --fd-output

Note that the save file dialog (output) may be shown first before the input dialog.

For examples, see https://github.com/jqnatividad/qsv/blob/master/tests/test_prompt.rs.

Usage:
    qsv prompt [options]
    qsv prompt --help

prompt options:
    -f, --fd-output        Write output to a file by using a save file dialog.
                           Used when piping into qsv prompt.

Common options:
    -h, --help             Display this message
    -n, --no-headers       When set, does not include headers in output.
    -o, --output <file>    Write output to <file> without showing a save dialog.

"#;

use rfd::FileDialog;

use crate::{config::Config, util, CliResult, Deserialize};

#[derive(Deserialize)]
struct Args {
    flag_fd_output:  bool,
    flag_no_headers: bool,
    flag_output:     Option<String>,
}

fn write_to_file(
    args: &Args,
    input_path: Option<String>,
    output_path: Option<String>,
) -> CliResult<()> {
    let rconfig = Config::new(&input_path).no_headers(args.flag_no_headers);
    let mut rdr = rconfig.reader()?;
    let mut wtr = Config::new(&output_path).writer()?;
    if !rconfig.no_headers {
        rconfig.write_headers(&mut rdr, &mut wtr)?;
    }
    let mut record = csv::ByteRecord::new();

    while rdr.read_byte_record(&mut record)? {
        wtr.write_byte_record(&record)?;
    }

    wtr.flush()?;
    Ok(())
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;
    if args.flag_fd_output && args.flag_output.is_some() {
        return fail_incorrectusage_clierror!(
            "Cannot use --fd-output (-f) and --output (-o) together, choose one."
        );
    }
    if !args.flag_fd_output && args.flag_output.is_none() {
        if let Some(input_path) = FileDialog::new().set_directory("/").pick_file() {
            if let Some(input_path_str) = input_path.to_str() {
                write_to_file(&args, Some(input_path_str.to_string()), None)?;
            } else {
                return fail_clierror!(
                    "Error while running qsv prompt. Perhaps the path to the file is not valid \
                     unicode?"
                );
            };
        } else {
            return fail_clierror!(
                "Error while running qsv prompt. Perhaps you did not select a file for input?"
            );
        };
    }
    // If fd_output then write to output using save file
    else if args.flag_fd_output {
        if let Some(output_path) = FileDialog::new().set_directory("/").save_file() {
            if let Some(output_path_str) = output_path.to_str() {
                write_to_file(&args, None, Some(output_path_str.to_string()))?;
            } else {
                return fail_clierror!(
                    "Error while running qsv prompt. Perhaps the path to the file is not valid \
                     unicode?"
                );
            };
        } else {
            return fail_clierror!(
                "Error while running qsv prompt. Perhaps you did not select a file for output?"
            );
        }
    }
    // If output then write to output and skip input path pick file
    else {
        write_to_file(&args, None, args.flag_output.clone())?;
    }
    Ok(())
}
