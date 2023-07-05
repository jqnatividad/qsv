# Contributing

We welcome and encourage all contributions to the project. Please read the following guidelines before submitting a pull request.

## Code Contributions

For code contributions, we follow several conventions:

* Please run `cargo +nightly fmt` before submitting a pull request. We use [rustfmt settings](https://github.com/jqnatividad/qsv/blob/master/rustfmt.toml) that require nightly.
* Please run `cargo +nightly clippy -F all_features -- -W clippy::perf` before submitting a pull request. The project has its clippy preferences set [here](https://github.com/jqnatividad/qsv/blob/bb4f4c7d683719a30f5e9552d16fba96a6872ce9/src/main.rs#L1-L36), and we generally apply clippy's suggestions with those preferences unless there is a good reason not to.   
In that case, our practice is to suppress lints for each specific instance with an optional comment so it does not show up again in future clippy runs, e.g:   
```rust 
#[allow(clippy::unused_assignments)]
let var_a = String::with_capacity(10); // amortize allocation
```
* Ensure you have the latest version of Rust installed. We use the latest stable version of Rust, and the latest nightly version of Rust for clippy and rustfmt. In particular, running `cargo +nightly fmt` and `cargo +nightly clippy` may return different results if you are not using the latest nightly version of Rust.
* We use docopt for command line argument parsing as we fully take advantage of its ability to parse command line arguments from the contiguous, verbose usage text that is at the beginning of each command's source code that more popular libraries like clap or structopt do not offer.   
However, since [docopt.rs is unmaintained](https://github.com/docopt/docopt.rs#this-crate-is-unmaintained), we have a [fork](https://github.com/jqnatividad/docopt.rs) that will be maintained along with this project. See this [discussion thread](https://github.com/jqnatividad/qsv/discussions/463) for more details.
* `unwrap()` and `expect()` are allowed, but there should be an accompanying comment detailing safety
* TODO: explain testing conventions, and test helpers
* TODO: explain error handling conventions
* TODO: explain logging conventions
* TODO: release practices
* TODO: explain the various GitHub Action workflows

## Recipe Contributions

We also welcome and highly encourage recipe contributions!

The recipes need not be all that complicated or use `qsv` exclusively (feel free to mix and match qsv with other CLI tools) but they should be useful and not trivial. We also ask that you include a short description of the recipe, and a link to the source of the recipe if it is not your own.

Just go to the [Cookbook](https://github.com/jqnatividad/qsv/wiki/Cookbook#cookbook) and add your recipe.

## Documentation Contributions

We are always looking for ways to improve the documentation. If you find a typo, or have a suggestion for improvement, please submit a pull request.

And if you want to add a new page to the Wiki, please do! Just make sure to add section headers to your Wiki contributions, so it automatically shows up in the sidebar.
