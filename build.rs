extern crate rustc_version;

use rustc_version::{version, Version};
use std::process::exit;

fn main() {
    if version().unwrap() < Version::parse("1.50.0").unwrap() {
        println!("qsv requires rustc >= 1.50.0.");
        exit(1);
    }
}
