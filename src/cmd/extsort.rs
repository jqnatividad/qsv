static USAGE: &str = r#"
Sort an arbitrarily large CSV/text file using a multithreaded external sort algorithm.

This command has TWO modes of operation.

 * CSV MODE
   when --select is set, it sorts based on the given column/s. Requires an index.
   See `qsv select --help` for select syntax details.
 * LINE MODE
   when --select is NOT set, it sorts any input text file (not just CSVs) on a
   line-by-line basis. If sorting a non-CSV file, be sure to set --no-headers, 
   otherwise, the first line will not be included in the external sort.

Usage:
    qsv extsort [options] [<input>] [<output>]
    qsv extsort --help

External sort option:
    -s, --select <arg>     Select a subset of columns to sort (CSV MODE).
                           Note that the outputs will remain at the full width of the CSV.
                           If --select is NOT set, extsort will work in LINE MODE, sorting
                           the input as a text file on a line-by-line basis.
    -R, --reverse          Reverse order
    --memory-limit <arg>   The maximum amount of memory to buffer the external merge sort.
                           If less than 50, this is a percentage of total memory.
                           If more than 50, this is the memory in MB to allocate, capped
                           at 90 percent of total memory.
                           [default: 10]
    --tmp-dir <arg>        The directory to use for externally sorting file segments.
                           [default: ./]
    -j, --jobs <arg>       The number of jobs to run in parallel.
                           When not set, the number of jobs is set to the
                           number of CPUs detected.

Common options:
                           CSV MODE ONLY:
    -d, --delimiter <arg>  The field delimiter for reading CSV data.
                           Must be a single character. (default: ,)

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

use crate::{
    cmd::extdedup::calculate_memory_limit,
    config,
    config::{Config, Delimiter},
    select::SelectColumns,
    util, CliResult,
};

#[derive(Deserialize)]
struct Args {
    arg_input:         Option<String>,
    arg_output:        Option<String>,
    flag_select:       Option<SelectColumns>,
    flag_reverse:      bool,
    flag_delimiter:    Option<Delimiter>,
    flag_jobs:         Option<usize>,
    flag_memory_limit: Option<u64>,
    flag_tmp_dir:      Option<String>,
    flag_no_headers:   bool,
}

const RW_BUFFER_CAPACITY: usize = 1_000_000; // 1 MB

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;

    // check if tmp dir exists
    let tmp_dir = match &args.flag_tmp_dir {
        Some(tmp_dir) => {
            if !path::Path::new(tmp_dir).exists() {
                return fail_clierror!("tmp-dir '{tmp_dir}' does not exist");
            }
            tmp_dir.to_string()
        },
        None => "./".to_string(),
    };

    // Set the memory buffer size for the external merge sort based on --memory-limit
    // and system capabilities.
    let mem_limited_buffer_bytes = calculate_memory_limit(args.flag_memory_limit);
    log::info!("{mem_limited_buffer_bytes} bytes used for in memory mergesort buffer...");

    let sorter: ExternalSorter<String, io::Error, MemoryLimitedBufferBuilder> =
        match ExternalSorterBuilder::new()
            .with_tmp_dir(path::Path::new(&tmp_dir))
            .with_buffer(MemoryLimitedBufferBuilder::new(mem_limited_buffer_bytes))
            .with_rw_buf_size(RW_BUFFER_CAPACITY)
            .with_threads_number(util::njobs(args.flag_jobs))
            .build()
        {
            Ok(sorter) => sorter,
            Err(e) => {
                return fail_clierror!("cannot create external sorter: {e}");
            },
        };

    if args.flag_select.is_some() {
        sort_csv(&args, &tmp_dir, &sorter)
    } else {
        sort_lines(&args, &sorter)
    }
}

fn sort_csv(
    args: &Args,
    tmp_dir: &str,
    sorter: &ExternalSorter<String, io::Error, MemoryLimitedBufferBuilder>,
) -> Result<(), crate::clitypes::CliError> {
    let rconfig = Config::new(args.arg_input.as_ref())
        .delimiter(args.flag_delimiter)
        .no_headers(args.flag_no_headers)
        .select(args.flag_select.clone().unwrap());

    let mut idxfile = if let Ok(idx) = rconfig.indexed() {
        if idx.is_none() {
            return fail_incorrectusage_clierror!("extsort CSV mode requires an index");
        }
        idx.unwrap()
    } else {
        return fail_incorrectusage_clierror!("extsort CSV mode requires an index");
    };

    let mut input_rdr = rconfig.reader()?;

    let linewtr_tfile = tempfile::NamedTempFile::new_in(tmp_dir)?;
    let mut line_wtr = io::BufWriter::with_capacity(RW_BUFFER_CAPACITY, linewtr_tfile.as_file());

    let headers = input_rdr.byte_headers()?.clone();
    let sel = rconfig.selection(&headers)?;

    let mut sort_key = String::with_capacity(20);
    let mut utf8_string = String::with_capacity(20);
    let mut curr_row = csv::ByteRecord::new();

    let rowcount = idxfile.count();
    let width = rowcount.to_string().len();

    // first pass. get the selected columns, and the record position
    // then write them to a temp text file with the selected columns and the position
    // separated by "|". Pad the position with leading zeroes, so it will always be the same width
    for row in input_rdr.byte_records() {
        curr_row.clone_from(&row?);
        sort_key.clear();
        for field in sel.select(&curr_row) {
            if let Ok(s_utf8) = simdutf8::basic::from_utf8(field) {
                sort_key.push_str(s_utf8);
            } else {
                utf8_string.clear();
                utf8_string.push_str(&String::from_utf8_lossy(field));
                sort_key.push_str(&utf8_string);
            }
        }
        let idx_position = curr_row.position().unwrap();

        sort_key.push_str(&format!("|{:01$}", idx_position.line(), width));

        writeln!(line_wtr, "{sort_key}")?;
    }
    line_wtr.flush()?;

    let line_rdr = io::BufReader::with_capacity(
        RW_BUFFER_CAPACITY,
        std::fs::File::open(linewtr_tfile.path())?,
    );

    let reverse_flag = args.flag_reverse;
    let compare = |a: &String, b: &String| {
        if reverse_flag {
            a.cmp(b).reverse()
        } else {
            a.cmp(b)
        }
    };

    // Now sort the temp text file
    let sorted = match sorter.sort_by(line_rdr.lines(), compare) {
        Ok(sorted) => sorted,
        Err(e) => {
            return fail!(format!("cannot do external sort: {e:?}"));
        },
    };

    let sorted_tfile = tempfile::NamedTempFile::new_in(tmp_dir)?;
    let mut sorted_line_wtr =
        io::BufWriter::with_capacity(RW_BUFFER_CAPACITY, sorted_tfile.as_file());

    for item in sorted.map(Result::unwrap) {
        sorted_line_wtr.write_all(format!("{item}\n").as_bytes())?;
    }
    sorted_line_wtr.flush()?;
    // Delete the temporary file containing unsorted lines
    drop(line_wtr);
    linewtr_tfile.close()?;

    // now write the sorted CSV file by reading the sorted_line temp file
    // and extracting the position from each line
    // and then using that to seek the input file to retrieve the record
    // and then write the record to the final sorted CSV
    let sorted_lines = std::fs::File::open(sorted_tfile.path())?;
    let sorted_line_rdr = io::BufReader::with_capacity(RW_BUFFER_CAPACITY, sorted_lines);

    let mut sorted_csv_wtr = Config::new(args.arg_output.as_ref()).writer()?;

    let position_delta: u64 = if args.flag_no_headers {
        1
    } else {
        // Write the header row if --no-headers is false
        sorted_csv_wtr.write_byte_record(&headers)?;
        2
    };

    // amortize allocations
    let mut record_wrk = csv::ByteRecord::new();
    let mut line = String::new();

    for l in sorted_line_rdr.lines() {
        line.clone_from(&l?);
        let Ok(position) = atoi_simd::parse::<u64>((&line[line.len() - width..]).as_bytes()) else {
            return fail!("Failed to retrieve position: invalid integer");
        };

        idxfile.seek(position - position_delta)?;
        idxfile.read_byte_record(&mut record_wrk)?;
        sorted_csv_wtr.write_byte_record(&record_wrk)?;
    }
    sorted_csv_wtr.flush()?;
    drop(sorted_line_wtr);
    sorted_tfile.close()?;

    Ok(())
}

fn sort_lines(
    args: &Args,
    sorter: &ExternalSorter<String, io::Error, MemoryLimitedBufferBuilder>,
) -> Result<(), crate::clitypes::CliError> {
    let mut input_rdr: Box<dyn BufRead> = match &args.arg_input {
        Some(input_path) => {
            if input_path.to_lowercase().ends_with(".sz") {
                return fail_incorrectusage_clierror!(
                    "Input file cannot be a .sz file. Use 'qsv snappy decompress' first."
                );
            }
            let file = fs::File::open(input_path)?;
            Box::new(io::BufReader::with_capacity(
                config::DEFAULT_RDR_BUFFER_CAPACITY,
                file,
            ))
        },
        None => Box::new(io::BufReader::new(stdin().lock())),
    };

    let mut output_wtr: Box<dyn Write> = match &args.arg_output {
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
        },
        None => Box::new(io::BufWriter::with_capacity(
            RW_BUFFER_CAPACITY,
            stdout().lock(),
        )),
    };

    let mut header = String::new();
    if !args.flag_no_headers {
        input_rdr.read_line(&mut header)?;
    }

    let reverse_flag = args.flag_reverse;
    let compare = |a: &String, b: &String| {
        if reverse_flag {
            a.cmp(b).reverse()
        } else {
            a.cmp(b)
        }
    };

    let sorted = match sorter.sort_by(input_rdr.lines(), compare) {
        Ok(sorted) => sorted,
        Err(e) => {
            return fail!(format!("cannot do external sort: {e:?}"));
        },
    };

    if !header.is_empty() {
        output_wtr.write_all(format!("{}\n", header.trim_end()).as_bytes())?;
    }

    for item in sorted.map(Result::unwrap) {
        output_wtr.write_all(format!("{item}\n").as_bytes())?;
    }
    output_wtr.flush()?;
    Ok(())
}
