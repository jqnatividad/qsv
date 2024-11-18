static USAGE: &str = r#"
Renders a template using CSV data with the MiniJinja template engine.
https://docs.rs/minijinja/latest/minijinja/

This command processes each row of the CSV file, making the column values available as variables.
Each row is rendered using the template. Column headers become variable names, with non-alphanumeric
characters converted to underscore (_).

Templates use Jinja2 syntax (https://jinja.palletsprojects.com/en/stable/templates/) 
and can access an extensive library of built-in filters/functions, with additional ones
from minijinja_contrib https://docs.rs/minijinja-contrib/latest/minijinja_contrib/.
Additional qsv custom filters are also documented at the end of this file.

If the <outdir> argument is specified, it will create a file for each row in <outdir>, with
the filename rendered using --outfilename option.
Otherwise, ALL the rendered rows will be sent to STDOUT or the designated --output. 

Example:
data.csv:
  "first name","last name",balance,"loyalty points",active
  alice,jones,100.50,1000,true
  bob,smith,200.75,2000,false
  john,doe,10,1,true

template.tpl
  Dear {{ first_name|title }} {{ last_name|title }}!
    Your account balance is {{ balance|format_float(2) }}
       with {{ loyalty_points|human_count }} point{{ loyalty_points|int|pluralize }}!
    {# This is a comment and will not be rendered. The closing minus sign in this
       block tells MiniJinja to trim whitespaces -#}
    {% set loyalty_value = loyalty_points|int / 100 -%}
    Value of Points: {{ loyalty_value }} 
    Final Balance: {{ balance|int - loyalty_value }}
    Status: {% if active|str_to_bool is true %}Active{% else %}Inactive{% endif %}

qsv template --template-file template.tpl data.csv

For more examples, see https://github.com/jqnatividad/qsv/blob/master/tests/test_template.rs.
For a relatively complex MiniJinja template, see https://github.com/jqnatividad/qsv/blob/master/scripts/template.tpl

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
                                Note that all the fields, including QSV_ROWNO, are available
                                when defining the filename template.
                                [default: QSV_ROWNO]
    --customfilter-error <msg>  The value to return when a custom filter returns an error.
                                Use "<empty string>" to return an empty string.
                                [default: <FILTER_ERROR>]
    -j, --jobs <arg>            The number of jobs to run in parallel.
                                When not set, the number of jobs is set to the number of CPUs detected.
    -b, --batch <size>          The number of rows per batch to load into memory, before running in parallel.
                                Set to 0 to load all rows in one batch.
                                [default: 50000]

Common options:
    -h, --help                  Display this message
    -o, --output <file>         Write output to <file> instead of stdout
    -n, --no-headers            When set, the first row will not be interpreted
                                as headers. Templates must use numeric 1-based indices
                                with the "_c" prefix.(e.g. col1: {{_c1}} col2: {{_c2}})
    --delimiter <sep>           Field separator for reading CSV [default: ,]
    -p, --progressbar           Show progress bars. Not valid for stdin.
"#;

use std::{
    fs,
    io::{BufWriter, Write},
    sync::OnceLock,
};

use indicatif::{ProgressBar, ProgressDrawTarget};
use minijinja::Environment;
use minijinja_contrib::pycompat::unknown_method_callback;
use rayon::{
    iter::{IndexedParallelIterator, ParallelIterator},
    prelude::IntoParallelRefIterator,
};
use serde::Deserialize;
use simd_json::BorrowedValue;

use crate::{
    config::{Config, Delimiter, DEFAULT_WTR_BUFFER_CAPACITY},
    util, CliError, CliResult,
};

const QSV_ROWNO: &str = "QSV_ROWNO";

#[derive(Deserialize)]
struct Args {
    arg_input:               Option<String>,
    arg_outdir:              Option<String>,
    flag_template:           Option<String>,
    flag_template_file:      Option<String>,
    flag_output:             Option<String>,
    flag_outfilename:        String,
    flag_customfilter_error: String,
    flag_jobs:               Option<usize>,
    flag_batch:              usize,
    flag_delimiter:          Option<Delimiter>,
    flag_no_headers:         bool,
    flag_progressbar:        bool,
}

static FILTER_ERROR: OnceLock<String> = OnceLock::new();

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
        _ => {
            return fail_incorrectusage_clierror!(
                "Must provide either --template or --template-file"
            )
        },
    };

    // Initialize FILTER_ERROR from args.flag_customfilter_error
    if FILTER_ERROR
        .set(if args.flag_customfilter_error == "<empty string>" {
            String::new()
        } else {
            args.flag_customfilter_error
        })
        .is_err()
    {
        return fail!("Cannot initialize custom filter error message.");
    }

    // Set up minijinja environment
    let mut env = Environment::new();

    // Add minijinja_contrib functions/filters
    // see https://docs.rs/minijinja-contrib/latest/minijinja_contrib/
    minijinja_contrib::add_to_environment(&mut env);
    env.set_unknown_method_callback(unknown_method_callback);

    // Add our own custom filters
    env.add_filter("substr", substr);
    env.add_filter("format_float", format_float);
    env.add_filter("human_count", human_count);
    env.add_filter("human_float_count", human_float_count);
    env.add_filter("round_num", round_num);
    env.add_filter("str_to_bool", str_to_bool);
    // TODO: Add lookup filter

    // Set up template
    env.add_template("template", &template_content)?;
    let template = env.get_template("template")?;

    // Set up CSV reader
    let rconfig = Config::new(args.arg_input.as_ref())
        .delimiter(args.flag_delimiter)
        .no_headers(args.flag_no_headers);
    let mut rdr = rconfig.reader()?;

    // read headers
    let headers = if args.flag_no_headers {
        csv::StringRecord::new()
    } else {
        let headers = rdr.headers()?.clone();
        let mut sanitized_headers: Vec<String> = headers
            .iter()
            .map(|h| {
                h.chars()
                    .map(|c| if c.is_alphanumeric() { c } else { '_' })
                    .collect()
            })
            .collect();
        // add a column named QSV_ROWNO at the end
        sanitized_headers.push(QSV_ROWNO.to_owned());
        csv::StringRecord::from(sanitized_headers)
    };
    let headers_len = headers.len();

    // Set up output handling
    let output_to_dir = args.arg_outdir.is_some();
    let mut row_no = 0_u64;

    let use_rowno_filename = args.flag_outfilename == QSV_ROWNO;

    // Create filename template once if needed
    #[allow(unused_assignments)]
    let mut filename_env = Environment::empty();
    let filename_template = if output_to_dir && !use_rowno_filename {
        // actually init the MiniJinja environment with default filters, tests and globals loaded
        filename_env = Environment::new();

        minijinja_contrib::add_to_environment(&mut filename_env);
        filename_env.set_unknown_method_callback(unknown_method_callback);
        filename_env.add_template("filename", &args.flag_outfilename)?;
        filename_env.get_template("filename")?
    } else {
        filename_env.template_from_str("")?
    };

    // Get width of rowcount for padding leading zeroes
    let rowcount = util::count_rows(&rconfig)?;
    let width = rowcount.to_string().len();

    let mut bulk_wtr = if output_to_dir {
        fs::create_dir_all(args.arg_outdir.as_ref().unwrap())?;
        None
    } else {
        // we use a bigger BufWriter buffer here than the default 8k as ALL the output
        // is going to one destination and we want to minimize I/O syscalls
        Some(match args.flag_output {
            Some(file) => Box::new(BufWriter::with_capacity(
                DEFAULT_WTR_BUFFER_CAPACITY,
                fs::File::create(file)?,
            )) as Box<dyn Write>,
            None => Box::new(BufWriter::with_capacity(
                DEFAULT_WTR_BUFFER_CAPACITY,
                std::io::stdout(),
            )) as Box<dyn Write>,
        })
    };

    let num_jobs = util::njobs(args.flag_jobs);
    let batchsize = util::optimal_batch_size(&rconfig, args.flag_batch, num_jobs);

    // prep progress bar
    let show_progress =
        (args.flag_progressbar || util::get_envvar_flag("QSV_PROGRESSBAR")) && !rconfig.is_stdin();

    let progress = ProgressBar::with_draw_target(None, ProgressDrawTarget::stderr_with_hz(5));
    if show_progress {
        util::prep_progress(&progress, rowcount);
    } else {
        progress.set_draw_target(ProgressDrawTarget::hidden());
    }

    // reuse batch buffers
    #[allow(unused_assignments)]
    let mut batch_record = csv::StringRecord::new();
    let mut batch: Vec<csv::StringRecord> = Vec::with_capacity(batchsize);
    // batch_results stores the results of template rendering for each batch:
    // - First tuple element is the optional output filename (when writing to directory)
    // - Second tuple element is the rendered template content
    let mut batch_results: Vec<(Option<String>, String)> = Vec::with_capacity(batchsize);

    let no_headers = args.flag_no_headers;

    // main loop to read CSV and construct batches for parallel processing.
    // each batch is processed via Rayon parallel iterator.
    // loop exits when batch is empty.
    'batch_loop: loop {
        for _ in 0..batchsize {
            match rdr.read_record(&mut batch_record) {
                Ok(has_data) => {
                    if has_data {
                        row_no += 1;
                        batch_record.push_field(itoa::Buffer::new().format(row_no));
                        batch.push(std::mem::take(&mut batch_record));
                    } else {
                        // nothing else to add to batch
                        break;
                    }
                },
                Err(e) => {
                    return fail_clierror!("Error reading file: {e}");
                },
            }
        }

        if batch.is_empty() {
            // break out of infinite loop when at EOF
            break 'batch_loop;
        }

        // do actual template rendering via Rayon parallel iterator
        batch
            .par_iter()
            .with_min_len(1024)
            .map(|record| {
                let curr_record = record;

                let mut context = simd_json::borrowed::Object::default();
                context.reserve(headers_len);
                let mut row_number = 0_u64;

                // add the fields of the current record to the context
                if no_headers {
                    // Use numeric, column 1-based indices (e.g. _c1, _c2, etc.)
                    let headers_len = curr_record.len();

                    for (i, field) in curr_record.iter().enumerate() {
                        if i == headers_len - 1 {
                            // set the last field to QSV_ROWNO
                            row_number = atoi_simd::parse::<u64>(field.as_bytes()).unwrap();
                            context.insert(
                                std::borrow::Cow::Borrowed(QSV_ROWNO),
                                BorrowedValue::String(std::borrow::Cow::Borrowed(field)),
                            );
                        } else {
                            context.insert(
                                format!("_c{}", i + 1).into(),
                                BorrowedValue::String(std::borrow::Cow::Borrowed(field)),
                            );
                        }
                    }
                } else {
                    // Use header names
                    for (header, field) in headers.iter().zip(curr_record.iter()) {
                        context.insert(
                            std::borrow::Cow::Borrowed(header),
                            BorrowedValue::String(std::borrow::Cow::Borrowed(field)),
                        );
                        // when headers are defined, the last one is QSV_ROWNO
                        if header == QSV_ROWNO {
                            row_number = atoi_simd::parse::<u64>(field.as_bytes()).unwrap();
                        }
                    }
                }

                // Render template with record data using context
                let rendered = template
                    .render(&context)
                    .unwrap_or_else(|_| "RENDERING ERROR".to_owned());

                if output_to_dir {
                    let outfilename = if use_rowno_filename {
                        // Pad row number with required number of leading zeroes
                        format!("{row_number:0width$}.txt")
                    } else {
                        // render filename with record data using context
                        // if the filename cannot be rendered, set the filename so the user
                        // can easily find the record which caused the rendering error
                        // e.g. FILENAME_RENDERING_ERROR-00035.txt is the 35th record in a CSV
                        // with at least 10000 rows (the three leading zeros)
                        filename_template.render(&context).unwrap_or_else(|_| {
                            format!("FILENAME_RENDERING_ERROR-{row_number:0width$}.txt")
                        })
                    };
                    (Some(outfilename), rendered)
                } else {
                    (None, rendered)
                }
            })
            .collect_into_vec(&mut batch_results);

        let mut outpath = std::path::PathBuf::new();
        for result_record in &batch_results {
            if output_to_dir {
                // safety: this is safe as output_to_dir = args.arg_outdir.is_some()
                // and result_record.0 (the filename to use) is_some()
                outpath.push(args.arg_outdir.as_ref().unwrap());
                outpath.push(result_record.0.as_deref().unwrap());

                // if output_to_dir is true, we'll be writing a LOT of files (one for each row)
                // and this hot loop will be I/O bound
                // we optimize the size of the BufWriter buffer here
                // so that it's only one I/O syscall per row
                let mut row_writer =
                    BufWriter::with_capacity(result_record.1.len(), fs::File::create(&outpath)?);
                row_writer.write_all(result_record.1.as_bytes())?;
                row_writer.flush()?;

                outpath.clear();
            } else if let Some(ref mut w) = bulk_wtr {
                w.write_all(result_record.1.as_bytes())?;
            }
        }

        if show_progress {
            progress.inc(batch.len() as u64);
        }

        batch.clear();
    } // end batch loop

    if show_progress {
        util::finish_progress(&progress);
    }

    if let Some(mut w) = bulk_wtr {
        w.flush()?;
    }

    Ok(())
}

// CUSTOM MINIJINJA FILTERS =========================================
// safety: for all FILTER_ERROR.gets, safe to unwrap as FILTER_ERROR
// is initialized on startup

/// Returns a substring of the input string from start index to end index (exclusive).
/// If end is not provided, returns substring from start to end of string.
/// Returns --customfilter-error (default: <FILTER_ERROR>) if indices are invalid.
fn substr(value: &str, start: u32, end: Option<u32>) -> String {
    let end = end.unwrap_or(value.len() as _);
    if let Some(s) = value.get(start as usize..end as usize) {
        s.into()
    } else {
        FILTER_ERROR.get().unwrap().clone()
    }
}

/// Formats a float number string with the specified decimal precision.
/// Returns --customfilter-error (default: <FILTER_ERROR>) if input cannot be parsed as float.
fn format_float(value: &str, precision: u32) -> String {
    // Prevent excessive precision
    let precision = precision.min(16);
    value.parse::<f64>().map_or_else(
        |_| FILTER_ERROR.get().unwrap().clone(),
        |num| format!("{:.1$}", num, precision as usize),
    )
}

/// Formats an integer with thousands separators (e.g. "1,234,567").
/// Returns --customfilter-error (default: <FILTER_ERROR>) if input cannot be parsed as integer.
fn human_count(value: &str) -> String {
    atoi_simd::parse::<u64>(value.as_bytes()).map_or_else(
        |_| FILTER_ERROR.get().unwrap().clone(),
        |num| indicatif::HumanCount(num).to_string(),
    )
}

/// Formats a float number with thousands separators (e.g. "1,234,567.89").
/// Returns --customfilter-error (default: <FILTER_ERROR>) if input cannot be parsed as float.
fn human_float_count(value: &str) -> String {
    value.parse::<f64>().map_or_else(
        |_| FILTER_ERROR.get().unwrap().clone(),
        |num| indicatif::HumanFloatCount(num).to_string(),
    )
}

/// Rounds a float number to specified number of decimal places.
/// Round using Midpoint Nearest Even Rounding Strategy AKA "Bankers Rounding."
/// https://docs.rs/rust_decimal/latest/rust_decimal/enum.RoundingStrategy.html#variant.MidpointNearestEven
/// Returns --customfilter-error (default: <FILTER_ERROR>) if input cannot be parsed as float.
fn round_num(value: &str, places: u32) -> String {
    value.parse::<f64>().map_or_else(
        |_| FILTER_ERROR.get().unwrap().clone(),
        |num| util::round_num(num, places),
    )
}

/// Converts string to boolean.
/// Returns true for "true", "1", "yes", "t" or "y" (case insensitive).
/// Returns false for all other values.
fn str_to_bool(value: &str) -> bool {
    matches!(
        value.to_ascii_lowercase().as_str(),
        "true" | "1" | "yes" | "t" | "y"
    )
}
