static USAGE: &str = r#"
Provide input from the clipboard or save output to the clipboard.

Note when saving to clipboard on Windows, line breaks may be represented as \r\n (CRLF).
Meanwhile on Linux and macOS, they may be represented as \n (LF).

Examples:
Pipe into qsv stats using qsv clipboard and render it as a table:

  qsv clipboard | qsv stats | qsv table

If you want to save the output of a command to the clipboard,
pipe into qsv clipboard using the --save or -s flag:

  qsv clipboard | qsv stats | qsv clipboard -s

Usage:
    qsv clipboard [options]
    qsv clipboard --help

clip options:
    -s, --save             Save output to clipboard.
Common options:
    -h, --help             Display this message
"#;

use std::io::Read;

use arboard::Clipboard;
use serde::Deserialize;

use crate::{util, CliError, CliResult};

#[allow(dead_code)]
#[derive(Deserialize)]
struct Args {
    flag_save: bool,
}

impl From<arboard::Error> for CliError {
    fn from(err: arboard::Error) -> Self {
        match err {
            arboard::Error::ClipboardNotSupported => {
                CliError::Other("The clipboard may not be supported for the current environment.")
            },
            arboard::Error::ConversionFailure => CliError::Other(
                "The content that was about the be transferred to/from the clipboard could not be \
                 converted to the appropriate format.",
            ),
            arboard::Error::ClipboardOccupied => CliError::Other(
                "The clipboard was unaccessible when attempting to access/use it. It may have \
                 been held by another process/thread.",
            ),
            arboard::Error::ContentNotAvailable => CliError::Other(
                "The clipboard contents were not available in the requested format. This could \
                 either be due to the clipboard being empty or the clipboard contents having an \
                 incompatible format to the requested one.",
            ),
            arboard::Error::Unknown { description: _ } => {
                CliError::Other("An unknown error occurred while attempting to use the clipboard.")
            },
            _ => CliError::Other("An unexpected error occurred while using the clipboard."),
        }
    }
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;
    let mut clipboard = Clipboard::new()?;
    if args.flag_save {
        let mut buffer = String::new();
        std::io::stdin().read_to_string(&mut buffer)?;
        clipboard.set_text(buffer).unwrap();
    } else {
        print!("{}", clipboard.get_text().unwrap());
    }

    Ok(())
}
