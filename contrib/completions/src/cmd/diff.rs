use clap::{arg, Command};

pub fn diff_cmd() -> Command {
    Command::new("diff").args([
        arg!(--"no-headers-left"),
        arg!(--"no-headers-right"),
        arg!(--"no-headers-output"),
        arg!(--"delimiter-left"),
        arg!(--"delimiter-right"),
        arg!(--"delimiter-output"),
        arg!(--key),
        arg!(--"sort-columns"),
        arg!(--jobs),
        arg!(--output),
    ])
}
