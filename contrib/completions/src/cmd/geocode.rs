use clap::{arg, Command};

pub fn geocode_cmd() -> Command {
    let global_args = [
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
    ];
    Command::new("geocode")
        .subcommands([
            Command::new("suggest").args(&global_args),
            Command::new("suggestnow").args(&global_args),
            Command::new("reverse").args(&global_args),
            Command::new("reversenow").args(&global_args),
            Command::new("countryinfo").args(&global_args),
            Command::new("countryinfonow").args(&global_args),
            Command::new("index-load").args(&global_args),
            Command::new("index-check").args(&global_args),
            Command::new("index-update").args(&global_args),
            Command::new("index-reset").args(&global_args),
        ])
        .args(global_args)
}
