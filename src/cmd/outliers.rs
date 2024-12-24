static USAGE: &str = r#"
Detect outliers in numeric columns using statistical methods.

Usage:
    qsv outliers [options] [<input>]
    qsv outliers --help

outliers options:
    -s, --select <arg>       Select specific columns to analyze for outliers
                            (comma separated). By default all numeric columns
                            are analyzed.
    -m, --method <method>    Method to use for outlier detection:
                              outer - Use outer fences (Q3 + 3.0×IQR) [default]
                              inner - Use inner fences (Q3 + 1.5×IQR)
                              both  - Show outliers using both fence types
    --force                 Force recomputing stats even if cache exists
    -q, --quiet            Don't show detailed outlier information, only summary

Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
    -n, --no-headers       When set, the first row will not be interpreted
                          as headers.
    -d, --delimiter <arg>  The field delimiter for reading CSV data.
                          Must be a single character. (default: ,)

Notes:
    - Uses the stats cache if available (see 'qsv stats --help')
    - For numeric columns: Values outside the IQR fences are considered outliers
    - For dates: Values are converted to days before outlier detection
    - Outputs both a summary count and detailed list of outliers per column
    - The --quiet flag suppresses detailed outlier listings

Examples:
    # Find outliers in all numeric columns using outer fences
    qsv outliers data.csv

    # Find outliers in specific columns using inner fences
    qsv outliers -s "temperature,pressure" -m inner data.csv

    # Show both inner and outer fence outliers with minimal output
    qsv outliers -m both -q data.csv
"#;

use polars::prelude::*;
use std::collections::HashMap;
use indicatif::{ProgressBar, ProgressStyle};

#[derive(Debug)]
struct OutlierResult {
    column: String,
    data_type: String,
    outlier_count: usize,
    outlier_details: Vec<OutlierDetail>,
}

#[derive(Debug)]
struct OutlierDetail {
    value: String,
    reason: String,
    fence_type: FenceType, // inner or outer
}

#[derive(Debug, PartialEq)]
enum FenceType {
    Inner,
    Outer,
    Both,
}

impl FenceType {
    fn from_str(s: &str) -> FenceType {
        match s.to_lowercase().as_str() {
            "inner" => FenceType::Inner,
            "outer" => FenceType::Outer,
            "both" => FenceType::Both,
            _ => FenceType::Outer, // default
        }
    }
}

// Helper function to determine if a value is an outlier based on fences
fn is_outlier(value: f64, lower_fence: f64, upper_fence: f64) -> bool {
    value < lower_fence || value > upper_fence
}

fn process_outliers(
    df: &DataFrame,
    stats: &[StatsData],
    method: FenceType,
    quiet: bool,
) -> CliResult<Vec<OutlierResult>> {
    let mut results = Vec::new();
    let pb = if !quiet {
        let pb = ProgressBar::new(stats.len() as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} columns")
                .unwrap(),
        );
        Some(pb)
    } else {
        None
    };

    for stat in stats {
        if let Some(pb) = &pb {
            pb.inc(1);
        }

        let mut outlier_details = Vec::new();
        
        match stat.r#type.as_str() {
            "Integer" | "Float" => {
                // Process numeric outliers using fences
                if let (Some(lower_inner), Some(upper_inner), Some(lower_outer), Some(upper_outer)) = (
                    stat.lower_inner_fence,
                    stat.upper_inner_fence,
                    stat.lower_outer_fence,
                    stat.upper_outer_fence,
                ) {
                    let col = df.column(&stat.field)?;
                    let values = col.f64()?;
                    
                    values.into_iter().flatten().enumerate().for_each(|(idx, val)| {
                        let (is_inner, is_outer) = (
                            is_outlier(val, lower_inner, upper_inner),
                            is_outlier(val, lower_outer, upper_outer),
                        );
                        
                        match (method, is_inner, is_outer) {
                            (FenceType::Inner, true, _) |
                            (FenceType::Outer, _, true) |
                            (FenceType::Both, true, _) => {
                                outlier_details.push(OutlierDetail {
                                    value: val.to_string(),
                                    reason: format!("Outside {} fences ({:.2}, {:.2})", 
                                        if is_outer { "outer" } else { "inner" },
                                        if is_outer { lower_outer } else { lower_inner },
                                        if is_outer { upper_outer } else { upper_inner }),
                                    fence_type: if is_outer { FenceType::Outer } else { FenceType::Inner },
                                });
                            },
                            _ => {},
                        }
                    });
                }
            },
            "Date" | "DateTime" => {
                // Process date outliers using fences (converted to days)
                if let (Some(lower_inner), Some(upper_inner), Some(lower_outer), Some(upper_outer)) = (
                    stat.lower_inner_fence,
                    stat.upper_inner_fence,
                    stat.lower_outer_fence,
                    stat.upper_outer_fence,
                ) {
                    let col = df.column(&stat.field)?;
                    if let Ok(dates) = col.datetime() {
                        dates.into_iter().flatten().enumerate().for_each(|(idx, val)| {
                            let days = val.timestamp_millis() as f64 / (24.0 * 60.0 * 60.0 * 1000.0);
                            let (is_inner, is_outer) = (
                                is_outlier(days, lower_inner, upper_inner),
                                is_outlier(days, lower_outer, upper_outer),
                            );
                            
                            match (method, is_inner, is_outer) {
                                (FenceType::Inner, true, _) |
                                (FenceType::Outer, _, true) |
                                (FenceType::Both, true, _) => {
                                    outlier_details.push(OutlierDetail {
                                        value: val.to_string(),
                                        reason: format!("Outside {} fences", 
                                            if is_outer { "outer" } else { "inner" }),
                                        fence_type: if is_outer { FenceType::Outer } else { FenceType::Inner },
                                    });
                                },
                                _ => {},
                            }
                        });
                    }
                }
            },
            "String" => {
                // Process string outliers using length statistics
                if let (Some(mean_len), Some(stddev)) = (stat.avg_length, stat.stddev) {
                    let col = df.column(&stat.field)?;
                    let strings = col.utf8()?;
                    
                    strings.into_iter().flatten().enumerate().for_each(|(idx, val)| {
                        let len = val.len() as f64;
                        let z_score = (len - mean_len) / stddev;
                        
                        if z_score.abs() > 3.0 {
                            outlier_details.push(OutlierDetail {
                                value: val.to_string(),
                                reason: format!("Unusual length: {} (z-score: {:.2})", len, z_score),
                                fence_type: FenceType::Both,
                            });
                        }
                    });
                }
                
                // Also check for rare categories using antimode information
                if let Some(ref antimode) = stat.antimode {
                    if !antimode.starts_with("*ALL") { // Skip if all values are unique
                        let antimodes: Vec<&str> = antimode.split(',').collect();
                        let col = df.column(&stat.field)?;
                        let strings = col.utf8()?;
                        
                        strings.into_iter().flatten().enumerate().for_each(|(idx, val)| {
                            if antimodes.contains(&val) {
                                outlier_details.push(OutlierDetail {
                                    value: val.to_string(),
                                    reason: "Rare category (antimode)".to_string(),
                                    fence_type: FenceType::Both,
                                });
                            }
                        });
                    }
                }
            },
            _ => {}, // Skip other types
        }

        if !outlier_details.is_empty() {
            results.push(OutlierResult {
                column: stat.field.clone(),
                data_type: stat.r#type.clone(),
                outlier_count: outlier_details.len(),
                outlier_details,
            });
        }
    }

    if let Some(pb) = &pb {
        pb.finish_with_message("Analysis complete");
    }

    Ok(results)
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;
    
    // Get stats records
    let schema_args = util::SchemaArgs {
        flag_enum_threshold: 0,
        flag_ignore_case: false,
        flag_strict_dates: false,
        flag_pattern_columns: crate::select::SelectColumns::parse("").unwrap(),
        flag_dates_whitelist: String::new(),
        flag_prefer_dmy: false,
        flag_force: args.flag_force,
        flag_stdout: false,
        flag_jobs: None,
        flag_no_headers: args.flag_no_headers,
        flag_delimiter: args.flag_delimiter.clone(),
        arg_input: Some(args.arg_input.clone()),
        flag_memcheck: false,
    };

    let (csv_fields, csv_stats) = get_stats_records(&schema_args, StatsMode::FrequencyForceStats)?;

    // Read the CSV file
    let mut csv_reader = LazyCsvReader::new(&args.arg_input)
        .with_has_header(!args.flag_no_headers)
        .with_delimiter(args.flag_delimiter.unwrap_or(Delimiter(b',')).0);

    if args.flag_infer_dates {
        csv_reader = csv_reader.with_try_parse_dates(true);
    }

    let df = csv_reader.finish()?.collect()?;

    // Process selected columns
    let selected_stats = if let Some(select) = args.flag_select {
        let selected: Vec<String> = select.split(',').map(String::from).collect();
        csv_stats
            .into_iter()
            .filter(|stat| selected.contains(&stat.field))
            .collect()
    } else {
        csv_stats
    };

    // Process outliers
    let method = FenceType::from_str(args.flag_method.as_deref().unwrap_or("outer"));
    let results = process_outliers(&df, &selected_stats, method, args.flag_quiet)?;

    // Write results
    let mut wtr: Box<dyn io::Write> = match args.flag_output {
        Some(ref output_path) => Box::new(File::create(Path::new(output_path))?),
        None => Box::new(io::stdout()),
    };

    // Write summary
    writeln!(wtr, "\nOutlier Analysis Summary:")?;
    writeln!(wtr, "=======================")?;
    
    for result in &results {
        writeln!(
            wtr,
            "\nColumn: {} ({})",
            result.column, result.data_type
        )?;
        writeln!(wtr, "Found {} outliers", result.outlier_count)?;
        
        if !args.flag_quiet {
            writeln!(wtr, "\nOutlier Details:")?;
            for detail in &result.outlier_details {
                writeln!(
                    wtr,
                    "  - Value: {:<20} | Reason: {}",
                    detail.value, detail.reason
                )?;
            }
        }
    }

    Ok(())
}
