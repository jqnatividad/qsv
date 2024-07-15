use clap::{arg, Command};

pub fn excel_cmd() -> Command {
    Command::new("excel").args([
        arg!(--sheet),
        arg!(--metadata),
        arg!(--"error-format"),
        arg!(--flexible),
        arg!(--trim),
        arg!(--"date-format"),
        arg!(--"keep-zero-time"),
        arg!(--"range"),
        arg!(--jobs),
        arg!(--output),
        arg!(--delimiter),
        arg!(--quiet),
    ])
}
