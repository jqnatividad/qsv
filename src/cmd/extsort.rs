static USAGE: &str = r#"
Sort an arbitrarily large CSV/text file using a multithreaded external sort algorithm.

This command is not specific to CSV data, it sorts any text file on a 
line-by-line basis. If sorting a non-CSV file, be sure to set --no-headers, 
otherwise, the first line will not be included in the external sort.

Usage:
    qsv extsort [options] [<input>] [<output>]
    qsv extsort --help

External sort option:
    --memory-limit <arg>   The maximum amount of memory to buffer the on-disk hash table.
                           This is a percentage of total memory. [default: 10]
    -j, --jobs <arg>       The number of jobs to run in parallel.
                           When not set, the number of jobs is set to the
                           number of CPUs detected.

Common options:
    -h, --help             Display this message
    -n, --no-headers       When set, the first row will not be interpreted
                           as headers and will be sorted with the rest
                           of the rows. Otherwise, the first row will always
                           appear as the header row in the output.
"#;

use std::{
    fs,
    io::{self, stdin, stdout, BufRead, Write},
    path,
};

use ext_sort::{buffer::mem::MemoryLimitedBufferBuilder, ExternalSorter, ExternalSorterBuilder};
use serde::Deserialize;
use sysinfo::{System, SystemExt};

use crate::{config, util, CliResult};

#[derive(Deserialize)]
struct Args {
    arg_input:         Option<String>,
    arg_output:        Option<String>,
    flag_jobs:         Option<usize>,
    flag_memory_limit: Option<u8>,
    flag_no_headers:   bool,
}

const MEMORY_LIMITED_BUFFER: u64 = 100 * 1_000_000; // 100 MB
const RW_BUFFER_CAPACITY: usize = 1_000_000; // 1 MB

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;

    // memory buffer to use for external merge sort,
    // if we can detect the total memory, use 10% of it by default
    // and up to --memory-limit (capped at 50%),
    // otherwise, if we cannot detect the free memory use a default of 100 MB
    let mem_limited_buffer = if System::IS_SUPPORTED {
        let mut sys = System::new();
        sys.refresh_memory();
        (sys.total_memory() * 1000) / u8::min(args.flag_memory_limit.unwrap_or(10), 50) as u64
    } else {
        MEMORY_LIMITED_BUFFER
    };
    log::info!("{mem_limited_buffer} bytes used for in memory mergesort buffer...");

    let mut input_reader: Box<dyn BufRead> = match &args.arg_input {
        Some(input_path) => {
            if input_path.to_lowercase().ends_with(".sz") {
                return fail_clierror!(
                    "Input file cannot be a .sz file. Use 'qsv snappy decompress' first."
                );
            }
            let file = fs::File::open(input_path)?;
            Box::new(io::BufReader::with_capacity(
                config::DEFAULT_RDR_BUFFER_CAPACITY,
                file,
            ))
        }
        None => Box::new(io::BufReader::new(stdin().lock())),
    };

    let mut output_writer: Box<dyn Write> = match &args.arg_output {
        Some(output_path) => {
            if output_path.to_lowercase().ends_with(".sz") {
                return fail_clierror!(
                    "Output file cannot be a .sz file. Compress it after sorting with 'qsv snappy \
                     compress'."
                );
            }
            Box::new(io::BufWriter::with_capacity(
                RW_BUFFER_CAPACITY,
                fs::File::create(output_path)?,
            ))
        }
        None => Box::new(io::BufWriter::with_capacity(
            RW_BUFFER_CAPACITY,
            stdout().lock(),
        )),
    };

    let sorter: ExternalSorter<String, io::Error, MemoryLimitedBufferBuilder> =
        match ExternalSorterBuilder::new()
            .with_tmp_dir(path::Path::new("./"))
            .with_buffer(MemoryLimitedBufferBuilder::new(mem_limited_buffer))
            .with_rw_buf_size(RW_BUFFER_CAPACITY)
            .with_threads_number(util::njobs(args.flag_jobs))
            .build()
        {
            Ok(sorter) => sorter,
            Err(e) => {
                return fail_clierror!("cannot create external sorter: {e}");
            }
        };

    let mut header = String::new();
    if !args.flag_no_headers {
        input_reader.read_line(&mut header)?;
    }

    let Ok(sorted) = sorter.sort(input_reader.lines()) else {
                 return fail!("cannot do external sort");
             };

    if !header.is_empty() {
        output_writer.write_all(format!("{}\n", header.trim_end()).as_bytes())?;
    }

    for item in sorted.map(Result::unwrap) {
        output_writer.write_all(format!("{item}\n").as_bytes())?;
    }
    output_writer.flush()?;
    Ok(())
}

#[test]
fn test_mem_check() {
    // check to see if sysinfo return meminfo without segfaulting
    let mut sys = System::new();
    sys.refresh_memory();
    let mem10percent = (sys.total_memory() * 1000) / 10; // 10 percent of total memory
    assert!(mem10percent > 0);
}
