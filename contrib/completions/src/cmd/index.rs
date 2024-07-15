use clap::{arg, Command};

pub fn index_cmd() -> Command {
    Command::new("index").args([arg!(--output)])
}
