static USAGE: &str = r#"
Splits the given CSV data into chunks.

Uses multithreading to go faster if the CSV has an index.

The files are written to the directory given with the name '{start}.csv',
where {start} is the index of the first record of the chunk (starting at 0).

Examples:
    qsv split outdir --size 100 --filename chunk_{}.csv input.csv

    qsv split outputdir -s 100 --filename chunk_{}.csv --pad 5 input.csv

    qsv split . -s 100 input.csv

    cat in.csv | qsv split mysplitoutput -s 1000

    qsv split outdir --chunks 10 input.csv

    qsv split splitoutdir -c 10 -j 4 input.csv

For more examples, see https://github.com/jqnatividad/qsv/blob/master/tests/test_split.rs.

Usage:
    qsv split [options] (--size <arg> | --chunks <arg>) <outdir> [<input>]
    qsv split --help

split arguments:
    <outdir>              The directory where the output files will be written.
                          If it does not exist, it will be created.
    <input>               The CSV file to read. If not given, input is read from
                          STDIN.

split options:
    -s, --size <arg>       The number of records to write into each chunk.
                           [default: 500]
    -c, --chunks <arg>     The number of chunks to split the data into.
                           This option is mutually exclusive with --size.
                           The number of rows in each chunk is determined by
                           the number of records in the CSV data and the number
                           of desired chunks. If the number of records is not evenly
                           divisible by the number of chunks, the last chunk will
                           have fewer records.

    -j, --jobs <arg>       The number of splitting jobs to run in parallel.
                           This only works when the given CSV data has
                           an index already created. Note that a file handle
                           is opened for each job.
                           When not set, the number of jobs is set to the
                           number of CPUs detected.
    --filename <filename>  A filename template to use when constructing
                           the names of the output files.  The string '{}'
                           will be replaced by a value based on the value
                           of the field, but sanitized for shell safety.
                           [default: {}.csv]
    --pad <arg>            The zero padding width that is used in the
                           generated filename.
                           [default: 0] 

Common options:
    -h, --help             Display this message
    -n, --no-headers       When set, the first row will NOT be interpreted
                           as column names. Otherwise, the first row will
                           appear in all chunks as the header row.
    -d, --delimiter <arg>  The field delimiter for reading CSV data.
                           Must be a single character. (default: ,)
    -Q, --quiet            Do not display an output summmary to stderr.
"#;

use std::{fs, io, path::Path};

use rayon::iter::{IntoParallelIterator, ParallelIterator};
use serde::Deserialize;

use crate::{
    config::{Config, Delimiter},
    index::Indexed,
    util::{self, FilenameTemplate},
    CliResult,
};

#[derive(Clone, Deserialize)]
struct Args {
    arg_input:       Option<String>,
    arg_outdir:      String,
    flag_size:       usize,
    flag_chunks:     Option<usize>,
    flag_jobs:       Option<usize>,
    flag_filename:   FilenameTemplate,
    flag_pad:        usize,
    flag_no_headers: bool,
    flag_delimiter:  Option<Delimiter>,
    flag_quiet:      bool,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;
    if args.flag_size == 0 {
        return fail_incorrectusage_clierror!("--size must be greater than 0.");
    }

    // check if outdir is set correctly
    if Path::new(&args.arg_outdir).is_file() && args.arg_input.is_none() {
        return fail_incorrectusage_clierror!("<outdir> is not specified or is a file.");
    }

    fs::create_dir_all(&args.arg_outdir)?;

    match args.rconfig().indexed()? {
        Some(idx) => args.parallel_split(&idx),
        None => args.sequential_split(),
    }
}

impl Args {
    fn sequential_split(&self) -> CliResult<()> {
        let rconfig = self.rconfig();
        let mut rdr = rconfig.reader()?;
        let headers = rdr.byte_headers()?.clone();

        #[allow(clippy::cast_precision_loss)]
        let chunk_size = if let Some(flag_chunks) = self.flag_chunks {
            let count = util::count_rows(&rconfig)?;
            let chunk = flag_chunks;
            if chunk == 0 {
                return fail_incorrectusage_clierror!("--chunk must be greater than 0.");
            }
            (count as f64 / chunk as f64).ceil() as usize
        } else {
            self.flag_size
        };

        let mut wtr = self.new_writer(&headers, 0, self.flag_pad)?;
        let mut i = 0;
        let mut nchunks: usize = 0;
        let mut row = csv::ByteRecord::new();
        while rdr.read_byte_record(&mut row)? {
            if i > 0 && i % chunk_size == 0 {
                wtr.flush()?;
                nchunks += 1;
                wtr = self.new_writer(&headers, i, self.flag_pad)?;
            }
            wtr.write_byte_record(&row)?;
            i += 1;
        }
        wtr.flush()?;

        if !self.flag_quiet {
            eprintln!(
                "Wrote {} chunks to '{}'. Rows/chunk: {} Num records: {}",
                nchunks + 1,
                Path::new(&self.arg_outdir).canonicalize()?.display(),
                chunk_size,
                i
            );
        }

        Ok(())
    }

    fn parallel_split(&self, idx: &Indexed<fs::File, fs::File>) -> CliResult<()> {
        let args = self.clone();
        let chunk_size;
        let idx_count = idx.count();

        #[allow(clippy::cast_precision_loss)]
        let nchunks = if let Some(flag_chunks) = args.flag_chunks {
            chunk_size = (idx_count as f64 / flag_chunks as f64).ceil() as usize;
            flag_chunks
        } else {
            chunk_size = args.flag_size;
            util::num_of_chunks(idx_count as usize, self.flag_size)
        };
        if nchunks == 1 {
            // there's only one chunk, we can just do a sequential split
            // which has less overhead and better error handling
            return self.sequential_split();
        }

        util::njobs(args.flag_jobs);

        // safety: we cannot use ? here because we're in a closure
        (0..nchunks).into_par_iter().for_each(|i| {
            let conf = args.rconfig();
            // safety: safe to unwrap because we know the file is indexed
            let mut idx = conf.indexed().unwrap().unwrap();
            // safety: the only way this can fail is if the file first row of the chunk
            // is not a valid CSV record, which is impossible because we're reading
            // from a file with a valid index
            let headers = idx.byte_headers().unwrap();

            let mut wtr = args
                // safety: the only way this can fail is if we cannot create a file
                .new_writer(headers, i * chunk_size, args.flag_pad)
                .unwrap();

            // safety: we know that there is more than one chunk, so we can safely
            // seek to the start of the chunk
            idx.seek((i * chunk_size) as u64).unwrap();
            for row in idx.byte_records().take(chunk_size) {
                let row = row.unwrap();
                wtr.write_byte_record(&row).unwrap();
            }
            // safety: safe to unwrap because we know the writer is a file
            // the only way this can fail is if we cannot write to the file
            wtr.flush().unwrap();
        });

        if !args.flag_quiet {
            eprintln!(
                "Wrote {} chunks to '{}'. Rows/chunk: {} Num records: {}",
                nchunks,
                Path::new(&self.arg_outdir).canonicalize()?.display(),
                chunk_size,
                idx_count
            );
        }

        Ok(())
    }

    fn new_writer(
        &self,
        headers: &csv::ByteRecord,
        start: usize,
        width: usize,
    ) -> CliResult<csv::Writer<Box<dyn io::Write + 'static>>> {
        let dir = Path::new(&self.arg_outdir);
        let path = dir.join(self.flag_filename.filename(&format!("{start:0>width$}")));
        let spath = Some(path.display().to_string());
        let mut wtr = Config::new(&spath).writer()?;
        if !self.rconfig().no_headers {
            wtr.write_record(headers)?;
        }
        Ok(wtr)
    }

    fn rconfig(&self) -> Config {
        Config::new(&self.arg_input)
            .delimiter(self.flag_delimiter)
            .no_headers(self.flag_no_headers)
    }
}
