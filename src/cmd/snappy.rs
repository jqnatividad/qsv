static USAGE: &str = r#"
Does streaming compression/decompression of the input using the Snappy format.
https://google.github.io/snappy/

It has three subcommands:
    compress:   Compress the input.
    decompress: Decompress the input.
    check:      Check if the input is a valid Snappy file. Returns exitcode 0 if valid,
                exitcode 1 otherwise.

Note that this command is not specific to CSV data, it can compress/decompress any file.

For examples, see https://github.com/jqnatividad/qsv/blob/master/tests/test_snappy.rs.

Usage:
    qsv snappy compress [options] [<input>]
    qsv snappy decompress [options] [<input>]
    qsv snappy check [<input>]
    qsv snappy --help

snappy arguments:
    <input>  The input file to compress/decompress. If not specified, stdin is used.

Common options:
    -h, --help           Display this message
    -o, --output <file>  Write output to <output> instead of stdout.
"#;

use std::{
    fs,
    io::{self, stdin, stdout, BufRead, Read, Write},
};

use serde::Deserialize;
use snap;

use crate::{config, util, CliError, CliResult};

#[derive(Deserialize)]
struct Args {
    arg_input:      Option<String>,
    flag_output:    Option<String>,
    cmd_compress:   bool,
    cmd_decompress: bool,
    cmd_check:      bool,
}

impl From<snap::Error> for CliError {
    fn from(err: snap::Error) -> CliError {
        CliError::Other(format!("Snap error: {err:?}"))
    }
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;

    let input_reader: Box<dyn BufRead> = match &args.arg_input {
        Some(input_path) => {
            let file = fs::File::open(input_path)?;
            Box::new(io::BufReader::with_capacity(
                config::DEFAULT_RDR_BUFFER_CAPACITY,
                file,
            ))
        }
        None => Box::new(io::BufReader::new(stdin().lock())),
    };

    let output_writer: Box<dyn Write> = match &args.flag_output {
        Some(output_path) => Box::new(io::BufWriter::with_capacity(
            config::DEFAULT_WTR_BUFFER_CAPACITY,
            fs::File::create(output_path)?,
        )),
        None => Box::new(io::BufWriter::with_capacity(
            config::DEFAULT_WTR_BUFFER_CAPACITY,
            stdout().lock(),
        )),
    };

    if args.cmd_compress {
        compress(input_reader, output_writer)?;
    } else if args.cmd_decompress {
        decompress(input_reader, output_writer)?;
    } else if args.cmd_check && !check(input_reader) {
        return fail_clierror!("Not a snappy file.");
    }

    Ok(())
}

// streaming snappy compression
fn compress<R: Read, W: Write>(mut src: R, dst: W) -> CliResult<()> {
    let mut dst = snap::write::FrameEncoder::new(dst);
    io::copy(&mut src, &mut dst)?;

    Ok(())
}

// streaming snappy decompression
fn decompress<R: Read, W: Write>(src: R, mut dst: W) -> CliResult<()> {
    let mut src = snap::read::FrameDecoder::new(src);
    io::copy(&mut src, &mut dst)?;

    Ok(())
}

// check if a file is a valid snappy file
fn check<R: Read>(src: R) -> bool {
    let src = snap::read::FrameDecoder::new(src);

    // read the first 50 or less bytes of a file
    // the snap decoder will return an error if the file is not a valid snappy file
    let mut buffer = Vec::with_capacity(51);
    if src.take(50).read_to_end(&mut buffer).is_err() {
        return false;
    }
    true
}
