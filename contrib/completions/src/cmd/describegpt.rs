use clap::{arg, Command};

pub fn describegpt_cmd() -> Command {
    Command::new("describegpt").args([
        arg!(--all),
        arg!(--description),
        arg!(--dictionary),
        arg!(--tags),
        arg!(--"api-key"),
        arg!(--"max-tokens"),
        arg!(--json),
        arg!(--jsonl),
        arg!(--prompt),
        arg!(--"prompt-file"),
        arg!(--"base-url"),
        arg!(--model),
        arg!(--timeout),
        arg!(--"user-agent"),
        arg!(--output),
        arg!(--quiet),
    ])
}
