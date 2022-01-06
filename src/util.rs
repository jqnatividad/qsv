#![allow(dead_code)]
use std::borrow::Cow;
use std::cmp;
use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::str;
use std::thread;
use std::time;

use ::num_cpus;
use docopt::Docopt;
use log::{info, log_enabled, Level};
use serde::de::{Deserialize, DeserializeOwned, Deserializer, Error};

use crate::config::{Config, Delimiter};
use crate::CliResult;
use indicatif::{ProgressBar, ProgressStyle};
use thousands::Separable;

pub fn num_cpus() -> usize {
    num_cpus::get()
}

const MAX_JOBS_CPU_DIVISOR: usize = 3;

pub fn max_jobs() -> usize {
    let cpus = num_cpus::get();
    let max_jobs_env = match env::var("QSV_MAX_JOBS") {
        Ok(val) => val.parse::<isize>().unwrap_or_default(),
        Err(_) => 0,
    };
    match max_jobs_env {
        x if x > cpus as isize => cpus,
        x if x <= 0 => cmp::max(cpus / MAX_JOBS_CPU_DIVISOR, 1),
        _ => max_jobs_env as usize,
    }
}

pub fn version() -> String {
    let mut enabled_features = "".to_string();
    if let Some(qsv_type) = option_env!("CARGO_BIN_NAME") {
        if qsv_type != "qsvlite" {
            #[cfg(feature = "apply")]
            enabled_features.push_str("apply;");
            #[cfg(feature = "foreach")]
            enabled_features.push_str("foreach;");
            #[cfg(feature = "generate")]
            enabled_features.push_str("generate;");
            #[cfg(feature = "lua")]
            enabled_features.push_str("lua;");
            #[cfg(feature = "python")]
            enabled_features.push_str("python;");

            enabled_features.push('-');
        }
    }

    #[cfg(feature = "mimalloc")]
    let malloc_kind = "mimalloc".to_string();
    #[cfg(not(feature = "mimalloc"))]
    let malloc_kind = "standard".to_string();
    let (qsvtype, maj, min, pat, pre) = (
        option_env!("CARGO_BIN_NAME"),
        option_env!("CARGO_PKG_VERSION_MAJOR"),
        option_env!("CARGO_PKG_VERSION_MINOR"),
        option_env!("CARGO_PKG_VERSION_PATCH"),
        option_env!("CARGO_PKG_VERSION_PRE"),
    );
    match (qsvtype, maj, min, pat, pre) {
        (Some(qsvtype), Some(maj), Some(min), Some(pat), Some(pre)) => {
            if pre.is_empty() {
                return format!(
                    "{} {}.{}.{}-{}-{}{}-{}",
                    qsvtype,
                    maj,
                    min,
                    pat,
                    malloc_kind,
                    enabled_features,
                    max_jobs(),
                    num_cpus()
                );
            } else {
                return format!(
                    "{} {}.{}.{}-{}-{}-{}{}-{}",
                    qsvtype,
                    maj,
                    min,
                    pat,
                    pre,
                    malloc_kind,
                    enabled_features,
                    max_jobs(),
                    num_cpus(),
                );
            }
        }
        _ => "".to_owned(),
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
            println!("{}: {}", env_var, v.into_string().unwrap());
        }
    }
    if !env_var_set {
        println!("No qsv-relevant environment variables set.");
    }
}

pub fn count_rows(conf: &Config) -> u64 {
    match conf.indexed().unwrap() {
        Some(idx) => idx.count(),
        None => {
            let mut rdr = conf.reader().unwrap();
            let mut count = 0u64;
            let mut record = csv::ByteRecord::new();
            while rdr.read_byte_record(&mut record).unwrap() {
                count += 1;
            }
            count
        }
    }
}

pub fn prep_progress(progress: &ProgressBar, record_count: u64) {
    progress.set_length(record_count);
    progress.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] [{bar:20} {percent}%{msg}] ({eta})")
            .progress_chars("=>-"),
    );
    progress.set_message(format!(
        " of {} records",
        record_count.separate_with_commas()
    ));
    progress.set_draw_delta(record_count / 100);

    if log_enabled!(Level::Info) {
        info!("Progress started... {} records", record_count);
    }
}

pub fn finish_progress(progress: &ProgressBar) {
    let per_sec_rate = progress.per_sec();

    let finish_template = format!(
        "[{{elapsed_precise}}] [{{bar:20}} {{percent}}%{{msg}}] ({}/sec)",
        per_sec_rate.separate_with_commas()
    );

    progress.set_style(
        ProgressStyle::default_bar()
            .template(&finish_template)
            .progress_chars("=>-"),
    );
    progress.finish();

    if log_enabled!(Level::Info) {
        info!("Progress done... {} records/sec", per_sec_rate);
    }
}

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
        })
        .collect::<Vec<_>>();
    errif_greater_one_stdin(&*confs)?;
    Ok(confs)
}

pub fn errif_greater_one_stdin(inps: &[Config]) -> Result<(), String> {
    let nstd = inps.iter().filter(|inp| inp.is_std()).count();
    if nstd > 1 {
        return Err("At most one <stdin> input is allowed.".to_owned());
    }
    Ok(())
}

pub fn chunk_size(nitems: usize, njobs: usize) -> usize {
    if nitems < njobs {
        nitems
    } else {
        nitems / njobs
    }
}

pub fn num_of_chunks(nitems: usize, chunk_size: usize) -> usize {
    if chunk_size == 0 {
        return nitems;
    }
    let mut n = nitems / chunk_size;
    if nitems % chunk_size != 0 {
        n += 1;
    }
    n
}

pub fn last_modified(md: &fs::Metadata) -> u64 {
    use filetime::FileTime;
    FileTime::from_last_modification_time(md).unix_seconds() as u64
}

pub fn condense(val: Cow<[u8]>, n: Option<usize>) -> Cow<[u8]> {
    match n {
        None => val,
        Some(n) => {
            let mut is_short_utf8 = false;
            if let Ok(s) = str::from_utf8(&*val) {
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
                s.extend(b"...".iter().cloned());
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
                    "The end of the range ({}) must be greater than or\n\
                             equal to the start of the range ({}).",
                    e, s
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
        thread::sleep(time::Duration::from_millis(25));
    }
    // Try one last time, returning whatever happens.
    fs::create_dir_all(path)
}

/// Represents a filename template of the form `"{}.csv"`, where `"{}"` is
/// the splace to insert the part of the filename generated by `qsv`.
#[derive(Clone, Debug)]
pub struct FilenameTemplate {
    prefix: String,
    suffix: String,
}

impl FilenameTemplate {
    /// Generate a new filename using `unique_value` to replace the `"{}"`
    /// in the template.
    pub fn filename(&self, unique_value: &str) -> String {
        format!("{}{}{}", &self.prefix, unique_value, &self.suffix)
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

pub fn init_logger() {
    use flexi_logger::{Cleanup, Criterion, FileSpec, Logger, Naming};

    let qsv_log_env = env::var("QSV_LOG_LEVEL").unwrap_or_else(|_| "off".to_string());
    let qsv_log_dir = env::var("QSV_LOG_DIR").unwrap_or_else(|_| ".".to_string());

    Logger::try_with_env_or_str(qsv_log_env)
        .unwrap()
        .log_to_file(
            FileSpec::default()
                .directory(qsv_log_dir)
                .suppress_timestamp(),
        )
        .format_for_files(flexi_logger::detailed_format)
        .o_append(true)
        .rotate(
            Criterion::Size(1_000_000),
            Naming::Numbers,
            Cleanup::KeepLogAndCompressedFiles(10, 100),
        )
        .start()
        .unwrap();
}

pub fn qsv_update(verbose: bool) -> Result<(), Box<dyn ::std::error::Error>> {
    use self_update::cargo_crate_version;

    if env::var("QSV_NO_UPDATE").is_ok() {
        return Ok(());
    }

    let curr_version = cargo_crate_version!();
    let releases = self_update::backends::github::ReleaseList::configure()
        .repo_owner("jqnatividad")
        .repo_name("qsv")
        .build()?
        .fetch()?;
    let latest_release = &releases[0].version;

    if log_enabled!(Level::Info) {
        info!(
            "Current version: {} Latest Release: {}",
            curr_version, latest_release
        );
    }

    if latest_release > &curr_version.to_string() {
        println!(
            "Update {} available. Current version is {}.",
            latest_release, curr_version
        );
        let status = self_update::backends::github::Update::configure()
            .repo_owner("jqnatividad")
            .repo_name("qsv")
            .bin_name("qsvlite")
            .show_download_progress(true)
            .show_output(verbose)
            .no_confirm(false)
            .current_version(curr_version)
            .build()?
            .update()?;
        let exe_full_path = format!("{:?}", std::env::current_exe().unwrap());
        let update_status = format!(
            "Update successful for {}: `{}`!",
            exe_full_path,
            status.version()
        );
        if verbose {
            println!("{}", update_status);
        }
        if log_enabled!(Level::Info) {
            info!("{}", update_status);
        }
    }
    Ok(())
}
