use clap::{arg, Command};

pub fn apply_cmd() -> Command {
    let global_args = [
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
    ];
    Command::new("apply")
        .subcommands([
            Command::new("operations").args(&global_args),
            Command::new("emptyreplace").args(&global_args),
            Command::new("dynfmt").args(&global_args),
            Command::new("calcconv").args(&global_args),
        ])
        .args(global_args)
}
