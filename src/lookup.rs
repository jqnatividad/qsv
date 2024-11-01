use std::{
    fs,
    io::Write,
    path::Path,
    time::{Instant, SystemTime},
};

use log::{debug, info};
use reqwest::blocking::Client;
use serde_json::Value;
use simple_expand_tilde::expand_tilde;

use crate::CliError;

pub struct LookupTableOptions {
    pub name:           String,
    pub uri:            String,
    pub cache_age_secs: i64,
    pub cache_dir:      String,
    pub delimiter:      Option<crate::config::Delimiter>,
    pub ckan_api_url:   Option<String>,
    pub ckan_token:     Option<String>,
    pub timeout_secs:   u16,
}

pub struct LookupTableResult {
    pub filepath: String,
    pub headers:  csv::StringRecord,
}

pub fn set_qsv_cache_dir(cache_dir: &str) -> Result<String, CliError> {
    let qsv_cache_dir = if let Ok(cache_path) = std::env::var("QSV_CACHE_DIR") {
        // if QSV_CACHE_DIR env var is set, check if it exists. If it doesn't, create it.
        if cache_path.starts_with('~') {
            // expand the tilde
            let expanded_dir = expand_tilde(&cache_path).unwrap();
            expanded_dir.to_string_lossy().to_string()
        } else {
            cache_path
        }
    } else if cache_dir.starts_with('~') {
        // expand the tilde
        let expanded_dir = expand_tilde(&cache_dir).unwrap();
        expanded_dir.to_string_lossy().to_string()
    } else {
        cache_dir.to_string()
    };
    if !Path::new(&qsv_cache_dir).exists() {
        fs::create_dir_all(&qsv_cache_dir)?;
    }
    Ok(qsv_cache_dir)
}

pub fn load_lookup_table(
    opts: &LookupTableOptions,
) -> Result<LookupTableResult, Box<dyn std::error::Error>> {
    let mut lookup_table_uri = opts.uri.clone();
    let cached_csv_path = Path::new(&opts.cache_dir).join(format!("{}.csv", opts.name));

    // Check if local file
    let lookup_table_path = Path::new(&lookup_table_uri);
    let lookup_table_is_file = lookup_table_path.exists();

    // Check cache status
    let (cached_csv_exists, cached_csv_age_secs, cached_csv_size, cache_csv_last_modified) =
        if cached_csv_path.exists() {
            if opts.cache_age_secs < 0 {
                // Delete cached file if negative cache age
                std::fs::remove_file(&cached_csv_path)?;
                (false, 0, 0, None)
            } else {
                let metadata = cached_csv_path.metadata()?;
                let last_modified = metadata.modified()?;
                let modified_secs = last_modified
                    .duration_since(SystemTime::UNIX_EPOCH)?
                    .as_secs();
                let now_secs = SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)?
                    .as_secs();
                let age = if opts.cache_age_secs > 0 {
                    (now_secs - modified_secs).try_into().unwrap_or(0_i64)
                } else {
                    0_i64
                };
                (true, age, metadata.len(), Some(last_modified))
            }
        } else {
            (false, 0, 0, None)
        };

    // Use cached file if valid
    if !lookup_table_is_file
        && cached_csv_exists
        && cached_csv_age_secs <= opts.cache_age_secs
        && cached_csv_size > 0
    {
        lookup_table_uri = cached_csv_path.display().to_string();
        info!("Using cached lookup table {}", lookup_table_uri);
    } else if !lookup_table_is_file {
        // Handle remote files
        if let Some(lookup_url) = lookup_table_uri.strip_prefix("dathere://") {
            lookup_table_uri = format!(
                "https://raw.githubusercontent.com/dathere/qsv-lookup-tables/main/lookup-tables/{lookup_url}"
            );
        }

        let (lookup_ckan, resource_search) =
            if let Some(lookup_url) = lookup_table_uri.strip_prefix("ckan://") {
                let lookup_url = lookup_url.trim();
                if lookup_url.ends_with('?') {
                    lookup_table_uri = format!(
                        "{}/resource_search?query=name:{}",
                        opts.ckan_api_url.as_deref().unwrap_or_default(),
                        lookup_url
                    );
                    lookup_table_uri.pop();
                    (true, true)
                } else {
                    lookup_table_uri = format!(
                        "{}/resource_show?id={}",
                        opts.ckan_api_url.as_deref().unwrap_or_default(),
                        lookup_url
                    );
                    (true, false)
                }
            } else {
                (false, false)
            };

        let lookup_on_url = lookup_table_uri.to_lowercase().starts_with("http");

        if lookup_on_url {
            download_lookup_table(
                &lookup_table_uri,
                &cached_csv_path,
                lookup_ckan,
                resource_search,
                cache_csv_last_modified,
                opts,
            )?;
            lookup_table_uri = cached_csv_path.to_string_lossy().to_string();
        }
    }

    // Read headers from the lookup table
    let conf = crate::config::Config::new(Some(lookup_table_uri.clone()).as_ref())
        .delimiter(opts.delimiter)
        .comment(Some(b'#'))
        .no_headers(false);

    let mut rdr = conf.reader()?;
    let headers = rdr.headers()?.clone();

    let lur = LookupTableResult {
        filepath: lookup_table_uri,
        headers,
    };

    Ok(lur)
}

fn download_lookup_table(
    lookup_table_uri: &str,
    cache_file_path: &Path,
    lookup_ckan: bool,
    resource_search: bool,
    cache_csv_last_modified: Option<SystemTime>,
    opts: &LookupTableOptions,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder()
        .user_agent(crate::util::set_user_agent(None).unwrap())
        .brotli(true)
        .gzip(true)
        .deflate(true)
        .zstd(true)
        .use_rustls_tls()
        .http2_adaptive_window(true)
        .connection_verbose(log::log_enabled!(log::Level::Trace))
        .timeout(std::time::Duration::from_secs(opts.timeout_secs as u64))
        .build()?;

    let now = SystemTime::now();
    let now_dt_utc: chrono::DateTime<chrono::Utc> = now.into();
    let download_start = Instant::now();
    let last_modified_rfc8222 = now_dt_utc.to_rfc2822();

    let lookup_csv_response = if lookup_ckan {
        get_ckan_response(&client, lookup_table_uri, resource_search, opts)?
    } else {
        get_http_response(&client, lookup_table_uri, cache_csv_last_modified)?
    };

    let write_csv_contents = should_write_contents(&lookup_csv_response);
    let lookup_csv_contents = lookup_csv_response.text()?;

    if write_csv_contents && !lookup_csv_contents.is_empty() {
        write_cache_file(
            cache_file_path,
            &lookup_csv_contents,
            &last_modified_rfc8222,
            download_start,
            opts,
        )?;
    }

    Ok(())
}

// Helper functions for download_lookup_table
fn get_ckan_response(
    client: &Client,
    uri: &str,
    resource_search: bool,
    opts: &LookupTableOptions,
) -> Result<reqwest::blocking::Response, Box<dyn std::error::Error>> {
    let mut headers = reqwest::header::HeaderMap::new();

    if let Some(token) = &opts.ckan_token {
        headers.insert(
            reqwest::header::AUTHORIZATION,
            reqwest::header::HeaderValue::from_str(token)?,
        );
    }

    if resource_search {
        let resource_search_result = client.get(uri).headers(headers.clone()).send()?.text()?;
        let resource_search_json: Value = serde_json::from_str(&resource_search_result)?;

        let resource_id = resource_search_json["result"]["results"][0]["id"]
            .as_str()
            .ok_or("Cannot find resource name")?;

        let resource_uri = format!(
            "{}/resource_show?id={}",
            opts.ckan_api_url.as_deref().unwrap_or_default(),
            resource_id
        );

        let resource_show_result = client
            .get(resource_uri)
            .headers(headers.clone())
            .send()?
            .text()?;
        let resource_show_json: Value = serde_json::from_str(&resource_show_result)?;

        let url = resource_show_json["result"]["url"]
            .as_str()
            .ok_or("Cannot get resource URL from resource_show JSON response")?;

        client.get(url).headers(headers).send().map_err(Into::into)
    } else {
        client.get(uri).headers(headers).send().map_err(Into::into)
    }
}

fn get_http_response(
    client: &Client,
    uri: &str,
    cache_csv_last_modified: Option<SystemTime>,
) -> Result<reqwest::blocking::Response, Box<dyn std::error::Error>> {
    let mut headers = reqwest::header::HeaderMap::new();

    if let Some(modified) = cache_csv_last_modified {
        let last_modified: chrono::DateTime<chrono::Utc> = modified.into();
        let last_modified_rfc8222 = last_modified.to_rfc2822();
        headers.insert(
            reqwest::header::IF_MODIFIED_SINCE,
            reqwest::header::HeaderValue::from_str(&last_modified_rfc8222)?,
        );
    }

    client.get(uri).headers(headers).send().map_err(Into::into)
}

fn should_write_contents(response: &reqwest::blocking::Response) -> bool {
    if response.status() == reqwest::StatusCode::NOT_MODIFIED {
        debug!("Lookup CSV hasn't changed, so using cached CSV.");
        false
    } else {
        response.status().is_success()
    }
}

fn write_cache_file(
    cache_file_path: &Path,
    contents: &str,
    last_modified: &str,
    download_start: Instant,
    opts: &LookupTableOptions,
) -> Result<(), Box<dyn std::error::Error>> {
    info!(
        "Writing lookup CSV to cache file: {}",
        cache_file_path.display()
    );
    let mut cache_file = std::fs::File::create(cache_file_path)?;

    writeln!(
        cache_file,
        "# qsv_register_lookup({}, {}, {})",
        opts.name, opts.uri, opts.cache_age_secs
    )?;
    writeln!(cache_file, "# Last-Modified: {last_modified}")?;
    writeln!(
        cache_file,
        "# Download-duration-ms: {}",
        download_start.elapsed().as_millis()
    )?;
    cache_file.write_all(contents.as_bytes())?;
    cache_file.flush()?;

    Ok(())
}
