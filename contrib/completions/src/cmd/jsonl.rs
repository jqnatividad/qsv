use clap::{arg, Command};

pub fn jsonl_cmd() -> Command {
    Command::new("jsonl").args([
        arg!(--"ignore-errors"),
        arg!(--jobs),
        arg!(--batch),
        arg!(--output),
        arg!(--delimiter),
    ])
}
