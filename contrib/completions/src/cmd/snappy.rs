use clap::{arg, Command};

pub fn snappy_cmd() -> Command {
    let global_args = [
        arg!(--"user-agent"),
        arg!(--timeout),
        arg!(--output),
        arg!(--jobs),
        arg!(--quiet),
        arg!(--progressbar),
    ];
    Command::new("snappy")
        .subcommands([
            Command::new("compress").args(&global_args),
            Command::new("decompress").args(&global_args),
            Command::new("check").args(&global_args),
            Command::new("validate").args(&global_args),
        ])
        .args(global_args)
}
