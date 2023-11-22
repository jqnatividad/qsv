#![cfg_attr(
    feature = "cargo-clippy",
    allow(
        clippy::cast_possible_truncation,
        clippy::cast_possible_wrap,
        clippy::cast_sign_loss,
        // things are often more readable this way
        clippy::cast_lossless,
        clippy::module_name_repetitions,
        clippy::type_complexity,
        clippy::zero_prefixed_literal,
        // correctly used
        clippy::derive_partial_eq_without_eq,
        clippy::enum_glob_use,
        let_underscore_drop,
        clippy::result_unit_err,
        // not practical
        clippy::similar_names,
        clippy::too_many_lines,
        clippy::struct_excessive_bools,
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
        clippy::use_self,
        clippy::cognitive_complexity,
        clippy::option_if_let_else,
    )
)]
extern crate crossbeam_channel as channel;
use std::{env, io, time::Instant};

extern crate qsv_docopt as docopt;
use docopt::Docopt;
use serde::Deserialize;

use crate::clitypes::{CliError, CliResult, QsvExitCode, CURRENT_COMMAND};

#[cfg(feature = "mimalloc")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[cfg(feature = "jemallocator")]
#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

macro_rules! command_list {
    () => {
        "
    applydp     Apply series of transformations to a column
    count       Count records
    describegpt Infer extended metadata using a LLM
    diff        Find the difference between two CSVs
    dedup       Remove redundant rows
    excel       Exports an Excel sheet to a CSV
    exclude     Excludes the records in one CSV from another
    extdedup    Remove duplicates rows from an arbitrarily large text file
    frequency   Show frequency tables
    headers     Show header names
    help        Show this usage message
    index       Create CSV index for faster access
    input       Read CSVs w/ special quoting, skipping, trimming & transcoding rules
    luau        Execute Luau script on CSV data
    pseudo      Pseudonymise the values of a column
    rename      Rename the columns of CSV data efficiently
    replace     Replace patterns in CSV data
    safenames   Modify a CSV's header names to db-safe names
    sample      Randomly sample CSV data
    search      Search CSV data with a regex
    searchset   Search CSV data with a regex set
    select      Select, re-order, duplicate or drop columns
    slice       Slice records from CSV
    snappy      Compress/decompress data using the Snappy algorithm
    sniff       Quickly sniff CSV metadata
    sort        Sort CSV data in alphabetical, numerical, reverse or random order
    sortcheck   Check if a CSV is sorted
    stats       Infer data types and compute summary statistics
    validate    Validate CSV data for RFC4180-compliance or with JSON Schema

    NOTE: qsvdp ignores the --progressbar option for all commands.

sponsored by datHere - Data Infrastructure Engineering (https://qsv.datHere.com)
"
    };
}
mod clitypes;
mod cmd;
mod config;
mod index;
mod odhtcache;
mod select;
mod util;

static USAGE: &str = r#"
Usage:
    qsvdp <command> [<args>...]
    qsvdp [options]

Options:
    --list               List all commands available.
    --envlist            List all qsv-relevant environment variables.
    -u, --update         Check for the latest qsv release.
    -U, --updatenow      Update qsv to the latest release from GitHub without confirming.
    -h, --help           Display this message
    <command> -h         Display the command help message
    -v, --version        Print version info, mem allocator, features installed, 
                         max_jobs, num_cpus, build info then exit

sponsored by datHere - Data Infrastructure Engineering (https://qsv.datHere.com)
"#;
#[derive(Deserialize)]
struct Args {
    arg_command:    Option<Command>,
    flag_list:      bool,
    flag_envlist:   bool,
    flag_update:    bool,
    flag_updatenow: bool,
}

fn main() -> QsvExitCode {
    let now = Instant::now();
    let (qsv_args, _logger_handle) = util::init_logger();

    let args: Args = Docopt::new(USAGE)
        .and_then(|d| {
            d.options_first(true)
                .version(Some(util::version()))
                .deserialize()
        })
        .unwrap_or_else(|e| e.exit());

    if util::load_dotenv().is_err() {
        return QsvExitCode::Bad;
    }

    if args.flag_list {
        wout!(concat!("Installed commands:", command_list!()));
        util::log_end(qsv_args, now);
        return QsvExitCode::Good;
    } else if args.flag_envlist {
        util::show_env_vars();
        util::log_end(qsv_args, now);
        return QsvExitCode::Good;
    }
    if args.flag_update || args.flag_updatenow {
        util::log_end(qsv_args, now);
        if let Err(e) = util::qsv_check_for_update(false, args.flag_updatenow) {
            eprintln!("{e}");
            return QsvExitCode::Bad;
        }
        return QsvExitCode::Good;
    }
    match args.arg_command {
        None => {
            werr!(concat!(
                "qsvdp is a suite of CSV command line utilities optimized for Datapusher+.

Please choose one of the following commands:",
                command_list!()
            ));
            _ = util::qsv_check_for_update(true, false);
            util::log_end(qsv_args, now);
            QsvExitCode::Good
        },
        Some(cmd) => match cmd.run() {
            Ok(()) => {
                util::log_end(qsv_args, now);
                QsvExitCode::Good
            },
            Err(CliError::Flag(err)) => {
                werr!("usage error: {err}");
                util::log_end(qsv_args, now);
                QsvExitCode::IncorrectUsage
            },
            Err(CliError::IncorrectUsage(err)) => {
                werr!("usage error: {err}");
                util::log_end(qsv_args, now);
                QsvExitCode::IncorrectUsage
            },
            Err(CliError::Csv(err)) => {
                werr!("csv error: {err}");
                util::log_end(qsv_args, now);
                QsvExitCode::Bad
            },
            Err(CliError::Io(ref err)) if err.kind() == io::ErrorKind::BrokenPipe => {
                werr!("broken pipe error: {err}");
                util::log_end(qsv_args, now);
                QsvExitCode::Abort
            },
            Err(CliError::Io(err)) => {
                werr!("io error: {err}");
                util::log_end(qsv_args, now);
                QsvExitCode::Bad
            },
            Err(CliError::NoMatch()) => {
                util::log_end(qsv_args, now);
                QsvExitCode::Bad
            },
            Err(CliError::Other(msg)) => {
                werr!("{msg}");
                util::log_end(qsv_args, now);
                QsvExitCode::Bad
            },
            Err(CliError::Network(msg)) => {
                werr!("network error: {msg}");
                util::log_end(qsv_args, now);
                QsvExitCode::NetworkError
            },
            Err(CliError::OutOfMemory(msg)) => {
                werr!("out of memory error: {msg}");
                util::log_end(qsv_args, now);
                QsvExitCode::OutOfMemory
            },
            Err(CliError::Encoding(msg)) => {
                werr!("encoding error: {msg}");
                util::log_end(qsv_args, now);
                QsvExitCode::EncodingError
            },
        },
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Command {
    ApplyDP,
    Count,
    Dedup,
    Describegpt,
    Diff,
    Excel,
    Exclude,
    ExtDedup,
    Frequency,
    Headers,
    Help,
    Index,
    Input,
    #[cfg(feature = "luau")]
    Luau,
    Pseudo,
    Rename,
    Replace,
    Safenames,
    Sample,
    Search,
    SearchSet,
    Select,
    Slice,
    Snappy,
    Sniff,
    Sort,
    SortCheck,
    Stats,
    Validate,
}

impl Command {
    fn run(self) -> CliResult<()> {
        let argv: Vec<_> = env::args().collect();
        let argv: Vec<_> = argv.iter().map(|s| &**s).collect();
        let argv = &*argv;

        assert!(argv.len() > 1);
        if !argv[1].chars().all(char::is_lowercase) {
            return fail_incorrectusage_clierror!(
                "qsv expects commands in lowercase. Did you mean '{}'?",
                argv[1].to_lowercase()
            );
        }
        CURRENT_COMMAND.get_or_init(|| argv[1].to_lowercase());
        match self {
            Command::ApplyDP => cmd::applydp::run(argv),
            Command::Count => cmd::count::run(argv),
            Command::Dedup => cmd::dedup::run(argv),
            Command::Describegpt => cmd::describegpt::run(argv),
            Command::Diff => cmd::diff::run(argv),
            Command::Excel => cmd::excel::run(argv),
            Command::Exclude => cmd::exclude::run(argv),
            Command::ExtDedup => cmd::extdedup::run(argv),
            Command::Frequency => cmd::frequency::run(argv),
            Command::Headers => cmd::headers::run(argv),
            Command::Help => {
                wout!("{USAGE}");
                util::qsv_check_for_update(true, false)?;
                Ok(())
            },
            Command::Index => cmd::index::run(argv),
            Command::Input => cmd::input::run(argv),
            #[cfg(feature = "luau")]
            Command::Luau => cmd::luau::run(argv),
            Command::Pseudo => cmd::pseudo::run(argv),
            Command::Rename => cmd::rename::run(argv),
            Command::Replace => cmd::replace::run(argv),
            Command::Safenames => cmd::safenames::run(argv),
            Command::Sample => cmd::sample::run(argv),
            Command::Search => cmd::search::run(argv),
            Command::SearchSet => cmd::searchset::run(argv),
            Command::Select => cmd::select::run(argv),
            Command::Slice => cmd::slice::run(argv),
            Command::Snappy => cmd::snappy::run(argv),
            Command::Sniff => cmd::sniff::run(argv),
            Command::Sort => cmd::sort::run(argv),
            Command::SortCheck => cmd::sortcheck::run(argv),
            Command::Stats => cmd::stats::run(argv),
            Command::Validate => cmd::validate::run(argv),
        }
    }
}
