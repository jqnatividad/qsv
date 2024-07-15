use clap::{arg, Command};

pub fn fmt_cmd() -> Command {
    Command::new("fmt").args([
        arg!(--"out-delimiter"),
        arg!(--crlf),
        arg!(--ascii),
        arg!(--quote),
        arg!(--"quote-always"),
        arg!(--"quote-never"),
        arg!(--escape),
        arg!(--"no-final-newline"),
        arg!(--output),
        arg!(--delimiter),
    ])
}
