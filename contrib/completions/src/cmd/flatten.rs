use clap::{arg, Command};

pub fn flatten_cmd() -> Command {
    Command::new("flatten").args([
        arg!(--condense),
        arg!(--"field-separator"),
        arg!(--separator),
        arg!(--"no-headers"),
        arg!(--delimiter),
    ])
}
