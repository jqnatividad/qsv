use std::borrow::ToOwned;
use std::env;
use std::fs;
use std::io::{self, Read};
use std::ops::Deref;
use std::path::PathBuf;

use qsv_sniffer::{SampleSize, Sniffer};
use log::{debug, info, warn};
use serde::de::{Deserialize, Deserializer, Error};

use crate::index::Indexed;
use crate::select::{SelectColumns, Selection};
use crate::util;
use crate::CliResult;

// rdr default is 8k in csv crate, we're doubling it
const DEFAULT_RDR_BUFFER_CAPACITY: usize = 16 * (1 << 10);
// previous wtr default in xsv is 32k, we're doubling it
pub const DEFAULT_WTR_BUFFER_CAPACITY: usize = 64 * (1 << 10);
// number of rows for qsv_sniffer to sample
const DEFAULT_SNIFFER_SAMPLE: usize = 100;
// for files, number of bytes to check for UTF8 encoding
const DEFAULT_UTF8_CHECK_BUFFER_LEN: usize = 8192;
const UTF8_ERROR_MSG: &str = "is not UTF-8 encoded. Use the input command to transcode to UTF-8.";

#[derive(Clone, Copy, Debug)]
pub struct Delimiter(pub u8);

/// Delimiter represents values that can be passed from the command line that
/// can be used as a field delimiter in CSV data.
///
/// Its purpose is to ensure that the Unicode character given decodes to a
/// valid ASCII character as required by the CSV parser.
impl Delimiter {
    pub fn as_byte(self) -> u8 {
        self.0
    }

    fn decode_delimiter(s: &str) -> Result<Delimiter, String> {
        if s == r"\t" {
            return Ok(Delimiter(b'\t'));
        }

        if s.len() != 1 {
            return Err(format!(
                "Could not convert '{s}' to a single ASCII character."
            ));
        }

        let c = s.chars().next().unwrap();
        if c.is_ascii() {
            Ok(Delimiter(c as u8))
        } else {
            Err(format!("Could not convert '{c}' to ASCII delimiter."))
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

#[derive(Debug)]
pub struct Config {
    path: Option<PathBuf>, // None implies <stdin>
    idx_path: Option<PathBuf>,
    select_columns: Option<SelectColumns>,
    delimiter: u8,
    pub no_headers: bool,
    flexible: bool,
    terminator: csv::Terminator,
    pub quote: u8,
    quote_style: csv::QuoteStyle,
    double_quote: bool,
    escape: Option<u8>,
    quoting: bool,
    pub preamble_rows: u64,
    trim: csv::Trim,
    autoindex: bool,
    checkutf8: bool,
}

// Empty trait as an alias for Seek and Read that avoids auto trait errors
pub trait SeekRead: io::Seek + io::Read {}
impl<T: io::Seek + io::Read> SeekRead for T {}

impl Config {
    pub fn new(path: &Option<String>) -> Config {
        let default_delim = match env::var("QSV_DEFAULT_DELIMITER") {
            Ok(delim) => Delimiter::decode_delimiter(&delim).unwrap().as_byte(),
            _ => b',',
        };
        let (path, mut delim) = match *path {
            None => (None, default_delim),
            Some(ref s) if s.deref() == "-" => (None, default_delim),
            Some(ref s) => {
                let path = PathBuf::from(s);
                let file_extension = path
                    .extension()
                    .unwrap_or_default()
                    .to_str()
                    .unwrap()
                    .to_lowercase();
                let delim = if file_extension == "tsv" || file_extension == "tab" {
                    b'\t'
                } else if file_extension == "csv" {
                    b','
                } else {
                    default_delim
                };
                (Some(path), delim)
            }
        };
        let sniff =
            env::var("QSV_SNIFF_DELIMITER").is_ok() || env::var("QSV_SNIFF_PREAMBLE").is_ok();
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
                }
                Err(e) => {
                    // we only warn, as we don't want to stop processing the file
                    // if sniffing doesn't work
                    warn!("sniff error: {e}");
                }
            }
        }

        Config {
            path,
            idx_path: None,
            select_columns: None,
            delimiter: delim,
            no_headers: false,
            flexible: false,
            terminator: csv::Terminator::Any(b'\n'),
            quote: b'"',
            quote_style: csv::QuoteStyle::Necessary,
            double_quote: true,
            escape: None,
            quoting: true,
            preamble_rows: preamble,
            trim: csv::Trim::None,
            autoindex: env::var("QSV_AUTOINDEX").is_ok(),
            checkutf8: env::var("QSV_SKIPUTF8_CHECK").is_err(),
        }
    }

    pub fn delimiter(mut self, d: Option<Delimiter>) -> Config {
        if let Some(d) = d {
            self.delimiter = d.as_byte();
        }
        self
    }

    #[allow(dead_code)]
    pub fn get_delimiter(&self) -> u8 {
        self.delimiter
    }

    pub fn no_headers(mut self, mut yes: bool) -> Config {
        if env::var("QSV_TOGGLE_HEADERS").unwrap_or_else(|_| "0".to_owned()) == "1" {
            yes = !yes;
        }
        if env::var("QSV_NO_HEADERS").is_ok() {
            self.no_headers = true;
        } else {
            self.no_headers = yes;
        }
        self
    }

    pub fn flexible(mut self, yes: bool) -> Config {
        self.flexible = yes;
        self
    }

    pub fn crlf(mut self, yes: bool) -> Config {
        if yes {
            self.terminator = csv::Terminator::CRLF;
        } else {
            self.terminator = csv::Terminator::Any(b'\n');
        }
        self
    }

    pub fn terminator(mut self, term: csv::Terminator) -> Config {
        self.terminator = term;
        self
    }

    pub fn quote(mut self, quote: u8) -> Config {
        self.quote = quote;
        self
    }

    pub fn quote_style(mut self, style: csv::QuoteStyle) -> Config {
        self.quote_style = style;
        self
    }

    pub fn double_quote(mut self, yes: bool) -> Config {
        self.double_quote = yes;
        self
    }

    pub fn escape(mut self, escape: Option<u8>) -> Config {
        self.escape = escape;
        self
    }

    pub fn quoting(mut self, yes: bool) -> Config {
        self.quoting = yes;
        self
    }

    pub fn trim(mut self, trim_type: csv::Trim) -> Config {
        self.trim = trim_type;
        self
    }

    pub fn select(mut self, sel_cols: SelectColumns) -> Config {
        self.select_columns = Some(sel_cols);
        self
    }

    pub fn is_stdin(&self) -> bool {
        self.path.is_none()
    }

    pub fn checkutf8(mut self, yes: bool) -> Config {
        self.checkutf8 = yes;
        self
    }

    pub fn selection(&self, first_record: &csv::ByteRecord) -> Result<Selection, String> {
        match self.select_columns {
            None => Err("Config has no 'SelectColums'. Did you call \
                         Config::select?"
                .to_owned()),
            Some(ref sel) => sel.selection(first_record, !self.no_headers),
        }
    }

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

    pub fn reader(&self) -> io::Result<csv::Reader<Box<dyn io::Read + 'static>>> {
        Ok(self.from_reader(self.io_reader()?))
    }

    pub fn reader_file(&self) -> io::Result<csv::Reader<fs::File>> {
        match self.path {
            None => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Cannot use <stdin> here",
            )),
            Some(ref p) => {
                if !self.is_utf8_encoded() {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("{p:?} {UTF8_ERROR_MSG}"),
                    ));
                }
                fs::File::open(p).map(|f| self.from_reader(f))
            }
        }
    }

    pub fn reader_file_stdin(&self) -> io::Result<csv::Reader<Box<dyn SeekRead + 'static>>> {
        Ok(match self.path {
            None => {
                // Create a buffer in memory when stdin needs to be indexed
                let mut buffer: Vec<u8> = Vec::new();
                let stdin = io::stdin();
                stdin.lock().read_to_end(&mut buffer)?;
                // check if its utf8-encoded
                if self.checkutf8 {
                    debug!("checking stdin encoding...");
                    let s = std::str::from_utf8(&buffer);
                    if s.is_err() {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            format!("<stdin> {UTF8_ERROR_MSG}"),
                        ));
                    }
                }
                self.from_reader(Box::new(io::Cursor::new(buffer)))
            }
            Some(ref p) => {
                if !self.is_utf8_encoded() {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("{p:?} {UTF8_ERROR_MSG}"),
                    ));
                }
                self.from_reader(Box::new(fs::File::open(p).unwrap()))
            }
        })
    }

    // qsv only works safely with utf8 encoded files
    // check first DEFAULT_UTF8_CHECK_BUFFER_LEN bytes
    // of file to quickly check if its utf8
    fn is_utf8_encoded(&self) -> bool {
        if !self.checkutf8 {
            return true;
        }
        if let Some(path_buf) = &self.path {
            debug!("checking encoding...");
            let mut f = fs::File::open(path_buf).unwrap();
            let fsize = f.metadata().unwrap().len() as usize;
            let mut buffer_size = DEFAULT_UTF8_CHECK_BUFFER_LEN;
            if fsize < buffer_size {
                buffer_size = fsize;
            }
            let mut buffer = vec![0; buffer_size];
            if f.read_exact(&mut buffer).is_ok() {
                let s = std::str::from_utf8(&buffer);
                return s.is_ok();
            }
        }
        false
    }

    fn autoindex_file(&self) {
        if let Some(path_buf) = &self.path {
            let path_clone = path_buf.clone();
            let path_str = path_clone.into_os_string().into_string().unwrap();
            let index_argv: Vec<&str> = vec!["", "index", &path_str];
            crate::cmd::index::run(&*index_argv).unwrap();
            debug!("autoindex for {path_str} created");
        }
    }

    pub fn index_files(&self) -> io::Result<Option<(csv::Reader<fs::File>, fs::File)>> {
        let (csv_file, idx_file) = match (&self.path, &self.idx_path) {
            (&None, &None) => return Ok(None),
            (&None, &Some(_)) => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Cannot use <stdin> with indexes",
                ));
            }
            (&Some(ref p), &None) => {
                // We generally don't want to report an error here, since we're
                // passively trying to find an index, so we just log the warning...
                let idx_file = match fs::File::open(&util::idx_path(p)) {
                    Err(e) => {
                        if self.autoindex {
                            self.autoindex_file();
                        } else {
                            warn!("No index file found - {p:?}: {e}");
                        }
                        return Ok(None);
                    }
                    Ok(f) => f,
                };
                (fs::File::open(p)?, idx_file)
            }
            (&Some(ref p), &Some(ref ip)) => (fs::File::open(p)?, fs::File::open(ip)?),
        };
        // If the CSV data was last modified after the index file was last
        // modified, then return an error and demand the user regenerate the
        // index.
        let data_modified = util::last_modified(&csv_file.metadata()?);
        let idx_modified = util::last_modified(&idx_file.metadata()?);
        if data_modified > idx_modified {
            if self.autoindex {
                debug!("index stale... autoindexing...");
                self.autoindex_file();
            } else {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    "The CSV file was modified after the index file. \
                 Please re-create the index.",
                ));
            }
        }
        let csv_rdr = self.from_reader(csv_file);
        Ok(Some((csv_rdr, idx_file)))
    }

    pub fn indexed(&self) -> CliResult<Option<Indexed<fs::File, fs::File>>> {
        match self.index_files()? {
            None => Ok(None),
            Some((r, i)) => Ok(Some(Indexed::open(r, i)?)),
        }
    }

    pub fn io_reader(&self) -> io::Result<Box<dyn io::Read + 'static>> {
        Ok(match self.path {
            None => {
                if self.checkutf8 {
                    let stdin_reader = io::stdin();
                    let mut buffer: Vec<u8> = Vec::new();
                    stdin_reader.lock().read_to_end(&mut buffer)?;
                    // check if its utf8-encoded
                    let s = std::str::from_utf8(&buffer);
                    if s.is_err() {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidData,
                            format!("<stdin> {UTF8_ERROR_MSG}"),
                        ));
                    }
                    Box::new(io::Cursor::new(buffer))
                } else {
                    Box::new(io::stdin())
                }
            }
            Some(ref p) => {
                if !self.is_utf8_encoded() {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("{p:?} {UTF8_ERROR_MSG}"),
                    ));
                }
                match fs::File::open(p) {
                    Ok(x) => Box::new(x),
                    Err(err) => {
                        let msg = format!("failed to open {}: {}", p.display(), err);
                        return Err(io::Error::new(io::ErrorKind::NotFound, msg));
                    }
                }
            }
        })
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn from_reader<R: Read>(&self, rdr: R) -> csv::Reader<R> {
        let rdr_capacitys = env::var("QSV_RDR_BUFFER_CAPACITY")
            .unwrap_or_else(|_| DEFAULT_RDR_BUFFER_CAPACITY.to_string());
        let rdr_buffer: usize = rdr_capacitys.parse().unwrap_or(DEFAULT_RDR_BUFFER_CAPACITY);

        let rdr_comment: Option<u8> = env::var("QSV_COMMENT_CHAR")
            .ok()
            .map(|s| s.as_bytes().first().unwrap().to_owned());

        csv::ReaderBuilder::new()
            .flexible(self.flexible)
            .delimiter(self.delimiter)
            .has_headers(!self.no_headers)
            .quote(self.quote)
            .quoting(self.quoting)
            .escape(self.escape)
            .buffer_capacity(rdr_buffer)
            .comment(rdr_comment)
            .trim(self.trim)
            .from_reader(rdr)
    }

    pub fn io_writer(&self) -> io::Result<Box<dyn io::Write + 'static>> {
        Ok(match self.path {
            None => Box::new(io::stdout()),
            Some(ref p) => Box::new(fs::File::create(p)?),
        })
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn from_writer<W: io::Write>(&self, wtr: W) -> csv::Writer<W> {
        let wtr_capacitys = env::var("QSV_WTR_BUFFER_CAPACITY")
            .unwrap_or_else(|_| DEFAULT_WTR_BUFFER_CAPACITY.to_string());
        let wtr_buffer: usize = wtr_capacitys.parse().unwrap_or(DEFAULT_WTR_BUFFER_CAPACITY);

        csv::WriterBuilder::new()
            .flexible(self.flexible)
            .delimiter(self.delimiter)
            .terminator(self.terminator)
            .quote(self.quote)
            .quote_style(self.quote_style)
            .double_quote(self.double_quote)
            .escape(self.escape.unwrap_or(b'\\'))
            .buffer_capacity(wtr_buffer)
            .from_writer(wtr)
    }
}
