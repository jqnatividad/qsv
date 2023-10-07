static USAGE: &str = r#"
Convert CSV files to PostgreSQL, SQLite, XLSX, Parquet and Data Package.

POSTGRES
To convert to postgres you need to supply connection string.
The format is described here - https://docs.rs/postgres/latest/postgres/config/struct.Config.html#examples-1.
Additionally you can use `env=MY_ENV_VAR` and qsv will get the connection string from the
environment variable `MY_ENV_VAR`.

If using the `--dump` option instead of a connection string put a name of a file or `-` for stdout.

Examples:

Load `file1.csv` and `file2.csv' file to local database `test`, with user `testuser`, and password `pass`.

  $ qsv to postgres 'postgres://testuser:pass@localhost/test' file1.csv file2.csv

Load same files into a new/existing postgres schema `myschema`

  $ qsv to postgres 'postgres://testuser:pass@localhost/test' --schema=myschema file1.csv file2.csv

Load same files into a new/existing postgres database whose connection string is in the
`DATABASE_URL` environment variable.

  $ qsv to postgres 'env=DATABASE_URL' file1.csv file2.csv

Load files inside a directory to a local database 'test' with user `testuser`, password `pass`.

  $ qsv to postgres 'postgres://testuser:pass@localhost/test' dir1 

Drop tables if they exist before loading.

  $ qsv to postgres 'postgres://testuser:pass@localhost/test' --drop file1.csv file2.csv

Evolve tables if they exist before loading. Read http://datapackage_convert.opendata.coop/evolve.html
to explain how evolving works.

  $ qsv to postgres 'postgres://testuser:pass@localhost/test' --evolve file1.csv file2.csv

Create dump file.

  $ qsv to postgres --dump dumpfile.sql file1.csv file2.csv

Print dump to stdout.

  $ qsv to postgres --dump - file1.csv file2.csv


SQLITE
Convert to sqlite db file. Will be created if it does not exist.

If using the `--dump` option, instead of a sqlite database file, put the name of the dump file or `-` for stdout.

Examples:

Load `file1.csv` and `file2.csv' files to sqlite database `test.db`

  $ qsv to sqlite test.db file1.csv file2.csv

Load all files in dir1 to sqlite database `test.db`

  $ qsv to sqlite test.db dir

Drop tables if they exist before loading.

  $ qsv to sqlite test.db --drop file1.csv file2.csv

Evolve tables if they exist. Read http://datapackage_convert.opendata.coop/evolve.html
to explain how evolving is done.

  $ qsv to sqlite test.db --evolve file1.csv file2.csv

Create dump file .

  $ qsv to sqlite --dump dumpfile.sql file1.csv file2.csv

Print dump to stdout.

  $ qsv to sqlite --dump - file1.csv file2.csv


XLSX
Convert to new xlsx file.

Example:

Load `file1.csv` and `file2.csv' into xlsx file.
Will create `output.xlsx`, creating new sheets for each file, with the sheet name being the
filename without the extension. Note the `output.xlsx` will be overwritten if it exists.

  $ qsv to xlsx output.xlsx file1.csv file2.csv

PARQUET (only available if compiled with `to_parquet` feature)
Convert to directory of parquet files.  Need to select a directory, it will be created if it does not exists.
If the `to_parquet` feature is not enabled, a simpler parquet conversion is available using the `sqlp`
subcommand with the `--format parquet` option.

To stream the data use the `pipe` option.  To pipe from stdin use `-` for the filename or use named pipe.
Type guessing is more limited with this option.

Examples:

Convert `file1.csv` and `file2.csv' into `mydir/file1.parquet` and `mydir/file2.parquet` files.

  $ qsv to parquet mydir file1.csv file2.csv

Convert from stdin.

  $ qsv to parquet --pipe mydir -

DATAPACKAGE
Generate a datapackage, which contains stats and information about what is in the CSV files.

Examples:

Generate a `datapackage.json` file from `file1.csv` and `file2.csv' files.

  $ qsv to datapackage datapackage.json file1.csv file2.csv

Add more stats to datapackage.

  $ qsv to datapackage datapackage.json --stats file1.csv file2.csv

Generate a `datapackage.json` file from all the files in dir1

  $ qsv to datapackage datapackage.json dir1

For all other conversions you can output the datapackage created by specifying `--print-package`.

  $ qsv to xlsx datapackage.xlsx --stats --print-package file1.csv file2.csv

For more examples, see https://github.com/jqnatividad/qsv/blob/master/tests/test_to.rs.

Usage:
    qsv to postgres [options] <postgres> [<input>...]
    qsv to sqlite [options] <sqlite> [<input>...]
    qsv to xlsx [options] <xlsx> [<input>...]
    qsv to parquet [options] <parquet> [<input>...]
    qsv to datapackage [options] <datapackage> [<input>...]
    qsv to --help

To options:
    -k --print-package     Print statistics as datapackage, by default will print field summary.
    -u --dump              Create database dump file for use with `psql` or `sqlite3` command line tools (postgres/sqlite only).
    -a --stats             Produce extra statistics about the data beyond just type guessing.
    -c --stats-csv <path>  Output stats as CSV to specified file.
    -q --quiet             Do not print out field summary.
    -s --schema <arg>      The schema to load the data into. (postgres only).
    -d --drop              Drop tables before loading new data into them (postgres/sqlite only).
    -e --evolve            If loading into existing db, alter existing tables so that new data will load. (postgres/sqlite only).
    -i --pipe              For parquet, allow piping from stdin (using `-`) or from a named pipe.
    -p --separator <arg>   For xlsx, use this character to help truncate xlsx sheet names.
                           Defaults to space.
    -j, --jobs <arg>       The number of jobs to run in parallel.
                           When not set, the number of jobs is set to the number of CPUs detected.
                           
Common options:
    -h, --help             Display this message
    -d, --delimiter <arg>  The field delimiter for reading CSV data.
                           Must be a single character. (default: ,)
"#;

use std::{io::Write, path::PathBuf};

#[cfg(all(feature = "to_parquet", feature = "feature_capable"))]
use csvs_convert::csvs_to_parquet_with_options;
use csvs_convert::{
    csvs_to_postgres_with_options, csvs_to_sqlite_with_options, csvs_to_xlsx_with_options,
    make_datapackage, DescribeOptions, Options,
};
use log::debug;
use serde::Deserialize;

use crate::{
    config::{self, Delimiter},
    util,
    util::process_input,
    CliError, CliResult,
};

#[allow(dead_code)]
#[derive(Deserialize)]
struct Args {
    cmd_postgres:       bool,
    arg_postgres:       Option<String>,
    cmd_sqlite:         bool,
    arg_sqlite:         Option<String>,
    cmd_parquet:        bool,
    arg_parquet:        Option<String>,
    cmd_xlsx:           bool,
    arg_xlsx:           Option<String>,
    cmd_datapackage:    bool,
    arg_datapackage:    Option<String>,
    arg_input:          Vec<PathBuf>,
    flag_delimiter:     Option<Delimiter>,
    flag_schema:        Option<String>,
    flag_separator:     Option<String>,
    flag_dump:          bool,
    flag_drop:          bool,
    flag_evolve:        bool,
    flag_stats:         bool,
    flag_stats_csv:     Option<String>,
    flag_jobs:          Option<usize>,
    flag_print_package: bool,
    flag_quiet:         bool,
    flag_pipe:          bool,
}

impl From<csvs_convert::Error> for CliError {
    fn from(err: csvs_convert::Error) -> CliError {
        CliError::Other(format!("Conversion error: {err:?}"))
    }
}

impl From<csvs_convert::DescribeError> for CliError {
    fn from(err: csvs_convert::DescribeError) -> CliError {
        CliError::Other(format!("Conversion error: {err:?}"))
    }
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;
    debug!("'to' command running");
    let mut options = Options::builder()
        .delimiter(args.flag_delimiter.map(config::Delimiter::as_byte))
        .schema(args.flag_schema.unwrap_or_default())
        .seperator(args.flag_separator.unwrap_or_else(|| " ".into()))
        .evolve(args.flag_evolve)
        .stats(args.flag_stats)
        .pipe(args.flag_pipe)
        .stats_csv(args.flag_stats_csv.unwrap_or_default())
        .drop(args.flag_drop)
        .threads(util::njobs(args.flag_jobs))
        .build();

    let output;
    let mut arg_input = args.arg_input.clone();
    let tmpdir = tempfile::tempdir()?;

    if args.cmd_postgres {
        debug!("converting to postgres");
        arg_input = process_input(
            arg_input,
            &tmpdir,
            "No data on stdin. Need to add connection string as first argument then the input CSVs",
        )?;
        if args.flag_dump {
            options.dump_file = args.arg_postgres.expect("checked above");
            output = csvs_to_postgres_with_options(String::new(), arg_input, options)?;
        } else {
            output = csvs_to_postgres_with_options(
                args.arg_postgres.expect("checked above"),
                arg_input,
                options,
            )?;
        }
        debug!("conversion to postgres complete");
    } else if args.cmd_sqlite {
        debug!("converting to sqlite");
        arg_input = process_input(
            arg_input,
            &tmpdir,
            "No data on stdin. Need to add the name of a sqlite db as first argument then the \
             input CSVs",
        )?;
        if args.flag_dump {
            options.dump_file = args.arg_sqlite.expect("checked above");
            output = csvs_to_sqlite_with_options(String::new(), arg_input, options)?;
        } else {
            output = csvs_to_sqlite_with_options(
                args.arg_sqlite.expect("checked above"),
                arg_input,
                options,
            )?;
        }
        debug!("conversion to sqlite complete");
    } else if args.cmd_parquet {
        #[cfg(all(feature = "to_parquet", feature = "feature_capable"))]
        {
            debug!("converting to parquet");
            arg_input = process_input(
                arg_input,
                &tmpdir,
                "No data on stdin. Need to add the name of a parquet directory as first argument \
                 then the input CSVs",
            )?;
            output = csvs_to_parquet_with_options(
                args.arg_parquet.expect("checked above"),
                arg_input,
                options,
            )?;
            debug!("conversion to parquet complete");
        }
        #[cfg(not(all(feature = "to_parquet", feature = "feature_capable")))]
        {
            return fail_clierror!(
                "`to_parquet` feature disabled. `to parquet` subcommand not available."
            );
        }
    } else if args.cmd_xlsx {
        debug!("converting to xlsx");
        arg_input = process_input(
            arg_input,
            &tmpdir,
            "No data on stdin. Need to add the name of an xlsx file as first argument then the \
             input CSVs",
        )?;

        output =
            csvs_to_xlsx_with_options(args.arg_xlsx.expect("checked above"), arg_input, options)?;
        debug!("conversion to xlsx complete");
    } else if args.cmd_datapackage {
        debug!("creating datapackage");
        arg_input = process_input(
            arg_input,
            &tmpdir,
            "No data on stdin. Need to add the name of a datapackage file as first argument then \
             the input CSVs",
        )?;

        let describe_options = DescribeOptions::builder()
            .delimiter(options.delimiter)
            .stats(options.stats)
            .threads(options.threads)
            .stats_csv(options.stats_csv);
        output = make_datapackage(arg_input, PathBuf::new(), &describe_options.build())?;
        let file = std::fs::File::create(args.arg_datapackage.expect("checked above"))?;
        serde_json::to_writer_pretty(file, &output)?;
        debug!("datapackage complete");
    } else {
        return fail_clierror!(
            "Need to supply either xlsx,parquet,postgres,sqlite,datapackage as subcommand"
        );
    }

    if args.flag_print_package {
        println!(
            "{}",
            serde_json::to_string_pretty(&output).expect("values should be serializable")
        );
    } else if !args.flag_quiet && !args.flag_dump {
        let empty_array = vec![];
        for resource in output["resources"].as_array().unwrap_or(&empty_array) {
            let mut stdout = std::io::stdout();
            writeln!(&mut stdout)?;
            if args.flag_pipe {
                writeln!(
                    &mut stdout,
                    "Table '{}'",
                    resource["name"].as_str().unwrap_or("")
                )?;
            } else {
                writeln!(
                    &mut stdout,
                    "Table '{}' ({} rows)",
                    resource["name"].as_str().unwrap_or(""),
                    resource["row_count"].as_i64().unwrap_or(0)
                )?;
            }

            writeln!(&mut stdout)?;

            let mut tabwriter = tabwriter::TabWriter::new(stdout);

            if args.flag_pipe {
                writeln!(
                    &mut tabwriter,
                    "{}",
                    ["Field Name", "Field Type"].join("\t")
                )?;
            } else {
                writeln!(
                    &mut tabwriter,
                    "{}",
                    ["Field Name", "Field Type", "Field Format"].join("\t")
                )?;
            }

            for field in resource["schema"]["fields"]
                .as_array()
                .unwrap_or(&empty_array)
            {
                writeln!(
                    &mut tabwriter,
                    "{}",
                    [
                        field["name"].as_str().unwrap_or(""),
                        field["type"].as_str().unwrap_or(""),
                        field["format"].as_str().unwrap_or("")
                    ]
                    .join("\t")
                )?;
            }
            tabwriter.flush()?;
        }
        let mut stdout = std::io::stdout();
        writeln!(&mut stdout)?;
    }

    Ok(())
}
