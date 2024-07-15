use clap::{arg, Command};

pub fn fetch_cmd() -> Command {
    Command::new("fetch").args([
        arg!(--"url-template"),
        arg!(--"new-column"),
        arg!(--jql),
        arg!(--jqlfile),
        arg!(--pretty),
        arg!(--"rate-limit"),
        arg!(--timeout),
        arg!(--"http-header"),
        arg!(--"max-retries"),
        arg!(--"max-errors"),
        arg!(--"store-error"),
        arg!(--cookies),
        arg!(--"user-agent"),
        arg!(--report),
        arg!(--"no-cache"),
        arg!(--"mem-cache-size"),
        arg!(--"disk-cache"),
        arg!(--"disk-cache-dir"),
        arg!(--"redis-cache"),
        arg!(--"cache-error"),
        arg!(--"flush-cache"),
        arg!(--output),
        arg!(--"no-headers"),
        arg!(--delimiter),
        arg!(--progressbar),
    ])
}
