## qsv: Ultra-fast CSV data-wrangling CLI toolkit   
[![Linux build status](https://github.com/jqnatividad/qsv/actions/workflows/rust.yml/badge.svg)](https://github.com/jqnatividad/qsv/actions/workflows/rust.yml)
[![Windows build status](https://github.com/jqnatividad/qsv/actions/workflows/rust-windows.yml/badge.svg)](https://github.com/jqnatividad/qsv/actions/workflows/rust-windows.yml)
[![macOS build status](https://github.com/jqnatividad/qsv/actions/workflows/rust-macos.yml/badge.svg)](https://github.com/jqnatividad/qsv/actions/workflows/rust-macos.yml)
[![Security audit](https://github.com/jqnatividad/qsv/actions/workflows/security-audit.yml/badge.svg)](https://github.com/jqnatividad/qsv/actions/workflows/security-audit.yml)
[![Crates.io](https://img.shields.io/crates/v/qsv.svg)](https://crates.io/crates/qsv)
[![Minimum supported Rust version](https://img.shields.io/badge/Rust-1.58.1-red?logo=rust)](#minimum-supported-rust-version)
[![Discussions](https://img.shields.io/github/discussions/jqnatividad/qsv)](https://github.com/jqnatividad/qsv/discussions)
[![Docs](https://img.shields.io/badge/wiki-docs-yellowgreen)](https://github.com/jqnatividad/qsv/wiki)
[![Downloads](https://img.shields.io/github/downloads/jqnatividad/qsv/total)](https://github.com/jqnatividad/qsv/releases/latest)   
qsv is a command line program for indexing, slicing, analyzing, splitting, enriching, validating & joining
CSV files. Commands are simple, fast and composable.

* [Available Commands](#available-commands)
* [Installation](#installation)
* [Whirlwind Tour](docs/whirlwind_tour.md#a-whirlwind-tour)
* [Cookbook](https://github.com/jqnatividad/qsv/wiki)
* [FAQ](https://github.com/jqnatividad/qsv/wiki/FAQ)
* [Benchmarks](docs/BENCHMARKS.md)
* [Sponsor](#sponsor)

> **NOTE:** qsv is a fork of the popular [xsv](https://github.com/BurntSushi/xsv) utility, merging several pending PRs [since xsv 0.13.0's release](https://github.com/BurntSushi/xsv/issues/267), along with additional features & commands for data-wrangling. See [FAQ](https://github.com/jqnatividad/qsv/wiki/FAQ) for more details.

Available commands
------------------
| Command | Description |
| --- | --- |
| [apply](/src/cmd/apply.rs#L25)[^1] | Apply series of string, date, currency & geocoding transformations to a CSV column. It also has some basic NLP functions ([similarity](https://crates.io/crates/strsim), [sentiment analysis](https://crates.io/crates/vader_sentiment), [profanity](https://docs.rs/censor/latest/censor/), [eudex](https://github.com/ticki/eudex#eudex-a-blazingly-fast-phonetic-reductionhashing-algorithm) & [language detection](https://crates.io/crates/whatlang)).  |
| [behead](/src/cmd/behead.rs#L7) | Drop headers from a CSV.  |
| [cat](/src/cmd/cat.rs#L7) | Concatenate CSV files by row or by column. |
| [count](/src/cmd/count.rs#L7)[^2] | Count the rows in a CSV file. (Instantaneous with an index.) |
| [dedup](/src/cmd/dedup.rs#L13)[^3] | Remove redundant rows.  |
| [enum](/src/cmd/enumerate.rs#L10) | Add a new column enumerating rows by adding a column of incremental or uuid identifiers. Can also be used to copy a column or fill a new column with a constant value.  |
| [exclude](/src/cmd/exclude.rs#L17)[^2] | Removes a set of CSV data from another set based on the specified columns.  |
| [explode](/src/cmd/explode.rs#L8) | Explode rows into multiple ones by splitting a column value based on the given separator.  |
| [fetch](/src/cmd/fetch.rs#L10) | Fetches HTML/data from web pages or web services for every row in a URL column. |
| [fill](/src/cmd/fill.rs#L13) | Fill empty values.  |
| [fixlengths](/src/cmd/fixlengths.rs#L9) | Force a CSV to have same-length records by either padding or truncating them. Can also be used to inspect if a CSV is well-formed. |
| [flatten](/src/cmd/flatten.rs#L12) | A flattened view of CSV records. Useful for viewing one record at a time.<br />e.g. `qsv slice -i 5 data.csv \| qsv flatten`. |
| [fmt](/src/cmd/fmt.rs#L7) | Reformat a CSV with different delimiters, record terminators or quoting rules. (Supports ASCII delimited data.)  |
| [foreach](/src/cmd/foreach.rs#L17)[^1] | Loop over a CSV to execute bash commands. (not available on Windows)  |
| [frequency](/src/cmd/frequency.rs#L15)[^2][^4] | Build frequency tables of each column. (Uses multithreading to go faster if an index is present.) |
| [generate](/src/cmd/generate.rs#L12)[^1] | Generate test data by profiling a CSV using [Markov decision process](https://crates.io/crates/test-data-generation) machine learning.  |
| [headers](/src/cmd/headers.rs#L11) | Show the headers of a CSV. Or show the intersection of all headers between many CSV files. |
| [index](/src/cmd/index.rs#L13) | Create an index for a CSV. This is very quick & provides constant time indexing into the CSV file. |
| [input](/src/cmd/input.rs#L7) | Read a CSV with exotic quoting/escaping rules. |
| [join](/src/cmd/join.rs#L17)[^2] | Inner, outer, cross, anti & semi joins. Uses a simple hash index to make it fast.  |
| [jsonl](/src/cmd/jsonl.rs#L11) | Convert newline-delimited JSON ([JSONL](https://jsonlines.org/)/[NDJSON](http://ndjson.org/)) to CSV. 
| [lua](/src/cmd/lua.rs#L15)[^1] | Execute a [Lua](https://www.lua.org/about.html) script over CSV lines to transform, aggregate or filter them.  |
| [partition](/src/cmd/partition.rs#L16) | Partition a CSV based on a column value. |
| [pseudo](/src/cmd/pseudo.rs#L10) | Pseudonymise the value of the given column by replacing them with an incremental identifier.  |
| [py](/src/cmd/python.rs#L45)[^1] | Evaluate a Python expression over CSV lines to transform, aggregate or filter them. Python's [f-strings](https://www.freecodecamp.org/news/python-f-strings-tutorial-how-to-use-f-strings-for-string-formatting/) is particularly useful for extended formatting (Python 3.7+ required).  |
| [rename](/src/cmd/rename.rs#L7) |  Rename the columns of a CSV efficiently.  |
| [replace](/src/cmd/replace.rs#L12) | Replace CSV data using a regex.  |
| [reverse](/src/cmd/reverse.rs#L7)[^3] | Reverse order of rows in a CSV. Unlike the `sort --reverse` command, it preserves the order of rows with the same key.  |
| [sample](/src/cmd/sample.rs#L12)[^2] | Randomly draw rows (with optional seed) from a CSV using [reservoir sampling](https://en.wikipedia.org/wiki/Reservoir_sampling) (i.e., use memory proportional to the size of the sample).  |
| [search](/src/cmd/search.rs#L11) | Run a regex over a CSV. Applies the regex to each field individually & shows only matching rows.  |
| [searchset](/src/cmd/searchset.rs#L15) | Run multiple regexes over a CSV in a single pass. Applies the regexes to each field individually & shows only matching rows.  |
| [select](/src/cmd/select.rs#L8) | Select, re-order, duplicate or drop columns.  |
| [slice](/src/cmd/slice.rs#L10)[^2][^3] | Slice rows from any part of a CSV. When an index is present, this only has to parse the rows in the slice (instead of all rows leading up to the start of the slice).  |
| [sort](/src/cmd/sort.rs#L14) | Sorts CSV data in alphabetical, numerical, reverse or random (with optional seed) order.  |
| [split](/src/cmd/split.rs#L14)[^2][^4] | Split one CSV file into many CSV files of N chunks.  |
| [stats](/src/cmd/stats.rs#L24)[^2][^3][^4] | Infer data type & compute descriptive statistics for each column in a CSV (sum, min/max, min/max length, mean, stddev, variance, quartiles, IQR, lower/upper fences, skew, median, mode, cardinality & nullcount)  |
| [table](/src/cmd/table.rs#L12)[^3] | Show aligned output of a CSV using [elastic tabstops](https://github.com/BurntSushi/tabwriter).  |
| [transpose](/src/cmd/transpose.rs#L9)[^3] | Transpose rows/columns of a CSV.  |

[^1]: enabled by optional feature flag. Not available on `qsvlite`.   
[^2]: uses an index when available. `join` always uses indices.   
[^3]: loads the entire CSV into memory. Note that `stats` & `transpose` have modes that do not load the entire CSV into memory.   
[^4]: multithreaded by default (use `--jobs` option to adjust).   

Installation
------------
Pre-built binaries for Windows, Linux and macOS are available [from GitHub](https://github.com/jqnatividad/qsv/releases/latest).

There are two versions of qsv. `qsvlite` has all features disabled. `qsv` supports features, with the pre-built binaries
enabling all valid platform features[^5].

Alternatively, you can compile from source by
[installing Cargo](https://crates.io/install)
([Rust's](https://www.rust-lang.org/) package manager)
and installing `qsv` using Cargo:

```bash
cargo install qsv --path .
```

If you encounter compilation errors, ensure you're using the exact
version of the dependencies qsv was built with by issuing:

```bash
cargo install qsv --path . --frozen
```

Compiling from this repository also works similarly:

```bash
git clone git://github.com/jqnatividad/qsv
cd qsv
cargo build --release
# or if you encounter compilation errors
cargo build --release --frozen
```

The compiled binary will end up in `./target/release/qsv`.

To enable optional features, use the `--features` or `--all-features` options (see [Feature Flags](#feature-flags) for more info if you're compiling/installing a custom build of qsv):

```bash
cargo install qsv --features apply,generate,lua,foreach,python
# or
cargo install qsv --all-features

# or when compiling from a local repo
cargo build --release --features apply,generate,lua,foreach,python
# or
cargo build --release --all-features
```

[^5]: The `foreach` feature is not available on Windows. The `python` feature is not enabled on cross-compiled pre-built binaries as we don't have
access to a native python interpreter for those platforms (aarch64, i686, and arm) on GitHub's action runners. Compile natively on those platforms with Python 3.7+ installed, if you want to enable the `python` feature.
### Minimum Supported Rust Version
Building qsv requires Rust version 1.58.1.

Tab Completion
--------------
qsv's command-line options are quite extensive. Thankfully, since it uses [docopt](http://docopt.org/) for CLI processing,
we can take advantage of [docopt.rs' tab completion support](https://github.com/docopt/docopt.rs#tab-completion-support) to make it
easier to use qsv at the command-line (currently, only bash shell is supported):

```bash
# install docopt-wordlist
cargo install docopt

# IMPORTANT: run these commands from the root directory of your qsv git repository
# to setup bash qsv tab completion
echo "DOCOPT_WORDLIST_BIN=\"$(which docopt-wordlist)"\" >> $HOME/.bash_completion
echo "source \"$(pwd)/scripts/docopt-wordlist.bash\"" >> $HOME/.bash_completion
echo "complete -F _docopt_wordlist_commands qsv" >> $HOME/.bash_completion
```

Recognized file formats
-----------------------
qsv recognizes CSV (`.csv` file extension) and TSV files (`.tsv` and `.tab` file extensions). CSV files are assummed to have "," (comma) as a delimiter,
and TSV files, "\t" (tab) as a delimiter. The delimiter is a single ascii character that can be set either by the `--delimiter` command-line option or
with the `QSV_DEFAULT_DELIMITER` environment variable.

[JSONL](https://jsonlines.org/)/[NDJSON](http://ndjson.org/) files are also recognized and converted to CSV with the [`jsonl`](/src/cmd/jsonl.rs#L11) command.

Environment Variables
---------------------

* `QSV_DEFAULT_DELIMITER` - single ascii character to use as delimiter.  Overrides `--delimeter` option. Defaults to "," (comma) for CSV files and "\t" (tab) for TSV files, when not set. Note that this will also set the delimiter for qsv's output.
* `QSV_NO_HEADERS` - when set, the first row will **NOT** be interpreted as headers. Supersedes `QSV_TOGGLE_HEADERS`.
* `QSV_TOGGLE_HEADERS` - if set to `1`, toggles header setting - i.e. inverts qsv header behavior, with no headers being the default, and setting `--no-headers` will actually mean headers will not be ignored.
* `QSV_MAX_JOBS` - number of jobs to use for multi-threaded commands (currently `frequency`, `split` and `stats`). If not set, max_jobs is set
to number of logical processors divided by three.  See [Multithreading](#multithreading) for more info.
* `QSV_REGEX_UNICODE` - if set, makes `search`, `searchset` and `replace` commands unicode-aware. For increased performance, these
commands are not unicode-aware and will ignore unicode values when matching and will panic when unicode characters are used in the regex.
* `QSV_RDR_BUFFER_CAPACITY` - set to change reader buffer size (bytes - default when not set: 16384)
* `QSV_WTR_BUFFER_CAPACITY` - set to change writer buffer size (bytes - default when not set: 65536)
* `QSV_COMMENT_CHAR` - set to a comment character which will ignore any lines (including the header) that start with this character (default: comments disabled).
* `QSV_LOG_LEVEL` - set to desired level (default - off, error, warn, info, trace, debug).
* `QSV_LOG_DIR` - when logging is enabled, the directory where the log files will be stored. If the specified directory does not exist, qsv will attempt to create it. If not set, the log files are created in the directory where qsv was started. See [Logging](docs/Logging.md#logging) for more info.
* `QSV_NO_UPDATE` - prohibit self-update version check for the latest qsv release published on GitHub.

Several dependencies also have environment variables that influence qsv's performance & behavior:

* Memory Management ([mimalloc](https://docs.rs/mimalloc/latest/mimalloc/))   
  When incorporating qsv into a data pipeline that runs in batch mode, particularly with very large CSV files using qsv commands that load entire CSV files into memory, you can 
  [fine-tune Mimalloc's behavior using its environment variables](https://github.com/microsoft/mimalloc#environment-options).
* Network Access ([reqwest](https://docs.rs/reqwest/latest/reqwest/))   
  qsv uses reqwest for its `fetch` and `--update` functions and will honor [proxy settings](https://docs.rs/reqwest/latest/reqwest/index.html#proxies) set through `HTTP_PROXY`, `HTTPS_PROXY` and `NO_PROXY`.
  

> **NOTE:** To get a list of all qsv-relevant environment variables, run `qsv --envlist`.

Feature Flags
-------------
`qsv` has several features:

* `mimalloc` (default) - use the mimalloc allocator (see [Memory Allocator](#memory_allocator) for more info).
* `apply` - enable `apply` command. This swiss-army knife of CSV transformations is very powerful, but it has a lot of dependencies that increases both compile time and binary size. 
* `generate` - enable `generate` command. The test data generator also has a large dependency tree.

The following "power-user" commands can be abused and present "foot-shooting" scenarios.
* `lua` - enable `lua` command.
* `foreach` - enable `foreach` command (not valid for Windows).
* `python` - enable `py` command (requires Python 3.7+). Note that qsv will automatically use the currently activated python version when run in a virtual environment.

> **NOTE:** `qsvlite`, as the name implies, always has **non-default features disabled**. `qsv` can be built with any combination of the above features  using the cargo `--features`, `--all-features` & `--no-default-features` flags. The pre-built `qsv` binaries has **all applicable features enabled for the target platform**[^5].

Performance Tuning
------------------
### CPU Optimization
Modern CPUs have various features that the Rust compiler can take advantage
of to increase performance. If you want the compiler to take advantage of these
CPU-specific speed-ups, set this environment variable **BEFORE** installing/compiling qsv:

On Linux and macOS:
```bash
export CARGO_BUILD_RUSTFLAGS='-C target-cpu=native'
```

On Windows Powershell:
```powershell
$env:CARGO_BUILD_RUSTFLAGS='-C target-cpu=native'
```

Do note though that the resulting binary will only run on machines with the
same architecture as the machine you installed/compiled from.   
To find out your CPU architecture and other valid values for `target-cpu`:

```bash
rustc --print target-cpus

# to find out what CPU features are used by the Rust compiler WITHOUT specifying target-cpu
rustc --print cfg | grep -i target_feature

# to find out what additional CPU features will be used by the Rust compiler when you specify target-cpu=native
rustc --print cfg -C target-cpu=native | grep -i target_feature

# to get a short explanation of each CPU target-feature
rustc --print target-features
```
### Memory Allocator
By default, qsv uses an alternative allocator - [mimalloc](https://github.com/microsoft/mimalloc),
a performance-oriented allocator from Microsoft.
If you want to use the standard allocator, use the `--no-default-features` flag
when installing/compiling qsv, e.g.:

```bash
cargo install qsv --path . --no-default-features
```

or 

```bash
cargo build --release --no-default-features
```

To find out what memory allocator qsv is using, run `qsv --version`. After the qsv version number, the allocator used is displayed ("`standard`" or "`mimalloc`"). Note that mimalloc is not supported on the `x86_64-pc-windows-gnu` and `arm` targets, and you'll need to use the "standard" allocator on those platforms.

### Buffer size
Depending on your filesystem's configuration (e.g. block size, file system type, writing to remote file systems (e.g. sshfs, efs, nfs),
SSD or rotating magnetic disks, etc.), you can also fine-tune qsv's read/write buffers.

By default, the read buffer size is set to [16k](https://github.com/jqnatividad/qsv/blob/master/src/config.rs#L16), you can change it by setting the environment
variable `QSV_RDR_BUFFER_CAPACITY` in bytes.

The same is true with the write buffer (default: 64k) with the `QSV_WTR_BUFFER_CAPACITY` environment variable.

### Multithreading
Several commands support multithreading - `stats`, `frequency` and `split`.

Previously, these commands spawned several jobs equal to the number of logical processors. After extensive benchmarking, it turns out
doing so often results in the multithreaded runs running slower than single-threaded runs.

Multithreaded jobs do increase performance - to a point. After a certain number of threads, there are not only diminishing returns, the multithreading overhead actually results in slower runs.

Starting with qsv 0.22.0, a heuristic of setting the maximum number of jobs to the number of logical processors divided by 3 is applied. The user can still manually override this using the `--jobs` command-line option or the `QSV_MAX_JOBS` environment variable, but testing shows negative returns start at around this point.

These [observations were gathered using the benchmark script](https://github.com/jqnatividad/qsv/blob/master/docs/BENCHMARKS.md), using a relatively large file (520mb, 41 column, 1M row sample of NYC's 311 data). Performance will vary based on environment - CPU architecture, amount of memory, operating system, I/O speed, and the number of background tasks, so this heuristic will not work for every situation.

To find out your jobs setting, call `qsv --version`. The second to the last number is the number of jobs qsv will use for multi-threaded commands. The last number is the number of logical processors detected by qsv.

### Benchmarking for Performance
Use and fine-tune the [benchmark script](scripts/benchmark-basic.sh) when tweaking qsv's performance to your environment.
Don't be afraid to change the benchmark data and the qsv commands to something that is more representative of your
workloads.

Use the generated benchmark TSV files to meter and compare performance across platforms. You'd be surprised how performance varies
across environments - e.g. qsv's `join` performs abysmally on Windows's WSL running Ubuntu 20.04 LTS, taking 172.44 seconds.
On the same machine, running in a VirtualBox VM at that with the same Ubuntu version, `join` was done in 1.34 seconds - 
two orders of magnitude faster!

However, `stats` performs two times faster on WSL vs the VirtualBox VM - 2.80 seconds vs 5.33 seconds for the `stats_index` benchmark.

License
-------
Dual-licensed under MIT or the [UNLICENSE](https://unlicense.org).

Sponsor
-------
qsv was made possible by **[datHere](https://dathere.com) - Data Infrastructure Engineering.**   
Standards-based, best-of-breed, open source solutions to make your **Data Useful, Usable & Used.**

Naming Collision
----------------
This project is unrelated to [Intel's Quick Sync Video](https://www.intel.com/content/www/us/en/architecture-and-technology/quick-sync-video/quick-sync-video-general.html).
