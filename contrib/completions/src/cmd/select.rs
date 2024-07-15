use clap::{arg, Command};

pub fn select_cmd() -> Command {
    Command::new("select").args([
        arg!(--random),
        arg!(--seed),
        arg!(--sort),
        arg!(--output),
        arg!(--"no-headers"),
        arg!(--delimiter),
    ])
}
