qsv: Ultra-fast, data-wrangling CLI toolkit for CSVs
====================================================
[![Linux build status](https://github.com/jqnatividad/qsv/actions/workflows/rust.yml/badge.svg)](https://github.com/jqnatividad/qsv/actions/workflows/rust.yml)
[![Windows build status](https://github.com/jqnatividad/qsv/actions/workflows/rust-windows.yml/badge.svg)](https://github.com/jqnatividad/qsv/actions/workflows/rust-windows.yml)
[![macOS build status](https://github.com/jqnatividad/qsv/actions/workflows/rust-macos.yml/badge.svg)](https://github.com/jqnatividad/qsv/actions/workflows/rust-macos.yml)
[![Security audit](https://github.com/jqnatividad/qsv/actions/workflows/security-audit.yml/badge.svg)](https://github.com/jqnatividad/qsv/actions/workflows/security-audit.yml)
[![Crates.io](https://img.shields.io/crates/v/qsv.svg)](https://crates.io/crates/qsv)
[![Minimum supported Rust version](https://img.shields.io/badge/Rust-1.56%2B-red?logo=rust)](#minimum-supported-rust-version)
[![Discussions](https://img.shields.io/github/discussions/jqnatividad/qsv)](https://github.com/jqnatividad/qsv/discussions)
[![Docs](https://img.shields.io/badge/wiki-docs-yellowgreen)](https://github.com/jqnatividad/qsv/wiki)   
qsv is a command line program for indexing, slicing, analyzing, splitting, enriching,
validating & joining CSV files. Commands are simple, fast and composable:

1. Simple tasks are easy.
2. Performance trade offs are exposed in the CLI interface.
3. Composition does not come at the expense of performance.
----

* [Available Commands](#available-commands)
* [Installation](#installation)
* [Whirlwind Tour](docs/whirlwind_tour.md#a-whirlwind-tour)
* [Cookbook](https://github.com/jqnatividad/qsv/wiki)
* [FAQ](https://github.com/jqnatividad/qsv/wiki/FAQ)
* [Benchmarks](docs/BENCHMARKS.md)
* [Sponsor](#sponsor)

> **NOTE:** qsv is a fork of the popular [xsv](https://github.com/BurntSushi/xsv) utility, merging several pending PRs [since xsv 0.13.0's release](https://github.com/BurntSushi/xsv/issues/267), along with additional features & commands for data-wrangling. See [FAQ](https://github.com/jqnatividad/qsv/wiki/FAQ) for more details. (_**NEW**_ and _**EXTENDED**_ commands are marked accordingly).

Available commands
------------------
| Command | Description |
| --- | --- |
| **[apply](/src/cmd/apply.rs#L24)** | Apply series of string, profanity, similarity, date, currency & geocoding transformations to a CSV column. _**(NEW)**_ |
| **[behead](/src/cmd/behead.rs#L7)** | Drop headers from a CSV. _**(NEW)**_ |
| **[cat](/src/cmd/cat.rs#L7)** | Concatenate CSV files by row or by column. |
| **[count](/src/cmd/count.rs#L7)**[^1] | Count the rows in a CSV file. (Instantaneous with an index.) |
| **[dedup](/src/cmd/dedup.rs#L13)**[^2] | Remove redundant rows. _**(NEW)**_ |
| **[enum](/src/cmd/enumerate.rs#L10)** | Add a new column enumerating rows by adding a column of incremental or uuid identifiers. Can also be used to copy a column or fill a new column with a constant value. _**(NEW)**_ |
| **[exclude](/src/cmd/exclude.rs#L17)**[^1] | Removes a set of CSV data from another set based on the specified columns. _**(NEW)**_ |
| **[explode](/src/cmd/explode.rs#L8)** | Explode rows into multiple ones by splitting a column value based on the given separator. _**(NEW)**_ |
| **[fill](/src/cmd/fill.rs#L13)** | Fill empty values. _**(NEW)**_ |
| **[fixlengths](/src/cmd/fixlengths.rs#L9)** | Force a CSV to have same-length records by either padding or truncating them. |
| **[flatten](/src/cmd/flatten.rs#L12)** | A flattened view of CSV records. Useful for viewing one record at a time.<br />e.g. `qsv slice -i 5 data.csv \| qsv flatten`. |
| **[fmt](/src/cmd/fmt.rs#L7)** | Reformat a CSV with different delimiters, record terminators or quoting rules. (Supports ASCII delimited data.) _**(EXTENDED)**_ |
| **[foreach](/src/cmd/foreach.rs#L17)** | Loop over a CSV to execute bash commands. (*nix only) _**(NEW)**_ |
| **[frequency](/src/cmd/frequency.rs#L15)**[^1][^3] | Build frequency tables of each column. (Uses parallelism to go faster if an index is present.) |
| **[generate](/src/cmd/generate.rs#L12)** | Generate test data by profiling a CSV using [Markov decision process](https://crates.io/crates/test-data-generation) machine learning. _**(NEW)**_ |
| **[headers](/src/cmd/headers.rs#L11)** | Show the headers of a CSV. Or show the intersection of all headers between many CSV files. |
| **[index](/src/cmd/index.rs#L13)** | Create an index for a CSV. This is very quick & provides constant time indexing into the CSV file. |
| **[input](/src/cmd/input.rs#L7)** | Read a CSV with exotic quoting/escaping rules. |
| **[join](/src/cmd/join.rs#L17)**[^1] | Inner, outer, cross, anti & semi joins. Uses a simple hash index to make it fast. _**(EXTENDED)**_ |
| **[jsonl](/src/cmd/jsonl.rs#L11)** | Convert newline-delimited JSON to CSV. _**(NEW)**_
| **[lua](/src/cmd/lua.rs#L15)** | Execute a [Lua](https://www.lua.org/about.html) script over CSV lines to transform, aggregate or filter them. _**(NEW)**_ |
| **[partition](/src/cmd/partition.rs#L16)** | Partition a CSV based on a column value. |
| **[pseudo](/src/cmd/pseudo.rs#L10)** | Pseudonymise the value of the given column by replacing them with an incremental identifier. _**(NEW)**_ |
| **[rename](/src/cmd/rename.rs#L7)** |  Rename the columns of a CSV efficiently. _**(NEW)**_ |
| **[replace](/src/cmd/replace.rs#L12)** | Replace CSV data using a regex. _**(NEW)**_ |
| **[reverse](/src/cmd/reverse.rs#L7)**[^2] | Reverse order of rows in a CSV. Unlike the `sort --reverse` command, it preserves the order of rows with the same key. _**(NEW)**_ |
| **[sample](/src/cmd/sample.rs#L12)**[^1] | Randomly draw rows (with optional seed) from a CSV using [reservoir sampling](https://en.wikipedia.org/wiki/Reservoir_sampling) (i.e., use memory proportional to the size of the sample). _**(EXTENDED)**_ |
| **[search](/src/cmd/search.rs#L11)** | Run a regex over a CSV. Applies the regex to each field individually & shows only matching rows. _**(EXTENDED)**_ |
| **[searchset](/src/cmd/searchset.rs#L15)** | Run **multiple regexes** over a CSV in a **single pass**. Applies the regexes to each field individually & shows only matching rows. _**(NEW)**_ |
| **[select](/src/cmd/select.rs#L8)** | Select, re-order, duplicate or drop columns. _**(EXTENDED)**_ |
| **[slice](/src/cmd/slice.rs#L10)**[^1][^2] | Slice rows from any part of a CSV. When an index is present, this only has to parse the rows in the slice (instead of all rows leading up to the start of the slice). |
| **[sort](/src/cmd/sort.rs#L14)** | Sorts CSV data in alphabetical, numerical, reverse or random (with optional seed) order. _**(EXTENDED)**_ |
| **[split](/src/cmd/split.rs#L14)**[^1][^3] | Split one CSV file into many CSV files of N chunks. |
| **[stats](/src/cmd/stats.rs#L24)**[^1][^2][^3] | Show data type & descriptive statistics of each column in a CSV. (i.e., sum, min/max, min/max length, mean, stddev, variance, quartiles, IQR, lower/upper fences, skew, median, mode, cardinality & nullcount) _**(EXTENDED)**_ |
| **[table](/src/cmd/table.rs#L12)**[^2] | Show aligned output of a CSV using [elastic tabstops](https://github.com/BurntSushi/tabwriter). _**(EXTENDED)**_ |
| **[transpose](/src/cmd/transpose.rs#L9)**[^2] | Transpose rows/columns of a CSV. _**(NEW)**_ |

[^1]: uses an index when available. `join` always uses indices.   
[^2]: loads the entire CSV into memory. Note that `stats` & `transpose` have modes that do not load the entire CSV into memory.   
[^3]: runs parallel jobs by default (use `--jobs` option to adjust)   

Installation
------------
Binaries for Windows, Linux and macOS are available [from Github](https://github.com/jqnatividad/qsv/releases/latest).

Alternatively, you can compile from source by
[installing Cargo](https://crates.io/install)
([Rust's](https://www.rust-lang.org/) package manager)
and installing `qsv` using Cargo:

```bash
cargo install qsv
```

Compiling from this repository also works similarly:

```bash
git clone git://github.com/jqnatividad/qsv
cd qsv
cargo build --release
```

The compiled binary will end up in `./target/release/qsv`.

### Minimum Supported Rust Version
Building qsv requires Rust version 1.56+.

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

Environment Variables
---------------------

* `QSV_DEFAULT_DELIMITER` - single ascii character to use as delimiter.  Overrides `--delimeter` option. Defaults to "," (comma) for CSV files and "\t" (tab) for TSV files, when not set. Note that this will also set the delimiter for qsv's output.
* `QSV_NO_HEADERS` - when set, the first row will **NOT** be interpreted as headers. Supersedes `QSV_TOGGLE_HEADERS`.
* `QSV_TOGGLE_HEADERS` - if set to `1`, toggles header setting - i.e. inverts qsv header behavior, with no headers being the default, and setting `--no-headers` will actually mean headers will not be ignored.
* `QSV_MAX_JOBS` - number of jobs to use for multi-threaded commands (currently `frequency`, `split` and `stats`). If not set, max_jobs is set
to number of logical processors divided by four.  See [Parallelization](#parallelization) for more info.
* `QSV_REGEX_UNICODE` - if set, makes `search`, `searchset` and `replace` commands unicode-aware. For increased performance, these
commands are not unicode-aware and will ignore unicode values when matching and will panic when unicode characters are used in the regex.
* `QSV_RDR_BUFFER_CAPACITY` - set to change reader buffer size (bytes - default when not set: 16384)
* `QSV_WTR_BUFFER_CAPACITY` - set to change writer buffer size (bytes - default when not set: 65536)

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
cargo install qsv --no-default-features
```

or 

```bash
cargo build --release --no-default-features
```

To find out what memory allocator qsv is using, do `qsv --version`. After the qsv version number, the allocator used is displayed ("`standard`" or "`mimalloc`"). Note that mimalloc is not supported on the `x86_64-pc-windows-gnu` and `arm` targets, and you'll need to use the "standard" allocator on those platforms.

### Buffer size
Depending on your filesystem's configuration (e.g. block size, file system type, writing to remote file systems (e.g. sshfs, efs, nfs),
SSD or rotating magnetic disks, etc.), you can also fine-tune qsv's read/write buffers.

By default, the read buffer size is set to [16k](https://github.com/jqnatividad/qsv/blob/master/src/config.rs#L16), you can change it by setting the environment
variable `QSV_RDR_BUFFER_CAPACITY` in bytes.

The same is true with the write buffer (default: 64k) with the `QSV_WTR_BUFFER_CAPACITY` environment variable.

### Parallelization
Several commands support parallelization/multi-threading - `stats`, `frequency` and `split`.

Previously, these commands spawned several jobs equal to the number of logical processors. After extensive benchmarking, it turns out
doing so often results in the multi-threaded runs running slower than single-threaded runs.

Parallelized jobs do increase performance - to a point. After a certain number of threads, there are not only diminishing returns, the parallelization overhead actually results in slower runs.

Starting with qsv 0.22.0, a heuristic of setting the maximum number of jobs to the number of logical processors divided by 4 is applied. The user can still manually override this using the `--jobs` command-line option or the `QSV_MAX_JOBS` environment variable, but testing shows negative returns start at around this point.

These [observations were gathered using the benchmark script](https://github.com/jqnatividad/qsv/blob/master/docs/BENCHMARKS.md), using a relatively large file (520mb, 41 column, 1M row sample of NYC's 311 data). Performance will vary based on environment - CPU architecture, amount of memory, operating system, I/O speed, and the number of background tasks, so this heuristic will not work for every situation.

To find out your jobs setting, call `qsv --version`. The last number is the default number of jobs qsv will use for parallelized commands if `--jobs` and `QSV_MAX_JOBS` are not specified.

### Benchmarking for Performance
Use and fine-tune the [benchmark script](scripts/benchmark-basic.sh) when tweaking qsv's performance to your environment.
Don't be afraid to change the benchmark data and the qsv commands to something that is more representative of your
workloads.

Use the generated TSV files to meter and compare performance across platforms. You'd be surprised how performance varies
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
