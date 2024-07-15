use clap::{arg, Command};

pub fn prompt_cmd() -> Command {
    Command::new("prompt").args([
        arg!(--msg),
        arg!(--filters),
        arg!(--workdir),
        arg!(--"fd-output"),
        arg!(--"save-fname"),
        arg!(--"base-delay-ms"),
        arg!(--output),
        arg!(--quiet),
    ])
}
