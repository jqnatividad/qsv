# Features:

`qsv` has several features:

* `mimalloc` (default) - use the mimalloc allocator (see [Memory Allocator](./PERFORMANCE.md#memory-allocator) for more info).
* `jemallocator` - use the jemalloc allocator (see [Memory Allocator](./PERFORMANCE.md#memory-allocator) for more info).
* `apply` - enable `apply` command. This swiss-army knife of CSV transformations is very powerful, but it has a lot of dependencies that increases both compile time and binary size.
* `fetch` - enables the `fetch` & `fetchpost` commands.
* `foreach` - enable `foreach` command.
* `geocode` - enable `geocode` command.
* `lens` - enable `lens` command.
* `luau` - enable `luau` command. Embeds a [Luau](https://luau-lang.org) interpreter into qsv. [Luau has type-checking, sandboxing, additional language operators, increased performance & other improvements](https://luau-lang.org/2022/11/04/luau-origins-and-evolution.html) over Lua. Luau is the DSL of qsv - as its statically linked, has a MUCH smaller footprint (in both file size and memory without having to deal with Python's infamous [Global Interpreter Lock](https://wiki.python.org/moin/GlobalInterpreterLock)) & is faster (in both startup & execution time) than Python.
* `polars` - enables all [Polars](https://pola.rs)-powered commands (currently, `joinp` and `sqlp`. Also enables polars mode in `count`). Note that Polars is a very powerful library, but it has a lot of dependencies that drastically increases both compile time and binary size.
* `prompt` - enable `prompt` command.
* `python` - enable `py` command. Note that qsv will look for the shared library for the Python version (Python 3.8 & above supported) it was compiled against & will abort on startup if the library is not found, even if you're NOT using the `py` command. Check [Python](#python) section for more info. Though Luau is the preferred DSL for qsv for all the reasons stated above, Python is still the lingua franca of data wrangling.
* `to` - enables the `to` command.
* `self_update` - enable self-update engine, checking GitHub for the latest release. Note that if you manually built qsv, `self-update` will only alert you about new releases (it checks GitHub for the latest release 10% of the time upon startup unless the `QSV_NO_UPDATE` environment variable is set). It will NOT offer the choice to update itself to the prebuilt binaries published on GitHub.  
You need not worry that your manually built qsv will be overwritten by a self-update.  
To check if your qsv build will have the option to self-update, run `qsv --version`. If you see `self_update` in the enabled features, and QSV_KIND is `prebuilt*` at the end, then you have the option to self-update.
* `ui` - enables commands that require linking UI libraries - `clipboard`, `lens` and `prompt`. Disable this feature if you're building for a headless environment. Note that `qsvdp` and `qsvlite` does not enable the `ui` feature by default.

## Special Features for building qsv binary variants:

* `feature_capable` - enable to build `qsv` binary variant which is feature-capable. (mutually exclusive with `lite` and `datapusher_plus`)
  * `all_features` - shortcut to build `qsv` binary variant with all features enabled (apply,fetch,foreach,geocode,luau,polars,python,to,self_update,ui).

* `lite` - enable to build `qsvlite` binary variant with all features disabled. (mutually exclusive with `feature_capable` and `datapusher_plus`)
* `datapusher_plus` - enable to build `qsvdp` binary variant - the [DataPusher+](https://github.com/dathere/datapusher-plus) optimized qsv binary. (mutually exclusive with `feature_capable` and `lite`)
* `nightly` - enable to turn on nightly/unstable features in the `crc32fast`, `hashbrown`, `polars`, `pyo3` & `rand` crates when building with Rust nightly/unstable.
* `distrib_features` - enable to build `qsv` binary variant with all features enabled except `self_update`. This should make it easier for distro packagers to build `qsv` with all features enabled except `self_update` as qsv removes and adds features over time.

> ℹ️ **NOTE:** `qsvlite`, as the name implies, always has **non-default features disabled**. `qsv` can be built with any combination of the above features using the cargo `--features` & `--no-default-features` flags. The prebuilt `qsv` binaries has **all applicable features valid for the target platform**.
