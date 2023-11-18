#[cfg(any(feature = "feature_capable", feature = "lite"))]
use std::borrow::Cow;
use std::{
    cmp::min,
    env, fs,
    fs::File,
    io::{BufReader, Read, Write},
    path::{Path, PathBuf},
    str,
    sync::OnceLock,
};

use docopt::Docopt;
#[cfg(any(feature = "feature_capable", feature = "lite"))]
use indicatif::{HumanCount, ProgressBar, ProgressDrawTarget, ProgressStyle};
use log::log_enabled;
use reqwest::Client;
use serde::de::DeserializeOwned;
#[cfg(any(feature = "feature_capable", feature = "lite"))]
use serde::de::{Deserialize, Deserializer, Error};
use sysinfo::{System, SystemExt};

use crate::{
    config,
    config::{Config, Delimiter},
    CliError, CliResult, CURRENT_COMMAND,
};

#[macro_export]
macro_rules! regex_oncelock {
    ($re:literal $(,)?) => {{
        static RE: std::sync::OnceLock<regex::Regex> = std::sync::OnceLock::new();
        RE.get_or_init(|| regex::Regex::new($re).unwrap())
    }};
}

// leave at least 20% of the available memory free
const DEFAULT_FREEMEMORY_HEADROOM_PCT: u8 = 20;

static ROW_COUNT: OnceLock<u64> = OnceLock::new();

pub type ByteString = Vec<u8>;

#[inline]
pub fn num_cpus() -> usize {
    num_cpus::get()
}

const CARGO_BIN_NAME: &str = env!("CARGO_BIN_NAME");
const CARGO_PKG_VERSION: &str = env!("CARGO_PKG_VERSION");

const TARGET: &str = match option_env!("TARGET") {
    Some(target) => target,
    None => "Unknown_target",
};
const QSV_KIND: &str = match option_env!("QSV_KIND") {
    Some(kind) => kind,
    None => "installed",
};

fn default_user_agent() -> String {
    let unknown_command = "Unknown".to_string();
    let current_command = CURRENT_COMMAND.get().unwrap_or(&unknown_command);
    format!("{CARGO_BIN_NAME}/{CARGO_PKG_VERSION} ({TARGET}; {current_command}; {QSV_KIND}; https://github.com/jqnatividad/qsv)")
}

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

pub fn timeout_secs(timeout: u16) -> Result<u64, String> {
    let timeout = match env::var("QSV_TIMEOUT") {
        Ok(val) => val.parse::<u16>().unwrap_or(30_u16),
        Err(_) => timeout,
    };

    if timeout > 3600 {
        return fail_format!("Timeout cannot be more than 3,600 seconds (1 hour): {timeout}");
    } else if timeout == 0 {
        return fail!("Timeout cannot be zero.");
    }
    log::info!("TIMEOUT: {timeout}");
    Ok(timeout as u64)
}

/// sets custom user agent
/// if user agent is not set, then use the default user agent
/// it supports four special LITERALs: $QSV_BIN_NAME, $QSV_VERSION, $QSV_TARGET, $QSV_KIND
/// and $QSV_COMMAND which will be replaced with the actual values during runtime
pub fn set_user_agent(user_agent: Option<String>) -> CliResult<String> {
    use reqwest::header::HeaderValue;

    let ua = match user_agent {
        Some(ua_arg) => ua_arg,
        None => env::var("QSV_USER_AGENT").unwrap_or_else(|_| default_user_agent()),
    };

    let unknown_command = "Unknown".to_string();
    let current_command = CURRENT_COMMAND.get().unwrap_or(&unknown_command);

    // look for special literals - $QSV_VERSION and $QSV_TARGET and replace them
    let ua = ua
        .replace("$QSV_BIN_NAME", CARGO_BIN_NAME)
        .replace("$QSV_VERSION", CARGO_PKG_VERSION)
        .replace("$QSV_TARGET", TARGET)
        .replace("$QSV_KIND", QSV_KIND)
        .replace("$QSV_COMMAND", current_command);

    match HeaderValue::from_str(ua.as_str()) {
        Ok(_) => (),
        Err(e) => return fail_incorrectusage_clierror!("Invalid user-agent value: {e}"),
    };

    log::info!("set user agent: {ua}");
    Ok(ua)
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
    #[cfg(all(feature = "geocode", not(feature = "lite")))]
    enabled_features.push_str("geocode;");

    #[cfg(all(feature = "luau", not(feature = "lite")))]
    {
        let luau = mlua::Lua::new();
        match luau.load("return _VERSION").eval() {
            Ok(version_info) => {
                if let mlua::Value::String(luaustring_val) = version_info {
                    let string_val = luaustring_val.to_str().unwrap_or("Luau - invalid version");
                    if string_val == "Luau" {
                        enabled_features.push_str("Luau - version not specified;");
                    } else {
                        enabled_features.push_str(&format!("{string_val};"));
                    }
                } else {
                    enabled_features.push_str("Luau - ?;");
                }
            },
            Err(e) => enabled_features.push_str(&format!("Luau - cannot retrieve version: {e};")),
        };
    }

    #[cfg(all(feature = "python", not(feature = "lite")))]
    {
        enabled_features.push_str("python-");
        pyo3::Python::with_gil(|py| {
            enabled_features.push_str(py.version());
            enabled_features.push(';');
        });
    }
    #[cfg(all(feature = "to", not(feature = "lite")))]
    enabled_features.push_str("to;");
    #[cfg(all(feature = "polars", not(feature = "lite")))]
    enabled_features.push_str(format!("polars-{};", polars::VERSION).as_str());
    #[cfg(feature = "self_update")]
    enabled_features.push_str("self_update");
    enabled_features.push('-');

    // get max_file_size & memory info. max_file_size is based on QSV_FREEMEMORY_HEADROOM_PCT
    // setting and is only enforced when qsv is running in "non-streaming" mode (i.e. needs to
    // load the entire file into memory).
    let mut sys = System::new();
    sys.refresh_memory();
    let avail_mem = sys.available_memory();
    let total_mem = sys.total_memory();
    let free_swap = sys.free_swap();
    let max_file_size = mem_file_check(Path::new(""), true, false).unwrap_or(0) as u64;

    #[cfg(feature = "mimalloc")]
    let malloc_kind = "mimalloc";
    #[cfg(feature = "jemallocator")]
    let malloc_kind = "jemalloc";
    #[cfg(not(any(feature = "mimalloc", feature = "jemallocator")))]
    let malloc_kind = "standard";
    let (qsvtype, maj, min, pat, pre, rustversion) = (
        option_env!("CARGO_BIN_NAME"),
        option_env!("CARGO_PKG_VERSION_MAJOR"),
        option_env!("CARGO_PKG_VERSION_MINOR"),
        option_env!("CARGO_PKG_VERSION_PATCH"),
        option_env!("CARGO_PKG_VERSION_PRE"),
        option_env!("CARGO_PKG_RUST_VERSION"),
    );
    if let (Some(qsvtype), Some(maj), Some(min), Some(pat), Some(pre), Some(rustversion)) =
        (qsvtype, maj, min, pat, pre, rustversion)
    {
        if pre.is_empty() {
            format!(
                "{qsvtype} {maj}.{min}.{pat}-{malloc_kind}-{enabled_features}{maxjobs}-{numcpus};\
                 {max_file_size}-{free_swap}-{avail_mem}-{total_mem} ({TARGET} compiled with Rust \
                 {rustversion}) {QSV_KIND}",
                maxjobs = max_jobs(),
                numcpus = num_cpus(),
                max_file_size = indicatif::HumanBytes(max_file_size),
                free_swap = indicatif::HumanBytes(free_swap),
                avail_mem = indicatif::HumanBytes(avail_mem),
                total_mem = indicatif::HumanBytes(total_mem),
            )
        } else {
            format!(
                "{qsvtype} {maj}.{min}.\
                 {pat}-{pre}-{malloc_kind}-{enabled_features}{maxjobs}-{numcpus};\
                 {max_file_size}-{free_swap}-{avail_mem}-{total_mem} ({TARGET} compiled with Rust \
                 {rustversion}) {QSV_KIND}",
                maxjobs = max_jobs(),
                numcpus = num_cpus(),
                max_file_size = indicatif::HumanBytes(max_file_size),
                free_swap = indicatif::HumanBytes(free_swap),
                avail_mem = indicatif::HumanBytes(avail_mem),
                total_mem = indicatif::HumanBytes(total_mem),
            )
        }
    } else {
        String::new()
    }
}

const OTHER_ENV_VARS: &[&str] = &["all_proxy", "no_proxy", "http_proxy", "https_proxy"];

pub fn show_env_vars() {
    let mut env_var_set = false;
    for (n, v) in env::vars_os() {
        let env_var = n.into_string().unwrap();
        #[cfg(feature = "mimalloc")]
        if env_var.starts_with("QSV_")
            || env_var.starts_with("MIMALLOC_")
            || OTHER_ENV_VARS.contains(&env_var.to_lowercase().as_str())
        {
            env_var_set = true;
            woutinfo!("{env_var}: {v:?}");
        }
        #[cfg(feature = "jemallocator")]
        if env_var.starts_with("QSV_")
            || env_var.starts_with("JEMALLOC_")
            || env_var.starts_with("MALLOC_CONF")
            || OTHER_ENV_VARS.contains(&env_var.to_lowercase().as_str())
        {
            env_var_set = true;
            woutinfo!("{env_var}: {v:?}");
        }
        #[cfg(not(any(feature = "mimalloc", feature = "jemallocator")))]
        if env_var.starts_with("QSV_") || OTHER_ENV_VARS.contains(&env_var.to_lowercase().as_str())
        {
            env_var_set = true;
            woutinfo!("{env_var}: {v:?}");
        }
    }
    if !env_var_set {
        woutinfo!("No qsv-relevant environment variables set.");
    }
}

#[inline]
pub fn count_rows(conf: &Config) -> Result<u64, CliError> {
    if let Some(idx) = conf.indexed().unwrap_or(None) {
        Ok(idx.count())
    } else {
        // index does not exist or is stale,
        // count records by iterating through records
        // Do this only once per invocation and cache the result, so we don't
        // have to re-count rows every time we need to know the row count for CSVs
        // that don't have an index.
        let rc = ROW_COUNT.get_or_init(|| {
            if let Ok(mut rdr) = conf.reader() {
                let mut count = 0_u64;
                let mut record = csv::ByteRecord::new();
                while rdr.read_byte_record(&mut record).unwrap_or_default() {
                    count += 1;
                }
                count
            } else {
                // sentinel value to indicate that we were unable to count rows
                u64::MAX
            }
        });

        if *rc < u64::MAX {
            Ok(*rc)
        } else {
            Err(CliError::Other("Unable to get row count".to_string()))
        }
    }
}

#[cfg(any(feature = "feature_capable", feature = "lite"))]
pub fn prep_progress(progress: &ProgressBar, record_count: u64) {
    progress.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] [{wide_bar} {percent}%{msg}] ({per_sec} - {eta})")
            .unwrap(),
    );
    progress.set_message(format!(" of {} records", HumanCount(record_count)));

    // draw progress bar for the first time using specified style
    progress.set_length(record_count);

    log::info!("Progress started... {record_count} records");
}

#[cfg(any(feature = "feature_capable", feature = "lite"))]
pub fn finish_progress(progress: &ProgressBar) {
    progress.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] [{wide_bar} {percent}%{msg}] ({per_sec})")
            .unwrap(),
    );

    if progress.length().unwrap_or_default() == progress.position() {
        progress.finish();
        log::info!("Progress done... {}", progress.message());
    } else {
        progress.abandon();
        log::info!("Progress abandoned... {}", progress.message());
    }
}

#[cfg(all(
    any(feature = "apply", feature = "fetch", feature = "geocode"),
    not(feature = "lite")
))]
macro_rules! update_cache_info {
    ($progress:expr, $cache_instance:expr) => {
        use cached::Cached;
        use indicatif::HumanCount;

        match $cache_instance.lock() {
            Ok(cache) => {
                let size = cache.cache_size();
                if size > 0 {
                    let hits = cache.cache_hits().unwrap_or_default();
                    let misses = cache.cache_misses().unwrap_or(1);
                    let hit_ratio = (hits as f64 / (hits + misses) as f64) * 100.0;
                    let capacity = cache.cache_capacity();
                    $progress.set_message(format!(
                        " of {} records. Cache {:.2}% entries: {} capacity: {}.",
                        HumanCount($progress.length().unwrap()),
                        hit_ratio,
                        HumanCount(size as u64),
                        HumanCount(capacity.unwrap() as u64),
                    ));
                }
            },
            _ => {},
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

#[cfg(all(
    any(feature = "apply", feature = "fetch", feature = "geocode"),
    not(feature = "lite")
))]
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

#[inline]
pub fn many_configs(
    inps: &[String],
    delim: Option<Delimiter>,
    no_headers: bool,
    flexible: bool,
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
                .flexible(flexible)
        })
        .collect::<Vec<_>>();
    errif_greater_one_stdin(&confs)?;
    Ok(confs)
}

pub fn errif_greater_one_stdin(inps: &[Config]) -> Result<(), String> {
    let nstd = inps.iter().filter(|inp| inp.is_stdin()).count();
    if nstd > 1 {
        return fail!("At most one <stdin> input is allowed.");
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

pub fn file_metadata(md: &fs::Metadata) -> (u64, u64) {
    use filetime::FileTime;
    let last_modified = FileTime::from_last_modification_time(md).unix_seconds() as u64;
    let fsize = md.len();
    (last_modified, fsize)
}

/// Check if there is enough memory to process the file.
/// Return the maximum file size that can be processed.
/// If the file is larger than the maximum file size, return an error.
/// If memcheck is true, check memory in CONSERVATIVE mode (i.e., Filesize < AVAIL memory + SWAP -
/// headroom) If memcheck is false, check memory in NORMAL mode (i.e., Filesize < TOTAL memory -
/// headroom)
pub fn mem_file_check(
    path: &Path,
    version_check: bool,
    conservative_memcheck: bool,
) -> CliResult<i64> {
    // if we're NOT calling this from the version() and the file doesn't exist,
    // we don't need to check memory as file existence is checked before this function is called.
    // If we do get here with a non-existent file, that means we're using stdin,
    // so this check doesn't apply, so we return -1
    if !path.exists() && !version_check {
        return Ok(-1_i64);
    }

    let conservative_memcheck_work = get_envvar_flag("QSV_MEMORY_CHECK") || conservative_memcheck;

    let mut sys = sysinfo::System::new();
    sys.refresh_memory();
    let avail_mem = sys.available_memory();
    let free_swap = sys.free_swap();
    let total_mem = sys.total_memory();
    let mut mem_pct = env::var("QSV_FREEMEMORY_HEADROOM_PCT")
        .unwrap_or_else(|_| DEFAULT_FREEMEMORY_HEADROOM_PCT.to_string())
        .parse::<u8>()
        .unwrap_or(DEFAULT_FREEMEMORY_HEADROOM_PCT);

    // for safety, we don't want to go below 10% memory headroom
    // nor above 90% memory headroom as its too memory-restrictive
    mem_pct = mem_pct.clamp(10, 90);

    #[allow(clippy::cast_precision_loss)]
    let max_avail_mem = if conservative_memcheck_work {
        ((avail_mem + free_swap) as f32 * ((100 - mem_pct) as f32 / 100.0_f32)) as u64
    } else {
        (total_mem as f32 * ((100 - mem_pct) as f32 / 100.0_f32)) as u64
    };

    // if we're calling this from version(), we don't need to check the file size
    if !version_check {
        let file_metadata =
            fs::metadata(path).map_err(|e| format!("Failed to get file size: {e}"))?;
        let fsize = file_metadata.len();
        if fsize > max_avail_mem {
            return fail_OOM_clierror!(
                "Not enough memory to process the file. qsv running in non-streaming {mode} mode. \
                 Total memory: {total_mem} Available memory: {avail_mem}. Free swap: {free_swap} \
                 Max Available memory/Max input file size: {max_avail_mem}. \
                 QSV_FREEMEMORY_HEADROOM_PCT: {mem_pct}%. File size: {fsize}.",
                mode = if conservative_memcheck_work {
                    "CONSERVATIVE"
                } else {
                    "NORMAL"
                },
                total_mem = indicatif::HumanBytes(total_mem),
                avail_mem = indicatif::HumanBytes(avail_mem),
                free_swap = indicatif::HumanBytes(free_swap),
                max_avail_mem = indicatif::HumanBytes(max_avail_mem),
                mem_pct = mem_pct,
                fsize = indicatif::HumanBytes(fsize)
            );
        }
    }

    Ok(max_avail_mem as i64)
}

#[cfg(any(feature = "feature_capable", feature = "lite"))]
#[inline]
pub fn condense(val: Cow<[u8]>, n: Option<usize>) -> Cow<[u8]> {
    match n {
        None => val,
        Some(n) => {
            let mut is_short_utf8 = false;
            if let Ok(s) = simdutf8::basic::from_utf8(&val) {
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
        },
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
        (_, _, _, Some(_)) => fail!("--index cannot be used with --start, --end or --len"),
        (_, Some(_), Some(_), None) => {
            fail!("--end and --len cannot be used at the same time.")
        },
        (_, None, None, None) => Ok((start.unwrap_or(0), ::std::usize::MAX)),
        (_, Some(e), None, None) => {
            let s = start.unwrap_or(0);
            if s > e {
                fail_format!(
                    "The end of the range ({e}) must be greater than or\nequal to the start of \
                     the range ({s})."
                )
            } else {
                Ok((s, e))
            }
        },
        (_, None, Some(l), None) => {
            let s = start.unwrap_or(0);
            Ok((s, s + l))
        },
    }
}

/// Create a directory recursively, avoiding the race conditions fixed by
/// https://github.com/rust-lang/rust/pull/39799.
#[cfg(any(feature = "feature_capable", feature = "lite"))]
fn create_dir_all_threadsafe(path: &Path) -> std::io::Result<()> {
    use std::thread;

    // Try 20 times. This shouldn't theoretically need to be any larger
    // than the number of nested directories we need to create.
    for _ in 0..20 {
        match fs::create_dir_all(path) {
            // This happens if a directory in `path` doesn't exist when we
            // test for it, and another thread creates it before we can.
            Err(ref err) if err.kind() == std::io::ErrorKind::AlreadyExists => {},
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
/// the place to insert the part of the filename generated by `qsv`.
#[cfg(any(feature = "feature_capable", feature = "lite"))]
#[derive(Clone)]
pub struct FilenameTemplate {
    prefix: String,
    suffix: String,
}

#[cfg(any(feature = "feature_capable", feature = "lite"))]
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
    ) -> std::io::Result<csv::Writer<Box<dyn std::io::Write + 'static>>>
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

#[cfg(any(feature = "feature_capable", feature = "lite"))]
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

pub fn init_logger() -> (String, flexi_logger::LoggerHandle) {
    use flexi_logger::{Cleanup, Criterion, FileSpec, Logger, Naming};

    let qsv_log_env = env::var("QSV_LOG_LEVEL").unwrap_or_else(|_| "off".to_string());
    let qsv_log_dir = env::var("QSV_LOG_DIR").unwrap_or_else(|_| ".".to_string());
    let write_mode = if get_envvar_flag("QSV_LOG_UNBUFFERED") {
        flexi_logger::WriteMode::Direct
    } else {
        flexi_logger::WriteMode::BufferAndFlush
    };

    let logger = Logger::try_with_env_or_str(qsv_log_env)
        .unwrap()
        .use_utc()
        .log_to_file(
            FileSpec::default()
                .directory(qsv_log_dir)
                .suppress_timestamp(),
        )
        .write_mode(write_mode)
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
    (qsv_args, logger)
}

#[cfg(feature = "self_update")]
pub fn qsv_check_for_update(check_only: bool, no_confirm: bool) -> Result<bool, String> {
    use self_update::cargo_crate_version;
    const GITHUB_RATELIMIT_MSG: &str =
        "Github is rate-limiting self-update checks at the moment. Try again in an hour.";

    if get_envvar_flag("QSV_NO_UPDATE") {
        return Ok(false);
    }

    let bin_name = match std::env::current_exe() {
        Ok(pb) => {
            if let Some(fs) = pb.file_stem() {
                fs.to_string_lossy().into_owned()
            } else {
                return fail!("Can't get the exec stem name");
            }
        },
        Err(e) => return fail_format!("Can't get the exec path: {e}"),
    };

    winfo!("Checking GitHub for updates...");

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
            return fail!(GITHUB_RATELIMIT_MSG);
        }
    } else {
        return fail!(GITHUB_RATELIMIT_MSG);
    };
    let latest_release = &releases[0].version;

    log::info!("Current version: {curr_version} Latest Release: {latest_release}");

    let mut updated = false;
    let Ok(latest_release_sv) = semver::Version::parse(latest_release) else {
        return fail_format!("Can't parse latest release version: {latest_release}");
    };
    let Ok(curr_version_sv) = semver::Version::parse(curr_version) else {
        return fail_format!("Can't parse current version: {curr_version}");
    };

    if latest_release_sv > curr_version_sv {
        eprintln!("Update {latest_release} available. Current version is {curr_version}.");
        eprintln!(
            "Release notes: https://github.com/jqnatividad/qsv/releases/tag/{latest_release}\n"
        );
        if QSV_KIND.starts_with("prebuilt") && !check_only {
            match self_update::backends::github::Update::configure()
                .repo_owner("jqnatividad")
                .repo_name("qsv")
                .bin_name(&bin_name)
                .show_download_progress(true)
                .show_output(false)
                .no_confirm(no_confirm)
                .current_version(curr_version)
                .verifying_keys([*include_bytes!("qsv-zipsign-public.key")])
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
                    },
                    Err(e) => werr!("Update job error: {e}"),
                },
                Err(e) => werr!("Update builder error: {e}"),
            };
        } else if check_only {
            winfo!("Use the --update option to upgrade {bin_name} to the latest release.");
        } else {
            // we don't want to overwrite manually curated/configured qsv installations.
            // If QSV_KIND is not "prebuilt", just inform the user of the new release, and let them
            // rebuild their qsvs the way they like it, instead of overwriting it with
            // our prebuilt binaries.
            winfo!(
                r#"This qsv was {QSV_KIND}. self-update does not work for manually {QSV_KIND} binaries.
If you wish to update to the latest version of qsv, manually install/compile from source.
Self-update only works with prebuilt binaries released on GitHub https://github.com/jqnatividad/qsv/releases/latest"#
            );
        }
    } else {
        winfo!("Up to date ({curr_version})... no update required.");
    };

    if !check_only {
        if let Ok(status_code) =
            send_hwsurvey(&bin_name, updated, latest_release, curr_version, false)
        {
            log::info!("HW survey sent. Status code: {status_code}");
        }
    }

    Ok(updated)
}

#[cfg(not(feature = "self_update"))]
pub const fn qsv_check_for_update(_check_only: bool, _no_confirm: bool) -> Result<bool, String> {
    Ok(true)
}

// the qsv hwsurvey allows us to keep a better
// track of qsv's usage in the wild, so we can do a
// better job of prioritizing platforms/features we support
// no personally identifiable information is collected
#[cfg(feature = "self_update")]
fn send_hwsurvey(
    bin_name: &str,
    updated: bool,
    latest_release: &str,
    curr_version: &str,
    dry_run: bool,
) -> Result<reqwest::StatusCode, String> {
    use serde_json::json;
    use sysinfo::CpuExt;

    static HW_SURVEY_URL: &str =
        "https://4dhmneehnl.execute-api.us-east-1.amazonaws.com/dev/qsv-hwsurvey";

    let mut sys = System::new();
    sys.refresh_all();
    let total_mem = sys.total_memory();
    let kernel_version = sys
        .kernel_version()
        .unwrap_or_else(|| "Unknown kernel".to_string());
    let long_os_version = sys
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
            "os": long_os_version,
            "target": TARGET,
        }
    );
    log::debug!("hwsurvey: {hwsurvey_json}");

    let mut survey_done = false;
    let mut status = reqwest::StatusCode::OK;
    if dry_run {
        log::info!("Survey dry run. hw survey compiled successfully, but not sent.");
    } else {
        let client = match reqwest::blocking::Client::builder()
            .user_agent(default_user_agent())
            .brotli(true)
            .gzip(true)
            .deflate(true)
            .use_rustls_tls()
            .http2_adaptive_window(true)
            .build()
        {
            Ok(c) => c,
            Err(e) => return fail_format!("Cannot build hw_survey reqwest client: {e}"),
        };

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
            },
            Err(e) => {
                log::warn!("Cannot send hw survey: {e}");
                status = reqwest::StatusCode::BAD_REQUEST;
            },
        };
    }
    if survey_done || dry_run {
        Ok(status)
    } else {
        fail!("hw survey failed.")
    }
}

#[allow(clippy::needless_pass_by_value)]
pub fn safe_header_names(
    headers: &csv::StringRecord,
    check_first_char: bool,
    conditional: bool,
    reserved_names: Option<Vec<String>>,
    unsafe_prefix: &str,
    keep_case: bool,
) -> (Vec<String>, u16) {
    // Create "safe" var/key names - to support dynfmt/url-template, valid python vars & db-safe
    // column names. Fold to lowercase if keep_case is false. Trim leading & trailing whitespace.
    // Replace whitespace/non-alphanumeric) with _. If name starts with a number & check_first_char
    // is true, prepend the unsafe_prefix. If a column with the same name already exists,
    // append a sequence suffix (e.g. _n). Names are limited to 60 characters in length.
    // Empty names are replaced with unsafe_prefix as well.

    // If conditional = true & reserved_names is none, only rename the header if its not safe
    let prefix = if unsafe_prefix.is_empty() {
        "_"
    } else {
        unsafe_prefix
    };
    let safename_regex = regex_oncelock!(r"[^A-Za-z0-9]");
    let mut changed_count = 0_u16;
    let mut name_vec: Vec<String> = Vec::with_capacity(headers.len());
    let mut safe_name: String;
    let mut safename_always: String;
    let mut safename_candidate: String;
    let mut final_candidate: String;
    let mut buf_wrk = String::new();

    for header_name in headers {
        let reserved_found = if let Some(reserved_names_vec) = reserved_names.clone() {
            if keep_case {
                buf_wrk = header_name.to_owned();
            } else {
                to_lowercase_into(header_name, &mut buf_wrk);
            };
            reserved_names_vec
                .iter()
                .any(|reserved_name| reserved_name == &buf_wrk)
        } else {
            false
        };
        safe_name = if conditional && is_safe_name(header_name) && !reserved_found {
            header_name.to_string()
        } else {
            safename_always = if header_name.is_empty() {
                prefix.to_string()
            } else {
                safename_regex
                    .replace_all(header_name.trim(), "_")
                    .to_string()
            };
            if check_first_char && safename_always.as_bytes()[0].is_ascii_digit() {
                safename_always = format!("{prefix}{safename_always}");
            }

            safename_candidate = if reserved_found {
                log::warn!("\"{safename_always}\" is a reserved name: {reserved_names:?}");
                format!("reserved_{safename_always}")
            } else {
                safename_always
            };

            final_candidate = safename_candidate[..safename_candidate
                .chars()
                .map(char::len_utf8)
                .take(60)
                .sum()]
                .to_string();

            final_candidate = if keep_case {
                final_candidate
            } else {
                final_candidate.to_lowercase()
            };

            if prefix != "_" && final_candidate.starts_with('_') {
                final_candidate = format!("{prefix}{final_candidate}");
            }
            final_candidate
        };
        let mut sequence_suffix = 2_u16;
        let mut candidate_name = safe_name.clone();
        while name_vec.contains(&candidate_name) {
            candidate_name = format!("{safe_name}_{sequence_suffix}");
            sequence_suffix += 1;
        }
        if candidate_name.ne(header_name) {
            changed_count += 1;
        }
        name_vec.push(candidate_name);
    }
    log::debug!("safe header names: {name_vec:?}");
    (name_vec, changed_count)
}

#[inline]
pub fn is_safe_name(header_name: &str) -> bool {
    if header_name.trim().is_empty()
        || header_name.trim_start_matches('_').is_empty()
        || header_name.len() > 60
    {
        return false;
    }
    let first_character = header_name.trim_start_matches('_').as_bytes()[0];
    if first_character.is_ascii_digit() || first_character.is_ascii_whitespace() {
        return false;
    }
    let safename_re = regex_oncelock!(r"^[\w\-\s]+$");
    safename_re.is_match(header_name)
}

pub fn log_end(mut qsv_args: String, now: std::time::Instant) {
    if log::log_enabled!(log::Level::Info) {
        let ellipsis = if qsv_args.len() > 24 {
            utf8_truncate(&mut qsv_args, 24);
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

// taken from https://gist.github.com/dginev/f6da5e94335d545e0a7b
pub fn utf8_truncate(input: &mut String, maxsize: usize) {
    let mut utf8_maxsize = input.len();
    if utf8_maxsize >= maxsize {
        {
            let mut char_iter = input.char_indices();
            while utf8_maxsize >= maxsize {
                utf8_maxsize = match char_iter.next_back() {
                    Some((index, _)) => index,
                    _ => 0,
                };
            }
        } // Extra {} wrap to limit the immutable borrow of char_indices()
        input.truncate(utf8_maxsize);
    }
}

#[test]
#[cfg(feature = "self_update")]
fn test_hw_survey() {
    // we have this test primarily to exercise the sysinfo module
    assert!(send_hwsurvey("qsv", false, "0.0.2", "0.0.1", true).is_ok());
}

#[cfg(any(feature = "apply", feature = "datapusher_plus", feature = "geocode"))]
pub struct ColumnNameParser {
    chars: Vec<char>,
    pos:   usize,
}

#[cfg(any(feature = "apply", feature = "datapusher_plus", feature = "geocode"))]
impl ColumnNameParser {
    pub fn new(s: &str) -> ColumnNameParser {
        ColumnNameParser {
            chars: s.chars().collect(),
            pos:   0,
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
                    return fail!("Unclosed quote, missing \".");
                },
                Some('"') => {
                    self.bump();
                    if self.cur() == Some('"') {
                        self.bump();
                        name.push('"');
                        name.push('"');
                        continue;
                    }
                    break;
                },
                Some(c) => {
                    name.push(c);
                    self.bump();
                },
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

pub fn round_num(dec_f64: f64, places: u32) -> String {
    use rust_decimal::{Decimal, RoundingStrategy};

    // use from_f64_retain, so we have all the excess bits before rounding with
    // round_dp_with_strategy as from_f64 will prematurely round when it drops the excess bits
    let Some(dec_num) = Decimal::from_f64_retain(dec_f64) else {
        return String::new();
    };

    // round using Midpoint Nearest Even Rounding Strategy AKA "Bankers Rounding."
    // https://docs.rs/rust_decimal/latest/rust_decimal/enum.RoundingStrategy.html#variant.MidpointNearestEven
    // we also normalize to remove trailing zeroes and to change -0.0 to 0.0.
    dec_num
        .round_dp_with_strategy(places, RoundingStrategy::MidpointNearestEven)
        .normalize()
        .to_string()
}

#[inline]
pub fn transform(bs: &[u8], casei: bool) -> ByteString {
    if let Ok(s) = simdutf8::basic::from_utf8(bs) {
        if casei {
            let norm: String = s
                .trim()
                .chars()
                .map(|c| c.to_lowercase().next().unwrap())
                .collect();
            norm.into_bytes()
        } else {
            s.trim().as_bytes().to_vec()
        }
    } else {
        bs.to_vec()
    }
}

pub fn load_dotenv() -> CliResult<()> {
    // Use the default .env file in the current directory.
    // If there is no .env file in the current directory, check if there is
    // an .env file with the same filestem as the binary, in the same directory as the binary.
    // If there is, use that. Failing that, qsv proceeds with its default settings and
    // whatever manually set environment variables are present.

    // check if there is an .env file in the current directory
    if dotenvy::dotenv_override().is_ok() {
        log::info!(
            "Using .env file in current directory: {}",
            std::env::current_dir()?.display()
        );
    } else {
        // no .env file in the current directory or it was invalid
        // now check if there is an .env file with the same name as the executable
        // in the same directory as the executable
        let qsv_binary_path = std::env::current_exe()?;

        let qsv_dir = qsv_binary_path.parent().unwrap();

        let qsv_binary_filestem = qsv_binary_path
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        let mut qsv_binary_envprofile = qsv_dir.to_path_buf();
        qsv_binary_envprofile.set_file_name(format!("{qsv_binary_filestem}.env"));

        if std::path::Path::new(&qsv_binary_envprofile).exists() {
            log::info!(
                "Using binary .env file: {}",
                qsv_binary_envprofile.display()
            );
            if let Err(e) = dotenvy::from_filename_override(qsv_binary_envprofile.clone()) {
                return fail_clierror!(
                    "Cannot process binary .env file - {}: {e}",
                    qsv_binary_envprofile.display()
                );
            }
        } else {
            // there is no binary .env file, just use the default settings
            // and whatever manually set environment variables are present
            log::info!(
                "No valid .env file found. Proceeding with default settings and current \
                 environment variable settings."
            );
        }
    }

    Ok(())
}

pub fn get_envvar_flag(key: &str) -> bool {
    if let Ok(tf_val) = std::env::var(key) {
        let tf_val = tf_val.to_lowercase();
        match tf_val {
            s if s == "true" || s == "t" || s == "1" || s == "yes" || s == "y" => true,
            s if s == "false" || s == "f" || s == "0" || s == "no" || s == "n" => false,
            _ => false,
        }
    } else {
        false
    }
}

pub fn decompress_snappy_file(
    path: &PathBuf,
    tmpdir: &tempfile::TempDir,
) -> Result<String, CliError> {
    let mut snappy_file = std::fs::File::open(path.clone())?;
    let mut snappy_reader = snap::read::FrameDecoder::new(&mut snappy_file);
    let file_stem = Path::new(&path).file_stem().unwrap().to_str().unwrap();
    let decompressed_filepath = tmpdir
        .path()
        .join(format!("qsv__{file_stem}__qsv_temp_decompressed"));
    let mut decompressed_file = std::fs::File::create(decompressed_filepath.clone())?;
    std::io::copy(&mut snappy_reader, &mut decompressed_file)?;
    decompressed_file.flush()?;
    Ok(format!("{}", decompressed_filepath.display()))
}

/// downloads a file from a url and saves it to a path
/// if show_progress is true, a progress bar will be shown
/// if custom_user_agent is Some, it will be used as the user agent
/// if download_timeout is Some, it will be used as the timeout in seconds
/// if sample_size is Some, it will be used as the number of bytes to download
pub async fn download_file(
    url: &str,
    path: PathBuf,
    show_progress: bool,
    custom_user_agent: Option<String>,
    download_timeout: Option<u16>,
    sample_size: Option<u64>,
) -> CliResult<()> {
    use futures_util::StreamExt;

    let user_agent = set_user_agent(custom_user_agent)?;

    let download_timeout = match download_timeout {
        Some(t) => std::time::Duration::from_secs(timeout_secs(t).unwrap_or(30)),
        None => std::time::Duration::from_secs(30),
    };

    // setup the reqwest client
    let client = match Client::builder()
        .user_agent(user_agent)
        .brotli(true)
        .gzip(true)
        .deflate(true)
        .use_rustls_tls()
        .http2_adaptive_window(true)
        .connection_verbose(log_enabled!(log::Level::Debug) || log_enabled!(log::Level::Trace))
        .timeout(download_timeout)
        .build()
    {
        Ok(c) => c,
        Err(e) => {
            return fail_clierror!("Cannot build reqwest client: {e}.");
        },
    };

    let res = client.get(url).send().await?;

    let total_size = match res.content_length() {
        Some(l) => l,
        None => {
            // if we can't get the content length, set it to sentinel value
            u64::MAX
        },
    };

    // progressbar setup
    #[cfg(any(feature = "feature_capable", feature = "lite"))]
    let show_progress = (show_progress || get_envvar_flag("QSV_PROGRESSBAR")) && total_size > 0;

    #[cfg(any(feature = "feature_capable", feature = "lite"))]
    let pb = ProgressBar::with_draw_target(Some(total_size), ProgressDrawTarget::stderr_with_hz(5));

    #[cfg(any(feature = "feature_capable", feature = "lite"))]
    if show_progress {
        pb.set_style(
            ProgressStyle::default_bar()
                .template(if total_size == u64::MAX {
                    // only do a spinner if we don't know the total size
                    "{msg}\n{spinner:.green} ({bytes_per_sec})"
                } else {
                    "{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.white/blue}] \
                     {bytes}/{total_bytes} ({bytes_per_sec}, {eta})"
                })
                .unwrap(),
        );
        pb.set_message(format!("Downloading {url}"));
    } else {
        pb.set_draw_target(ProgressDrawTarget::hidden());
    }

    let sample_size = sample_size.unwrap_or(0);

    // download chunks
    let mut file = File::create(path)?;
    let mut downloaded: u64 = 0;
    let mut stream = res.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item?;
        file.write_all(&chunk)?;
        let new = min(downloaded + (chunk.len() as u64), total_size);
        downloaded = new;

        #[cfg(any(feature = "feature_capable", feature = "lite"))]
        if show_progress {
            pb.set_position(new);
        }

        if sample_size > 0 && downloaded >= sample_size {
            break;
        }
    }

    #[cfg(any(feature = "feature_capable", feature = "lite"))]
    if show_progress {
        pb.finish_with_message(format!("Downloaded {url}"));
        eprintln!(); // newline after progress bar
    }

    Ok(())
}

/// load the first BUFFER*4 (128k) bytes of the file and check if it is utf8
pub fn isutf8_file(path: &Path) -> Result<bool, CliError> {
    let metadata = std::fs::metadata(path)?;
    let buffer_len = config::DEFAULT_RDR_BUFFER_CAPACITY * 4;
    let file_size = metadata.len() as usize;
    let bytes_to_read: usize = if file_size < buffer_len {
        file_size
    } else {
        buffer_len
    };

    let file = std::fs::File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut buffer = Vec::with_capacity(bytes_to_read);
    reader.read_to_end(&mut buffer)?;

    Ok(simdutf8::basic::from_utf8(&buffer).is_ok())
}

/// Process the input files and return a vector of paths to the input files
///
/// If the input is empty, try to copy stdin to a file named stdin in the passed temp directory
/// If the input is empty and stdin is empty, return an error
/// If it's not empty, check the input files if they exist, and return an error if they don't
///
/// If the input is a directory, add all the files in the directory to the input
/// If the input is a file, add the file to the input
/// If the input are snappy compressed files, uncompress them before adding them to the input
pub fn process_input(
    mut arg_input: Vec<PathBuf>,
    tmpdir: &tempfile::TempDir,
    empty_stdin_errmsg: &str,
) -> Result<Vec<PathBuf>, CliError> {
    let mut processed_input = Vec::with_capacity(arg_input.len());

    // if the input is empty or "-", its stdin. try to copy stdin to a file named
    // "stdin" in the passed temp directory
    if arg_input.is_empty() || arg_input[0] == PathBuf::from("-") {
        // copy stdin to a file named stdin in a temp directory
        let tmp_filename = tmpdir.path().join("stdin");
        let mut tmp_file = std::fs::File::create(&tmp_filename)?;
        std::io::copy(&mut std::io::stdin(), &mut tmp_file)?;
        tmp_file.flush()?;
        processed_input.push(tmp_filename);
        // remove the "-" from the input
        // if the input was empty, this does nothing
        arg_input.remove(0);
    }

    let work_input = if arg_input.len() == 1 && arg_input[0].is_dir() {
        // if the input is a directory, add all the files in the directory to the input
        std::fs::read_dir(&arg_input[0])?
            .map(|entry| entry.map(|e| e.path()))
            .collect::<Result<Vec<_>, _>>()?
    } else {
        arg_input
    };

    // check the input files
    for path in work_input {
        // does the input file exist?
        if !path.exists() {
            return fail_clierror!("Input file '{}' does not exist", path.display());
        }

        // is the input file snappy compressed?
        if path.extension().and_then(std::ffi::OsStr::to_str) == Some("sz") {
            // if so, decompress the file
            let decompressed_filepath = decompress_snappy_file(&path, tmpdir)?;

            // rename the decompressed file to the original filename, but still
            // inside the temp directory. this is so that the decompressed file can be
            // processed as if it was the original file without the "sz" extension
            let original_filepath = path.with_extension("");
            // safety: we know the path has a filename
            let original_filename = original_filepath.file_name().unwrap();

            let final_decompressed_filepath = tmpdir.path().join(original_filename);
            std::fs::rename(&decompressed_filepath, &final_decompressed_filepath)?;

            processed_input.push(final_decompressed_filepath);
        } else {
            processed_input.push(path);
        }
    }

    if processed_input.is_empty() {
        return fail_clierror!("{empty_stdin_errmsg}");
    }
    log::debug!("processed input: {:?}", processed_input);
    Ok(processed_input)
}
