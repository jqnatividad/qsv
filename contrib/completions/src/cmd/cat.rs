use clap::{arg, Command};

pub fn cat_cmd() -> Command {
    let global_args = vec![arg!(--output), arg!(--"no-headers"), arg!(--delimiter)];
    Command::new("cat")
        .subcommands([
            Command::new("rows")
                .args([arg!(--flexible)])
                .args(&global_args),
            Command::new("rowskey")
                .args([arg!(--group), arg!(--"group-name")])
                .args(&global_args),
            Command::new("columns")
                .args([arg!(--pad)])
                .args(&global_args),
        ])
        .args(global_args)
}
