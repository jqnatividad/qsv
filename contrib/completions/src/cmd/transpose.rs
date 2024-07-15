use clap::{arg, Command};

pub fn transpose_cmd() -> Command {
    Command::new("transpose").args([
        arg!(--multipass),
        arg!(--output),
        arg!(--delimiter),
        arg!(--memcheck),
    ])
}
