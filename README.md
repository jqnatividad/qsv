qsv: Ultra-fast, data-wrangling CLI toolkit for CSVs
====================================================
[![Ubuntu build status](https://github.com/jqnatividad/qsv/actions/workflows/rust.yml/badge.svg)](https://github.com/jqnatividad/qsv/actions/workflows/rust.yml)
[![Windows build status](https://github.com/jqnatividad/qsv/actions/workflows/rust-windows.yml/badge.svg)](https://github.com/jqnatividad/qsv/actions/workflows/rust-windows.yml)
[![macOS build status](https://github.com/jqnatividad/qsv/actions/workflows/rust-macos.yml/badge.svg)](https://github.com/jqnatividad/qsv/actions/workflows/rust-macos.yml)
[![Crates.io](https://img.shields.io/crates/v/qsv.svg)](https://crates.io/crates/qsv)
[![Rust](https://img.shields.io/badge/rust-1.50.0%2B-blue.svg?maxAge=3600)](https://github.com/jqnatividad/qsv)   
qsv is a command line program for indexing, slicing, analyzing, splitting
and joining CSV/TSV files. Commands are simple, fast and composable:

1. Simple tasks are easy.
2. Performance trade offs are exposed in the CLI interface.
3. Composition does not come at the expense of performance.
----

* [Available Commands](#available-commands)
* [Installation](#installation)
* [Whirlwind Tour](docs/whirlwind_tour.md#a-whirlwind-tour)
* [Cookbook](https://github.com/jqnatividad/qsv/wiki)
* [Benchmarks](#benchmarks)
* [License](#license)
* [Sponsor](#sponsor)

> **NOTE: qsv is a fork of the popular [xsv](https://github.com/BurntSushi/xsv) utility, merging several pending PRs [since xsv 0.13.0's release](https://github.com/BurntSushi/xsv/issues/267), along with additional features & commands for data-wrangling (_NEW/EXTENDED_ commands are marked accordingly).**

Available commands
------------------
| Command | Description |
| --- | --- |
| **[apply](/src/cmd/apply.rs#L14)** | Apply series of string, date, currency and geocoding transformations to a CSV column. _**(NEW)**_ |
| **[behead](/src/cmd/behead.rs#L7)** | Drop headers from CSV file. _**(NEW)**_ |
| **[cat](/src/cmd/cat.rs#L7)** | Concatenate CSV files by row or by column. |
| **[count](/src/cmd/count.rs#L7)**[^1] | Count the rows in a CSV file. (Instantaneous with an index.) |
| **[dedup](/src/cmd/dedup.rs#L13)**[^2] | Remove redundant rows. _**(NEW)**_ |
| **[enum](/src/cmd/enumerate.rs#L10)** | Add a new column enumerating rows by adding a column of incremental or uuid identifiers. Can also be used to copy a column or fill a new column with a constant value. _**(NEW)**_ |
| **[exclude](/src/cmd/exclude.rs#L17)**[^1] | Removes a set of CSV data from another set based on the specified columns. _**(NEW)**_ |
| **[explode](/src/cmd/explode.rs#L8)** | Explode rows into multiple ones by splitting a column value based on the given separator. _**(NEW)**_ |
| **[fill](/src/cmd/fill.rs#L13)** | Fill empty values. _**(NEW)**_ |
| **[fixlengths](/src/cmd/fixlengths.rs#L9)** | Force a CSV file to have same-length records by either padding or truncating them. |
| **[flatten](/src/cmd/flatten.rs#L12)** | A flattened view of CSV records. Useful for viewing one record at a time. e.g., `qsv slice -i 5 data.csv | qsv flatten`. |
| **[fmt](/src/cmd/fmt.rs#L7)** | Reformat CSV data with different delimiters, record terminators or quoting rules. (Supports ASCII delimited data.) _**(EXTENDED)**_ |
| **[foreach](/src/cmd/foreach.rs#L15)** | Loop over a CSV file to execute bash commands. (*nix only) _**(NEW)**_ |
| **[frequency](/src/cmd/frequency.rs#L15)**[^1][^3] | Build frequency tables of each column in CSV data. (Uses parallelism to go faster if an index is present.) |
| **[headers](/src/cmd/headers.rs#L11)** | Show the headers of CSV data. Or show the intersection of all headers between many CSV files. |
| **[index](/src/cmd/index.rs#L13)** | Create an index for a CSV file. This is very quick and provides constant time indexing into the CSV file. |
| **[input](/src/cmd/input.rs#L7)** | Read CSV data with exotic quoting/escaping rules. |
| **[join](/src/cmd/join.rs#L18)**[^1] | Inner, outer and cross joins. Uses a simple hash index to make it fast. _**(EXTENDED)**_ |
| **[jsonl](/src/cmd/jsonl.rs#L11)** | Convert newline-delimited JSON to CSV. _**(NEW)**_
| **[lua](/src/cmd/lua.rs#L14)** | Execute a Lua script over CSV lines to transform, aggregate or filter them. _**(NEW)**_ |
| **[partition](/src/cmd/partition.rs#L16)** | Partition CSV data based on a column value. |
| **[pseudo](/src/cmd/pseudo.rs#L10)** | Pseudonymise the value of the given column by replacing them with an incremental identifier. _**(NEW)**_ |
| **[rename](/src/cmd/rename.rs#L7)** |  Rename the columns of CSV data efficiently. _**(NEW)**_ |
| **[replace](/src/cmd/replace.rs#L11)** | Replace CSV data using a regex. _**(NEW)**_ |
| **[reverse](/src/cmd/reverse.rs#L7)**[^2] | Reverse order of rows in CSV data. _**(NEW)**_ |
| **[sample](/src/cmd/sample.rs#L15)**[^1] | Randomly draw rows from CSV data using reservoir sampling (i.e., use memory proportional to the size of the sample). _**(EXTENDED)**_ |
| **[search](/src/cmd/search.rs#L10)** | Run a regex over CSV data. Applies the regex to each field individually and shows only matching rows. _**(EXTENDED)**_ |
| **[searchset](/src/cmd/searchset.rs#L14)** | Run **multiple regexes** over CSV data in a **single pass**. Applies the regexes to each field individually and shows only matching rows. _**(NEW)**_ |
| **[select](/src/cmd/select.rs#L8)**[^1] | Select or re-order columns from CSV data. _**(EXTENDED)**_ |
| **[slice](/src/cmd/slice.rs#L10)**[^1][^2] | Slice rows from any part of a CSV file. When an index is present, this only has to parse the rows in the slice (instead of all rows leading up to the start of the slice). |
| **[sort](/src/cmd/sort.rs#L13)** | Sort CSV data. _**(EXTENDED)**_ |
| **[split](/src/cmd/split.rs#L14)**[^1][^3] | Split one CSV file into many CSV files of N chunks. |
| **[stats](/src/cmd/stats.rs#L23)**[^1][^2][^3] | Show basic types and statistics of each column in the CSV file. (i.e., mean, standard deviation, variance, median, min/max, nullcount, mode, quartiles, etc.) _**(EXTENDED)**_ |
| **[table](/src/cmd/table.rs#L12)**[^2] | Show aligned output of any CSV data using [elastic tabstops](https://github.com/BurntSushi/tabwriter). _**(EXTENDED)**_ |
| **[transpose](/src/cmd/transpose.rs#L9)**[^2] | Transpose rows/columns of CSV data. _**(NEW)**_ |

[^1]: uses an index when available   
[^2]: loads the entire CSV into memory. Note that `stats` and `transpose` have modes that do not load the entire CSV into memory.   
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

The binary will end up in `./target/release/qsv`.

If you want to squeeze more performance from your build, set this environment
variable before compiling:

```bash
export CARGO_BUILD_RUSTFLAGS='-C target-cpu=native'
```

Do note though that the resulting binary will only run on machines with the
same architecture as the machine you compiled from.  To find out your CPU 
architecture and other valid values for `target-cpu`:

```bash
rustc --print target-cpus
```

Benchmarks
----------
Some [very rough benchmarks](docs/BENCHMARKS.md) of
various `qsv` commands.

License
-------
Dual-licensed under MIT or the [UNLICENSE](https://unlicense.org).

Sponsor
-------
qsv was made possible by **[datHere](https://dathere.com) - Data Infrastructure Engineering.**   
Standards-based, best-of-breed, open source solutions to make your **Data Useful, Usable & Used.**
