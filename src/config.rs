use std::{
    env, fs,
    io::{self, Read},
    path::{Path, PathBuf},
    sync::atomic::{AtomicBool, Ordering},
};

use log::{debug, info, warn};
use qsv_sniffer::{SampleSize, Sniffer};
use serde::de::{Deserialize, Deserializer, Error};

use crate::{
    index::Indexed,
    select::{SelectColumns, Selection},
    util, CliResult,
};

// rdr default is 8k in csv crate, we're making it 128k
pub const DEFAULT_RDR_BUFFER_CAPACITY: usize = 128 * (1 << 10);
// previous wtr default in xsv is 32k, we're making it 512k
pub const DEFAULT_WTR_BUFFER_CAPACITY: usize = 512 * (1 << 10);

// number of rows for qsv_sniffer to sample
const DEFAULT_SNIFFER_SAMPLE: usize = 100;

// file size at which we warn user that a large file has not been indexed
const NO_INDEX_WARNING_FILESIZE: u64 = 100_000_000; // 100MB

// so we don't have to keep checking if the index has been created
static AUTO_INDEXED: AtomicBool = AtomicBool::new(false);

pub static SPONSOR_MESSAGE: &str = r#"sponsored by datHere - Data Infrastructure Engineering (https://qsv.datHere.com)
Need a UI & more advanced data-wrangling? Upgrade to qsv pro (https://qsvpro.datHere.com)
"#;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Delimiter(pub u8);

/// Delimiter represents values that can be passed from the command line that
/// can be used as a field delimiter in CSV data.
///
/// Its purpose is to ensure that the Unicode character given decodes to a
/// valid ASCII character as required by the CSV parser.
impl Delimiter {
    pub const fn as_byte(self) -> u8 {
        self.0
    }

    pub fn decode_delimiter(s: &str) -> Result<Delimiter, String> {
        if s == r"\t" {
            return Ok(Delimiter(b'\t'));
        }

        if s.len() != 1 {
            return fail_format!("Could not convert '{s}' to a single ASCII character.");
        }

        let c = s.chars().next().unwrap();
        if c.is_ascii() {
            Ok(Delimiter(c as u8))
        } else {
            fail_format!("Could not convert '{c}' to ASCII delimiter.")
        }
    }
}

impl<'de> Deserialize<'de> for Delimiter {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Delimiter, D::Error> {
        let s = String::deserialize(d)?;
        match Delimiter::decode_delimiter(&s) {
            Ok(delim) => Ok(delim),
            Err(msg) => Err(D::Error::custom(msg)),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Config {
    pub path:              Option<PathBuf>, // None implies <stdin>
    idx_path:              Option<PathBuf>,
    select_columns:        Option<SelectColumns>,
    delimiter:             u8,
    pub no_headers:        bool,
    pub flexible:          bool,
    terminator:            csv::Terminator,
    pub quote:             u8,
    quote_style:           csv::QuoteStyle,
    double_quote:          bool,
    escape:                Option<u8>,
    quoting:               bool,
    pub preamble_rows:     u64,
    trim:                  csv::Trim,
    pub autoindex_size:    u64,
    prefer_dmy:            bool,
    pub comment:           Option<u8>,
    snappy:                bool, // flag to enable snappy compression/decompression
    pub read_buffer:       u32,
    pub write_buffer:      u32,
    pub skip_format_check: bool,
    pub format_error:      Option<String>,
}

// Empty trait as an alias for Seek and Read that avoids auto trait errors
#[cfg(any(feature = "feature_capable", feature = "lite"))]
pub trait SeekRead: io::Seek + io::Read {}
#[cfg(any(feature = "feature_capable", feature = "lite"))]
impl<T: io::Seek + io::Read> SeekRead for T {}

impl Config {
    /// Creates a new `Config` instance with default settings and optional file path.
    ///
    /// # Arguments
    ///
    /// * `path` - An optional reference to a `String` representing the file path.
    ///
    /// # Returns
    ///
    /// A new `Config` instance.
    ///
    /// # Details
    ///
    /// This function initializes a `Config` with the following behavior:
    /// - Uses env var `QSV_DEFAULT_DELIMITER` for default delimiter, or ',' if not set
    /// - Determines delimiter and Snappy compression based on file extension.
    /// - Supports sniffing delimiter and preamble rows if `QSV_SNIFF_DELIMITER` or
    ///   `QSV_SNIFF_PREAMBLE` is set.
    /// - Sets comment character from `QSV_COMMENT_CHAR` environment variable.
    /// - Sets headers behavior based on `QSV_NO_HEADERS` environment variable.
    /// - Configures various other settings from environment variables.
    ///
    /// # Environment Variables
    ///
    /// - `QSV_DEFAULT_DELIMITER`: Sets the default delimiter.
    /// - `QSV_SNIFF_DELIMITER` or `QSV_SNIFF_PREAMBLE`: Enables sniffing of delimiter and preamble
    ///   rows.
    /// - `QSV_COMMENT_CHAR`: Sets the comment character.
    /// - `QSV_NO_HEADERS`: Determines if the file has headers.
    /// - `QSV_AUTOINDEX_SIZE`: Sets the auto-index size.
    /// - `QSV_PREFER_DMY`: Sets date format preference.
    /// - `QSV_RDR_BUFFER_CAPACITY`: Sets read buffer capacity.
    /// - `QSV_WTR_BUFFER_CAPACITY`: Sets write buffer capacity.
    /// - `QSV_SKIP_FORMAT_CHECK`: Set to skip mime-type checking.
    pub fn new(path: Option<&String>) -> Config {
        let default_delim = match env::var("QSV_DEFAULT_DELIMITER") {
            Ok(delim) => Delimiter::decode_delimiter(&delim).unwrap().as_byte(),
            _ => b',',
        };
        let mut skip_format_check = true;
        let mut format_error = None;
        let (path, mut delim, snappy) = match path {
            None => (None, default_delim, false),
            // WIP: support remote files; currently only http(s) is supported
            // Some(ref s) if s.starts_with("http") && Url::parse(s).is_ok() => {
            //     let mut snappy = false;
            //     let delim = if s.ends_with(".csv.sz") {
            //         snappy = true;
            //         b','
            //     } else if s.ends_with(".tsv.sz") || s.ends_with(".tab.sz") {
            //         snappy = true;
            //         b'\t'
            //     } else {
            //         default_delim
            //     };
            //     // download the file to a temporary location
            //     util::download_file()
            //     (Some(PathBuf::from(s)), delim, snappy)
            // },
            Some(s) if s == "-" => (None, default_delim, false),
            Some(ref s) => {
                let path = PathBuf::from(s);
                skip_format_check = util::get_envvar_flag("QSV_SKIP_FORMAT_CHECK");
                if !skip_format_check {
                    if let Ok(file_format) = file_format::FileFormat::from_file(&path) {
                        let detected_mime = file_format.media_type();
                        // determine the file type by scanning the file
                        // we support the following mime-types:
                        //  x-empty: empty file
                        //  octet-stream: the file-format crate falls back to this when it cannot
                        //   figure the mime-type, so its not actually binary data
                        //  x-snappy-framed: for snappy compressed files
                        //  text/*: its a text file type of some sort that is a possible CSV
                        //   candidate that we will trap later on with the csv crate
                        if !(detected_mime == "application/x-empty"
                            || detected_mime == "application/octet-stream"
                            || detected_mime == "application/x-snappy-framed"
                            || detected_mime.starts_with("text/"))
                        {
                            format_error = Some(format!(
                                "{} is using an unsupported file format: {detected_mime}",
                                path.display()
                            ));
                        }
                    }
                }
                let (file_extension, delim, snappy) = get_delim_by_extension(&path, default_delim);
                (Some(path), delim, snappy || file_extension.ends_with("sz"))
            },
        };
        let sniff = util::get_envvar_flag("QSV_SNIFF_DELIMITER")
            || util::get_envvar_flag("QSV_SNIFF_PREAMBLE");
        let comment: Option<u8> = match env::var("QSV_COMMENT_CHAR") {
            Ok(comment_char) => Some(comment_char.as_bytes().first().unwrap().to_owned()),
            Err(_) => None,
        };
        let no_headers = util::get_envvar_flag("QSV_NO_HEADERS");
        let mut preamble = 0_u64;
        if sniff && path.is_some() {
            let sniff_path = path.as_ref().unwrap().to_str().unwrap();

            match Sniffer::new()
                .sample_size(SampleSize::Records(DEFAULT_SNIFFER_SAMPLE))
                .sniff_path(sniff_path)
            {
                Ok(metadata) => {
                    delim = metadata.dialect.delimiter;
                    preamble = metadata.dialect.header.num_preamble_rows as u64;
                    info!(
                        "sniffed delimiter {} and {preamble} preamble rows",
                        delim as char
                    );
                },
                Err(e) => {
                    // we only warn, as we don't want to stop processing the file
                    // if sniffing doesn't work
                    warn!("sniff error: {e}");
                },
            }
        }

        Config {
            path,
            idx_path: None,
            select_columns: None,
            delimiter: delim,
            no_headers,
            flexible: false,
            terminator: csv::Terminator::Any(b'\n'),
            quote: b'"',
            quote_style: csv::QuoteStyle::Necessary,
            double_quote: true,
            escape: None,
            quoting: true,
            preamble_rows: preamble,
            trim: csv::Trim::None,
            autoindex_size: std::env::var("QSV_AUTOINDEX_SIZE")
                .unwrap_or_else(|_| "0".to_owned())
                .parse()
                .unwrap_or(0),
            prefer_dmy: util::get_envvar_flag("QSV_PREFER_DMY"),
            comment,
            snappy,
            read_buffer: std::env::var("QSV_RDR_BUFFER_CAPACITY")
                .unwrap_or_else(|_| DEFAULT_RDR_BUFFER_CAPACITY.to_string())
                .parse()
                .unwrap_or(DEFAULT_RDR_BUFFER_CAPACITY as u32),
            write_buffer: std::env::var("QSV_WTR_BUFFER_CAPACITY")
                .unwrap_or_else(|_| DEFAULT_WTR_BUFFER_CAPACITY.to_string())
                .parse()
                .unwrap_or(DEFAULT_WTR_BUFFER_CAPACITY as u32),
            format_error,
            skip_format_check,
        }
    }

    pub const fn delimiter(mut self, d: Option<Delimiter>) -> Config {
        if let Some(d) = d {
            self.delimiter = d.as_byte();
        }
        self
    }

    pub const fn get_delimiter(&self) -> u8 {
        self.delimiter
    }

    pub const fn comment(mut self, c: Option<u8>) -> Config {
        self.comment = c;
        self
    }

    pub const fn get_dmy_preference(&self) -> bool {
        self.prefer_dmy
    }

    pub fn no_headers(mut self, mut yes: bool) -> Config {
        if env::var("QSV_TOGGLE_HEADERS").unwrap_or_else(|_| "0".to_owned()) == "1" {
            yes = !yes;
        }
        self.no_headers = yes;
        self
    }

    pub const fn flexible(mut self, yes: bool) -> Config {
        self.flexible = yes;
        self
    }

    #[cfg(any(feature = "feature_capable", feature = "lite"))]
    pub const fn crlf(mut self, yes: bool) -> Config {
        if yes {
            self.terminator = csv::Terminator::CRLF;
        } else {
            self.terminator = csv::Terminator::Any(b'\n');
        }
        self
    }

    #[cfg(any(feature = "feature_capable", feature = "lite"))]
    pub const fn terminator(mut self, term: csv::Terminator) -> Config {
        self.terminator = term;
        self
    }

    pub const fn quote(mut self, quote: u8) -> Config {
        self.quote = quote;
        self
    }

    pub const fn quote_style(mut self, style: csv::QuoteStyle) -> Config {
        self.quote_style = style;
        self
    }

    pub const fn double_quote(mut self, yes: bool) -> Config {
        self.double_quote = yes;
        self
    }

    pub const fn escape(mut self, escape: Option<u8>) -> Config {
        self.escape = escape;
        self
    }

    pub const fn quoting(mut self, yes: bool) -> Config {
        self.quoting = yes;
        self
    }

    pub const fn trim(mut self, trim_type: csv::Trim) -> Config {
        self.trim = trim_type;
        self
    }

    // comment read_buffer() and write_buffer() out for now, as they're not used
    // pub const fn read_buffer(mut self, buffer: u32) -> Config {
    //     self.read_buffer = buffer;
    //     self
    // }

    // pub const fn write_buffer(mut self, buffer: u32) -> Config {
    //     self.write_buffer = buffer;
    //     self
    // }

    #[allow(clippy::missing_const_for_fn)]
    pub fn select(mut self, sel_cols: SelectColumns) -> Config {
        self.select_columns = Some(sel_cols);
        self
    }

    pub const fn is_stdin(&self) -> bool {
        self.path.is_none()
    }

    #[cfg(feature = "polars")]
    pub const fn is_snappy(&self) -> bool {
        self.snappy
    }

    #[inline]
    /// Returns a `Selection` based on the config's `select_columns` & the first record of the CSV.
    ///
    /// # Arguments
    ///
    /// * `first_record` - A reference to the first `ByteRecord` of the CSV.
    ///
    /// # Returns
    ///
    /// * `Result<Selection, String>` - A `Selection` if successful, otherwise, an error msg
    ///
    /// # Errors
    ///
    /// This function will return an error if:
    /// * The `Config` has no `SelectColumns` (i.e., `Config::select` was not called).
    pub fn selection(&self, first_record: &csv::ByteRecord) -> Result<Selection, String> {
        match self.select_columns {
            None => fail!("Config has no 'SelectColumns'. Did you call Config::select?"),
            Some(ref sel) => sel.selection(first_record, !self.no_headers),
        }
    }

    /// Writes the headers from a CSV reader to a CSV writer.
    ///
    /// This function reads the headers from the given CSV reader and writes them to the CSV writer,
    /// but only if the `no_headers` flag is not set. If the headers are empty, nothing is written.
    ///
    /// # Arguments
    ///
    /// * `r` - A mutable reference to a CSV reader.
    /// * `w` - A mutable reference to a CSV writer.
    ///
    /// # Returns
    ///
    /// Returns a `csv::Result<()>` which is `Ok(())` if the operation was successful,
    /// or an error if there was a problem reading or writing.
    pub fn write_headers<R: io::Read, W: io::Write>(
        &self,
        r: &mut csv::Reader<R>,
        w: &mut csv::Writer<W>,
    ) -> csv::Result<()> {
        if !self.no_headers {
            let r = r.byte_headers()?;
            if !r.is_empty() {
                w.write_record(r)?;
            }
        }
        Ok(())
    }

    pub fn writer(&self) -> io::Result<csv::Writer<Box<dyn io::Write + 'static>>> {
        Ok(self.from_writer(self.io_writer()?))
    }

    pub fn reader(&self) -> io::Result<csv::Reader<Box<dyn io::Read + Send + 'static>>> {
        if !self.skip_format_check && self.format_error.is_some() {
            Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                self.format_error.clone().unwrap(),
            ))
        } else {
            Ok(self.from_reader(self.io_reader()?))
        }
    }

    pub fn reader_file(&self) -> io::Result<csv::Reader<fs::File>> {
        match self.path {
            None => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Cannot use <stdin> here",
            )),
            Some(ref p) => {
                if !self.skip_format_check && self.format_error.is_some() {
                    Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        self.format_error.clone().unwrap(),
                    ))
                } else {
                    fs::File::open(p).map(|f| self.from_reader(f))
                }
            },
        }
    }

    #[cfg(any(feature = "feature_capable", feature = "lite"))]
    pub fn reader_file_stdin(&self) -> io::Result<csv::Reader<Box<dyn SeekRead + 'static>>> {
        Ok(match self.path {
            None => {
                // Create a buffer in memory for stdin
                let mut buffer: Vec<u8> = Vec::new();
                let stdin = io::stdin();
                stdin.lock().read_to_end(&mut buffer)?;
                self.from_reader(Box::new(io::Cursor::new(buffer)))
            },
            Some(ref p) => {
                if !self.skip_format_check && self.format_error.is_some() {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        self.format_error.clone().unwrap(),
                    ));
                }
                self.from_reader(Box::new(fs::File::open(p)?))
            },
        })
    }

    /// Automatically creates an index file for the CSV file.
    ///
    /// This function attempts to create an index file for the CSV file specified in `self.path`.
    /// It's designed to fail silently if any step of the process encounters an error, as it's
    /// intended to be a convenience function.
    ///
    /// # Behavior
    ///
    /// - If the file is Snappy-compressed, the function returns immediately w/o creating an index.
    /// - If `self.path` is `None`, the function returns without action.
    /// - The function creates an index file using `util::idx_path()` to determine index file path.
    /// - It uses `csv_index::RandomAccessSimple::create()` to generate the index.
    /// - If index creation is successful, it sets the `AUTO_INDEXED` atomic flag to `true`.
    ///
    /// # Errors
    ///
    /// While this function doesn't return any errors, it logs debug messages for both successful
    /// and failed index creation attempts.
    fn autoindex_file(&self) {
        if self.snappy {
            return;
        }

        let Some(path_buf) = &self.path else { return };

        let pidx = util::idx_path(Path::new(path_buf));
        let Ok(idxfile) = fs::File::create(pidx) else {
            return;
        };
        let Ok(mut rdr) = self.reader_file() else {
            return;
        };
        let mut wtr = io::BufWriter::with_capacity(DEFAULT_WTR_BUFFER_CAPACITY, idxfile);
        match csv_index::RandomAccessSimple::create(&mut rdr, &mut wtr) {
            Ok(()) => {
                let Ok(()) = io::Write::flush(&mut wtr) else {
                    return;
                };
                debug!("autoindex of {path_buf:?} successful.");
                AUTO_INDEXED.store(true, Ordering::Relaxed);
            },
            Err(e) => debug!("autoindex of {path_buf:?} failed: {e}"),
        }
    }

    /// Check if the index file exists and is newer than the CSV file.
    /// If so, return the CSV file handle and the index file handle. If not, return None.
    /// Unless the CSV's file size >= QSV_AUTOINDEX_SIZE, then we'll create an index automatically.
    /// This will also automatically update stale indices (i.e. the CSV is newer than the index )
    pub fn index_files(&self) -> io::Result<Option<(csv::Reader<fs::File>, fs::File)>> {
        let mut data_modified = 0_u64;
        let data_fsize;
        let mut idx_path_work = PathBuf::new();

        // the auto_indexed flag is set when an index is created automatically with
        // autoindex_file(). We use this flag to avoid checking if the index exists every
        // time this function is called. If the index was already auto-indexed, we can just
        // use it & return immediately.
        let auto_indexed = AUTO_INDEXED.load(Ordering::Relaxed);

        let (csv_file, mut idx_file) = if auto_indexed {
            (
                fs::File::open(self.path.clone().unwrap())?,
                fs::File::open(util::idx_path(&self.path.clone().unwrap()))?,
            )
        } else {
            match (&self.path, &self.idx_path) {
                (&None, &None) => return Ok(None),
                (&None, &Some(_)) => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "Cannot use <stdin> with indexes",
                    ));
                },
                (Some(p), Some(ip)) => (fs::File::open(p)?, fs::File::open(ip)?),
                (Some(p), &None) => {
                    // We generally don't want to report an error here, since we're
                    // passively trying to find an index.

                    (data_modified, data_fsize) = util::file_metadata(&p.metadata()?);
                    idx_path_work = util::idx_path(p);
                    let idx_file = match fs::File::open(&idx_path_work) {
                        Err(_) => {
                            // the index file doesn't exist
                            if self.snappy {
                                // cannot index snappy compressed files
                                return Ok(None);
                            } else if self.autoindex_size > 0 && data_fsize >= self.autoindex_size {
                                // if CSV file size >= QSV_AUTOINDEX_SIZE, and
                                // its not a snappy file, create an index automatically
                                self.autoindex_file();
                                fs::File::open(&idx_path_work)?
                            } else if data_fsize >= NO_INDEX_WARNING_FILESIZE {
                                // warn user that the CSV file is large and not indexed
                                use indicatif::HumanCount;

                                warn!(
                                    "The {} MB CSV file is larger than the {} MB \
                                     NO_INDEX_WARNING_FILESIZE threshold. Consider creating an \
                                     index file as it will make qsv commands much faster.",
                                    HumanCount(data_fsize * 100),
                                    HumanCount(NO_INDEX_WARNING_FILESIZE * 100)
                                );
                                return Ok(None);
                            } else {
                                // CSV not greater than QSV_AUTOINDEX_SIZE, and not greater than
                                // NO_INDEX_WARNING_FILESIZE, so we don't create an index
                                return Ok(None);
                            }
                        },
                        Ok(f) => f,
                    };
                    (fs::File::open(p)?, idx_file)
                },
            }
        };
        // If the CSV data was last modified after the index file was last
        // modified, recreate the stale index automatically
        let (idx_modified, _) = util::file_metadata(&idx_file.metadata()?);
        if data_modified > idx_modified {
            info!("index stale... autoindexing...");
            self.autoindex_file();
            idx_file = fs::File::open(&idx_path_work)?;
        }

        let csv_rdr = self.from_reader(csv_file);
        Ok(Some((csv_rdr, idx_file)))
    }

    /// Check if the index file exists and is newer than the CSV file.
    /// If so, return the index file.
    /// If not, return None.
    /// Unless QSV_AUTOINDEX is set, in which case, we'll recreate the
    /// stale index automatically
    #[inline]
    pub fn indexed(&self) -> CliResult<Option<Indexed<fs::File, fs::File>>> {
        match self.index_files()? {
            None => Ok(None),
            Some((r, i)) => Ok(Some(Indexed::open(r, i)?)),
        }
    }

    pub fn io_reader(&self) -> io::Result<Box<dyn io::Read + Send + 'static>> {
        Ok(match self.path {
            None => Box::new(io::stdin()),
            Some(ref p) => match fs::File::open(p) {
                Ok(x) => {
                    if self.snappy {
                        info!("decoding snappy-compressed file: {}", p.display());
                        Box::new(snap::read::FrameDecoder::new(x))
                    } else {
                        Box::new(x)
                    }
                },
                Err(err) => {
                    let msg = format!("failed to open {}: {}", p.display(), err);
                    return Err(io::Error::new(io::ErrorKind::NotFound, msg));
                },
            },
        })
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn from_reader<R: Read>(&self, rdr: R) -> csv::Reader<R> {
        csv::ReaderBuilder::new()
            .flexible(self.flexible)
            .delimiter(self.delimiter)
            .has_headers(!self.no_headers)
            .quote(self.quote)
            .quoting(self.quoting)
            .escape(self.escape)
            .buffer_capacity(self.read_buffer as usize)
            .comment(self.comment)
            .trim(self.trim)
            .from_reader(rdr)
    }

    pub fn io_writer(&self) -> io::Result<Box<dyn io::Write + 'static>> {
        Ok(match self.path {
            None => Box::new(io::stdout()),
            Some(ref p) => {
                let p_str = p.as_os_str();
                if p_str == "sink" {
                    // sink is /dev/null
                    Box::new(io::sink())
                } else if self.snappy {
                    info!("writing snappy-compressed file: {p:?}");
                    Box::new(snap::write::FrameEncoder::new(fs::File::create(p)?))
                } else {
                    Box::new(fs::File::create(p)?)
                }
            },
        })
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn from_writer<W: io::Write>(&self, mut wtr: W) -> csv::Writer<W> {
        if util::get_envvar_flag("QSV_OUTPUT_BOM") {
            wtr.write_all("\u{FEFF}".as_bytes()).unwrap();
        }

        csv::WriterBuilder::new()
            .flexible(self.flexible)
            .delimiter(self.delimiter)
            .terminator(self.terminator)
            .quote(self.quote)
            .quote_style(self.quote_style)
            .double_quote(self.double_quote)
            .escape(self.escape.unwrap_or(b'\\'))
            .buffer_capacity(self.write_buffer as usize)
            .from_writer(wtr)
    }
}

/// Determines the delimiter and compression status based on the file extension.
///
/// # Arguments
///
/// * `path` - A reference to the `Path` of the file.
/// * `default_delim` - The default delimiter to use if not determined by extension.
///
/// # Returns
///
/// A tuple containing:
/// * `String` - The lowercase file extension.
/// * `u8` - The determined delimiter.
/// * `bool` - Whether the file is Snappy-compressed.
///
/// # Details
///
/// This function examines the file extension to determine:
/// 1. The appropriate delimiter (tab for .tsv/.tab, semicolon for .ssv, comma for .csv).
/// 2. Whether the file is Snappy-compressed (indicated by a .sz extension).
/// 3. For Snappy-compressed files, it checks the extension before .sz to determine the delimiter.
///
/// If the file extension doesn't match known types, it returns the default delimiter.
pub fn get_delim_by_extension(path: &Path, default_delim: u8) -> (String, u8, bool) {
    let path_str = path.to_str().unwrap_or_default().to_ascii_lowercase();

    // we already lowercased the path_str, so allow this false positive lint
    #[allow(clippy::case_sensitive_file_extension_comparisons)]
    let snappy = path_str.ends_with(".sz");

    // Get the extension before .sz if it's a snappy file, otherwise get the normal extension
    let file_extension = if snappy {
        path_str
            .strip_suffix(".sz")
            .and_then(|s| s.split('.').last())
            .unwrap_or("")
            .to_string()
    } else {
        path.extension()
            .unwrap_or_default()
            .to_str()
            .unwrap()
            .to_ascii_lowercase()
    };

    let delim = match file_extension.as_str() {
        "tsv" | "tab" => b'\t',
        "ssv" => b';',
        "csv" => b',',
        _ => default_delim,
    };

    (file_extension, delim, snappy)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    #[test]
    fn test_csv_extension() {
        let path = PathBuf::from("test.csv");
        let (ext, delim, snappy) = get_delim_by_extension(&path, b',');
        assert_eq!(ext, "csv");
        assert_eq!(delim, b',');
        assert!(!snappy);
    }

    #[test]
    fn test_tsv_extension() {
        let path = PathBuf::from("test.tsv");
        let (ext, delim, snappy) = get_delim_by_extension(&path, b',');
        assert_eq!(ext, "tsv");
        assert_eq!(delim, b'\t');
        assert!(!snappy);
    }

    #[test]
    fn test_ssv_extension() {
        let path = PathBuf::from("test.ssv");
        let (ext, delim, snappy) = get_delim_by_extension(&path, b',');
        assert_eq!(ext, "ssv");
        assert_eq!(delim, b';');
        assert!(!snappy);
    }

    #[test]
    fn test_snappy_csv_extension() {
        let path = PathBuf::from("test.csv.sz");
        let (ext, delim, snappy) = get_delim_by_extension(&path, b',');
        assert_eq!(ext, "csv");
        assert_eq!(delim, b',');
        assert!(snappy);
    }

    #[test]
    fn test_snappy_tsv_extension() {
        let path = PathBuf::from("test.tsv.sz");
        let (ext, delim, snappy) = get_delim_by_extension(&path, b',');
        assert_eq!(ext, "tsv");
        assert_eq!(delim, b'\t');
        assert!(snappy);
    }

    #[test]
    fn test_unknown_extension() {
        let path = PathBuf::from("test.unknown");
        let default_delim = b'|';
        let (ext, delim, snappy) = get_delim_by_extension(&path, default_delim);
        assert_eq!(ext, "unknown");
        assert_eq!(delim, default_delim);
        assert!(!snappy);
    }

    #[test]
    fn test_no_extension() {
        let path = PathBuf::from("test");
        let default_delim = b',';
        let (ext, delim, snappy) = get_delim_by_extension(&path, default_delim);
        assert_eq!(ext, "");
        assert_eq!(delim, default_delim);
        assert!(!snappy);
    }
}
