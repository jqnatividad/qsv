# Interpreters

## Luau

[Luau](https://luau-lang.org) is a fast, small, safe, gradually typed, statically linked, embeddable scripting language derived from [Lua](https://www.lua.org/about.html). It lies at the [heart of Roblox technology](https://luau-lang.org/2022/11/04/luau-origins-and-evolution.html) - powering all it's user generated content, with [Roblox](https://en.wikipedia.org/wiki/Roblox)'s own internal code having more than 2 millions lines of Luau. 

It has [sandboxing](https://luau-lang.org/sandbox), [type-checking](https://luau-lang.org/typecheck), [additional operators](https://luau-lang.org/syntax) & [increased performance](https://luau-lang.org/performance) while [maintaining compatibility with Lua](https://luau-lang.org/compatibility).

[Lua is faster than Python](https://benchmarksgame-team.pages.debian.net/benchmarksgame/fastest/lua-python3.html) & Luau is even faster still - more so, as qsv precompiles Luau into bytecode. In addition, [`luau`](/src/cmd/luau.rs#L2) is embedded into qsv, has debug logging, can do aggregations with its `--begin` & `--end` options & has no external dependencies unlike the `py` command.

It also allows mapping of multiple new computed columns, supports random access with indexed CSV files, and has [several helper functions](https://github.com/jqnatividad/qsv/blob/c0c2d5ab3e4ea9cc0e861c6ad41652677ffc4f20/src/cmd/luau.rs#L1250-L1931) to help ease the development of [full-fledged data-wrangling scripts](https://github.com/jqnatividad/qsv/blob/4e521b177ea3a6a06c83222458bb1349a67606f4/tests/test_luau.rs#L524-L571).

As date manipulation is often needed, the [LuaDate](https://tieske.github.io/date/) module is also bundled.

Finally, as [qsv's DSL](../README.md#luau_deeplink) (👑), `luau` will gain even more features over time compared to the `python` feature.

[Luau 0.603](https://github.com/Roblox/luau/releases/tag/0.603) is currently embedded - qsv's policy is to use the latest stable Luau version at the time of each qsv release.

## Python

The `python` feature is NOT enabled by default on the prebuilt binaries as its dynamically linked to python libraries at runtime, which presents distribution issues, as various operating systems have differing Python versions.

If you wish to enable the `python` feature - you'll just have to install/compile from source, making sure you have the development libraries for the desired Python version (Python 3.7 and above are supported) installed when doing so (e.g. on Debian/Ubuntu - `apt-get install python-dev`; on CentOS/RedHat/Amazon Linux - `yum install python-devel`; on Windows and macOS - use the [Python installer](https://www.python.org/downloads/) for the desired version).

If you plan to distribute your manually built `qsv` with the `python` feature, `qsv` will look for the specific version of Python shared libraries (libpython* on Linux/macOS, python*.dll on Windows) against which it was compiled starting with the current directory & abort with an error if not found, detailing the Python library it was looking for. 

Note that this will happen on qsv startup, even if you're NOT running the `py` command.

When building from source - [PyO3](https://pyo3.rs) - the underlying crate that enables the `python` feature, uses a build script to determine the Python version & set the correct linker arguments. By default it uses the python3 executable.
You can override this by setting `PYO3_PYTHON` (e.g., `PYO3_PYTHON=python3.7`), before installing/compiling qsv. See the [PyO3 User Guide](https://pyo3.rs/v0.17.1/building_and_distribution.html) for more information.

Consider using the [`luau`](/src/cmd/luau.rs#L2) command instead of the [`py`]((/src/cmd/python.rs#L2)) command if the operation you're trying to do can be done with `luau` - as `luau` is statically linked, has no external dependencies, much faster than `py`, can do aggregations, supports random access, has a bevy of qsv helper functions, and allows mapping of multiple new columns. 

The `py` command cannot do aggregations because [PyO3's GIL-bound memory](https://pyo3.rs/v0.17.2/memory.html#gil-bound-memory) limitations will quickly consume a lot of memory (see [issue 449](https://github.com/jqnatividad/qsv/issues/449#issuecomment-1226095316) for details).
To prevent this, the `py` command processes CSVs in batches (default: 30,000 records), with a GIL pool for each batch, so no globals are available across batches.
