use clap::{arg, Command};

pub fn replace_cmd() -> Command {
    Command::new("replace").args([
        arg!(--"ignore-case"),
        arg!(--select),
        arg!(--unicode),
        arg!(--"size-limit"),
        arg!(--"dfa-size-limit"),
        arg!(--output),
        arg!(--"no-headers"),
        arg!(--delimiter),
        arg!(--progressbar),
        arg!(--quiet),
    ])
}
