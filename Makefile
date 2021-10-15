all:
	@echo Nothing to do...

ctags:
	ctags --recurse --options=ctags.rust --languages=Rust

docs:
	cargo doc
	in-dir ./target/doc fix-perms

debug:
	cargo build --verbose
	rustc -L ./target/deps/ -g -Z lto --opt-level 3 src/main.rs

push:
	git push home master
	git push origin master

dev:
	cargo build
	cp ./target/qsv ~/bin/bin/qsv

release:
	cargo build --release
	mkdir -p ~/bin/bin
	cp ./target/release/qsv ~/bin/bin/qsv
