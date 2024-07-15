use clap::{arg, Command};

pub fn cat_cmd() -> Command {
    Command::new("cat")
        .subcommands([
            Command::new("rows").args([arg!(--flexible)]),
            Command::new("rowskey").args([arg!(--group), arg!(--"group-name")]),
            Command::new("columns").args([arg!(--pad)]),
        ])
        .args([arg!(--output), arg!(--"no-headers"), arg!(--delimiter)])
}
