static USAGE: &str = r#"
Exports a specified Excel/ODS sheet to a CSV file.
The first row of a sheet is assumed to be the header row.

Examples:

Export the first sheet of an Excel file to a CSV file:
    qsv excel input.xlsx > output.csv
    qsv excel input.xlsx --output output.csv

Export the first sheet of an ODS file to a CSV file:
    qsv excel input.ods > output.csv
    qsv excel input.ods -o output.csv

Export the first sheet of an Excel file to a CSV file with different delimiters:
    # semicolon
    qsv excel input.xlsx -d ";" > output.csv
    # tab
    qsv excel input.xlsx -d "\t" > output.tsv

Export a sheet by name (case-insensitive):
    qsv excel --sheet "Sheet 3" input.xlsx

Export a sheet by index:
    # this exports the 3nd sheet (0-based index)
    qsv excel -s 2 input.xlsx

Export the last sheet (negative index)):
    qsv excel -s -1 input.xlsx

Export the second to last sheet:
    qsv excel -s -2 input.xls

Export a range of cells in the first sheet:
    qsv excel --range C3:T25 input.xlsx

Export a range of cells in the second sheet:
    qsv excel --range C3:T25 -s 1 input.xlsx

Export metadata for all sheets in CSV format:
    qsv excel --metadata c input.xlsx

Export metadata for all sheets in JSON format:
    qsv excel --metadata j input.xlsx
    # pretty-printed JSON
    qsv excel --metadata J input.xlsx

For more examples, see https://github.com/jqnatividad/qsv/blob/master/tests/test_excel.rs.

Usage:
    qsv excel [options] [<input>]
    qsv excel --help

Excel options:
    -s, --sheet <name/index>   Name (case-insensitive) or zero-based index of sheet to export.
                               Negative indices start from the end (-1 = last sheet). 
                               If the sheet cannot be found, qsv will read the first sheet.
                               [default: 0]
    --metadata <c|s|j|J|S>     Outputs workbook metadata in CSV or JSON format:
                                 index, sheet_name, headers, type, visible, num_columns, num_rows,
                                 safe_headers, safe_headers_count, unsafe_headers, unsafe_headers_count
                                 and duplicate_headers_count.
                               headers is a list of the first row which is presumed to be the header row.
                               type is the sheet type (WorkSheet, DialogSheet, MacroSheet, ChartSheet, Vba).
                               visible is the sheet visibility (Visible, Hidden, VeryHidden).
                               num_rows includes all rows, including the first row.
                               safe_headers is a list of header with "safe"(database-ready) names.
                               unsafe_headers is a list of headers with "unsafe" names.
                               duplicate_headers_count is a count of duplicate header names.

                               In CSV(c) mode, the output is in CSV format.
                               In short(s) CSV mode, the output is in CSV format with only the
                               index, sheet_name, type and visible fields.
                               
                               In JSON(j) mode, the output is minified JSON.
                               In Pretty JSON(J) mode, the output is pretty-printed JSON.
                               In Short(S) JSON mode, the output is minified JSON with only the
                                 index, sheet_name, type and visible fields.
                               For all JSON modes, the filename, the full file path, the workbook format
                               and the number of sheets are also included.
                               
                               All other Excel options are ignored.
                               [default: none]
    --error-format <format>    The format to use when formatting error cells.
                               There are 3 formats:
                                 - "code": return the error code.
                                    (e.g. #DIV/0!)
                                 - "formula": return the formula.
                                    (e.g. #=A1/B1 where B1 is 0; #=100/0)
                                 - "both": return both error code and the formula.
                                    (e.g. #DIV/0!: =A1/B1)
                               For now, extracting the formula text only works reliably on macOS.
                               [default: code]
    --flexible                 Continue even if the number of columns is different from row to row.
    --trim                     Trim all fields so that leading & trailing whitespaces are removed.
                               Also removes embedded linebreaks.
    --date-format <format>     Optional date format to use when formatting dates.
                               See https://docs.rs/chrono/latest/chrono/format/strftime/index.html
                               for the full list of supported format specifiers.
                               Note that if a date format is invalid, qsv will fall back and
                               return the date as if no date-format was specified.
     --keep-zero-time          Keep the time part of a date-time field if it is 00:00:00.
                               By default, qsv will remove the time part if it is 00:00:00.
     --range <range>           An Excel format range, like C:T or C3:T25, to extract to the CSV.
     -j, --jobs <arg>          The number of jobs to run in parallel.
                               When not set, the number of jobs is set to the number of CPUs detected.

Common options:
    -h, --help                 Display this message
    -o, --output <file>        Write output to <file> instead of stdout.
    -d, --delimiter <arg>      The delimiter to use when writing CSV data.
                               Must be a single character. [default: ,]
    -Q, --quiet                Do not display export summary message.
"#;

use std::{cmp, fmt::Write, path::PathBuf};

use calamine::{open_workbook, Data, Error, Range, Reader, SheetType, Sheets};
use indicatif::HumanCount;
use log::info;
use rayon::prelude::{ParallelIterator, ParallelSlice};
use serde::{Deserialize, Serialize};

use crate::{
    config::{Config, Delimiter},
    util, CliError, CliResult,
};

#[derive(Deserialize)]
struct Args {
    arg_input:           String,
    flag_sheet:          String,
    flag_metadata:       String,
    flag_error_format:   String,
    flag_flexible:       bool,
    flag_trim:           bool,
    flag_output:         Option<String>,
    flag_delimiter:      Option<Delimiter>,
    flag_quiet:          bool,
    flag_date_format:    Option<String>,
    flag_keep_zero_time: bool,
    flag_range:          String,
    flag_jobs:           Option<usize>,
}

#[derive(PartialEq)]
enum MetadataMode {
    Csv,
    ShortCsv,
    Json,
    PrettyJSON,
    ShortJSON,
    None,
}

#[derive(PartialEq)]
enum ErrorFormat {
    Code,
    Formula,
    Both,
}

#[derive(Serialize, Deserialize)]
struct SheetMetadata {
    index:                   usize,
    name:                    String,
    typ:                     String,
    visible:                 String,
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
struct ShortSheetMetadata {
    index:   usize,
    name:    String,
    typ:     String,
    visible: String,
}

impl From<calamine::Error> for CliError {
    fn from(e: calamine::Error) -> Self {
        CliError::Other(format!("{e}"))
    }
}

#[derive(Serialize, Deserialize)]
struct MetadataStruct {
    filename:           String,
    canonical_filename: String,
    format:             String,
    num_sheets:         usize,
    sheet:              Vec<SheetMetadata>,
}

#[derive(Serialize, Deserialize)]
struct ShortMetadataStruct {
    filename:           String,
    canonical_filename: String,
    format:             String,
    num_sheets:         usize,
    sheet:              Vec<ShortSheetMetadata>,
}

struct RequestedRange {
    // matches args for https://docs.rs/calamine/latest/calamine/struct.Range.html#method.rows
    start: (u32, u32), // upper left, 0 based, row, column
    end:   (u32, u32), // lower right.
}

impl RequestedRange {
    fn parse_col(col: &str) -> Option<u32> {
        // takes a string like C3 and returns a 0 indexed column number, 2
        // returns 0 on missing.

        col.chars()
            .filter(|c| !c.is_ascii_digit())
            .map(|i| u32::from(i) - (u32::from('a') - 1))
            .reduce(|sum, i| 26 * sum + i)
            .map(|r| r - 1)
    }

    fn parse_row(row: &str) -> Option<u32> {
        // takes a string like R32 and returns 0 indexed row number, 31.
        // returns 0 on missing
        row.chars()
            .filter(char::is_ascii_digit)
            .collect::<String>()
            .parse::<u32>()
            .ok()
            .map(|r| r - 1)
    }

    pub fn from_string(range: &str, worksheet_size: (usize, usize)) -> CliResult<RequestedRange> {
        // worksheet_size is from range.getsize, height,width.  1 indexed.

        let Some((start, end)) = range.split_once(':') else {
            return fail_clierror!("Unable to parse range string");
        };

        let start_row = Self::parse_row(start);
        let end_row = Self::parse_row(end);
        let start_col = Self::parse_col(start);
        let end_col = Self::parse_col(end);

        Ok(RequestedRange {
            start: (start_row.unwrap_or(0), start_col.unwrap_or(0)),
            end:   (
                end_row.unwrap_or_else(|| (worksheet_size.0 as u32).saturating_sub(1)),
                end_col.unwrap_or_else(|| (worksheet_size.1 as u32).saturating_sub(1)),
            ),
        })
    }
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;
    let path = &args.arg_input;

    let sce = PathBuf::from(path);
    let filename = sce
        .file_name()
        .and_then(std::ffi::OsStr::to_str)
        .unwrap_or_default()
        .to_string();
    let canonical_filename = sce.canonicalize()?.display().to_string();
    let format = sce
        .extension()
        .and_then(std::ffi::OsStr::to_str)
        .unwrap_or_default()
        .to_ascii_lowercase();

    let requested_range = args.flag_range.to_lowercase();

    let mut ods_flag = false;
    let mut workbook = match (format).as_str() {
        "xls" | "xla" => Sheets::Xls(open_workbook(path).map_err(Error::Xls)?),
        "xlsx" | "xlsm" => Sheets::Xlsx(open_workbook(path).map_err(Error::Xlsx)?),
        "xlsb" => Sheets::Xlsb(open_workbook(path).map_err(Error::Xlsb)?),
        "ods" => {
            ods_flag = true;
            Sheets::Ods(open_workbook(path).map_err(Error::Ods)?)
        },
        _ => {
            return fail_incorrectusage_clierror!(
                "\"{format}\" not supported. The excel command only supports the following file \
                 formats - xls, xlsx, xlsm, xlsb and ods."
            );
        },
    };

    let sheet_names = workbook.sheet_names();
    if sheet_names.is_empty() {
        return fail!("No sheets found.");
    }
    let num_sheets = sheet_names.len();

    let mut wtr = Config::new(&args.flag_output)
        .flexible(args.flag_flexible)
        .delimiter(args.flag_delimiter)
        .writer()?;

    // set Metadata Mode
    let first_letter = args.flag_metadata.chars().next().unwrap_or_default();
    let metadata_mode = match first_letter {
        'n' | 'N' => MetadataMode::None,
        'c' | 'C' => MetadataMode::Csv,
        's' => MetadataMode::ShortCsv,
        'j' => MetadataMode::Json,
        'J' => MetadataMode::PrettyJSON,
        'S' => MetadataMode::ShortJSON,
        _ => {
            return fail_incorrectusage_clierror!("Invalid metadata mode: {}", args.flag_metadata);
        },
    };

    // check if we're exporting workbook metadata only
    if metadata_mode != MetadataMode::None {
        let mut excelmetadata_struct = MetadataStruct {
            filename: filename.clone(),
            canonical_filename: canonical_filename.clone(),
            format: if ods_flag {
                "ODS".to_string()
            } else {
                format!("Excel: {format}")
            },
            num_sheets,
            sheet: vec![],
        };
        let mut metadata_record;
        let sheet_vec = sheet_names;

        for (i, sheet_name) in sheet_vec.iter().enumerate() {
            let range = if metadata_mode == MetadataMode::ShortCsv
                || metadata_mode == MetadataMode::ShortJSON
            {
                Range::empty()
            } else if let Some(result) = workbook.worksheet_range_at(i) {
                match result {
                    Ok(result) => result,
                    Err(e) => {
                        if workbook.sheets_metadata()[i].typ == SheetType::ChartSheet {
                            // return an empty range for ChartSheet
                            Range::empty()
                        } else {
                            return fail_clierror!("Cannot retrieve range from {sheet_name}: {e}.");
                        }
                    },
                }
            } else {
                Range::empty()
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

                                if util::is_safe_name(&header) {
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
                name: sheet_name.to_string(),
                typ: format!("{:?}", workbook.sheets_metadata()[i].typ),
                visible: format!("{:?}", workbook.sheets_metadata()[i].visible),
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
                let mut metadata_fields = Vec::with_capacity(10);
                metadata_fields.extend_from_slice(&[
                    "index",
                    "sheet_name",
                    "type",
                    "visible",
                    "headers",
                    "num_columns",
                    "num_rows",
                    "safe_headers",
                    "safe_headers_count",
                    "unsafe_headers",
                    "unsafe_headers_count",
                    "duplicate_headers_count",
                ]);
                metadata_record = csv::StringRecord::from(metadata_fields);

                wtr.write_record(&metadata_record)?;

                for sheetmetadata in excelmetadata_struct.sheet {
                    let metadata_values = vec![
                        sheetmetadata.index.to_string(),
                        sheetmetadata.name,
                        format!("{:?}", sheetmetadata.headers),
                        sheetmetadata.typ,
                        sheetmetadata.visible,
                        sheetmetadata.num_columns.to_string(),
                        sheetmetadata.num_rows.to_string(),
                        format!("{:?}", sheetmetadata.safe_headers),
                        sheetmetadata.safe_headers_count.to_string(),
                        format!("{:?}", sheetmetadata.unsafe_headers),
                        sheetmetadata.unsafe_headers_count.to_string(),
                        sheetmetadata.duplicate_headers_count.to_string(),
                    ];
                    metadata_record = csv::StringRecord::from(metadata_values);

                    wtr.write_record(&metadata_record)?;
                }
                wtr.flush()?;
            },
            MetadataMode::ShortCsv => {
                let mut metadata_fields = Vec::with_capacity(4);
                metadata_fields.extend_from_slice(&["index", "sheet_name", "type", "visible"]);
                metadata_record = csv::StringRecord::from(metadata_fields);

                wtr.write_record(&metadata_record)?;

                for sheetmetadata in excelmetadata_struct.sheet {
                    let metadata_values = vec![
                        sheetmetadata.index.to_string(),
                        sheetmetadata.name,
                        sheetmetadata.typ,
                        sheetmetadata.visible,
                    ];
                    metadata_record = csv::StringRecord::from(metadata_values);

                    wtr.write_record(&metadata_record)?;
                }
                wtr.flush()?;
            },
            MetadataMode::Json => {
                let Ok(json_result) = serde_json::to_string(&excelmetadata_struct) else {
                    return fail!("Cannot create JSON");
                };
                println!("{json_result}");
            },
            MetadataMode::PrettyJSON => {
                let Ok(json_result) = serde_json::to_string_pretty(&excelmetadata_struct) else {
                    return fail!("Cannot create pretty JSON");
                };
                println!("{json_result}");
            },
            MetadataMode::ShortJSON => {
                let mut short_excelmetadata_struct = ShortMetadataStruct {
                    filename,
                    canonical_filename,
                    format,
                    num_sheets,
                    sheet: vec![],
                };
                for sheetmetadata in excelmetadata_struct.sheet {
                    let short_sheetmetadata = ShortSheetMetadata {
                        index:   sheetmetadata.index,
                        name:    sheetmetadata.name,
                        typ:     sheetmetadata.typ,
                        visible: sheetmetadata.visible,
                    };
                    short_excelmetadata_struct.sheet.push(short_sheetmetadata);
                }
                let Ok(json_result) = serde_json::to_string(&short_excelmetadata_struct) else {
                    return fail!("Cannot create short JSON");
                };
                println!("{json_result}");
            },
            MetadataMode::None => {},
        }
        info!(r#"exported metadata for "{path}" workbook sheets: {sheet_vec:?}"#);
        // after we export metadata, we're done.
        // we're not exporting the spreadsheet to CSV
        return Ok(());
    }

    // --------------------------------------------------------------------
    // we're not exporting metadata, we're exporting the spreadsheet to CSV

    // convert sheet_names to lowercase so we can do a case-insensitive compare
    let lower_sheet_names: Vec<String> = sheet_names.iter().map(|s| s.to_lowercase()).collect();

    // if --sheet name was passed, see if its a valid sheet name.
    let mut sheet = if lower_sheet_names.contains(&args.flag_sheet.to_lowercase()) {
        args.flag_sheet
    } else {
        // otherwise, if --sheet is a number, its a zero-based index, fetch it
        if let Ok(sheet_index) = atoi_simd::parse::<i32>(args.flag_sheet.as_bytes()) {
            if sheet_index >= 0 {
                if let Some(sheet_name) = sheet_names.get(sheet_index as usize) {
                    sheet_name.to_string()
                } else {
                    return fail_incorrectusage_clierror!(
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
            // safety: its safe to use index access here as sheet_names is guaranteed to have at
            // least one element as we check if its not empty in  the beginning
            let first_sheet = sheet_names[0].to_string();
            info!(
                r#"Invalid sheet "{}". Using the first sheet "{}" instead."#,
                args.flag_sheet, first_sheet
            );
            first_sheet
        }
    }
    .to_lowercase();

    let sheet_index = if let Some(idx) = lower_sheet_names.iter().position(|s| *s == sheet) {
        // set to actual name of the sheet, not the one passed using the --sheet option,
        // as we process the option case insensitively
        // safety: it's safe to use index access here because lower_sheet_names is a lowercase copy
        // of sheet_names
        sheet.clone_from(&sheet_names[idx]);
        idx
    } else {
        return fail_clierror!("Cannot get sheet index for {sheet}");
    };

    let sheet_type = workbook.sheets_metadata()[sheet_index].typ;
    if sheet_type != SheetType::WorkSheet {
        return fail_incorrectusage_clierror!(
            "Can only export Worksheets. {sheet} is a {sheet_type:?}."
        );
    }

    let mut range = if let Some(result) = workbook.worksheet_range_at(sheet_index) {
        result?
    } else {
        Range::empty()
    };

    if !requested_range.is_empty() {
        info!("using range: {requested_range}");
        let parsed_range = RequestedRange::from_string(&requested_range, range.get_size())?;
        info!(
            "Range start: {} {}",
            parsed_range.start.0, parsed_range.start.1
        );
        info!("Range end: {} {}", parsed_range.end.0, parsed_range.end.1);
        if parsed_range.start < range.start().unwrap_or((0, 0))
            || parsed_range.end > range.end().unwrap_or((0, 0))
        {
            return fail_clierror!("Cannot retrieve range {requested_range}: larger than sheet");
        }
        range = range.range(parsed_range.start, parsed_range.end);
    }

    let (row_count, col_count) = range.get_size();

    let error_format = match args.flag_error_format.to_lowercase().as_str() {
        "formula" => ErrorFormat::Formula,
        "both" => ErrorFormat::Both,
        _ => ErrorFormat::Code,
    };

    if row_count == 0 {
        if !requested_range.is_empty() {
            return fail_clierror!("\"{requested_range}\" range in sheet \"{sheet}\" is empty.");
        }
        return fail_clierror!("\"{sheet}\" sheet is empty.");
    }
    // there are rows to export
    let mut rows_iter = range.rows();

    let range_start = range.start().unwrap_or((0, 0));
    let sheet_formulas = workbook.worksheet_formula(&sheet)?;

    // amortize allocations
    let mut record = csv::StringRecord::with_capacity(500, col_count);
    let mut trimmed_record = csv::StringRecord::with_capacity(500, col_count);
    let mut col_name: String;

    // process the first row as the header row
    info!("exporting sheet ({sheet})... processing first row as header...");
    let first_row = match rows_iter.next() {
        Some(first_row) => first_row,
        None => &[Data::Empty],
    };
    for cell in first_row {
        col_name = match *cell {
            Data::String(ref s) => s.to_string(),
            Data::Empty => String::new(),
            Data::Error(ref _e) => String::new(),
            Data::Int(ref i) => i.to_string(),
            Data::Float(ref f) => f.to_string(),
            Data::DateTime(ref exceldatetime) => exceldatetime.to_string(),
            Data::Bool(ref b) => b.to_string(),
            Data::DateTimeIso(ref dt) => dt.to_string(),
            Data::DurationIso(ref d) => d.to_string(),
        };
        record.push_field(&col_name);
    }

    let trim = args.flag_trim;

    if trim {
        record.trim();
        record.iter().for_each(|field| {
            if field.contains('\n') {
                trimmed_record.push_field(&field.replace('\n', " "));
            } else {
                trimmed_record.push_field(field);
            }
        });
        record.clone_from(&trimmed_record);
    }
    info!("header: {record:?}");
    wtr.write_record(&record)?;

    let no_date_format: bool;
    let date_format = if let Some(df) = args.flag_date_format {
        no_date_format = false;
        df
    } else {
        no_date_format = true;
        String::new()
    };

    let mut rows: Vec<(u32, &[Data])> = Vec::with_capacity(row_count);

    // we add 1 as we already processed the header row
    let mut row_idx = range_start.0 + 1;
    // queue rest of the rows for processing as data rows
    for row in rows_iter {
        rows.push((row_idx, row));
        row_idx += 1;
    }

    // set RAYON_NUM_THREADS
    let ncpus = util::njobs(args.flag_jobs);

    // set chunk_size to number of rows per core/thread
    let chunk_size = row_count.div_ceil(ncpus);

    let keep_zero_time = args.flag_keep_zero_time;

    let processed_rows: Vec<Vec<csv::StringRecord>> = rows
        .par_chunks(chunk_size)
        .map(|chunk| {
            let mut record = csv::StringRecord::with_capacity(500, col_count);
            let mut trimmed_record = csv::StringRecord::with_capacity(500, col_count);
            let mut float_val;
            let mut work_date;
            let mut ryu_buffer = ryu::Buffer::new();
            let mut itoa_buffer = itoa::Buffer::new();
            let mut formatted_date = String::new();

            let mut processed_chunk: Vec<csv::StringRecord> = Vec::with_capacity(chunk_size);
            let mut col_idx = 0_u32;

            let formula_get_value_error = "cannot get formula".to_string();
            let mut cell_value: &String;

            for (row_idx, row) in chunk {
                for cell in *row {
                    match *cell {
                        Data::Empty => record.push_field(""),
                        Data::String(ref s) => record.push_field(s),
                        Data::Int(ref i) => record.push_field(itoa_buffer.format(*i)),
                        Data::Float(ref f) => {
                            float_val = *f;
                            // push the ryu-formatted float value if its
                            // not an integer or the candidate
                            // integer is too big or too small to be an i64
                            #[allow(clippy::cast_precision_loss)]
                            if float_val.fract().abs() > f64::EPSILON
                                || float_val > i64::MAX as f64
                                || float_val < i64::MIN as f64
                            {
                                record.push_field(ryu_buffer.format_finite(float_val));
                            } else {
                                // its an i64 integer. We can't use ryu to format it, because it
                                // will be formatted as a
                                // float (have a ".0"). So we use itoa.
                                record.push_field(itoa_buffer.format(float_val as i64));
                            }
                        },
                        Data::DateTime(ref edt) => {
                            if edt.is_datetime() {
                                if let Some(dt) = edt.as_datetime() {
                                    if no_date_format {
                                        // no date format specified, so we'll just use the
                                        // default format for the datetime
                                        work_date = dt.to_string();
                                    } else {
                                        // a date format was specified, so we'll use it
                                        formatted_date.clear();
                                        if write!(formatted_date, "{}", dt.format(&date_format))
                                            .is_ok()
                                        {
                                            // the format string was ok, so use to_string()
                                            // to actually apply the DelayedFormat
                                            work_date = formatted_date.to_string();
                                        } else {
                                            // if there was a format error, revert to the
                                            // default format
                                            work_date = dt.to_string();
                                        }
                                    }
                                    if !keep_zero_time && work_date.ends_with(" 00:00:00") {
                                        work_date.truncate(work_date.len() - 9);
                                    }
                                } else {
                                    // if the datetime is invalid, just return the datetime as a
                                    // string this should never
                                    // happen as we did a is_datetime check
                                    // before we got here. We're just doing it so that work_date
                                    // is initialized properly without wasting an allocation
                                    work_date = edt.to_string();
                                }
                            } else {
                                // its not a datetime, its a duration
                                // return the duration as a string in ISO 8601 format
                                // https://www.digi.com/resources/documentation/digidocs/90001488-13/reference/r_iso_8601_duration_format.htm
                                // safety: we know this is a duration coz we did a is_datetime check
                                // above & ExcelDataTime only has 2 variants, DateTime & Duration
                                work_date = edt.as_duration().unwrap().to_string();
                            };

                            record.push_field(&work_date);
                        },
                        Data::Bool(ref b) => {
                            record.push_field(if *b { "true" } else { "false" });
                        },
                        Data::DateTimeIso(ref dt) => record.push_field(dt),
                        Data::DurationIso(ref d) => record.push_field(d),
                        Data::Error(ref e) => {
                            match error_format {
                                ErrorFormat::Code => record.push_field(&format!("{e}")),
                                ErrorFormat::Formula | ErrorFormat::Both => {
                                    cell_value = sheet_formulas
                                        .get_value((*row_idx, col_idx))
                                        .unwrap_or(&formula_get_value_error);
                                    if error_format == ErrorFormat::Formula {
                                        record.push_field(&format!("#={cell_value}"));
                                    } else {
                                        record.push_field(&format!("{e}: ={cell_value}"));
                                    }
                                },
                            };
                        },
                    };
                    col_idx += 1;
                }

                if trim {
                    // record.trim() is faster than trimming each field piecemeal
                    record.trim();
                    record.iter().for_each(|field| {
                        if field.contains('\n') {
                            trimmed_record.push_field(&field.replace('\n', " "));
                        } else {
                            trimmed_record.push_field(field);
                        }
                    });
                    record.clone_from(&trimmed_record);
                    trimmed_record.clear();
                }

                // we use mem::take here to avoid a clone/allocation of the record
                // it also has the nice side-effect of clearing the record, so we don't
                // need to call clear() on it.
                processed_chunk.push(std::mem::take(&mut record));
            }
            processed_chunk
        })
        .collect();

    // rayon collect() guarantees original order,
    // so we can just write results for each chunk in order
    for processed_chunk in processed_rows {
        for processed_row in processed_chunk {
            wtr.write_record(&processed_row)?;
        }
    }

    wtr.flush()?;

    if !args.flag_quiet {
        winfo!(
            "{}",
            format!(
                "{} {}-column rows exported from \"{sheet}\" sheet",
                HumanCount(row_count.saturating_sub(1) as u64),
                HumanCount(col_count as u64),
            )
        );
    }

    Ok(())
}
