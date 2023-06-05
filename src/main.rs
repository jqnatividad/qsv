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
        clippy::unnecessary_wraps,
        // false positive
        clippy::needless_doctest_main,
        // performance
        clippy::len_zero,
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

use crate::clitypes::{CliError, CliResult, QsvExitCode};

#[cfg(feature = "mimalloc")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[cfg(feature = "jemallocator")]
#[global_allocator]
static GLOBAL: jemallocator::Jemalloc = jemallocator::Jemalloc;

mod clitypes;
mod cmd;
mod config;
mod index;
mod odhtcache;
mod select;
mod util;

static USAGE: &str = r#"
Usage:
    qsv <command> [<args>...]
    qsv [options]

Options:
    --list               List all commands available.
    --envlist            List all qsv-relevant environment variables.
    -u, --update         Update qsv to the latest release from GitHub.
    -U, --updatenow      Update qsv to the latest release from GitHub without confirming.
    -h, --help           Display this message
    <command> -h         Display the command help message
    -v, --version        Print version info, mem allocator, features installed, 
                         max_jobs, num_cpus, build info then exit

* sponsored by datHere - Data Infrastructure Engineering
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
    let mut enabled_commands = String::new();
    #[cfg(all(feature = "apply", feature = "feature_capable"))]
    enabled_commands.push_str("    apply       Apply series of transformations to a column\n");

    enabled_commands.push_str(
        "    behead      Drop header from CSV file
    cat         Concatenate by row or column
    count       Count records
    dedup       Remove redundant rows
    diff        Find the difference between two CSVs
    enum        Add a new column enumerating CSV lines
    excel       Exports an Excel sheet to a CSV
    exclude     Excludes the records in one CSV from another
    explode     Explode rows based on some column separator
    extdedup    Remove duplicates rows from an arbitrarily large text file
    extsort     Sort arbitrarily large text file\n",
    );

    #[cfg(all(feature = "fetch", feature = "feature_capable"))]
    enabled_commands.push_str(
        "    fetch       Fetches data from web services for every row using HTTP Get.
    fetchpost   Fetches data from web services for every row using HTTP Post.\n",
    );

    enabled_commands.push_str(
        "    fill        Fill empty values
    fixlengths  Makes all records have same length
    flatten     Show one field per line
    fmt         Format CSV output (change field delimiter)\n",
    );

    #[cfg(all(feature = "foreach", feature = "feature_capable"))]
    enabled_commands
        .push_str("    foreach     Loop over a CSV file to execute bash commands (*nix only)\n");

    enabled_commands.push_str("    frequency   Show frequency tables\n");

    #[cfg(all(feature = "generate", not(feature = "lite")))]
    enabled_commands.push_str("    generate    Generate test data by profiling a CSV\n");

    enabled_commands.push_str(
        "    headers     Show header names
    help        Show this usage message
    index       Create CSV index for faster access
    input       Read CSVs w/ special quoting, skipping, trimming & transcoding rules
    join        Join CSV files\n",
    );

    #[cfg(all(feature = "polars", feature = "feature_capable"))]
    enabled_commands.push_str("    joinp       Join CSV files using the Pola.rs engine\n");

    enabled_commands.push_str("    jsonl       Convert newline-delimited JSON files to CSV\n");

    #[cfg(all(feature = "luau", feature = "feature_capable"))]
    enabled_commands.push_str("    luau        Execute Luau script on CSV data\n");

    enabled_commands.push_str(
        "    partition   Partition CSV data based on a column value
    pseudo      Pseudonymise the values of a column\n",
    );

    #[cfg(all(feature = "python", feature = "feature_capable"))]
    enabled_commands.push_str("    py          Evaluate a Python expression on CSV data\n");

    enabled_commands.push_str(
        "    rename      Rename the columns of CSV data efficiently
    replace     Replace patterns in CSV data
    reverse     Reverse rows of CSV data
    safenames   Modify a CSV's header names to db-safe names
    sample      Randomly sample CSV data
    schema      Generate JSON Schema from CSV data
    search      Search CSV data with a regex
    searchset   Search CSV data with a regex set
    select      Select, re-order, duplicate or drop columns
    slice       Slice records from CSV
    snappy      Compress/decompress data using the Snappy algorithm
    sniff       Quickly sniff CSV metadata
    sort        Sort CSV data in alphabetical, numerical, reverse or random order
    sortcheck   Check if a CSV is sorted
    split       Split CSV data into many files\n",
    );

    #[cfg(all(feature = "polars", feature = "feature_capable"))]
    enabled_commands.push_str(
        "    sqlp        Run a SQL query against several CSVs using the Pola.rs engine\n",
    );

    enabled_commands.push_str(
        "    stats       Infer data types and compute summary statistics
    table       Align CSV data into columns
    tojsonl     Convert CSV to newline-delimited JSON\n",
    );

    #[cfg(all(feature = "to", feature = "feature_capable"))]
    enabled_commands
        .push_str("    to          Convert CSVs to PostgreSQL/XLSX/Parquet/SQLite/Data Package\n");

    enabled_commands.push_str(
        "    transpose   Transpose rows/columns of CSV data
    validate    Validate CSV data for RFC4180-compliance or with JSON Schema",
    );
    let num_commands = enabled_commands.split('\n').count();

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
        wout!("Installed commands ({num_commands}):");
        wout!(
            "{enabled_commands}\n
sponsored by datHere - Data Infrastructure Engineering
        "
        );
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
            werr!(
                "qsv is a suite of CSV command line utilities.

Please choose one of the following {num_commands} commands:\n{enabled_commands}\n
sponsored by datHere - Data Infrastructure Engineering
"
            );
            _ = util::qsv_check_for_update(true, false);
            util::log_end(qsv_args, now);
            QsvExitCode::Good
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
                QsvExitCode::Bad
            }
        },
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Command {
    #[cfg(all(feature = "apply", feature = "feature_capable"))]
    Apply,
    Behead,
    Cat,
    Count,
    Dedup,
    Diff,
    Enum,
    Excel,
    Exclude,
    Explode,
    ExtDedup,
    ExtSort,
    #[cfg(all(feature = "fetch", feature = "feature_capable"))]
    Fetch,
    #[cfg(all(feature = "fetch", feature = "feature_capable"))]
    FetchPost,
    Fill,
    FixLengths,
    Flatten,
    Fmt,
    #[cfg(all(feature = "foreach", target_family = "unix", not(feature = "lite")))]
    ForEach,
    Frequency,
    #[cfg(all(feature = "generate", feature = "feature_capable"))]
    Generate,
    Headers,
    Help,
    Index,
    Input,
    Join,
    #[cfg(all(feature = "polars", feature = "feature_capable"))]
    JoinP,
    Jsonl,
    #[cfg(all(feature = "luau", feature = "feature_capable"))]
    Luau,
    Partition,
    Pseudo,
    #[cfg(all(feature = "python", feature = "feature_capable"))]
    Py,
    Rename,
    Replace,
    Reverse,
    Safenames,
    Sample,
    Schema,
    Search,
    SearchSet,
    Select,
    Slice,
    Snappy,
    Sniff,
    Sort,
    SortCheck,
    Split,
    #[cfg(all(feature = "polars", feature = "feature_capable"))]
    SqlP,
    Stats,
    Table,
    Transpose,
    #[cfg(all(feature = "to", feature = "feature_capable"))]
    To,
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
            #[cfg(all(feature = "apply", feature = "feature_capable"))]
            Command::Apply => cmd::apply::run(argv),
            Command::Cat => cmd::cat::run(argv),
            Command::Count => cmd::count::run(argv),
            Command::Dedup => cmd::dedup::run(argv),
            Command::Diff => cmd::diff::run(argv),
            Command::Enum => cmd::enumerate::run(argv),
            Command::Excel => cmd::excel::run(argv),
            Command::Exclude => cmd::exclude::run(argv),
            Command::Explode => cmd::explode::run(argv),
            Command::ExtDedup => cmd::extdedup::run(argv),
            Command::ExtSort => cmd::extsort::run(argv),
            #[cfg(all(feature = "fetch", feature = "feature_capable"))]
            Command::Fetch => cmd::fetch::run(argv),
            #[cfg(all(feature = "fetch", feature = "feature_capable"))]
            Command::FetchPost => cmd::fetchpost::run(argv),
            #[cfg(all(feature = "foreach", target_family = "unix", not(feature = "lite")))]
            Command::ForEach => cmd::foreach::run(argv),
            Command::Fill => cmd::fill::run(argv),
            Command::FixLengths => cmd::fixlengths::run(argv),
            Command::Flatten => cmd::flatten::run(argv),
            Command::Fmt => cmd::fmt::run(argv),
            Command::Frequency => cmd::frequency::run(argv),
            #[cfg(all(feature = "generate", feature = "feature_capable"))]
            Command::Generate => cmd::generate::run(argv),
            Command::Headers => cmd::headers::run(argv),
            Command::Help => {
                wout!("{USAGE}");
                util::qsv_check_for_update(true, false)?;
                Ok(())
            }
            Command::Index => cmd::index::run(argv),
            Command::Input => cmd::input::run(argv),
            Command::Join => cmd::join::run(argv),
            #[cfg(all(feature = "polars", feature = "feature_capable"))]
            Command::JoinP => cmd::joinp::run(argv),
            Command::Jsonl => cmd::jsonl::run(argv),
            #[cfg(all(feature = "luau", feature = "feature_capable"))]
            Command::Luau => cmd::luau::run(argv),
            Command::Partition => cmd::partition::run(argv),
            Command::Pseudo => cmd::pseudo::run(argv),
            #[cfg(all(feature = "python", feature = "feature_capable"))]
            Command::Py => cmd::python::run(argv),
            Command::Rename => cmd::rename::run(argv),
            Command::Replace => cmd::replace::run(argv),
            Command::Reverse => cmd::reverse::run(argv),
            Command::Safenames => cmd::safenames::run(argv),
            Command::Sample => cmd::sample::run(argv),
            Command::Schema => cmd::schema::run(argv),
            Command::Search => cmd::search::run(argv),
            Command::SearchSet => cmd::searchset::run(argv),
            Command::Select => cmd::select::run(argv),
            Command::Slice => cmd::slice::run(argv),
            Command::Snappy => cmd::snappy::run(argv),
            Command::Sniff => cmd::sniff::run(argv),
            Command::Sort => cmd::sort::run(argv),
            Command::SortCheck => cmd::sortcheck::run(argv),
            Command::Split => cmd::split::run(argv),
            #[cfg(all(feature = "polars", feature = "feature_capable"))]
            Command::SqlP => cmd::sqlp::run(argv),
            Command::Stats => cmd::stats::run(argv),
            Command::Table => cmd::table::run(argv),
            Command::Transpose => cmd::transpose::run(argv),
            #[cfg(all(feature = "to", feature = "feature_capable"))]
            Command::To => cmd::to::run(argv),
            Command::Tojsonl => cmd::tojsonl::run(argv),
            Command::Validate => cmd::validate::run(argv),
        }
    }
}
