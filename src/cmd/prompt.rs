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
    --save-fname <file>    The filename to save the output as when using --fd-output.
                           [default: output.csv]

Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> without showing a save dialog.
                           Mutually exclusive with --fd-output.
    -Q, --quiet            Do not print --fd-output message to stderr.

"#;

use std::{
    fs,
    io::{self, Read, Write},
    path::PathBuf,
};

use rfd::FileDialog;

use crate::{util, CliResult, Deserialize};

#[derive(Deserialize)]
#[allow(clippy::struct_field_names)]
struct Args {
    flag_msg:        Option<String>,
    flag_workdir:    PathBuf,
    flag_filters:    String,
    flag_fd_output:  bool,
    flag_output:     Option<PathBuf>,
    flag_save_fname: String,
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

fn copy_file_to_output(input_path: Option<PathBuf>, output_path: Option<PathBuf>) -> CliResult<()> {
    let mut buffer: Vec<u8> = Vec::new();
    let input_filename = if let Some(input_path) = input_path {
        if output_path.is_none() {
            // we are copying from file to stdout
            // so read the file into buffer
            let mut file = std::fs::File::open(&input_path)?;
            file.read_to_end(&mut buffer)?;
        }
        input_path
    } else {
        // its stdin, copy to buffer
        io::stdin().read_to_end(&mut buffer)?;
        PathBuf::new()
    };

    if let Some(output_path) = output_path {
        if input_filename.exists() {
            // both input and output are files
            // use fs::copy to copy from input file to output file
            fs::copy(&input_filename, &output_path)?;
        } else {
            // we copied stdin into a buffer, write to output file
            let mut file = std::fs::File::create(&output_path)?;
            file.write_all(&buffer)?;
            file.flush()?;
        }
    } else if input_filename.exists() {
        // we copied from file, copy to stdout
        let mut input_file = std::fs::File::open(input_filename)?;
        let mut stdout = io::stdout();
        std::io::copy(&mut input_file, &mut stdout)?;
    } else {
        // we copied stdin into a buffer, write buffer to stdout
        io::stdout().write_all(&buffer)?;
        io::stdout().flush()?;
    }

    Ok(())
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
                copy_file_to_output(Some(input_path), None)?;
            } else {
                return fail_clierror!(
                    "Error while running qsv prompt. Perhaps you did not select a file for input?"
                );
            };
        },
        PromptMode::Output => {
            // If output then write to output and skip input path pick file
            copy_file_to_output(None, args.flag_output)?;
        },
        PromptMode::FdOutput => {
            // If fd_output then write to output using save file
            let title = args
                .flag_msg
                .unwrap_or_else(|| DEFAULT_OUTPUT_TITLE.to_owned());
            let mut fd = FileDialog::new()
                .set_directory(args.flag_workdir)
                .set_title(title)
                .set_file_name(args.flag_save_fname);

            #[cfg(target_os = "macos")]
            {
                fd = fd.set_can_create_directories(true);
            }

            if let Some(output_path) = fd.save_file() {
                copy_file_to_output(None, Some(output_path.clone()))?;
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
