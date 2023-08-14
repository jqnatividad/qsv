static USAGE: &str = r#"
Run blazing-fast Polars SQL queries against several CSVs - replete with joins, aggregations,
grouping, sorting, and more - working on larger than memory CSV files.

Polars SQL is a SQL dialect, converting SQL queries to fast Polars LazyFrame expressions
(see https://pola-rs.github.io/polars-book/user-guide/sql/intro/).

For a list of SQL functions and keywords supported by Polars SQL, see
https://github.com/pola-rs/polars/blob/main/polars/polars-sql/src/functions.rs,
https://github.com/pola-rs/polars/blob/main/polars/polars-sql/src/keywords.rs and
https://github.com/pola-rs/polars/issues/7227

Returns the shape of the query result (number of rows, number of columns) to stderr.

Example queries:

  qsv sqlp data.csv 'select * from data where col1 > 10 order by col2 desc limit 20'

  qsv sqlp data.csv 'select col1, col2 as friendlyname from data' --format parquet --output data.parquet

  qsv sqlp data.csv data2.csv 'select * from data join data2 on data.colname = data2.colname'

  qsv sqlp data.csv data2.csv 'select * from _t_1 join _t_2 on _t_1.colname = _t_2.colname'

  qsv sqlp data.csv 'SELECT col1, count(*) AS cnt FROM data GROUP BY col1 ORDER BY cnt DESC, col1 ASC'

  qsv sqlp data.csv "select lower(col1), substr(col2, 2, 4) from data WHERE starts_with(col1, 'foo')"

  # Use a SQL script to run a long, complex SQL query or to run SEVERAL SQL queries.
  # When running several queries, each query needs to be separated by a semicolon,
  # the last query will be returned as the result.
  # Typically, earlier queries are used to create tables that can be used in later queries.
  # See test_sqlp/sqlp_boston311_sql_script() for an example.
  qsv sqlp data.csv data2.csv data3.csv data4.csv script.sql --format json --output data.json

  # use Common Table Expressions (CTEs) using WITH to simplify complex queries
  qsv sqlp people.csv "WITH millenials AS (SELECT * FROM people WHERE age >= 25 and age <= 40) \
    SELECT * FROM millenials WHERE STARTS_WITH(name,'C')"

  # spaceship operator: "<=>" (three-way comparison operator)
  #  returns -1 if left < right, 0 if left == right, 1 if left > right
  # https://en.wikipedia.org/wiki/Three-way_comparison#Spaceship_operator
  qsv sqlp data.csv data2.csv "select data.c2 <=> data2.c2 from data join data2 on data.c1 = data2.c1"

  # regex operators: "~" (contains pattern, case-sensitive); "~*" (contains pattern, case-insensitive)
  #   "!~" (does not contain pattern, case-sensitive); "!~*" (does not contain pattern, case-insensitive)
    qsv sqlp data.csv "select * from data WHERE col1 ~ '^foo' AND col2 > 10"
    qsv sqlp data.csv "select * from data WHERE col1 !~* 'bar$' AND col2 > 10"

  # regexp_like function: regexp_like(<string>, <pattern>, <optional flags>)
  # returns true if <string> matches <pattern>, false otherwise
  #   <optional flags> can be one or more of the following:
  #   'c' (case-sensitive - default), 'i' (case-insensitive), 'm' (multiline)
  qsv sqlp data.csv "select * from data WHERE regexp_like(col1, '^foo') AND col2 > 10"
  # case-insensitive regexp_like
  qsv sqlp data.csv "select * from data WHERE regexp_like(col1, '^foo', 'i') AND col2 > 10"

  # use Parquet, JSONL and Arrow files in SQL queries
  qsv sqlp data.csv "select * from data join read_parquet('data2.parquet') as t2 ON data.c1 = t2.c1"
  qsv sqlp data.csv "select * from data join read_ndjson('data2.jsonl') as t2 on data.c1 = t2.c1"
  qsv sqlp data.csv "select * from data join read_ipc('data2.arrow') as t2 ON data.c1 = t2.c1"

  # use stdin as input
  cat data.csv | qsv sqlp - 'select * from stdin'
  cat data.csv | qsv sqlp - data2.csv 'select * from stdin join data2 on stdin.col1 = data2.col1'

  # automatic snappy decompression/compression
  qsv sqlp data.csv.sz 'select * from data where col1 > 10' --output result.csv.sz

  # explain query plan
  qsv sqlp data.csv 'explain select * from data where col1 > 10 order by col2 desc limit 20'

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
                           with each SQL query separated by a semicolon. It will execute the queries
                           in order, and the result of the LAST query will be returned as the result.

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
    --infer-len <arg>         The number of rows to scan when inferring the schema of the CSV.
                              Set to 0 to do a full table scan (warning: very slow).
                              (default: 250)
    --low-memory              Use low memory mode when parsing CSVs. This will use less memory
                              but will be slower. It will also process LazyFrames in streaming mode.
                              Only use this when you get out of memory errors.
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

                              PARQUET OUTPUT FORMAT ONLY:
    --compression <arg>       The compression codec to use when writing parquet files.
                                Valid values are: zstd, lz4raw, gzip, snappy, uncompressed
                              (default: zstd)
    --compress-level <arg>    The compression level to use when using zstd or gzip compression.
                              When using zstd, valid values are -7 to 22, with -7 being the
                              lowest compression level and 22 being the highest compression level.
                              When using gzip, valid values are 1-9, with 1 being the lowest
                              compression level and 9 being the highest compression level.
                              Higher compression levels are slower.
                              The zstd default is 3, and the gzip default is 6.
    --statistics              Compute column statistics when writing parquet files.
    
Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
    -d, --delimiter <arg>  The field delimiter for reading and writing CSV data.
                           Must be a single character. (default: ,)
    -Q, --quiet            Do not return result shape to stderr.
"#;

use std::{
    borrow::Cow,
    collections::HashMap,
    env,
    fs::File,
    io,
    io::{BufWriter, Read, Write},
    path::{Path, PathBuf},
    str::FromStr,
    time::Instant,
};

use polars::{
    prelude::{
        CsvWriter, DataFrame, GzipLevel, IpcWriter, JsonWriter, LazyCsvReader, LazyFileListReader,
        ParquetCompression, ParquetWriter, SerWriter, ZstdLevel,
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

static DEFAULT_GZIP_COMPRESSION_LEVEL: u8 = 6;
static DEFAULT_ZSTD_COMPRESSION_LEVEL: i32 = 3;

#[derive(Deserialize, Clone)]
struct Args {
    arg_input:            Vec<PathBuf>,
    arg_sql:              String,
    flag_format:          String,
    flag_try_parsedates:  bool,
    flag_infer_len:       usize,
    flag_low_memory:      bool,
    flag_ignore_errors:   bool,
    flag_datetime_format: Option<String>,
    flag_date_format:     Option<String>,
    flag_time_format:     Option<String>,
    flag_float_precision: Option<usize>,
    flag_null_value:      String,
    flag_compression:     String,
    flag_compress_level:  Option<i32>,
    flag_statistics:      bool,
    flag_output:          Option<String>,
    flag_delimiter:       Option<Delimiter>,
    flag_quiet:           bool,
}

#[derive(Default, Clone)]
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
        args: Args,
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

            let w = match args.flag_output {
                Some(x) => {
                    let path = Path::new(&x);
                    Box::new(File::create(path)?) as Box<dyn Write>
                },
                None => Box::new(io::stdout()) as Box<dyn Write>,
            };
            let mut w = io::BufWriter::with_capacity(256_000, w);

            let out_result = match self {
                OutputMode::Csv => CsvWriter::new(&mut w)
                    .with_delimiter(delim)
                    .with_datetime_format(args.flag_datetime_format)
                    .with_date_format(args.flag_date_format)
                    .with_time_format(args.flag_time_format)
                    .with_float_precision(args.flag_float_precision)
                    .with_null_value(args.flag_null_value)
                    .finish(&mut df),
                OutputMode::Json => JsonWriter::new(&mut w).finish(&mut df),
                OutputMode::Parquet => {
                    let compression: PqtCompression = args
                        .flag_compression
                        .parse()
                        .unwrap_or(PqtCompression::Lz4Raw);

                    let parquet_compression = match compression {
                        PqtCompression::Uncompressed => ParquetCompression::Uncompressed,
                        PqtCompression::Snappy => ParquetCompression::Snappy,
                        PqtCompression::Lz4Raw => ParquetCompression::Lz4Raw,
                        PqtCompression::Gzip => {
                            let gzip_level = args
                                .flag_compress_level
                                .unwrap_or_else(|| DEFAULT_GZIP_COMPRESSION_LEVEL.into())
                                as u8;
                            ParquetCompression::Gzip(Some(GzipLevel::try_new(gzip_level)?))
                        },
                        PqtCompression::Zstd => {
                            let zstd_level = args
                                .flag_compress_level
                                .unwrap_or(DEFAULT_ZSTD_COMPRESSION_LEVEL);
                            ParquetCompression::Zstd(Some(ZstdLevel::try_new(zstd_level)?))
                        },
                    };

                    ParquetWriter::new(&mut w)
                        .with_row_group_size(Some(768 ^ 2))
                        .with_statistics(args.flag_statistics)
                        .with_compression(parquet_compression)
                        .finish(&mut df)
                        .map(|_| ())
                },
                OutputMode::Arrow => IpcWriter::new(&mut w).finish(&mut df),
                OutputMode::None => Ok(()),
            };

            w.flush()?;
            out_result
        };

        match execute_inner() {
            Ok(()) => Ok(df.shape()),
            Err(e) => {
                fail_clierror!("Failed to execute query: {query}: {e}")
            },
        }
    }
}

impl FromStr for OutputMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "csv" => Ok(OutputMode::Csv),
            "json" => Ok(OutputMode::Json),
            "parquet" => Ok(OutputMode::Parquet),
            "arrow" => Ok(OutputMode::Arrow),
            _ => Err(format!("Invalid output mode: {s}")),
        }
    }
}

#[derive(Default, Copy, Clone)]
enum PqtCompression {
    Uncompressed,
    Gzip,
    Snappy,
    #[default]
    Zstd,
    Lz4Raw,
}

impl FromStr for PqtCompression {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "uncompressed" => Ok(PqtCompression::Uncompressed),
            "gzip" => Ok(PqtCompression::Gzip),
            "snappy" => Ok(PqtCompression::Snappy),
            "lz4raw" => Ok(PqtCompression::Lz4Raw),
            "zstd" => Ok(PqtCompression::Zstd),
            _ => Err(format!("Invalid compression format: {s}")),
        }
    }
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let mut args: Args = util::get_args(USAGE, argv)?;

    let tmpdir = tempfile::tempdir()?;
    args.arg_input = process_input(
        args.arg_input,
        &tmpdir,
        "No data on stdin. Please provide at least one input file or pipe data to stdin.",
    )?;

    if args.flag_null_value == "<empty string>" {
        args.flag_null_value.clear();
    };

    let output_mode: OutputMode = args.flag_format.parse().unwrap_or(OutputMode::Csv);
    let no_output: OutputMode = OutputMode::None;

    let delim = if let Some(delimiter) = args.flag_delimiter {
        delimiter.as_byte()
    } else if let Ok(delim) = env::var("QSV_DEFAULT_DELIMITER") {
        Delimiter::decode_delimiter(&delim)?.as_byte()
    } else {
        b','
    };

    let num_rows = if args.flag_infer_len == 0 {
        None
    } else {
        Some(args.flag_infer_len)
    };

    let optimize_all = polars::lazy::frame::OptState {
        projection_pushdown: true,
        predicate_pushdown:  true,
        type_coercion:       true,
        simplify_expr:       true,
        file_caching:        !args.flag_low_memory,
        slice_pushdown:      true,
        comm_subplan_elim:   true,
        comm_subexpr_elim:   true,
        streaming:           args.flag_low_memory,
    };

    let mut ctx = SQLContext::new();
    let mut table_aliases = HashMap::with_capacity(args.arg_input.len());
    let mut lossy_table_name = Cow::default();
    let mut table_name;

    for (idx, table) in args.arg_input.iter().enumerate() {
        // as we are using the table name as alias, we need to make sure that the table name is a
        // valid identifier. if its not utf8, we use the lossy version
        table_name = Path::new(table)
            .file_stem()
            .and_then(std::ffi::OsStr::to_str)
            .unwrap_or_else(|| {
                lossy_table_name = table.to_string_lossy();
                &lossy_table_name
            });
        table_aliases.insert(table_name.to_string(), format!("_t_{}", idx + 1));

        if log::log_enabled!(log::Level::Debug) {
            log::debug!(
                "Registering table: {table_name} as {alias} -  Delimiter: {delim} \
                 Infer_schema_len: {num_rows:?} try_parse_dates: {parse_dates} ignore_errors: \
                 {ignore_errors}, low_memory: {low_memory}",
                alias = table_aliases.get(table_name).unwrap(),
                parse_dates = args.flag_try_parsedates,
                ignore_errors = args.flag_ignore_errors,
                low_memory = args.flag_low_memory
            );
        }
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
    let last_query: usize = num_queries.saturating_sub(1);
    let mut is_last_query;
    let mut current_query = String::new();
    let mut query_result_shape = (0_usize, 0_usize);
    let mut now = Instant::now();

    for (idx, query) in queries.iter().enumerate() {
        // check if this is the last query in the script
        is_last_query = idx == last_query;

        // replace aliases in query
        current_query.clone_from(query);
        for (table_name, table_alias) in &table_aliases {
            // we quote the table name to avoid issues with reserved keywords and
            // other characters that are not allowed in identifiers
            current_query = current_query.replace(table_alias, &(format!(r#""{table_name}""#)));
        }

        if log::log_enabled!(log::Level::Debug) {
            log::debug!("Executing query {idx}: {current_query}");
            now = Instant::now();
        }
        query_result_shape = if is_last_query {
            // if this is the last query, we use the output mode specified by the user
            output_mode.execute_query(&current_query, &mut ctx, delim, args.clone())?
        } else {
            // this is not the last query, we only execute the query, but don't write the output
            no_output.execute_query(&current_query, &mut ctx, delim, args.clone())?
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
