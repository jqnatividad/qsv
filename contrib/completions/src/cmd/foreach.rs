use clap::{arg, Command};

pub fn foreach_cmd() -> Command {
    Command::new("foreach").args([
        arg!(--unify),
        arg!(--"new-column"),
        arg!(--"dry-run"),
        arg!(--"no-headers"),
        arg!(--delimiter),
        arg!(--progressbar),
    ])
}
