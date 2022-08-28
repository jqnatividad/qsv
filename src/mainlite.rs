#![allow(dead_code)]
extern crate crossbeam_channel as channel;

use crate::clitypes::{CliError, CliResult, QsvExitCode};
use std::env;
use std::io;
use std::time::Instant;

use docopt::Docopt;
use log::{error, info, log_enabled, Level};
use serde::Deserialize;

#[cfg(feature = "mimalloc")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

macro_rules! command_list {
    () => {
        "
    behead      Drop header from CSV file
    cat         Concatenate by row or column
    count       Count records
    dedup       Remove redundant rows
    enum        Add a new column enumerating CSV lines
    excel       Exports an Excel sheet to a CSV
    exclude     Excludes the records in one CSV from another
    explode     Explode rows based on some column separator
    extsort     Sort arbitrarily large text file
    fill        Fill empty values
    fixlengths  Makes all records have same length
    flatten     Show one field per line
    fmt         Format CSV output (change field delimiter)
    frequency   Show frequency tables
    headers     Show header names
    help        Show this usage message
    index       Create CSV index for faster access
    input       Read CSVs w/ special quoting, skipping, trimming & transcoding rules
    join        Join CSV files
    jsonl       Convert newline-delimited JSON files to CSV
    partition   Partition CSV data based on a column value
    pseudo      Pseudonymise the values of a column
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
    sortcheck   Check if a CSV is sorted
    split       Split CSV data into many files
    stats       Infer data types and compute descriptive statistics
    table       Align CSV data into columns
    tojsonl     Convert CSV to newline-delimited JSON
    transpose   Transpose rows/columns of CSV data
    validate    Validate CSV data for RFC4180-compliance or with JSON Schema

    sponsored by datHere - Data Infrastructure Engineering
"
    };
}
mod clitypes;
mod cmd;
mod config;
mod index;
mod select;
mod util;

static USAGE: &str = concat!(
    "
Usage:
    qsvlite <command> [<args>...]
    qsvlite [options]

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

fn main() -> QsvExitCode {
    util::init_logger();

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
        util::log_end(qsv_args, now);
        return QsvExitCode::Good;
    } else if args.flag_envlist {
        util::show_env_vars();
        util::log_end(qsv_args, now);
        return QsvExitCode::Good;
    }
    if args.flag_update {
        util::qsv_check_for_update();
        util::log_end(qsv_args, now);
        return QsvExitCode::Good;
    }
    match args.arg_command {
        None => {
            werr!(concat!(
                "qsvlite is a suite of CSV command line utilities.

Please choose one of the following commands:",
                command_list!()
            ));
            util::qsv_check_for_update();
            util::log_end(qsv_args, now);
            return QsvExitCode::Good;
        }
        Some(cmd) => match cmd.run() {
            Ok(()) => {
                util::log_end(qsv_args, now);
                QsvExitCode::Good
            }
            Err(CliError::Flag(err)) => {
                werr!("{err}");
                util::log_end(qsv_args, now);
                QsvExitCode::IncorrectUsage
            }
            Err(CliError::Csv(err)) => {
                werr!("{err}");
                util::log_end(qsv_args, now);
                QsvExitCode::Bad
            }
            Err(CliError::Io(ref err)) if err.kind() == io::ErrorKind::BrokenPipe => {
                werr!("Broken pipe: {err}");
                util::log_end(qsv_args, now);
                QsvExitCode::Abort
            }
            Err(CliError::Io(err)) => {
                werr!("{err}");
                util::log_end(qsv_args, now);
                QsvExitCode::Bad
            }
            Err(CliError::NoMatch()) => {
                util::log_end(qsv_args, now);
                QsvExitCode::Bad
            }
            Err(CliError::Other(msg)) => {
                werr!("{msg}");
                util::log_end(qsv_args, now);
                QsvExitCode::IncorrectUsage
            }
        },
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Command {
    Behead,
    Cat,
    Count,
    Dedup,
    Enum,
    Excel,
    Exclude,
    Explode,
    ExtSort,
    Fill,
    FixLengths,
    Flatten,
    Fmt,
    Frequency,
    Headers,
    Help,
    Index,
    Input,
    Join,
    Jsonl,
    Partition,
    Pseudo,
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
    SortCheck,
    Split,
    Stats,
    Table,
    Tojsonl,
    Transpose,
    Validate,
}

impl Command {
    fn run(self) -> CliResult<()> {
        let argv: Vec<_> = env::args().collect();
        let argv: Vec<_> = argv.iter().map(|s| &**s).collect();
        let argv = &*argv;

        if !argv[1].chars().all(char::is_lowercase) {
            return Err(CliError::Other(format!(
                "qsvlite expects commands in lowercase. Did you mean '{}'?",
                argv[1].to_lowercase()
            )));
        }
        match self {
            Command::Behead => cmd::behead::run(argv),
            Command::Cat => cmd::cat::run(argv),
            Command::Count => cmd::count::run(argv),
            Command::Dedup => cmd::dedup::run(argv),
            Command::Enum => cmd::enumerate::run(argv),
            Command::Excel => cmd::excel::run(argv),
            Command::Exclude => cmd::exclude::run(argv),
            Command::Explode => cmd::explode::run(argv),
            Command::ExtSort => cmd::extsort::run(argv),
            Command::Fill => cmd::fill::run(argv),
            Command::FixLengths => cmd::fixlengths::run(argv),
            Command::Flatten => cmd::flatten::run(argv),
            Command::Fmt => cmd::fmt::run(argv),
            Command::Frequency => cmd::frequency::run(argv),
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
            Command::Partition => cmd::partition::run(argv),
            Command::Pseudo => cmd::pseudo::run(argv),
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
            Command::SortCheck => cmd::sortcheck::run(argv),
            Command::Split => cmd::split::run(argv),
            Command::Stats => cmd::stats::run(argv),
            Command::Table => cmd::table::run(argv),
            Command::Tojsonl => cmd::tojsonl::run(argv),
            Command::Transpose => cmd::transpose::run(argv),
            Command::Validate => cmd::validate::run(argv),
        }
    }
}
