# Features:

`qsv` has several features:

* `mimalloc` (default) - use the mimalloc allocator (see [Memory Allocator](docs/PERFORMANCE.md#memory-allocator) for more info).
* `jemallocator` - use the jemalloc allocator (see [Memory Allocator](docs/PERFORMANCE.md#memory-allocator) for more info).
* `apply` - enable `apply` command. This swiss-army knife of CSV transformations is very powerful, but it has a lot of dependencies that increases both compile time and binary size.
* `fetch` - enables the `fetch` & `fetchpost` commands.
* `foreach` - enable `foreach` command (not valid for Windows).
* `geocode` - enable `geocode` command.
* `luau` - enable `luau` command. Embeds a [Luau](https://luau-lang.org) interpreter into qsv. [Luau has type-checking, sandboxing, additional language operators, increased performance & other improvements](https://luau-lang.org/2022/11/04/luau-origins-and-evolution.html) over Lua.
* `polars` - enables all [Polars](https://pola.rs)-powered commands (currently, `joinp` and `sqlp`). Note that Polars is a very powerful library, but it has a lot of dependencies that drastically increases both compile time and binary size.
* `python` - enable `py` command. Note that qsv will look for the shared library for the Python version (Python 3.7 & above supported) it was compiled against & will abort on startup if the library is not found, even if you're NOT using the `py` command. Check [Python](#python) section for more info.
* `to` - enables the `to` command except the parquet option.
* `to_parquet` - enables the `parquet` option of the `to` command. This is a separate feature as it brings in the `duckdb` dependency, which markedly increases binary size and compile time.
Use the `sqlp` command with the `--format parquet` option instead if you don't need the `to` command's other options and you don't need to convert to parquet a directory of CSVs.
* `self_update` - enable self-update engine, checking GitHub for the latest release. Note that if you manually built qsv, `self-update` will only check for new releases.
It will NOT offer the choice to update itself to the prebuilt binaries published on GitHub. You need not worry that your manually built qsv will be overwritten by a self-update.

* `feature_capable` - enable to build `qsv` binary variant which is feature-capable.
* `all_features` - enable to build `qsv` binary variant with all features enabled (apply,fetch,foreach,geocode,luau,polars,python,to,to_parquet,self_update).
* `lite` - enable to build `qsvlite` binary variant with all features disabled.
* `datapusher_plus` - enable to build `qsvdp` binary variant - the [DataPusher+](https://github.com/dathere/datapusher-plus) optimized qsv binary.
* `nightly` - enable to turn on nightly/unstable features in the `rand`, `regex`, `hashbrown` & `pyo3` crates when building with Rust nightly/unstable.
* `distrib_features` - enable to build `qsv` binary variant with all features enabled except `self_update`. This should make it easier for distro packagers to build `qsv` with all features enabled except `self_update` as qsv removes and adds features over time.

> ℹ️ **NOTE:** `qsvlite`, as the name implies, always has **non-default features disabled**. `qsv` can be built with any combination of the above features using the cargo `--features` & `--no-default-features` flags. The prebuilt `qsv` binaries has **all applicable features valid for the target platform**.
