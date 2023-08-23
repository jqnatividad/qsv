static USAGE: &str = r#"
Find the difference between two CSVs.

Note that diff does not support stdin. A file path is required for both arguments.

Usage:
    qsv diff [options] [<input-left>] [<input-right>]
    qsv diff --help

For examples, see https://github.com/jqnatividad/qsv/blob/master/tests/test_diff.rs

diff options:
    --no-headers-left           When set, the first row will be considered as part of
                                the left CSV to diff. (When not set, the
                                first row is the header row and will be skipped during
                                the diff. It will always appear in the output.)
    --no-headers-right          When set, the first row will be considered as part of
                                the right CSV to diff. (When not set, the
                                first row is the header row and will be skipped during
                                the diff. It will always appear in the output.)
    --delimiter-left <arg>      The field delimiter for reading CSV data on the left.
                                Must be a single character. (default: ,)
    --delimiter-right <arg>     The field delimiter for reading CSV data on the right.
                                Must be a single character. (default: ,)
    -k, --key <arg...>          The column indices that uniquely identify a record
                                as a comma separated list of indices, e.g. 0,1,2.
                                (default: 0)
    --sort-columns <arg...>     The column indices by which the diff result should be
                                sorted as a comma separated list of indices, e.g. 0,1,2.
                                Records in the diff result that are marked as "modified"
                                ("delete" and "add" records that have the same key,
                                but have different content) will always be kept together
                                in the sorted diff result and so won't be sorted
                                independently from each other.
    -j, --jobs <arg>            The number of jobs to run in parallel.
                                When not set, the number of jobs is set to the number
                                of CPUs detected.

Common options:
    -h, --help                  Display this message
    -o, --output <file>         Write output to <file> instead of stdout.
"#;

use std::io::{self, Read, Write};

use csv_diff::{csv_diff::CsvByteDiffBuilder, diff_row::DiffByteRecord};
use serde::Deserialize;

use crate::{
    clitypes::CliError,
    config::{Config, Delimiter},
    util, CliResult,
};

#[derive(Deserialize)]
struct Args {
    arg_input_left:        Option<String>,
    arg_input_right:       Option<String>,
    flag_output:           Option<String>,
    flag_jobs:             Option<usize>,
    flag_no_headers_left:  bool,
    flag_no_headers_right: bool,
    flag_delimiter_left:   Option<Delimiter>,
    flag_delimiter_right:  Option<Delimiter>,
    flag_key:              Option<String>,
    flag_sort_columns:     Option<String>,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;

    let rconfig_left = Config::new(&args.arg_input_left)
        .delimiter(args.flag_delimiter_left)
        .no_headers(args.flag_no_headers_left);

    let rconfig_right = Config::new(&args.arg_input_right)
        .delimiter(args.flag_delimiter_right)
        .no_headers(args.flag_no_headers_right);

    if rconfig_left.is_stdin() || rconfig_right.is_stdin() {
        return fail_incorrectusage_clierror!(
            "diff does not support stdin. A file path is required for both arguments."
        );
    }

    let primary_key_cols = match args.flag_key {
        None => vec![0],
        Some(s) => s
            .split(',')
            .map(str::parse::<usize>)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|err| CliError::Other(err.to_string()))?,
    };

    let sort_cols = args
        .flag_sort_columns
        .map(|s| {
            s.split(',')
                .map(str::parse::<usize>)
                .collect::<Result<Vec<_>, _>>()
                .map_err(|err| CliError::Other(err.to_string()))
        })
        .transpose()?;

    let wtr = Config::new(&args.flag_output).writer()?;
    let mut csv_rdr_left = rconfig_left.reader()?;
    let mut csv_rdr_right = rconfig_right.reader()?;

    // set RAYON_NUM_THREADS
    util::njobs(args.flag_jobs);

    let mut csv_diff_writer = CsvDiffWriter::new(wtr);
    csv_diff_writer.write_headers(&mut csv_rdr_left, &mut csv_rdr_right)?;

    let Ok(csv_diff) = CsvByteDiffBuilder::new()
        .primary_key_columns(primary_key_cols)
        .build()
    else {
        return fail_clierror!("Cannot instantiate diff");
    };

    let mut diff_byte_records = csv_diff
        .diff(csv_rdr_left.into(), csv_rdr_right.into())
        .try_to_diff_byte_records()?;

    match sort_cols {
        Some(sort_cols) => {
            diff_byte_records
                .sort_by_columns(sort_cols)
                .map_err(|e| CliError::Other(e.to_string()))?;
        },
        None => {
            diff_byte_records.sort_by_line();
        },
    }

    Ok(csv_diff_writer.write_diff_byte_records(diff_byte_records)?)
}

struct CsvDiffWriter<W: Write> {
    csv_writer: csv::Writer<W>,
}

impl<W: Write> CsvDiffWriter<W> {
    fn new(csv_writer: csv::Writer<W>) -> Self {
        Self { csv_writer }
    }

    fn write_headers<R: Read>(
        &mut self,
        rdr_left: &mut csv::Reader<R>,
        rdr_right: &mut csv::Reader<R>,
    ) -> csv::Result<()> {
        match (rdr_left.has_headers(), rdr_right.has_headers()) {
            (true, true) => {
                let rdr_bh = rdr_left.byte_headers()?;

                rdr_bh.write_diffresult_header(&mut self.csv_writer)?;
                // we also read the headers from the right CSV, so that both readers end up
                // before the actual records. Otherwise, it would lead to errors when we
                // diff the CSVs, because the header of one CSV would have been read and the other
                // not.
                #[allow(clippy::let_underscore_untyped)]
                let _ = rdr_right.byte_headers()?;
            },
            (true, false) => {
                let rdr_bh = rdr_left.byte_headers()?;

                rdr_bh.write_diffresult_header(&mut self.csv_writer)?;
            },
            (false, true) => {
                let rdr_bh = rdr_right.byte_headers()?;

                rdr_bh.write_diffresult_header(&mut self.csv_writer)?;
            },
            // nothing to do, because there are no headers
            (false, false) => {},
        }

        Ok(())
    }

    fn write_diff_byte_records(
        &mut self,
        diff_byte_records: impl IntoIterator<Item = DiffByteRecord>,
    ) -> io::Result<()> {
        for dbr in diff_byte_records {
            self.write_diff_byte_record(&dbr)?;
        }
        self.csv_writer.flush()?;
        Ok(())
    }

    fn write_diff_byte_record(&mut self, diff_byte_record: &DiffByteRecord) -> csv::Result<()> {
        let add_sign: &[u8] = &b"+"[..];
        let remove_sign: &[u8] = &b"-"[..];

        match diff_byte_record {
            DiffByteRecord::Add(add) => {
                let mut vec = vec![add_sign];
                vec.extend(add.byte_record());
                self.csv_writer.write_record(vec)
            },
            DiffByteRecord::Modify {
                delete,
                add,
                // TODO: this should be used in the future to highlight the column where differences
                // occur
                field_indices: _field_indices,
            } => {
                let mut vec_del = vec![remove_sign];
                vec_del.extend(delete.byte_record());
                self.csv_writer.write_record(vec_del)?;

                let mut vec_add = vec![add_sign];
                vec_add.extend(add.byte_record());
                self.csv_writer.write_record(vec_add)
            },
            DiffByteRecord::Delete(del) => {
                let mut vec = vec![remove_sign];
                vec.extend(del.byte_record());
                self.csv_writer.write_record(vec)
            },
        }
    }
}

trait WriteDiffResultHeader {
    fn write_diffresult_header<W: Write>(&self, csv_writer: &mut csv::Writer<W>)
        -> csv::Result<()>;
}

impl WriteDiffResultHeader for csv::ByteRecord {
    fn write_diffresult_header<W: Write>(
        &self,
        csv_writer: &mut csv::Writer<W>,
    ) -> csv::Result<()> {
        if !self.is_empty() {
            let mut new_header = vec![&b"diffresult"[..]];
            new_header.extend(self);
            csv_writer.write_record(new_header)?;
        }
        Ok(())
    }
}
