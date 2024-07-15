use clap::{arg, Command};

pub fn geocode_cmd() -> Command {
    Command::new("geocode").args([
        arg!(--"new-column"),
        arg!(--rename),
        arg!(--country),
        arg!(--"min-score"),
        arg!(--admin1),
        arg!(--"k_weight"),
        arg!(--formatstr),
        arg!(--language),
        arg!(--"invalid-result"),
        arg!(--jobs),
        arg!(--batch),
        arg!(--timeout),
        arg!(--"cache-dir"),
        arg!(--languages),
        arg!(--"cities-url"),
        arg!(--force),
        arg!(--output),
        arg!(--delimiter),
        arg!(--progressbar),
    ])
}
