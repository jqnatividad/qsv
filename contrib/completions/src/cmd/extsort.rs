use clap::{arg, Command};

pub fn extsort_cmd() -> Command {
    Command::new("extsort").args([
        arg!(--select),
        arg!(--reverse),
        arg!(--"memory-limit"),
        arg!(--"tmp-dir"),
        arg!(--jobs),
        arg!(--delimiter),
        arg!(--"no-headers"),
    ])
}
