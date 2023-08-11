static USAGE: &str = r#"
Randomly samples CSV data uniformly using memory proportional to the size of
the sample.

When an index is present and a seed is not specified, this command will use 
random indexing if the sample size is less than 10% of the total number of records.
This allows for efficient sampling such that the entire CSV file is not parsed.

This command is intended to provide a means to sample from a CSV data set that
is too big to fit into memory (for example, for use with commands like 'qsv
frequency' or 'qsv stats'). It will however visit every CSV record exactly
once, which is necessary to provide a uniform random sample. If you wish to
limit the number of records visited, use the 'qsv slice' command to pipe into
'qsv sample'.

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

sample options:
    --seed <number>        Random number generator seed.
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
    -d, --delimiter <arg>  The field delimiter for reading CSV data.
                           Must be a single character. (default: ,)
"#;

use std::io;

use log::debug;
use rand::{self, rngs::StdRng, seq::SliceRandom, Rng, SeedableRng};
use serde::Deserialize;
use tempfile::NamedTempFile;
use url::Url;

use crate::{
    config::{Config, Delimiter},
    index::Indexed,
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

    let mut wtr = Config::new(&args.flag_output).writer()?;
    let sampled = if let Some(mut idx) = rconfig.indexed()? {
        #[allow(clippy::cast_precision_loss)]
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
    } else {
        debug!("no index");
        #[allow(clippy::cast_precision_loss)]
        if sample_size < 1.0 {
            let Ok(row_count) = util::count_rows(&rconfig) else {
                return fail!("Cannot get rowcount. Percentage sampling requires a rowcount.");
            };
            sample_size *= row_count as f64;
        }
        let mut rdr = rconfig.reader()?;
        rconfig.write_headers(&mut rdr, &mut wtr)?;
        sample_reservoir(&mut rdr, sample_size as u64, args.flag_seed)?
    };

    for row in sampled {
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
    debug!("doing sample_random_access");
    let mut all_indices = (0..idx.count()).collect::<Vec<_>>();
    let mut rng = ::rand::thread_rng();
    // this non-cryptographic shuffle is sufficient for our use case
    // as we're optimizing for performance. Add DevSkim lint ignore.
    SliceRandom::shuffle(&mut *all_indices, &mut rng); //DevSkim: ignore DS148264

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
    debug!("doing sample_reservoir");
    // The following algorithm has been adapted from:
    // https://en.wikipedia.org/wiki/Reservoir_sampling
    let mut reservoir = Vec::with_capacity(sample_size as usize);
    let mut records = rdr.byte_records().enumerate();
    for (_, row) in records.by_ref().take(reservoir.capacity()) {
        reservoir.push(row?);
    }

    // Seeding RNG
    let mut rng: StdRng = match seed {
        None => StdRng::from_rng(rand::thread_rng()).unwrap(),
        // the non-cryptographic seed_from_u64 is sufficient for our use case
        // as we're optimizing for performance
        Some(seed) => StdRng::seed_from_u64(seed as u64), //DevSkim: ignore DS148264
    };

    // Now do the sampling.
    for (i, row) in records {
        let random = rng.gen_range(0..=i);
        if random < sample_size as usize {
            reservoir[random] = row?;
        }
    }
    Ok(reservoir)
}

fn do_random_access(sample_size: u64, total: u64) -> bool {
    let raflag = sample_size <= (total / 10);
    debug!("sample_size: {sample_size}, total: {total}, raflag: {raflag}");
    raflag
}
