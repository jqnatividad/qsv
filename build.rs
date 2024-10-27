fn main() {
    // we use TARGET in --version and user-agent strings
    println!(
        "cargo:rustc-env=TARGET={}",
        std::env::var("TARGET").unwrap()
    );
    // QSV_KIND is used to determine how qsv was built and is displayed in --version
    // check PERFORMANCE.md for more info
    println!(
        "cargo:rustc-env=QSV_KIND={}",
        std::env::var("QSV_KIND").unwrap_or_else(|_| "compiled".to_string())
    );

    #[cfg(feature = "polars")]
    {
        use std::{fs, path::Path};

        // This is the string that is searched for in Cargo.toml to find the Polars revision
        const QSV_POLARS_REV: &str = "# QSV_POLARS_REV=";

        // QSV_POLARS_REV contains either the commit id short hash or the git tag
        // of the Polars version qsv was built against
        let cargo_toml_path = Path::new("Cargo.toml");
        let cargo_toml_content =
            fs::read_to_string(cargo_toml_path).expect("Failed to read Cargo.toml");
        let polars_rev = cargo_toml_content.find(QSV_POLARS_REV).map_or_else(
            || "unknown".to_string(),
            |index| {
                let start_index = index + QSV_POLARS_REV.len();
                let end_index = cargo_toml_content[start_index..]
                    .find('\n')
                    .map_or(cargo_toml_content.len(), |i| start_index + i);
                let final_rev = cargo_toml_content[start_index..end_index]
                    .trim()
                    .to_string();
                if final_rev.is_empty() {
                    "release".to_string()
                } else {
                    final_rev
                }
            },
        );
        println!("cargo:rustc-env=QSV_POLARS_REV={polars_rev}");
    }
}
