use clap::{arg, Command};

pub fn edit_cmd() -> Command {
    Command::new("edit").args([arg!(--output), arg!(--"no-headers")])
}
