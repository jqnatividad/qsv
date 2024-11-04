use clap::{arg, Command};

pub fn template_cmd() -> Command {
    Command::new("template").args([
        arg!(--"template"),
        arg!(--"template-file"),
        arg!(--"outfilename"),
        arg!(--"customfilter-error"),
        arg!(--output),
        arg!(--"no-headers"),
        arg!(--delimiter),
    ])
}
