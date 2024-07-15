use clap::{arg, Command};

pub fn behead_cmd() -> Command {
    Command::new("behead").args([arg!(--flexible), arg!(--output)])
}
