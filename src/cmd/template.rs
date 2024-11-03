static USAGE: &str = r#"
Renders a template using CSV data with the MiniJinja template engine.
https://docs.rs/minijinja/latest/minijinja/

Each CSV row is used to populate the template, with column headers used as variable names.
The template syntax follows the Jinja2 template language.

Example template:
  Dear {{ name }},
    Your account balance is {{ balance|format_float(2) }} with {{ points|human_count }} points!
    Status: {% if active|str_to_bool is true %}Active{% else %}Inactive{% endif %}

For examples, see https://github.com/jqnatividad/qsv/blob/master/tests/test_template.rs.

Usage:
    qsv template [options] [--template <str> | --template-file <file>] [<input>] [<outdir> | --output <file>]
    qsv template --help

template arguments:
    <input>                     The CSV file to read. If not given, input is read from STDIN.
    <outdir>                    The directory where the output files will be written.
                                If it does not exist, it will be created.
template options:
    --template <str>            Template string to use (alternative to --template-file)
    -t, --template-file <file>  Template file to use
    --outfilename <str>         Template string to use to create the filename of the output 
                                files to write to <outdir>. If set to just QSV_ROWNO, the filestem
                                is set to the current rowno of the record, padded with leading
                                zeroes, with the ".txt" extension (e.g. 001.txt, 002.txt, etc.)
                                Note that the QSV_ROWNO variable is also available in the context
                                if you want to use it in the filename template.
                                [default: QSV_ROWNO]

Common options:
    -h, --help                  Display this message
    -o, --output <file>         Write output to <file> instead of stdout
    -n, --no-headers            When set, the first row will not be interpreted
                                as headers. Templates must use numeric 1-based indices
                                with the "_c" prefix.(e.g. col1: {{_c1}} col2: {{_c2}})
    --delimiter <sep>           Field separator for reading CSV [default: ,]
"#;

use std::{
    fs,
    io::{BufWriter, Write},
};

use minijinja::Environment;
use serde::Deserialize;
use serde_json::Value;

use crate::{
    config::{Config, Delimiter},
    util, CliError, CliResult,
};

const QSV_ROWNO: &str = "QSV_ROWNO";

#[derive(Deserialize)]
struct Args {
    arg_input:     Option<String>,
    arg_outdir:    Option<String>,
    flag_template: Option<String>,

    flag_template_file: Option<String>,
    flag_output:        Option<String>,
    flag_outfilename:   String,
    flag_delimiter:     Option<Delimiter>,
    flag_no_headers:    bool,
}

impl From<minijinja::Error> for CliError {
    fn from(err: minijinja::Error) -> CliError {
        CliError::Other(err.to_string())
    }
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;

    // Get template content
    let template_content = match (args.flag_template_file, args.flag_template) {
        (Some(path), None) => fs::read_to_string(path)?,
        (None, Some(template)) => template,
        _ => return fail_clierror!("Must provide either --template or --template-string"),
    };

    // Set up minijinja environment
    let mut env = Environment::new();

    // Add custom filters
    env.add_filter("substr", substr);
    env.add_filter("format_float", format_float);
    env.add_filter("human_count", human_count);
    env.add_filter("human_float_count", human_float_count);
    env.add_filter("human_bytes", human_bytes);
    env.add_filter("round_num", round_num);
    env.add_filter("str_to_bool", str_to_bool);

    // Set up template
    env.add_template("template", &template_content)?;
    let template = env.get_template("template")?;

    // Set up CSV reader
    let rconfig = Config::new(args.arg_input.as_ref())
        .delimiter(args.flag_delimiter)
        .no_headers(args.flag_no_headers);

    let mut rdr = rconfig.reader()?;
    let headers = if args.flag_no_headers {
        csv::StringRecord::new()
    } else {
        rdr.headers()?.clone()
    };
    let context_capacity = if args.flag_no_headers {
        rdr.headers()?.len()
    } else {
        headers.len()
    };

    // Reuse context and pre-allocate
    let mut context = serde_json::Map::with_capacity(context_capacity);

    // Set up output handling
    let output_to_dir = args.arg_outdir.is_some();
    let mut row_number = 0_u64;
    let mut rowcount = 0;

    // Create filename environment once if needed
    let filename_env = if output_to_dir && args.flag_outfilename != QSV_ROWNO {
        let mut env = Environment::new();
        env.add_template("filename", &args.flag_outfilename)?;
        Some(env)
    } else {
        rowcount = util::count_rows(&rconfig)?;
        None
    };
    // Get width of rowcount for padding leading zeroes
    let width = rowcount.to_string().len();

    if output_to_dir {
        fs::create_dir_all(args.arg_outdir.as_ref().unwrap())?;
    }

    let mut wtr = if output_to_dir {
        None
    } else {
        Some(match args.flag_output {
            Some(file) => Box::new(BufWriter::new(fs::File::create(file)?)) as Box<dyn Write>,
            None => Box::new(BufWriter::new(std::io::stdout())) as Box<dyn Write>,
        })
    };

    let mut curr_record = csv::StringRecord::new();

    // Process each record
    for record in rdr.records() {
        row_number += 1;
        curr_record.clone_from(&record?);
        context.clear();

        if args.flag_no_headers {
            // Use numeric, column 1-based indices (e.g. _c1, _c2, etc.)
            for (i, field) in curr_record.iter().enumerate() {
                context.insert(format!("_c{}", i + 1), Value::String(field.to_string()));
            }
        } else {
            // Use header names
            for (header, field) in headers.iter().zip(curr_record.iter()) {
                context.insert(header.to_string(), Value::String(field.to_string()));
            }
        }
        // Always add row number to context
        context.insert(
            QSV_ROWNO.to_string(),
            Value::Number(serde_json::Number::from(row_number)),
        );

        // Render template with record data
        let rendered = template.render(&context)?;

        if output_to_dir {
            let outfilename = if args.flag_outfilename == QSV_ROWNO {
                // Pad row number with required number of leading zeroes
                format!("{row_number:0width$}.txt")
            } else {
                filename_env
                    .as_ref()
                    .unwrap()
                    .get_template("filename")?
                    .render(&context)?
            };
            let outpath = std::path::Path::new(args.arg_outdir.as_ref().unwrap()).join(outfilename);
            let mut writer = BufWriter::new(fs::File::create(outpath)?);
            write!(writer, "{rendered}")?;
        } else if let Some(ref mut w) = wtr {
            write!(w, "{rendered}")?;
        }
    }

    if let Some(mut w) = wtr {
        w.flush()?;
    }

    Ok(())
}

// CUSTOM FILTERS ===============================================

fn substr(value: &str, start: u32, end: Option<u32>) -> String {
    let end = end.unwrap_or(value.len() as _);
    value
        .get(start as usize..end as usize)
        .unwrap_or_default()
        .into()
}

fn format_float(value: &str, precision: u32) -> String {
    value
        .parse::<f64>()
        .map(|num| format!("{:.1$}", num, precision as usize))
        .unwrap_or_default()
}

fn human_count(value: &str) -> String {
    atoi_simd::parse::<u64>(value.as_bytes())
        .map(|num| indicatif::HumanCount(num).to_string())
        .unwrap_or_default()
}

fn human_float_count(value: &str) -> String {
    value
        .parse::<f64>()
        .map(|num| indicatif::HumanFloatCount(num).to_string())
        .unwrap_or_default()
}

fn human_bytes(value: &str) -> String {
    atoi_simd::parse::<u64>(value.as_bytes())
        .map(|num| indicatif::HumanBytes(num).to_string())
        .unwrap_or_default()
}

fn round_num(value: &str, places: u32) -> String {
    value
        .parse::<f64>()
        .map(|num| util::round_num(num, places))
        .unwrap_or_default()
}

fn str_to_bool(value: &str) -> bool {
    matches!(value.to_ascii_lowercase().as_str(), "true" | "1" | "yes")
}
