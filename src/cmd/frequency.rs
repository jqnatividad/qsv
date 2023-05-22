static USAGE: &str = r#"
Compute a frequency table on CSV data.

The frequency table is formatted as CSV data:

    field,value,count

By default, there is a row for the N most frequent values for each field in the
data. The order and number of values can be tweaked with --asc and --limit,
respectively.

Since this computes an exact frequency table, memory proportional to the
cardinality of each column is required.

For examples, see https://github.com/jqnatividad/qsv/blob/master/tests/test_frequency.rs.

Usage:
    qsv frequency [options] [<input>]
    qsv frequency --help

frequency options:
    -s, --select <arg>     Select a subset of columns to compute frequencies
                           for. See 'qsv select --help' for the format
                           details. This is provided here because piping 'qsv
                           select' into 'qsv frequency' will disable the use
                           of indexing.
    -l, --limit <arg>      Limit the frequency table to the N most common
                           items. Set to '0' to disable a limit.
                           [default: 10]
    -a, --asc              Sort the frequency tables in ascending order by
                           count. The default is descending order.
    --no-nulls             Don't include NULLs in the frequency table.
    -j, --jobs <arg>       The number of jobs to run in parallel.
                           This works better when the given CSV data has
                           an index already created. Note that a file handle
                           is opened for each job.
                           When not set, the number of jobs is set to the
                           number of CPUs detected.

Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
    -n, --no-headers       When set, the first row will NOT be included
                           in the frequency table. Additionally, the 'field'
                           column will be 1-based indices instead of header
                           names.
    -d, --delimiter <arg>  The field delimiter for reading CSV data.
                           Must be a single character. (default: ,)
    --memcheck             Check if there is enough memory to load the entire
                           CSV into memory using CONSERVATIVE heuristics.
"#;

use std::{fs, io};

use serde::Deserialize;
use stats::{merge_all, Frequencies};
use threadpool::ThreadPool;

use crate::{
    config::{Config, Delimiter},
    index::Indexed,
    select::{SelectColumns, Selection},
    util,
    util::ByteString,
    CliResult,
};

#[derive(Clone, Deserialize)]
pub struct Args {
    pub arg_input:       Option<String>,
    pub flag_select:     SelectColumns,
    pub flag_limit:      usize,
    pub flag_asc:        bool,
    pub flag_no_nulls:   bool,
    pub flag_jobs:       Option<usize>,
    pub flag_output:     Option<String>,
    pub flag_no_headers: bool,
    pub flag_delimiter:  Option<Delimiter>,
    pub flag_memcheck:   bool,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;
    let rconfig = args.rconfig();

    // we're loading the entire file into memory, we need to check avail mem
    if let Some(path) = rconfig.path.clone() {
        util::mem_file_check(&path, false, args.flag_memcheck)?;
    }

    let mut wtr = Config::new(&args.flag_output).writer()?;
    let (headers, tables) = match args.rconfig().indexed()? {
        Some(ref mut idx) if util::njobs(args.flag_jobs) > 1 => args.parallel_ftables(idx),
        _ => args.sequential_ftables(),
    }?;

    wtr.write_record(vec!["field", "value", "count"])?;
    let head_ftables = headers.into_iter().zip(tables.into_iter());
    for (i, (header, ftab)) in head_ftables.enumerate() {
        let header = if rconfig.no_headers {
            (i + 1).to_string().into_bytes()
        } else {
            header.to_vec()
        };
        for (value, count) in args.counts(&ftab) {
            let count = count.to_string();
            let row = vec![&*header, &*value, count.as_bytes()];
            wtr.write_record(row)?;
        }
    }
    Ok(())
}

type Headers = csv::ByteRecord;
type FTable = Frequencies<Vec<u8>>;
type FTables = Vec<Frequencies<Vec<u8>>>;

impl Args {
    pub fn rconfig(&self) -> Config {
        Config::new(&self.arg_input)
            .delimiter(self.flag_delimiter)
            .no_headers(self.flag_no_headers)
            .select(self.flag_select.clone())
    }

    fn counts(&self, ftab: &FTable) -> Vec<(ByteString, u64)> {
        let mut counts = if self.flag_asc {
            ftab.least_frequent()
        } else {
            ftab.most_frequent()
        };
        if self.flag_limit > 0 {
            counts = counts.into_iter().take(self.flag_limit).collect();
        }
        counts
            .into_iter()
            .map(|(bs, c)| {
                if b"" == &**bs {
                    (b"(NULL)"[..].to_vec(), c)
                } else {
                    (bs.clone(), c)
                }
            })
            .collect()
    }

    pub fn sequential_ftables(&self) -> CliResult<(Headers, FTables)> {
        let mut rdr = self.rconfig().reader()?;
        let (headers, sel) = self.sel_headers(&mut rdr)?;
        Ok((headers, self.ftables(&sel, rdr.byte_records())?))
    }

    pub fn parallel_ftables(
        &self,
        idx: &mut Indexed<fs::File, fs::File>,
    ) -> CliResult<(Headers, FTables)> {
        let mut rdr = self.rconfig().reader()?;
        let (headers, sel) = self.sel_headers(&mut rdr)?;

        if idx.count() == 0 {
            return Ok((headers, vec![]));
        }

        let chunk_size = util::chunk_size(idx.count() as usize, util::njobs(self.flag_jobs));
        let nchunks = util::num_of_chunks(idx.count() as usize, chunk_size);

        let pool = ThreadPool::new(util::njobs(self.flag_jobs));
        let (send, recv) = channel::bounded(0);
        for i in 0..nchunks {
            let (send, args, sel) = (send.clone(), self.clone(), sel.clone());
            pool.execute(move || {
                let mut idx = args.rconfig().indexed().unwrap().unwrap();
                idx.seek((i * chunk_size) as u64).unwrap();
                let it = idx.byte_records().take(chunk_size);
                send.send(args.ftables(&sel, it).unwrap()).unwrap();
            });
        }
        drop(send);
        Ok((headers, merge_all(recv.iter()).unwrap()))
    }

    fn ftables<I>(&self, sel: &Selection, it: I) -> CliResult<FTables>
    where
        I: Iterator<Item = csv::Result<csv::ByteRecord>>,
    {
        let null = &b""[..].to_vec();
        let nsel = sel.normal();
        let mut tabs: Vec<_> = (0..nsel.len()).map(|_| Frequencies::new()).collect();
        #[allow(unused_assignments)]
        // amortize allocation
        let mut field_work: Vec<u8> = Vec::with_capacity(100);
        let mut row_work: csv::ByteRecord;
        for row in it {
            row_work = row?;
            for (i, field) in nsel.select(row_work.into_iter()).enumerate() {
                field_work = {
                    match simdutf8::basic::from_utf8(field) {
                        Ok(s) => s.trim().as_bytes().to_vec(),
                        Err(_) => field.to_vec(),
                    }
                };
                if !field_work.is_empty() {
                    tabs[i].add(field_work);
                } else if !self.flag_no_nulls {
                    tabs[i].add(null.clone());
                }
            }
        }
        Ok(tabs)
    }

    fn sel_headers<R: io::Read>(
        &self,
        rdr: &mut csv::Reader<R>,
    ) -> CliResult<(csv::ByteRecord, Selection)> {
        let headers = rdr.byte_headers()?;
        let sel = self.rconfig().selection(headers)?;
        Ok((sel.select(headers).map(<[u8]>::to_vec).collect(), sel))
    }
}
