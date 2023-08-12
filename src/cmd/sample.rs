static USAGE: &str = r#"
Randomly samples CSV data uniformly using memory proportional to the size of
the sample if no index is present, or constant memory if an index is present.

When an index is present, this command will use random indexing.
This allows for efficient sampling such that the entire CSV file is not parsed.

Otherwise, if no index is present, it will visit every CSV record exactly once,
which is necessary to provide a uniform random sample (reservoir sampling).
If you wish to limit the number of records visited, use the 'qsv slice' command
to pipe into 'qsv sample'.

This command is intended to provide a means to sample from a CSV data set that
is too big to fit into memory (for example, for use with commands like 'qsv
frequency' or 'qsv stats'). 

For examples, see https://github.com/jqnatividad/qsv/blob/master/tests/test_sample.rs.

Usage:
    qsv sample [options] <sample-size> [<input>]
    qsv sample --help

sample arguments:
    <input>                The CSV file to sample. This can be a local file,
                           stdin, or a URL (http and https schemes supported).
    <sample-size>          The number of records to sample. If this is between
                           0 and 1 exclusive, it is treated as a percentage of
                           the CSV to sample (e.g. 0.20 is 20 percent).

                           If an index is present, this command will
                           use random indexing to sample efficiently. Otherwise,
                           it will use reservoir sampling, which requires
                           visiting every record in the CSV.

sample options:
    --seed <number>        Random Number Generator (RNG) seed.
    --faster               Use a faster RNG that uses the Wyrand algorithm instead
                           of the ChaCha algorithm used by the standard RNG.
    --user-agent <agent>   Specify custom user agent to use when the input is a URL.
                           It supports the following variables -
                           $QSV_VERSION, $QSV_TARGET, $QSV_BIN_NAME, $QSV_KIND and $QSV_COMMAND.
                           Try to follow the syntax here -
                           https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/User-Agent
    --timeout <secs>       Timeout for downloading URLs in seconds.
                           [default: 30]

Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
    -n, --no-headers       When set, the first row will be considered as part of
                           the population to sample from. (When not set, the
                           first row is the header row and will always appear
                           in the output.)
    -d, --delimiter <arg>  The field delimiter for reading/writing CSV data.
                           Must be a single character. (default: ,)
"#;

use std::io;

use fastrand; //DevSkim: ignore DS148264
use rand::{self, rngs::StdRng, seq::SliceRandom, Rng, SeedableRng};
use serde::Deserialize;
use tempfile::NamedTempFile;
use url::Url;

use crate::{
    config::{Config, Delimiter},
    util, CliResult,
};

#[derive(Deserialize)]
struct Args {
    arg_input:       Option<String>,
    arg_sample_size: f64,
    flag_output:     Option<String>,
    flag_no_headers: bool,
    flag_delimiter:  Option<Delimiter>,
    flag_seed:       Option<usize>,
    flag_faster:     bool,
    flag_user_agent: Option<String>,
    flag_timeout:    Option<u16>,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let mut args: Args = util::get_args(USAGE, argv)?;

    if args.arg_sample_size.is_sign_negative() {
        return fail!("Sample size cannot be negative.");
    }

    let temp_download = NamedTempFile::new()?;

    args.arg_input = match args.arg_input {
        Some(uri) => {
            if Url::parse(&uri).is_ok() && uri.starts_with("http") {
                // its a remote file, download it first
                let temp_download_path = temp_download.path().to_str().unwrap().to_string();

                let future = util::download_file(
                    &uri,
                    &temp_download_path,
                    false,
                    args.flag_user_agent,
                    args.flag_timeout,
                    None,
                );
                tokio::runtime::Runtime::new()?.block_on(future)?;
                Some(temp_download_path)
            } else {
                // its a local file
                Some(uri)
            }
        },
        None => None,
    };

    let rconfig = Config::new(&args.arg_input)
        .delimiter(args.flag_delimiter)
        .no_headers(args.flag_no_headers);

    let mut sample_size = args.arg_sample_size;

    let mut wtr = Config::new(&args.flag_output)
        .delimiter(args.flag_delimiter)
        .writer()?;

    if let Some(mut idx) = rconfig.indexed()? {
        // the index is present, so we can use random indexing
        #[allow(clippy::cast_precision_loss)]
        if sample_size < 1.0 {
            sample_size *= idx.count() as f64;
        }
        rconfig.write_headers(&mut *idx, &mut wtr)?;

        let mut all_indices = (0..idx.count()).collect::<Vec<_>>();

        if args.flag_faster {
            log::info!(
                "doing --faster sample_random_access. Seed: {:?}",
                args.flag_seed
            );
            if let Some(seed) = args.flag_seed {
                fastrand::seed(seed as u64); //DevSkim: ignore DS148264
            }
            all_indices = fastrand::choose_multiple(all_indices.into_iter(), sample_size as usize); //DevSkim: ignore DS148264
        } else {
            log::info!(
                "doing standard sample_random_access. Seed: {:?}",
                args.flag_seed
            );
            let mut rng: StdRng = match args.flag_seed {
                None => StdRng::from_rng(rand::thread_rng()).unwrap(),
                Some(seed) => StdRng::seed_from_u64(seed as u64), //DevSkim: ignore DS148264
            };
            SliceRandom::shuffle(&mut *all_indices, &mut rng); //DevSkim: ignore DS148264
        }

        for i in all_indices.into_iter().take(sample_size as usize) {
            idx.seek(i)?;
            wtr.write_byte_record(&idx.byte_records().next().unwrap()?)?;
        }
    } else {
        // the index is not present, so we have to do reservoir sampling
        #[allow(clippy::cast_precision_loss)]
        if sample_size < 1.0 {
            let Ok(row_count) = util::count_rows(&rconfig) else {
                return fail!("Cannot get rowcount. Percentage sampling requires a rowcount.");
            };
            sample_size *= row_count as f64;
        }
        let mut rdr = rconfig.reader()?;
        rconfig.write_headers(&mut rdr, &mut wtr)?;
        let sampled = sample_reservoir(
            &mut rdr,
            sample_size as u64,
            args.flag_seed,
            args.flag_faster,
        )?;
        for row in sampled {
            wtr.write_byte_record(&row)?;
        }
    };

    Ok(wtr.flush()?)
}

fn sample_reservoir<R: io::Read>(
    rdr: &mut csv::Reader<R>,
    sample_size: u64,
    seed: Option<usize>,
    faster: bool,
) -> CliResult<Vec<csv::ByteRecord>> {
    // The following algorithm has been adapted from:
    // https://en.wikipedia.org/wiki/Reservoir_sampling
    let mut reservoir = Vec::with_capacity(sample_size as usize);
    let mut records = rdr.byte_records().enumerate();
    for (_, row) in records.by_ref().take(reservoir.capacity()) {
        reservoir.push(row?);
    }

    if faster {
        log::info!("doing --faster sample_reservoir. Seed: {seed:?}");
        if let Some(seed) = seed {
            fastrand::seed(seed as u64); //DevSkim: ignore DS148264
        }

        let mut random: usize;
        for (i, row) in records {
            random = fastrand::usize(0..=i); //DevSkim: ignore DS148264
            if random < sample_size as usize {
                reservoir[random] = row?;
            }
        }
    } else {
        log::info!("doing standard sample_reservoir. Seed: {seed:?}");
        // Seeding RNG
        let mut rng: StdRng = match seed {
            None => StdRng::from_rng(rand::thread_rng()).unwrap(),
            // the non-cryptographic seed_from_u64 is sufficient for our use case
            // as we're optimizing for performance
            Some(seed) => StdRng::seed_from_u64(seed as u64), //DevSkim: ignore DS148264
        };

        let mut random: usize;
        // Now do the sampling.
        for (i, row) in records {
            random = rng.gen_range(0..=i);
            if random < sample_size as usize {
                reservoir[random] = row?;
            }
        }
    }
    Ok(reservoir)
}
