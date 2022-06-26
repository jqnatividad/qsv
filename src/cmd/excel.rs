use crate::config::Config;
use crate::util;
use crate::CliResult;
use calamine::{open_workbook_auto, DataType, Reader};
use log::debug;
use serde::Deserialize;
use std::cmp;
use std::path::PathBuf;
use thousands::Separable;

static USAGE: &str = r#"
Exports a specified Excel/ODS sheet to a CSV file.

NOTE: Excel stores dates as number of days since 1900.
https://support.microsoft.com/en-us/office/date-systems-in-excel-e7fe7167-48a9-4b96-bb53-5612a800b487

Because of this, this command uses a --dates-whitelist to determine if it
will attempt to transform a numeric value to an ISO 8601 date based on its name.
If the column name satisfies the whitelist and a row value for a candidate date column
is a float, it will infer a date for integer values and a datetime for float values.

We need a whitelist so we know to only do this date conversions for date fields and
not all columns with numeric values.

Usage:
    qsv excel [options] <input>

Excel options:
    -s, --sheet <name/index>   Name or zero-based index of sheet to export.
                               Negative indices start from the end (-1 = last sheet). 
                               If the sheet cannot be found, qsv will read the first sheet.
                               [default: 0]
    --flexible                 Continue even if the number of fields is different 
                               from the previous record.
    --trim                     Trim all fields of records so that leading and trailing
                               whitespaces (Unicode definition) are removed.
                               Also removes embedded linebreaks.
    --dates-whitelist <list>   The case-insensitive patterns to look for when 
                               shortlisting fields for date inferencing.
                               i.e. if the field's name has any of these patterns,
                               it is shortlisted for date inferencing.
                               Set to "all" to inspect ALL fields for date/datetime types.
                               Note that doing so will cause false positive data conversions
                               for numeric columns.
                               Otherwise, Excel date fields that do not satisfy the
                               whitelist will be returned as number of days since 1900.
                               [default: date,time,due,opened,closed]                               

Common options:
    -h, --help                 Display this message
    -o, --output <file>        Write output to <file> instead of stdout.
"#;

#[derive(Deserialize)]
struct Args {
    arg_input: String,
    flag_sheet: String,
    flag_flexible: bool,
    flag_trim: bool,
    flag_dates_whitelist: String,
    flag_output: Option<String>,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;
    let path = &args.arg_input;

    let sce = PathBuf::from(path.to_ascii_lowercase());
    match sce.extension().and_then(std::ffi::OsStr::to_str) {
        Some("xls" | "xlsx" | "xlsm" | "xlsb" | "ods") => (),
        _ => {
            return fail!("Expecting an Excel/ODS file.");
        }
    }

    let mut workbook = match open_workbook_auto(path) {
        Ok(workbook) => workbook,
        Err(e) => return fail!(format!("Cannot open workbook: {e}.")),
    };

    let sheet_names = workbook.sheet_names();
    let num_sheets = sheet_names.len();

    // if --sheet name was passed, see if its a valid sheet name.
    let sheet = if sheet_names.contains(&args.flag_sheet) {
        args.flag_sheet
    } else {
        // otherwise, if --sheet is a number, its a zero-based index, fetch it
        if let Ok(sheet_index) = args.flag_sheet.parse::<i32>() {
            if sheet_index >= 0 {
                sheet_names[sheet_index as usize].to_string()
            } else {
                // if its a negative number, start from the end
                // i.e -1 is the last sheet; -2 = 2nd to last sheet
                sheet_names[cmp::max(
                    0,
                    cmp::min(
                        num_sheets,
                        num_sheets.abs_diff(sheet_index.unsigned_abs() as usize),
                    ),
                )]
                .to_string()
            }
        } else {
            // failing all else, get the first sheet
            let first_sheet = sheet_names[0].to_string();
            debug!(
                "Invalid sheet \"{}\". Using the first sheet \"{}\" instead.",
                args.flag_sheet, first_sheet
            );
            first_sheet
        }
    };
    let range = if let Ok(range) = workbook.worksheet_range(&sheet).unwrap() {
        range
    } else {
        return fail!("Cannot get worksheet data from {sheet}");
    };

    let whitelist_lower = args.flag_dates_whitelist.to_lowercase();
    log::info!("inferring dates with date-whitelist: {whitelist_lower}");

    let dates_whitelist =
        itertools::Itertools::collect_vec(whitelist_lower.split(',').map(|s| s.trim().to_string()));

    let mut wtr = Config::new(&args.flag_output)
        .flexible(args.flag_flexible)
        .writer()?;
    let mut record = csv::StringRecord::new();
    let mut trimmed_record = csv::StringRecord::new();
    let mut date_flag: Vec<bool> = Vec::new();
    let mut count = 0_u32; // use u32 as Excel can only hold 1m rows anyways, ODS - only 32k
    for (row_idx, row) in range.rows().enumerate() {
        record.clear();
        for (col_idx, cell) in row.iter().enumerate() {
            if row_idx == 0 {
                // its the header row, capture the column names
                let column_name = cell.get_string().unwrap_or_default();
                record.push_field(column_name);
                if whitelist_lower == "all" {
                    date_flag.insert(col_idx, true);
                } else {
                    // see if the column name is in the dates_whitelist
                    let mut date_found = false;
                    for whitelist_item in &dates_whitelist {
                        if column_name.contains(whitelist_item) {
                            date_found = true;
                            log::info!("inferring dates for {column_name}");
                            break;
                        }
                    }
                    date_flag.insert(col_idx, date_found);
                }
                continue;
            }
            match *cell {
                DataType::Empty => record.push_field(""),
                DataType::String(ref s) => record.push_field(s),
                DataType::Float(ref f) | DataType::DateTime(ref f) => {
                    if date_flag[col_idx] {
                        if f.fract() > 0.0 {
                            record.push_field(&format!("{}", &cell.as_datetime().unwrap()));
                        } else {
                            record.push_field(&format!("{}", &cell.as_date().unwrap()));
                        }
                    } else {
                        record.push_field(&f.to_string());
                    }
                }
                DataType::Int(ref i) => record.push_field(&i.to_string()),
                DataType::Error(ref e) => record.push_field(&format!("{e:?}")),
                DataType::Bool(ref b) => record.push_field(&b.to_string()),
            };
        }
        if args.flag_trim {
            record.trim();
            trimmed_record.clear();
            record.iter().for_each(|field| {
                if field.contains('\n') {
                    trimmed_record.push_field(&field.to_string().replace('\n', " "));
                } else {
                    trimmed_record.push_field(field);
                }
            });
            wtr.write_record(&trimmed_record)?;
        } else {
            wtr.write_record(&record)?;
        }
        count += 1;
    }
    wtr.flush()?;

    eprintln!(
        "{} {}-column rows exported from \"{sheet}\"",
        // don't count the header in row count
        (count - 1).separate_with_commas(),
        record.len().separate_with_commas(),
    );

    Ok(())
}
