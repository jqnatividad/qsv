static USAGE: &str = r#"
Joins two sets of CSV data on the specified columns using the Pola.rs engine.

The default join operation is an 'inner' join. This corresponds to the
intersection of rows on the keys specified.

Unlike the join command, joinp can process files larger than RAM, is multi-threaded,
supports asof joins & its output does not have duplicate columns.

However, joinp doesn't have an --ignore-case option & it doesn't support right outer joins.

The columns arguments specify the columns to join for each input. Columns are referenced
by name. Specify multiple columns by separating them with a comma.
Both columns1 and columns2 must specify exactly the same number of columns.

Returns the shape of the join result (number of rows, number of columns) to stderr.

For examples, see https://github.com/jqnatividad/qsv/blob/master/tests/test_joinp.rs.

Usage:
    qsv joinp [options] <columns1> <input1> <columns2> <input2>
    qsv joinp --cross <input1> <input2>
    qsv joinp --help

joinp arguments:
    Both <input1> and <input2> files need to have headers. Stdin is not supported.

joinp options:
    --left                 Do a 'left outer' join. This returns all rows in
                           first CSV data set, including rows with no
                           corresponding row in the second data set. When no
                           corresponding row exists, it is padded out with
                           empty fields.
    --left-anti            This returns only the rows in the first CSV data set
                           that do not have a corresponding row in the second
                           data set. The output schema is the same as the
                           first dataset.
    --left-semi            This returns only the rows in the first CSV data set
                           that have a corresponding row in the second data set.
                           The output schema is the same as the first data set.
    --full                 Do a 'full outer' join. This returns all rows in
                           both data sets with matching records joined. If
                           there is no match, the missing side will be padded
                           out with empty fields.
    --cross                USE WITH CAUTION.
                           This returns the cartesian product of the CSV
                           data sets given. The number of rows return is
                           equal to N * M, where N and M correspond to the
                           number of rows in the given data sets, respectively.
                           The columns1 and columns2 arguments are ignored.

    --asof                 Do an 'asof' join. This is similar to a left outer
                           join, except we match on nearest key rather than
                           equal keys. Note that both CSV data sets will be SORTED
                           AUTOMATICALLY on the join columns.
    --strategy <arg>       The strategy to use for the asof join:
                             backward - For each row in the first CSV data set,
                                        we find the last row in the second data set
                                        whose key is less than or equal to the key
                                        in the first data set.
                             forward -  For each row in the first CSV data set,
                                        we find the first row in the second data set
                                        whose key is greater than or equal to the key
                                        in the first data set.
                             nearest -  selects the last row in the second data set
                                        whose value is nearest to the value in the
                                        first data set.
                           [default: backward]
    --tolerance <arg>      The tolerance for the nearest asof join. This is only
                           used when the nearest strategy is used. The
                           tolerance is a positive integer that specifies
                           the maximum number of rows to search for a match.

                           If the join is done on a column of type Date, Time or
                           DateTime, then the tolerance is interpreted using
                           the following language:
                                1d - 1 day
                                1h - 1 hour
                                1m - 1 minute
                                1s - 1 second
                                1ms - 1 millisecond
                                1us - 1 microsecond
                                1ns - 1 nanosecond
                                1w - 1 week
                                1mo - 1 month
                                1q - 1 quarter
                                1y - 1 year
                                1i - 1 index count
                             Or combine them: “3d12h4m25s” # 3 days, 12 hours,
                             4 minutes, and 25 seconds
                             Suffix with “_saturating” to indicate that dates too
                             large for their month should saturate at the largest date
                             (e.g. 2022-02-29 -> 2022-02-28) instead of erroring.

    --nulls                When set, joins will work on empty fields.
                           Otherwise, empty fields are completely ignored.
    --try-parsedates       When set, the join will attempt to parse the columns
                           as dates. If the parse fails, columns remain as strings.
                           This is useful when the join columns are formatted as 
                           dates with differing date formats, as the date formats
                           will be normalized. Note that this will be automatically 
                           enabled when using asof joins.

Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
    -d, --delimiter <arg>  The field delimiter for reading CSV data.
                           Must be a single character. (default: ,)
    -Q, --quiet            Do not return join shape to stderr.
"#;

use std::{
    fs::File,
    io::{self, Write},
    path::Path,
    str,
};

use polars::{
    chunked_array::object::{AsOfOptions, AsofStrategy},
    datatypes::AnyValue,
    frame::hash_join::JoinType,
    prelude::{CsvWriter, LazyCsvReader, LazyFileListReader, LazyFrame, SerWriter, SortOptions},
};
use serde::Deserialize;

use crate::{config::Delimiter, util, CliError, CliResult};

#[derive(Deserialize)]
struct Args {
    arg_columns1:        String,
    arg_input1:          String,
    arg_columns2:        String,
    arg_input2:          String,
    flag_left:           bool,
    flag_left_anti:      bool,
    flag_left_semi:      bool,
    flag_full:           bool,
    flag_cross:          bool,
    flag_asof:           bool,
    flag_strategy:       Option<String>,
    flag_tolerance:      Option<String>,
    flag_nulls:          bool,
    flag_try_parsedates: bool,
    flag_output:         Option<String>,
    flag_delimiter:      Option<Delimiter>,
    flag_quiet:          bool,
}

impl From<polars::error::PolarsError> for CliError {
    fn from(err: polars::error::PolarsError) -> CliError {
        CliError::Other(format!("Polars error: {err:?}"))
    }
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let mut args: Args = util::get_args(USAGE, argv)?;

    // always try to parse dates when its an asof join
    // just in case the user doesn't specify it
    // and they're using date/time/datetime columns
    if args.flag_asof {
        args.flag_try_parsedates = true;
    }
    let join = args.new_join(args.flag_try_parsedates)?;

    let join_shape = match (
        args.flag_left,
        args.flag_left_anti,
        args.flag_left_semi,
        args.flag_full,
        args.flag_cross,
        args.flag_asof,
    ) {
        (false, false, false, false, false, false) => join.polars_join(JoinType::Inner, false),
        (true, false, false, false, false, false) => join.polars_join(JoinType::Left, false),
        (false, true, false, false, false, false) => join.polars_join(JoinType::Anti, false),
        (false, false, true, false, false, false) => join.polars_join(JoinType::Semi, false),
        (false, false, false, true, false, false) => join.polars_join(JoinType::Outer, false),
        (false, false, false, false, true, false) => join.polars_join(JoinType::Cross, false),
        (false, false, false, false, false, true) => {
            // safety: flag_strategy is always is_some() as it has a default value
            args.flag_strategy = Some(args.flag_strategy.unwrap().to_lowercase());
            let strategy = match args.flag_strategy.as_deref() {
                Some("backward") | None => AsofStrategy::Backward,
                Some("forward") => AsofStrategy::Forward,
                Some("nearest") => AsofStrategy::Nearest,
                Some(s) => return fail_clierror!("Invalid asof strategy: {}", s),
            };

            let mut asof_options = AsOfOptions {
                strategy,
                ..Default::default()
            };

            if strategy == AsofStrategy::Nearest {
                if let Some(ref tolerance) = args.flag_tolerance {
                    // set is_date_tolerance to true if the tolerance is set to a
                    // non-numerical value, indicating that it is a
                    // tolerance date language
                    let is_date_tolerance = if let Some(tolerance) = &args.flag_tolerance {
                        tolerance.parse::<i64>().is_err()
                    } else {
                        false
                    };

                    if is_date_tolerance {
                        asof_options.tolerance_str = Some(tolerance.into());
                    } else {
                        let numeric_tolerance = tolerance.parse::<i64>().unwrap();
                        asof_options.tolerance = Some(AnyValue::Int64(numeric_tolerance));
                    }
                }
            }
            join.polars_join(JoinType::AsOf(asof_options), true)
        }
        _ => fail!("Please pick exactly one join operation."),
    }?;

    if !args.flag_quiet {
        eprintln!("{join_shape:?}");
    }

    Ok(())
}

struct JoinStruct {
    lf1:    LazyFrame,
    sel1:   String,
    lf2:    LazyFrame,
    sel2:   String,
    output: Option<String>,
    delim:  u8,
}

impl JoinStruct {
    fn polars_join(mut self, jointype: JoinType, asof_join: bool) -> CliResult<(usize, usize)> {
        let selcols1: Vec<_> = self.sel1.split(',').map(polars::lazy::dsl::col).collect();
        let selcols2: Vec<_> = self.sel2.split(',').map(polars::lazy::dsl::col).collect();

        let selcols1_len = selcols1.len();
        let selcols2_len = selcols2.len();

        if selcols1_len != selcols2_len {
            return fail_clierror!(
                "Both columns1 ({selcols1:?}) and columns2 ({selcols2:?}) must specify the same \
                 number of columns ({selcols1_len } != {selcols2_len})."
            );
        }

        let optimize_all = polars::lazy::frame::OptState {
            projection_pushdown:        true,
            predicate_pushdown:         true,
            type_coercion:              true,
            simplify_expr:              true,
            file_caching:               true,
            slice_pushdown:             true,
            common_subplan_elimination: true,
            streaming:                  true,
        };

        let mut join_results = if jointype == JoinType::Cross {
            self.lf1
                .with_optimizations(optimize_all)
                .join_builder()
                .with(self.lf2.with_optimizations(optimize_all))
                .how(JoinType::Cross)
                .force_parallel(true)
                .finish()
                .collect()?
        } else {
            if asof_join {
                // sort by the asof columns, as asof joins require sorted join column data
                self.lf1 = self.lf1.sort(&self.sel1, SortOptions::default());
                self.lf2 = self.lf2.sort(&self.sel2, SortOptions::default());
            }

            self.lf1
                .with_optimizations(optimize_all)
                .join_builder()
                .with(self.lf2.with_optimizations(optimize_all))
                .left_on(selcols1)
                .right_on(selcols2)
                .how(jointype)
                .force_parallel(true)
                .finish()
                .collect()?
        };

        // no need to use buffered writer here, as CsvWriter already does that
        let mut out_writer = match self.output {
            Some(output_file) => {
                let path = Path::new(&output_file);
                Box::new(File::create(path).unwrap()) as Box<dyn Write>
            }
            None => Box::new(io::stdout()) as Box<dyn Write>,
        };

        // shape is the number of rows and columns
        let join_shape = join_results.shape();

        CsvWriter::new(&mut out_writer)
            .has_header(true)
            .with_delimiter(self.delim)
            .finish(&mut join_results)?;

        Ok(join_shape)
    }
}

impl Args {
    fn new_join(&self, try_parse_dates: bool) -> CliResult<JoinStruct> {
        let delim = if let Some(delimiter) = self.flag_delimiter {
            delimiter.as_byte()
        } else {
            b','
        };

        let lf1 = LazyCsvReader::new(&self.arg_input1)
            .has_header(true)
            .with_missing_is_null(self.flag_nulls)
            .with_delimiter(delim)
            .with_try_parse_dates(try_parse_dates)
            .finish()?;

        let lf2 = LazyCsvReader::new(&self.arg_input2)
            .has_header(true)
            .with_missing_is_null(self.flag_nulls)
            .with_delimiter(delim)
            .with_try_parse_dates(try_parse_dates)
            .finish()?;

        Ok(JoinStruct {
            lf1,
            sel1: self.arg_columns1.clone(),
            lf2,
            sel2: self.arg_columns2.clone(),
            output: self.flag_output.clone(),
            delim,
        })
    }
}
