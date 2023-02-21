static USAGE: &str = r#"
Joins two sets of CSV data on the specified columns using the pola.rs engine.

The default join operation is an 'inner' join. This corresponds to the
intersection of rows on the keys specified.

The columns arguments specify the columns to join for each input. Columns can
be referenced by name. Specify multiple columns by separating them with a comma.
Both columns1 and columns2 must specify exactly the same number of columns.

For examples, see https://github.com/jqnatividad/qsv/blob/master/tests/test_joinp.rs.

Usage:
    qsv joinp [options] <columns1> <input1> <columns2> <input2>
    qsv joinp --help

input parameters:
    Both <input1> and <input2> files need to have headers. Stdin is not supported.

joinp options:
    --left                 Do a 'left outer' join. This returns all rows in
                           first CSV data set, including rows with no
                           corresponding row in the second data set. When no
                           corresponding row exists, it is padded out with
                           empty fields.
    --outer                Do a 'full outer' join. This returns all rows in
                           both data sets with matching records joined. If
                           there is no match, the missing side will be padded
                           out with empty fields. (This is the combination of
                           'outer left' and 'outer right'.)
    --cross                USE WITH CAUTION.
                           This returns the cartesian product of the CSV
                           data sets given. The number of rows return is
                           equal to N * M, where N and M correspond to the
                           number of rows in the given data sets, respectively.
    --semi                 This returns only the rows in the first CSV data set
                           that have a corresponding row in the second CSV data
                           set. The output is the same as the first data set.
    --anti                 This returns only the rows in the first CSV data set
                           that do not have a corresponding row in the second
                           CSV data set. The output is the same as the first dataset.
    --nulls                When set, joins will work on empty fields.
                           Otherwise, empty fields are completely ignored.

Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
    -d, --delimiter <arg>  The field delimiter for reading CSV data.
                           Must be a single character. (default: ,)
    -Q, --quiet            Do not print join rowcount to stderr.
    --no-memcheck          Do not check if there is enough memory to load the
                           entire CSVs into memory.
"#;

use std::{
    fs::File,
    io::{self, Write},
    path::{Path, PathBuf},
    str,
};

use polars::{io::prelude::*, prelude::*};
use serde::Deserialize;

use crate::{config::Delimiter, util, CliError, CliResult};

#[derive(Deserialize)]
struct Args {
    arg_columns1:     String,
    arg_input1:       String,
    arg_columns2:     String,
    arg_input2:       String,
    flag_left:        bool,
    flag_outer:       bool,
    flag_cross:       bool,
    flag_semi:        bool,
    flag_anti:        bool,
    flag_output:      Option<String>,
    flag_nulls:       bool,
    flag_delimiter:   Option<Delimiter>,
    flag_quiet:       bool,
    flag_no_memcheck: bool,
}

impl From<polars::error::PolarsError> for CliError {
    fn from(err: polars::error::PolarsError) -> CliError {
        CliError::Other(format!("Polars error: {err:?}"))
    }
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;
    let join = args.new_join()?;
    let join_rowcount = match (
        args.flag_left,
        args.flag_outer,
        args.flag_cross,
        args.flag_semi,
        args.flag_anti,
    ) {
        (false, false, false, false, false) => join.polars_join(JoinType::Inner),
        (true, false, false, false, false) => join.polars_join(JoinType::Left),
        (false, true, false, false, false) => join.polars_join(JoinType::Outer),
        (false, false, true, false, false) => join.polars_join(JoinType::Cross),
        (false, false, false, true, false) => join.polars_join(JoinType::Semi),
        (false, false, false, false, true) => join.polars_join(JoinType::Anti),
        _ => fail!("Please pick exactly one join operation."),
    }?;

    if !args.flag_quiet {
        eprintln!("{join_rowcount}");
    }

    Ok(())
}

struct JoinStruct {
    df1:    DataFrame,
    sel1:   String,
    df2:    DataFrame,
    sel2:   String,
    output: Option<String>,
    delim:  u8,
}

impl JoinStruct {
    fn polars_join(self, jointype: JoinType) -> CliResult<usize> {
        let selcols1: Vec<&str> = self.sel1.split(',').collect();
        let selcols2: Vec<&str> = self.sel2.split(',').collect();

        let selcols1_len = selcols1.len();
        let selcols2_len = selcols2.len();

        if selcols1_len != selcols2_len {
            return fail_clierror!(
                "Both columns1 ({selcols1:?}) and columns2 ({selcols2:?}) must specify the same \
                 number of columns ({selcols1_len } != {selcols2_len})."
            );
        }

        let mut join_results = self
            .df1
            .join(&self.df2, selcols1, selcols2, jointype, None)?;

        // no need to use buffered writer here, as CsvWriter already does that
        let mut out_writer = match self.output {
            Some(output_file) => {
                let path = Path::new(&output_file);
                Box::new(File::create(path).unwrap()) as Box<dyn Write>
            }
            None => Box::new(io::stdout()) as Box<dyn Write>,
        };

        // height of the dataframe is the number of rows
        let join_results_len = join_results.height();

        CsvWriter::new(&mut out_writer)
            .has_header(true)
            .with_delimiter(self.delim)
            .finish(&mut join_results)?;

        Ok(join_results_len)
    }
}

impl Args {
    fn new_join(&self) -> CliResult<JoinStruct> {
        // we're loading entire files into memory, we need to check avail mem
        // definitely can have a more sophisticated check, but this is better than getting an OOM
        let f1 = PathBuf::from(self.arg_input1.clone());
        if f1.exists() {
            util::mem_file_check(&f1, false, self.flag_no_memcheck)?;
        } else {
            return fail_clierror!("{f1:?} does not exist.");
        }
        let f2 = PathBuf::from(self.arg_input2.clone());
        if f1.exists() {
            util::mem_file_check(&f1, false, self.flag_no_memcheck)?;
        } else {
            return fail_clierror!("{f2:?} does not exist.");
        }

        let delim = if let Some(delimiter) = self.flag_delimiter {
            delimiter.as_byte()
        } else {
            b','
        };

        let df1 = CsvReader::from_path(&self.arg_input1)?
            .has_header(true)
            .with_missing_is_null(self.flag_nulls)
            .with_delimiter(delim)
            .finish()?;

        let df2 = CsvReader::from_path(&self.arg_input2)?
            .has_header(true)
            .with_missing_is_null(self.flag_nulls)
            .with_delimiter(delim)
            .finish()?;

        Ok(JoinStruct {
            df1,
            sel1: self.arg_columns1.clone(),
            df2,
            sel2: self.arg_columns2.clone(),
            output: self.flag_output.clone(),
            delim,
        })
    }
}
