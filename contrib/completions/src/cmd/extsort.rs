use clap::{arg, Command};

pub fn extsort_cmd() -> Command {
    Command::new("extsort").args([
        arg!(--"memory-limit"),
        arg!(--"tmp-dir"),
        arg!(--jobs),
        arg!(--"no-headers"),
    ])
}
