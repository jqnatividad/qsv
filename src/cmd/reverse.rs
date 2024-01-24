static USAGE: &str = r#"
Reverses rows of CSV data.

Useful for cases when there is no column that can be used for sorting in reverse order,
or when keys are not unique and order of rows with the same key needs to be preserved.

Note that if the CSV is not indexed, this operation will require reading all of the
CSV data into memory

Usage:
    qsv reverse [options] [<input>]
    qsv reverse --help

Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
    -n, --no-headers       When set, the first row will not be interpreted
                           as headers. Namely, it will be reversed with the rest
                           of the rows. Otherwise, the first row will always
                           appear as the header row in the output.
    -d, --delimiter <arg>  The field delimiter for reading CSV data.
                           Must be a single character. (default: ,)
    --memcheck             Check if there is enough memory to load the entire
                           CSV into memory using CONSERVATIVE heuristics.
"#;

use serde::Deserialize;

use crate::{
    config::{Config, Delimiter},
    util, CliResult,
};

#[derive(Deserialize)]
struct Args {
    arg_input:       Option<String>,
    flag_output:     Option<String>,
    flag_no_headers: bool,
    flag_delimiter:  Option<Delimiter>,
    flag_memcheck:   bool,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;
    let rconfig = Config::new(&args.arg_input)
        .delimiter(args.flag_delimiter)
        .no_headers(args.flag_no_headers);

    // quadruple the buffer sizes as this is IO intensive
    // this is our brute force way of speeding up the process
    // as we can't really parallelize reversing a file
    let mut rdr = rconfig
        .clone()
        .read_buffer(rconfig.read_buffer * 4)
        .reader()?;
    let mut wtr = Config::new(&args.flag_output)
        .write_buffer(rconfig.write_buffer * 4)
        .writer()?;

    let Some(mut idx_file) = rconfig.indexed()? else {
        // we don't have an index, we need to read the entire file into memory
        // we're loading the entire file into memory, we need to check avail mem
        if let Some(ref path) = rconfig.path {
            util::mem_file_check(path, false, args.flag_memcheck)?;
        }

        let mut all = rdr.byte_records().collect::<Result<Vec<_>, _>>()?;
        all.reverse();

        rconfig.write_headers(&mut rdr, &mut wtr)?;
        for r in all {
            wtr.write_byte_record(&r)?;
        }
        return Ok(wtr.flush()?);
    };

    // we have an index, no need to check avail mem,
    // we're reading the file in reverse streaming
    rconfig.write_headers(&mut rdr, &mut wtr)?;
    let mut record = csv::ByteRecord::new();
    let mut pos = idx_file.count().saturating_sub(1);
    idx_file.seek(pos)?;
    while idx_file.read_byte_record(&mut record)? {
        wtr.write_byte_record(&record)?;
        pos -= 1;
        idx_file.seek(pos)?; // seek to next pos
    }

    Ok(wtr.flush()?)
}
