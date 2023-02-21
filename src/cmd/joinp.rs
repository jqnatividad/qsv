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
"#;

use std::{
    fs::File,
    io::{self, Write},
    path::Path,
    str,
};

use polars::{io::prelude::*, prelude::*};
use serde::Deserialize;

use crate::{config::Delimiter, util, CliResult};

#[derive(Deserialize)]
struct Args {
    arg_columns1:   String,
    arg_input1:     String,
    arg_columns2:   String,
    arg_input2:     String,
    flag_left:      bool,
    flag_outer:     bool,
    flag_cross:     bool,
    flag_semi:      bool,
    flag_anti:      bool,
    flag_output:    Option<String>,
    flag_nulls:     bool,
    flag_delimiter: Option<Delimiter>,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;
    let state = args.new_join()?;
    match (
        args.flag_left,
        args.flag_outer,
        args.flag_cross,
        args.flag_semi,
        args.flag_anti,
    ) {
        (false, false, false, false, false) => state.polars_join(JoinType::Inner),
        (true, false, false, false, false) => state.polars_join(JoinType::Left),
        (false, true, false, false, false) => state.polars_join(JoinType::Outer),
        (false, false, true, false, false) => state.polars_join(JoinType::Cross),
        (false, false, false, true, false) => state.polars_join(JoinType::Semi),
        (false, false, false, false, true) => state.polars_join(JoinType::Anti),
        _ => fail!("Please pick exactly one join operation."),
    }
}

struct JoinStruct {
    df1:    DataFrame,
    sel1:   String,
    df2:    DataFrame,
    sel2:   String,
    output: Option<String>,
}

impl JoinStruct {
    fn polars_join(self, jointype: JoinType) -> CliResult<()> {
        let selcol1: Vec<&str> = self.sel1.split(',').collect();
        let selcol2: Vec<&str> = self.sel2.split(',').collect();

        let selcol1_len = selcol1.len();
        let selcol2_len = selcol2.len();

        if selcol1_len != selcol2_len {
            return fail_clierror!(
                "Both columns1 ({selcol1:?}) and columns2 ({selcol2:?}) must specify the same \
                 number of columns ({selcol1_len } != {selcol2_len})."
            );
        }

        let mut join_results = self.df1.join(&self.df2, selcol1, selcol2, jointype, None)?;

        // no need to use buffered writer here, as CsvWriter already does that
        let mut out_writer = match self.output {
            Some(x) => {
                let path = Path::new(&x);
                Box::new(File::create(path).unwrap()) as Box<dyn Write>
            }
            None => Box::new(io::stdout()) as Box<dyn Write>,
        };

        CsvWriter::new(&mut out_writer)
            .has_header(true)
            .with_delimiter(b',')
            .finish(&mut join_results)?;

        Ok(())
    }
}

impl Args {
    fn new_join(&self) -> CliResult<JoinStruct> {
        let df1 = CsvReader::from_path(&self.arg_input1)?
            .has_header(true)
            .with_missing_is_null(self.flag_nulls)
            .with_delimiter(if let Some(delimiter) = self.flag_delimiter {
                delimiter.as_byte()
            } else {
                b','
            })
            .finish()?;

        let df2 = CsvReader::from_path(&self.arg_input2)?
            .has_header(true)
            .with_missing_is_null(self.flag_nulls)
            .with_delimiter(if let Some(delimiter) = self.flag_delimiter {
                delimiter.as_byte()
            } else {
                b','
            })
            .finish()?;

        Ok(JoinStruct {
            df1,
            sel1: self.arg_columns1.clone(),
            df2,
            sel2: self.arg_columns2.clone(),
            output: self.flag_output.clone(),
        })
    }
}
