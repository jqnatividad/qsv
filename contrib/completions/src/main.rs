mod cli;
mod cmd;
use clap_complete::{
    generate,
    shells::{Bash, Elvish, Fish, PowerShell, Zsh},
};
use clap_complete_fig::Fig;
use clap_complete_nushell::Nushell;
use std::{io, process::exit};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let args_error =
        "Please provide an argument of either: bash, zsh, fish, powershell, nushell, fig, elvish";
    if args.len() != 2 {
        println!("{args_error}");
        exit(1);
    }
    // generate(Bash, &mut cli::build_cli(), "qsv", &mut io::stdout());
    let first_arg = args[1].as_str();
    match first_arg {
        "bash" => generate(Bash, &mut cli::build_cli(), "qsv", &mut io::stdout()),
        "zsh" => generate(Zsh, &mut cli::build_cli(), "qsv", &mut io::stdout()),
        "fish" => generate(Fish, &mut cli::build_cli(), "qsv", &mut io::stdout()),
        "powershell" => generate(PowerShell, &mut cli::build_cli(), "qsv", &mut io::stdout()),
        "nushell" => generate(Nushell, &mut cli::build_cli(), "qsv", &mut io::stdout()),
        "fig" => generate(Fig, &mut cli::build_cli(), "qsv", &mut io::stdout()),
        "elvish" => generate(Elvish, &mut cli::build_cli(), "qsv", &mut io::stdout()),
        _ => println!("{args_error}"),
    }
}
