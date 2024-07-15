use clap::{arg, Command};

pub fn input_cmd() -> Command {
    Command::new("input").args([
        arg!(--quote),
        arg!(--escape),
        arg!(--"no-quoting"),
        arg!(--"quote-style"),
        arg!(--"skip-lines"),
        arg!(--"auto-skip"),
        arg!(--"skip-lastlines"),
        arg!(--"trim-headers"),
        arg!(--"trim-fields"),
        arg!(--comment),
        arg!(--"encoding-errors"),
        arg!(--output),
        arg!(--delimiter),
    ])
}
