static USAGE: &str = r#"
Pivots a CSV using the Pola.rs engine.

Returns the shape of the pivot result (number of rows, number of columns) to stderr.

For examples, see https://github.com/jqnatividad/qsv/blob/master/tests/test_pivotp.rs.

Usage:
    qsv pivotp [options] <input>
    qsv pivotp --help

pivot arguments:
    <input>                The CSV file to pivot. Use '-' for standard input.
                           If the input is a snappy compressed file, it will
                           be decompressed automatically.

joinp options:
    --values <arg>         Column values to aggregate. Can be multiple columns
                           if the columns option contains multiple columns as well.
    --index <arg>          One or multiple keys to group by.
    --columns <arg>        Name of the column(s) whose values will be used as
                           the new column names.
    --aggregate-fn <arg>   The aggregate function to use. Can be one of:
                             first, sum, min, max, mean, median, last, count
                             none: no aggregation takes place. Will raise an error
                             if multiple values are found for a single index/column pair.
                           [default: count]
    --maintain-order       Sort the grouped keys so that the output order
                           is predictable/deterministic.
    --sort-columns <arg>   Sort the columns in the output frame. Can be one of:
                             none, ascending, descending
                           [default: none]
    --separator <arg>      The separator to use when joining the index and
                           column names. [default: _]

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
    arg_input:          String,
    flag_values:        Vec<String>,
    flag_index:         Vec<String>,
    flag_columns:       Vec<String>,
    flag_aggregate_fn:  String,
    flag_maintain_order: bool,
    flag_sort_columns:  String,
    flag_separator:     String,
    flag_output:         Option<String>,
    flag_delimiter:      Option<Delimiter>,
    flag_quiet:          bool,
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
        // TODO: add support for join_asof_by()
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
                    // If the tolerance is an integer, it is tolerance number of rows.
                    // Otherwise, it is a tolerance date language spec.
                    if let Ok(numeric_tolerance) = tolerance.parse::<i64>() {
                        asof_options.tolerance = Some(AnyValue::Int64(numeric_tolerance));
                    } else {
                        asof_options.tolerance_str = Some(tolerance.into());
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
    lf1:       LazyFrame,
    sel1:      String,
    lf2:       LazyFrame,
    sel2:      String,
    output:    Option<String>,
    delim:     u8,
    streaming: bool,
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
            streaming:                  self.streaming,
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
            streaming: self.flag_streaming,
        })
    }
}
