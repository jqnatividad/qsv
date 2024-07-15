use clap::{arg, Command};

pub fn snappy_cmd() -> Command {
    Command::new("snappy")
        .subcommands([
            Command::new("compress"),
            Command::new("decompress"),
            Command::new("check"),
            Command::new("validate"),
        ])
        .args([
            arg!(--"user-agent"),
            arg!(--timeout),
            arg!(--output),
            arg!(--jobs),
            arg!(--quiet),
            arg!(--progressbar),
        ])
}
