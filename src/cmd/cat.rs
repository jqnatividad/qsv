static USAGE: &str = r#"
Concatenates CSV data by column or by row.

When concatenating by column, the columns will be written in the same order as
the inputs given. The number of rows in the result is always equivalent to
the minimum number of rows across all given CSV data. (This behavior can be
reversed with the '--pad' flag.)

Concatenating by rows can be done in two ways:

'rows' subcommand: 
   All CSV data must have the same number of columns and in the same order. 
   If you need to rearrange the columns or fix the lengths of records, use the
   'select' or 'fixlengths' commands. Also, only the headers of the *first* CSV
   data given are used. Headers in subsequent inputs are ignored. (This behavior
   can be disabled with --no-headers.)

'rowskey' subcommand:
   CSV data can have different numbers of columns and in different orders. All
   columns are written in insertion order. Does not work with --no-headers, as
   the column header names are used as keys. Nor does it work with stdin, as 
   input files are scanned twice - once for collecting all the column names, and
   the second time for writing the output.

For examples, see https://github.com/jqnatividad/qsv/blob/master/tests/test_cat.rs.

Usage:
    qsv cat rows    [options] [<input>...]
    qsv cat rowskey [options] [<input>...]
    qsv cat columns [options] [<input>...]
    qsv cat --help

cat options:
                             COLUMNS OPTION:
    -p, --pad                When concatenating columns, this flag will cause
                             all records to appear. It will pad each row if
                             other CSV data isn't long enough.

                             ROWSKEY OPTIONS:
    -g, --group              When concatenating with rowskey, use the file stem of each
                             input file as a grouping value. A new column will be added
                             to the beginning of each row with the name given by --group-name.
    -N, --group-name <arg>   When concatenating with rowskey, this flag provides the name
                             for the new grouping column. [default: file]
                             
Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
    -n, --no-headers       When set, the first row will NOT be interpreted
                           as column names. Note that this has no effect when
                           concatenating columns.
    -d, --delimiter <arg>  The field delimiter for reading CSV data.
                           Must be a single character. (default: ,)
"#;

use indexmap::{IndexMap, IndexSet};
use serde::Deserialize;

use crate::{
    config::{Config, Delimiter},
    util, CliResult,
};

#[derive(Deserialize)]
struct Args {
    cmd_rows:        bool,
    cmd_rowskey:     bool,
    cmd_columns:     bool,
    flag_group:      bool,
    flag_group_name: String,
    arg_input:       Vec<String>,
    flag_pad:        bool,
    flag_output:     Option<String>,
    flag_no_headers: bool,
    flag_delimiter:  Option<Delimiter>,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;
    if args.cmd_rows {
        args.cat_rows()
    } else if args.cmd_rowskey {
        args.cat_rowskey()
    } else if args.cmd_columns {
        args.cat_columns()
    } else {
        unreachable!();
    }
}

impl Args {
    fn configs(&self) -> CliResult<Vec<Config>> {
        util::many_configs(&self.arg_input, self.flag_delimiter, self.flag_no_headers)
            .map_err(From::from)
    }

    fn cat_rows(&self) -> CliResult<()> {
        let mut row = csv::ByteRecord::new();
        let mut wtr = Config::new(&self.flag_output).writer()?;
        for (i, conf) in self.configs()?.into_iter().enumerate() {
            let mut rdr = conf.reader()?;
            if i == 0 {
                conf.write_headers(&mut rdr, &mut wtr)?;
            }
            while rdr.read_byte_record(&mut row)? {
                wtr.write_byte_record(&row)?;
            }
        }
        wtr.flush().map_err(From::from)
    }

    fn cat_rowskey(&self) -> CliResult<()> {
        // this algorithm is largely inspired by https://github.com/vi/csvcatrow by @vi
        if self.flag_no_headers {
            return fail_clierror!(
                "cat rowskey does not support --no-headers, as we use column headers as keys."
            );
        }
        let mut columns_global: IndexSet<Box<[u8]>> = IndexSet::with_capacity(32);

        if self.flag_group {
            columns_global.insert(self.flag_group_name.as_bytes().to_vec().into_boxed_slice());
        }

        // First pass, add all column headers to an IndexSet
        for conf in &self.configs()? {
            if conf.is_stdin() {
                return fail_clierror!(
                    "cat rowskey does not support stdin, as we need to scan files twice."
                );
            }
            let mut rdr = conf.reader()?;
            let h = rdr.byte_headers()?;
            for field in h {
                let fi = field.to_vec().into_boxed_slice();
                columns_global.insert(fi);
            }
        }
        let num_columns_global = columns_global.len();

        // Second pass, write all columns to a new file
        let mut wtr = Config::new(&self.flag_output).writer()?;
        for c in &columns_global {
            wtr.write_field(c)?;
        }
        wtr.write_byte_record(&csv::ByteRecord::new())?;

        #[allow(unused_assignments)]
        let mut grouping_value = String::with_capacity(64); // amortize allocation

        for conf in self.configs()? {
            let mut rdr = conf.reader()?;
            let h = rdr.byte_headers()?;

            let mut columns_of_this_file = IndexMap::with_capacity(num_columns_global);

            for (n, field) in h.iter().enumerate() {
                let fi = field.to_vec().into_boxed_slice();
                if columns_of_this_file.contains_key(&fi) {
                    wwarn!(
                        "Duplicate column `{}` name in file `{:?}`.",
                        String::from_utf8_lossy(&fi),
                        conf.path,
                    );
                }
                columns_of_this_file.insert(fi, n);
            }

            // use the file stem as the grouping value
            // safety: we know that this is a file path
            grouping_value = conf
                .path
                .clone()
                .unwrap()
                .file_stem()
                .unwrap()
                .to_string_lossy()
                .to_string();

            for row in rdr.byte_records() {
                let row = row?;
                for (col_idx, c) in columns_global.iter().enumerate() {
                    if let Some(idx) = columns_of_this_file.get(c) {
                        if let Some(d) = row.get(*idx) {
                            wtr.write_field(d)?;
                        } else {
                            wtr.write_field(b"")?;
                        }
                    } else if self.flag_group && col_idx == 0 {
                        wtr.write_field(&grouping_value)?;
                    } else {
                        wtr.write_field(b"")?;
                    }
                }
                wtr.write_byte_record(&csv::ByteRecord::new())?;
            }
        }
        wtr.flush().map_err(From::from)
    }

    fn cat_columns(&self) -> CliResult<()> {
        let mut wtr = Config::new(&self.flag_output).writer()?;
        let mut rdrs = self
            .configs()?
            .into_iter()
            .map(|conf| conf.no_headers(true).reader())
            .collect::<Result<Vec<_>, _>>()?;

        // Find the lengths of each record. If a length varies, then an error
        // will occur so we can rely on the first length being the correct one.
        let mut lengths = vec![];
        for rdr in &mut rdrs {
            lengths.push(rdr.byte_headers()?.len());
        }

        let mut iters = rdrs
            .iter_mut()
            .map(csv::Reader::byte_records)
            .collect::<Vec<_>>();
        'OUTER: loop {
            let mut record = csv::ByteRecord::new();
            let mut num_done = 0;
            for (iter, &len) in iters.iter_mut().zip(lengths.iter()) {
                match iter.next() {
                    None => {
                        num_done += 1;
                        if self.flag_pad {
                            for _ in 0..len {
                                record.push_field(b"");
                            }
                        } else {
                            break 'OUTER;
                        }
                    }
                    Some(Err(err)) => return fail!(err),
                    Some(Ok(next)) => record.extend(&next),
                }
            }
            // Only needed when `--pad` is set.
            // When not set, the OUTER loop breaks when the shortest iterator
            // is exhausted.
            if num_done >= iters.len() {
                break 'OUTER;
            }
            wtr.write_byte_record(&record)?;
        }
        wtr.flush().map_err(From::from)
    }
}
