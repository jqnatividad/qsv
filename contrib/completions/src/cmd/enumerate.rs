use clap::{arg, Command};

pub fn enum_cmd() -> Command {
    Command::new("enum").args([
        arg!(--"new-column"),
        arg!(--start),
        arg!(--increment),
        arg!(--constant),
        arg!(--copy),
        arg!(--uuid4),
        arg!(--uuid7),
        arg!(--hash),
        arg!(--output),
        arg!(--"no-headers"),
        arg!(--delimiter),
    ])
}
