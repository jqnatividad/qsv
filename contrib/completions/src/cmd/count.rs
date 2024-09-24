use clap::{arg, Command};

pub fn count_cmd() -> Command {
    Command::new("count").args([
        arg!(--"human-readable"),
        arg!(--width),
        arg!(--"width-no-delims"),
        arg!(--json),
        arg!(--"no-polars"),
        arg!(--"low-memory"),
        arg!(--flexible),
        arg!(--"no-headers"),
        arg!(--delimiter),
    ])
}
