use clap::{arg, Command};

pub fn py_cmd() -> Command {
    Command::new("py").args([
        arg!(--helper),
        arg!(--batch),
        arg!(--output),
        arg!(--"no-headers"),
        arg!(--delimiter),
        arg!(--progressbar),
    ])
}
