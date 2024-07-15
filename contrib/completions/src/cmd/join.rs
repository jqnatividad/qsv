use clap::{arg, Command};

pub fn join_cmd() -> Command {
    Command::new("join").args([
        arg!(--"ignore-case"),
        arg!(--"left-anti"),
        arg!(--"left-semi"),
        arg!(--right),
        arg!(--full),
        arg!(--cross),
        arg!(--nulls),
        arg!(--output),
        arg!(--"no-headers"),
        arg!(--delimiter),
    ])
}
