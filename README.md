## qsv: Ultra-fast CSV data-wrangling toolkit

[![Linux build status](https://github.com/jqnatividad/qsv/actions/workflows/rust.yml/badge.svg)](https://github.com/jqnatividad/qsv/actions/workflows/rust.yml)
[![Windows build status](https://github.com/jqnatividad/qsv/actions/workflows/rust-windows.yml/badge.svg)](https://github.com/jqnatividad/qsv/actions/workflows/rust-windows.yml)
[![macOS build status](https://github.com/jqnatividad/qsv/actions/workflows/rust-macos.yml/badge.svg)](https://github.com/jqnatividad/qsv/actions/workflows/rust-macos.yml)
[![Security audit](https://github.com/jqnatividad/qsv/actions/workflows/security-audit.yml/badge.svg)](https://github.com/jqnatividad/qsv/actions/workflows/security-audit.yml)
[![Downloads](https://img.shields.io/github/downloads/jqnatividad/qsv/total?logo=github)](https://github.com/jqnatividad/qsv/releases/latest)
[![Clones](https://img.shields.io/badge/dynamic/json?color=success&label=clones&query=count&url=https://gist.githubusercontent.com/jqnatividad/13f60ad0b54856a55f60b8e653079349/raw/clone.json&logo=github)](https://github.com/MShawon/github-clone-count-badge)
[![Discussions](https://img.shields.io/github/discussions/jqnatividad/qsv)](https://github.com/jqnatividad/qsv/discussions)
[![HomeBrew](https://img.shields.io/homebrew/v/qsv?logo=homebrew)](https://formulae.brew.sh/formula/qsv)
[![Crates.io](https://img.shields.io/crates/v/qsv.svg?logo=crates.io)](https://crates.io/crates/qsv)
[![Crates.io downloads](https://img.shields.io/crates/d/qsv?color=orange&label=crates.io%20downloads)](https://crates.io/crates/qsv)
[![Minimum supported Rust version](https://img.shields.io/badge/Rust-1.67.0-red?logo=rust)](#minimum-supported-rust-version)

<div align="center">

 &nbsp;          |  Table of Contents
:--------------------------|:-------------------------
![qsv logo](docs/images/qsv-logo.png)<br/><sub><sup>[logo details](https://github.com/jqnatividad/qsv/discussions/295)</sup></sub>  |qsv is a command line program for<br>indexing, slicing, analyzing, filtering,<br>enriching, validating & joining CSV files.<br>Commands are simple, fast & composable.<br><br>* [Available Commands](#available-commands)<br>* [Installation](#installation)<br> * [Whirlwind Tour](docs/whirlwind_tour.md#a-whirlwind-tour)<br>* [Cookbook](https://github.com/jqnatividad/qsv/wiki/Cookbook#cookbook)<br>* [FAQ](https://github.com/jqnatividad/qsv/discussions/categories/faq)<br>* [Changelog](https://github.com/jqnatividad/qsv/blob/master/CHANGELOG.md#changelog)<br>* [Performance Tuning](https://github.com/jqnatividad/qsv/blob/master/docs/PERFORMANCE.md#performance-tuning)<br>* [Benchmarks](docs/BENCHMARKS.md)<br>* [Environment Variables](#environment-variables)<br>* [Feature Flags](#feature-flags)<br>* [Testing](#testing)<br>* [NYC School of Data 2022 slides](https://docs.google.com/presentation/d/e/2PACX-1vQ12ndZL--gkz0HLQRaxqsNOwzddkv1iUKB3sq661yA77OPlAsmHJHpjaqt9s9QEf73VqMfb0cv4jHU/pub?start=false&loop=false&delayms=3000)<br>* [Sponsor](#sponsor)

</div>

> ℹ️ **NOTE:** qsv is a fork of the popular [xsv](https://github.com/BurntSushi/xsv) utility, merging several pending PRs [since xsv 0.13.0's May 2018 release](https://github.com/BurntSushi/xsv/issues/267). On top of xsv's 20 commands, it adds numerous new features; 30 additional commands; 6 `apply` subcommands & 35 operations; and 5 `to` subcommands (for a total of 96).
See [FAQ](https://github.com/jqnatividad/qsv/discussions/categories/faq) for more details.

## Available commands

| Command | Description |
| --- | --- |
| [apply](/src/cmd/apply.rs#L2)<br>❇️🚀🧠 | Apply series of string, date, math, currency & geocoding transformations to a CSV column. It also has some basic [NLP](https://en.wikipedia.org/wiki/Natural_language_processing) functions ([similarity](https://crates.io/crates/strsim), [sentiment analysis](https:❇️//crates.io/crates/vader_sentiment), [profanity](https://docs.rs/censor/latest/censor/), [eudex](https://github.com/ticki/eudex#eudex-a-blazingly-fast-phonetic-reductionhashing-algorithm) & [language detection](https://crates.io/crates/whatlang)).  |
| [applydp](/src/cmd/applydp.rs#L2)<br>🚀 | applydp is a slimmed-down version of `apply` with only [Datapusher+](https://github.com/dathere/datapusher-plus) relevant subcommands/operations (`qsvdp` binary variant only). |
| [behead](/src/cmd/behead.rs#L2) | Drop headers from a CSV.  |
| [cat](/src/cmd/cat.rs#L2) | Concatenate CSV files by row or by column. |
| [count](/src/cmd/count.rs#L2)<br>📇 | Count the rows in a CSV file. (Instantaneous with an index.) |
| [dedup](/src/cmd/dedup.rs#L2)<br>🗜️🚀 | Remove duplicate rows (See also `extsort`, `sort` & `sortcheck` commands). |
| [diff](/src/cmd/diff.rs#L2)<br>🚀 | Find the difference between two CSVs with ludicrous speed!<br/>e.g. *compare two CSVs with 1M rows x 9 columns in under 600ms!* |
| [enum](/src/cmd/enumerate.rs#L2) | Add a new column enumerating rows by adding a column of incremental or uuid identifiers. Can also be used to copy a column or fill a new column with a constant value.  |
| [excel](/src/cmd/excel.rs#L2) | Exports a specified Excel/ODS sheet to a CSV file. |
| [exclude](/src/cmd/exclude.rs#L2)<br>📇 | Removes a set of CSV data from another set based on the specified columns.  |
| [explode](/src/cmd/explode.rs#L2) | Explode rows into multiple ones by splitting a column value based on the given separator.  |
| [extsort](/src/cmd/extsort.rs#L2)<br>🚀 | Sort an arbitrarily large CSV/text file using a multithreaded [external merge sort](https://en.wikipedia.org/wiki/External_sorting) algorithm. |
| [fetch](/src/cmd/fetch.rs#L2)<br>❇️🧠 | Fetches data from web services for every row using **HTTP Get**. Comes with [HTTP/2](https://http2-explained.haxx.se/en/part1) [adaptive flow control](https://medium.com/coderscorner/http-2-flow-control-77e54f7fd518), [jql](https://github.com/yamafaktory/jql#%EF%B8%8F-usage) JSON query language support, dynamic throttling ([RateLimit](https://tools.ietf.org/id/draft-polli-ratelimit-headers-00.html)) & caching with optional [Redis](https://redis.io/) support for persistent caching. |
| [fetchpost](/src/cmd/fetchpost.rs#L2)<br>❇️🧠 | Similar to `fetch`, but uses **HTTP Post**. ([HTTP GET vs POST methods](https://www.geeksforgeeks.org/difference-between-http-get-and-post-methods/)) |
| [fill](/src/cmd/fill.rs#L2) | Fill empty values.  |
| [fixlengths](/src/cmd/fixlengths.rs#L2) | Force a CSV to have same-length records by either padding or truncating them. |
| [flatten](/src/cmd/flatten.rs#L2) | A flattened view of CSV records. Useful for viewing one record at a time.<br />e.g. `qsv slice -i 5 data.csv \| qsv flatten`. |
| [fmt](/src/cmd/fmt.rs#L2) | Reformat a CSV with different delimiters, record terminators or quoting rules. (Supports ASCII delimited data.)  |
| [foreach](/src/cmd/foreach.rs#L3)<br>❇️ | Loop over a CSV to execute bash commands. (not available on Windows)  |
| [frequency](/src/cmd/frequency.rs#L2)<br>📇🏎️ | Build [frequency tables](https://statisticsbyjim.com/basics/frequency-table/) of each column. Uses multithreading to go faster if an index is present. |
| [generate](/src/cmd/generate.rs#L2)<br>❇️ | Generate test data by profiling a CSV using [Markov decision process](https://crates.io/crates/test-data-generation) machine learning.  |
| [headers](/src/cmd/headers.rs#L2) | Show the headers of a CSV. Or show the intersection of all headers between many CSV files. |
| [index](/src/cmd/index.rs#L2) | Create an index for a CSV. This is very quick & provides constant time indexing into the CSV file. Also enables multithreading for `frequency`, `split`, `stats` & `schema` commands. |
| [input](/src/cmd/input.rs#L2) | Read CSV data with special quoting, trimming, line-skipping & UTF-8 transcoding rules. Typically used to "normalize" a CSV for further processing with other qsv commands. |
| [join](/src/cmd/join.rs#L2)<br>📇 | Inner, outer, cross, anti & semi joins. Automatically creates a simple, in-memory hash index to make it fast.  |
| [jsonl](/src/cmd/jsonl.rs#L2) | Convert newline-delimited JSON ([JSONL](https://jsonlines.org/)/[NDJSON](http://ndjson.org/)) to CSV. See `tojsonl` command to convert CSV to JSONL.
| [luau](/src/cmd/luau.rs#L2)<br>❇️ | Create a new computed column, filter rows or compute aggregations by executing a [Luau](https://luau-lang.org) script for every row of a CSV file. |
| [partition](/src/cmd/partition.rs#L2) | Partition a CSV based on a column value. |
| [pseudo](/src/cmd/pseudo.rs#L2) | [Pseudonymise](https://en.wikipedia.org/wiki/Pseudonymization) the value of the given column by replacing them with an incremental identifier.  |
| [py](/src/cmd/python.rs#L2)<br>❇️ | Create a new computed column or filter rows by evaluating a python expression on every row of a CSV file. Python's [f-strings](https://www.freecodecamp.org/news/python-f-strings-tutorial-how-to-use-f-strings-for-string-formatting/) is particularly useful for extended formatting, [with the ability to evaluate Python expressions as well](https://github.com/jqnatividad/qsv/blob/4cd00dca88addf0d287247fa27d40563b6d46985/src/cmd/python.rs#L23-L31). |
| [rename](/src/cmd/rename.rs#L2) |  Rename the columns of a CSV efficiently.  |
| [replace](/src/cmd/replace.rs#L2) | Replace CSV data using a regex.  |
| [reverse](/src/cmd/reverse.rs#L2)<br>🗜️ | Reverse order of rows in a CSV. Unlike the `sort --reverse` command, it preserves the order of rows with the same key.  |
| [safenames](/src/cmd/safenames.rs#L2) | Modify headers of a CSV to only have ["safe" names](/src/cmd/safenames.rs#L5-L14) - guaranteed "database-ready" names.  |
| [sample](/src/cmd/sample.rs#L2)<br>📇 | Randomly draw rows (with optional seed) from a CSV using [reservoir sampling](https://en.wikipedia.org/wiki/Reservoir_sampling) (i.e., use memory proportional to the size of the sample).  |
| [schema](/src/cmd/schema.rs#L2)<br>📇🏎️ | Infer schema from CSV data, replete with data type & domain/range validation & output in [JSON Schema](https://json-schema.org/) format. Uses multithreading to go faster if an index is present. See `validate` command to use the generated JSON Schema to validate if similar CSVs comply with the schema. |
| [search](/src/cmd/search.rs#L2) | Run a regex over a CSV. Applies the regex to each field individually & shows only matching rows.  |
| [searchset](/src/cmd/searchset.rs#L3) | **Run multiple regexes over a CSV in a single pass.** Applies the regexes to each field individually & shows only matching rows.  |
| [select](/src/cmd/select.rs#L2) | Select, re-order, duplicate or drop columns.  |
| [slice](/src/cmd/slice.rs#L2)<br>📇 | Slice rows from any part of a CSV. When an index is present, this only has to parse the rows in the slice (instead of all rows leading up to the start of the slice).  |
| [sniff](/src/cmd/sniff.rs#L2) | Quickly sniff CSV metadata (delimiter, header row, preamble rows, quote character, flexible, is_utf8, number of records, number of fields, field names & data types). |
| [sort](/src/cmd/sort.rs#L2)<br>🚀🗜️ | Sorts CSV data in alphabetical (with case-insensitive option), numerical, reverse, unique or random (with optional seed) order (See also `extsort` & `sortcheck` commands).  |
| [sortcheck](/src/cmd/sortcheck.rs#L2)<br>📇 | Check if a CSV is sorted. With the --json options, also retrieve record count, sort breaks & duplicate count. |
| [split](/src/cmd/split.rs#L2)<br>📇🏎️ | Split one CSV file into many CSV files of N chunks. Uses multithreading to go faster if an index is present. |
| [stats](/src/cmd/stats.rs#L2)<br>📇🗜️🏎️ | Compute [summary statistics](https://en.wikipedia.org/wiki/Summary_statistics) (sum, min/max/range, min/max length, mean, stddev, variance, nullcount, sparsity, quartiles, IQR, lower/upper fences, skewness, median, mode/s, antimode/s & cardinality) & make GUARANTEED data type inferences (Null, String, Float, Integer, Date, DateTime) for each column in a CSV. Uses multithreading to go faster if an index is present. |
| [table](/src/cmd/table.rs#L2)<br>🗜️ | Show aligned output of a CSV using [elastic tabstops](https://github.com/BurntSushi/tabwriter).  |
| [to](/src/cmd/to.rs#L2)<br>❇️🚀 | Convert CSV files to [PostgreSQL](https://www.postgresql.org), [SQLite](https://www.sqlite.org/index.html), XLSX, [Parquet](https://parquet.apache.org) and [Data Package](https://datahub.io/docs/data-packages/tabular). |
| [tojsonl](/src/cmd/tojsonl.rs#L3)<br>📇🏎️ | Smartly converts CSV to a newline-delimited JSON ([JSONL](https://jsonlines.org/)/[NDJSON](http://ndjson.org/)). By scanning the CSV first, it "smartly" infers the appropriate JSON data type for each column. See `jsonl` command to convert JSONL to CSV. Uses multithreading to go faster if an index is present. |
| [transpose](/src/cmd/transpose.rs#L2)<br>🗜️ | Transpose rows/columns of a CSV.  |
| [validate](/src/cmd/validate.rs#L2)<br>📇🚀 | Validate CSV data with JSON Schema (See `schema` command) & put invalid records into a separate file & a validation error report file. If no jsonschema file is provided, validates if a CSV conforms to the [RFC 4180 standard](https://datatracker.ietf.org/doc/html/rfc4180). |

 ❇️: enabled by a feature flag on `qsv`. Not available on `qsvlite`. `qsvdp` has `luau` & `applydp` pre-enabled.   
📇: uses an index when available. `join` creates its own in-memory index automatically.   
🗜️: loads entire CSV into memory, though `dedup`, `stats` & `transpose` have "streaming" modes as well.   
🧠: expensive operations are memoized (cached) with available inter-session Redis caching for fetch commands.    
🏎️: multithreaded when an index is available.   
🚀: multithreaded even without an index.

## Installation

For [macOS and Linux (64-bit)](https://formulae.brew.sh/formula/qsv), you can quickly install qsv with [Homebrew](https://brew.sh). However, only the `apply` [feature](#feature-flags) is enabled.

```
brew install qsv
```

Prebuilt binary variants of the latest qsv version with more enabled features for Windows, Linux & macOS are also available [for download](https://github.com/jqnatividad/qsv/releases/latest), including binaries compiled with [Rust Nightly/Unstable](https://stackoverflow.com/questions/70745970/rust-nightly-vs-beta-version) ([more info](https://github.com/jqnatividad/qsv/blob/master/docs/PERFORMANCE.md#nightly-release-builds)).

There are three binary variants of qsv:
 * `qsv` - [feature](#feature-flags)-capable(❇️), with the [prebuilt binaries](https://github.com/jqnatividad/qsv/releases/latest) enabling all applicable features except Python [^6]
 * `qsvlite` - all features disabled (~40% of the size of `qsv`)
 * `qsvdp` - optimized for use with [DataPusher+](https://github.com/dathere/datapusher-plus) with only DataPusher+ relevant commands; `applydp`, a slimmed-down version of the `apply` feature; embedded `luau` interpreter; the `--progressbar` option disabled; and the self-update only checking for new releases, requiring an explicit `--update` (~40% of the the size of `qsv`).

Alternatively, you can install from source by [installing Rust](https://www.rust-lang.org/tools/install)
and installing `qsv` using Rust's cargo command[^5]:

[^5]: Of course, you'll also need a linker & a C compiler. Linux users should generally install GCC or Clang, according to their distribution’s documentation.
For example, if you use Ubuntu, you can install the `build-essential` package. On macOS, you can get a C compiler by running `$ xcode-select --install`.
For Windows, this means installing [Visual Studio 2022](https://visualstudio.microsoft.com/downloads/). When prompted for workloads, include "Desktop Development with C++",
the Windows 10 or 11 SDK & the English language pack, along with any other language packs your require.

```bash
cargo install qsv --locked --features all_full
```

The binary will be installed in `~/.cargo/bin`.

Compiling from source also works similarly:

```bash
git clone https://github.com/jqnatividad/qsv.git
cd qsv
cargo build --release --locked --features all_full
```

The compiled binary will end up in `./target/release/`.

To enable optional features, use cargo `--features` (see [Feature Flags](#feature-flags) for more info):

```bash
cargo install qsv --locked --features apply,generate,luau,fetch,foreach,python,to,self_update,full
# or shorthand
cargo install qsv --locked --features all_full
# or to install qsvlite
cargo install qsv --locked --features lite
# or to install qsvdp
cargo install qsv --locked --features datapusher_plus

# or when compiling from a local repo
cargo build --release --locked --features apply,generate,luau,fetch,foreach,python,to,self_update,full
# shorthand
cargo build --release --locked --features all_full
# for qsvlite
cargo build --release --locked --features lite
# for qsvdp
cargo build --release --locked --features datapusher_plus
```

[^6]: The `foreach` feature is not available on Windows. The `python` feature is not enabled on the prebuilt binaries. Compile with Python 3.6 and above development environment installed if you want to enable the `python` feature. Lua support is enabled by default on the prebuilt binaries, with preference for `luau` for platforms that support it.  

### Minimum Supported Rust Version

qsv's MSRV policy is to require the latest [Rust version](https://github.com/rust-lang/rust/blob/master/RELEASES.md) that is [supported by Homebrew](https://formulae.brew.sh/formula/rust#default).

## Tab Completion

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

## File formats

qsv recognizes UTF-8/ASCII encoded, CSV (`.csv`) & TSV files (`.tsv` & `.tab`). CSV files are assumed to have "," (comma) as a delimiter,
and TSV files, "\t" (tab) as a delimiter. The delimiter is a single ascii character that can be set either by the `--delimiter` command-line option or
with the `QSV_DEFAULT_DELIMITER` environment variable or automatically detected when `QSV_SNIFF_DELIMITER` is set.

When using the `--output` option, qsv will UTF-8 encode the file & automatically change the delimiter used in the generated file based on the file extension - i.e. comma for `.csv`, tab for `.tsv` & `.tab` files.

[JSONL](https://jsonlines.org/)/[NDJSON](http://ndjson.org/) files are also recognized & converted to/from CSV with the [`jsonl`](/src/cmd/jsonl.rs#L11) and [`tojsonl`](/src/cmd/tojsonl.rs#L12) commands respectively.

The `fetch` & `fetchpost` commands also produces JSONL files when its invoked without the `--new-column` option & TSV files with the `--report` option.

The `excel`, `safenames`, `sniff`, `sortcheck` & `validate` commands produce JSON files with their JSON options.

The `schema` command produces a [JSON Schema Validation (Draft 7)](https://json-schema.org/draft/2020-12/json-schema-validation.html) file with the ".schema.json" file extension, which can be used with the `validate` command.

The `excel` command recognizes Excel & Open Document Spreadsheet(ODS) files (`.xls`, `.xlsx`, `.xlsm`, `.xlsb` & `.ods` files).

The `to` command produces produces `.xlsx`, [Parquet](https://parquet.apache.org) & [Data Package](https://datahub.io/docs/data-packages/tabular) files, and populates [PostgreSQL](https://www.postgresql.org) and [SQLite](https://www.sqlite.org/index.html) databases.

### RFC 4180

qsv validates against the [RFC 4180](https://datatracker.ietf.org/doc/html/rfc4180) CSV standard. However IRL, CSV formats vary significantly & qsv is actually not strictly compliant with the specification so it can process "real-world" CSV files.
qsv leverages the awesome [Rust CSV](https://docs.rs/csv/latest/csv/) crate to read/write CSV files.

Click [here](https://docs.rs/csv-core/latest/csv_core/struct.Reader.html#rfc-4180) to find out more about how qsv conforms to the standard using this crate.

### **UTF-8 Encoding**

The following commands require UTF-8 encoded input (of which ASCII is a subset) - `dedup`, `exclude`, `fetch`, `fetchpost`, `frequency`, `join`, `schema`, `sort`, `stats` & `validate`.

For these commands, qsv checks if the input is UTF-8 encoded by scanning the first 8k & will abort if its not unless `QSV_SKIPUTF8_CHECK` is set. On Linux & macOS, UTF-8 encoding is the default.

This was done to increase performance of these commands, as they make extensive use of `from_utf8_unchecked` so as not to pay the repetitive utf-8 validation penalty, no matter how small, even for already utf-8 encoded files.

Should you need to re-encode CSV/TSV files, you can use the `input` command to transcode to UTF-8. It will replace all invalid UTF-8 sequences with `�`. Alternatively, there are several utilities you can use to do so on [Linux/macOS](https://stackoverflow.com/questions/805418/how-can-i-find-encoding-of-a-file-via-a-script-on-linux) & [Windows](https://superuser.com/questions/1163753/converting-text-file-to-utf-8-on-windows-command-prompt).

### **Windows Usage Note**

Unlike other modern operating systems, Microsoft Windows' [default encoding is UTF16-LE](https://stackoverflow.com/questions/66072117/why-does-windows-use-utf-16le). This will cause problems when redirecting qsv's output to a CSV file & trying to open it with Excel (which ignores the comma delimiter, with everything in the first column):

```
qsv stats wcp.csv > wcpstats.csv
```

Which is weird, since you would think [Microsoft's own Excel would properly recognize UTF16-LE encoded CSV files](https://answers.microsoft.com/en-us/msoffice/forum/all/opening-csv-file-with-utf16-encoding-in-excel-2010/ed522cb9-e88d-4b82-b88e-a2d4bd99f874?auth=1). Regardless, to create a properly UTF-8 encoded file on Windows, use the `--output` option instead:

```
# so instead of redirecting stdout to a file
qsv stats wcp.csv > wcpstats.csv

# do this instead
qsv stats wcp.csv --output wcpstats.csv
```


## Interpreters
For complex data-wrangling tasks, you can use Luau and Python scripts. The `qsv` binary variant can embed `luau` and `python` interpreters, enabled by identically named feature flags. The `qsvdp` binary variant has the `luau` interpreter embedded by default.
### Luau

[Luau](https://luau-lang.org) is a fast, small, safe, gradually typed embeddable scripting language derived from [Lua](https://www.lua.org/about.html). It lies at the [heart of Roblox technology](https://luau-lang.org/2022/11/04/luau-origins-and-evolution.html) - powering all it's user generated content, with [Roblox](https://en.wikipedia.org/wiki/Roblox)'s own internal code having more than 2 millions lines of Luau. 

It has [sandboxing](https://luau-lang.org/sandbox), [type-checking](https://luau-lang.org/typecheck), [additional operators](https://luau-lang.org/syntax) & [increased performance](https://luau-lang.org/performance) while [maintaining compatibility with Lua](https://luau-lang.org/compatibility).

[Lua is much faster than Python](https://benchmarksgame-team.pages.debian.net/benchmarksgame/fastest/lua-python3.html), and Luau is even faster still - more so, as qsv precompiles Luau scripts into bytecode. In addition, [`luau`](/src/cmd/luau.rs#L2) is embedded into qsv, has debug logging, can do aggregations with its `--prologue` & `--epilogue` options & has no external dependencies unlike the `py` command.

### Python

The `python` feature is NOT enabled by default on the prebuilt binaries, as doing so requires it to dynamically link to python at runtime, which presents distribution issues, as various operating systems have differing bundled Python versions.

If you wish to enable the `python` feature - you'll just have to install/compile from source, making sure you have the development libraries for the desired Python version (Python 3.6 to 3.11 are supported) installed when doing so.

If you plan to distribute your manually built `qsv` with the `python` feature, `qsv` will look for the specific version of Python shared libraries (libpython* on Linux/macOS, python*.dll on Windows) against which it was compiled starting with the current directory & abort with an error if not found, detailing the Python library it was looking for. 

Note that this will happen on qsv startup, even if you're not running the `py` command.

When building from source - [PyO3](https://pyo3.rs) - the underlying crate that enables the `python` feature, uses a build script to determine the Python version & set the correct linker arguments. By default it uses the python3 executable.
You can override this by setting `PYO3_PYTHON` (e.g., `PYO3_PYTHON=python3.6`), before installing/compiling qsv. See the [PyO3 User Guide](https://pyo3.rs/v0.17.1/building_and_distribution.html) for more information.

Consider using the [`luau`](/src/cmd/luau.rs#L2) command instead of the [`py`]((/src/cmd/python.rs#L2)) command if the operation you're trying to do can be done with `luau` - as `luau` is faster than `py` and can do aggregations. 

The `py` command cannot do aggregations because [PyO3's GIL-bound memory](https://pyo3.rs/v0.17.2/memory.html#gil-bound-memory) limitations will quickly consume a lot of memory (see [issue 449](https://github.com/jqnatividad/qsv/issues/449#issuecomment-1226095316) for details).
To prevent this, the `py` command processes CSVs in batches (default: 30,000 records), with a GIL pool for each batch, so no globals are available across batches.

## Environment Variables

| Variable | Description |
| --- | --- |
| `QSV_DEFAULT_DELIMITER` | single ascii character to use as delimiter.  Overrides `--delimeter` option. Defaults to "," (comma) for CSV files & "\t" (tab) for TSV files when not set. Note that this will also set the delimiter for qsv's output to stdout.<br>However, using the `--output` option, regardless of this environment variable, will automatically change the delimiter used in the generated file based on the file extension - i.e. comma for `.csv`, tab for `.tsv` & `.tab` files. |
| `QSV_SNIFF_DELIMITER` | if set, the delimiter is automatically detected. Overrides `QSV_DEFAULT_DELIMITER` & `--delimiter` option. Note that this does not work with stdin. |
| `QSV_NO_HEADERS` | if set, the first row will **NOT** be interpreted as headers. Supersedes `QSV_TOGGLE_HEADERS`. |
| `QSV_TOGGLE_HEADERS` | if set to `1`, toggles header setting - i.e. inverts qsv header behavior, with no headers being the default, & setting `--no-headers` will actually mean headers will not be ignored. |
| `QSV_AUTOINDEX` | if set, automatically create an index when none is detected. Also automatically updates stale indices. |
| `QSV_COMMENT_CHAR` | set to an ascii character. If set, any lines(including the header) that start with this character are ignored. |
| `QSV_MAX_JOBS` | number of jobs to use for multithreaded commands (currently `apply`, `dedup`, `extsort`, `frequency`, `schema`, `sort`, `split`, `stats`, `tojsonl` & `validate`). If not set, max_jobs is set to the detected number of logical processors.  See [Multithreading](docs/PERFORMANCE.md#multithreading) for more info. |
| `QSV_NO_UPDATE` | if set, prohibit self-update version check for the latest qsv release published on GitHub. |
| `QSV_PREFER_DMY` | if set, date parsing will use DMY format. Otherwise, use MDY format (used with `apply datefmt`, `schema`, `sniff` & `stats` commands). |
| `QSV_REGEX_UNICODE` | if set, makes `search`, `searchset` & `replace` commands unicode-aware. For increased performance, these commands are not unicode-aware by default & will ignore unicode values when matching & will abort when unicode characters are used in the regex. Note that the `apply operations regex_replace` operation is always unicode-aware. |
| `QSV_SKIPUTF8_CHECK` | if set, skip UTF-8 encoding check. Otherwise, for several commands that require UTF-8 encoded input (see [UTF8-Encoding](#utf-8-encoding)), qsv scans the first 8k. |
| `QSV_RDR_BUFFER_CAPACITY` | reader buffer size (default (bytes): 16384) |
| `QSV_WTR_BUFFER_CAPACITY` | writer buffer size (default (bytes): 65536) |
| `QSV_LOG_LEVEL` | desired level (default - off; `error`, `warn`, `info`, `trace`, `debug`). |
| `QSV_LOG_DIR` | when logging is enabled, the directory where the log files will be stored. If the specified directory does not exist, qsv will attempt to create it. If not set, the log files are created in the directory where qsv was started. See [Logging](docs/Logging.md#logging) for more info. |
| `QSV_PROGRESSBAR` | if set, enable the --progressbar option on the `apply`, `fetch`, `fetchpost`, `foreach`, `luau`, `py`, `replace`, `search`, `searchset`, `sortcheck` & `validate` commands.  |
| `QSV_REDIS_CONNSTR` | the `fetch` command can use [Redis](https://redis.io/) to cache responses. Set to connect to the desired Redis instance. (default: `redis:127.0.0.1:6379/1`). For more info on valid Redis connection string formats, see https://docs.rs/redis/latest/redis/#connection-parameters. |
| `QSV_FP_REDIS_CONNSTR` | the `fetchpost` command can also use Redis to cache responses (default: `redis:127.0.0.1:6379/2`). Note that `fetchpost` connects to database 2, as opposed to `fetch` which connects to database 1. |
| `QSV_REDIS_MAX_POOL_SIZE` | the maximum Redis connection pool size. (default: 20). |
| `QSV_REDIS_TTL_SECONDS` | set time-to-live of Redis cached values (default (seconds): 2419200 (28 days)). |
| `QSV_REDIS_TTL_REFRESH`| if set, enables cache hits to refresh TTL of cached values. |

Several dependencies also have environment variables that influence qsv's performance & behavior:

* Memory Management ([mimalloc](https://docs.rs/mimalloc/latest/mimalloc/))   
  When incorporating qsv into a data pipeline that runs in batch mode, particularly with very large CSV files using qsv commands that load entire CSV files into memory, you can
  [fine-tune Mimalloc's behavior using its environment variables](https://github.com/microsoft/mimalloc#environment-options).
* Network Access ([reqwest](https://docs.rs/reqwest/latest/reqwest/))   
  qsv uses reqwest for its `fetch`, `validate` & `--update` functions & will honor [proxy settings](https://docs.rs/reqwest/latest/reqwest/index.html#proxies) set through the `HTTP_PROXY`, `HTTPS_PROXY` & `NO_PROXY` environment variables.
  
> ℹ️ **NOTE:** To get a list of all active qsv-relevant environment variables, run `qsv --envlist`.
Relevant env vars are defined as anything that starts with `QSV_` & `MIMALLOC_` & the proxy variables listed above.

## Feature Flags

`qsv` has several features:

* `mimalloc` (default) - use the mimalloc allocator (see [Memory Allocator](docs/PERFORMANCE.md#memory-allocator) for more info).
* `apply` - enable `apply` command. This swiss-army knife of CSV transformations is very powerful, but it has a lot of dependencies that increases both compile time and binary size.
* `fetch` - enables the `fetch` & `fetchpost` commands.
* `foreach` - enable `foreach` command (not valid for Windows).
* `generate` - enable `generate` command.
* `luau` - enable `luau` command. Embeds a [Luau](https://luau-lang.org) interpreter into qsv. [Luau has type-checking, sandboxing, additional language operators, increased performance & other improvements](https://luau-lang.org/2022/11/04/luau-origins-and-evolution.html) over Lua.
* `python` - enable `py` command. Note that qsv will look for the shared library for the Python version (Python 3.6 & above supported) it was compiled against & will abort on startup if the library is not found, even if you're not using the `py` command. Check [Python](#python) section for more info.
* `to` - enables the `to` command.
* `self_update` - enable self-update engine, checking GitHub for the latest release. Note that if you manually built qsv, `self-update` will only check for new releases.
It will NOT offer the choice to update itself to the prebuilt binaries published on GitHub. You need not worry that your manually built qsv will be overwritten by a self-update.

* `full` - enable to build `qsv` binary variant which is feature-capable.
* `all_full` - enable to build `qsv` binary variant with all features enabled (apply,fetch,foreach,generate,luau,python,to,self_update).
* `lite` - enable to build `qsvlite` binary variant with all features disabled.
* `datapusher_plus` - enable to build `qsvdp` binary variant - the [DataPusher+](https://github.com/dathere/datapusher-plus) optimized qsv binary.
* `nightly` - enable to turn on nightly/unstable features in the `rand`, `regex`, `hashbrown`, `parking_lot` & `pyo3` crates when building with Rust nightly/unstable.


> ℹ️ **NOTE:** `qsvlite`, as the name implies, always has **non-default features disabled**. `qsv` can be built with any combination of the above features using the cargo `--features` & `--no-default-features` flags. The prebuilt `qsv` binaries has **all applicable features valid for the target platform**[^6].

## Testing
qsv has ~1,020 tests in the [tests](https://github.com/jqnatividad/qsv/tree/master/tests) directory.
Each command has its own test suite in a separate file with the convention `test_<COMMAND>.rs`.
Apart from preventing regressions, the tests also serve as good illustrative examples, and are often linked from the usage text of each corresponding command.

To test each binary variant:
```
# to test qsv
cargo test --feature all_full

# to test qsvlite
cargo test --feature lite

# to test qsvdp
cargo test --feature datapusher_plus

# to test a specific command
# here we test only stats and use the
# -F shortcut for --feature
cargo test stats -F all_full
```

## License

Dual-licensed under MIT or the [UNLICENSE](https://unlicense.org).

## Sponsor

<div align="center">

|qsv was made possible by|
:-------------------------:|
|[![datHere Logo](docs/images/datHere-logo-withtagline.png)](https://datHere.com)<br>|
|Standards-based, best-of-breed, open source solutions<br>to make your **Data Useful, Usable & Used.**   |

</div>

## Naming Collision

This project is unrelated to [Intel's Quick Sync Video](https://www.intel.com/content/www/us/en/architecture-and-technology/quick-sync-video/quick-sync-video-general.html).
