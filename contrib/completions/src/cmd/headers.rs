use clap::{arg, Command};

pub fn headers_cmd() -> Command {
    Command::new("headers").args([
        arg!(--"just-names"),
        arg!(--intersect),
        arg!(--trim),
        arg!(--delimiter),
    ])
}
