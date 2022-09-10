use crate::config::{Config, Delimiter};
use crate::CliResult;
use docopt::Docopt;
#[cfg(any(feature = "full", feature = "lite"))]
use indicatif::{HumanCount, ProgressBar, ProgressStyle};
#[cfg(any(feature = "apply", feature = "fetch", feature = "python"))]
use regex::Regex;
use serde::de::DeserializeOwned;
#[cfg(any(feature = "full", feature = "lite"))]
use serde::de::{Deserialize, Deserializer, Error};
#[cfg(any(feature = "full", feature = "lite"))]
use std::borrow::Cow;
use std::path::{Path, PathBuf};
use std::{env, fs, io, str, thread};

#[macro_export]
macro_rules! regex_once_cell {
    ($re:literal $(,)?) => {{
        static RE: once_cell::sync::OnceCell<regex::Regex> = once_cell::sync::OnceCell::new();
        RE.get_or_init(|| regex::Regex::new($re).unwrap())
    }};
}

#[inline]
pub fn num_cpus() -> usize {
    thread::available_parallelism().unwrap().get()
}

pub static DEFAULT_USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    " (https://github.com/jqnatividad/qsv)",
);
const TARGET: &str = match option_env!("TARGET") {
    Some(target) => target,
    None => "Unknown_target",
};

pub fn max_jobs() -> usize {
    let num_cpus = num_cpus();
    let max_jobs = match env::var("QSV_MAX_JOBS") {
        Ok(val) => val.parse::<usize>().unwrap_or(1_usize),
        Err(_) => num_cpus,
    };
    if (1..=num_cpus).contains(&max_jobs) {
        max_jobs
    } else {
        num_cpus
    }
}

pub fn njobs(flag_jobs: Option<usize>) -> usize {
    let max_jobs = max_jobs();
    flag_jobs.map_or(max_jobs, |jobs| {
        if jobs == 0 || jobs > max_jobs {
            env::set_var("RAYON_NUM_THREADS", max_jobs.to_string());
            log::info!("Using {max_jobs} max processors...");
            max_jobs
        } else {
            env::set_var("RAYON_NUM_THREADS", jobs.to_string());
            log::info!("Throttling to {max_jobs} processors...");
            jobs
        }
    })
}

pub fn version() -> String {
    let mut enabled_features = String::new();

    #[cfg(all(feature = "apply", not(feature = "lite")))]
    enabled_features.push_str("apply;");
    #[cfg(all(feature = "fetch", not(feature = "lite")))]
    enabled_features.push_str("fetch;");
    #[cfg(all(feature = "foreach", not(feature = "lite")))]
    enabled_features.push_str("foreach;");
    #[cfg(all(feature = "generate", not(feature = "lite")))]
    enabled_features.push_str("generate;");
    #[cfg(all(feature = "lua", not(feature = "lite")))]
    enabled_features.push_str("lua;");
    #[cfg(all(feature = "python", not(feature = "lite")))]
    {
        enabled_features.push_str("python-");
        pyo3::Python::with_gil(|py| {
            enabled_features.push_str(py.version());
            enabled_features.push(';');
        });
    }
    #[cfg(feature = "self_update")]
    enabled_features.push_str("self_update");
    enabled_features.push('-');

    #[cfg(feature = "mimalloc")]
    let malloc_kind = "mimalloc".to_string();
    #[cfg(not(feature = "mimalloc"))]
    let malloc_kind = "standard".to_string();
    let (qsvtype, maj, min, pat, pre, rustversion) = (
        option_env!("CARGO_BIN_NAME"),
        option_env!("CARGO_PKG_VERSION_MAJOR"),
        option_env!("CARGO_PKG_VERSION_MINOR"),
        option_env!("CARGO_PKG_VERSION_PATCH"),
        option_env!("CARGO_PKG_VERSION_PRE"),
        option_env!("CARGO_PKG_RUST_VERSION"),
    );
    match (qsvtype, maj, min, pat, pre, rustversion) {
        (Some(qsvtype), Some(maj), Some(min), Some(pat), Some(pre), Some(rustversion)) => {
            if pre.is_empty() {
                format!(
                    "{qsvtype} {maj}.{min}.{pat}-{malloc_kind}-{enabled_features}{maxjobs}-{numcpus} ({TARGET} compiled with Rust {rustversion})",
                    maxjobs = max_jobs(),
                    numcpus = num_cpus()
                )
            } else {
                format!(
                    "{qsvtype} {maj}.{min}.{pat}-{pre}-{malloc_kind}-{enabled_features}{maxjobs}-{numcpus} ({TARGET} compiled with Rust {rustversion})",
                    maxjobs = max_jobs(),
                    numcpus = num_cpus(),
                )
            }
        }
        _ => String::new(),
    }
}

const OTHER_ENV_VARS: &[&str] = &["no_proxy", "http_proxy", "https_proxy"];

pub fn show_env_vars() {
    let mut env_var_set = false;
    for (n, v) in env::vars_os() {
        let env_var = n.into_string().unwrap();
        if env_var.starts_with("QSV_")
            || env_var.starts_with("MIMALLOC_")
            || OTHER_ENV_VARS.contains(&env_var.to_lowercase().as_str())
        {
            env_var_set = true;
            println!("{env_var}: {v:?}");
        }
    }
    if !env_var_set {
        println!("No qsv-relevant environment variables set.");
    }
}

#[inline]
pub fn count_rows(conf: &Config) -> Result<u64, io::Error> {
    if let Some(idx) = conf.indexed().unwrap_or(None) {
        Ok(idx.count())
    } else {
        // index does not exist or is stale,
        // count records manually
        let mut rdr = conf.reader()?;
        let mut count = 0u64;
        let mut record = csv::ByteRecord::new();
        while rdr.read_byte_record(&mut record)? {
            count += 1;
        }
        Ok(count)
    }
}

#[cfg(any(feature = "full", feature = "lite"))]
pub fn prep_progress(progress: &ProgressBar, record_count: u64) {
    progress.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] [{bar:25} {percent}%{msg}] ({per_sec} - {eta})")
            .unwrap(),
    );
    progress.set_message(format!(" of {} records", HumanCount(record_count)));

    // draw progress bar for the first time using specified style
    progress.set_length(record_count);

    log::info!("Progress started... {record_count} records");
}

#[cfg(any(feature = "full", feature = "lite"))]
pub fn finish_progress(progress: &ProgressBar) {
    progress.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] [{bar:25} {percent}%{msg}] ({per_sec})")
            .unwrap(),
    );

    if progress.length().unwrap() == progress.position() {
        progress.finish();
        log::info!("Progress done... {}", progress.message());
    } else {
        progress.abandon();
        log::info!("Progress abandoned... {}", progress.message());
    }
}

#[cfg(all(any(feature = "apply", feature = "fetch"), not(feature = "lite")))]
macro_rules! update_cache_info {
    ($progress:expr, $cache_instance:expr) => {
        use cached::Cached;
        use indicatif::HumanCount;

        let cache_instance = $cache_instance.lock();
        match cache_instance {
            Ok(cache) => {
                let cache_size = cache.cache_size();
                if cache_size > 0 {
                    let hits = cache.cache_hits().expect("Cache hits required");
                    let misses = cache.cache_misses().expect("Cache misses required");
                    let hit_ratio = (hits as f64 / (hits + misses) as f64) * 100.0;
                    $progress.set_message(format!(
                        " of {} records. Cache hit ratio: {hit_ratio:.2}%",
                        HumanCount($progress.length().unwrap()),
                    ));
                }
            }
            _ => {}
        }
    };
    ($progress:expr, $cache_hits:expr, $num_rows:expr) => {
        use indicatif::HumanCount;

        let hit_ratio = ($cache_hits as f64 / $num_rows as f64) * 100.0;
        $progress.set_message(format!(
            " of {} records. Redis cache hit ratio: {hit_ratio:.2}%",
            HumanCount($progress.length().unwrap()),
        ));
    };
}

#[cfg(all(any(feature = "apply", feature = "fetch"), not(feature = "lite")))]
pub(crate) use update_cache_info;

pub fn get_args<T>(usage: &str, argv: &[&str]) -> CliResult<T>
where
    T: DeserializeOwned,
{
    Docopt::new(usage)
        .and_then(|d| {
            d.argv(argv.iter().copied())
                .version(Some(version()))
                .deserialize()
        })
        .map_err(From::from)
}

pub fn many_configs(
    inps: &[String],
    delim: Option<Delimiter>,
    no_headers: bool,
) -> Result<Vec<Config>, String> {
    let mut inps = inps.to_vec();
    if inps.is_empty() {
        inps.push("-".to_owned()); // stdin
    }
    let confs = inps
        .into_iter()
        .map(|p| {
            Config::new(&Some(p))
                .delimiter(delim)
                .no_headers(no_headers)
                .checkutf8(false)
        })
        .collect::<Vec<_>>();
    errif_greater_one_stdin(&confs)?;
    Ok(confs)
}

pub fn errif_greater_one_stdin(inps: &[Config]) -> Result<(), String> {
    let nstd = inps.iter().filter(|inp| inp.is_stdin()).count();
    if nstd > 1 {
        return Err("At most one <stdin> input is allowed.".to_owned());
    }
    Ok(())
}

pub const fn chunk_size(nitems: usize, njobs: usize) -> usize {
    if nitems < njobs {
        nitems
    } else {
        nitems / njobs
    }
}

pub const fn num_of_chunks(nitems: usize, chunk_size: usize) -> usize {
    if chunk_size == 0 {
        return nitems;
    }
    let mut n = nitems / chunk_size;
    if nitems % chunk_size != 0 {
        n += 1;
    }
    n
}

#[allow(clippy::cast_sign_loss)]
pub fn last_modified(md: &fs::Metadata) -> u64 {
    use filetime::FileTime;
    FileTime::from_last_modification_time(md).unix_seconds() as u64
}

#[cfg(any(feature = "full", feature = "lite"))]
pub fn condense(val: Cow<[u8]>, n: Option<usize>) -> Cow<[u8]> {
    match n {
        None => val,
        Some(n) => {
            let mut is_short_utf8 = false;
            if let Ok(s) = str::from_utf8(&val) {
                if n >= s.chars().count() {
                    is_short_utf8 = true;
                } else {
                    let mut s = s.chars().take(n).collect::<String>();
                    s.push_str("...");
                    return Cow::Owned(s.into_bytes());
                }
            }
            if is_short_utf8 || n >= (*val).len() {
                // already short enough
                val
            } else {
                // This is a non-Unicode string, so we just trim on bytes.
                let mut s = val[0..n].to_vec();
                s.extend(b"...".iter().copied());
                Cow::Owned(s)
            }
        }
    }
}

pub fn idx_path(csv_path: &Path) -> PathBuf {
    let mut p = csv_path
        .to_path_buf()
        .into_os_string()
        .into_string()
        .unwrap();
    p.push_str(".idx");
    PathBuf::from(&p)
}

pub type Idx = Option<usize>;

pub fn range(start: Idx, end: Idx, len: Idx, index: Idx) -> Result<(usize, usize), String> {
    match (start, end, len, index) {
        (None, None, None, Some(i)) => Ok((i, i + 1)),
        (_, _, _, Some(_)) => Err("--index cannot be used with --start, --end or --len".to_owned()),
        (_, Some(_), Some(_), None) => {
            Err("--end and --len cannot be used at the same time.".to_owned())
        }
        (_, None, None, None) => Ok((start.unwrap_or(0), ::std::usize::MAX)),
        (_, Some(e), None, None) => {
            let s = start.unwrap_or(0);
            if s > e {
                Err(format!(
                    "The end of the range ({e}) must be greater than or\n\
                             equal to the start of the range ({s})."
                ))
            } else {
                Ok((s, e))
            }
        }
        (_, None, Some(l), None) => {
            let s = start.unwrap_or(0);
            Ok((s, s + l))
        }
    }
}

/// Create a directory recursively, avoiding the race conditons fixed by
/// https://github.com/rust-lang/rust/pull/39799.
#[cfg(any(feature = "full", feature = "lite"))]
fn create_dir_all_threadsafe(path: &Path) -> io::Result<()> {
    // Try 20 times. This shouldn't theoretically need to be any larger
    // than the number of nested directories we need to create.
    for _ in 0..20 {
        match fs::create_dir_all(path) {
            // This happens if a directory in `path` doesn't exist when we
            // test for it, and another thread creates it before we can.
            Err(ref err) if err.kind() == io::ErrorKind::AlreadyExists => {}
            other => return other,
        }
        // We probably don't need to sleep at all, because the intermediate
        // directory is already created.  But let's attempt to back off a
        // bit and let the other thread finish.
        thread::sleep(std::time::Duration::from_millis(25));
    }
    // Try one last time, returning whatever happens.
    fs::create_dir_all(path)
}

/// Represents a filename template of the form `"{}.csv"`, where `"{}"` is
/// the splace to insert the part of the filename generated by `qsv`.
#[cfg(any(feature = "full", feature = "lite"))]
#[derive(Clone, Debug)]
pub struct FilenameTemplate {
    prefix: String,
    suffix: String,
}

#[cfg(any(feature = "full", feature = "lite"))]
impl FilenameTemplate {
    /// Generate a new filename using `unique_value` to replace the `"{}"`
    /// in the template.
    pub fn filename(&self, unique_value: &str) -> String {
        format!("{}{unique_value}{}", &self.prefix, &self.suffix)
    }

    /// Create a new, writable file in directory `path` with a filename
    /// using `unique_value` to replace the `"{}"` in the template.  Note
    /// that we do not output headers; the caller must do that if
    /// desired.
    pub fn writer<P>(
        &self,
        path: P,
        unique_value: &str,
    ) -> io::Result<csv::Writer<Box<dyn io::Write + 'static>>>
    where
        P: AsRef<Path>,
    {
        let filename = self.filename(unique_value);
        let full_path = path.as_ref().join(filename);
        if let Some(parent) = full_path.parent() {
            // We may be called concurrently, especially by parallel `qsv
            // split`, so be careful to avoid the `create_dir_all` race
            // condition.
            create_dir_all_threadsafe(parent)?;
        }
        let spath = Some(full_path.display().to_string());
        Config::new(&spath).writer()
    }
}

#[cfg(any(feature = "full", feature = "lite"))]
impl<'de> Deserialize<'de> for FilenameTemplate {
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<FilenameTemplate, D::Error> {
        let raw = String::deserialize(d)?;
        let chunks = raw.split("{}").collect::<Vec<_>>();
        if chunks.len() == 2 {
            Ok(FilenameTemplate {
                prefix: chunks[0].to_owned(),
                suffix: chunks[1].to_owned(),
            })
        } else {
            Err(D::Error::custom(
                "The --filename argument must contain one '{}'.",
            ))
        }
    }
}

pub fn init_logger() -> String {
    use flexi_logger::{Cleanup, Criterion, FileSpec, Logger, Naming};

    let qsv_log_env = env::var("QSV_LOG_LEVEL").unwrap_or_else(|_| "off".to_string());
    let qsv_log_dir = env::var("QSV_LOG_DIR").unwrap_or_else(|_| ".".to_string());

    Logger::try_with_env_or_str(qsv_log_env)
        .unwrap()
        .use_utc()
        .log_to_file(
            FileSpec::default()
                .directory(qsv_log_dir)
                .suppress_timestamp(),
        )
        .format_for_files(flexi_logger::detailed_format)
        .o_append(true)
        .rotate(
            Criterion::Size(20_000_000), // 20 mb
            Naming::Numbers,
            Cleanup::KeepLogAndCompressedFiles(10, 100),
        )
        .start()
        .unwrap();

    let qsv_args: String = if log::log_enabled!(log::Level::Info) {
        env::args().skip(1).collect::<Vec<_>>().join(" ")
    } else {
        String::new()
    };
    log::info!("START: {qsv_args}");
    qsv_args
}

#[cfg(all(any(feature = "full", feature = "lite"), feature = "self_update"))]
pub fn qsv_check_for_update() -> Result<bool, String> {
    if env::var("QSV_NO_UPDATE").is_ok() {
        return Ok(false);
    }

    let bin_name = std::env::current_exe()
        .expect("Can't get the exec path")
        .file_stem()
        .expect("Can't get the exec stem name")
        .to_string_lossy()
        .into_owned();

    winfo!("Checking GitHub for updates...");

    use self_update::cargo_crate_version;
    const GITHUB_RATELIMIT_MSG: &str =
        "Github is rate-limiting self-update checks at the moment. Try again in an hour.";

    let curr_version = cargo_crate_version!();
    let releases = if let Ok(releases_list) =
        self_update::backends::github::ReleaseList::configure()
            .repo_owner("jqnatividad")
            .repo_name("qsv")
            .build()
    {
        if let Ok(releases) = releases_list.fetch() {
            releases
        } else {
            return Err(GITHUB_RATELIMIT_MSG.to_string());
        }
    } else {
        return Err(GITHUB_RATELIMIT_MSG.to_string());
    };
    let latest_release = &releases[0].version;

    log::info!("Current version: {curr_version} Latest Release: {latest_release}");

    let mut updated = false;
    if latest_release > &curr_version.to_string() {
        eprintln!("Update {latest_release} available. Current version is {curr_version}.");
        eprintln!("Release notes: https://github.com/jqnatividad/qsv/releases/latest\n");
        match self_update::backends::github::Update::configure()
            .repo_owner("jqnatividad")
            .repo_name("qsv")
            .bin_name(&bin_name)
            .show_download_progress(true)
            .show_output(false)
            .no_confirm(false)
            .current_version(curr_version)
            .build()
        {
            Ok(update_job) => match update_job.update() {
                Ok(status) => {
                    updated = true;
                    let update_status = format!(
                        "Update successful for {}: `{}`!",
                        bin_name,
                        status.version()
                    );
                    winfo!("{update_status}");
                }
                Err(e) => {
                    eprintln!("Update job error: {e}");
                    log::error!("Update job error: {e}");
                }
            },
            Err(e) => {
                eprintln!("Update builder error: {e}");
                log::error!("Update builder error: {e}");
            }
        };
    } else {
        winfo!("Up to date ({curr_version})... no update required.");
    };

    let _temp = send_hwsurvey(&bin_name, updated, latest_release, curr_version, false);

    Ok(updated)
}

#[cfg(all(any(feature = "full", feature = "lite"), not(feature = "self_update")))]
pub fn qsv_check_for_update() -> Result<bool, String> {
    Ok(true)
}

// the qsv hwsurvey allows us to keep a better
// track of qsv's usage in the wild, so we can do a
// better job of prioritizing platforms/features we support
// no personally identifiable information is collected
#[cfg(all(any(feature = "full", feature = "lite"), feature = "self_update"))]
fn send_hwsurvey(
    bin_name: &str,
    updated: bool,
    latest_release: &str,
    curr_version: &str,
    dry_run: bool,
) -> Result<reqwest::StatusCode, String> {
    use serde_json::json;
    use sysinfo::{CpuExt, System, SystemExt};

    const QSV_KIND: &str = match option_env!("QSV_KIND") {
        Some(kind) => kind,
        None => "installed",
    };
    static HW_SURVEY_URL: &str =
        "https://4dhmneehnl.execute-api.us-east-1.amazonaws.com/dev/qsv-hwsurvey";

    let mut sys = System::new_all();
    sys.refresh_all();
    let total_mem = sys.total_memory();
    let kernel_version = sys
        .kernel_version()
        .unwrap_or_else(|| "Unknown kernel".to_string());
    let long_os_verion = sys
        .long_os_version()
        .unwrap_or_else(|| "Unknown OS version".to_string());
    let cpu_count = sys.cpus().len();
    let physical_cpu_count = sys.physical_core_count().unwrap_or_default();
    let cpu_vendor_id = sys.cpus()[0].vendor_id();
    let cpu_brand = sys.cpus()[0].brand().trim();
    let cpu_freq = sys.cpus()[0].frequency();
    let long_id: u128 = std::time::SystemTime::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();
    // the id doubles as a timestamp
    // we first get number of milliseconds since UNIX EPOCH
    // and then cast to u64 as serde_json cannot serialize u128
    let id: u64 = long_id.try_into().unwrap_or_default();
    let hwsurvey_json = json!(
        {
            "id": id,
            "variant": bin_name,
            "kind": QSV_KIND,
            "ver": if updated { latest_release } else { curr_version },
            "updated": updated,
            "prev_ver": curr_version,
            "cpu_phy_cores": physical_cpu_count,
            "cpu_log_cores": cpu_count,
            "cpu_vendor": cpu_vendor_id,
            "cpu_brand": cpu_brand,
            "cpu_freq": cpu_freq,
            "mem": total_mem,
            "kernel": kernel_version,
            "os": long_os_verion,
            "target": TARGET,
        }
    );
    log::debug!("hwsurvey: {hwsurvey_json}");

    let mut survey_done = true;
    let mut status = reqwest::StatusCode::OK;
    if !dry_run {
        let client = reqwest::blocking::Client::builder()
            .user_agent(DEFAULT_USER_AGENT)
            .brotli(true)
            .gzip(true)
            .deflate(true)
            .http2_adaptive_window(true)
            .build()
            .expect("Cannot build hw_survey reqwest client");

        match client
            .post(HW_SURVEY_URL)
            .body(hwsurvey_json.to_string())
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .header(reqwest::header::HOST, "qsv.rs")
            .send()
        {
            Ok(resp) => {
                log::debug!("hw_survey response sent: {:?}", &resp);
                status = resp.status();
                survey_done = status.is_success();
            }
            Err(e) => {
                log::warn!("Cannot send hw survey: {e}");
                survey_done = false;
                status = reqwest::StatusCode::BAD_REQUEST;
            }
        };
    }
    if survey_done {
        Ok(status)
    } else {
        Err("Cannot send hw survey".to_string())
    }
}

#[cfg(any(feature = "apply", feature = "fetch", feature = "python"))]
pub fn safe_header_names(headers: &csv::StringRecord, check_first_char: bool) -> Vec<String> {
    // Create "safe" var/key names - to support dynfmt/url-template and valid python vars
    // Replace whitespace/invalid chars with _.
    // If name starts with a number, replace it with an _ as well (for python vars)
    let re = Regex::new(r"[^A-Za-z0-9]").unwrap();
    let mut name_vec: Vec<String> = Vec::with_capacity(headers.len());
    for h in headers {
        let mut safe_name = re.replace_all(h, "_").to_string();
        if check_first_char && safe_name.as_bytes()[0].is_ascii_digit() {
            safe_name.replace_range(0..1, "_");
        }
        name_vec.push(safe_name);
    }
    log::debug!("safe header names: {name_vec:?}");
    name_vec
}

pub fn log_end(mut qsv_args: String, now: std::time::Instant) {
    if log::log_enabled!(log::Level::Info) {
        let ellipsis = if qsv_args.len() > 24 {
            qsv_args.truncate(24);
            "..."
        } else {
            ""
        };
        log::info!(
            "END \"{qsv_args}{ellipsis}\" elapsed: {}",
            now.elapsed().as_secs_f32()
        );
    }
}

#[test]
#[cfg(feature = "self_update")]
fn test_hw_survey() {
    // we have this test primarily to exercise the sysinfo module
    assert!(send_hwsurvey("qsv", false, "0.0.2", "0.0.1", true).is_ok());
}

#[cfg(feature = "apply")]
pub struct ColumnNameParser {
    chars: Vec<char>,
    pos: usize,
}

#[cfg(feature = "apply")]
impl ColumnNameParser {
    pub fn new(s: &str) -> ColumnNameParser {
        ColumnNameParser {
            chars: s.chars().collect(),
            pos: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Vec<String>, String> {
        let mut new_cols_name = vec![];
        loop {
            if self.cur().is_none() {
                break;
            }
            if self.cur() == Some('"') {
                self.bump();
                new_cols_name.push(self.parse_quoted_name()?);
            } else {
                new_cols_name.push(self.parse_name());
            }
            self.bump();
        }
        Ok(new_cols_name)
    }

    fn cur(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    fn bump(&mut self) {
        if self.pos < self.chars.len() {
            self.pos += 1;
        }
    }

    fn is_end_of_field(&self) -> bool {
        self.cur().map_or(true, |c| c == ',')
    }

    fn parse_quoted_name(&mut self) -> Result<String, String> {
        let mut name = String::new();
        loop {
            match self.cur() {
                None => {
                    return Err("Unclose quote, missing \".".to_owned());
                }
                Some('"') => {
                    self.bump();
                    if self.cur() == Some('"') {
                        self.bump();
                        name.push('"');
                        name.push('"');
                        continue;
                    }
                    break;
                }
                Some(c) => {
                    name.push(c);
                    self.bump();
                }
            }
        }
        Ok(name)
    }

    fn parse_name(&mut self) -> String {
        let mut name = String::new();
        loop {
            if self.is_end_of_field() {
                break;
            }
            name.push(self.cur().unwrap());
            self.bump();
        }
        name
    }
}
