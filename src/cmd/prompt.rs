static USAGE: &str = r#"
Open a file dialog to pick a file as input or save to an output file.

Examples:
Pick a single file as input to qsv stats using an INPUT file dialog,
pipe into qsv stats using qsv prompt:

  qsv prompt | qsv stats | qsv table

If you want to save the output of a command to a file using a save file OUTPUT dialog,
pipe into qsv prompt using the --fd-output flag:

  qsv prompt -m 'Pick a CSV file to summarize' | qsv stats -E | qsv prompt --fd-output

Prompt for a spreadsheet, and export to CSV using a save file dialog:

  qsv prompt -m 'Select a spreadsheet to export to CSV' -F xlsx,xls,ods | \
    qsv excel - | qsv prompt -m 'Save exported CSV to...' --fd-output

Usage:
    qsv prompt [options]
    qsv prompt --help

prompt options:
    -m, --msg <arg>        The prompt message to display in the file dialog title.
                           When not using --fd-output, the default is "Select a File".
                           When using --fd-output, the default is "Save File As".
    -F, --filters <arg>    The filter to use for the INPUT file dialog. Set to "None" to
                           disable filters. Filters are comma-delimited file extensions.
                           [default: csv,tsv,tab,xls,xlsx,ods]
    -d, --workdir <dir>    The directory to start the file dialog in.
                           [default: .]
    -f, --fd-output        Write output to a file by using a save file dialog.
                           Used when piping into qsv prompt. Mutually exclusive with --output.
    --save-fname <file>    The filename to save the output as when using --fd-output.
                           [default: output.csv]
    --base-delay-ms <ms>   The base delay in milliseconds to use when opening INPUT dialog.
                           This is to ensure that the INPUT dialog is shown before/over the
                           OUTPUT dialog when using the prompt command is used in both INPUT
                           and OUTPUT modes in a single pipeline.
                           [default: 200]

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
    flag_msg:           Option<String>,
    flag_workdir:       PathBuf,
    flag_filters:       String,
    flag_fd_output:     bool,
    flag_output:        Option<PathBuf>,
    flag_save_fname:    String,
    flag_base_delay_ms: u64,
    flag_quiet:         bool,
}

const DEFAULT_INPUT_TITLE: &str = "Select a File";
const DEFAULT_OUTPUT_TITLE: &str = "Save File As";

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

    let mut input_path = None;
    if !(args.flag_fd_output || args.flag_output.is_some()) {
        // prompt for input
        let title = args
            .flag_msg
            .clone()
            .unwrap_or_else(|| DEFAULT_INPUT_TITLE.to_owned());

        // piped commands are actually launched in parallel and not executed sequentially
        // from left to right as commonly thought. So we need to introduce a delay to ensure
        // that the input dialog is not opened before the save dialog.
        // e.g. cat file.csv | qsv prompt | qsv stats | qsv prompt --skip-input --fd-output
        // The delay ensures that the prompt for input is shown before the prompt for output.
        std::thread::sleep(std::time::Duration::from_millis(args.flag_base_delay_ms));

        let mut fd = FileDialog::new()
            .set_directory(args.flag_workdir.clone())
            .set_title(title.clone());

        if args.flag_filters.to_ascii_lowercase() != "none" {
            let ext_comma_delimited: Vec<&str> = args.flag_filters.split(',').collect();
            let ext_slice: &[&str] = &ext_comma_delimited;
            if !ext_slice.is_empty() {
                fd = fd.add_filter("Filter".to_string(), ext_slice);
            }
        }

        input_path = fd.pick_file();
        if input_path.is_none() {
            let err_msg = if title == DEFAULT_INPUT_TITLE {
                "Prompt error. Perhaps you did not select a file for input?".to_string()
            } else {
                format!(r#"Prompt error. Perhaps you did not select a file for input? "{title}""#)
            };
            return fail_clierror!("{err_msg}");
        }
    }

    if args.flag_fd_output {
        let title = args
            .flag_msg
            .unwrap_or_else(|| DEFAULT_OUTPUT_TITLE.to_owned());

        #[cfg(not(target_os = "macos"))]
        let fd = FileDialog::new()
            .set_directory(args.flag_workdir)
            .set_title(title)
            .set_file_name(args.flag_save_fname);

        #[cfg(target_os = "macos")]
        let fd = FileDialog::new()
            .set_directory(args.flag_workdir)
            .set_title(title)
            .set_file_name(args.flag_save_fname)
            .set_can_create_directories(true);

        // no delay here, we want the save dialog to appear immediately
        // the delay is only for input dialogs, so they pop over in reverse order
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
    } else if args.flag_output.is_some() {
        // If --output then write to output
        copy_file_to_output(None, args.flag_output)?;
    } else {
        // If not --output or --fd-output then write to stdout
        copy_file_to_output(input_path, None)?;
    }

    Ok(())
}
