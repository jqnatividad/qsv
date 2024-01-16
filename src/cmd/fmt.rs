static USAGE: &str = r#"
Formats CSV data with a custom delimiter or CRLF line endings.

Generally, all commands in qsv output CSV data in a default format, which is
the same as the default format for reading CSV data. This makes it easy to
pipe multiple qsv commands together. However, you may want the final result to
have a specific delimiter or record separator, and this is where 'qsv fmt' is
useful.

For examples, see https://github.com/jqnatividad/qsv/blob/master/tests/test_fmt.rs.

Usage:
    qsv fmt [options] [<input>]
    qsv fmt --help

fmt options:
    -t, --out-delimiter <arg>  The field delimiter for writing CSV data.
                               [default: ,]
    --crlf                     Use '\r\n' line endings in the output.
    --ascii                    Use ASCII field and record separators. Use Substitute (U+00A1) as the
                               quote character.
    --quote <arg>              The quote character to use. [default: "]
    --quote-always             Put quotes around every value.
    --quote-never              Never put quotes around any value.
    --escape <arg>             The escape character to use. When not specified,
                               quotes are escaped by doubling them.
    --no-final-newline         Do not write a newline at the end of the output.

Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
    -d, --delimiter <arg>  The field delimiter for reading CSV data.
                           Must be a single character. (default: ,)
"#;

use serde::Deserialize;

use crate::{
    config::{Config, Delimiter},
    util, CliResult,
};

#[derive(Deserialize)]
struct Args {
    arg_input:             Option<String>,
    flag_out_delimiter:    Option<Delimiter>,
    flag_crlf:             bool,
    flag_ascii:            bool,
    flag_output:           Option<String>,
    flag_delimiter:        Option<Delimiter>,
    flag_quote:            Delimiter,
    flag_quote_always:     bool,
    flag_quote_never:      bool,
    flag_escape:           Option<Delimiter>,
    flag_no_final_newline: bool,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let mut args: Args = util::get_args(USAGE, argv)?;

    let rconfig = Config::new(&args.arg_input)
        .delimiter(args.flag_delimiter)
        .no_headers(true);
    let mut wconfig = Config::new(&args.flag_output)
        .delimiter(args.flag_out_delimiter)
        .crlf(args.flag_crlf);

    if args.flag_ascii {
        wconfig = wconfig
            .delimiter(Some(Delimiter(b'\x1f')))
            .terminator(csv::Terminator::Any(b'\x1e'));
        args.flag_quote = Delimiter(b'\x1a');
    }
    if args.flag_quote_always {
        wconfig = wconfig.quote_style(csv::QuoteStyle::Always);
    } else if args.flag_quote_never {
        wconfig = wconfig.quote_style(csv::QuoteStyle::Never);
    }
    if let Some(escape) = args.flag_escape {
        wconfig = wconfig.escape(Some(escape.as_byte())).double_quote(false);
    }
    wconfig = wconfig.quote(args.flag_quote.as_byte());

    let mut rdr = rconfig.reader()?;
    let mut wtr = wconfig.writer()?;
    let mut wsconfig = (wconfig).clone();

    wsconfig.path = Some(
        tempfile::NamedTempFile::new()?
            .into_temp_path()
            .to_path_buf(),
    );

    let mut temp_writer = wsconfig.writer()?;
    let mut current_record = csv::ByteRecord::new();
    let mut next_record = csv::ByteRecord::new();
    let mut is_last_record;
    let mut records_exist = rdr.read_byte_record(&mut current_record)?;
    while records_exist {
        is_last_record = !rdr.read_byte_record(&mut next_record)?;
        if is_last_record {
            // If it's the last record and the --no-final-newline flag is set,
            // write the record to a temporary file, then read the file into a string.
            // Remove the last character (the newline) from the string, then write the string to the
            // output.
            temp_writer.write_record(&current_record)?;
            temp_writer.flush()?;
            let mut temp_string = match std::fs::read_to_string(
                wsconfig.path.as_ref().ok_or("Temp file path not found")?,
            ) {
                Ok(s) => s,
                Err(e) => return fail_clierror!("Error reading from temp file: {}", e),
            };
            if args.flag_no_final_newline {
                temp_string.pop();
            }
            match wtr.into_inner() {
                Ok(mut writer) => writer.write_all(temp_string.as_bytes())?,
                Err(e) => return fail_clierror!("Error writing to output: {}", e),
            };
            break;
        }
        wtr.write_record(&current_record)?;
        wtr.write_record(&next_record)?;
        records_exist = rdr.read_byte_record(&mut current_record)?;
    }

    // we don't flush the writer explicitly
    // because it will cause a borrow error
    // let's just let it drop and flush itself implicitly
    Ok(())
}
