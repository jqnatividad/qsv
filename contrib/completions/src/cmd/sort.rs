use clap::{arg, Command};

pub fn sort_cmd() -> Command {
    Command::new("sort").args([
        arg!(--select),
        arg!(--numeric),
        arg!(--reverse),
        arg!(--"ignore-case"),
        arg!(--unique),
        arg!(--random),
        arg!(--seed),
        arg!(--rng),
        arg!(--jobs),
        arg!(--faster),
        arg!(--output),
        arg!(--"no-headers"),
        arg!(--delimiter),
        arg!(--memcheck),
    ])
}
