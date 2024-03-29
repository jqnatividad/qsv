static USAGE: &str = r#"
Removes a set of CSV data from another set based on the specified columns.

Also can compute the intersection of two CSV sets with the -v flag.

Matching is always done by ignoring leading and trailing whitespace. By default,
matching is done case sensitively, but this can be disabled with the --ignore-case
flag.

The columns arguments specify the columns to match for each input. Columns can
be referenced by name or index, starting at 1. Specify multiple columns by
separating them with a comma. Specify a range of columns with `-`. Both
columns1 and columns2 must specify exactly the same number of columns.
(See 'qsv select --help' for the full syntax.)

Examples:

    qsv exclude id records.csv id previously-processed.csv
    qsv exclude col1,col2 records.csv col1,col2 previously-processed.csv
    qsv exclude col1-col5 records.csv col1-col5 previously-processed.csv
    qsv exclude id records.csv id previously-processed.csv > new-records.csv
    qsv exclude id records.csv id previously-processed.csv --output new-records.csv
    qsv exclude -v id records.csv id previously-processed.csv -o intersection.csv
    qsv exclude --ignore-case id records.csv id previously-processed.csv
    qsv exclude id records.csv id previously-processed.csv |
       qsv sort > new-sorted-records.csv
    qsv exclude id records.csv id previously-processed.csv | qsv sort |
       qsv --sorted dedup > new-sorted-deduped-records.csv

For more examples, see https://github.com/jqnatividad/qsv/blob/master/tests/test_exclude.rs.

Usage:
    qsv exclude [options] <columns1> <input1> <columns2> <input2>
    qsv exclude --help

input arguments:
    <input1> is the file from which data will be removed.
    <input2> is the file containing the data to be removed from <input1> 
     e.g. 'qsv exclude id records.csv id previously-processed.csv'

exclude options:
    -i, --ignore-case      When set, matching is done case insensitively.
    -v                     When set, matching rows will be the only ones included,
                           forming set intersection, instead of the ones discarded.

Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
    -n, --no-headers       When set, the first row will not be interpreted
                           as headers. (i.e., They are not searched, analyzed,
                           sliced, etc.)
    -d, --delimiter <arg>  The field delimiter for reading CSV data.
                           Must be a single character. (default: ,)
"#;

use std::{collections::hash_map::Entry, fs, io, str};

use ahash::AHashMap;
use byteorder::{BigEndian, WriteBytesExt};
use serde::Deserialize;

use crate::{
    config::{Config, Delimiter},
    index::Indexed,
    select::{SelectColumns, Selection},
    util,
    util::ByteString,
    CliResult,
};

#[derive(Deserialize)]
struct Args {
    arg_columns1:     SelectColumns,
    arg_input1:       String,
    arg_columns2:     SelectColumns,
    arg_input2:       String,
    flag_v:           bool,
    flag_output:      Option<String>,
    flag_no_headers:  bool,
    flag_ignore_case: bool,
    flag_delimiter:   Option<Delimiter>,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;
    let mut state = args.new_io_state()?;
    state.write_headers()?;
    state.exclude(args.flag_v)
}

struct IoState<R, W: io::Write> {
    wtr:        csv::Writer<W>,
    rdr1:       csv::Reader<R>,
    sel1:       Selection,
    rdr2:       csv::Reader<R>,
    sel2:       Selection,
    no_headers: bool,
    casei:      bool,
}

impl<R: io::Read + io::Seek, W: io::Write> IoState<R, W> {
    fn write_headers(&mut self) -> CliResult<()> {
        if !self.no_headers {
            let headers = self.rdr1.byte_headers()?.clone();
            self.wtr.write_record(&headers)?;
        }
        Ok(())
    }

    fn exclude(mut self, invert: bool) -> CliResult<()> {
        // amortize allocations
        #[allow(unused_assignments)]
        let mut curr_row = csv::ByteRecord::new();

        let validx = ValueIndex::new(self.rdr2, &self.sel2, self.casei)?;
        for row in self.rdr1.byte_records() {
            curr_row = row?;
            let key = get_row_key(&self.sel1, &curr_row, self.casei);
            if let Some(_rows) = validx.values.get(&key) {
                if invert {
                    self.wtr.write_record(curr_row.iter())?;
                }
            } else if !invert {
                self.wtr.write_record(curr_row.iter())?;
            }
        }
        Ok(())
    }
}

impl Args {
    fn new_io_state(&self) -> CliResult<IoState<fs::File, Box<dyn io::Write + 'static>>> {
        let rconf1 = Config::new(&Some(self.arg_input1.clone()))
            .delimiter(self.flag_delimiter)
            .no_headers(self.flag_no_headers)
            .select(self.arg_columns1.clone());
        let rconf2 = Config::new(&Some(self.arg_input2.clone()))
            .delimiter(self.flag_delimiter)
            .no_headers(self.flag_no_headers)
            .select(self.arg_columns2.clone());

        let mut rdr1 = rconf1.reader_file()?;
        let mut rdr2 = rconf2.reader_file()?;
        let (sel1, sel2) = self.get_selections(&rconf1, &mut rdr1, &rconf2, &mut rdr2)?;
        Ok(IoState {
            wtr: Config::new(&self.flag_output).writer()?,
            rdr1,
            sel1,
            rdr2,
            sel2,
            no_headers: rconf1.no_headers,
            casei: self.flag_ignore_case,
        })
    }

    #[allow(clippy::unused_self)]
    fn get_selections<R: io::Read>(
        &self,
        rconf1: &Config,
        rdr1: &mut csv::Reader<R>,
        rconf2: &Config,
        rdr2: &mut csv::Reader<R>,
    ) -> CliResult<(Selection, Selection)> {
        let headers1 = rdr1.byte_headers()?;
        let headers2 = rdr2.byte_headers()?;
        let select1 = rconf1.selection(headers1)?;
        let select2 = rconf2.selection(headers2)?;
        if select1.len() != select2.len() {
            return fail_incorrectusage_clierror!(
                "Column selections must have the same number of columns, but found column \
                 selections with {} and {} columns.",
                select1.len(),
                select2.len()
            );
        }
        Ok((select1, select2))
    }
}

#[allow(dead_code)]
struct ValueIndex<R> {
    // This maps tuples of values to corresponding rows.
    values:   AHashMap<Vec<ByteString>, Vec<usize>>,
    idx:      Indexed<R, io::Cursor<Vec<u8>>>,
    num_rows: usize,
}

impl<R: io::Read + io::Seek> ValueIndex<R> {
    fn new(mut rdr: csv::Reader<R>, sel: &Selection, casei: bool) -> CliResult<ValueIndex<R>> {
        let mut val_idx = AHashMap::with_capacity(10000);
        let mut row_idx = io::Cursor::new(Vec::with_capacity(8 * 10000));
        let (mut rowi, mut count) = (0_usize, 0_usize);

        // This logic is kind of tricky. Basically, we want to include
        // the header row in the line index (because that's what csv::index
        // does), but we don't want to include header values in the ValueIndex.
        if rdr.has_headers() {
            // ... so if there are headers, we make sure that we've parsed
            // them, and write the offset of the header row to the index.
            rdr.byte_headers()?;
            row_idx.write_u64::<BigEndian>(0)?;
            count += 1;
        } else {
            // ... and if there are no headers, we seek to the beginning and
            // index everything.
            let mut pos = csv::Position::new();
            pos.set_byte(0);
            rdr.seek(pos)?;
        }

        let mut row = csv::ByteRecord::new();
        while rdr.read_byte_record(&mut row)? {
            // This is a bit hokey. We're doing this manually instead of using
            // the `csv-index` crate directly so that we can create both
            // indexes in one pass.
            row_idx.write_u64::<BigEndian>(row.position().unwrap().byte())?;

            let fields: Vec<_> = sel
                .select(&row)
                .map(|v| util::transform(v, casei))
                .collect();
            match val_idx.entry(fields) {
                Entry::Vacant(v) => {
                    let mut rows = Vec::with_capacity(4);
                    rows.push(rowi);
                    v.insert(rows);
                },
                Entry::Occupied(mut v) => {
                    v.get_mut().push(rowi);
                },
            }
            rowi += 1;
            count += 1;
        }

        row_idx.write_u64::<BigEndian>(count as u64)?;
        let idx = Indexed::open(rdr, io::Cursor::new(row_idx.into_inner()))?;
        Ok(ValueIndex {
            values: val_idx,
            idx,
            num_rows: rowi,
        })
    }
}

// This is just for debugging, so comment out for now.
// use std::fmt;
// impl<R> fmt::Debug for ValueIndex<R> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         // Sort the values by order of first appearance.
//         let mut kvs = self.values.iter().collect::<Vec<_>>();
//         kvs.sort_by(|&(_, v1), &(_, v2)| v1[0].cmp(&v2[0]));
//         for (keys, rows) in kvs {
//             // This is just for debugging, so assume Unicode for now.
//             let keys = keys
//                 .iter()
//                 .map(|k| String::from_utf8(k.clone()).unwrap())
//                 .collect::<Vec<_>>();
//             writeln!(f, "({}) => {rows:?}", keys.join(", "))?;
//         }
//         Ok(())
//     }
// }

#[inline]
fn get_row_key(sel: &Selection, row: &csv::ByteRecord, casei: bool) -> Vec<ByteString> {
    sel.select(row).map(|v| util::transform(v, casei)).collect()
}
