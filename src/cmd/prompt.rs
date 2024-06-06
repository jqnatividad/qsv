static USAGE: &str = r#"
Open a file dialog to pick a file as input or save to an output file.

For example to pick a single file as input to qsv stats using a file dialog, we can
pipe into qsv stats using qsv prompt:

qsv prompt | qsv stats

If you want to save the output of a command to a file using a save file dialog, you
may pipe into qsv prompt but make sure you include the --fd-output (or -f) flag:

qsv prompt | qsv stats | qsv prompt --fd-output

Usage:
    qsv prompt [options]
    qsv prompt --help

prompt options:
    -m, --msg <arg>        The prompt message to display in the file dialog title.
                           When not using --fd-output, the default is "Select a File".
                           When using --fd-output, the default is "Save File As".
    -F, --filters <arg>    The filter to use for the file dialog. Set to "None" to
                           disable filters. Filters are comma-delimited file extensions.
                           [default: csv,tsv,tab,xls,xlsx,ods]
    -d, --workdir <dir>    The directory to start the file dialog in.
                           [default: .]
    -f, --fd-output        Write output to a file by using a save file dialog.
                           Used when piping into qsv prompt. Mutually exclusive with --output.

Common options:
    -h, --help             Display this message
    -n, --no-headers       When set, does not include headers in output.
    -o, --output <file>    Write output to <file> without showing a save dialog.
                           Mutually exclusive with --fd-output.
    -Q, --quiet            Do not print --fd-output message to stderr.

"#;

use std::path::PathBuf;

use rfd::FileDialog;

use crate::{config::Config, util, CliResult, Deserialize};

#[derive(Deserialize)]
#[allow(clippy::struct_field_names)]
struct Args {
    flag_msg:        Option<String>,
    flag_workdir:    PathBuf,
    flag_filters:    String,
    flag_fd_output:  bool,
    flag_no_headers: bool,
    flag_output:     Option<PathBuf>,
    flag_quiet:      bool,
}

const DEFAULT_INPUT_TITLE: &str = "Select a File";
const DEFAULT_OUTPUT_TITLE: &str = "Save File As";
const INPUT_DIALOG_DELAY_MS: u64 = 100;

enum PromptMode {
    Input,
    Output,
    FdOutput,
}

fn write_to_file(
    flag_no_headers: bool,
    input_path: Option<PathBuf>,
    output_path: Option<PathBuf>,
) -> CliResult<()> {
    let input_path = input_path.map(|pathbuf| pathbuf.to_string_lossy().into_owned());
    let output_path = output_path.map(|pathbuf| pathbuf.to_string_lossy().into_owned());

    let rconfig = Config::new(&input_path).no_headers(flag_no_headers);
    let mut rdr = rconfig.reader()?;
    let mut wtr = Config::new(&output_path).writer()?;

    if !rconfig.no_headers {
        rconfig.write_headers(&mut rdr, &mut wtr)?;
    }

    let mut record = csv::ByteRecord::new();
    while rdr.read_byte_record(&mut record)? {
        wtr.write_byte_record(&record)?;
    }

    Ok(wtr.flush()?)
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;

    if args.flag_fd_output && args.flag_output.is_some() {
        return fail_incorrectusage_clierror!(
            "Cannot use --fd-output (-f) and --output (-o) together, choose one."
        );
    }

    let prompt_mode = if !args.flag_fd_output && args.flag_output.is_none() {
        PromptMode::Input
    } else if args.flag_fd_output {
        PromptMode::FdOutput
    } else {
        PromptMode::Output
    };

    match prompt_mode {
        PromptMode::Input => {
            // just in case, sleep for a bit to allow the input dialog to pop over
            // the output dialog should there be a pipe into qsv prompt in the same pipeline.
            // e.g. cat file.csv | qsv prompt | qsv stats | qsv prompt --fd-output
            // this is needed as piped commands are actually launched in parallel and
            // not executed sequentially as commonly thought. So without this sleep,
            // the output dialog may pop over the input dialog.
            std::thread::sleep(std::time::Duration::from_millis(INPUT_DIALOG_DELAY_MS));

            let title = args
                .flag_msg
                .unwrap_or_else(|| DEFAULT_INPUT_TITLE.to_owned());
            let mut fd = FileDialog::new()
                .set_directory(args.flag_workdir)
                .set_title(title);

            if args.flag_filters.to_ascii_lowercase() != "none" {
                let ext_comma_delimited: Vec<&str> = args.flag_filters.split(',').collect();
                let ext_slice: &[&str] = &ext_comma_delimited;
                if !ext_slice.is_empty() {
                    fd = fd.add_filter("Filter".to_string(), ext_slice);
                }
            }

            if let Some(input_path) = fd.pick_file() {
                write_to_file(args.flag_no_headers, Some(input_path), None)?;
            } else {
                return fail_clierror!(
                    "Error while running qsv prompt. Perhaps you did not select a file for input?"
                );
            };
        },
        PromptMode::Output => {
            // If output then write to output and skip input path pick file
            write_to_file(args.flag_no_headers, None, args.flag_output)?;
        },
        PromptMode::FdOutput => {
            // If fd_output then write to output using save file
            let title = args
                .flag_msg
                .unwrap_or_else(|| DEFAULT_OUTPUT_TITLE.to_owned());
            let mut fd = FileDialog::new()
                .set_directory(args.flag_workdir)
                .set_title(title);

            #[cfg(target_os = "macos")]
            {
                fd = fd.set_can_create_directories(true);
            }

            if let Some(output_path) = fd.save_file() {
                write_to_file(args.flag_no_headers, None, Some(output_path.clone()))?;
                if !args.flag_quiet {
                    winfo!("Output saved to: {:?}", output_path);
                }
            } else {
                return fail_clierror!(
                    "Error while running qsv prompt. Perhaps you did not select a file for output?"
                );
            }
        },
    }

    Ok(())
}
