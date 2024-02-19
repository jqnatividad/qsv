static USAGE: &str = r#"
Concatenate CSV files by row or by column.

When concatenating by column, the columns will be written in the same order as
the inputs given. The number of rows in the result is always equivalent to
the minimum number of rows across all given CSV data. (This behavior can be
reversed with the '--pad' flag.)

Concatenating by rows can be done in two ways:

'rows' subcommand: 
   All CSV data must have the same number of columns (unless --flexible is enabled)
   and in the same order. 
   If you need to rearrange the columns or fix the lengths of records, use the
   'select' or 'fixlengths' commands. Also, only the headers of the *first* CSV
   data given are used. Headers in subsequent inputs are ignored. (This behavior
   can be disabled with --no-headers.)

'rowskey' subcommand:
   CSV data can have different numbers of columns and in different orders. All
   columns are written in insertion order. Does not work with --no-headers, as
   the column header names are used as keys.

For examples, see https://github.com/jqnatividad/qsv/blob/master/tests/test_cat.rs.

Usage:
    qsv cat rows    [options] [<input>...]
    qsv cat rowskey [options] [<input>...]
    qsv cat columns [options] [<input>...]
    qsv cat --help

cat arguments:
    <input>...              The CSV file(s) to read. Use '-' for standard input.
                            If input is a directory, all files in the directory will
                            be read as input.
                            If the input is a file with a '.infile-list' extension,
                            the file will be read as a list of input files.
                            If the input are snappy-compressed files(s), it will be
                            decompressed automatically.

cat options:
                             COLUMNS OPTION:
    -p, --pad                When concatenating columns, this flag will cause
                             all records to appear. It will pad each row if
                             other CSV data isn't long enough.

                             ROWS OPTION:
    --flexible               When concatenating rows, this flag turns off validation
                             that the input and output CSVs have the same number of columns.
                             This is faster, but may result in invalid CSV data.

                             ROWSKEY OPTIONS:
    -g, --group <grpkind>    When concatenating with rowskey, you can specify a grouping value
                             which will be used as the first column in the output. This is useful
                             when you want to know which file a row came from. Valid values are
                             'fullpath', 'parentdirfname', 'parentdirfstem', 'fname', 'fstem' and 'none'.
                             A new column will be added to the beginning of each row using --group-name.
                             If 'none' is specified, no grouping column will be added.
                             [default: none]
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

use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

use indexmap::{IndexMap, IndexSet};
use serde::Deserialize;
use strum_macros::EnumString;

use crate::{
    config::{Config, Delimiter},
    util, CliResult,
};

#[derive(Deserialize)]
struct Args {
    cmd_rows:        bool,
    cmd_rowskey:     bool,
    cmd_columns:     bool,
    flag_group:      String,
    flag_group_name: String,
    arg_input:       Vec<PathBuf>,
    flag_pad:        bool,
    flag_flexible:   bool,
    flag_output:     Option<String>,
    flag_no_headers: bool,
    flag_delimiter:  Option<Delimiter>,
}

#[derive(Debug, EnumString, PartialEq)]
#[strum(ascii_case_insensitive)]
enum GroupKind {
    FullPath,
    ParentDirFName,
    ParentDirFStem,
    FName,
    FStem,
    None,
}

fn get_parentdir_and_file(path: &Path, stem_only: bool) -> String {
    //safety: we know that this is a valid pathbuf
    let file_info = if stem_only {
        path.file_stem()
    } else {
        path.file_name()
    }
    .unwrap();

    let parent_dir = path.parent().unwrap();

    parent_dir.join(file_info).to_string_lossy().into_owned()
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let mut args: Args = util::get_args(USAGE, argv)?;

    let tmpdir = tempfile::tempdir()?;
    args.arg_input = util::process_input(args.arg_input, &tmpdir, "")?;
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
    #[inline]
    fn configs(&self) -> CliResult<Vec<Config>> {
        util::many_configs(
            &self.arg_input,
            self.flag_delimiter,
            self.flag_no_headers,
            // we can set flexible to true if we are using rowskey
            // as we don't need to validate that the number of columns
            // are the same across all files, increasing performance
            self.flag_flexible || self.cmd_rowskey,
        )
        .map_err(From::from)
    }

    fn cat_rows(&self) -> CliResult<()> {
        let mut row = csv::ByteRecord::new();
        let mut wtr = Config::new(&self.flag_output)
            .flexible(self.flag_flexible)
            .writer()?;
        let mut rdr;

        let mut configs = self.configs()?.into_iter();

        // the first file is special, as it has the headers
        // if --no-headers is set, we just write the first file
        if let Some(conf) = configs.next() {
            rdr = conf.reader()?;
            conf.write_headers(&mut rdr, &mut wtr)?;
            while rdr.read_byte_record(&mut row)? {
                wtr.write_byte_record(&row)?;
            }
        }

        // the rest of the files are just written
        // as fast as possible, as we don't need to
        // worry about headers
        for conf in configs {
            rdr = conf.reader()?;
            while rdr.read_byte_record(&mut row)? {
                wtr.write_byte_record(&row)?;
            }
        }

        Ok(wtr.flush()?)
    }

    // this algorithm is largely inspired by https://github.com/vi/csvcatrow by @vi
    // https://github.com/jqnatividad/qsv/issues/527
    fn cat_rowskey(&self) -> CliResult<()> {
        // ahash is a faster hasher than the default one used by IndexSet and IndexMap
        type AhashIndexSet<T> = IndexSet<T, ahash::RandomState>;
        type AhashIndexMap<T, T2> = IndexMap<T, T2, ahash::RandomState>;

        if self.flag_no_headers {
            return fail_incorrectusage_clierror!(
                "cat rowskey does not support --no-headers, as we use column headers as keys."
            );
        }

        let Ok(group_kind) = GroupKind::from_str(&self.flag_group) else {
            return fail_incorrectusage_clierror!(
                "Invalid grouping value `{}`. Valid values are 'fullpath', 'parentdirfname', \
                 'parentdirfstem', 'fname', 'fstem' and 'none'.",
                self.flag_group
            );
        };

        let mut columns_global: AhashIndexSet<Box<[u8]>> = AhashIndexSet::default();

        if group_kind != GroupKind::None {
            columns_global.insert(self.flag_group_name.as_bytes().to_vec().into_boxed_slice());
        }

        // we're creating a temp_dir in case we have stdin input, as we need to save it to a
        // file named "stdin" under the temp_dir. This is required as we need to scan
        // the files twice. temp_dir will be automatically deleted when it goes out of scope.
        let temp_dir = tempfile::tempdir()?;
        let mut stdin_tempfilename = std::path::PathBuf::new();

        // First pass, add all column headers to an IndexSet
        for conf in &self.configs()? {
            if conf.is_stdin() {
                stdin_tempfilename = temp_dir.path().join("stdin");
                let tmp_file = std::fs::File::create(&stdin_tempfilename)?;
                let mut tmp_file = std::io::BufWriter::new(tmp_file);
                std::io::copy(&mut std::io::stdin(), &mut tmp_file)?;
            }
            let mut rdr = conf.reader()?;
            let header = rdr.byte_headers()?;
            for field in header {
                let fi = field.to_vec().into_boxed_slice();
                columns_global.insert(fi);
            }
        }
        let num_columns_global = columns_global.len();

        // Second pass, write all columns to a new file
        // set flexible to true for faster writes
        // as we know that all columns are already in columns_global and we don't need to
        // validate that the number of columns are the same every time we write a row
        let mut wtr = Config::new(&self.flag_output).flexible(true).writer()?;
        let mut new_row = csv::ByteRecord::with_capacity(500, num_columns_global);

        for c in &columns_global {
            new_row.push_field(c);
        }
        wtr.write_byte_record(&new_row)?;

        // amortize allocations
        let mut grouping_value = String::new();
        let mut conf_path;
        let mut rdr;
        let mut header: &csv::ByteRecord;
        let mut columns_of_this_file: AhashIndexMap<Box<[u8]>, usize> = AhashIndexMap::default();
        columns_of_this_file.reserve(num_columns_global);
        let mut row: csv::ByteRecord = csv::ByteRecord::with_capacity(500, num_columns_global);

        for conf in self.configs()? {
            if conf.is_stdin() {
                rdr = Config::new(&Some(stdin_tempfilename.to_string_lossy().to_string()))
                    .reader()?;
                conf_path = Some(stdin_tempfilename.clone());
            } else {
                rdr = conf.reader()?;
                conf_path = conf.path.clone();
            }
            header = rdr.byte_headers()?;

            columns_of_this_file.clear();

            for (n, field) in header.iter().enumerate() {
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

            // safety: we know that this is a valid file path
            let conf_pathbuf = conf_path.unwrap();

            // set grouping_value
            // safety: we know that this is a valid file path and if the file path
            // is not utf8, we convert it to lossy utf8
            match group_kind {
                GroupKind::FullPath => {
                    grouping_value.clear();
                    grouping_value
                        .push_str(&conf_pathbuf.canonicalize().unwrap().to_string_lossy());
                },
                GroupKind::ParentDirFName => {
                    grouping_value = get_parentdir_and_file(&conf_pathbuf, false);
                },
                GroupKind::ParentDirFStem => {
                    grouping_value = get_parentdir_and_file(&conf_pathbuf, true);
                },
                GroupKind::FName => {
                    grouping_value.clear();
                    grouping_value.push_str(&conf_pathbuf.file_name().unwrap().to_string_lossy());
                },
                GroupKind::FStem => {
                    grouping_value.clear();
                    grouping_value.push_str(&conf_pathbuf.file_stem().unwrap().to_string_lossy());
                },
                GroupKind::None => {},
            }

            let group_flag = group_kind != GroupKind::None;
            let grouping_value_bytes = grouping_value.as_bytes();

            while rdr.read_byte_record(&mut row)? {
                new_row.clear();
                for (col_idx, c) in columns_global.iter().enumerate() {
                    if let Some(idx) = columns_of_this_file.get(c) {
                        if let Some(d) = row.get(*idx) {
                            new_row.push_field(d);
                        } else {
                            new_row.push_field(b"");
                        }
                    } else if group_flag && col_idx == 0 {
                        // we are in the first column, and --group is set
                        // so we write the grouping value
                        new_row.push_field(grouping_value_bytes);
                    } else {
                        new_row.push_field(b"");
                    }
                }
                wtr.write_byte_record(&new_row)?;
            }
        }

        Ok(wtr.flush()?)
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
                    },
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
        Ok(wtr.flush()?)
    }
}
