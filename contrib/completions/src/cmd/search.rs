use clap::{arg, Command};

pub fn search_cmd() -> Command {
    Command::new("search").args([
        arg!(--"ignore-case"),
        arg!(--select),
        arg!(--"invert-match"),
        arg!(--unicode),
        arg!(--flag),
        arg!(--quick),
        arg!(--"preview-match"),
        arg!(--count),
        arg!(--"size-limit"),
        arg!(--"dfa-size-limit"),
        arg!(--json),
        arg!(--"not-one"),
        arg!(--output),
        arg!(--"no-headers"),
        arg!(--delimiter),
        arg!(--progressbar),
        arg!(--quiet),
    ])
}
