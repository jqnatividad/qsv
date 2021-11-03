use std::io;

use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};

use crate::config::{Config, Delimiter};
use crate::index::Indexed;
use indicatif::ProgressBar;
use crate::util;
use crate::CliResult;
use serde::Deserialize;

static USAGE: &str = "
Randomly scrambles CSV records uniformly using memory proportional to the size of
the CSV.

Usage:
    qsv scramble [options] [<input>]
    qsv scramble --help

scramble options:
--seed <number>            RNG seed.

Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
    -n, --no-headers       When set, the first row will be considered as part of
                           the data to scramble. (When not set, the
                           first row is the header row and will always appear
                           in the output.)
    -d, --delimiter <arg>  The field delimiter for reading CSV data.
                           Must be a single character. (default: ,)
    -q, --quiet            Don't show progress bars.
";

#[derive(Deserialize)]
struct Args {
    arg_input: Option<String>,
    flag_output: Option<String>,
    flag_no_headers: bool,
    flag_delimiter: Option<Delimiter>,
    flag_seed: Option<u64>,
    flag_quiet: bool,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;
    let rconfig = Config::new(&args.arg_input)
        .delimiter(args.flag_delimiter)
        .no_headers(args.flag_no_headers);

    // prep progress bar
    let progress = ProgressBar::new(0u64);

    let mut wtr = Config::new(&args.flag_output).writer()?;
    let scrambled = match rconfig.indexed()? {
        Some(mut idx) => {
            if !args.flag_quiet {
                util::prep_progress(&progress, idx.count());
                progress.set_draw_rate(1);
            }
            rconfig.write_headers(&mut *idx, &mut wtr)?;
            scramble_random_access(&mut idx, &progress, args.flag_quiet, args.flag_seed)?
        }
        _ => {
            // scrambling requires an index
            return fail!("Scrambling requires an index.");
        }
    };

    for row in scrambled.into_iter() {
        wtr.write_byte_record(&row)?;
    }
    
    if !args.flag_quiet {
        util::finish_progress(&progress);
    }
    Ok(wtr.flush()?)
}

fn scramble_random_access<R, I>(
    idx: &mut Indexed<R, I>,
    progress: &ProgressBar,
    quiet: bool,
    seed: Option<u64>,
) -> CliResult<Vec<csv::ByteRecord>>
where
    R: io::Read + io::Seek,
    I: io::Read + io::Seek,
{
    let idxcount = idx.count();
    let mut all_indices = (0..idxcount).collect::<Vec<_>>();

    if let Some(val) = seed {
        let mut rng = StdRng::seed_from_u64(val);
        SliceRandom::shuffle(&mut *all_indices, &mut rng);
    } else {
        let mut rng = ::rand::thread_rng();
        SliceRandom::shuffle(&mut *all_indices, &mut rng);
    }

    let mut scrambled = Vec::with_capacity(idxcount as usize);
    for i in all_indices.into_iter().take(idxcount as usize) {
        if !quiet {
            progress.inc(1);
        }
        idx.seek(i)?;
        scrambled.push(idx.byte_records().next().unwrap()?);
    }
    Ok(scrambled)
}
