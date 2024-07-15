use clap::{arg, Command};

pub fn frequency_cmd() -> Command {
    Command::new("frequency").args([
        arg!(--select),
        arg!(--limit),
        arg!(--"unq-limit"),
        arg!(--"lmt-threshold"),
        arg!(--"pct-dec-places"),
        arg!(--"other-sorted"),
        arg!(--"other-text"),
        arg!(--asc),
        arg!(--"no-trim"),
        arg!(--"ignore-case"),
        arg!(--jobs),
        arg!(--output),
        arg!(--"no-headers"),
        arg!(--delimiter),
        arg!(--memcheck),
    ])
}
