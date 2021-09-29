extern crate rustc_version;

use rustc_version::{version, Version};
use std::process::{exit, Command};

fn main() {
    if version().unwrap() < Version::parse("1.50.0").unwrap() {
        println!("qsv requires rustc >= 1.50.0.");
        exit(1);
    }

    let _output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", "python --version"])
            .output()
            .expect("Python required.")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("python --version")
            .output()
            .expect("Python required.")
    };
}
