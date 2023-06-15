static USAGE: &str = r#"
Run blazing-fast Polars SQL queries against several CSVs - replete with joins, aggregations,
grouping, sorting, and more - working on larger than memory CSV files.

Polars SQL is a subset of ANSI SQL, converting SQL queries to fast Polars LazyFrame expressions
(see https://pola-rs.github.io/polars-book/user-guide/sql/intro/).

For a list of SQL functions and keywords supported by Polars SQL, see
https://github.com/pola-rs/polars/blob/main/polars/polars-sql/src/functions.rs,
https://github.com/pola-rs/polars/blob/main/polars/polars-sql/src/keywords.rs and
https://github.com/pola-rs/polars/issues/7227

Returns the shape of the query result (number of rows, number of columns) to stderr.

Example queries:

  qsv sqlp data.csv 'select * from data where col1 > 10 order by col2 desc limit 20'

  qsv sqlp data.csv 'select col1, col2, col3 as friendlyname from data' --format parquet --output data.parquet

  qsv sqlp data.csv data2.csv 'select * from data join data2 on data.colname = data2.colname'

  qsv sqlp data.csv data2.csv 'select * from _t_1 join _t_2 on _t_1.colname = _t_2.colname'

  qsv sqlp data.csv 'SELECT col1, count(*) AS cnt FROM data GROUP BY col1 ORDER BY cnt DESC, col1 ASC'

  qsv sqlp data.csv data2.csv script.sql --format json --output data.json

  qsv sqlp data.csv "select col1, col2, col3 from data WHERE col1 = 'foo' AND col2 > 10"

  qsv sqlp data.csv "select data.col1, tbl2.col1 from data join read_parquet('data2.parquet') as tbl2 ON data.col1 = tbl2.col1"

For more examples, see https://github.com/jqnatividad/qsv/blob/master/tests/test_sqlp.rs.

Usage:
    qsv sqlp [options] <input>... <sql>
    qsv sqlp --help

sqlp arguments:
    input                  The CSV file/s to query. Use '-' for standard input.
                           If input is a directory, all CSV files in the directory will
                           be used.
                           If the input are snappy compressed file(s), it will be
                           decompressed automatically.
                           Column headers are required. Use 'qsv rename _all_generic --no-headers'
                           to add generic column names (_col_N) to a CSV with no headers.

    sql                    The SQL query/ies to run. Each input file will be available as a table
                           named after the file name (without the extension), or as "_t_N"
                           where N is the 1-based index.
                           If the input ends with ".sql", the input will be read as a SQL script file,
                           with each SQL statement separated by a semicolon. It will execute the
                           statements in order, and the result of the LAST statement will be returned.

sqlp options:
    --format <arg>            The output format to use. Valid values are:
                                csv      Comma-separated values
                                json     JSON
                                parquet  Apache Parquet
                                arrow    Apache Arrow IPC
                              (default: csv)

                              POLARS CSV PARSING OPTIONS:
    --try-parsedates          Automatically try to parse dates/datetimes and time.
                              If parsing fails, columns remain as strings.
    --infer-schema-len        The number of rows to scan when inferring the schema of the CSV.
                              Set to 0 to do a full table scan (warning: very slow).
                              (default: 1000)
    --low-memory              Use low memory mode when parsing CSVs. This will use less memory
                              but will be slower. It will also process LazyFrames in streaming mode.
                              Only use this when you are running out of memory parsing CSVs.
    --ignore-errors           Ignore errors when parsing CSVs. If set, rows with errors
                              will be skipped. If not set, the query will fail.
                              Only use this when debugging queries, as polars does batched
                              parsing and will skip the entire batch where the error occurred.

                              CSV OUTPUT FORMAT ONLY:
    --datetime-format <fmt>   The datetime format to use writing datetimes.
                              See https://docs.rs/chrono/latest/chrono/format/strftime/index.html
                              for the list of valid format specifiers.
    --date-format <fmt>       The date format to use writing dates.
    --time-format <fmt>       The time format to use writing times.
    --float-precision <arg>   The number of digits of precision to use when writing floats.
                              (default: 6)
    --null-value <arg>        The string to use when writing null values.
                              (default: <empty string>)

Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
    -d, --delimiter <arg>  The field delimiter for reading and writing CSV data.
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
    time::Instant,
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
    arg_input:             Vec<PathBuf>,
    arg_sql:               String,
    flag_format:           String,
    flag_try_parsedates:   bool,
    flag_infer_schema_len: usize,
    flag_low_memory:       bool,
    flag_ignore_errors:    bool,
    flag_datetime_format:  Option<String>,
    flag_date_format:      Option<String>,
    flag_time_format:      Option<String>,
    flag_float_precision:  Option<usize>,
    flag_null_value:       String,
    flag_output:           Option<String>,
    flag_delimiter:        Option<Delimiter>,
    flag_quiet:            bool,
}

#[derive(Debug, Default, Clone)]
enum OutputMode {
    #[default]
    Csv,
    Json,
    Parquet,
    Arrow,
    None,
}

// shamelessly copied from
// https://github.com/pola-rs/polars/blob/main/polars-cli/src/main.rs
impl OutputMode {
    fn execute_query(
        &self,
        query: &str,
        ctx: &mut SQLContext,
        delim: u8,
        datetime_format: Option<String>,
        date_format: Option<String>,
        time_format: Option<String>,
        float_precision: Option<usize>,
        null_value: String,
        output: Option<String>,
    ) -> CliResult<(usize, usize)> {
        let mut df = DataFrame::default();
        let execute_inner = || {
            df = ctx
                .execute(query)
                .and_then(polars::prelude::LazyFrame::collect)?;

            // we don't want to write anything if the output mode is None
            if matches!(self, OutputMode::None) {
                return Ok(());
            }

            let w = match output {
                Some(x) => {
                    let path = Path::new(&x);
                    Box::new(File::create(path)?) as Box<dyn Write>
                }
                None => Box::new(io::stdout()) as Box<dyn Write>,
            };
            let mut w = io::BufWriter::new(w);

            let out_result = match self {
                OutputMode::Csv => CsvWriter::new(&mut w)
                    .with_delimiter(delim)
                    .with_datetime_format(datetime_format)
                    .with_date_format(date_format)
                    .with_time_format(time_format)
                    .with_float_precision(float_precision)
                    .with_null_value(null_value)
                    .finish(&mut df),
                OutputMode::Json => JsonWriter::new(&mut w).finish(&mut df),
                OutputMode::Parquet => ParquetWriter::new(&mut w).finish(&mut df).map(|_| ()),
                OutputMode::Arrow => IpcWriter::new(&mut w).finish(&mut df),
                OutputMode::None => Ok(()),
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

    let null_value = if args.flag_null_value == "<empty string>" {
        String::new()
    } else {
        args.flag_null_value
    };

    let output_mode: OutputMode = args.flag_format.parse().unwrap_or(OutputMode::Csv);
    let no_output: OutputMode = OutputMode::None;

    let delim = if let Some(delimiter) = args.flag_delimiter {
        delimiter.as_byte()
    } else {
        match env::var("QSV_DEFAULT_DELIMITER") {
            Ok(delim) => Delimiter::decode_delimiter(&delim)?.as_byte(),
            _ => b',',
        }
    };

    let num_rows = if args.flag_infer_schema_len == 0 {
        None
    } else {
        Some(args.flag_infer_schema_len)
    };

    let optimize_all = polars::lazy::frame::OptState {
        projection_pushdown: true,
        predicate_pushdown:  true,
        type_coercion:       true,
        simplify_expr:       true,
        file_caching:        !args.flag_low_memory,
        slice_pushdown:      true,
        streaming:           args.flag_low_memory,
    };

    let mut ctx = SQLContext::new();
    let mut table_aliases = HashMap::with_capacity(args.arg_input.len());
    let mut table_ctr_suffix = 1_u8;

    for table in &arg_input {
        // as we are using the table name as alias, we need to make sure that the table name is a
        // valid identifier if its not utf8, we use the lossy version
        let lossy_table_name = table.to_string_lossy();
        let table_name = Path::new(table)
            .file_stem()
            .and_then(std::ffi::OsStr::to_str)
            .unwrap_or(&lossy_table_name);

        table_aliases.insert(table_name.to_string(), format!("_t_{table_ctr_suffix}"));

        table_ctr_suffix += 1;
        let lf = LazyCsvReader::new(table)
            .has_header(true)
            .with_missing_is_null(true)
            .with_delimiter(delim)
            .with_infer_schema_length(num_rows)
            .with_try_parse_dates(args.flag_try_parsedates)
            .with_ignore_errors(args.flag_ignore_errors)
            .low_memory(args.flag_low_memory)
            .finish()?;

        ctx.register(table_name, lf.with_optimizations(optimize_all));
    }

    if log::log_enabled!(log::Level::Debug) {
        let tables_in_context = ctx.get_tables();
        log::debug!("Table(s) registered in SQL Context: {tables_in_context:?}");
    }

    // check if the query is a SQL script
    let queries = if std::path::Path::new(&args.arg_sql)
        .extension()
        .map_or(false, |ext| ext.eq_ignore_ascii_case("sql"))
    {
        let mut file = File::open(&args.arg_sql)?;
        let mut sql_script = String::new();
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
    let mut now = Instant::now();

    for (idx, query) in queries.iter().enumerate() {
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

        if log::log_enabled!(log::Level::Debug) {
            log::debug!("Executing query {idx}: {current_query}");
            now = Instant::now();
        }
        query_result_shape = if is_last_query {
            // if this is the last query, we use the output mode specified by the user
            output_mode.execute_query(
                &current_query,
                &mut ctx,
                delim,
                args.flag_datetime_format.clone(),
                args.flag_date_format.clone(),
                args.flag_time_format.clone(),
                args.flag_float_precision,
                null_value.clone(),
                args.flag_output.clone(),
            )?
        } else {
            // this is not the last query, we only execute the query, but don't write the output
            no_output.execute_query(
                &current_query,
                &mut ctx,
                delim,
                args.flag_datetime_format.clone(),
                args.flag_date_format.clone(),
                args.flag_time_format.clone(),
                args.flag_float_precision,
                null_value.clone(),
                None,
            )?
        };
        if log::log_enabled!(log::Level::Debug) {
            log::debug!(
                "Query {idx} successfully executed in {elapsed:?} seconds: {query_result_shape:?}",
                elapsed = now.elapsed().as_secs_f32()
            );
        }
    }

    if let Some(output) = args.flag_output {
        // if the output ends with ".sz", we snappy compress the output
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

            // safety: we just created the tempfile, so we know that the path is valid utf8
            // https://github.com/Stebalien/tempfile/issues/192
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
