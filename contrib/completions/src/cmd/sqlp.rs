use clap::{arg, Command};

pub fn sqlp_cmd() -> Command {
    Command::new("sqlp").args([
        arg!(--format),
        arg!(--"try-parsedates"),
        arg!(--"infer-len"),
        arg!(--streaming),
        arg!(--"low-memory"),
        arg!(--"no-optimizations"),
        arg!(--"truncate-ragged-lines"),
        arg!(--"ignore-errors"),
        arg!(--"rnull-values"),
        arg!(--"decimal-comma"),
        arg!(--"datetime-format"),
        arg!(--"date-format"),
        arg!(--"time-format"),
        arg!(--"float-precision"),
        arg!(--"wnull-value"),
        arg!(--"compression"),
        arg!(--"compress-level"),
        arg!(--statistics),
        arg!(--output),
        arg!(--delimiter),
        arg!(--quiet),
    ])
}
