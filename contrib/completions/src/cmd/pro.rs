use clap::Command;

pub fn pro_cmd() -> Command {
    Command::new("pro").subcommands([Command::new("lens"), Command::new("workflow")])
}
