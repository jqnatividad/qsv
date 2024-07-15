use clap::{arg, Command};

pub fn sample_cmd() -> Command {
    Command::new("sample").args([
        arg!(--seed),
        arg!(--rng),
        arg!(--"user-agent"),
        arg!(--timeout),
        arg!(--output),
        arg!(--"no-headers"),
        arg!(--delimiter),
    ])
}
