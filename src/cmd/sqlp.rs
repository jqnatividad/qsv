static USAGE: &str = r#"
Run a blazing-fast Polars SQL query against several CSVs - replete with joins, aggregations,
grouping, sorting, and more - working on larger than memory CSV files.

Polars SQL is a subset of ANSI SQL, converting SQL queries to fast Polars LazyFrame expressions
(see https://www.confessionsofadataguy.com/polars-laziness-and-sql-context/).

For a list of SQL functions and keywords supported by Polars SQL, see
https://github.com/pola-rs/polars/blob/main/polars/polars-sql/src/functions.rs,
https://github.com/pola-rs/polars/blob/main/polars/polars-sql/src/keywords.rs and
https://github.com/pola-rs/polars/issues/7227

Returns the shape of the query result (number of rows, number of columns) to stderr.

For examples, see https://github.com/jqnatividad/qsv/blob/master/tests/test_sqlp.rs.

Usage:
    qsv sqlp [options] <input>... <sql>
    qsv sqlp --help

sqlp arguments:
    input                  The CSV file(s) to query. Use '-' for standard input.
                           If input is a directory, all files in the directory will
                           be used.
                           If the input are snappy compressed file(s), it will be
                           decompressed automatically.
                           Column headers are required. Use 'qsv rename _all_generic --no-headers'
                           to add generic column names (_col_N) to a CSV with no headers.

    sql                    The SQL query to run. Each input file will be available as a table
                           named after the file name (without the extension), or as "_t_N"
                           where N is the 1-based index.
                           If the input ends with ".sql", the file will be read as a SQL script, with
                           each SQL statement separated by a semicolon. It will execute the statements
                           in order, and the result of the last statement will be returned.

sqlp options:
    --format <arg>         The output format to use. Valid values are:
                               csv      Comma-separated values
                               json     JSON
                               parquet  Apache Parquet
                               arrow    Apache Arrow IPC
                           (default: csv)
    --try-parsedates       Automatically try to parse dates/datetimes and time.
                           If parsing fails, columns remain as strings.

Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
    -d, --delimiter <arg>  The field delimiter for reading CSV data.
                           Must be a single character. (default: ,)
    -Q, --quiet            Do not return result shape to stderr.
"#;

use std::{
    collections::HashMap,
    env,
    fs::File,
    io,
    io::{BufWriter, Read, Write},
    path::{Path, PathBuf},
    str,
    str::FromStr,
    sync::OnceLock,
};

use polars::{
    prelude::{
        CsvWriter, DataFrame, IpcWriter, JsonWriter, LazyCsvReader, LazyFileListReader,
        ParquetWriter, SerWriter,
    },
    sql::SQLContext,
};
use serde::Deserialize;
use tempfile;

use crate::{
    cmd::snappy::compress,
    config::{Delimiter, DEFAULT_WTR_BUFFER_CAPACITY},
    util,
    util::process_input,
    CliResult,
};

#[derive(Deserialize, Debug)]
struct Args {
    arg_input:           Vec<PathBuf>,
    arg_sql:             String,
    flag_format:         String,
    flag_try_parsedates: bool,
    flag_output:         Option<String>,
    flag_delimiter:      Option<Delimiter>,
    flag_quiet:          bool,
}

static WTR_BUFFER_CAPACITY: OnceLock<usize> = OnceLock::new();

#[derive(Debug, Default, Clone)]
enum OutputMode {
    #[default]
    Csv,
    Json,
    Parquet,
    Arrow,
}

// shamelessly copied from
// https://github.com/pola-rs/polars/blob/main/polars-cli/src/main.rs
impl OutputMode {
    fn execute_query(
        &self,
        query: &str,
        ctx: &mut SQLContext,
        output: Option<String>,
    ) -> CliResult<(usize, usize)> {
        let mut df = DataFrame::default();
        let execute_inner = || {
            let w = match output {
                Some(x) => {
                    if x == "sink" {
                        Box::new(io::sink()) as Box<dyn Write>
                    } else {
                        let path = Path::new(&x);
                        Box::new(File::create(path)?) as Box<dyn Write>
                    }
                }
                None => Box::new(io::stdout()) as Box<dyn Write>,
            };

            df = ctx
                .execute(query)
                .and_then(polars::prelude::LazyFrame::collect)?;

            let wtr_buffer_capacity = WTR_BUFFER_CAPACITY.get().unwrap();
            let mut w = io::BufWriter::with_capacity(*wtr_buffer_capacity, w);

            let out_result = match self {
                OutputMode::Csv => CsvWriter::new(&mut w).finish(&mut df),
                OutputMode::Json => JsonWriter::new(&mut w).finish(&mut df),
                OutputMode::Parquet => ParquetWriter::new(&mut w).finish(&mut df).map(|_| ()),
                OutputMode::Arrow => IpcWriter::new(&mut w).finish(&mut df),
            };

            w.flush()?;
            out_result
        };

        match execute_inner() {
            Ok(_) => Ok(df.shape()),
            Err(e) => {
                fail_clierror!("Failed to execute query: {query}: {e}")
            }
        }
    }
}

impl FromStr for OutputMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "csv" => Ok(OutputMode::Csv),
            "json" => Ok(OutputMode::Json),
            "parquet" => Ok(OutputMode::Parquet),
            "arrow" => Ok(OutputMode::Arrow),
            _ => Err(format!("Invalid output mode: {s}")),
        }
    }
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;

    let mut arg_input = args.arg_input.clone();
    let tmpdir = tempfile::tempdir()?;
    arg_input = process_input(
        arg_input,
        &tmpdir,
        "No data on stdin. Please provide at least one input file or pipe data to stdin.",
    )?;

    let output_mode: OutputMode = args.flag_format.parse().unwrap_or(OutputMode::Csv);

    let delim = if let Some(delimiter) = args.flag_delimiter {
        delimiter.as_byte()
    } else {
        match env::var("QSV_DEFAULT_DELIMITER") {
            Ok(delim) => Delimiter::decode_delimiter(&delim).unwrap().as_byte(),
            _ => b',',
        }
    };

    let wtr_capacitys = env::var("QSV_WTR_BUFFER_CAPACITY")
        .unwrap_or_else(|_| DEFAULT_WTR_BUFFER_CAPACITY.to_string());
    let wtr_buffer_capacity: usize = wtr_capacitys.parse().unwrap_or(DEFAULT_WTR_BUFFER_CAPACITY);
    WTR_BUFFER_CAPACITY.set(wtr_buffer_capacity).unwrap();

    let optimize_all = polars::lazy::frame::OptState {
        projection_pushdown: true,
        predicate_pushdown:  true,
        type_coercion:       true,
        simplify_expr:       true,
        file_caching:        true,
        slice_pushdown:      true,
        streaming:           true,
    };

    let mut ctx = SQLContext::new();
    let mut table_aliases = HashMap::with_capacity(args.arg_input.len());
    let mut table_ctr_suffix = 1_u8;

    for table in &arg_input {
        let table_name = Path::new(table)
            .file_stem()
            .and_then(std::ffi::OsStr::to_str)
            .unwrap_or_else(|| table.to_str().unwrap());

        table_aliases.insert(table_name.to_string(), format!("_t_{table_ctr_suffix}"));

        table_ctr_suffix += 1;
        let lf = LazyCsvReader::new(table)
            .has_header(true)
            .with_missing_is_null(true)
            .with_delimiter(delim)
            .with_try_parse_dates(args.flag_try_parsedates)
            .finish()?;

        ctx.register(table_name, lf.with_optimizations(optimize_all));
    }

    if log::log_enabled!(log::Level::Debug) {
        let tables_in_context = ctx.get_tables();
        log::debug!("Table(s) registered in SQL Context: {tables_in_context:?}");
    }

    let mut sql_script = String::new();

    // check if the query is a SQL script
    let queries = if std::path::Path::new(&args.arg_sql)
        .extension()
        .map_or(false, |ext| ext.eq_ignore_ascii_case("sql"))
    {
        let mut file = File::open(&args.arg_sql)?;
        file.read_to_string(&mut sql_script)?;
        sql_script
            .split(';')
            .map(std::string::ToString::to_string)
            .filter(|s| s.trim().len() > 0)
            .collect()
    } else {
        // its not a sql script, just a single query
        vec![args.arg_sql.clone()]
    };

    log::debug!("Executing query/ies: {queries:?}");

    let num_queries = queries.len();
    let mut query_result_shape = (0_usize, 0_usize);

    for (idx, query) in queries.iter().enumerate() {
        // we need to clone the table aliases because we need to pass a mutable
        // reference to the query string
        let table_aliases = table_aliases.clone();

        // check if this is the last query in the script
        let is_last_query = idx == num_queries - 1;

        // replace aliases in query
        let mut current_query = query.to_string();
        for (table_name, table_alias) in &table_aliases {
            // we quote the table name to avoid issues with reserved keywords and
            // other characters that are not allowed in identifiers
            let quoted_table_name = format!(r#""{table_name}""#);
            current_query = current_query.replace(table_alias, &quoted_table_name);
        }

        log::debug!("Executing query {idx}: {query}");
        query_result_shape = if is_last_query {
            output_mode.execute_query(&current_query, &mut ctx, args.flag_output.clone())?
        } else {
            output_mode.execute_query(&current_query, &mut ctx, Some("sink".to_string()))?
        };
        log::debug!("Query {idx} successfully executed! result shape: {query_result_shape:?}");
    }

    if let Some(output) = args.flag_output {
        // if output ends with .sz, we snappy compress the output
        if std::path::Path::new(&output)
            .extension()
            .map_or(false, |ext| ext.eq_ignore_ascii_case("sz"))
        {
            log::debug!("Compressing output with Snappy");

            // we need to copy the output to a tempfile first, and then
            // compress the tempfile to the original output sz file
            let mut tempfile = tempfile::NamedTempFile::new()?;
            io::copy(&mut File::open(output.clone())?, tempfile.as_file_mut())?;
            tempfile.flush()?;

            let input_fname = tempfile.path().to_str().unwrap();
            let input = File::open(input_fname)?;
            let output_sz_writer = BufWriter::with_capacity(
                DEFAULT_WTR_BUFFER_CAPACITY,
                std::fs::File::create(output)?,
            );
            compress(input, output_sz_writer, util::max_jobs())?;
        }
    }

    if !args.flag_quiet {
        eprintln!("{query_result_shape:?}");
    }

    Ok(())
}
