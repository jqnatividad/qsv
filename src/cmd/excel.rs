use crate::config::Config;
use crate::util;
use crate::CliResult;
use calamine::{open_workbook_auto, DataType, Reader};
use log::debug;
use serde::Deserialize;
use std::cmp;
use std::path::PathBuf;
use thousands::Separable;

static USAGE: &str = "
Exports a specified Excel/ODS sheet to a CSV file.

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

Common options:
    -h, --help                 Display this message
    -o, --output <file>        Write output to <file> instead of stdout.
";

#[derive(Deserialize)]
struct Args {
    arg_input: String,
    flag_sheet: String,
    flag_flexible: bool,
    flag_trim: bool,
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
                        num_sheets.abs_diff(sheet_index.unsigned_abs().try_into().unwrap()),
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
    let range = workbook.worksheet_range(&sheet).unwrap().unwrap();

    let mut wtr = Config::new(&args.flag_output)
        .flexible(args.flag_flexible)
        .writer()?;
    let mut record = csv::StringRecord::new();
    let mut trimmed_record = csv::StringRecord::new();
    let mut count = 0_u32; // Excel can only hold 1m rows anyways, ODS - only 32k
    for row in range.rows() {
        record.clear();
        for cell in row {
            match *cell {
                DataType::Empty => record.push_field(""),
                DataType::String(ref s) => record.push_field(s),
                DataType::Float(ref f) | DataType::DateTime(ref f) => {
                    record.push_field(&f.to_string())
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
            wtr.write_record(&trimmed_record).unwrap();
        } else {
            wtr.write_record(&record).unwrap();
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
