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
        arg!(--"prefer-dmy"),
        arg!(--force),
        arg!(--jobs),
        arg!(--"stats-binout"),
        arg!(--"cache-threshold"),
        arg!(--output),
        arg!(--"no-headers"),
        arg!(--delimiter),
        arg!(--memcheck),
    ])
}
