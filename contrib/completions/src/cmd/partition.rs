use clap::{arg, Command};

pub fn partition_cmd() -> Command {
    Command::new("partition").args([
        arg!(--filename),
        arg!(--"prefix-length"),
        arg!(--drop),
        arg!(--"no-headers"),
        arg!(--delimiter),
    ])
}
