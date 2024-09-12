use clap::{arg, Command};

pub fn py_cmd() -> Command {
    let global_args = [
        arg!(--helper),
        arg!(--batch),
        arg!(--output),
        arg!(--"no-headers"),
        arg!(--delimiter),
        arg!(--progressbar),
    ];
    Command::new("py")
        .subcommands([
            Command::new("map").args(&global_args),
            Command::new("filter").args(&global_args),
        ])
        .args(global_args)
}
