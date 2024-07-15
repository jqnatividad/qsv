use clap::{arg, Command};

pub fn datefmt_cmd() -> Command {
    Command::new("datefmt").args([
        arg!(--formatstr),
        arg!(--"new-column"),
        arg!(--rename),
        arg!(--"prefer-dmy"),
        arg!(--"keep-zero-time"),
        arg!(--"input-tz"),
        arg!(--"output-tz"),
        arg!(--"default-tz"),
        arg!(--utc),
        arg!(--zulu),
        arg!(--"ts-resolution"),
        arg!(--jobs),
        arg!(--batch),
        arg!(--output),
        arg!(--"no-headers"),
        arg!(--delimiter),
        arg!(--progressbar),
    ])
}
