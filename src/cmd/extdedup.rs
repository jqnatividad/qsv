static USAGE: &str = r#"
Remove duplicate rows from an arbitrarily large CSV/text file using a memory-mapped,
on-disk hash table.

Unlike the 'dedup' command, this command does not load the entire file into memory
to sort the CSV first before deduping it. 

This allows it to run in constant memory and the output will retain the input sort order.

Also, this command is not specific to CSV data, it deduplicates any text file on a 
line-by-line basis.

A duplicate count will be sent to <stderr>.

Usage:
    qsv extdedup [options] [<input>] [<output>]
    qsv extdedup --help

extdedup options:
    --no-output                Do not write deduplicated output to <output>.
                               Use this if you only want to know the duplicate count.
    -D, --dupes-output <file>  Write duplicates to <file>.
                               Note that the file will NOT be a valid CSV.
                               It is a list of duplicate lines, with the row number of the
                               duplicate separated by a tab from the duplicate line itself.
    -H, --human-readable       Comma separate duplicate count.
    --memory-limit <arg>       The maximum amount of memory to buffer the on-disk hash table.
                               This is a percentage of total memory. [default: 10]

Common options:
    -h, --help                 Display this message
    -Q, --quiet                Do not print duplicate count to stderr.
"#;

use std::{
    fs,
    io::{self, stdin, stdout, BufRead, Write},
};

use serde::Deserialize;
use sysinfo::{System, SystemExt};
use thousands::Separable;

use crate::{config, odhtcache, util, CliResult};

#[derive(Deserialize)]
struct Args {
    arg_input:           Option<String>,
    arg_output:          Option<String>,
    flag_no_output:      bool,
    flag_dupes_output:   Option<String>,
    flag_human_readable: bool,
    flag_memory_limit:   Option<u8>,
    flag_quiet:          bool,
}

const MEMORY_LIMITED_BUFFER: u64 = 100 * 1_000_000; // 100 MB

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;

    // memory buffer to use for on-disk hash table,
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
    log::info!("{mem_limited_buffer} bytes used for memory buffer for on-disk hash table...");

    let input_reader: Box<dyn BufRead> = match &args.arg_input {
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
        },
        None => Box::new(io::BufReader::new(stdin().lock())),
    };

    let mut output_writer: Box<dyn Write> = match &args.arg_output {
        Some(output_path) => Box::new(io::BufWriter::with_capacity(
            config::DEFAULT_WTR_BUFFER_CAPACITY,
            fs::File::create(output_path)?,
        )),
        None => Box::new(io::BufWriter::with_capacity(
            config::DEFAULT_WTR_BUFFER_CAPACITY,
            stdout().lock(),
        )),
    };

    let mut write_dupes = false;

    #[cfg(target_family = "unix")]
    let mut dupes_writer = if let Some(dupes_output) = args.flag_dupes_output {
        write_dupes = true;
        io::BufWriter::with_capacity(
            config::DEFAULT_WTR_BUFFER_CAPACITY,
            fs::File::create(dupes_output)?,
        )
    } else {
        io::BufWriter::with_capacity(
            config::DEFAULT_WTR_BUFFER_CAPACITY,
            fs::File::create("/dev/null")?,
        )
    };

    #[cfg(target_family = "windows")]
    let mut dupes_writer = if let Some(dupes_output) = args.flag_dupes_output {
        write_dupes = true;
        io::BufWriter::with_capacity(
            config::DEFAULT_WTR_BUFFER_CAPACITY,
            fs::File::create(dupes_output)?,
        )
    } else {
        io::BufWriter::with_capacity(
            config::DEFAULT_WTR_BUFFER_CAPACITY,
            fs::File::create("nul")?,
        )
    };

    let mut dedup_cache = odhtcache::ExtDedupCache::new(mem_limited_buffer);

    let mut dupes_count = 0_u64;
    let mut line_work = String::with_capacity(100);
    for (row_idx, line) in input_reader.lines().enumerate() {
        line_work.clone_from(&line?);
        if dedup_cache.contains(&line_work) {
            dupes_count += 1;
            if write_dupes {
                dupes_writer.write_all(format!("{row_idx}\t{line_work}\n").as_bytes())?;
            }
        } else {
            dedup_cache.insert(&line_work);
            if args.flag_no_output {
                continue;
            }
            output_writer.write_all(format!("{line_work}\n").as_bytes())?;
        }
    }

    dupes_writer.flush()?;
    output_writer.flush()?;

    if args.flag_quiet {
        return Ok(());
    }

    eprintln!(
        "{}",
        if args.flag_human_readable {
            dupes_count.separate_with_commas()
        } else {
            dupes_count.to_string()
        }
    );

    Ok(())
}

#[test]
fn test_extdedup_mem_check() {
    // check to see if sysinfo return meminfo without segfaulting
    let mut sys = System::new();
    sys.refresh_memory();
    let mem10percent = (sys.total_memory() * 1000) / 10; // 10 percent of total memory
    assert!(mem10percent > 0);
}
