use clap::{arg, Command};

pub fn fill_cmd() -> Command {
    Command::new("fill").args([
        arg!(--groupby),
        arg!(--first),
        arg!(--backfill),
        arg!(--default),
        arg!(--output),
        arg!(--"no-headers"),
        arg!(--delimiter),
    ])
}
