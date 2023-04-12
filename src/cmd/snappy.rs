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
    qsv snappy check [<input>]
    qsv snappy validate [<input>]
    qsv snappy --help

snappy arguments:
    <input>  The input file to compress/decompress. If not specified, stdin is used.

options:
    -h, --help           Display this message
    -o, --output <file>  Write output to <output> instead of stdout.
    -j, --jobs <arg>     The number of jobs to run in parallel when compressing.
                         When not set, the number of jobs is set to 8 or the number
                         of CPUs detected, whichever is smaller.
"#;

use std::{
    env, fs,
    io::{self, stdin, BufRead, Read, Write},
};

use gzp::{par::compress::ParCompressBuilder, snap::Snap};
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
    } else if args.cmd_decompress {
        decompress(input_reader, output_writer)?;
    } else if args.cmd_validate {
        if validate(input_reader) {
            eprintln!("Valid snappy file.");
        } else {
            return fail_clierror!("Not a valid snappy file.");
        }
    } else if args.cmd_check {
        if check(input_reader) {
            eprintln!("Snappy file.");
        } else {
            return fail_clierror!("Not a snappy file.");
        }
    }

    Ok(())
}

// multi-threaded streaming snappy compression
fn compress<R: Read, W: Write + Send + 'static>(mut src: R, dst: W, jobs: usize) -> CliResult<()> {
    let rdr_capacitys = env::var("QSV_RDR_BUFFER_CAPACITY")
        .unwrap_or_else(|_| config::DEFAULT_RDR_BUFFER_CAPACITY.to_string());
    let mut buffer_size: usize = rdr_capacitys
        .parse()
        .unwrap_or(config::DEFAULT_RDR_BUFFER_CAPACITY);

    // the buffer size must be at least 32768 bytes, otherwise, ParCompressBuilder panics
    // as it expects the buffer size to be >= its DICT_SIZE which is 32768
    if buffer_size < 32768 {
        buffer_size = 32768;
    };

    let mut writer = ParCompressBuilder::<Snap>::new()
        .num_threads(jobs)
        .unwrap()
        .buffer_size(buffer_size)
        .unwrap()
        .pin_threads(Some(0))
        .from_writer(dst);
    io::copy(&mut src, &mut writer)?;

    Ok(())
}

// streaming, single-threaded snappy decompression
fn decompress<R: Read, W: Write>(src: R, mut dst: W) -> CliResult<()> {
    let mut src = snap::read::FrameDecoder::new(src);
    io::copy(&mut src, &mut dst)?;

    Ok(())
}

// check if a file is a snappy file
// note that we only read the first 50 bytes of the file
// and do not check the entire file for validity
fn check<R: Read>(src: R) -> bool {
    let src = snap::read::FrameDecoder::new(src);

    // read the first 50 or less bytes of a file
    // the snap decoder will return an error if the file is not a valid snappy file
    let mut buffer = Vec::with_capacity(51);
    src.take(50).read_to_end(&mut buffer).is_ok()
}

// validate an entire snappy file by decompressing it
// to sink (i.e. /dev/null). This is useful for checking
// if a snappy file is corrupted.
// Note that this is more expensive than check() as it has to
// decompress the entire file.
fn validate<R: Read>(src: R) -> bool {
    let mut src = snap::read::FrameDecoder::new(src);
    let mut sink = io::sink();
    io::copy(&mut src, &mut sink).is_ok()
}
