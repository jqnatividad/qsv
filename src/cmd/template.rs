static USAGE: &str = r#"
Renders a template using CSV data with the minijinja template engine.
https://docs.rs/minijinja/latest/minijinja/

Each CSV row is used to populate the template, with column headers used as variable names.
The template syntax follows the Jinja2 template language.

Example template:
    Dear {{ name }},
    Your account balance is {{ balance | format_float(precision=2) }}.
    Status: {{ if active }}Active{{ else }}Inactive{{ endif }}

Usage:
    qsv template [options] [--template <str> | --template-file <file>] [<input>] [<outdir> | --output <file>]
    qsv template --help

template arguments:
    <input>                 The CSV file to read. If not given, input is read from STDIN.
    <outdir>                The directory where the output files will be written.
                            If it does not exist, it will be created.
template options:
    --template <str>        Template string to use (alternative to --template-file)
    --template-file <file>  Template file to use
    --outfilename <str>     Template string to use to create the filestem of the output 
                            files to write to <outdir>. If set to ROWNO, the filestem
                            is set to the current rowno of the record, padded with leading
                            zeroes, with the ".txt" extension (e.g. 001.txt, 002.txt, etc.)
                            [default: ROWNO]
    -n, --no-headers        When set, the first row will not be interpreted
                            as headers. Templates must use numeric 1-based indices
                            with the "_c" prefix.(e.g. col1: {{_c1}} col2: {{_c2}})

Common options:
    -o, --output <file>     Write output to <file> instead of stdout
    --delimiter <sep>       Field separator for reading CSV [default: ,]
    -h, --help              Display this message
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

    // Set up output handling
    let output_to_dir = args.arg_outdir.is_some();
    let mut row_number = 0_u64;
    let mut rowcount = 0;

    // Create filename environment once if needed
    let filename_env = if output_to_dir && args.flag_outfilename != "ROWNO" {
        let mut env = Environment::new();
        env.add_template("filename", &args.flag_outfilename)?;
        Some(env)
    } else {
        rowcount = util::count_rows(&rconfig)?;
        None
    };

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
        let mut context = serde_json::Map::with_capacity(curr_record.len());

        if args.flag_no_headers {
            // Use numeric indices
            for (i, field) in curr_record.iter().enumerate() {
                context.insert(format!("_c{}", i + 1), Value::String(field.to_string()));
            }
        } else {
            // Use header names
            for (header, field) in headers.iter().zip(curr_record.iter()) {
                context.insert(header.to_string(), Value::String(field.to_string()));
            }
        }

        // Render template with record data
        let rendered = template.render(&context)?;

        if output_to_dir {
            let outfilename = if args.flag_outfilename == "ROWNO" {
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
