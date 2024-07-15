use clap::{arg, Command};

pub fn pseudo_cmd() -> Command {
    Command::new("pseudo").args([
        arg!(--start),
        arg!(--increment),
        arg!(--formatstr),
        arg!(--"no-headers"),
        arg!(--delimiter),
    ])
}
