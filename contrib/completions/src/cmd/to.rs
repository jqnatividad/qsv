use clap::{arg, Command};

pub fn to_cmd() -> Command {
    Command::new("to")
        .subcommands([
            Command::new("postgres"),
            Command::new("sqlite"),
            Command::new("xlsx"),
            Command::new("datapackage"),
        ])
        .args([
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
        ])
}
