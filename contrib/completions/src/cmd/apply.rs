use clap::{arg, Command};

pub fn apply_cmd() -> Command {
    Command::new("apply")
        .subcommands([
            Command::new("operations"),
            Command::new("emptyreplace"),
            Command::new("dynfmt"),
            Command::new("calcconv"),
        ])
        .args([
            arg!(--"new-column"),
            arg!(--rename),
            arg!(--comparand),
            arg!(--replacement),
            arg!(--formatstr),
            arg!(--jobs),
            arg!(--batch),
            arg!(--output),
            arg!(--"no-headers"),
            arg!(--delimiter),
            arg!(--progressbar),
        ])
}
