use clap::{arg, Command};

pub fn sortcheck_cmd() -> Command {
    Command::new("sortcheck").args([
        arg!(--select),
        arg!(--"ignore-case"),
        arg!(--all),
        arg!(--json),
        arg!(--"pretty-json"),
        arg!(--"no-headers"),
        arg!(--delimiter),
        arg!(--progressbar),
    ])
}
