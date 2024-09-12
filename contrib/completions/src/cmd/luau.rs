use clap::{arg, Command};

pub fn luau_cmd() -> Command {
    let global_args = [
        arg!(--"no-globals"),
        arg!(--colindex),
        arg!(--remap),
        arg!(--begin),
        arg!(--end),
        arg!(--"luau-path"),
        arg!(--"max-errors"),
        arg!(--timeout),
        arg!(--"ckan-api"),
        arg!(--"ckan-token"),
        arg!(--"cache-dir"),
        arg!(--output),
        arg!(--"no-headers"),
        arg!(--delimiter),
        arg!(--progressbar),
    ];
    Command::new("luau")
        .subcommands([
            Command::new("map").args(&global_args),
            Command::new("filter").args(&global_args),
        ])
        .args(global_args)
}
