use clap::{arg, Command};

pub fn dedup_cmd() -> Command {
    Command::new("dedup").args([
        arg!(--select),
        arg!(--numeric),
        arg!(--"ignore-case"),
        arg!(--sorted),
        arg!(--"dupes-output"),
        arg!(--"human-readable"),
        arg!(--jobs),
        arg!(--output),
        arg!(--"no-headers"),
        arg!(--delimiter),
        arg!(--quiet),
        arg!(--memcheck),
    ])
}
