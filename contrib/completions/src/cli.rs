use crate::cmd::{clipboard::clipboard_cmd, count::count_cmd};
use clap::{arg, Command};

pub fn build_cli() -> Command {
    Command::new("qsv")
        .args([
            arg!(--list),
            arg!(--envlist),
            arg!(--update),
            arg!(--updatenow),
            arg!(--version),
        ])
        .subcommands([clipboard_cmd(), count_cmd()])
}
