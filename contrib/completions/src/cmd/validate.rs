use clap::{arg, Command};

pub fn validate_cmd() -> Command {
    Command::new("validate").args([
        arg!(--trim),
        arg!(--"fail-fast"),
        arg!(--valid),
        arg!(--invalid),
        arg!(--json),
        arg!(--"pretty-json"),
        arg!(--"valid-output"),
        arg!(--jobs),
        arg!(--batch),
        arg!(--timeout),
        arg!(--"no-headers"),
        arg!(--delimiter),
        arg!(--progressbar),
        arg!(--quiet),
    ])
}
