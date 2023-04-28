static USAGE: &str = r#"
Creates an index of the given CSV data, which can make other operations like
slicing, splitting and gathering statistics much faster.

Note that this does not accept CSV data on stdin. You must give a file
path. The index is created at 'path/to/input.csv.idx'. The index will be
automatically used by commands that can benefit from it. If the original CSV
data changes after the index is made, commands that try to use it will result
in an error (you have to regenerate the index before it can be used again).

However, if the environment variable QSV_AUTOINDEX is set, qsv will automatically
create an index when none is detected, and stale indices will be automatically
updated as well.

Usage:
    qsv index [options] <input>
    qsv index --help

index options:
    -o, --output <file>    Write index to <file> instead of <input>.idx.
                           Generally, this is not currently useful because
                           the only way to use an index is if it is specially
                           named <input>.idx.

Common options:
    -h, --help             Display this message
"#;

use std::{
    fs, io,
    path::{Path, PathBuf},
};

use csv_index::RandomAccessSimple;
use serde::Deserialize;

use crate::{
    config::{Config, DEFAULT_WTR_BUFFER_CAPACITY},
    util, CliResult,
};

#[derive(Deserialize)]
struct Args {
    arg_input:   String,
    flag_output: Option<String>,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;

    if args.arg_input.to_lowercase().ends_with(".sz") {
        return fail_clierror!("Cannot index a snappy file.");
    }

    let pidx = match args.flag_output {
        None => util::idx_path(Path::new(&args.arg_input)),
        Some(p) => PathBuf::from(&p),
    };

    let rconfig = Config::new(&Some(args.arg_input));
    let mut rdr = rconfig.reader_file()?;
    let mut wtr =
        io::BufWriter::with_capacity(DEFAULT_WTR_BUFFER_CAPACITY, fs::File::create(pidx)?);
    RandomAccessSimple::create(&mut rdr, &mut wtr)?;
    io::Write::flush(&mut wtr)?;

    Ok(())
}
