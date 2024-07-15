use clap::{arg, Command};

pub fn exclude_cmd() -> Command {
    Command::new("exclude").args([
        arg!(--"ignore-case"),
        arg!(name: -v),
        arg!(--output),
        arg!(--"no-headers"),
        arg!(--delimiter),
    ])
}
