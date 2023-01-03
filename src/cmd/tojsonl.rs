static USAGE: &str = r#"
Smartly converts CSV to a newline-delimited JSON (JSONL/NDJSON).

By scanning the CSV first, it "smartly" infers the appropriate JSON data type
for each column.

For examples, see https://github.com/jqnatividad/qsv/blob/master/tests/test_tojsonl.rs.

Usage:
    qsv tojsonl [options] [<input>]
    qsv tojsonl --help

Tojsonl optionns:
    -j, --jobs <arg>       The number of jobs to run in parallel.
                           When not set, the number of jobs is set to the
                           number of CPUs detected.

Common options:
    -h, --help             Display this message
    -d, --delimiter <arg>  The field delimiter for reading CSV data.
                           Must be a single character. (default: ,)
    -o, --output <file>    Write output to <file> instead of stdout.
"#;

use std::{env::temp_dir, fs::File, path::Path};

use serde::Deserialize;
use serde_json::{Map, Value};
use uuid::Uuid;

use super::schema::infer_schema_from_stats;
use crate::{
    config::{Config, Delimiter},
    util, CliResult,
};

#[derive(Deserialize, Clone)]
struct Args {
    arg_input:      Option<String>,
    flag_jobs:      Option<usize>,
    flag_delimiter: Option<Delimiter>,
    flag_output:    Option<String>,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let preargs: Args = util::get_args(USAGE, argv)?;
    let mut args = preargs.clone();
    let conf = Config::new(&args.arg_input).delimiter(args.flag_delimiter);
    let mut is_stdin = false;

    let stdin_fpath = format!("{}/{}.csv", temp_dir().to_string_lossy(), Uuid::new_v4());
    let stdin_temp = stdin_fpath.clone();

    // if using stdin, we create a stdin.csv file as stdin is not seekable and we need to
    // open the file multiple times to compile stats/unique values, etc.
    let input_filename = if preargs.arg_input.is_none() {
        let mut stdin_file = File::create(stdin_fpath.clone())?;
        let stdin = std::io::stdin();
        let mut stdin_handle = stdin.lock();
        std::io::copy(&mut stdin_handle, &mut stdin_file)?;
        args.arg_input = Some(stdin_fpath.clone());
        is_stdin = true;
        stdin_fpath
    } else {
        let filename = Path::new(args.arg_input.as_ref().unwrap())
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string();
        filename
    };
    // we're calling the schema command to infer data types and enums
    let schema_args = crate::cmd::schema::Args {
        // we only do three, as we're only inferring boolean based on enum
        flag_enum_threshold:  3,
        flag_strict_dates:    false,
        flag_pattern_columns: crate::select::SelectColumns::parse("")?,
        // json doesn't have a date type, so don't infer dates
        flag_dates_whitelist: "none".to_string(),
        flag_prefer_dmy:      false,
        flag_stdout:          false,
        flag_jobs:            Some(util::njobs(args.flag_jobs)),
        flag_no_headers:      false,
        flag_delimiter:       args.flag_delimiter,
        arg_input:            args.arg_input.clone(),
    };
    // build schema for each field by their inferred type, min/max value/length, and unique values
    let properties_map: Map<String, Value> =
        match infer_schema_from_stats(&schema_args, &input_filename) {
            Ok(map) => map,
            Err(e) => {
                return fail_clierror!("Failed to infer field types via stats and frequency: {e}");
            }
        };

    let mut rdr = if is_stdin {
        Config::new(&Some(stdin_temp))
            .delimiter(args.flag_delimiter)
            .reader()?
    } else {
        conf.reader()?
    };
    let mut wtr = Config::new(&args.flag_output)
        .flexible(true)
        .no_headers(true)
        .quote_style(csv::QuoteStyle::Never)
        .writer()?;

    let headers = rdr.headers()?.clone();
    let mut record = csv::StringRecord::new();

    // create a vec lookup about inferred field data types
    let mut field_type_vec: Vec<String> = Vec::with_capacity(headers.len());
    for (_field_name, field_def) in properties_map.iter() {
        let Some(field_map) = field_def.as_object() else { return fail!("Cannot create field map") };
        let prelim_type = field_map.get("type").unwrap();
        let field_values_enum = field_map.get("enum");

        // check if a field has a boolean data type
        if let Some(values) = field_values_enum {
            let vals = values.as_array().unwrap();
            if vals.len() == 2 {
                let val1 = vals[0].as_str().unwrap().to_string().to_lowercase();
                let val2 = vals[1].as_str().unwrap().to_string().to_lowercase();
                if let ("true", "false")
                | ("false", "true")
                | ("1", "0")
                | ("0", "1")
                | ("yes", "no")
                | ("no", "yes") = (val1.as_str(), val2.as_str())
                {
                    field_type_vec.push("boolean".to_string());
                    continue;
                }
            }
        }
        // its not a boolean, so its a Number, String or Null
        let temp_type = prelim_type.clone();
        let temp_string = temp_type.as_array().unwrap()[0]
            .as_str()
            .unwrap()
            .to_string();
        if temp_string == "integer" {
            field_type_vec.push("number".to_string());
        } else {
            field_type_vec.push(temp_string);
        }
    }

    // amortize allocs
    #[allow(unused_assignments)]
    let mut temp_str = String::with_capacity(100);

    // write jsonl file
    while rdr.read_record(&mut record)? {
        use std::fmt::Write as _;

        temp_str.clear();
        let _ = write!(temp_str, "{{");
        for (idx, field) in record.iter().enumerate() {
            let field_val = match field_type_vec[idx].as_str() {
                "string" => format!(r#""{}""#, field.escape_default()),
                "number" => field.to_string(),
                "boolean" => match field.to_lowercase().as_str() {
                    "true" | "yes" | "1" => "true".to_string(),
                    _ => "false".to_string(),
                },
                "null" => "null".to_string(),
                _ => "unknown".to_string(),
            };
            let _ = write!(temp_str, r#""{}":{field_val},"#, &headers[idx]);
        }
        temp_str.pop(); // remove last comma
        temp_str.push('}');
        record.clear();
        record.push_field(&temp_str);
        wtr.write_record(&record)?;
    }

    Ok(wtr.flush()?)
}
