#![allow(unknown_lints)]
#![cfg_attr(
    feature = "cargo-clippy",
    allow(
        clippy::cast_possible_truncation,
        clippy::cast_possible_wrap,
        clippy::cast_sign_loss,
        // things are often more readable this way
        clippy::cast_lossless,
        clippy::module_name_repetitions,
        clippy::option_if_let_else,
        clippy::single_match_else,
        clippy::type_complexity,
        clippy::use_self,
        clippy::zero_prefixed_literal,
        // correctly used
        clippy::derive_partial_eq_without_eq,
        clippy::enum_glob_use,
        clippy::explicit_auto_deref,
        clippy::let_underscore_drop,
        clippy::map_err_ignore,
        clippy::result_unit_err,
        clippy::wildcard_imports,
        // not practical
        clippy::needless_pass_by_value,
        clippy::similar_names,
        clippy::too_many_lines,
        clippy::struct_excessive_bools,
        clippy::cognitive_complexity,
        // preference
        clippy::doc_markdown,
        clippy::unseparated_literal_suffix,
        clippy::items_after_statements,
        clippy::unnecessary_wraps,
        // false positive
        clippy::needless_doctest_main,
        // noisy
        clippy::missing_errors_doc,
        clippy::must_use_candidate,
    )
)]

extern crate crossbeam_channel as channel;

use std::borrow::ToOwned;
use std::env;
use std::fmt;
use std::io;
use std::process;
use std::time::Instant;

use docopt::Docopt;
use log::{info, log_enabled, Level};
use serde::Deserialize;

#[cfg(feature = "python")]
use pyo3::Python;

#[cfg(feature = "mimalloc")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

macro_rules! wout {
    ($($arg:tt)*) => ({
        use std::io::Write;
        (writeln!(&mut ::std::io::stdout(), $($arg)*)).unwrap();
    });
}

macro_rules! werr {
    ($($arg:tt)*) => ({
        use std::io::Write;
        use log::error;
        error!("{}", $($arg)*);
        (writeln!(&mut ::std::io::stderr(), $($arg)*)).unwrap();
    });
}

macro_rules! fail {
    ($e:expr) => {{
        use log::error;
        let err = ::std::convert::From::from($e);
        error!("{err}");
        Err(err)
    }};
}

macro_rules! command_list {
    () => {
        "
    apply*      Apply series of transformations to a column
    behead      Drop header from CSV file
    cat         Concatenate by row or column
    count       Count records
    dedup       Remove redundant rows
    enum        Add a new column enumerating CSV lines
    excel       Exports an Excel sheet to a CSV
    exclude     Excludes the records in one CSV from another
    explode     Explode rows based on some column separator
    extsort     Sort arbitrarily large text file
    fetch*      Fetches data from web services for every row using HTTP Get.
    fetchpost*  Fetches data from web services for every row using HTTP Post.
    fill        Fill empty values
    fixlengths  Makes all records have same length
    flatten     Show one field per line
    fmt         Format CSV output (change field delimiter)
    foreach*    Loop over a CSV file to execute bash commands (*nix only)
    frequency   Show frequency tables
    generate*   Generate test data by profiling a CSV
    headers     Show header names
    help        Show this usage message
    index       Create CSV index for faster access
    input       Read CSVs w/ special quoting, skipping, trimming & transcoding rules
    join        Join CSV files
    jsonl       Convert newline-delimited JSON files to CSV
    lua*        Execute Lua script on CSV data
    partition   Partition CSV data based on a column value
    pseudo      Pseudonymise the values of a column
    py*         Evaluate a Python expression on CSV data
    rename      Rename the columns of CSV data efficiently
    replace     Replace patterns in CSV data
    reverse     Reverse rows of CSV data
    sample      Randomly sample CSV data
    schema      Generate JSON Schema from CSV data
    search      Search CSV data with a regex
    searchset   Search CSV data with a regex set
    select      Select, re-order, duplicate or drop columns
    slice       Slice records from CSV
    sniff       Quickly sniff CSV metadata
    sort        Sort CSV data in alphabetical, numerical, reverse or random order
    split       Split CSV data into many files
    stats       Infer data types and compute descriptive statistics
    table       Align CSV data into columns
    tojsonl     Convert CSV to newline-delimited JSON
    transpose   Transpose rows/columns of CSV data
    validate    Validate CSV data for RFC4180-compliance or with JSON Schema

    * optional feature

    sponsored by datHere - Data Infrastructure Engineering
"
    };
}
mod cmd;
mod config;
mod index;
mod select;
mod util;

static USAGE: &str = concat!(
    "
Usage:
    qsv <command> [<args>...]
    qsv [options]

Options:
    --list               List all commands available.
    --envlist            List all qsv-relevant environment variables.
    -u, --update         Update qsv to the latest release from GitHub.
    -h, --help           Display this message
    <command> -h         Display the command help message
    -v, --version        Print version info, mem allocator, features installed, 
                         max_jobs, num_cpus then exit

* sponsored by datHere - Data Infrastructure Engineering
"
);

#[derive(Deserialize)]
struct Args {
    arg_command: Option<Command>,
    flag_list: bool,
    flag_envlist: bool,
    flag_update: bool,
}

#[cfg(feature = "python")]
fn check_python() -> bool {
    Python::with_gil(|py| py.version_info() >= (3, 8))
}

fn main() {
    util::init_logger();

    #[cfg(feature = "python")]
    if !check_python() {
        werr!("Python 3.8+ required.");
        ::std::process::exit(1);
    }

    let now = Instant::now();
    let qsv_args: String = if log_enabled!(Level::Info) {
        env::args().skip(1).collect::<Vec<_>>().join(" ")
    } else {
        "".to_string()
    };
    info!("START: {qsv_args}");

    let args: Args = Docopt::new(USAGE)
        .and_then(|d| {
            d.options_first(true)
                .version(Some(util::version()))
                .deserialize()
        })
        .unwrap_or_else(|e| e.exit());
    if args.flag_list {
        wout!(concat!("Installed commands:", command_list!()));
        log_end(qsv_args, now);
        return;
    } else if args.flag_envlist {
        util::show_env_vars();
        log_end(qsv_args, now);
        return;
    }
    if args.flag_update {
        util::qsv_check_for_update();
        log_end(qsv_args, now);
        return;
    }
    match args.arg_command {
        None => {
            werr!(concat!(
                "qsv is a suite of CSV command line utilities.

Please choose one of the following commands:",
                command_list!()
            ));
            util::qsv_check_for_update();
            log_end(qsv_args, now);
            ::std::process::exit(0);
        }
        Some(cmd) => match cmd.run() {
            Ok(()) => {
                log_end(qsv_args, now);
                process::exit(0);
            }
            Err(CliError::Flag(err)) => err.exit(),
            Err(CliError::Csv(err)) => {
                werr!("{err}");
                log_end(qsv_args, now);
                process::exit(1);
            }
            Err(CliError::Io(ref err)) if err.kind() == io::ErrorKind::BrokenPipe => {
                log_end(qsv_args, now);
                process::exit(0);
            }
            Err(CliError::Io(err)) => {
                werr!("{err}");
                log_end(qsv_args, now);
                process::exit(1);
            }
            Err(CliError::Other(msg)) => {
                werr!("{msg}");
                log_end(qsv_args, now);
                process::exit(1);
            }
        },
    }
}

fn log_end(mut qsv_args: String, now: Instant) {
    if log_enabled!(Level::Info) {
        let ellipsis = if qsv_args.len() > 24 {
            qsv_args.truncate(24);
            "..."
        } else {
            ""
        };
        info!(
            "END \"{qsv_args}{ellipsis}\" elapsed: {}",
            now.elapsed().as_secs_f32()
        );
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Command {
    #[cfg(all(feature = "apply", not(feature = "lite")))]
    Apply,
    Behead,
    Cat,
    Count,
    Dedup,
    Enum,
    Excel,
    Exclude,
    Explode,
    ExtSort,
    #[cfg(all(feature = "fetch", not(feature = "lite")))]
    Fetch,
    #[cfg(all(feature = "fetch", not(feature = "lite")))]
    FetchPost,
    Fill,
    FixLengths,
    Flatten,
    Fmt,
    #[cfg(all(feature = "foreach", target_family = "unix", not(feature = "lite")))]
    ForEach,
    Frequency,
    #[cfg(all(feature = "generate", not(feature = "lite")))]
    Generate,
    Headers,
    Help,
    Index,
    Input,
    Join,
    Jsonl,
    #[cfg(all(feature = "lua", not(feature = "lite")))]
    Lua,
    Partition,
    Pseudo,
    #[cfg(all(feature = "python", not(feature = "lite")))]
    Py,
    Rename,
    Replace,
    Reverse,
    Sample,
    Schema,
    Search,
    SearchSet,
    Select,
    Slice,
    Sniff,
    Sort,
    Split,
    Stats,
    Table,
    Transpose,
    Tojsonl,
    Validate,
}

impl Command {
    fn run(self) -> CliResult<()> {
        let argv: Vec<_> = env::args().collect();
        let argv: Vec<_> = argv.iter().map(|s| &**s).collect();
        let argv = &*argv;

        if !argv[1].chars().all(char::is_lowercase) {
            return Err(CliError::Other(format!(
                "qsv expects commands in lowercase. Did you mean '{}'?",
                argv[1].to_lowercase()
            )));
        }
        match self {
            Command::Behead => cmd::behead::run(argv),
            #[cfg(all(feature = "apply", not(feature = "lite")))]
            Command::Apply => cmd::apply::run(argv),
            Command::Cat => cmd::cat::run(argv),
            Command::Count => cmd::count::run(argv),
            Command::Dedup => cmd::dedup::run(argv),
            Command::Enum => cmd::enumerate::run(argv),
            Command::Excel => cmd::excel::run(argv),
            Command::Exclude => cmd::exclude::run(argv),
            Command::Explode => cmd::explode::run(argv),
            Command::ExtSort => cmd::extsort::run(argv),
            #[cfg(all(feature = "fetch", not(feature = "lite")))]
            Command::Fetch => cmd::fetch::run(argv),
            #[cfg(all(feature = "fetch", not(feature = "lite")))]
            Command::FetchPost => cmd::fetchpost::run(argv),
            #[cfg(all(feature = "foreach", target_family = "unix", not(feature = "lite")))]
            Command::ForEach => cmd::foreach::run(argv),
            Command::Fill => cmd::fill::run(argv),
            Command::FixLengths => cmd::fixlengths::run(argv),
            Command::Flatten => cmd::flatten::run(argv),
            Command::Fmt => cmd::fmt::run(argv),
            Command::Frequency => cmd::frequency::run(argv),
            #[cfg(all(feature = "generate", not(feature = "lite")))]
            Command::Generate => cmd::generate::run(argv),
            Command::Headers => cmd::headers::run(argv),
            Command::Help => {
                wout!("{USAGE}");
                util::qsv_check_for_update();
                Ok(())
            }
            Command::Index => cmd::index::run(argv),
            Command::Input => cmd::input::run(argv),
            Command::Join => cmd::join::run(argv),
            Command::Jsonl => cmd::jsonl::run(argv),
            #[cfg(all(feature = "lua", not(feature = "lite")))]
            Command::Lua => cmd::lua::run(argv),
            Command::Partition => cmd::partition::run(argv),
            Command::Pseudo => cmd::pseudo::run(argv),
            #[cfg(all(feature = "python", not(feature = "lite")))]
            Command::Py => cmd::python::run(argv),
            Command::Rename => cmd::rename::run(argv),
            Command::Replace => cmd::replace::run(argv),
            Command::Reverse => cmd::reverse::run(argv),
            Command::Sample => cmd::sample::run(argv),
            Command::Schema => cmd::schema::run(argv),
            Command::Search => cmd::search::run(argv),
            Command::SearchSet => cmd::searchset::run(argv),
            Command::Select => cmd::select::run(argv),
            Command::Slice => cmd::slice::run(argv),
            Command::Sniff => cmd::sniff::run(argv),
            Command::Sort => cmd::sort::run(argv),
            Command::Split => cmd::split::run(argv),
            Command::Stats => cmd::stats::run(argv),
            Command::Table => cmd::table::run(argv),
            Command::Transpose => cmd::transpose::run(argv),
            Command::Tojsonl => cmd::tojsonl::run(argv),
            Command::Validate => cmd::validate::run(argv),
        }
    }
}

pub type CliResult<T> = Result<T, CliError>;

#[derive(Debug)]
pub enum CliError {
    Flag(docopt::Error),
    Csv(csv::Error),
    Io(io::Error),
    Other(String),
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CliError::Flag(ref e) => e.fmt(f),
            CliError::Csv(ref e) => e.fmt(f),
            CliError::Io(ref e) => e.fmt(f),
            CliError::Other(ref s) => f.write_str(s),
        }
    }
}

impl From<docopt::Error> for CliError {
    fn from(err: docopt::Error) -> CliError {
        CliError::Flag(err)
    }
}

impl From<csv::Error> for CliError {
    fn from(err: csv::Error) -> CliError {
        if !err.is_io_error() {
            return CliError::Csv(err);
        }
        match err.into_kind() {
            csv::ErrorKind::Io(v) => From::from(v),
            _ => unreachable!(),
        }
    }
}

impl From<io::Error> for CliError {
    fn from(err: io::Error) -> CliError {
        CliError::Io(err)
    }
}

impl From<String> for CliError {
    fn from(err: String) -> CliError {
        CliError::Other(err)
    }
}

impl<'a> From<&'a str> for CliError {
    fn from(err: &'a str) -> CliError {
        CliError::Other(err.to_owned())
    }
}

impl From<regex::Error> for CliError {
    fn from(err: regex::Error) -> CliError {
        CliError::Other(format!("{err:?}"))
    }
}
