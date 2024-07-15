use clap::{arg, Command};

pub fn reverse_cmd() -> Command {
    Command::new("reverse").args([
        arg!(--output),
        arg!(--"no-headers"),
        arg!(--delimiter),
        arg!(--memcheck),
    ])
}
