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
}
