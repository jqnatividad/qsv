static USAGE: &str = r#"
Validates CSV data using two modes:

JSON SCHEMA VALIDATION MODE:
This mode is invoked if a JSON Schema file is provided.

The CSV data is validated against the JSON Schema. If the CSV data is valid, no output
files are created and the command returns an exit code of 0.

If invalid records are found, they are put into an "invalid" file, with the rest of the
records put into a "valid"" file.

A "validation-errors.tsv" report is also created with the following columns:

  * row_number: the row number of the invalid record
  * field: the field name of the invalid field
  * error: a validation error message detailing why the field is invalid

It uses the JSON Schema Validation Specification (draft 2020-12) to validate the CSV.
It validates not only the structure of the file, but the data types and domain/range of the
fields as well. See https://json-schema.org/draft/2020-12/json-schema-validation.html

You can create a JSON Schema file from a reference CSV file using the `qsv schema` command.
Once the schema is created, you can fine-tune it to your needs and use it to validate other CSV
files that have the same structure.

Be sure to select a “training” CSV file that is representative of the data you want to validate
when creating a schema. The data types, domain/range and regular expressions inferred from the
reference CSV file should be appropriate for the data you want to validate. 

Typically, after creating a schema, you should edit it to fine-tune each field's inferred
validation rules.

For example, if we created a JSON schema file called "reference.schema.json" using the `schema` command.
And want to validate "mydata.csv" which we know has validation errors, the output files from running
`qsv validate mydata.csv reference.schema.json` are: 

  * mydata.csv.valid
  * mydata.csv.invalid
  * mydata.csv.validation-errors.tsv

With an exit code of 1 to indicate a validation error.

If we validate another CSV file, "mydata2.csv", which we know is valid, there are no output files,
and the exit code is 0.

If piped from stdin, the filenames will use `stdin.csv` as the base filename. For example:
`cat mydata.csv | qsv validate reference.schema.json`

   * stdin.csv.valid
   * stdin.csv.invalid
   * stdin.csv.validation-errors.tsv

RFC 4180 VALIDATION MODE:
If run without a JSON Schema file, the CSV is validated if it complies with qsv's interpretation of
the RFC 4180 CSV standard (see https://github.com/jqnatividad/qsv#rfc-4180-csv-standard).

It also confirms if the CSV is UTF-8 encoded.

For both modes, returns exit code 0 when the CSV file is valid, exitcode > 0 otherwise.
If all records are valid, no output files are produced.

For examples, see https://github.com/jqnatividad/qsv/blob/master/tests/test_validate.rs.

Usage:
    qsv validate [options] [<input>] [<json-schema>]
    qsv validate --help

Validate arguments:
    <input>                    Input CSV file to validate. If not provided, will read from stdin.
    <json-schema>              JSON Schema file to validate against. If not provided, `validate`
                               will run in RFC 4180 validation mode. The file can be a local file
                               or a URL (http and https schemes supported).

Validate options:
    --trim                     Trim leading and trailing whitespace from fields before validating.
    --fail-fast                Stops on first error.
    --valid <suffix>           Valid record output file suffix. [default: valid]
    --invalid <suffix>         Invalid record output file suffix. [default: invalid]
    --json                     When validating without a schema, return the RFC 4180 check
                               as a JSON file instead of a message.
    --pretty-json              Same as --json, but pretty printed.
    --valid-output <file>      Change validation mode behavior so if ALL rows are valid, to pass it to
                               output, return exit code 1, and set stderr to the number of valid rows.
                               Setting this will override the default behavior of creating
                               a valid file only when there are invalid records.
                               To send valid records to stdout, use `-` as the filename.
    -j, --jobs <arg>           The number of jobs to run in parallel.
                               When not set, the number of jobs is set to the
                               number of CPUs detected.
    -b, --batch <size>         The number of rows per batch to load into memory,
                               before running in parallel.
                               [default: 50000]
    --timeout <seconds>        Timeout for downloading json-schemas on URLs.
                               [default: 30]

Common options:
    -h, --help                 Display this message
    -n, --no-headers           When set, the first row will not be interpreted
                               as headers. Namely, it will be sorted with the rest
                               of the rows. Otherwise, the first row will always
                               appear as the header row in the output.
    -d, --delimiter <arg>      The field delimiter for reading CSV data.
                               Must be a single character. [default: ,]
    -p, --progressbar          Show progress bars. Not valid for stdin.
    -Q, --quiet                Do not display validation summary message.
"#;

use std::{
    env,
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
    str,
    sync::{
        atomic::{AtomicU16, Ordering},
        OnceLock,
    },
};

use csv::ByteRecord;
use indicatif::HumanCount;
#[cfg(any(feature = "feature_capable", feature = "lite"))]
use indicatif::{ProgressBar, ProgressDrawTarget};
use itertools::Itertools;
use jsonschema::{
    output::BasicOutput,
    paths::{JSONPointer, JsonPointerNode, PathChunk},
    ErrorIterator, JSONSchema, Keyword, ValidationError,
};
use log::{debug, info, log_enabled};
use rayon::{
    iter::{IndexedParallelIterator, ParallelIterator},
    prelude::IntoParallelRefIterator,
};
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::{json, value::Number, Map, Value};
use simdutf8::basic::from_utf8;

use crate::{
    config::{Config, Delimiter, DEFAULT_WTR_BUFFER_CAPACITY},
    regex_oncelock, util, CliResult,
};

// to save on repeated init/allocs
static NULL_TYPE: OnceLock<Value> = OnceLock::new();

static TIMEOUT_SECS: AtomicU16 = AtomicU16::new(30);

#[derive(Deserialize)]
#[allow(dead_code)]
struct Args {
    flag_trim:         bool,
    flag_fail_fast:    bool,
    flag_valid:        Option<String>,
    flag_invalid:      Option<String>,
    flag_json:         bool,
    flag_pretty_json:  bool,
    flag_valid_output: Option<String>,
    flag_jobs:         Option<usize>,
    flag_batch:        u32,
    flag_no_headers:   bool,
    flag_delimiter:    Option<Delimiter>,
    flag_progressbar:  bool,
    flag_quiet:        bool,
    arg_input:         Option<String>,
    arg_json_schema:   Option<String>,
    flag_timeout:      u16,
}

enum JSONtypes {
    String,
    Number,
    Integer,
    Boolean,
    Unsupported,
}

#[derive(Serialize, Deserialize)]
struct RFC4180Struct {
    delimiter_char: char,
    header_row:     bool,
    quote_char:     char,
    num_records:    u64,
    num_fields:     u64,
    fields:         Vec<String>,
}

struct IsAsciiValidator;
impl Keyword for IsAsciiValidator {
    fn validate<'instance>(
        &self,
        instance: &'instance Value,
        instance_path: &JsonPointerNode,
    ) -> ErrorIterator<'instance> {
        let mut errors = vec![];
        for key in instance.as_object().unwrap().keys() {
            if !key.is_ascii() {
                let error = ValidationError::custom(
                    JSONPointer::default(),
                    instance_path.into(),
                    instance,
                    "Key is not ASCII",
                );
                errors.push(error);
            }
        }
        Box::new(errors.into_iter())
    }

    fn is_valid(&self, instance: &Value) -> bool {
        for (key, _value) in instance.as_object().unwrap() {
            if !key.is_ascii() {
                return false;
            }
        }
        true
    }
}

fn custom_object_type_factory<'a>(
    _: &'a Map<String, Value>,
    schema: &'a Value,
    path: JSONPointer,
) -> Result<Box<dyn Keyword>, ValidationError<'a>> {
    const EXPECTED: &str = "ascii-keys";
    if schema.as_str().map_or(true, |key| key != EXPECTED) {
        Err(ValidationError::custom(
            JSONPointer::default(),
            path,
            schema,
            "Expected 'ascii-keys'",
        ))
    } else {
        Ok(Box::new(IsAsciiValidator))
    }
}

/// Check that a string has some number of digits followed by a dot followed by exactly 2 digits.
fn currency_format_checker(s: &str) -> bool {
    let currency_re: &'static Regex = regex_oncelock!("^(0|([1-9]+[0-9]*))(\\.[0-9]{2})$");
    currency_re.is_match(s)
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;

    TIMEOUT_SECS.store(
        util::timeout_secs(args.flag_timeout)? as u16,
        Ordering::Relaxed,
    );

    #[cfg(any(feature = "feature_capable", feature = "lite"))]
    let mut rconfig = Config::new(&args.arg_input)
        .delimiter(args.flag_delimiter)
        .no_headers(args.flag_no_headers);
    #[cfg(feature = "datapusher_plus")]
    let rconfig = Config::new(&args.arg_input)
        .delimiter(args.flag_delimiter)
        .no_headers(args.flag_no_headers);

    let mut rdr = rconfig.reader()?;

    // if no JSON Schema supplied, only let csv reader RFC4180-validate csv file
    if args.arg_json_schema.is_none() {
        // just read csv file and let csv reader report problems
        // since we're using csv::StringRecord, this will also detect non-utf8 sequences

        let flag_json = args.flag_json || args.flag_pretty_json;
        let flag_pretty_json = args.flag_pretty_json;

        // first, let's validate the header row
        let mut header_msg = String::new();
        let mut header_len = 0_usize;
        let mut field_vec: Vec<String> = Vec::new();
        if !args.flag_no_headers {
            let fields_result = rdr.headers();
            match fields_result {
                Ok(fields) => {
                    header_len = fields.len();
                    field_vec.reserve(header_len);
                    for field in fields {
                        field_vec.push(field.to_string());
                    }
                    let field_list = field_vec.join(r#"", ""#);
                    header_msg = format!(
                        "{} columns (\"{field_list}\") and ",
                        HumanCount(header_len as u64)
                    );
                },
                Err(e) => {
                    // we're returning a JSON error for the header,
                    // so we have more machine-friendly details
                    if flag_json {
                        // there's a UTF-8 error, so we report utf8 error metadata
                        if let csv::ErrorKind::Utf8 { pos, err } = e.kind() {
                            let header_error = json!({
                                "errors": [{
                                    "title" : "Header UTF-8 validation error",
                                    "detail" : format!("{e}"),
                                    "meta": {
                                        "record_position": format!("{pos:?}"),
                                        "record_error": format!("{err}"),
                                    }
                                }]
                            });
                            let json_error = if flag_pretty_json {
                                serde_json::to_string_pretty(&header_error).unwrap()
                            } else {
                                header_error.to_string()
                            };

                            return fail_encoding_clierror!("{json_error}");
                        }
                        // it's not a UTF-8 error, so we report a generic
                        // header validation error
                        let header_error = json!({
                            "errors": [{
                                "title" : "Header Validation error",
                                "detail" : format!("{e}"),
                            }]
                        });
                        let json_error = if flag_pretty_json {
                            serde_json::to_string_pretty(&header_error).unwrap()
                        } else {
                            header_error.to_string()
                        };
                        return fail_encoding_clierror!("{json_error}");
                    }
                    // we're not returning a JSON error, so we can use
                    // a user-friendly error message with suggestions
                    if let csv::ErrorKind::Utf8 { pos, err } = e.kind() {
                        return fail_encoding_clierror!(
                            "non-utf8 sequence detected in header, position {pos:?}.\n{err}\nUse \
                             `qsv input` to fix formatting and to handle non-utf8 sequences.\n
                             You may also want to transcode your data to UTF-8 first using `iconv` \
                             or `recode`."
                        );
                    }
                    // its not a UTF-8 error, report a generic header validation error
                    return fail_clierror!("Header Validation error: {e}.");
                },
            }
        }

        // Now, let's validate the rest of the records the fastest way possible.
        // We do this by using csv::ByteRecord, which does not validate utf8
        // making for higher throughput and lower memory usage compared to csv::StringRecord
        // which validates each field SEPARATELY as a utf8 string.
        // Combined with simdutf8::basic::from_utf8(), we utf8-validate the entire record in one go
        // as a slice of bytes, this approach is much faster than csv::StringRecord's
        // per-field validation.
        let mut record = csv::ByteRecord::with_capacity(500, header_len);
        let mut result;
        let mut record_idx: u64 = 0;

        'rfc4180_check: loop {
            result = rdr.read_byte_record(&mut record);
            if let Err(e) = result {
                // read_byte_record() does not validate utf8, so we know this is not a utf8 error
                if flag_json {
                    // we're returning a JSON error, so we have more machine-friendly details
                    // using the JSON API error format

                    let validation_error = json!({
                        "errors": [{
                            "title" : "Validation error",
                            "detail" : format!("{e}"),
                            "meta": {
                                "last_valid_record": format!("{record_idx}"),
                            }
                        }]
                    });

                    let json_error = if flag_pretty_json {
                        serde_json::to_string_pretty(&validation_error).unwrap()
                    } else {
                        validation_error.to_string()
                    };

                    return fail!(json_error);
                }

                // we're not returning a JSON error, so we can use a
                // user-friendly error message with a fixlengths suggestion
                if let csv::ErrorKind::UnequalLengths {
                    expected_len: _,
                    len: _,
                    pos: _,
                } = e.kind()
                {
                    return fail_clierror!(
                        "Validation error: {e}.\nUse `qsv fixlengths` to fix record length issues."
                    );
                }
                return fail_clierror!("Validation error: {e}.\nLast valid record: {record_idx}");
            }

            // use SIMD accelerated UTF-8 validation, validate the entire record in one go
            if simdutf8::basic::from_utf8(record.as_slice()).is_err() {
                // there's a UTF-8 error, so we report utf8 error metadata
                if flag_json {
                    let validation_error = json!({
                        "errors": [{
                            "title" : "UTF-8 validation error",
                            "detail" : "Cannot parse CSV record as UTF-8",
                            "meta": {
                                "last_valid_record": format!("{record_idx}"),
                            }
                        }]
                    });

                    let json_error = if flag_pretty_json {
                        serde_json::to_string_pretty(&validation_error).unwrap()
                    } else {
                        validation_error.to_string()
                    };
                    return fail_encoding_clierror!("{json_error}");
                }
                // we're not returning a JSON error, so we can use a
                // user-friendly error message with utf8 transcoding suggestions
                return fail_encoding_clierror!(
                    "non-utf8 sequence at record {record_idx}.\nUse `qsv input` to fix formatting \
                     and to handle non-utf8 sequences.\nYou may also want to transcode your data \
                     to UTF-8 first using `iconv` or `recode`."
                );
            }

            if result.is_ok_and(|more_data| !more_data) {
                // we've read the CSV to the end, so break out of loop
                break 'rfc4180_check;
            }
            record_idx += 1;
        } // end rfc4180_check loop

        // if we're here, we know the CSV is valid
        let msg = if flag_json {
            let rfc4180 = RFC4180Struct {
                delimiter_char: rconfig.get_delimiter() as char,
                header_row:     !rconfig.no_headers,
                quote_char:     rconfig.quote as char,
                num_records:    record_idx,
                num_fields:     header_len as u64,
                fields:         field_vec,
            };

            if flag_pretty_json {
                serde_json::to_string_pretty(&rfc4180).unwrap()
            } else {
                serde_json::to_string(&rfc4180).unwrap()
            }
        } else {
            format!(
                "Valid: {header_msg}{} records detected.",
                HumanCount(record_idx)
            )
        };
        if !args.flag_quiet {
            woutinfo!("{msg}");
        }

        // we're done when validating without a schema
        return Ok(());
    }

    // prep progress bar
    #[cfg(any(feature = "feature_capable", feature = "lite"))]
    let progress = ProgressBar::with_draw_target(None, ProgressDrawTarget::stderr_with_hz(5));

    #[cfg(any(feature = "feature_capable", feature = "lite"))]
    let show_progress =
        (args.flag_progressbar || util::get_envvar_flag("QSV_PROGRESSBAR")) && !rconfig.is_stdin();

    #[cfg(any(feature = "feature_capable", feature = "lite"))]
    if show_progress {
        // for full row count, prevent CSV reader from aborting on inconsistent column count
        rconfig = rconfig.flexible(true);
        let record_count = util::count_rows(&rconfig)?;
        rconfig = rconfig.flexible(false);
        util::prep_progress(&progress, record_count);
    } else {
        progress.set_draw_target(ProgressDrawTarget::hidden());
    }

    let headers = rdr.byte_headers()?.clone();
    let header_len = headers.len();

    // parse and compile supplied JSON Schema
    let (schema_json, schema_compiled): (Value, JSONSchema) =
        match load_json(&args.arg_json_schema.unwrap()) {
            Ok(s) => {
                // parse JSON string
                match serde_json::from_str(&s) {
                    Ok(json) => {
                        // compile JSON Schema
                        match JSONSchema::options()
                            .with_format("currency", currency_format_checker)
                            .with_keyword("ascii-keys", custom_object_type_factory)
                            .compile(&json)
                        {
                            Ok(schema) => (json, schema),
                            Err(e) => {
                                return fail_clierror!("Cannot compile schema json. error: {e}");
                            },
                        }
                    },
                    Err(e) => {
                        return fail_clierror!("Unable to parse schema json. error: {e}");
                    },
                }
            },
            Err(e) => {
                return fail_clierror!("Unable to retrieve json. error: {e}");
            },
        };

    if log::log_enabled!(log::Level::Debug) {
        // only log if debug is enabled
        // as it can be quite large and expensive to deserialize the schema
        debug!("schema json: {:?}", &schema_json);
    }

    // set this once, as this is used repeatedly in a hot loop
    NULL_TYPE.set(Value::String("null".to_string())).unwrap();

    // get JSON types for each column in CSV file
    let header_types = get_json_types(&headers, &schema_json)?;

    // how many rows read and processed as batches
    let mut row_number: u64 = 0;
    // how many invalid rows found
    let mut invalid_count: u64 = 0;

    // amortize memory allocation by reusing record
    let mut record = csv::ByteRecord::new();
    // reuse batch buffer
    let batch_size = args.flag_batch as usize;
    let mut batch = Vec::with_capacity(batch_size);
    let mut validation_results = Vec::with_capacity(batch_size);
    let mut valid_flags: Vec<bool> = Vec::with_capacity(batch_size);
    let mut validation_error_messages: Vec<String> = Vec::with_capacity(50);
    let flag_trim = args.flag_trim;

    // set RAYON_NUM_THREADS
    util::njobs(args.flag_jobs);

    // main loop to read CSV and construct batches for parallel processing.
    // each batch is processed via Rayon parallel iterator.
    // loop exits when batch is empty.
    'batch_loop: loop {
        let mut buffer = itoa::Buffer::new();
        for _ in 0..batch_size {
            match rdr.read_byte_record(&mut record) {
                Ok(has_data) => {
                    if has_data {
                        row_number += 1;
                        record.push_field(buffer.format(row_number).as_bytes());
                        if flag_trim {
                            record.trim();
                        }
                        batch.push(std::mem::take(&mut record));
                    } else {
                        // nothing else to add to batch
                        break;
                    }
                },
                Err(e) => {
                    return fail_clierror!("Error reading row: {row_number}: {e}");
                },
            }
        }

        if batch.is_empty() {
            // break out of infinite loop when at EOF
            break 'batch_loop;
        }

        // do actual validation via Rayon parallel iterator
        // validation_results vector should have same row count and in same order as input CSV
        batch
            .par_iter()
            .map(|record| do_json_validation(&header_types, header_len, record, &schema_compiled))
            .collect_into_vec(&mut validation_results);

        // write to validation error report, but keep Vec<bool> to gen valid/invalid files later
        // because Rayon collect() guarantees original order, we can sequentially append results
        // to vector with each batch
        for result in &validation_results {
            if let Some(validation_error_msg) = result {
                invalid_count += 1;
                valid_flags.push(false);

                validation_error_messages.push(validation_error_msg.to_string());
            } else {
                valid_flags.push(true);
            }
        }

        #[cfg(any(feature = "feature_capable", feature = "lite"))]
        if show_progress {
            progress.inc(batch.len() as u64);
        }
        batch.clear();

        // for fail-fast, exit loop if batch has any error
        if args.flag_fail_fast && invalid_count > 0 {
            break 'batch_loop;
        }
    } // end batch loop

    #[cfg(any(feature = "feature_capable", feature = "lite"))]
    if show_progress {
        progress.set_message(format!(
            " validated {} records.",
            HumanCount(progress.length().unwrap())
        ));
        util::finish_progress(&progress);
    }

    if invalid_count == 0 {
        // no invalid records found
        // see if we need to pass all valid records to output
        if let Some(valid_output) = args.flag_valid_output {
            // pass all valid records to output and return exit code 1
            let valid_path = if valid_output == "-" {
                // write to stdout
                None
            } else {
                Some(valid_output)
            };

            let mut valid_wtr = Config::new(&valid_path).writer()?;
            valid_wtr.write_byte_record(&headers)?;

            let mut rdr = rconfig.reader()?;
            let mut record = csv::ByteRecord::new();
            while rdr.read_byte_record(&mut record)? {
                valid_wtr.write_byte_record(&record)?;
            }
            valid_wtr.flush()?;
            // return 1 as an exitcode and the number of valid rows to stderr
            return fail_clierror!("{row_number}");
        }
    } else {
        // there are invalid records. write out invalid/valid/errors output files.
        // if 100% invalid, valid file isn't needed, but this is rare so OK creating empty file.
        woutinfo!("Writing invalid/valid/error files...");

        let input_path = args
            .arg_input
            .clone()
            .unwrap_or_else(|| "stdin.csv".to_string());

        write_error_report(&input_path, validation_error_messages)?;

        let valid_suffix = args.flag_valid.unwrap_or_else(|| "valid".to_string());
        let invalid_suffix = args.flag_invalid.unwrap_or_else(|| "invalid".to_string());

        split_invalid_records(
            &rconfig,
            &valid_flags[..],
            &headers,
            &input_path,
            &valid_suffix,
            &invalid_suffix,
        )?;

        // done with validation; print output
        let fail_fast_msg = if args.flag_fail_fast {
            format!(
                "fail-fast enabled. stopped after row {}.\n",
                HumanCount(row_number)
            )
        } else {
            String::new()
        };

        return fail_clierror!(
            "{fail_fast_msg}{} out of {} records invalid.",
            HumanCount(invalid_count),
            HumanCount(row_number)
        );
    }

    if !args.flag_quiet {
        winfo!("All {} records valid.", HumanCount(row_number));
    }
    Ok(())
}

fn split_invalid_records(
    rconfig: &Config,
    valid_flags: &[bool],
    headers: &ByteRecord,
    input_path: &str,
    valid_suffix: &str,
    invalid_suffix: &str,
) -> CliResult<()> {
    // track how many rows read for splitting into valid/invalid
    // should not exceed row_number when aborted early due to fail-fast
    let mut split_row_num: usize = 0;

    // prepare output writers
    let mut valid_wtr = Config::new(&Some(input_path.to_owned() + "." + valid_suffix)).writer()?;
    valid_wtr.write_byte_record(headers)?;

    let mut invalid_wtr =
        Config::new(&Some(input_path.to_owned() + "." + invalid_suffix)).writer()?;
    invalid_wtr.write_byte_record(headers)?;

    let mut rdr = rconfig.reader()?;

    let mut record = csv::ByteRecord::new();
    while rdr.read_byte_record(&mut record)? {
        split_row_num += 1;

        // length of valid_flags is max number of rows we can split
        if split_row_num > valid_flags.len() {
            break;
        }

        // vector is 0-based, row_num is 1-based
        let is_valid = valid_flags[split_row_num - 1];

        if is_valid {
            valid_wtr.write_byte_record(&record)?;
        } else {
            invalid_wtr.write_byte_record(&record)?;
        }
    }

    valid_wtr.flush()?;
    invalid_wtr.flush()?;

    Ok(())
}

fn write_error_report(input_path: &str, validation_error_messages: Vec<String>) -> CliResult<()> {
    let wtr_capacitys = env::var("QSV_WTR_BUFFER_CAPACITY")
        .unwrap_or_else(|_| DEFAULT_WTR_BUFFER_CAPACITY.to_string());
    let wtr_buffer_size: usize = wtr_capacitys.parse().unwrap_or(DEFAULT_WTR_BUFFER_CAPACITY);

    let output_file = File::create(input_path.to_owned() + ".validation-errors.tsv")?;

    let mut output_writer = BufWriter::with_capacity(wtr_buffer_size, output_file);

    output_writer.write_all(b"row_number\tfield\terror\n")?;

    // write out error report
    for error_msg in validation_error_messages {
        output_writer.write_all(error_msg.as_bytes())?;
        // since writer is buffered, it's more efficient to do additional write than append Newline
        // to message
        output_writer.write_all(&[b'\n'])?;
    }

    // flush error report; file gets closed automagically when out-of-scope
    output_writer.flush()?;

    Ok(())
}

/// if given record is valid, return None, otherwise, error file entry string
#[inline]
fn do_json_validation(
    header_types: &[(String, JSONtypes)],
    header_len: usize,
    record: &ByteRecord,
    schema_compiled: &JSONSchema,
) -> Option<String> {
    // row number was added as last column. We use can do unwrap safely since we know its there
    let row_number_string = from_utf8(record.get(header_len).unwrap()).unwrap();

    validate_json_instance(
        &(match to_json_instance(header_types, header_len, record) {
            Ok(obj) => obj,
            Err(e) => {
                return Some(format!("{row_number_string}\t<RECORD>\t{e}"));
            },
        }),
        schema_compiled,
    )
    .map(|validation_errors| {
        // squash multiple errors into one long String with linebreaks
        let combined_errors: String = validation_errors
            .iter()
            .map(|tuple| {
                // validation error file format: row_number, field, error
                format!("{row_number_string}\t{}\t{}", tuple.0, tuple.1)
            })
            .join("\n");

        combined_errors
    })
}

/// convert CSV Record into JSON instance by referencing JSON types
#[inline]
fn to_json_instance(
    header_types: &[(String, JSONtypes)],
    header_len: usize,
    record: &ByteRecord,
) -> CliResult<Value> {
    let mut json_object_map: Map<String, Value> = Map::with_capacity(header_len);

    let mut key_string: String;
    let mut value_string: String;
    for ((key_iter, json_type), value) in header_types.iter().zip(record.iter()) {
        key_string = key_iter.to_owned();

        if value.is_empty() {
            json_object_map.insert(key_string, Value::Null);
            continue;
        }

        let value_str = match from_utf8(value) {
            Ok(v) => v,
            Err(e) => {
                let s = String::from_utf8_lossy(value);
                return fail_encoding_clierror!("CSV value \"{s}\" is not valid UTF-8: {e}");
            },
        };

        match *json_type {
            JSONtypes::String => {
                value_string = value_str.to_owned();
                json_object_map.insert(key_string, Value::String(value_string));
            },
            JSONtypes::Number => {
                if let Ok(float) = value_str.parse::<f64>() {
                    json_object_map
                        .insert(key_string, Value::Number(Number::from_f64(float).unwrap()));
                } else {
                    return fail_clierror!(
                        "Can't cast into Number. key: {key_string}, value: {value_str}"
                    );
                }
            },
            JSONtypes::Integer => {
                if let Ok(int) = atoi_simd::parse::<i64>(value_str.as_bytes()) {
                    json_object_map.insert(key_string, Value::Number(Number::from(int)));
                } else {
                    return fail_clierror!(
                        "Can't cast into Integer. key: {key_string}, value: {value_str}"
                    );
                }
            },
            JSONtypes::Boolean => {
                if let Ok(boolean) = value_str.parse::<bool>() {
                    json_object_map.insert(key_string, Value::Bool(boolean));
                } else {
                    return fail_clierror!(
                        "Can't cast into Boolean. key: {key_string}, value: {value_str}"
                    );
                }
            },
            JSONtypes::Unsupported => {
                unreachable!("we should never get an unsupported JSON type");
            },
        }
    }

    Ok(Value::Object(json_object_map))
}

/// get JSON types for each column in CSV file
/// returns a Vector of tuples of column/header name (String) & JSON type (JSONtypes enum)
#[inline]
fn get_json_types(headers: &ByteRecord, schema: &Value) -> CliResult<Vec<(String, JSONtypes)>> {
    // make sure schema has expected structure
    let Some(schema_properties) = schema.get("properties") else {
        return fail_clierror!("JSON Schema missing 'properties' object");
    };

    // safety: we set NULL_TYPE in main() and it's never changed
    let null_type = NULL_TYPE.get().unwrap();

    let mut key_string: String;
    let mut field_def: &Value;
    let mut field_type_def: &Value;
    let mut json_type: JSONtypes;
    let mut header_types: Vec<(String, JSONtypes)> = Vec::with_capacity(headers.len());

    // iterate over each CSV field and convert to JSON type
    for header in headers {
        // convert csv header to string. It's the key in the JSON object
        key_string = if let Ok(s) = from_utf8(header) {
            s.to_owned()
        } else {
            let s = String::from_utf8_lossy(header);
            return fail_encoding_clierror!("CSV header is not valid UTF-8: {s}");
        };

        field_def = schema_properties
            .get(key_string.clone())
            .unwrap_or(&Value::Null);
        field_type_def = field_def.get("type").unwrap_or(&Value::Null);

        json_type = match field_type_def {
            Value::String(s) => match s.as_str() {
                "string" => JSONtypes::String,
                "number" => JSONtypes::Number,
                "integer" => JSONtypes::Integer,
                "boolean" => JSONtypes::Boolean,
                _ => JSONtypes::Unsupported,
            },
            Value::Array(vec) => {
                let mut return_val = JSONtypes::String;
                for val in vec {
                    if *val == *null_type {
                        continue;
                    }
                    return_val = if let Some(s) = val.as_str() {
                        match s {
                            "string" => JSONtypes::String,
                            "number" => JSONtypes::Number,
                            "integer" => JSONtypes::Integer,
                            "boolean" => JSONtypes::Boolean,
                            _ => JSONtypes::Unsupported,
                        }
                    } else {
                        JSONtypes::String
                    };
                }
                return_val
            },
            _ => JSONtypes::String,
        };

        header_types.push((key_string, json_type));
    }
    Ok(header_types)
}

#[cfg(test)]
mod tests_for_csv_to_json_conversion {

    use serde_json::json;

    use super::*;

    /// get schema used for unit tests
    fn schema_json() -> Value {
        // from https://json-schema.org/learn/miscellaneous-examples.html
        serde_json::json!({
            "$id": "https://example.com/test.schema.json",
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "title": "test",
            "type": "object",
            "properties": {
                "A": {
                    "type": "string",
                },
                "B": {
                    "type": "number",
                },
                "C": {
                    "type": "integer",
                },
                "D": {
                    "type": "boolean",
                },
                "E": {
                    "type": ["string", "null"],
                },
                "F": {
                    "type": ["number", "null"],
                },
                "G": {
                    "type": ["integer", "null"],
                },
                "H": {
                    "type": ["boolean", "null"],
                },
                "I": {
                    "type": ["string", "null"],
                },
                "J": {
                    "type": ["number", "null"],
                },
                "K": {
                    "type": ["null", "integer"],
                },
                "L": {
                    "type": ["boolean", "null"],
                },
            }
        })
    }

    #[test]
    #[allow(clippy::approx_constant)]
    fn test_to_json_instance() {
        let _ = NULL_TYPE.get_or_init(|| Value::String("null".to_string()));
        let csv = "A,B,C,D,E,F,G,H,I,J,K,L
        hello,3.1415,300000000,true,,,,,hello,3.1415,300000000,true";

        let mut rdr = csv::Reader::from_reader(csv.as_bytes());
        let headers = rdr.byte_headers().unwrap().clone();
        let header_types = get_json_types(&headers, &schema_json()).unwrap();
        let mut record = rdr.byte_records().next().unwrap().unwrap();
        record.trim();

        assert_eq!(
            to_json_instance(&header_types, headers.len(), &record)
                .expect("can't convert csv to json instance"),
            json!({
                "A": "hello",
                "B": 3.1415,
                "C": 300_000_000,
                "D": true,
                "E": null,
                "F": null,
                "G": null,
                "H": null,
                "I": "hello",
                "J": 3.1415,
                "K": 300_000_000,
                "L": true,
            })
        );
    }

    #[test]
    fn test_to_json_instance_cast_integer_error() {
        let _ = NULL_TYPE.get_or_init(|| Value::String("null".to_string()));
        let csv = "A,B,C,D,E,F,G,H
        hello,3.1415,3.0e8,true,,,,";

        let mut rdr = csv::Reader::from_reader(csv.as_bytes());
        let headers = rdr.byte_headers().unwrap().clone();
        let header_types = get_json_types(&headers, &schema_json()).unwrap();

        let result = to_json_instance(
            &header_types,
            headers.len(),
            &rdr.byte_records().next().unwrap().unwrap(),
        );
        assert!(&result.is_err());
        let error = result.err().unwrap().to_string();
        assert_eq!("Can't cast into Integer. key: C, value: 3.0e8", error);
    }
}

/// Validate JSON instance against compiled JSON Schema
/// If invalid, returns Some(Vec<(String,String)>) holding the error messages
#[inline]
fn validate_json_instance(
    instance: &Value,
    schema_compiled: &JSONSchema,
) -> Option<Vec<(String, String)>> {
    let validation_output = schema_compiled.apply(instance);

    // If validation output is Invalid, then grab field names and errors
    if validation_output.flag() {
        None
    } else {
        // get validation errors as String
        let validation_errors: Vec<(String, String)> = match validation_output.basic() {
            BasicOutput::Invalid(errors) => errors
                .iter()
                .map(|e| {
                    if let Some(PathChunk::Property(box_str)) = e.instance_location().last() {
                        (box_str.to_string(), e.error_description().to_string())
                    } else {
                        (
                            e.instance_location().to_string(),
                            e.error_description().to_string(),
                        )
                    }
                })
                .collect(),
            BasicOutput::Valid(_annotations) => {
                // shouldn't happen
                unreachable!("Unexpected error.");
            },
        };

        Some(validation_errors)
    }
}

#[cfg(test)]
mod tests_for_schema_validation {
    use super::*;

    fn schema_json() -> Value {
        // from https://json-schema.org/learn/miscellaneous-examples.html
        serde_json::json!({
            "$id": "https://example.com/person.schema.json",
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "title": "Person",
            "type": "object",
            "properties": {
                "title": {
                    "type": "string",
                    "description": "The person's title.",
                    "minLength": 2
                },
                "name": {
                    "type": "string",
                    "description": "The person's name.",
                    "minLength": 2
                },
                "age": {
                    "description": "Age in years which must be equal to or greater than 18.",
                    "type": "integer",
                    "minimum": 18
                }
            }
        })
    }

    fn compiled_schema() -> JSONSchema {
        JSONSchema::options()
            .compile(&schema_json())
            .expect("Invalid schema")
    }

    #[test]
    fn test_validate_with_no_errors() {
        let _ = NULL_TYPE.get_or_init(|| Value::String("null".to_string()));
        let csv = "title,name,age
        Professor,Xaviers,60";

        let mut rdr = csv::Reader::from_reader(csv.as_bytes());
        let headers = rdr.byte_headers().unwrap().clone();
        let header_types = get_json_types(&headers, &schema_json()).unwrap();

        let record = &rdr.byte_records().next().unwrap().unwrap();

        let instance = to_json_instance(&header_types, headers.len(), record).unwrap();

        let result = validate_json_instance(&instance, &compiled_schema());

        assert!(result.is_none());
    }

    #[test]
    fn test_validate_with_error() {
        let _ = NULL_TYPE.get_or_init(|| Value::String("null".to_string()));
        let csv = "title,name,age
        Professor,X,60";

        let mut rdr = csv::Reader::from_reader(csv.as_bytes());
        let headers = rdr.byte_headers().unwrap().clone();
        let header_types = get_json_types(&headers, &schema_json()).unwrap();

        let record = &rdr.byte_records().next().unwrap().unwrap();

        let instance = to_json_instance(&header_types, headers.len(), record).unwrap();

        let result = validate_json_instance(&instance, &compiled_schema());

        assert!(result.is_some());

        assert_eq!(
            vec![(
                "name".to_string(),
                "\"X\" is shorter than 2 characters".to_string()
            )],
            result.unwrap()
        );
    }
}

#[test]
fn test_validate_currency_validator() {
    fn schema_currency_json() -> Value {
        serde_json::json!({
            "$id": "https://example.com/person.schema.json",
            "$schema": "https://json-schema.org/draft/2020-12/schema",
            "title": "Person",
            "type": "object",
            "properties": {
                "title": {
                    "type": "string",
                    "description": "The person's title.",
                    "minLength": 2
                },
                "name": {
                    "type": "string",
                    "description": "The person's name.",
                    "minLength": 2
                },
                "fee": {
                    "description": "The required fee to see the person.",
                    "type": "string",
                    "format": "currency",
                    "minimum": 18
                }
            }
        })
    }

    let _ = NULL_TYPE.get_or_init(|| Value::String("null".to_string()));
    let csv = "title,name,fee
    Professor,Xaviers,60.02123";

    let mut rdr = csv::Reader::from_reader(csv.as_bytes());
    let headers = rdr.byte_headers().unwrap().clone();
    let header_types = get_json_types(&headers, &schema_currency_json()).unwrap();

    let record = &rdr.byte_records().next().unwrap().unwrap();

    let instance = to_json_instance(&header_types, headers.len(), record).unwrap();

    let compiled_schema = JSONSchema::options()
        .with_format("currency", currency_format_checker)
        .compile(&schema_currency_json())
        .expect("Invalid schema");

    let result = validate_json_instance(&instance, &compiled_schema);

    assert_eq!(
        result,
        Some(vec![(
            "fee".to_owned(),
            "\"60.02123\" is not a \"currency\"".to_owned()
        )])
    );

    let csv = "title,name,fee
    Professor,Xaviers,60.02";

    let mut rdr = csv::Reader::from_reader(csv.as_bytes());
    let headers = rdr.byte_headers().unwrap().clone();
    let header_types = get_json_types(&headers, &schema_currency_json()).unwrap();

    let record = &rdr.byte_records().next().unwrap().unwrap();

    let instance = to_json_instance(&header_types, headers.len(), record).unwrap();

    let compiled_schema = JSONSchema::options()
        .with_format("currency", currency_format_checker)
        .compile(&schema_currency_json())
        .expect("Invalid schema");

    let result = validate_json_instance(&instance, &compiled_schema);

    // no validation error for currency format
    assert_eq!(result, None);
}

fn load_json(uri: &str) -> Result<String, String> {
    let json_string = match uri {
        url if url.to_lowercase().starts_with("http") => {
            use reqwest::blocking::Client;

            let client_timeout =
                std::time::Duration::from_secs(TIMEOUT_SECS.load(Ordering::Relaxed) as u64);

            let client = match Client::builder()
                // safety: we're using a validated QSV_USER_AGENT or if it's not set,
                // the default user agent
                .user_agent(util::set_user_agent(None).unwrap())
                .brotli(true)
                .gzip(true)
                .deflate(true)
                .zstd(true)
                .use_rustls_tls()
                .http2_adaptive_window(true)
                .connection_verbose(
                    log_enabled!(log::Level::Debug) || log_enabled!(log::Level::Trace),
                )
                .timeout(client_timeout)
                .build()
            {
                Ok(c) => c,
                Err(e) => {
                    return fail_format!("Cannot build reqwest client: {e}.");
                },
            };

            match client.get(url).send() {
                Ok(response) => response.text().unwrap_or_default(),
                Err(e) => return fail_format!("Cannot read JSON at url {url}: {e}."),
            }
        },
        path => {
            let mut buffer = String::new();
            match File::open(path) {
                Ok(p) => {
                    BufReader::new(p)
                        .read_to_string(&mut buffer)
                        .unwrap_or_default();
                },
                Err(e) => return fail_format!("Cannot read JSON file {path}: {e}."),
            }
            buffer
        },
    };

    Ok(json_string)
}

#[test]
fn test_load_json_via_url() {
    let json_string_result = load_json("https://geojson.org/schema/FeatureCollection.json");
    assert!(&json_string_result.is_ok());

    let json_result: Result<Value, serde_json::Error> =
        serde_json::from_str(&json_string_result.unwrap());
    assert!(&json_result.is_ok());
}
