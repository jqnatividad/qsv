use clap::{arg, Command};

pub fn slice_cmd() -> Command {
    Command::new("slice").args([
        arg!(--start),
        arg!(--end),
        arg!(--len),
        arg!(--index),
        arg!(--json),
        arg!(--output),
        arg!(--"no-headers"),
        arg!(--delimiter),
    ])
}
