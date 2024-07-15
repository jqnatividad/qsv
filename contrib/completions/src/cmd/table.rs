use clap::{arg, Command};

pub fn table_cmd() -> Command {
    Command::new("table").args([
        arg!(--width),
        arg!(--pad),
        arg!(--align),
        arg!(--condense),
        arg!(--output),
        arg!(--delimiter),
        arg!(--memcheck),
    ])
}
