static USAGE: &str = r#"
Convert non-nested JSON to CSV (polars feature only).

You may provide JSON data either from stdin or a file path.
This command may not work with nested JSON data.

As a basic example, say we have a file fruits.json with contents:

[
    {
        "fruit": "apple",
        "price": 2.5
    },
    {
        "fruit": "banana",
        "price": 3.0
    }
]

To convert it to CSV format, run:

qsv jsonp fruits.json

And the following is printed to the terminal:

fruit,price
apple,2.5
banana,3.0

If fruits.json was provided using stdin then either use - or do not provide a file path. For example:

cat fruits.json | qsv jsonp -

For more examples, see https://github.com/jqnatividad/qsv/blob/master/tests/test_jsonp.rs.

Usage:
    qsv jsonp [options] [<input>]
    qsv jsonp --help

jsonp options:
    --datetime-format <fmt>   The datetime format to use writing datetimes.
                              See https://docs.rs/chrono/latest/chrono/format/strftime/index.html
                              for the list of valid format specifiers.
    --date-format <fmt>       The date format to use writing dates.
    --time-format <fmt>       The time format to use writing times.
    --float-precision <arg>   The number of digits of precision to use when writing floats.
    --wnull-value <arg>       The string to use when WRITING null values.

Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
"#;

use std::io::{Cursor, Read, Seek, SeekFrom, Write};

use polars::prelude::*;
use serde::Deserialize;

use crate::{util, CliResult};

#[derive(Deserialize)]
struct Args {
    arg_input:            Option<String>,
    flag_datetime_format: Option<String>,
    flag_date_format:     Option<String>,
    flag_time_format:     Option<String>,
    flag_float_precision: Option<usize>,
    flag_wnull_value:     Option<String>,
    flag_output:          Option<String>,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;

    fn df_from_stdin() -> PolarsResult<DataFrame> {
        // Create a buffer in memory for stdin
        let mut buffer: Vec<u8> = Vec::new();
        let stdin = std::io::stdin();
        stdin.lock().read_to_end(&mut buffer)?;
        JsonReader::new(Box::new(std::io::Cursor::new(buffer))).finish()
    }

    fn df_from_path(path: String) -> PolarsResult<DataFrame> {
        JsonReader::new(std::fs::File::open(path)?).finish()
    }

    let df = match args.arg_input.clone() {
        Some(path) => {
            if path == "-" {
                df_from_stdin()?
            } else {
                df_from_path(path)?
            }
        },
        None => df_from_stdin()?,
    };

    fn df_to_csv<W: Write>(mut writer: W, mut df: DataFrame, args: &Args) -> PolarsResult<()> {
        CsvWriter::new(&mut writer)
            .with_datetime_format(args.flag_datetime_format.clone())
            .with_date_format(args.flag_date_format.clone())
            .with_time_format(args.flag_time_format.clone())
            .with_float_precision(args.flag_float_precision)
            .with_null_value(args.flag_wnull_value.clone().unwrap_or("".to_string()))
            .include_bom(util::get_envvar_flag("QSV_OUTPUT_BOM"))
            .finish(&mut df)?;
        Ok(())
    }

    if let Some(output_path) = args.flag_output.clone() {
        let mut output = std::fs::File::create(output_path)?;
        df_to_csv(&mut output, df, &args)?;
    } else {
        let mut res = Cursor::new(Vec::new());
        df_to_csv(&mut res, df, &args)?;
        res.seek(SeekFrom::Start(0))?;
        let mut out = String::new();
        res.read_to_string(&mut out)?;
        println!("{out}");
    }

    Ok(())
}
