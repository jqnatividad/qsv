static USAGE: &str = r#"
Pivots CSV data using the Polars engine.

The pivot operation consists of:
- One or more index columns (these will be the new rows)
- A column that will be pivoted (this will create the new columns)
- A values column that will be aggregated
- An aggregation function to apply

Usage:
    qsv pivotp [options] <on-cols> <input>
    qsv pivotp --help

pivotp arguments:
    <on-cols>     The column(s) to pivot on (creates new columns).
    <input>       is the input CSV file. The file must have headers.
                  Stdin is not supported.


pivotp options:
    -i, --index <cols>      The column(s) to use as the index (row labels).
                            Specify multiple columns by separating them with a comma.
                            The output will have one row for each unique combination of the index’s values.
                            If None, all remaining columns not specified on --on and --values will be used.
                            At least one of --index and --values must be specified.
    -v, --values <cols>     The column(s) containing values to aggregate.
                            If an aggregation is specified, these are the values on which the aggregation
                            will be computed. If None, all remaining columns not specified on --on and --index
                            will be used. At least one of --index and --values must be specified.
    -a, --agg <func>        The aggregation function to use:
                              first - First value encountered
                              sum - Sum of values
                              min - Minimum value
                              max - Maximum value
                              mean - Average value
                              median - Median value
                              count - Count of values
                              last - Last value encountered
                            [default: count]
    --sort-columns          Sort the transposed columns by name. Default is by order of discovery.
    --col-separator <arg>   The separator in generated column names in case of multiple --values columns.
                            [default: _]
    --validate              Validate a pivot by checking the pivot column(s)' cardinality.
    --try-parsedates        When set, will attempt to parse columns as dates.
    --infer-len <arg>       Number of rows to scan when inferring schema.
                            Set to 0 to scan entire file. [default: 10000]
    --decimal-comma         Use comma as decimal separator.
    --ignore-errors         Skip rows that can't be parsed.

Common options:
    -h, --help              Display this message
    -o, --output <file>     Write output to <file> instead of stdout.
    -d, --delimiter <arg>   The field delimiter for reading/writing CSV data.
                            Must be a single character. (default: ,)
"#;

use std::{fs::File, io, io::Write, path::Path};

use indicatif::HumanCount;
use polars::prelude::*;
use polars_ops::pivot::{pivot_stable, PivotAgg};
use serde::Deserialize;

use crate::{
    config::Delimiter,
    util,
    util::{get_stats_records, StatsMode},
    CliResult,
};

#[derive(Deserialize)]
struct Args {
    arg_on_cols:         String,
    arg_input:           String,
    flag_index:          Option<String>,
    flag_values:         Option<String>,
    flag_agg:            Option<String>,
    flag_sort_columns:   bool,
    flag_col_separator:  String,
    flag_validate:       bool,
    flag_try_parsedates: bool,
    flag_infer_len:      usize,
    flag_decimal_comma:  bool,
    flag_ignore_errors:  bool,
    flag_output:         Option<String>,
    flag_delimiter:      Option<Delimiter>,
}

/// Structure to hold pivot operation metadata
struct PivotMetadata {
    estimated_columns:    u64,
    on_col_cardinalities: Vec<(String, u64)>,
}

/// Calculate pivot operation metadata using stats information
fn calculate_pivot_metadata(
    args: &Args,
    on_cols: &[String],
    value_cols: Option<&Vec<String>>,
) -> CliResult<Option<PivotMetadata>> {
    // Get stats records
    let schema_args = util::SchemaArgs {
        flag_enum_threshold:  0,
        flag_ignore_case:     false,
        flag_strict_dates:    false,
        flag_pattern_columns: crate::select::SelectColumns::parse("").unwrap(),
        flag_dates_whitelist: String::new(),
        flag_prefer_dmy:      false,
        flag_force:           false,
        flag_stdout:          false,
        flag_jobs:            None,
        flag_no_headers:      false,
        flag_delimiter:       args.flag_delimiter,
        arg_input:            Some(args.arg_input.clone()),
        flag_memcheck:        false,
    };

    let Ok((csv_fields, csv_stats)) =
        get_stats_records(&schema_args, StatsMode::FrequencyForceStats)
    else {
        return Ok(None);
    };

    if csv_stats.is_empty() {
        return Ok(None);
    }

    // Get cardinalities for pivot columns
    let mut on_col_cardinalities = Vec::with_capacity(on_cols.len());
    let mut total_new_columns: u64 = 1;

    for on_col in on_cols {
        if let Some(pos) = csv_fields
            .iter()
            .position(|f| std::str::from_utf8(f).unwrap_or("") == on_col)
        {
            let cardinality = csv_stats[pos].cardinality;
            total_new_columns = total_new_columns.saturating_mul(cardinality);
            on_col_cardinalities.push((on_col.clone(), cardinality));
        }
    }

    // Calculate total columns in result
    let value_cols_count = match value_cols {
        Some(cols) => cols.len() as u64,
        None => 1,
    };
    let total_columns = total_new_columns.saturating_mul(value_cols_count);

    Ok(Some(PivotMetadata {
        estimated_columns: total_columns,
        on_col_cardinalities,
    }))
}

/// Validate pivot operation using metadata
fn validate_pivot_operation(metadata: &PivotMetadata) -> CliResult<()> {
    const COLUMN_WARNING_THRESHOLD: u64 = 1000;

    // Print cardinality information
    eprintln!("Pivot column cardinalities:");
    for (col, card) in &metadata.on_col_cardinalities {
        eprintln!("  {col}: {}", HumanCount(*card));
    }

    // Warn about large number of columns
    if metadata.estimated_columns > COLUMN_WARNING_THRESHOLD {
        eprintln!(
            "Warning: Pivot will create {} columns. This might impact performance.",
            HumanCount(metadata.estimated_columns)
        );
    }

    // Error if operation would create an unreasonable number of columns
    if metadata.estimated_columns > 100_000 {
        return fail_clierror!(
            "Pivot would create too many columns ({}). Consider reducing the number of pivot \
             columns or using a different approach.",
            HumanCount(metadata.estimated_columns)
        );
    }

    Ok(())
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;

    // Parse on column(s)
    let on_cols: Vec<String> = args
        .arg_on_cols
        .as_str()
        .split(',')
        .map(std::string::ToString::to_string)
        .collect();

    // Parse index column(s)
    let index_cols = if let Some(ref flag_index) = args.flag_index {
        let idx_cols: Vec<String> = flag_index
            .as_str()
            .split(',')
            .map(std::string::ToString::to_string)
            .collect();
        Some(idx_cols)
    } else {
        None
    };

    // Parse values column(s)
    let value_cols = if let Some(ref flag_values) = args.flag_values {
        let val_cols: Vec<String> = flag_values
            .as_str()
            .split(',')
            .map(std::string::ToString::to_string)
            .collect();
        Some(val_cols)
    } else {
        None
    };

    if index_cols.is_none() && value_cols.is_none() {
        return fail_incorrectusage_clierror!(
            "Either --index <cols> or --values <cols> must be specified."
        );
    }

    // Get aggregation function
    let agg_fn = if let Some(ref agg) = args.flag_agg {
        Some(match agg.to_lowercase().as_str() {
            "first" => PivotAgg::First,
            "sum" => PivotAgg::Sum,
            "min" => PivotAgg::Min,
            "max" => PivotAgg::Max,
            "mean" => PivotAgg::Mean,
            "median" => PivotAgg::Median,
            "count" => PivotAgg::Count,
            "last" => PivotAgg::Last,
            _ => return fail_clierror!("Invalid pivot aggregation function: {agg}"),
        })
    } else {
        None
    };

    // Set delimiter if specified
    let delim = if let Some(delimiter) = args.flag_delimiter {
        delimiter.as_byte()
    } else {
        b','
    };

    // Create CSV reader config
    let csv_reader = LazyCsvReader::new(&args.arg_input)
        .with_has_header(true)
        .with_try_parse_dates(args.flag_try_parsedates)
        .with_decimal_comma(args.flag_decimal_comma)
        .with_separator(delim)
        .with_ignore_errors(args.flag_ignore_errors)
        .with_infer_schema_length(Some(args.flag_infer_len));

    // Read the CSV into a DataFrame
    let df = csv_reader.finish()?.collect()?;

    if args.flag_validate {
        // Validate the operation
        if let Some(metadata) = calculate_pivot_metadata(&args, &on_cols, value_cols.as_ref())? {
            validate_pivot_operation(&metadata)?;
        }
    }

    // Perform pivot operation
    let mut pivot_result = pivot_stable(
        &df,
        on_cols,
        index_cols,
        value_cols,
        args.flag_sort_columns,
        agg_fn,
        Some(&args.flag_col_separator),
    )?;

    // Write output
    let mut writer = match args.flag_output {
        Some(ref output_file) => {
            // no need to use buffered writer here, as CsvWriter already does that
            let path = Path::new(&output_file);
            Box::new(File::create(path).unwrap()) as Box<dyn Write>
        },
        None => Box::new(io::stdout()) as Box<dyn Write>,
    };
    CsvWriter::new(&mut writer)
        .include_header(true)
        .with_datetime_format(Some("%Y-%m-%d %H:%M:%S".to_string()))
        .with_separator(delim)
        .finish(&mut pivot_result)?;

    // Print shape to stderr
    eprintln!("{:?}", pivot_result.shape());

    Ok(())
}
