use clap::{arg, Command};

pub fn sniff_cmd() -> Command {
    Command::new("sniff").args([
        arg!(--sample),
        arg!(--"prefer-dmy"),
        arg!(--delimiter),
        arg!(--quote),
        arg!(--json),
        arg!(--"pretty-json"),
        arg!(--"save-urlsample"),
        arg!(--timeout),
        arg!(--"user-agent"),
        arg!(--"stats-types"),
        arg!(--"no-infer"),
        arg!(--"just-mime"),
        arg!(--quick),
        arg!(--"harvest-mode"),
        arg!(--progressbar),
    ])
}
