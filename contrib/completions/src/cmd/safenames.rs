use clap::{arg, Command};

pub fn safenames_cmd() -> Command {
    Command::new("safenames").args([
        arg!(--mode),
        arg!(--reserved),
        arg!(--prefix),
        arg!(--output),
        arg!(--delimiter),
    ])
}
