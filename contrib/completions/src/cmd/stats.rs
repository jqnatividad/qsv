use clap::{arg, Command};

pub fn stats_cmd() -> Command {
    Command::new("stats").args([
        arg!(--select),
        arg!(--everything),
        arg!(--typesonly),
        arg!(--"infer-boolean"),
        arg!(--mode),
        arg!(--cardinality),
        arg!(--median),
        arg!(--mad),
        arg!(--quartiles),
        arg!(--round),
        arg!(--nulls),
        arg!(--"infer-dates"),
        arg!(--"dates-whitelist"),
        arg!(--"prefer-dmy"),
        arg!(--force),
        arg!(--jobs),
        arg!(--"stats-jsonl"),
        arg!(--"cache-threshold"),
        arg!(--output),
        arg!(--"no-headers"),
        arg!(--delimiter),
        arg!(--memcheck),
    ])
}
