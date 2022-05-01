use std::fs;
use std::io::{self, prelude::*};
use std::path;

use crate::util;
use crate::CliResult;
use ext_sort::{buffer::mem::MemoryLimitedBufferBuilder, ExternalSorter, ExternalSorterBuilder};
use serde::Deserialize;

static USAGE: &str = "
Sort an arbitrarily large text file using a multi-threaded external sort algorithm.

This command does not work with <stdin>/<stdout>. Valid input, and output
files are expected.

Also, this command is not specific to CSV data, it sorts any text file on a 
line-by-line basis. If sorting a non-CSV file, be sure to set --no-headers, 
otherwise, the first line will not be included in the external sort.

Usage:
    qsv extsort [options] [<input>] [<output>]

External sort option:
    -j, --jobs <arg>       The number of jobs to run in parallel.
                           When not set, the number of jobs is set to the
                           number of CPUs detected.

Common options:
    -h, --help             Display this message
    -n, --no-headers       When set, the first row will not be interpreted
                           as headers. Namely, it will be sorted with the rest
                           of the rows. Otherwise, the first row will always
                           appear as the header row in the output.
";

#[derive(Deserialize)]
struct Args {
    arg_input: String,
    arg_output: String,
    flag_jobs: Option<usize>,
    flag_no_headers: bool,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;

    let mut input_reader = io::BufReader::new(fs::File::open(&args.arg_input)?);

    let sorter: ExternalSorter<String, io::Error, MemoryLimitedBufferBuilder> =
        ExternalSorterBuilder::new()
            .with_tmp_dir(path::Path::new("./"))
            .with_threads_number(util::njobs(self.flag_jobs))
            .build()
            .unwrap();

    let mut header = String::new();
    if !args.flag_no_headers {
        input_reader.read_line(&mut header)?;
    }

    let sorted = sorter.sort(input_reader.lines()).unwrap();

    let mut output_writer = io::BufWriter::new(fs::File::create(&args.arg_output)?);
    if !header.is_empty() {
        output_writer.write_all(format!("{}\n", header.trim_end()).as_bytes())?;
    }

    for item in sorted.map(Result::unwrap) {
        output_writer.write_all(format!("{item}\n").as_bytes())?;
    }
    output_writer.flush().unwrap();
    Ok(())
}
