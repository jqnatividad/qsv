use std::io;

use rand::{self, rngs::StdRng, seq::SliceRandom, Rng, SeedableRng};

use crate::config::{Config, Delimiter};
use crate::index::Indexed;
use crate::util;
use crate::CliResult;
use serde::Deserialize;

static USAGE: &str = "
Randomly samples CSV data uniformly using memory proportional to the size of
the sample.

When an index is present and a seed is not specified, this command will use 
random indexing if the sample size is less than 10% of the total number of records.
This allows for efficient sampling such that the entire CSV file is not parsed.

When sample-size is between 0 and 1 exclusive, it is treated as a percentage
of the CSV to sample (e.g. 0.20 is 20 percent). This requires an index.

This command is intended to provide a means to sample from a CSV data set that
is too big to fit into memory (for example, for use with commands like 'qsv
frequency' or 'qsv stats'). It will however visit every CSV record exactly
once, which is necessary to provide a uniform random sample. If you wish to
limit the number of records visited, use the 'qsv slice' command to pipe into
'qsv sample'.

Usage:
    qsv sample [options] <sample-size> [<input>]
    qsv sample --help

sample options:
    --seed <number>        RNG seed.

Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
    -n, --no-headers       When set, the first row will be considered as part of
                           the population to sample from. (When not set, the
                           first row is the header row and will always appear
                           in the output.)
    -d, --delimiter <arg>  The field delimiter for reading CSV data.
                           Must be a single character. (default: ,)
";

#[derive(Deserialize)]
struct Args {
    arg_input: Option<String>,
    arg_sample_size: f64,
    flag_output: Option<String>,
    flag_no_headers: bool,
    flag_delimiter: Option<Delimiter>,
    flag_seed: Option<usize>,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;
    let rconfig = Config::new(&args.arg_input)
        .delimiter(args.flag_delimiter)
        .no_headers(args.flag_no_headers);
    let mut sample_size = args.arg_sample_size;

    let mut wtr = Config::new(&args.flag_output).writer()?;
    let sampled = match rconfig.indexed()? {
        Some(mut idx) => {
            if sample_size < 1.0 {
                sample_size *= idx.count() as f64;
            }
            if args.flag_seed.is_none() && do_random_access(sample_size as u64, idx.count()) {
                rconfig.write_headers(&mut *idx, &mut wtr)?;
                sample_random_access(&mut idx, sample_size as u64)?
            } else {
                let mut rdr = rconfig.reader()?;
                rconfig.write_headers(&mut rdr, &mut wtr)?;
                sample_reservoir(&mut rdr, sample_size as u64, args.flag_seed)?
            }
        }
        _ => {
            if sample_size < 1.0 {
                return fail!("Percentage sampling requires an index.");
            }
            let mut rdr = rconfig.reader()?;
            rconfig.write_headers(&mut rdr, &mut wtr)?;
            sample_reservoir(&mut rdr, sample_size as u64, args.flag_seed)?
        }
    };
    for row in sampled.into_iter() {
        wtr.write_byte_record(&row)?;
    }
    Ok(wtr.flush()?)
}

fn sample_random_access<R, I>(
    idx: &mut Indexed<R, I>,
    sample_size: u64,
) -> CliResult<Vec<csv::ByteRecord>>
where
    R: io::Read + io::Seek,
    I: io::Read + io::Seek,
{
    let mut all_indices = (0..idx.count()).collect::<Vec<_>>();
    let mut rng = ::rand::thread_rng();
    SliceRandom::shuffle(&mut *all_indices, &mut rng);

    let mut sampled = Vec::with_capacity(sample_size as usize);
    for i in all_indices.into_iter().take(sample_size as usize) {
        idx.seek(i)?;
        sampled.push(idx.byte_records().next().unwrap()?);
    }
    Ok(sampled)
}

fn sample_reservoir<R: io::Read>(
    rdr: &mut csv::Reader<R>,
    sample_size: u64,
    seed: Option<usize>,
) -> CliResult<Vec<csv::ByteRecord>> {
    // The following algorithm has been adapted from:
    // https://en.wikipedia.org/wiki/Reservoir_sampling
    let mut reservoir = Vec::with_capacity(sample_size as usize);
    let mut records = rdr.byte_records().enumerate();
    for (_, row) in records.by_ref().take(reservoir.capacity()) {
        reservoir.push(row?);
    }

    // Seeding rng
    let mut rng: StdRng = match seed {
        None => StdRng::from_rng(rand::thread_rng()).unwrap(),
        Some(seed) => StdRng::seed_from_u64(seed as u64),
    };

    // Now do the sampling.
    for (i, row) in records {
        let random = rng.gen_range(0..i + 1);
        if random < sample_size as usize {
            reservoir[random] = row?;
        }
    }
    Ok(reservoir)
}

fn do_random_access(sample_size: u64, total: u64) -> bool {
    sample_size <= (total / 10)
}
