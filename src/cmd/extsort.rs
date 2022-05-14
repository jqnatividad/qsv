use std::fs;
use std::io::{self, prelude::*};
use std::path;

use crate::util;
use crate::CliResult;
use ext_sort::{buffer::mem::MemoryLimitedBufferBuilder, ExternalSorter, ExternalSorterBuilder};
use serde::Deserialize;
use sysinfo::{System, SystemExt};

static USAGE: &str = "
Sort an arbitrarily large CSV/text file using a multithreaded external sort algorithm.

This command does not work with <stdin>/<stdout>. Valid input, and output
files are expected.

Also, this command is not specific to CSV data, it sorts any text file on a 
line-by-line basis. If sorting a non-CSV file, be sure to set --no-headers, 
otherwise, the first line will not be included in the external sort.

Usage:
    qsv extsort [options] <input> <output>

External sort option:
    -j, --jobs <arg>       The number of jobs to run in parallel.
                           When not set, the number of jobs is set to the
                           number of CPUs detected.

Common options:
    -h, --help             Display this message
    -n, --no-headers       When set, the first row will not be interpreted
                           as headers and will be sorted with the rest
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

const MEMORY_LIMITED_BUFFER: u64 = 100 * 1_000_000; // 100 MB
const RW_BUFFER_CAPACITY: usize = 1_000_000; // 1 MB

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;

    // buffer to use for sorting in memory,
    // use 10% of total memory if we can detect it, otherwise
    // set it to MEMORY_LIMITED_BUFFER
    let mem_limited_buffer = if System::IS_SUPPORTED {
        let mut sys = System::new_all();
        sys.refresh_memory();
        (sys.total_memory() * 1000) / 10 // 10 percent of total memory
    } else {
        MEMORY_LIMITED_BUFFER
    };
    log::info!("{mem_limited_buffer} bytes used for in memory mergesort buffer...");

    let mut input_reader = io::BufReader::new(fs::File::open(&args.arg_input)?);

    let sorter: ExternalSorter<String, io::Error, MemoryLimitedBufferBuilder> =
        ExternalSorterBuilder::new()
            .with_tmp_dir(path::Path::new("./"))
            .with_buffer(MemoryLimitedBufferBuilder::new(mem_limited_buffer))
            .with_rw_buf_size(RW_BUFFER_CAPACITY)
            .with_threads_number(util::njobs(args.flag_jobs))
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
    output_writer.flush()?;
    Ok(())
}
