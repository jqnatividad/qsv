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
[![Minimum supported Rust version](https://img.shields.io/badge/Rust-1.68.2-red?logo=rust)](#minimum-supported-rust-version)

<div align="center">

 &nbsp;          |  Table of Contents
:--------------------------|:-------------------------
![qsv logo](docs/images/qsv-logo.png)<br/><sub><sup>[logo details](https://github.com/jqnatividad/qsv/discussions/295)</sup></sub>  |qsv is a command line program for<br>indexing, slicing, analyzing, filtering,<br>enriching, validating & joining CSV files.<br>Commands are simple, fast & composable.<br><br>* [Available Commands](#available-commands)<br>* [Installation Options](#installation-options)<br> * [Whirlwind Tour](docs/whirlwind_tour.md#a-whirlwind-tour)<br>* [Cookbook](https://github.com/jqnatividad/qsv/wiki/Cookbook#cookbook)<br>* [FAQ](https://github.com/jqnatividad/qsv/discussions/categories/faq)<br>* [Changelog](https://github.com/jqnatividad/qsv/blob/master/CHANGELOG.md#changelog)<br>* [Performance Tuning](https://github.com/jqnatividad/qsv/blob/master/docs/PERFORMANCE.md#performance-tuning)<br>* [Benchmarks](docs/BENCHMARKS.md)<br>* [Environment Variables](#environment-variables)<br>* [Feature Flags](#feature-flags)<br>* [Testing](#testing)<br>* [NYC School of Data 2022 slides](https://docs.google.com/presentation/d/e/2PACX-1vQ12ndZL--gkz0HLQRaxqsNOwzddkv1iUKB3sq661yA77OPlAsmHJHpjaqt9s9QEf73VqMfb0cv4jHU/pub?start=false&loop=false&delayms=3000)<br>* [Sponsor](#sponsor)

</div>

> ℹ️ **NOTE:** qsv is a fork of the popular [xsv](https://github.com/BurntSushi/xsv) utility, merging several pending PRs [since xsv 0.13.0's May 2018 release](https://github.com/BurntSushi/xsv/issues/267). On top of xsv's 20 commands, it adds numerous new features; 33 additional commands; 6 `apply` subcommands & 35 operations; 5 `to` subcommands; 3 `cat` subcommands; and 4 `snappy` subcommands (for a total of 105).
See [FAQ](https://github.com/jqnatividad/qsv/discussions/categories/faq) for more details.

## Available commands

| Command | Description |
| --- | --- |
| [apply](/src/cmd/apply.rs#L2)<br>✨🚀🧠 | Apply series of string, date, math, currency & geocoding transformations to a CSV column. It also has some basic [NLP](https://en.wikipedia.org/wiki/Natural_language_processing) functions ([similarity](https://crates.io/crates/strsim), [sentiment analysis](https://crates.io/crates/vader_sentiment), [profanity](https://docs.rs/censor/latest/censor/), [eudex](https://github.com/ticki/eudex#eudex-a-blazingly-fast-phonetic-reductionhashing-algorithm) & [language detection](https://crates.io/crates/whatlang)).  |
| <a name="applydp_deeplink"></a>[applydp](/src/cmd/applydp.rs#L2)<br>🚀 | applydp is a slimmed-down version of `apply` with only [Datapusher+](https://github.com/dathere/datapusher-plus) relevant subcommands/operations (`qsvdp` binary variant only). |
| [behead](/src/cmd/behead.rs#L2) | Drop headers from a CSV.  |
| [cat](/src/cmd/cat.rs#L2) | Concatenate CSV files by row or by column. |
| [count](/src/cmd/count.rs#L2)<br>📇 | Count the rows in a CSV file. (15.82 seconds for 15gb, 27m row NYC 311 dataset without an index. Instantaneous with an index.) |
| [dedup](/src/cmd/dedup.rs#L2)<br>🗜️🚀 | Remove duplicate rows (See also `extdedup`, `extsort`, `sort` & `sortcheck` commands). |
| [diff](/src/cmd/diff.rs#L2)<br>🚀 | Find the difference between two CSVs with ludicrous speed!<br/>e.g. *compare two CSVs with 1M rows x 9 columns in under 600ms!* |
| [enum](/src/cmd/enumerate.rs#L2) | Add a new column enumerating rows by adding a column of incremental or uuid identifiers. Can also be used to copy a column or fill a new column with a constant value.  |
| [excel](/src/cmd/excel.rs#L2) | Exports a specified Excel/ODS sheet to a CSV file. |
| [exclude](/src/cmd/exclude.rs#L2)<br>📇 | Removes a set of CSV data from another set based on the specified columns.  |
| [explode](/src/cmd/explode.rs#L2) | Explode rows into multiple ones by splitting a column value based on the given separator.  |
| [extdedup](/src/cmd/extdedup.rs#L2)<br> | Remove duplicate rows from an arbitrarily large CSV/text file using a memory-mapped, [on-disk hash table](https://crates.io/crates/odht). Unlike the `dedup` command, this command does not load the entire file into memory nor does it sort the deduped file. |
| [extsort](/src/cmd/extsort.rs#L2)<br>🚀 | Sort an arbitrarily large CSV/text file using a multithreaded [external merge sort](https://en.wikipedia.org/wiki/External_sorting) algorithm. |
| [fetch](/src/cmd/fetch.rs#L3)<br>✨🧠 | Fetches data from web services for every row using **HTTP Get**. Comes with [HTTP/2](https://http2-explained.haxx.se/en/part1) [adaptive flow control](https://medium.com/coderscorner/http-2-flow-control-77e54f7fd518), [jql](https://github.com/yamafaktory/jql#%EF%B8%8F-usage) JSON query language support, dynamic throttling ([RateLimit](https://tools.ietf.org/id/draft-polli-ratelimit-headers-00.html)) & caching with optional [Redis](https://redis.io/) support for persistent caching. |
| [fetchpost](/src/cmd/fetchpost.rs#L3)<br>✨🧠 | Similar to `fetch`, but uses **HTTP Post**. ([HTTP GET vs POST methods](https://www.geeksforgeeks.org/difference-between-http-get-and-post-methods/)) |
| [fill](/src/cmd/fill.rs#L2) | Fill empty values.  |
| [fixlengths](/src/cmd/fixlengths.rs#L2) | Force a CSV to have same-length records by either padding or truncating them. |
| [flatten](/src/cmd/flatten.rs#L2) | A flattened view of CSV records. Useful for viewing one record at a time.<br />e.g. `qsv slice -i 5 data.csv \| qsv flatten`. |
| [fmt](/src/cmd/fmt.rs#L2) | Reformat a CSV with different delimiters, record terminators or quoting rules. (Supports ASCII delimited data.)  |
| [foreach](/src/cmd/foreach.rs#L3)<br>✨ | Loop over a CSV to execute shell commands. (not available on Windows)  |
| [frequency](/src/cmd/frequency.rs#L2)<br>📇🪗🏎️ | Build [frequency tables](https://statisticsbyjim.com/basics/frequency-table/) of each column. Uses multithreading to go faster if an index is present. |
| [generate](/src/cmd/generate.rs#L2)<br>✨ | Generate test data by profiling a CSV using [Markov decision process](https://crates.io/crates/test-data-generation) machine learning.  |
| [headers](/src/cmd/headers.rs#L2) | Show the headers of a CSV. Or show the intersection of all headers between many CSV files. |
| [index](/src/cmd/index.rs#L2) | Create an index for a CSV. This is very quick (even the 15gb, 28m row NYC 311 dataset takes all of 15 seconds to index) & provides constant time indexing/random access into the CSV file. Accelerates  the `count`, `sample` & `slice` commands; enables random access mode in `luau`; and enables multithreading (🏎️) for the `frequency`, `split`, `stats`, `schema` & `tojsonl` commands. |
| [input](/src/cmd/input.rs#L2) | Read CSV data with special quoting, trimming, line-skipping & UTF-8 transcoding rules. Typically used to "normalize" a CSV for further processing with other qsv commands. |
| [join](/src/cmd/join.rs#L2)<br>📇 | Inner, outer, cross, anti & semi joins. Automatically creates a simple, in-memory hash index to make it fast.  |
| [joinp](/src/cmd/joinp.rs#L2)<br>✨🚀🐻‍❄️ | Inner, left, outer, cross, anti & semi joins using the [Pola.rs](https://www.pola.rs) engine. Unlike `join`, it can process files larger than RAM, is multi-threaded & the output does not have duplicate columns. However, it cannot do case-insensitive joins. |
| [jsonl](/src/cmd/jsonl.rs#L2) | Convert newline-delimited JSON ([JSONL](https://jsonlines.org/)/[NDJSON](http://ndjson.org/)) to CSV. See `tojsonl` command to convert CSV to JSONL.
| <a name="luau_deeplink"></a><br>[luau](/src/cmd/luau.rs#L2) 👑<br>✨📇 | Create multiple new computed columns, filter rows, compute aggregations and build complex data pipelines by executing a [Luau](https://luau-lang.org) [0.573](https://github.com/Roblox/luau/releases/tag/0.573) expression/script for every row of a CSV file ([sequential mode](https://github.com/jqnatividad/qsv/blob/bb72c4ef369d192d85d8b7cc6e972c1b7df77635/tests/test_luau.rs#L254-L298)), or using [random access](https://www.webopedia.com/definitions/random-access/) with an index ([random access mode](https://github.com/jqnatividad/qsv/blob/bb72c4ef369d192d85d8b7cc6e972c1b7df77635/tests/test_luau.rs#L367-L415)).<br>Can process a single Luau expression or [full-fledged data-wrangling scripts using lookup tables](https://github.com/dathere/qsv-lookup-tables#example) with discrete BEGIN, MAIN and END sections.<br> It is not just another qsv command, it is qsv's [Domain-specific Language](https://en.wikipedia.org/wiki/Domain-specific_language) (DSL) with [numerous qsv-specific helper functions](https://github.com/jqnatividad/qsv/blob/113eee17b97882dc368b2e65fec52b86df09f78b/src/cmd/luau.rs#L1356-L2290) to build production data pipelines. |
| [partition](/src/cmd/partition.rs#L2) | Partition a CSV based on a column value. |
| [pseudo](/src/cmd/pseudo.rs#L2) | [Pseudonymise](https://en.wikipedia.org/wiki/Pseudonymization) the value of the given column by replacing them with an incremental identifier.  |
| [py](/src/cmd/python.rs#L2)<br>✨ | Create a new computed column or filter rows by evaluating a python expression on every row of a CSV file. Python's [f-strings](https://www.freecodecamp.org/news/python-f-strings-tutorial-how-to-use-f-strings-for-string-formatting/) is particularly useful for extended formatting, [with the ability to evaluate Python expressions as well](https://github.com/jqnatividad/qsv/blob/4cd00dca88addf0d287247fa27d40563b6d46985/src/cmd/python.rs#L23-L31). |
| [rename](/src/cmd/rename.rs#L2) |  Rename the columns of a CSV efficiently.  |
| [replace](/src/cmd/replace.rs#L2) | Replace CSV data using a regex.  |
| [reverse](/src/cmd/reverse.rs#L2)<br>🗜️ | Reverse order of rows in a CSV. Unlike the `sort --reverse` command, it preserves the order of rows with the same key.  |
| [safenames](/src/cmd/safenames.rs#L2) | Modify headers of a CSV to only have ["safe" names](/src/cmd/safenames.rs#L5-L14) - guaranteed "database-ready" names.  |
| [sample](/src/cmd/sample.rs#L2)<br>📇 | Randomly draw rows (with optional seed) from a CSV using [reservoir sampling](https://en.wikipedia.org/wiki/Reservoir_sampling) (i.e., use memory proportional to the size of the sample).  |
| [schema](/src/cmd/schema.rs#L2)<br>📇🪗🏎️ | Infer schema from CSV data, replete with data type & domain/range validation & output in [JSON Schema](https://json-schema.org/) format. Uses multithreading to go faster if an index is present. See `validate` command to use the generated JSON Schema to validate if similar CSVs comply with the schema. |
| [search](/src/cmd/search.rs#L2) | Run a regex over a CSV. Applies the regex to each field individually & shows only matching rows.  |
| [searchset](/src/cmd/searchset.rs#L3) | *Run multiple regexes over a CSV in a single pass.* Applies the regexes to each field individually & shows only matching rows.  |
| [select](/src/cmd/select.rs#L2) | Select, re-order, duplicate or drop columns.  |
| [slice](/src/cmd/slice.rs#L2)<br>📇 | Slice rows from any part of a CSV. When an index is present, this only has to parse the rows in the slice (instead of all rows leading up to the start of the slice).  |
| <a name="snappy_deeplink"></a>[snappy](/src/cmd/snappy.rs#L2)<br>🚀 | Does streaming compression/decompression of the input using Google's [Snappy](https://github.com/google/snappy/blob/main/docs/README.md) framing format ([more info](#snappy-compressiondecompression)). |
| [sniff](/src/cmd/sniff.rs#L2) | Quickly sniff & infer CSV metadata (delimiter, header row, preamble rows, quote character, flexible, is_utf8, average record length, number of records, content length & estimated number of records if sniffing a CSV on a URL, number of fields, field names & data types). |
| [sort](/src/cmd/sort.rs#L2)<br>🚀🗜️ | Sorts CSV data in alphabetical (with case-insensitive option), numerical, reverse, unique or random (with optional seed) order (See also `extsort` & `sortcheck` commands).  |
| [sortcheck](/src/cmd/sortcheck.rs#L2)<br>📇 | Check if a CSV is sorted. With the --json options, also retrieve record count, sort breaks & duplicate count. |
| [split](/src/cmd/split.rs#L2)<br>📇🏎️ | Split one CSV file into many CSV files of N chunks. Uses multithreading to go faster if an index is present. |
| [stats](/src/cmd/stats.rs#L2)<br>📇🗜️🏎️ | Compute [summary statistics](https://en.wikipedia.org/wiki/Summary_statistics) (sum, min/max/range, min/max length, mean, stddev, variance, nullcount, sparsity, quartiles, IQR, lower/upper fences, skewness, median, mode/s, antimode/s & cardinality) & make GUARANTEED data type inferences (Null, String, Float, Integer, Date, DateTime) for each column in a CSV.<br>Uses multithreading to go faster if an index is present (with an index, can compile "streaming" stats on NYC's 311 data (15gb, 28m rows) in less than 20 seconds). |
| [table](/src/cmd/table.rs#L2)<br>🗜️ | Show aligned output of a CSV using [elastic tabstops](https://github.com/BurntSushi/tabwriter).  |
| [to](/src/cmd/to.rs#L2)<br>✨🚀 | Convert CSV files to [PostgreSQL](https://www.postgresql.org), [SQLite](https://www.sqlite.org/index.html), XLSX, [Parquet](https://parquet.apache.org) and [Data Package](https://datahub.io/docs/data-packages/tabular). |
| [tojsonl](/src/cmd/tojsonl.rs#L3)<br>📇🪗🏎️ | Smartly converts CSV to a newline-delimited JSON ([JSONL](https://jsonlines.org/)/[NDJSON](http://ndjson.org/)). By scanning the CSV first, it "smartly" infers the appropriate JSON data type for each column. See `jsonl` command to convert JSONL to CSV. Uses multithreading to go faster if an index is present. |
| [transpose](/src/cmd/transpose.rs#L2)<br>🗜️ | Transpose rows/columns of a CSV.  |
| [validate](/src/cmd/validate.rs#L2)<br>📇🚀 | Validate CSV data blazingly-fast using [JSON Schema Validation](https://json-schema.org/draft/2020-12/json-schema-validation.html) & put invalid records into a separate file with an accompanying detailed validation error report file (e.g. *up to 350,000 rows/second* using [NYC's 311 schema](https://github.com/jqnatividad/qsv/blob/master/resources/test/311_Service_Requests_from_2010_to_Present-2022-03-04.csv.schema.json) generated by the `schema` command).<br>If no JSON schema file is provided, validates if a CSV conforms to the [RFC 4180 standard](#rfc-4180-csv-standard). |

<div style="text-align: right"><sub><sup>Performance metrics compiled on an M2 Pro 12-core Mac Mini with 32gb RAM</sup></sub></div>

 ✨: enabled by a feature flag on `qsv`. Not available on `qsvlite` or `qsvdp`.   
📇: uses an index when available. `join` creates its own in-memory index automatically.   
🗜️: loads entire CSV into memory, though `dedup`, `stats` & `transpose` have "streaming" modes as well.   
🪗: uses additional memory proportional to the cardinality of the columns in the CSV.   
🧠: expensive operations are memoized (cached) with available inter-session Redis caching for fetch commands.    
🐻‍❄️: command powered by [Pola.rs](https://pola.rs) engine.   
🏎️: multithreaded when an index is available.   
🚀: multithreaded even without an index.

## Installation Options

### Option 1: Download Prebuilt Binaries

Full-featured prebuilt [binary variants](#variants) of the latest qsv version for Windows, Linux & macOS are available [for download](https://github.com/jqnatividad/qsv/releases/latest), including binaries compiled with [Rust Nightly](https://stackoverflow.com/questions/70745970/rust-nightly-vs-beta-version) ([more info](https://github.com/jqnatividad/qsv/blob/master/docs/PERFORMANCE.md#nightly-release-builds)).

### Option 2: Homebrew

For [macOS and Linux (64-bit)](https://formulae.brew.sh/formula/qsv), you can quickly install qsv with [Homebrew](https://brew.sh). However, only the `apply` and `luau` [features](#feature-flags) are enabled.

```
brew install qsv
```

### Option 3: Install with Rust

If you have [Rust installed](https://www.rust-lang.org/tools/install), you can also install from source using Rust's cargo command[^1]:

[^1]: Of course, you'll also need a linker & a C compiler. Linux users should generally install GCC or Clang, according to their distribution’s documentation.
For example, if you use Ubuntu, you can install the `build-essential` package. On macOS, you can get a C compiler by running `$ xcode-select --install`.
For Windows, this means installing [Visual Studio 2022](https://visualstudio.microsoft.com/downloads/). When prompted for workloads, include "Desktop Development with C++",
the Windows 10 or 11 SDK & the English language pack, along with any other language packs your require.

```bash
cargo install qsv --locked --features all_features
```

The binary will be installed in `~/.cargo/bin`.

To install different [variants](#variants) and enable optional features, use cargo `--features` (see [Feature Flags](#feature-flags) for more info):

```bash
# to install qsv with all features enabled
cargo install qsv --locked --features apply,generate,luau,fetch,foreach,python,to,self_update,feature_capable
# or shorthand
cargo install qsv --locked -F all_features

# or to install qsvlite
cargo install qsv --locked -F lite

# or to install qsvdp
cargo install qsv --locked -F datapusher_plus,luau
```

### Option 4: Compile from Source

Compiling from source also works similarly[^1]:

```bash
git clone https://github.com/jqnatividad/qsv.git
cd qsv
cargo build --release --locked --features all_features
```

The compiled binary will end up in `./target/release/`.

To compile different [variants](#variants) and enable optional [features](#feature-flags):

```bash
# to compile qsv with all features enabled
cargo build --release --locked --features apply,generate,luau,fetch,foreach,python,to,self_update,feature_capable
# shorthand
cargo build --release --locked -F all_features

# for qsvlite
cargo build --release --locked -F lite

# for qsvdp
cargo build --release --locked -F datapusher_plus,luau
```

### Variants

There are three binary variants of qsv:
* `qsv` - [feature](#feature-flags)-capable(✨), with the [prebuilt binaries](https://github.com/jqnatividad/qsv/releases/latest) enabling all applicable features except Python [^2]
* `qsvlite` - all features disabled (~13% of the size of `qsv`)
* `qsvdp` - optimized for use with [DataPusher+](https://github.com/dathere/datapusher-plus) with only DataPusher+ relevant commands; an embedded [`luau`](#luau_deeplink) interpreter; [`applydp`](#applydp_deeplink), a slimmed-down version of the `apply` feature; the `--progressbar` option disabled; and the self-update only checking for new releases, requiring an explicit `--update` (~12% of the the size of `qsv`).

[^2]: The `foreach` feature is not available on Windows. The `python` feature is not enabled on the prebuilt binaries. Compile qsv with Python development environment installed if you want to enable the `python` feature (Python 3.7 & above supported). The `luau` feature is enabled by default on the prebuilt binaries if the platform supports it.  

## Regular Expression Syntax

The `--select` option and several commands (`apply`, `schema`, `search`, `searchset`, `select` & `replace`) allow the user to specify regular expressions. We use the [`regex`](https://docs.rs/regex) crate to parse, compile and execute these expressions. [^3]

[^3]: This is the same regex engine used by [`ripgrep`](https://github.com/BurntSushi/ripgrep#ripgrep-rg) - the [blazingly fast grep replacement](https://blog.burntsushi.net/ripgrep/) that powers Visual Studio's [magical](https://lab.cccb.org/en/arthur-c-clarke-any-sufficiently-advanced-technology-is-indistinguishable-from-magic/) ["Find in Files"](https://github.com/microsoft/vscode-ripgrep) feature.

Its syntax can be found [here](https://docs.rs/regex/latest/regex/#syntax) and *"is similar to Perl-style regular expressions, but lacks a few features like look around and back references. In exchange, all searches execute in linear time with respect to the size of the regular expression and search text."*

 If you want to test your regular expressions, [regex101](https://regex101.com) supports the syntax used by the `regex` crate. Just select the "Rust" flavor.

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

## Snappy Compression/Decompression

qsv supports *automatic compression/decompression* using the [Snappy frame format](https://github.com/google/snappy/blob/main/framing_format.txt). Snappy was chosen instead of more popular compression formats like gzip because it was designed for [high-performance streaming compression & decompression](https://github.com/google/snappy/tree/main/docs#readme).

For all commands except the `index`, `extdedup` & `extsort` commands, if the input file has an ".sz" extension, qsv will *automatically* do streaming decompression as it reads it. Further, if the input file has an extended CSV/TSV ".sz" extension (e.g nyc311.csv.sz/nyc311.tsv.sz/nyc311.tab.sz), qsv will also use the file extension to determine the delimiter to use.   

Similarly, if the `--output` file has an ".sz" extension, qsv will *automatically* do streaming compression as it writes it.
If the output file has an extended CSV/TSV ".sz" extension, qsv will also use the file extension to determine the delimiter to use.  

Note however that compressed files cannot be indexed, so index-accelerated commands (`frequency`, `schema`, `split`, `stats`, `tojsonl`) will not be multi-threaded. Random access is also disabled without an index, so `slice` will not be accelerated and `luau`'s random-access mode will not be available.

There is also a dedicated [`snappy`](/src/cmd/snappy.rs#L2) command with four subcommands for direct snappy file operations — a multithreaded `compress` subcommand (4-5x faster than the built-in, single-threaded auto-compression); a `decompress` subcommand with detailed compression metadata; a `check` subcommand to quickly inspect if a file has a Snappy header; and a `validate` subcommand to confirm if a Snappy file is valid.

The `snappy` command can be used to compress/decompress ANY file, not just CSV/TSV files.

Using the `snappy` command, we can compress NYC's 311 data (15gb, 28m rows) to 4.95 gb in *5.77 seconds* with the multithreaded `compress` subcommand - *2.58 gb/sec* with a 0.33 (3.01:1) compression ratio.  With `snappy decompress`, we can roundtrip decompress the same file in *16.71 seconds* - *0.89 gb/sec*.

Compare that to [zip 3.0](https://infozip.sourceforge.net/Zip.html), which compressed the same file to 2.9 gb in *248.3 seconds - 43x slower at 0.06 gb/sec* with a 0.19 (5.17:1) compression ratio - for just an additional 14% (2.45 gb) of saved space. zip also took 4.3x longer to roundtrip decompress the same file in *72 seconds* - *0.20 gb/sec*.

## RFC 4180 CSV Standard

qsv validates against the [RFC 4180](https://datatracker.ietf.org/doc/html/rfc4180) CSV standard. However IRL, CSV formats vary significantly & qsv is actually not strictly compliant with the specification so it can process "real-world" CSV files.
qsv leverages the awesome [Rust CSV](https://docs.rs/csv/latest/csv/) crate to read/write CSV files.

Click [here](https://docs.rs/csv-core/latest/csv_core/struct.Reader.html#rfc-4180) to find out more about how qsv conforms to the standard using this crate.

## UTF-8 Encoding

qsv requires UTF-8 encoded input (of which ASCII is a subset).

Should you need to re-encode CSV/TSV files, you can use the `input` command to transcode to UTF-8. It will replace all invalid UTF-8 sequences with `�` ([U+FFFD REPLACEMENT CHARACTER](https://doc.rust-lang.org/std/char/constant.REPLACEMENT_CHARACTER.html)). Alternatively, there are several utilities you can use to do so on [Linux/macOS](https://stackoverflow.com/questions/805418/how-can-i-find-encoding-of-a-file-via-a-script-on-linux) & [Windows](https://superuser.com/questions/1163753/converting-text-file-to-utf-8-on-windows-command-prompt).

### Windows Usage Note

Unlike other modern operating systems, Microsoft Windows' [default encoding is UTF16-LE](https://stackoverflow.com/questions/66072117/why-does-windows-use-utf-16le). This will cause problems when redirecting qsv's output to a CSV file & trying to open it with Excel (which ignores the comma delimiter, with everything in the first column):

```
# the following command will produce a UTF16-LE encoded CSV file on Windows
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
For complex data-wrangling tasks, you can use Luau and Python scripts.
### Luau

[Luau](https://luau-lang.org) is a fast, small, safe, gradually typed, statically linked, embeddable scripting language derived from [Lua](https://www.lua.org/about.html). It lies at the [heart of Roblox technology](https://luau-lang.org/2022/11/04/luau-origins-and-evolution.html) - powering all it's user generated content, with [Roblox](https://en.wikipedia.org/wiki/Roblox)'s own internal code having more than 2 millions lines of Luau. 

It has [sandboxing](https://luau-lang.org/sandbox), [type-checking](https://luau-lang.org/typecheck), [additional operators](https://luau-lang.org/syntax) & [increased performance](https://luau-lang.org/performance) while [maintaining compatibility with Lua](https://luau-lang.org/compatibility).

[Lua is faster than Python](https://benchmarksgame-team.pages.debian.net/benchmarksgame/fastest/lua-python3.html) & Luau is even faster still - more so, as qsv precompiles Luau into bytecode. In addition, [`luau`](/src/cmd/luau.rs#L2) is embedded into qsv, has debug logging, can do aggregations with its `--begin` & `--end` options & has no external dependencies unlike the `py` command.

It also allows mapping of multiple new computed columns, supports random access with indexed CSV files, and has [several helper functions](https://github.com/jqnatividad/qsv/blob/c0c2d5ab3e4ea9cc0e861c6ad41652677ffc4f20/src/cmd/luau.rs#L1250-L1931) to help ease the development of [full-fledged data-wrangling scripts](https://github.com/jqnatividad/qsv/blob/4e521b177ea3a6a06c83222458bb1349a67606f4/tests/test_luau.rs#L524-L571).

As date manipulation is often needed, we're also preloading the [LuaDate](https://tieske.github.io/date/) module.

Finally, as [qsv's DSL](#luau_deeplink) (👑), `luau` will gain even more features over time compared to the `python` feature.

[Luau 0.573](https://github.com/Roblox/luau/releases/tag/0.573) is currently embedded - qsv's policy is to use the latest stable Luau version at the time of each qsv release.

### Python

The `python` feature is NOT enabled by default on the prebuilt binaries, as doing so requires it to dynamically link to python libraries at runtime, which presents distribution issues, as various operating systems have differing Python versions.

If you wish to enable the `python` feature - you'll just have to install/compile from source, making sure you have the development libraries for the desired Python version (Python 3.7 to 3.11 are supported) installed when doing so.

If you plan to distribute your manually built `qsv` with the `python` feature, `qsv` will look for the specific version of Python shared libraries (libpython* on Linux/macOS, python*.dll on Windows) against which it was compiled starting with the current directory & abort with an error if not found, detailing the Python library it was looking for. 

Note that this will happen on qsv startup, even if you're not running the `py` command.

When building from source - [PyO3](https://pyo3.rs) - the underlying crate that enables the `python` feature, uses a build script to determine the Python version & set the correct linker arguments. By default it uses the python3 executable.
You can override this by setting `PYO3_PYTHON` (e.g., `PYO3_PYTHON=python3.7`), before installing/compiling qsv. See the [PyO3 User Guide](https://pyo3.rs/v0.17.1/building_and_distribution.html) for more information.

Consider using the [`luau`](/src/cmd/luau.rs#L2) command instead of the [`py`]((/src/cmd/python.rs#L2)) command if the operation you're trying to do can be done with `luau` - as `luau` is much faster than `py`, can do aggregations, supports random access, and allows mapping of multiple new columns. 

The `py` command cannot do aggregations because [PyO3's GIL-bound memory](https://pyo3.rs/v0.17.2/memory.html#gil-bound-memory) limitations will quickly consume a lot of memory (see [issue 449](https://github.com/jqnatividad/qsv/issues/449#issuecomment-1226095316) for details).
To prevent this, the `py` command processes CSVs in batches (default: 30,000 records), with a GIL pool for each batch, so no globals are available across batches.

## Memory Management
Most qsv commands use a "streaming" approach to processing CSVs - "streaming" in the input record-by-record while processing it. This allows it to process arbitrarily large CSVs with constant memory.

There are a number of commands/modes however (denoted by the clamp emoji - 🗜️), that require qsv to load the entire CSV into memory - `dedup` (when not using the --sorted option), `reverse`, `sort`, `stats` (when calculating the "non-streaming" extended stats), `table` and `transpose` (when not running in --multipass mode).

> NOTE: Though not as flexible, `dedup` and `sort` have corresponding "external" versions - `extdedup` and `extsort` respectively, that use external memory (i.e. disk) to process arbitrarily large CSVs.

In addition, `frequency`, `schema` and `tojsonl` - though they do not load the entire file into memory, uses additional memory proportional to the cardinality (number of unique values) of each column compared to other "streaming" commands (denoted by the accordion emoji - 🪗).

For very large files, this can be a problem, as qsv will run of memory and crash.
To prevent this, qsv has two memory check heuristics when running "non-streaming" commands:

### NORMAL mode
1. at startup, get the TOTAL memory of the system
2. if the size of the CSV file is greater than TOTAL memory - HEADROOM (default: 20%), qsv will abort with an error

### CONSERVATIVE mode
1. at startup, compute total available memory by adding the current available memory and free swap space 
2. subtract a percentage headroom from the total available memory (default: 20%)
3. if this adjusted total available memory is less than the size of the CSV file, qsv will abort with an error

The percentage headroom can be changed by setting the `QSV_MEMORY_HEADROOM_PCT` environment variable to a value between 10 and 90 (default: 20).

This CONSERVATIVE heuristic can have false positives however, as modern operating systems can do a fair bit of juggling to handle file sizes larger than what this heuristic will allow, as it dynamically swaps apps to the swapfile, expand the swapfile, compress memory, etc.

For example, on a 16gb Mac mini running several common apps, it only allowed ~3gb csv files, but in practice, it was able to handle files up to 8gb before this heuristic was added.

To apply this CONSERVATIVE heuristic, you can use the command's `--memcheck` option or set the `QSV_MEMORY_CHECK` environment variable.

Otherwise, the default memory check heuristic (NORMAL mode) will only check if the input file's size is larger than the TOTAL memory of the computer minus `QSV_MEMORY_HEADROOM_PCT`.  We still do this to prevent OOM panics, but it's not as restrictive as the CONSERVATIVE heuristic. (e.g. if you have a 16gb computer, the maximum input file size is 12.8gb file - 16gb minus 20% headroom).

> NOTE: These memory checks are not invoked when using stdin as input, as the size of the input file is not known. Though `schema` and `tojsonl` will still abort if stdin is too large per this memory check as it creates a temporary file from stdin before inferring the schema.

## Environment Variables

| Variable | Description |
| --- | --- |
| `QSV_DEFAULT_DELIMITER` | single ascii character to use as delimiter.  Overrides `--delimeter` option. Defaults to "," (comma) for CSV files & "\t" (tab) for TSV files when not set. Note that this will also set the delimiter for qsv's output to stdout.<br>However, using the `--output` option, regardless of this environment variable, will automatically change the delimiter used in the generated file based on the file extension - i.e. comma for `.csv`, tab for `.tsv` & `.tab` files. |
| `QSV_SNIFF_DELIMITER` | if set, the delimiter is automatically detected. Overrides `QSV_DEFAULT_DELIMITER` & `--delimiter` option. Note that this does not work with stdin. |
| `QSV_NO_HEADERS` | if set, the first row will **NOT** be interpreted as headers. Supersedes `QSV_TOGGLE_HEADERS`. |
| `QSV_TOGGLE_HEADERS` | if set to `1`, toggles header setting - i.e. inverts qsv header behavior, with no headers being the default, & setting `--no-headers` will actually mean headers will not be ignored. |
| `QSV_AUTOINDEX` | if set, automatically create an index when none is detected. Also automatically updates stale indices. |
| `QSV_CACHE_DIR` | The directory to use for caching downloaded lookup_table resources using the `luau` qsv_register_lookup() helper function. |
| `QSV_CKAN_API` | The CKAN Action API endpoint to use with the `luau` qsv_register_lookup() helper function when using the "ckan://" scheme. |
| `QSV_CKAN_TOKEN`| The CKAN token to use with the `luau` qsv_register_lookup() helper function when using the "ckan://" scheme. Only required to access private resources. |
| `QSV_COMMENT_CHAR` | set to an ascii character. If set, any lines(including the header) that start with this character are ignored. |
| `QSV_MAX_JOBS` | number of jobs to use for multithreaded commands (currently `apply`, `applydp`, `dedup`, `diff`, `extsort`, `frequency`, `joinp`, `schema`, `snappy`, `sort`, `split`, `stats`, `to`, `tojsonl` & `validate`). If not set, max_jobs is set to the detected number of logical processors.  See [Multithreading](docs/PERFORMANCE.md#multithreading) for more info. |
| `QSV_NO_UPDATE` | if set, prohibit self-update version check for the latest qsv release published on GitHub. |
| `QSV_PREFER_DMY` | if set, date parsing will use DMY format. Otherwise, use MDY format (used with `apply datefmt`, `schema`, `sniff` & `stats` commands). |
| `QSV_REGEX_UNICODE` | if set, makes `search`, `searchset` & `replace` commands unicode-aware. For increased performance, these commands are not unicode-aware by default & will ignore unicode values when matching & will abort when unicode characters are used in the regex. Note that the `apply operations regex_replace` operation is always unicode-aware. |
| `QSV_RDR_BUFFER_CAPACITY` | reader buffer size (default (bytes): 16384) |
| `QSV_WTR_BUFFER_CAPACITY` | writer buffer size (default (bytes): 65536) |
| `QSV_FREEMEMORY_HEADROOM_PCT` | the percentage of free available memory required when running qsv in "non-streaming" mode (i.e. the entire file needs to be loaded into memory). If the incoming file is greater than the available memory after the headroom is subtracted, qsv will not proceed. See [Memory Management](#memory-management) for more info. (default: (percent) 20 ) |
| `QSV_MEMORY_CHECK` | if set, check if input file size < AVAILABLE memory - HEADROOM (CONSERVATIVE mode) when running in "non-streaming" mode. Otherwise, qsv will only check if the input file size < TOTAL memory - HEADROOM (NORMAL mode). This is done to prevent Out-of-Memory errors. See [Memory Management](#memory-management) for more info. |
| `QSV_LOG_LEVEL` | desired level (default - off; `error`, `warn`, `info`, `trace`, `debug`). |
| `QSV_LOG_DIR` | when logging is enabled, the directory where the log files will be stored. If the specified directory does not exist, qsv will attempt to create it. If not set, the log files are created in the directory where qsv was started. See [Logging](docs/Logging.md#logging) for more info. |
| `QSV_LOG_UNBUFFERED` | if set, log messages are written directly to disk, without buffering. Otherwise, log messages are buffered before being written to the log file (8k buffer, flushing every second). See [flexi_logger](https://docs.rs/flexi_logger/latest/flexi_logger/enum.WriteMode.html) for details. |
| `QSV_PROGRESSBAR` | if set, enable the --progressbar option on the `apply`, `fetch`, `fetchpost`, `foreach`, `luau`, `py`, `replace`, `search`, `searchset`, `sortcheck` & `validate` commands.  |
| `QSV_REDIS_CONNSTR` | the `fetch` command can use [Redis](https://redis.io/) to cache responses. Set to connect to the desired Redis instance. (default: `redis:127.0.0.1:6379/1`). For more info on valid Redis connection string formats, click [here](https://docs.rs/redis/latest/redis/#connection-parameters). |
| `QSV_FP_REDIS_CONNSTR` | the `fetchpost` command can also use Redis to cache responses (default: `redis:127.0.0.1:6379/2`). Note that `fetchpost` connects to database 2, as opposed to `fetch` which connects to database 1. |
| `QSV_REDIS_MAX_POOL_SIZE` | the maximum Redis connection pool size. (default: 20). |
| `QSV_REDIS_TTL_SECONDS` | set time-to-live of Redis cached values (default (seconds): 2419200 (28 days)). |
| `QSV_REDIS_TTL_REFRESH`| if set, enables cache hits to refresh TTL of cached values. |
| `QSV_TIMEOUT`| for commands with a --timeout option (`fetch`, `fetchpost`, `luau`, `sniff` and `validate`), the number of seconds before a web request times out (default: 30). |

Several dependencies also have environment variables that influence qsv's performance & behavior:

* Memory Management ([mimalloc](https://docs.rs/mimalloc/latest/mimalloc/))   
  When incorporating qsv into a data pipeline that runs in batch mode, particularly with very large CSV files using qsv commands that load entire CSV files into memory, you can
  [fine-tune Mimalloc's behavior using its environment variables](https://github.com/microsoft/mimalloc#environment-options).
* Network Access ([reqwest](https://docs.rs/reqwest/latest/reqwest/))   
  qsv uses reqwest for its `fetch`, `validate` & `--update` functions & will honor [proxy settings](https://docs.rs/reqwest/latest/reqwest/index.html#proxies) set through the `HTTP_PROXY`, `HTTPS_PROXY` & `NO_PROXY` environment variables.
  
> ℹ️ **NOTE:** To get a list of all active qsv-relevant environment variables, run `qsv --envlist`.
Relevant env vars are defined as anything that starts with `QSV_` & `MIMALLOC_` & the proxy variables listed above.

### .env File Support
qsv supports the use of `.env` files to set environment variables. The `.env` file is a simple text file that contains key-value pairs, one per line. 

It processes `.env` files as follows:

* Upon invocation, qsv will look for a file named `.env` in the current working directory. If one is found, it will be processed.
* If no `.env` file is not found in the current working directory, qsv will next look for an `.env` file with the same filestem as the binary in the directory where the binary is (e.g. if `qsv`/`qsvlite`/`qsvdp` is in `/usr/local/bin`, it will look for `/usr/local/bin/qsv.env`, `/usr/local/bin/qsvlite.env` or `/usr/local/bin/qsvdp.env` respectively).
* If no `.env` files are found, qsv will proceed with its default settings and the current environment variables, which may include "QSV_" variables.

When processing `.env` files, qsv will:
* overwrite any existing environment variables with the same name
* where multiple declarations of the same variable exist, the last one will be used
* ignore any lines that start with `#` (comments)

To facilitate the use of `.env` files, a [`dotenv.template.yaml`](dotenv.template.yaml) file is included in the qsv distribution. This file contains all the environment variables that qsv recognizes, along with their default values. Copy the template a file named '.env' and modify it to suit your needs.

## Feature Flags

`qsv` has several features:

* `mimalloc` (default) - use the mimalloc allocator (see [Memory Allocator](docs/PERFORMANCE.md#memory-allocator) for more info).
* `jemallocator` - use the jemalloc allocator (see [Memory Allocator](docs/PERFORMANCE.md#memory-allocator) for more info).
* `apply` - enable `apply` command. This swiss-army knife of CSV transformations is very powerful, but it has a lot of dependencies that increases both compile time and binary size.
* `fetch` - enables the `fetch` & `fetchpost` commands.
* `foreach` - enable `foreach` command (not valid for Windows).
* `generate` - enable `generate` command.
* `luau` - enable `luau` command. Embeds a [Luau](https://luau-lang.org) interpreter into qsv. [Luau has type-checking, sandboxing, additional language operators, increased performance & other improvements](https://luau-lang.org/2022/11/04/luau-origins-and-evolution.html) over Lua.
* `polars` - enables all [Polars](https://pola.rs)-powered commands (currently, only `joinp`). Note that Polars is a very powerful library, but it has a lot of dependencies that drastically increases both compile time and binary size.
* `python` - enable `py` command. Note that qsv will look for the shared library for the Python version (Python 3.7 & above supported) it was compiled against & will abort on startup if the library is not found, even if you're not using the `py` command. Check [Python](#python) section for more info.
* `to` - enables the `to` command. Note that enabling this feature will also noticeably increase both compile time and binary size.
* `self_update` - enable self-update engine, checking GitHub for the latest release. Note that if you manually built qsv, `self-update` will only check for new releases.
It will NOT offer the choice to update itself to the prebuilt binaries published on GitHub. You need not worry that your manually built qsv will be overwritten by a self-update.

* `feature_capable` - enable to build `qsv` binary variant which is feature-capable.
* `all_features` - enable to build `qsv` binary variant with all features enabled (apply,fetch,foreach,generate,luau,python,to,self_update).
* `lite` - enable to build `qsvlite` binary variant with all features disabled.
* `datapusher_plus` - enable to build `qsvdp` binary variant - the [DataPusher+](https://github.com/dathere/datapusher-plus) optimized qsv binary.
* `nightly` - enable to turn on nightly/unstable features in the `rand`, `regex`, `hashbrown`, `parking_lot`, `polars` & `pyo3` crates when building with Rust nightly/unstable.

> ℹ️ **NOTE:** `qsvlite`, as the name implies, always has **non-default features disabled**. `qsv` can be built with any combination of the above features using the cargo `--features` & `--no-default-features` flags. The prebuilt `qsv` binaries has **all applicable features valid for the target platform**[^2].

## Minimum Supported Rust Version

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

## Testing
qsv has ~1,090 tests in the [tests](https://github.com/jqnatividad/qsv/tree/master/tests) directory.
Each command has its own test suite in a separate file with the convention `test_<COMMAND>.rs`.
Apart from preventing regressions, the tests also serve as good illustrative examples, and are often linked from the usage text of each corresponding command.

To test each binary variant:

```bash
# to test qsv
cargo test --features all_features

# to test qsvlite
cargo test --features lite

# to test qsvdp
cargo test --features datapusher_plus,luau

# to test a specific command
# here we test only stats and use the
# t alias for test and the -F shortcut for --features
cargo t stats -F all_features
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
