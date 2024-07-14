use clap::{arg, Command};

pub fn clipboard_cmd() -> Command {
    Command::new("clipboard").args([arg!(--save)])
}
