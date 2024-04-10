static USAGE: &str = r#"
Returns the rows in the range specified (starting at 0, half-open interval).
The range does not include headers.

If the start of the range isn't specified, then the slice starts from the first
record in the CSV data.

If the end of the range isn't specified, then the slice continues to the last
record in the CSV data.

This operation can be made much faster by creating an index with 'qsv index'
first. Namely, a slice on an index requires parsing just the rows that are
sliced. Without an index, all rows up to the first row in the slice must be
parsed.

Usage:
    qsv slice [options] [<input>]
    qsv slice --help

slice options:
    -s, --start <arg>      The index of the record to slice from.
                           If negative, starts from the last record.
    -e, --end <arg>        The index of the record to slice to.
    -l, --len <arg>        The length of the slice (can be used instead
                           of --end).
    -i, --index <arg>      Slice a single record (shortcut for -s N -l 1).
                           If negative, starts from the last record.
    --json                 Output the result as JSON. Fields are written
                           as key-value pairs. The key is the column name.
                           The value is the field value. The output is a
                           JSON array. If --no-headers is set, then
                           the keys are the column indices (zero-based).

Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
    -n, --no-headers       When set, the first row will not be interpreted
                           as headers. Otherwise, the first row will always
                           appear in the output as the header row.
    -d, --delimiter <arg>  The field delimiter for reading CSV data.
                           Must be a single character. (default: ,)
"#;

use std::{fs, io, io::Write, sync::OnceLock};

use serde::Deserialize;

use crate::{
    config,
    config::{Config, Delimiter},
    index::Indexed,
    util, CliResult,
};

static NULL_VAL: OnceLock<String> = OnceLock::new();

#[allow(clippy::unsafe_derive_deserialize)]
#[derive(Deserialize)]
struct Args {
    arg_input:       Option<String>,
    flag_start:      Option<isize>,
    flag_end:        Option<usize>,
    flag_len:        Option<usize>,
    flag_index:      Option<isize>,
    flag_json:       bool,
    flag_output:     Option<String>,
    flag_no_headers: bool,
    flag_delimiter:  Option<Delimiter>,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;

    // set this once, as this is used repeatedly in a hot loop
    NULL_VAL.set("null".to_string()).unwrap();

    match args.rconfig().indexed()? {
        None => args.no_index(),
        Some(idxed) => args.with_index(idxed),
    }
}

impl Args {
    fn create_json_writer(&self) -> io::Result<Box<dyn Write + Send + 'static>> {
        // create a JSON writer
        // if flag_output is None or "-" then write to stdout
        let output = self.flag_output.as_ref().map_or("-", |s| s.as_str());
        let writer: Box<dyn Write + Send + 'static> = match output {
            "-" => Box::new(io::BufWriter::with_capacity(
                config::DEFAULT_WTR_BUFFER_CAPACITY,
                io::stdout(),
            )),
            _ => Box::new(io::BufWriter::with_capacity(
                config::DEFAULT_WTR_BUFFER_CAPACITY,
                fs::File::create(output)?,
            )),
        };
        Ok(writer)
    }

    #[inline]
    fn write_json(
        &self,
        headers: &csv::ByteRecord,
        records: impl Iterator<Item = csv::ByteRecord>,
    ) -> CliResult<()> {
        let mut json_wtr = self.create_json_writer()?;

        let header_vec: Vec<String> = headers
            .iter()
            .enumerate()
            .map(|(col_idx, b)| {
                if self.flag_no_headers {
                    col_idx.to_string()
                } else {
                    String::from_utf8_lossy(b).to_string()
                }
            })
            .collect();

        // Write the opening bracket for the JSON array
        write!(json_wtr, "[")?;
        let mut is_first = true;

        let rec_len = header_vec.len().saturating_sub(1);
        let mut temp_val;
        let mut json_string_val: serde_json::Value;
        for record in records {
            if !is_first {
                // Write a comma before each record except the first one
                write!(json_wtr, ",")?;
            }
            write!(json_wtr, "{{")?;
            for (idx, b) in record.iter().enumerate() {
                if let Ok(val) = simdutf8::basic::from_utf8(b) {
                    temp_val = val.to_owned();
                } else {
                    temp_val = String::from_utf8_lossy(b).to_string();
                }
                if temp_val.is_empty() {
                    temp_val.clone_from(NULL_VAL.get().unwrap());
                } else {
                    // we round-trip the value to serde_json::Value
                    // to escape the string properly per JSON spec
                    json_string_val = serde_json::Value::String(temp_val);
                    temp_val = json_string_val.to_string();
                }
                // safety: idx is always in bounds
                // so we can get_unchecked here
                if idx < rec_len {
                    unsafe {
                        write!(
                            &mut json_wtr,
                            "\"{key}\":{value},",
                            key = header_vec.get_unchecked(idx),
                            value = temp_val
                        )?;
                    }
                } else {
                    unsafe {
                        write!(
                            &mut json_wtr,
                            "\"{key}\":{value}",
                            key = header_vec.get_unchecked(idx),
                            value = temp_val
                        )?;
                    }
                }
            }
            write!(json_wtr, "}}")?;
            is_first = false;
        }
        writeln!(json_wtr, "]")?;
        Ok(json_wtr.flush()?)
    }

    fn no_index(&self) -> CliResult<()> {
        let mut rdr = self.rconfig().reader()?;

        let (start, end) = self.range()?;
        if self.flag_json {
            let headers = rdr.byte_headers()?.clone();
            let records = rdr
                .byte_records()
                .skip(start)
                .take(end - start)
                .map(|r| r.unwrap());
            self.write_json(&headers, records)
        } else {
            let mut wtr = self.wconfig().writer()?;
            self.rconfig().write_headers(&mut rdr, &mut wtr)?;
            for r in rdr.byte_records().skip(start).take(end - start) {
                wtr.write_byte_record(&r?)?;
            }
            Ok(wtr.flush()?)
        }
    }

    fn with_index(&self, mut indexed_file: Indexed<fs::File, fs::File>) -> CliResult<()> {
        let (start, end) = self.range()?;
        if end - start == 0 {
            return Ok(());
        }
        indexed_file.seek(start as u64)?;
        if self.flag_json {
            let headers = indexed_file.byte_headers()?.clone();
            let records = indexed_file
                .byte_records()
                .take(end - start)
                .map(|r| r.unwrap());
            self.write_json(&headers, records)
        } else {
            let mut wtr = self.wconfig().writer()?;
            self.rconfig().write_headers(&mut *indexed_file, &mut wtr)?;
            for r in indexed_file.byte_records().take(end - start) {
                wtr.write_byte_record(&r?)?;
            }
            Ok(wtr.flush()?)
        }
    }

    fn range(&self) -> CliResult<(usize, usize)> {
        let mut start = None;
        if let Some(start_arg) = self.flag_start {
            if start_arg < 0 {
                start = Some(
                    (util::count_rows(&self.rconfig())? as usize)
                        .abs_diff(start_arg.unsigned_abs()),
                );
            } else {
                start = Some(start_arg as usize);
            }
        }
        let index = if let Some(flag_index) = self.flag_index {
            if flag_index < 0 {
                let index = (util::count_rows(&self.rconfig())? as usize)
                    .abs_diff(flag_index.unsigned_abs());
                Some(index)
            } else {
                Some(flag_index as usize)
            }
        } else {
            None
        };
        Ok(util::range(start, self.flag_end, self.flag_len, index)?)
    }

    fn rconfig(&self) -> Config {
        Config::new(&self.arg_input)
            .delimiter(self.flag_delimiter)
            .no_headers(self.flag_no_headers)
    }

    fn wconfig(&self) -> Config {
        Config::new(&self.flag_output)
    }
}
