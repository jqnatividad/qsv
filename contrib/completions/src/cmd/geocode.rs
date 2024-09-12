use clap::{arg, Command};

pub fn geocode_cmd() -> Command {
    Command::new("geocode")
        .subcommands([
            Command::new("suggest"),
            Command::new("suggestnow"),
            Command::new("reverse"),
            Command::new("reversenow"),
            Command::new("countryinfo"),
            Command::new("countryinfonow"),
            Command::new("index-load"),
            Command::new("index-check"),
            Command::new("index-update"),
            Command::new("index-reset"),
        ])
        .args([
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
