use clap::{arg, Command};

pub fn tojsonl_cmd() -> Command {
    Command::new("tojsonl").args([
        arg!(--trim),
        arg!(--"no-boolean"),
        arg!(--jobs),
        arg!(--batch),
        arg!(--delimiter),
        arg!(--output),
        arg!(--memcheck),
    ])
}
