static USAGE: &str = r#"
Does streaming compression/decompression of the input using the Snappy format.
https://google.github.io/snappy/

It has four subcommands:
    compress:   Compress the input (multi-threaded).
    decompress: Decompress the input.
    check:      Check if the input is a Snappy file. Returns exitcode 0 if the
                first 50 bytes of the input are valid Snappy data.
                exitcode 1 otherwise.
    validate:   Check if the input is a valid Snappy file. Returns exitcode 0 if valid,
                exitcode 1 otherwise.
                Sends to stderr the compressed size, uncompressed size, the compression ratio,
                and the storage savings (in percent) if the input is valid.

Note that most qsv commands will automatically decompress Snappy files if the
input file has an ".sz" extension. It will also automatically compress the output
file (though only single-threaded) if the --output file has an ".sz" extension.

This command's multi-threaded compression is 5-6x faster than qsv's automatic 
single-threaded compression.

Also, this command is not specific to CSV data, it can compress/decompress ANY file.

For examples, see https://github.com/jqnatividad/qsv/blob/master/tests/test_snappy.rs.

Usage:
    qsv snappy compress [options] [<input>]
    qsv snappy decompress [options] [<input>]
    qsv snappy check [options] [<input>]
    qsv snappy validate [options] [<input>]
    qsv snappy --help

snappy arguments:
    <input>  The input file to compress/decompress. If not specified, stdin is used.

options:
    -h, --help           Display this message
    -o, --output <file>  Write output to <output> instead of stdout.
    -j, --jobs <arg>     The number of jobs to run in parallel when compressing.
                         When not set, the number of jobs is set to 8 or the number
                         of CPUs detected, whichever is smaller.
    -Q, --quiet          Supress messages to stderr.

"#;

use std::{
    fs,
    io::{self, stdin, BufRead, Read, Write},
};

use gzp::{par::compress::ParCompressBuilder, snap::Snap, ZWriter};
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
    cmd_validate:   bool,
    flag_jobs:      Option<usize>,
    flag_quiet:     bool,
}

impl From<snap::Error> for CliError {
    fn from(err: snap::Error) -> CliError {
        CliError::Other(format!("Snap error: {err:?}"))
    }
}

impl From<gzp::GzpError> for CliError {
    fn from(err: gzp::GzpError) -> CliError {
        CliError::Other(format!("Gzp error: {err:?}"))
    }
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;

    let input_bytes;

    let input_reader: Box<dyn BufRead> = match &args.arg_input {
        Some(path) => {
            let file = fs::File::open(path)?;
            input_bytes = file.metadata()?.len();
            Box::new(io::BufReader::with_capacity(
                config::DEFAULT_RDR_BUFFER_CAPACITY,
                file,
            ))
        }
        None => {
            input_bytes = 0;
            Box::new(io::BufReader::new(stdin().lock()))
        }
    };

    let output_writer: Box<dyn Write + Send + 'static> = match &args.flag_output {
        Some(output_path) => Box::new(io::BufWriter::with_capacity(
            config::DEFAULT_WTR_BUFFER_CAPACITY,
            fs::File::create(output_path)?,
        )),
        None => Box::new(io::BufWriter::with_capacity(
            config::DEFAULT_WTR_BUFFER_CAPACITY,
            io::stdout(),
        )),
    };

    if args.cmd_compress {
        let mut jobs = util::njobs(args.flag_jobs);
        if jobs > 1 {
            jobs -= 1; // save one thread for other tasks
        }

        compress(input_reader, output_writer, jobs)?;
        let compressed_bytes = if let Some(path) = &args.flag_output {
            fs::metadata(path)?.len()
        } else {
            0
        };
        if !args.flag_quiet && compressed_bytes > 0 {
            let compression_ratio = input_bytes as f64 / compressed_bytes as f64;
            winfo!(
                "Compression successful. Compressed bytes: {}, Decompressed bytes: {}, \
                 Compression ratio: {:.3}:1, Space savings: {} - {:.2}%",
                indicatif::HumanBytes(compressed_bytes),
                indicatif::HumanBytes(input_bytes),
                compression_ratio,
                indicatif::HumanBytes(
                    input_bytes
                        .checked_sub(compressed_bytes)
                        .unwrap_or_default()
                ),
                (1.0 - (compressed_bytes as f64 / input_bytes as f64)) * 100.0
            );
        }
    } else if args.cmd_decompress {
        let decompressed_bytes = decompress(input_reader, output_writer)?;
        if !args.flag_quiet {
            let compression_ratio = decompressed_bytes as f64 / input_bytes as f64;
            winfo!(
                "Decompression successful. Compressed bytes: {}, Decompressed bytes: {}, \
                 Compression ratio: {:.3}:1, Space savings: {} - {:.2}%",
                indicatif::HumanBytes(input_bytes),
                indicatif::HumanBytes(decompressed_bytes),
                compression_ratio,
                indicatif::HumanBytes(
                    decompressed_bytes
                        .checked_sub(input_bytes)
                        .unwrap_or_default()
                ),
                (1.0 - (input_bytes as f64 / decompressed_bytes as f64)) * 100.0
            );
        }
    } else if args.cmd_validate {
        if args.arg_input.is_none() {
            return fail_clierror!("stdin is not supported by the snappy validate subcommand.");
        }
        let Ok(decompressed_bytes) = validate(input_reader) else {
            return fail_clierror!("Not a valid snappy file.");
        };
        if !args.flag_quiet {
            let compression_ratio = decompressed_bytes as f64 / input_bytes as f64;
            winfo!(
                "Valid snappy file. Compressed bytes: {}, Decompressed bytes: {}, Compression \
                 ratio: {:.3}:1, Space savings: {} - {:.2}%",
                indicatif::HumanBytes(input_bytes),
                indicatif::HumanBytes(decompressed_bytes),
                compression_ratio,
                indicatif::HumanBytes(
                    decompressed_bytes
                        .checked_sub(input_bytes)
                        .unwrap_or_default()
                ),
                (1.0 - (input_bytes as f64 / decompressed_bytes as f64)) * 100.0
            );
        }
    } else if args.cmd_check {
        let check_ok = check(input_reader);
        if args.flag_quiet {
            if check_ok {
                return Ok(());
            }
            return fail!("Not a snappy file.");
        } else if check_ok {
            winfo!("Snappy file.");
        } else {
            return fail!("Not a snappy file.");
        }
    }

    Ok(())
}

// multi-threaded streaming snappy compression
fn compress<R: Read, W: Write + Send + 'static>(mut src: R, dst: W, jobs: usize) -> CliResult<()> {
    let mut writer = ParCompressBuilder::<Snap>::new()
        .num_threads(jobs)
        .unwrap()
        .buffer_size(gzp::BUFSIZE)
        .unwrap()
        .pin_threads(Some(0))
        .from_writer(dst);
    io::copy(&mut src, &mut writer)?;
    writer.finish()?;

    Ok(())
}

// single-threaded streaming snappy decompression
fn decompress<R: Read, W: Write>(src: R, mut dst: W) -> CliResult<u64> {
    let mut src = snap::read::FrameDecoder::new(src);
    let decompressed_bytes = io::copy(&mut src, &mut dst)?;

    Ok(decompressed_bytes)
}

// check if a file is a snappy file
// note that we only read the first 50 bytes of the file
// and do not check the entire file for validity
fn check<R: Read>(src: R) -> bool {
    let src = snap::read::FrameDecoder::new(src);

    // read the first 50 or less bytes of a file
    // the snap decoder will return an error if the file is not a valid snappy file
    let mut buffer = Vec::with_capacity(50);
    src.take(50).read_to_end(&mut buffer).is_ok()
}

// validate an entire snappy file by decompressing it to sink (i.e. /dev/null). This is useful for
// checking if a snappy file is corrupted.
// Note that this is more expensive than check() as it has to decompress the entire file.
fn validate<R: Read>(src: R) -> CliResult<u64> {
    let mut src = snap::read::FrameDecoder::new(src);
    let mut sink = io::sink();
    match io::copy(&mut src, &mut sink) {
        Ok(decompressed_bytes) => Ok(decompressed_bytes),
        Err(err) => fail_clierror!("Error validating snappy file: {err:?}"),
    }
}
