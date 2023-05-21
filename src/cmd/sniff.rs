static USAGE: &str = r#"
Quickly sniff and infer CSV metadata (delimiter, header row, number of preamble rows,
quote character, flexible, is_utf8, average record length, number of records, content
length and estimated number of records if sniffing a URL, file size, number of fields,
field names & data types) using a Viterbi algorithm.
(https://en.wikipedia.org/wiki/Viterbi_algorithm)

On Linux, `sniff` also acts as a general file type detector using the libmagic library
and returns the detected mime type, file size and last modified date if the file is not
a CSV. If --no-infer is enabled, it doesn't even bother to infer the CSV's schema.

On macOS and Windows however, `sniff` is only a CSV dialect detector and does not
detect other file types. It can only sniff files with the "csv", "tsv", "tab" and
"txt" extensions, and snappy compressed variants of these formats (e.g. "csv.sz",
"tsv.sz", etc.) extensions.

NOTE: This command "sniffs" a CSV's schema by sampling the first n rows (default: 1000)
of a file. Its inferences are sometimes wrong if the sample is not large enough 
(use --sample to adjust) or the file is too small to infer a pattern. 

If you want more robust, guaranteed schemata, use the "schema" or "stats" commands
instead as they scan the entire file.

For examples, see https://github.com/jqnatividad/qsv/blob/master/tests/test_sniff.rs.

Usage:
    qsv sniff [options] [<input>]
    qsv sniff --help

sniff arguments:
    <input>                  The file to sniff. This can be a local file, stdin 
                             or a URL (http and https schemes supported).

                             Note that when input is a URL, sniff will automatically
                             download the sample to a temporary file and sniff it. It
                             will create the file using csv::QuoteStyle::NonNumeric
                             so the sniffed schema may not be the same as the original.
                             This is done to increase the chances of sniffing the
                             correct schema.

sniff options:
    --sample <size>          First n rows to sample to sniff out the metadata.
                             When sample size is between 0 and 1 exclusive, 
                             it is treated as a percentage of the CSV to sample
                             (e.g. 0.20 is 20 percent).
                             When it is zero, the entire file will be sampled.
                             When the input is a URL, the sample size dictates
                             how many lines to sample without having to
                             download the entire file.
                             [default: 1000]
    --prefer-dmy             Prefer to parse dates in dmy format.
                             Otherwise, use mdy format.
    --json                   Return results in JSON format.
    --pretty-json            Return results in pretty JSON format.
    --save-urlsample <file>  Save the URL sample to a file.
                             Valid only when input is a URL.
    --timeout <secs>         Timeout for URL requests in seconds.
                             [default: 30]
    --user-agent <agent>     Specify a custom user agent to use when sniffing a CSV on a URL.
                             Try to follow the syntax here -
                             https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/User-Agent
    --stats-types            Use the same data type names as `stats`.
                             (Unsigned, Signed => Integer, Text => String, everything else the same)
    --no-infer               Do not infer the schema. Only return the file's mime type, size and
                             last modified date. Valid only on Linux.

Common options:
    -h, --help               Display this message
    -d, --delimiter <arg>    The field delimiter for reading CSV data.
                             Specify this when the delimiter is known beforehand,
                             as the delimiter guessing algorithm can sometimes be
                             wrong if not enough delimiters are present in the sample.
                             Must be a single ascii character.
    -p, --progressbar        Show progress bars. Only valid for URL input.
"#;

use std::{
    cmp::min,
    fmt, fs,
    io::Write,
    path::PathBuf,
    time::{Duration, SystemTime},
};

use bytes::Bytes;
use futures::executor::block_on;
use futures_util::StreamExt;
use indicatif::{HumanBytes, HumanCount, ProgressBar, ProgressDrawTarget, ProgressStyle};
use qsv_sniffer::{DatePreference, SampleSize, Sniffer};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tabwriter::TabWriter;
use tempfile::NamedTempFile;
use thousands::Separable;
use url::Url;

use crate::{
    config::{Config, Delimiter},
    util, CliResult,
};

#[derive(Deserialize)]
struct Args {
    arg_input:           Option<String>,
    flag_sample:         f64,
    flag_prefer_dmy:     bool,
    flag_json:           bool,
    flag_save_urlsample: Option<String>,
    flag_pretty_json:    bool,
    flag_delimiter:      Option<Delimiter>,
    flag_progressbar:    bool,
    flag_timeout:        u16,
    flag_user_agent:     Option<String>,
    flag_stats_types:    bool,
    flag_no_infer:       bool,
}

#[derive(Serialize, Deserialize, Default, Debug)]
struct SniffStruct {
    path:            String,
    sniff_timestamp: String,
    last_modified:   String,
    delimiter_char:  char,
    header_row:      bool,
    preamble_rows:   usize,
    quote_char:      String,
    flexible:        bool,
    is_utf8:         bool,
    #[cfg(all(target_os = "linux", feature = "magic"))]
    detected_mime:   String,
    retrieved_size:  usize,
    file_size:       usize,
    sampled_records: usize,
    estimated:       bool,
    num_records:     usize,
    avg_record_len:  usize,
    num_fields:      usize,
    stats_types:     bool,
    fields:          Vec<String>,
    types:           Vec<String>,
}
impl fmt::Display for SniffStruct {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            f,
            "Path: {}",
            // when sniffing a snappy compressed file, it is first decompressed
            // to a temporary file. The original file name is stored in the
            // temporary file name, so we extract the original file name
            if self.path.ends_with("__qsv_temp_decompressed") {
                // use a regular expression to extract the original file name
                // the original file name is between "qsv__" and "__qsv_temp_decompressed"
                let re =
                    regex::Regex::new(r"qsv__(?P<filename>.*)__qsv_temp_decompressed").unwrap();
                let caps = re.captures(&self.path).unwrap();
                let filename = caps.name("filename").unwrap().as_str();
                filename.to_string()
            } else {
                self.path.clone()
            }
        )?;
        writeln!(f, "Sniff Timestamp: {}", self.sniff_timestamp)?;
        writeln!(f, "Last Modified: {}", self.last_modified)?;
        writeln!(
            f,
            "Delimiter: {}",
            if self.delimiter_char == '\t' {
                "tab".to_string()
            } else {
                self.delimiter_char.to_string()
            }
        )?;
        writeln!(f, "Header Row: {}", self.header_row)?;
        writeln!(
            f,
            "Preamble Rows: {}",
            self.preamble_rows.separate_with_commas()
        )?;
        writeln!(f, "Quote Char: {}", self.quote_char)?;
        writeln!(f, "Flexible: {}", self.flexible)?;
        writeln!(f, "Is UTF8: {}", self.is_utf8)?;
        #[cfg(all(target_os = "linux", feature = "magic"))]
        writeln!(f, "Detected Mime Type: {}", self.detected_mime)?;
        writeln!(
            f,
            "Retrieved Size (bytes): {}",
            self.retrieved_size.separate_with_commas()
        )?;
        writeln!(
            f,
            "File Size (bytes): {}",
            self.file_size.separate_with_commas()
        )?;
        writeln!(
            f,
            "Sampled Records: {}",
            self.sampled_records.separate_with_commas()
        )?;
        writeln!(f, "Estimated: {}", self.estimated)?;
        writeln!(
            f,
            "Num Records: {}",
            self.num_records.separate_with_commas()
        )?;
        writeln!(
            f,
            "Avg Record Len (bytes): {}",
            self.avg_record_len.separate_with_commas()
        )?;
        writeln!(f, "Num Fields: {}", self.num_fields.separate_with_commas())?;
        writeln!(f, "Stats Types: {}", self.stats_types)?;
        writeln!(f, "Fields:")?;

        let mut tabwtr = TabWriter::new(vec![]);

        for (i, ty) in self.types.iter().enumerate() {
            let data_type = if self.stats_types {
                match ty.as_str() {
                    "Unsigned" | "Signed" => "Integer",
                    "Text" => "String",
                    _ => ty,
                }
            } else {
                ty
            };

            writeln!(
                &mut tabwtr,
                "\t{i}:\t{data_type}\t{}",
                self.fields.get(i).unwrap_or(&String::new())
            )
            .unwrap_or_default();
        }
        tabwtr.flush().unwrap();

        let tabbed_field_list = String::from_utf8(tabwtr.into_inner().unwrap()).unwrap();
        writeln!(f, "{tabbed_field_list}")?;

        Ok(())
    }
}

#[derive(Debug, Clone)]
struct SniffFileStruct {
    display_path:       String,
    file_to_sniff:      String,
    tempfile_flag:      bool,
    retrieved_size:     usize,
    file_size:          usize,
    last_modified:      String,
    downloaded_records: usize,
}

const fn rowcount(
    metadata: &qsv_sniffer::metadata::Metadata,
    sniff_file_info: &SniffFileStruct,
    count: usize,
) -> (usize, bool) {
    let mut estimated = false;
    let rowcount = if count == usize::MAX {
        // if the file is usize::MAX, it's a sentinel value for "Unknown" as the server
        // didn't provide a Content-Length header, so we estimate the rowcount by
        // dividing the file_size by avg_rec_len
        estimated = true;
        sniff_file_info.file_size / metadata.avg_record_len
    } else {
        count
    };

    let has_header_row = metadata.dialect.header.has_header_row;
    let num_preamble_rows = metadata.dialect.header.num_preamble_rows;
    let mut final_rowcount = rowcount;

    if !has_header_row {
        final_rowcount += 1;
    }

    final_rowcount -= num_preamble_rows;
    (final_rowcount, estimated)
}

async fn get_file_to_sniff(args: &Args, tmpdir: &tempfile::TempDir) -> CliResult<SniffFileStruct> {
    if let Some(uri) = args.arg_input.clone() {
        match uri {
            // its a URL, download sample to temp file
            url if Url::parse(&url).is_ok() && url.starts_with("http") => {
                let snappy_flag = url.to_lowercase().ends_with(".sz");

                // setup the reqwest client
                let client = match Client::builder()
                    .user_agent(util::set_user_agent(args.flag_user_agent.clone())?)
                    .brotli(true)
                    .gzip(true)
                    .deflate(true)
                    .use_rustls_tls()
                    .http2_adaptive_window(true)
                    .build()
                {
                    Ok(c) => c,
                    Err(e) => {
                        return fail_clierror!("Cannot build reqwest client: {e}.");
                    }
                };

                let res = client
                    .get(url.clone())
                    .timeout(Duration::from_secs(
                        util::timeout_secs(args.flag_timeout).unwrap_or(30),
                    ))
                    .send()
                    .await
                    .map_err(|e| format!("Download failed: {e}"))?;

                let last_modified = match res.headers().get("Last-Modified") {
                    Some(lm) => match lm.to_str() {
                        Ok(s) => {
                            // convert Last-Modified RFC2822 to RFC3339 format
                            let dt = chrono::DateTime::parse_from_rfc2822(s).unwrap();
                            dt.to_rfc3339_opts(chrono::SecondsFormat::Secs, false)
                        }
                        // server did not return Last-Modified header
                        Err(_) => String::from("Unknown"),
                    },
                    None => String::new(),
                };

                let total_size = match res.content_length() {
                    Some(l) => l as usize,
                    None => {
                        // if we can't get the content length, just set it to a large value
                        // so we just end up downloading the entire file
                        usize::MAX
                    }
                };

                #[allow(clippy::cast_precision_loss)]
                let lines_sample_size = if snappy_flag {
                    // if it's a snappy compressed file, we need to download the entire file
                    // to sniff it
                    usize::MAX
                } else if args.flag_sample > 1.0 {
                    args.flag_sample.round() as usize
                } else if args.flag_sample.abs() < f64::EPSILON {
                    // sample size is zero, so we want to download the entire file
                    usize::MAX
                } else {
                    // sample size is a percentage, download percentage number of lines
                    // from the file. Since we don't know how wide the lines are, we
                    // just download a percentage of the bytes, assuming the lines are
                    // 100 characters wide as a rough estimate.
                    ((total_size / 100_usize) as f64 * args.flag_sample) as usize
                };

                // prep progress bar
                let show_progress =
                    args.flag_progressbar || util::get_envvar_flag("QSV_PROGRESSBAR");

                let progress = ProgressBar::with_draw_target(
                    Some(total_size.try_into().unwrap_or(u64::MAX)),
                    ProgressDrawTarget::stderr_with_hz(5),
                );
                if show_progress {
                    progress.set_style(
                        ProgressStyle::default_bar()
                            .template(
                                "{msg}\n{spinner:.green} [{elapsed_precise}] \
                                 [{wide_bar:.white/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, \
                                 {eta})",
                            )
                            .unwrap(),
                    );
                    if lines_sample_size == usize::MAX {
                        progress.set_message(format!(
                            "Downloading {}...",
                            HumanBytes(total_size as u64)
                        ));
                    } else {
                        progress.set_message(format!(
                            "Downloading {} samples...",
                            HumanCount(lines_sample_size as u64)
                        ));
                    }
                } else {
                    progress.set_draw_target(ProgressDrawTarget::hidden());
                }

                let mut file = NamedTempFile::new()?;
                let mut downloaded = 0_usize;
                let mut stream = res.bytes_stream();
                let mut downloaded_lines = 0_usize;
                #[allow(unused_assignments)]
                let mut chunk = Bytes::new(); // amortize the allocation

                // on linux, we can short-circuit downloading by
                // checking the file type from the first chunk
                // the unused_muts are here to suppress warnings
                // when the magic feature is not enabled
                #[allow(unused_mut)]
                let mut shortcircuit_flag = false;
                #[allow(unused_mut)]
                let mut firstchunk = Bytes::new();
                #[allow(unused_mut)]
                let mut magic_flag = false;

                // download chunks until we have the desired sample size
                while let Some(item) = stream.next().await {
                    chunk = item.map_err(|_| "Error while downloading file")?;

                    file.write_all(&chunk)
                        .map_err(|_| "Error while writing to file")?;
                    let chunk_len = chunk.len();

                    #[cfg(all(target_os = "linux", feature = "magic"))]
                    {
                        // we're using magic!
                        magic_flag = true;
                    }

                    // on linux, we can short-circuit downloading
                    // by checking the file type from the first chunk
                    #[cfg(all(target_os = "linux", feature = "magic"))]
                    if downloaded == 0 && !snappy_flag && chunk_len >= util::FIRST_CHUNK_BUFFER_SIZE
                    {
                        let mime = util::sniff_filetype_from_buffer(&chunk)?;
                        if !mime.starts_with("text/") && mime != "application/csv" {
                            shortcircuit_flag = true;
                            downloaded = chunk_len;
                            firstchunk = chunk.clone();
                            break;
                        }
                    }

                    downloaded = min(downloaded + chunk_len, total_size);
                    if show_progress {
                        progress.inc(chunk_len as u64);
                    }

                    // check if we're downloading the entire file
                    if lines_sample_size != usize::MAX {
                        // we're not downloading the entire file, so we need to
                        // scan chunk for newlines
                        let num_lines = chunk.into_iter().filter(|&x| x == b'\n').count();
                        // and keep track of the number of lines downloaded which is ~= sample_size
                        downloaded_lines += num_lines;
                        // we downloaded enough samples, stop downloading
                        if downloaded_lines > lines_sample_size {
                            downloaded_lines -= 1; // subtract 1 because we don't want to count the header row
                            break;
                        }
                    }
                }
                drop(client);

                if show_progress {
                    if snappy_flag {
                        progress.finish_with_message(format!(
                            "Downloaded {}.",
                            HumanBytes(downloaded as u64)
                        ));
                    } else {
                        progress.finish_with_message(format!(
                            "Downloaded {} samples.",
                            HumanCount(downloaded_lines as u64)
                        ));
                    }
                }

                // create a temporary file to write the download file to
                let wtr_file = NamedTempFile::new()?;

                // keep the temporary file around so we can sniff it later
                // we'll delete it when we're done
                let (mut tmp_file, path) =
                    wtr_file.keep().map_err(|_| "Cannot keep temporary file")?;

                let wtr_file_path;
                let mut downloaded_records = 0_usize;

                if snappy_flag {
                    // we downloaded a snappy compressed file, we need to decompress it
                    // before we can sniff it
                    wtr_file_path =
                        util::decompress_snappy_file(&file.path().to_path_buf(), tmpdir)?;
                } else if shortcircuit_flag {
                    // on linux, we can short-circuit downloading by checking the file type
                    // from the first chunk. If the file is not a CSV file, we just write
                    // the first chunk to a file and return
                    wtr_file_path = path.display().to_string();
                    tmp_file.write_all(&firstchunk)?;
                    tmp_file.flush()?;
                } else {
                    // we downloaded a non-snappy file and it might be a CSV file.
                    // Rewrite it so we only have the exact sample size and truncate potentially
                    // incomplete lines. We do this coz we streamed the download and the downloaded
                    // file may be more than the sample size, and the final line
                    // may be incomplete.
                    wtr_file_path = path.display().to_string();
                    let mut wtr = Config::new(&Some(wtr_file_path.clone()))
                        .no_headers(false)
                        .flexible(true)
                        .quote_style(csv::QuoteStyle::NonNumeric)
                        .writer()?;

                    let retrieved_name = file.path().to_str().unwrap().to_string();
                    let config = Config::new(&Some(retrieved_name))
                        .delimiter(args.flag_delimiter)
                        // we say no_headers so we can just copy the downloaded file over
                        // including headers, to the exact sanple size file
                        .no_headers(true)
                        .flexible(true);

                    let mut rdr = config.reader()?;

                    // amortize allocation
                    #[allow(unused_assignments)]
                    let mut record = csv::ByteRecord::with_capacity(100, 20);

                    let header_row = rdr.byte_headers()?;
                    wtr.write_byte_record(header_row)?;
                    rdr.byte_records().next();

                    for rec in rdr.byte_records() {
                        record = rec?;
                        if downloaded_records >= lines_sample_size {
                            break;
                        }
                        downloaded_records += 1;
                        wtr.write_byte_record(&record)?;
                    }
                    wtr.flush()?;
                }

                Ok(SniffFileStruct {
                    display_path: url,
                    file_to_sniff: wtr_file_path,
                    tempfile_flag: true,
                    retrieved_size: downloaded,
                    file_size: if magic_flag && total_size == usize::MAX {
                        // we're using magic and the server didn't give us content length
                        // so send usize::MAX - 1 to indicate that we don't know the file size
                        usize::MAX - 1
                    } else if total_size == usize::MAX {
                        // the server didn't give us content length, so we just
                        // downloaded the entire file. downloaded variable
                        // is the total size of the file
                        downloaded
                    } else {
                        total_size
                    },
                    last_modified,
                    downloaded_records,
                })
            }
            // its a file. If its a snappy file, decompress it first
            // aftwerwards, check if its one of the supported file types
            // finally, check if its a utf8 file
            path => {
                let mut path = path;

                let mut pathbuf = PathBuf::from(path.clone());
                let file_ext = pathbuf.extension();
                match file_ext {
                    Some(ext) => {
                        let mut lower_ext =
                            ext.to_str().unwrap().to_lowercase().as_str().to_owned();
                        if lower_ext == "sz" {
                            path = util::decompress_snappy_file(&pathbuf, tmpdir)?;
                            pathbuf = PathBuf::from(path.clone());
                            lower_ext = pathbuf
                                .extension()
                                .unwrap()
                                .to_os_string()
                                .into_string()
                                .unwrap();
                            log::info!("Decompressed {lower_ext} file to {path}");
                        }
                        // on linux, we don't need to check the extension
                        // because we use magic to get the file type
                        #[cfg(not(feature = "magic"))]
                        match lower_ext.as_str() {
                            "csv" | "tsv" | "txt" | "tab" => {}
                            ext if ext.ends_with("_decompressed") => {}
                            _ => {
                                return fail_clierror!(
                                    "File extension '{lower_ext}' is not supported",
                                    // ext = ext.to_str().unwrap()
                                );
                            }
                        }
                    }
                    None => {
                        // on linux, we log a warning and continue if no
                        // extension is found. On other platforms, we fail
                        #[cfg(not(feature = "magic"))]
                        return fail_clierror!("File extension not found");
                        #[cfg(all(target_os = "linux", feature = "magic"))]
                        log::warn!("File extension not found");
                    }
                }

                let metadata = fs::metadata(&path)
                    .map_err(|_| format!("Cannot get metadata for file '{path}'"))?;

                let file_size = metadata.len() as usize;
                let last_modified = match metadata.modified() {
                    Ok(time) => {
                        let timestamp = time
                            .duration_since(SystemTime::UNIX_EPOCH)
                            .unwrap()
                            .as_secs();
                        let naive = chrono::NaiveDateTime::from_timestamp_opt(timestamp as i64, 0)
                            .unwrap_or_default();
                        let datetime =
                            chrono::DateTime::<chrono::Utc>::from_utc(naive, chrono::Utc);
                        // format the datetime to RFC3339
                        format!("{datetime}", datetime = datetime.format("%+"))
                    }
                    Err(_) => "N/A".to_string(),
                };

                let canonical_path = fs::canonicalize(&path)?.to_str().unwrap().to_string();

                Ok(SniffFileStruct {
                    display_path: canonical_path,
                    file_to_sniff: path,
                    tempfile_flag: false,
                    retrieved_size: file_size,
                    file_size,
                    last_modified,
                    downloaded_records: 0,
                })
            }
        }
    } else {
        // read from stdin and write to a temp file
        let mut stdin_file = NamedTempFile::new()?;
        let stdin = std::io::stdin();
        let mut stdin_handle = stdin.lock();
        std::io::copy(&mut stdin_handle, &mut stdin_file)?;
        drop(stdin_handle);
        let (file, path) = stdin_file
            .keep()
            .map_err(|_| "Cannot keep temporary file")?;

        if !util::isutf8_file(&path)? {
            return fail_clierror!("stdin input is not UTF8-encoded");
        }

        let metadata = file
            .metadata()
            .map_err(|_| "Cannot get metadata for stdin file")?;

        let file_size = metadata.len() as usize;
        // set last_modified to now in RFC3339 format
        let last_modified = chrono::Utc::now().format("%+").to_string();
        let path_string = path
            .into_os_string()
            .into_string()
            .unwrap_or_else(|_| "???".to_string());

        Ok(SniffFileStruct {
            display_path: "stdin".to_string(),
            file_to_sniff: path_string,
            tempfile_flag: true,
            retrieved_size: file_size,
            file_size,
            last_modified,
            downloaded_records: 0,
        })
    }
}

fn cleanup_tempfile(
    tempfile_flag: bool,
    tempfile: String,
) -> Result<(), crate::clitypes::CliError> {
    if tempfile_flag {
        fs::remove_file(tempfile)?;
    }
    Ok(())
}

#[allow(clippy::unused_async)] // false positive lint
pub async fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;

    let mut sample_size = args.flag_sample;
    if sample_size < 0.0 {
        if args.flag_json || args.flag_pretty_json {
            let json_result = json!({
                "errors": [{
                    "title": "sniff error",
                    "detail": "Sample size must be greater than or equal to zero."
                }]
            });
            return fail_clierror!("{json_result}");
        }
        return fail_clierror!("Sample size must be greater than or equal to zero.");
    }

    let sniffed_ts = chrono::Utc::now().to_rfc3339();
    let tmpdir = tempfile::tempdir()?;

    let future = get_file_to_sniff(&args, &tmpdir);
    let sfile_info = block_on(future)?;
    let tempfile_to_delete = sfile_info.file_to_sniff.clone();

    // on linux, check what kind of file we have
    // if its NOT a CSV or a text file, we fail, showing the detected mime type
    #[cfg(all(target_os = "linux", feature = "magic"))]
    let file_type = util::get_filetype(&sfile_info.file_to_sniff)?;
    #[cfg(all(target_os = "linux", feature = "magic"))]
    // we also accept text/* files, as sniff may still be able to suss out
    // if the file is a CSV, even if its not using a CSV extension
    // we can also be here if the user has specified --no-infer
    if (file_type != "application/csv" && !file_type.starts_with("text/")) || args.flag_no_infer {
        cleanup_tempfile(sfile_info.tempfile_flag, tempfile_to_delete)?;
        if args.flag_json || args.flag_pretty_json {
            let size = if sfile_info.file_size >= usize::MAX - 1 {
                "Unknown".to_string()
            } else {
                sfile_info.file_size.to_string()
            };
            if args.flag_no_infer {
                let json_result = json!({
                    "title": "sniff mime type",
                    "detail": format!("File is not a CSV file. Detected mime type: {file_type}"),
                    "meta": {
                        "detected_mime_type": file_type,
                        "size": size,
                        "last_modified": sfile_info.last_modified,
                    }
                });
                if args.flag_pretty_json {
                    woutinfo!("{}", serde_json::to_string_pretty(&json_result).unwrap());
                    return Ok(());
                }
                woutinfo!("{json_result}");
                return Ok(());
            } else {
                let json_result = json!({
                    "errors": [{
                        "title": "sniff error",
                        "detail": format!("File is not a CSV file. Detected mime type: {file_type}"),
                        "meta": {
                            "detected_mime_type": file_type,
                            "size": size,
                            "last_modified": sfile_info.last_modified,
                        }
                    }]
                });
                if args.flag_pretty_json {
                    return fail_clierror!(
                        "{}",
                        serde_json::to_string_pretty(&json_result).unwrap()
                    );
                }
                return fail_clierror!("{json_result}");
            }
        }
        if args.flag_no_infer {
            woutinfo!(
                "Detected mime type: {file_type}, size: {size}, last modified: {last_modified}"
            );
            return Ok(());
        } else {
            return fail_clierror!(
                "File is not a CSV file. Detected mime type: {file_type}, size: {size}, last \
                 modified: {last_modified}"
            );
        }
    }

    let conf = Config::new(&Some(sfile_info.file_to_sniff.clone()))
        .flexible(true)
        .delimiter(args.flag_delimiter);
    let n_rows = if sfile_info.downloaded_records == 0
        || sfile_info.retrieved_size <= sfile_info.file_size
    {
        //if we have the whole file and not just a sample, we can count the number of rows
        match util::count_rows(&conf) {
            Ok(n) => n as usize,
            Err(e) => {
                cleanup_tempfile(sfile_info.tempfile_flag, tempfile_to_delete)?;

                if args.flag_json || args.flag_pretty_json {
                    let json_result = json!({
                        "errors": [{
                            "title": "count rows error",
                            "detail": e.to_string()
                        }]
                    });
                    return fail_clierror!("{json_result}");
                }
                return fail_clierror!("{}", e);
            }
        }
    } else {
        // sfile_info.sampled_records
        // usize::MAX is a sentinel value to let us
        // know that we need to estimate the number of records
        // since we only downloaded a sample, not the entire file
        usize::MAX
    };

    // its an empty file, exit with an error
    if n_rows == 0 {
        cleanup_tempfile(sfile_info.tempfile_flag, tempfile_to_delete)?;

        if args.flag_json || args.flag_pretty_json {
            let json_result = json!({
                "errors": [{
                    "title": "sniff error",
                    "detail": "Empty file"
                }]
            });
            return fail_clierror!("{json_result}");
        }
        return fail_clierror!("Empty file");
    }

    let mut sample_all = false;
    // its a percentage, get the actual sample size
    #[allow(clippy::cast_precision_loss)]
    if sample_size < 1.0 {
        sample_size *= n_rows as f64;
    } else if (sample_size).abs() < f64::EPSILON {
        // its zero, the epsilon bit is because comparing a float
        // is really not precise - see https://floating-point-gui.de/errors/comparison/
        sample_all = true;
    }

    // for a local file and stdin, set sampled_records to the sample size
    // for a remote file, set sampled_records to the number of rows downloaded
    let sampled_records = if sfile_info.downloaded_records == 0 {
        sample_size as usize
    } else {
        sample_all = true;
        sfile_info.downloaded_records
    };

    let rdr = conf.reader_file()?;

    let dt_preference = if args.flag_prefer_dmy || conf.get_dmy_preference() {
        DatePreference::DmyFormat
    } else {
        DatePreference::MdyFormat
    };

    if let Some(save_urlsample) = args.flag_save_urlsample {
        fs::copy(sfile_info.file_to_sniff.clone(), save_urlsample)?;
    }

    let sniff_results = if sample_all {
        log::info!("Sniffing ALL rows...");
        if let Some(delimiter) = args.flag_delimiter {
            Sniffer::new()
                .sample_size(SampleSize::All)
                .date_preference(dt_preference)
                .delimiter(delimiter.as_byte())
                .sniff_reader(rdr.into_inner())
        } else {
            Sniffer::new()
                .sample_size(SampleSize::All)
                .date_preference(dt_preference)
                .sniff_reader(rdr.into_inner())
        }
    } else {
        let mut sniff_size = sample_size as usize;
        // sample_size is at least 20
        if sniff_size < 20 {
            sniff_size = 20;
        }
        log::info!("Sniffing {sniff_size} rows...");
        if let Some(delimiter) = args.flag_delimiter {
            Sniffer::new()
                .sample_size(SampleSize::Records(sniff_size))
                .date_preference(dt_preference)
                .delimiter(delimiter.as_byte())
                .sniff_reader(rdr.into_inner())
        } else {
            Sniffer::new()
                .sample_size(SampleSize::Records(sniff_size))
                .date_preference(dt_preference)
                .sniff_reader(rdr.into_inner())
        }
    };

    let mut processed_results = SniffStruct::default();
    let mut sniffing_error: Option<String> = None;

    match sniff_results {
        Ok(metadata) => {
            let (num_records, estimated) = rowcount(&metadata, &sfile_info, n_rows);

            let sniffedfields = metadata
                .fields
                .iter()
                .map(std::string::ToString::to_string)
                .collect();
            let sniffedtypes = metadata
                .types
                .iter()
                .map(std::string::ToString::to_string)
                .collect();

            processed_results = SniffStruct {
                path: sfile_info.display_path,
                sniff_timestamp: sniffed_ts,
                last_modified: sfile_info.last_modified,
                delimiter_char: metadata.dialect.delimiter as char,
                header_row: metadata.dialect.header.has_header_row,
                preamble_rows: metadata.dialect.header.num_preamble_rows,
                quote_char: match metadata.dialect.quote {
                    qsv_sniffer::metadata::Quote::Some(chr) => format!("{}", char::from(chr)),
                    qsv_sniffer::metadata::Quote::None => "none".into(),
                },
                flexible: metadata.dialect.flexible,
                is_utf8: metadata.dialect.is_utf8,
                #[cfg(all(target_os = "linux", feature = "magic"))]
                detected_mime: file_type,
                retrieved_size: sfile_info.retrieved_size,
                file_size: sfile_info.file_size, // sfile_info.file_size,
                sampled_records: if sampled_records > num_records {
                    num_records
                } else {
                    sampled_records
                },
                estimated,
                num_records,
                avg_record_len: metadata.avg_record_len,
                num_fields: metadata.num_fields,
                fields: sniffedfields,
                types: sniffedtypes,
                stats_types: args.flag_stats_types,
            };
        }
        Err(e) => {
            #[cfg(all(target_os = "linux", feature = "magic"))]
            {
                sniffing_error = Some(format!("{e}. Detected mime type: {file_type}"));
            }
            #[cfg(not(feature = "magic"))]
            {
                sniffing_error = Some(format!("{e}"));
            }
        }
    }

    cleanup_tempfile(sfile_info.tempfile_flag, tempfile_to_delete)?;

    if args.flag_json || args.flag_pretty_json {
        if sniffing_error.is_none() {
            if args.flag_pretty_json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&processed_results).unwrap()
                );
            } else {
                println!("{}", serde_json::to_string(&processed_results).unwrap());
            };
            Ok(())
        } else {
            let json_error = json!({
                "errors": [{
                    "title": "sniff error",
                    "detail": sniffing_error.unwrap()
                }]
            });
            fail_clierror!("{json_error}")
        }
    } else if sniffing_error.is_none() {
        println!("{processed_results}");
        return Ok(());
    } else {
        return fail_clierror!("{}", sniffing_error.unwrap());
    }
}
