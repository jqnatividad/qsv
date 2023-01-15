static USAGE: &str = r#"
Downloads a file, with special support for CSV/TSV files.
The first row of a sheet is assumed to be the header row.

pseudocode:
- check if input is valid, http https
- check if server supports range requests
- add option to check server status (range request support, etc.)
- add ability to change user agent
- allow resume
- slice option - get subset of csv (use slice semantics)
    - start
    - end
    - len
    - index (will look for idx file on server, to get position)
- sample option


Blah blah blah.

For examples, see https://github.com/jqnatividad/qsv/blob/master/tests/test_excel.rs.

Usage:
    qsv download [options] [<url>]
    qsv download --help

download options:
    -s, --start <arg>       The index of the record to slice from.
                            If negative, starts from the last record.
    -e, --end <arg>         The index of the record to slice to.
    -l, --len <arg>         The length of the slice (can be used instead
                            of --end).
    -i, --index <arg>       Slice a single record (shortcut for -s N -l 1).
    --timeout <seconds>     Timeout for each URL request.
                            [default: 15 ]
    --max-retries <count>   Maximum number of retries before an error is raised.
                            [default: 5]
    --user-agent <agent>    Specify a custom user agent. Try to follow the syntax here -
                            https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/User-Agent

                             

Common options:
    -h, --help                 Display this message
    -o, --output <file>        Write output to <file> instead of stdout.
"#;

use std::{cmp, path::PathBuf};

use calamine::{open_workbook_auto, DataType, Range, Reader};
use log::{debug, info};
use serde::{Deserialize, Serialize};
use thousands::Separable;

use crate::{config::Config, util, CliResult};

#[derive(Deserialize)]
struct Args {
    arg_input:            String,
    flag_sheet:           String,
    flag_metadata:        String,
    flag_flexible:        bool,
    flag_trim:            bool,
    flag_dates_whitelist: String,
    flag_output:          Option<String>,
}

#[derive(PartialEq)]
enum MetadataMode {
    Csv,
    Json,
    PrettyJSON,
    None,
}

#[derive(Serialize, Deserialize)]
struct SheetMetadata {
    index:                   usize,
    name:                    String,
    headers:                 Vec<String>,
    num_columns:             usize,
    num_rows:                usize,
    safe_headers:            Vec<String>,
    safe_headers_count:      usize,
    unsafe_headers:          Vec<String>,
    unsafe_headers_count:    usize,
    duplicate_headers_count: usize,
}

#[derive(Serialize, Deserialize)]
struct MetadataStruct {
    filename:   String,
    format:     String,
    num_sheets: usize,
    sheet:      Vec<SheetMetadata>,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;
    let path = &args.arg_input;

    let sce = PathBuf::from(path);
    let mut ods_flag = false;
    let filename = sce
        .file_name()
        .and_then(std::ffi::OsStr::to_str)
        .unwrap_or_default();
    let format = sce
        .extension()
        .and_then(std::ffi::OsStr::to_str)
        .unwrap_or_default();
    match format.to_ascii_lowercase().as_str() {
        "xls" | "xlsx" | "xlsm" | "xlsb" => (),
        "ods" => ods_flag = true,
        _ => {
            return fail!(
                "The excel command supports the following workbook formats - xls, xlsx, xlsm, \
                 xlsb and ods."
            );
        }
    };

    let mut workbook = match open_workbook_auto(path) {
        Ok(workbook) => workbook,
        Err(e) => {
            let es = e.to_string();
            // password protected errors come in different flavors for Excel
            if es.starts_with("Xls error: Cfb error")
                || es.starts_with("Xlsx error: Zip error: invalid Zip archive")
            {
                return fail_clierror!("{path} may be a password-protected workbook: {e}.");
            }
            return fail_clierror!("Cannot open workbook: {e}.");
        }
    };

    let sheet_names = workbook.sheet_names();
    if sheet_names.is_empty() {
        if ods_flag {
            return fail_clierror!("{path} may be password protected.");
        };
        return fail!("No sheets found.");
    }
    let num_sheets = sheet_names.len();
    let sheet_vec = sheet_names.to_owned();

    let mut wtr = Config::new(&args.flag_output)
        .flexible(args.flag_flexible)
        .writer()?;
    let mut record = csv::StringRecord::new();

    // set Metadata Mode
    let first_letter = args.flag_metadata.chars().next().unwrap_or_default();
    let metadata_mode = match first_letter {
        'c' | 'C' => MetadataMode::Csv,
        'j' => MetadataMode::Json,
        'J' => MetadataMode::PrettyJSON,
        'n' | 'N' => MetadataMode::None,
        _ => {
            return fail_clierror!("Invalid mode: {}", args.flag_metadata);
        }
    };

    if metadata_mode != MetadataMode::None {
        let mut excelmetadata_struct = MetadataStruct {
            filename: filename.to_string(),
            format: format.to_string(),
            num_sheets,
            sheet: vec![],
        };
        #[allow(clippy::needless_range_loop)]
        for i in 0..num_sheets {
            let sheet_name = sheet_vec[i].clone();

            let range = match workbook.worksheet_range_at(i) {
                Some(result) => {
                    if let Ok(result) = result {
                        result
                    } else {
                        return fail_clierror!("Cannot retrieve range from {}", sheet_name);
                    }
                }
                None => Range::empty(),
            };

            let (header_vec, num_columns, num_rows, safenames_vec, unsafeheaders_vec, dupe_count) =
                if range.is_empty() {
                    (vec![], 0_usize, 0_usize, vec![], vec![], 0_usize)
                } else {
                    let (num_rows, num_columns) = range.get_size();
                    let mut sheet_rows = range.rows();
                    let mut checkednames_vec: Vec<String> = Vec::with_capacity(num_columns);
                    let mut safenames_vec: Vec<String> = Vec::with_capacity(num_columns);
                    let mut unsafenames_vec: Vec<String> = Vec::new();
                    let mut dupe_count = 0_usize;
                    let mut header_vec: Vec<String> = Vec::with_capacity(num_columns);

                    if let Some(first_row) = sheet_rows.next() {
                        header_vec = first_row
                            .iter()
                            .map(|h| {
                                let header = h.to_string();

                                let safe_flag = util::is_safe_name(&header);
                                if safe_flag {
                                    if !safenames_vec.contains(&header) {
                                        safenames_vec.push(header.to_string());
                                    }
                                } else {
                                    unsafenames_vec.push(header.to_string());
                                };

                                // check for duplicate headers/columns
                                if checkednames_vec.contains(&header) {
                                    dupe_count += 1;
                                } else {
                                    checkednames_vec.push(header.to_string());
                                }

                                header
                            })
                            .collect();
                    }

                    (
                        header_vec,
                        num_columns,
                        num_rows,
                        safenames_vec,
                        unsafenames_vec,
                        dupe_count,
                    )
                };
            let sheetmetadata_struct = SheetMetadata {
                index: i,
                name: sheet_name,
                headers: header_vec,
                num_columns,
                num_rows,
                safe_headers_count: safenames_vec.len(),
                safe_headers: safenames_vec,
                unsafe_headers_count: unsafeheaders_vec.len(),
                unsafe_headers: unsafeheaders_vec,
                duplicate_headers_count: dupe_count,
            };

            excelmetadata_struct.sheet.push(sheetmetadata_struct);
        }
        match metadata_mode {
            MetadataMode::Csv => {
                record.push_field("index");
                record.push_field("sheet_name");
                record.push_field("headers");
                record.push_field("num_columns");
                record.push_field("num_rows");
                record.push_field("safe_headers");
                record.push_field("safe_headers_count");
                record.push_field("unsafe_headers");
                record.push_field("unsafe_headers_count");
                record.push_field("duplicate_headers_count");

                wtr.write_record(&record)?;

                for sheetmetadata in excelmetadata_struct.sheet {
                    record.clear();
                    record.push_field(&sheetmetadata.index.to_string());
                    record.push_field(&sheetmetadata.name);
                    record.push_field(&format!("{:?}", sheetmetadata.headers));
                    record.push_field(&sheetmetadata.num_columns.to_string());
                    record.push_field(&sheetmetadata.num_rows.to_string());
                    record.push_field(&format!("{:?}", sheetmetadata.safe_headers));
                    record.push_field(&sheetmetadata.safe_headers_count.to_string());
                    record.push_field(&format!("{:?}", sheetmetadata.unsafe_headers));
                    record.push_field(&sheetmetadata.unsafe_headers_count.to_string());
                    record.push_field(&sheetmetadata.duplicate_headers_count.to_string());

                    wtr.write_record(&record)?;
                }
                wtr.flush()?;
            }
            MetadataMode::Json => {
                let Ok(json_result) = serde_json::to_string(&excelmetadata_struct) else {
                    return fail!("Cannot create JSON");
                };
                println!("{json_result}");
            }
            MetadataMode::PrettyJSON => {
                let Ok(json_result) = serde_json::to_string_pretty(&excelmetadata_struct) else {
                    return fail!("Cannot create pretty JSON");
                };
                println!("{json_result}");
            }
            MetadataMode::None => {}
        }
        log::info!(r#"exported metadata for "{filename}" workbook sheets: {sheet_vec:?}"#);
        // after we export metadata, we're done.
        // we're not exporting the spreadsheet to CSV
        return Ok(());
    }

    // convert sheet_names to lowercase so we can do a case-insensitive compare
    let mut lower_sheet_names: Vec<String> = Vec::with_capacity(num_sheets);
    for s in sheet_names {
        lower_sheet_names.push(s.to_lowercase());
    }

    // if --sheet name was passed, see if its a valid sheet name.
    let mut sheet = if lower_sheet_names.contains(&args.flag_sheet.to_lowercase()) {
        args.flag_sheet
    } else {
        // otherwise, if --sheet is a number, its a zero-based index, fetch it
        if let Ok(sheet_index) = args.flag_sheet.parse::<i32>() {
            if sheet_index >= 0 {
                if sheet_index as usize <= sheet_names.len() {
                    sheet_names[sheet_index as usize].to_string()
                } else {
                    return fail_clierror!(
                        "sheet index {sheet_index} is greater than number of sheets {}",
                        sheet_names.len()
                    );
                }
            } else {
                // if its a negative number, start from the end
                // i.e -1 is the last sheet; -2 = 2nd to last sheet
                sheet_names[cmp::max(
                    0,
                    cmp::min(
                        num_sheets - 1,
                        num_sheets.abs_diff(sheet_index.unsigned_abs() as usize),
                    ),
                )]
                .to_string()
            }
        } else {
            // failing all else, get the first sheet
            let first_sheet = sheet_names[0].to_string();
            debug!(
                r#"Invalid sheet "{}". Using the first sheet "{}" instead."#,
                args.flag_sheet, first_sheet
            );
            first_sheet
        }
    };
    let lower_sheet = sheet.to_lowercase();
    let sheet_index = if let Some(idx) = lower_sheet_names.iter().position(|s| *s == lower_sheet) {
        // set to actual name of the sheet, not the one passed using the --sheet option,
        // as we process the option case insensitively
        sheet = sheet_names[idx].clone();
        idx
    } else {
        return fail_clierror!("Cannot get sheet index for {sheet}");
    };

    let range = match workbook.worksheet_range_at(sheet_index) {
        Some(result) => {
            if let Ok(result) = result {
                result
            } else {
                return fail_clierror!("Cannot retrieve range from {sheet}");
            }
        }
        None => Range::empty(),
    };

    let whitelist_lower = args.flag_dates_whitelist.to_lowercase();
    info!("using date-whitelist: {whitelist_lower}");

    // an all number whitelist means we're being given
    // the column indices of the date column names
    let mut all_numbers_whitelist = true;

    let mut dates_whitelist =
        itertools::Itertools::collect_vec(whitelist_lower.split(',').map(|s| {
            if all_numbers_whitelist && s.parse::<u16>().is_err() {
                all_numbers_whitelist = false;
                info!("NOT a column index dates whitelist");
            }
            s.trim().to_string()
        }));
    // we sort the whitelist, so we can do the faster binary_search() instead of contains()
    // with an all_numbers_whitelist
    if all_numbers_whitelist {
        dates_whitelist.sort_unstable();
    }

    let mut trimmed_record = csv::StringRecord::new();
    let mut date_flag: Vec<bool> = Vec::with_capacity(20); // to save allocs
    let mut cell_date_flag;
    let mut float_val = 0_f64;
    let mut float_flag;
    let mut row_count = 0_usize;

    debug!("exporting sheet ({sheet})...");
    for (row_idx, row) in range.rows().enumerate() {
        record.clear();
        for (col_idx, cell) in row.iter().enumerate() {
            if row_idx == 0 {
                // its the header row, check the dates whitelist
                debug!("processing first row...");
                let col_name = cell.get_string().unwrap_or_default();
                record.push_field(col_name);
                match whitelist_lower.as_str() {
                    // "all" - all numeric fields are to be treated as dates
                    "all" => date_flag.insert(col_idx, true),
                    // "none" - date processing will not be attempted
                    "none" => date_flag.insert(col_idx, false),
                    // check if the column name is in the dates_whitelist
                    _ => date_flag.insert(
                        col_idx,
                        if all_numbers_whitelist {
                            dates_whitelist.binary_search(&col_idx.to_string()).is_ok()
                        } else {
                            let mut date_found = false;
                            let col_name_lower = col_name.to_lowercase();
                            for whitelist_item in &dates_whitelist {
                                if col_name_lower.contains(whitelist_item) {
                                    date_found = true;
                                    log::info!("date-whitelisted: {col_name}");
                                    break;
                                }
                            }
                            date_found
                        },
                    ),
                }
                debug!("date_flag: {date_flag:?}");
                continue;
            }
            cell_date_flag = false;
            float_flag = false;
            match *cell {
                DataType::Empty => record.push_field(""),
                DataType::String(ref s) => record.push_field(s),
                DataType::Int(ref i) => {
                    let mut buffer = itoa::Buffer::new();
                    record.push_field(buffer.format(*i));
                }
                DataType::DateTime(ref f) => {
                    float_val = *f;
                    float_flag = true;
                    cell_date_flag = true;
                }
                DataType::Float(ref f) => {
                    float_val = *f;
                    float_flag = true;
                    cell_date_flag = date_flag[col_idx];
                }
                DataType::Error(ref e) => record.push_field(&format!("{e:?}")),
                DataType::Bool(ref b) => record.push_field(&b.to_string()),
            };
            // dates are stored as floats in Excel
            // that's why we need the --dates-whitelist, so we can convert the float to a date.
            // However, with the XLSX format, we can get a cell's format as an attribute. So we can
            // automatically process a cell as a date, even if its column is NOT in the whitelist
            if float_flag {
                if cell_date_flag {
                    if float_val.fract() > 0.0 {
                        record.push_field({
                            &cell.as_datetime().map_or_else(
                                || format!("ERROR: Cannot convert {float_val} to datetime"),
                                |dt| format!("{dt}"),
                            )
                        });
                    } else {
                        record.push_field({
                            &cell.as_date().map_or_else(
                                || format!("ERROR: Cannot convert {float_val} to date"),
                                |d| format!("{d}"),
                            )
                        });
                    };
                } else {
                    record.push_field(&float_val.to_string());
                }
            }
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
        row_count += 1;
    }
    wtr.flush()?;

    let end_msg = format!(
        "{} {}-column rows exported from \"{sheet}\"",
        // don't count the header in row count
        row_count.saturating_sub(1).separate_with_commas(),
        record.len().separate_with_commas(),
    );

    winfo!("{end_msg}");

    Ok(())
}
