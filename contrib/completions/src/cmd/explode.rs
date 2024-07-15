use clap::{arg, Command};

pub fn explode_cmd() -> Command {
    Command::new("explode").args([
        arg!(--rename),
        arg!(--output),
        arg!(--"no-headers"),
        arg!(--delimiter),
    ])
}
