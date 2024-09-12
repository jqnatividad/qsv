use clap::{arg, Command};

pub fn to_cmd() -> Command {
    let global_args = [
        arg!(--"print-package"),
        arg!(--dump),
        arg!(--stats),
        arg!(--"stats-csv"),
        arg!(--quiet),
        arg!(--schema),
        arg!(--drop),
        arg!(--evolve),
        arg!(--pipe),
        arg!(--separator),
        arg!(--jobs),
        arg!(--delimiter),
    ];
    Command::new("to")
        .subcommands([
            Command::new("postgres").args(&global_args),
            Command::new("sqlite").args(&global_args),
            Command::new("xlsx").args(&global_args),
            Command::new("datapackage").args(&global_args),
        ])
        .args(global_args)
}
