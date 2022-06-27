fn main() {
    println!(
        "cargo:rustc-env=TARGET={}",
        std::env::var("TARGET").unwrap()
    );
    println!(
        "cargo:rustc-env=QSV_KIND={}",
        std::env::var("QSV_KIND").unwrap_or_else(|_| "compiled".to_string())
    );
}
