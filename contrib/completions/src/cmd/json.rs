use clap::{arg, Command};

pub fn json_cmd() -> Command {
    Command::new("json").args([arg!(--jaq), arg!(--select), arg!(--output)])
}
