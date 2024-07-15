use clap::{arg, Command};

pub fn rename_cmd() -> Command {
    Command::new("rename").args([arg!(--output), arg!(--"no-headers"), arg!(--delimiter)])
}
