use clap::{arg, Command};

pub fn fetchpost_cmd() -> Command {
    Command::new("fetchpost").args([
        arg!(--"new-column"),
        arg!(--jql),
        arg!(--jqlfile),
        arg!(--pretty),
        arg!(--"rate-limit"),
        arg!(--timeout),
        arg!(--"http-header"),
        arg!(--compress),
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
