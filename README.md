## qsv: Blazing-fast CSV data-wrangling toolkit

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
[![Minimum supported Rust version](https://img.shields.io/badge/Rust-1.73.0-red?logo=rust)](#minimum-supported-rust-version)

<div align="center">

 &nbsp;          |  Table of Contents
:--------------------------|:-------------------------
![qsv logo](docs/images/qsv-logo.png)<br/>[_Hi-ho "Quicksilver" away!_](https://www.youtube.com/watch?v=p9lf76xOA5k)<br/><sub><sup>[logo details](https://github.com/jqnatividad/qsv/discussions/295)</sup></sub><br/>|qsv (pronounced "Quicksilver") is a command line program<br>for querying, indexing, slicing, analyzing, filtering, enriching,<br>transforming, sorting, validating & joining CSV files.<br>Commands are simple, fast & composable.<br><br>* [Commands](#available-commands)<br>* [Installation Options](#installation-options)<br> * [Whirlwind Tour](docs/whirlwind_tour.md#a-whirlwind-tour) / [Notebooks](contrib/notebooks/)<br>* [Cookbook](https://github.com/jqnatividad/qsv/wiki/Cookbook#cookbook)<br>* [FAQ](https://github.com/jqnatividad/qsv/discussions/categories/faq)<br>* [Performance Tuning](docs/PERFORMANCE.md#performance-tuning)<br>* 👉 [Benchmarks](https://qsv.dathere.com/benchmarks) 🚀<br>* [Environment Variables](docs/ENVIRONMENT_VARIABLES.md)<br>* [Feature Flags](#feature-flags)<br>* [Testing](#testing)<br>* [NYC School of Data 2022 slides](https://docs.google.com/presentation/d/e/2PACX-1vQ12ndZL--gkz0HLQRaxqsNOwzddkv1iUKB3sq661yA77OPlAsmHJHpjaqt9s9QEf73VqMfb0cv4jHU/pub?start=false&loop=false&delayms=3000)<br>* [Sponsor](#sponsor)
</div>
<div align="center">

## Try it out at [qsv.dathere.com](https://qsv.dathere.com)!

</div>

| <a name="available-commands">Command | Description |
| --- | --- |
| [apply](/src/cmd/apply.rs#L2)<br>✨🚀🧠🤖 | Apply series of string, date, math & currency transformations to given CSV column/s. It also has some basic [NLP](https://en.wikipedia.org/wiki/Natural_language_processing) functions ([similarity](https://crates.io/crates/strsim), [sentiment analysis](https://crates.io/crates/vader_sentiment), [profanity](https://docs.rs/censor/latest/censor/), [eudex](https://github.com/ticki/eudex#eudex-a-blazingly-fast-phonetic-reductionhashing-algorithm) & [language detection](https://crates.io/crates/whatlang)).  |
| <a name="applydp_deeplink"></a>[applydp](/src/cmd/applydp.rs#L2)<br>🚀 ![CKAN](docs/images/ckan.png)| applydp is a slimmed-down version of `apply` with only [Datapusher+](https://github.com/dathere/datapusher-plus) relevant subcommands/operations (`qsvdp` binary variant only). |
| [behead](/src/cmd/behead.rs#L2) | Drop headers from a CSV.  |
| [cat](/src/cmd/cat.rs#L2) | Concatenate CSV files by row or by column. |
| [count](/src/cmd/count.rs#L2)<br>📇🏎️ | Count the rows in a CSV file. (15.82 seconds for 15gb, 27m row NYC 311 dataset without an index. Instantaneous with an index.) |
| [dedup](/src/cmd/dedup.rs#L2)<br>🤯🚀 | Remove duplicate rows (See also `extdedup`, `extsort`, `sort` & `sortcheck` commands). |
| [describegpt](/src/cmd/describegpt.rs#L2)<br>🌐🤖 | Infer extended metadata about a CSV using a GPT model from [OpenAI's API](https://platform.openai.com/docs/introduction). |
| [diff](/src/cmd/diff.rs#L2)<br>🚀 | Find the difference between two CSVs with ludicrous speed!<br/>e.g. *compare two CSVs with 1M rows x 9 columns in under 600ms!* |
| [enum](/src/cmd/enumerate.rs#L2) | Add a new column enumerating rows by adding a column of incremental or uuid identifiers. Can also be used to copy a column or fill a new column with a constant value.  |
| [excel](/src/cmd/excel.rs#L2)<br>🚀 | Exports a specified Excel/ODS sheet to a CSV file. |
| [exclude](/src/cmd/exclude.rs#L2)<br>📇 | Removes a set of CSV data from another set based on the specified columns.  |
| [explode](/src/cmd/explode.rs#L2) | Explode rows into multiple ones by splitting a column value based on the given separator.  |
| [extdedup](/src/cmd/extdedup.rs#L2)<br> | Remove duplicate rows from an arbitrarily large CSV/text file using a memory-mapped, [on-disk hash table](https://crates.io/crates/odht). Unlike the `dedup` command, this command does not load the entire file into memory nor does it sort the deduped file. |
| [extsort](/src/cmd/extsort.rs#L2)<br>🚀 | Sort an arbitrarily large CSV/text file using a multithreaded [external merge sort](https://en.wikipedia.org/wiki/External_sorting) algorithm. |
| [fetch](/src/cmd/fetch.rs#L3)<br>✨🧠🌐 | Fetches data from web services for every row using **HTTP Get**. Comes with [HTTP/2](https://http2-explained.haxx.se/en/part1) [adaptive flow control](https://medium.com/coderscorner/http-2-flow-control-77e54f7fd518), [jql](https://github.com/yamafaktory/jql#%EF%B8%8F-usage) JSON query language support, dynamic throttling ([RateLimit](https://www.ietf.org/archive/id/draft-ietf-httpapi-ratelimit-headers-06.html)) & caching with optional [Redis](https://redis.io/) support for persistent caching. |
| [fetchpost](/src/cmd/fetchpost.rs#L3)<br>✨🧠🌐 | Similar to `fetch`, but uses **HTTP Post**. ([HTTP GET vs POST methods](https://www.geeksforgeeks.org/difference-between-http-get-and-post-methods/)) |
| [fill](/src/cmd/fill.rs#L2) | Fill empty values.  |
| [fixlengths](/src/cmd/fixlengths.rs#L2) | Force a CSV to have same-length records by either padding or truncating them. |
| [flatten](/src/cmd/flatten.rs#L2) | A flattened view of CSV records. Useful for viewing one record at a time.<br />e.g. `qsv slice -i 5 data.csv \| qsv flatten`. |
| [fmt](/src/cmd/fmt.rs#L2) | Reformat a CSV with different delimiters, record terminators or quoting rules. (Supports ASCII delimited data.)  |
| [foreach](/src/cmd/foreach.rs#L3)<br>✨ | Loop over a CSV to execute shell commands. (not available on Windows)  |
| [frequency](/src/cmd/frequency.rs#L2)<br>📇😣🏎️ | Build [frequency tables](https://statisticsbyjim.com/basics/frequency-table/) of each column. Uses multithreading to go faster if an index is present. |
| [geocode](/src/cmd/geocode.rs#L2)<br>✨🧠🌐🚀 | Geocodes a location against an updatable local copy of the [Geonames](https://www.geonames.org/) cities database. With caching and multi-threading, it geocodes up to 360,000 records/sec! |
| [generate](/src/cmd/generate.rs#L2)<br>✨ | Generate test data by profiling a CSV using [Markov decision process](https://crates.io/crates/test-data-generation) machine learning.  |
| [headers](/src/cmd/headers.rs#L2) | Show the headers of a CSV. Or show the intersection of all headers between many CSV files. |
| [index](/src/cmd/index.rs#L2) | Create an index (📇) for a CSV. This is very quick (even the 15gb, 28m row NYC 311 dataset takes all of 15 seconds to index) & provides constant time indexing/random access into the CSV. With an index, `count`, `sample` & `slice` work instantaneously; random access mode is enabled in `luau`; and multithreading (🏎️) is enabled for the `frequency`, `split`, `stats`, `schema` & `tojsonl` commands. |
| [input](/src/cmd/input.rs#L2) | Read CSV data with special quoting, trimming, line-skipping & non-UTF8 encoding handling rules. Typically used to "normalize" a CSV for further processing with other qsv commands. |
| [join](/src/cmd/join.rs#L2) | Inner, outer, right, cross, anti & semi joins. Automatically creates a simple, in-memory hash index to make it fast.  |
| [joinp](/src/cmd/joinp.rs#L2)<br>✨🚀🐻‍❄️ | Inner, outer, cross, anti, semi & asof joins using the [Pola.rs](https://www.pola.rs) engine. Unlike the `join` command, `joinp` can process files larger than RAM, is multi-threaded, has join key validation, pre-join filtering, supports [asof joins](https://pola-rs.github.io/polars/py-polars/html/reference/dataframe/api/polars.DataFrame.join_asof.html) & its output doesn't have duplicate columns. However, `joinp` doesn't have an --ignore-case option & it doesn't support right outer joins. |
| [jsonl](/src/cmd/jsonl.rs#L2) | Convert newline-delimited JSON ([JSONL](https://jsonlines.org/)/[NDJSON](http://ndjson.org/)) to CSV. See `tojsonl` command to convert CSV to JSONL.
| <a name="luau_deeplink"></a><br>[luau](/src/cmd/luau.rs#L2) 👑<br>✨📇🌐 ![CKAN](docs/images/ckan.png) | Create multiple new computed columns, filter rows, compute aggregations and build complex data pipelines by executing a [Luau](https://luau-lang.org) [0.598](https://github.com/Roblox/luau/releases/tag/0.598) expression/script for every row of a CSV file ([sequential mode](https://github.com/jqnatividad/qsv/blob/bb72c4ef369d192d85d8b7cc6e972c1b7df77635/tests/test_luau.rs#L254-L298)), or using [random access](https://www.webopedia.com/definitions/random-access/) with an index ([random access mode](https://github.com/jqnatividad/qsv/blob/bb72c4ef369d192d85d8b7cc6e972c1b7df77635/tests/test_luau.rs#L367-L415)).<br>Can process a single Luau expression or [full-fledged data-wrangling scripts using lookup tables](https://github.com/dathere/qsv-lookup-tables#example) with discrete BEGIN, MAIN and END sections.<br> It is not just another qsv command, it is qsv's [Domain-specific Language](https://en.wikipedia.org/wiki/Domain-specific_language) (DSL) with [numerous qsv-specific helper functions](https://github.com/jqnatividad/qsv/blob/113eee17b97882dc368b2e65fec52b86df09f78b/src/cmd/luau.rs#L1356-L2290) to build production data pipelines. |
| [partition](/src/cmd/partition.rs#L2) | Partition a CSV based on a column value. |
| [pseudo](/src/cmd/pseudo.rs#L2) | [Pseudonymise](https://en.wikipedia.org/wiki/Pseudonymization) the value of the given column by replacing them with an incremental identifier.  |
| [py](/src/cmd/python.rs#L2)<br>✨ | Create a new computed column or filter rows by evaluating a python expression on every row of a CSV file. Python's [f-strings](https://www.freecodecamp.org/news/python-f-strings-tutorial-how-to-use-f-strings-for-string-formatting/) is particularly useful for extended formatting, [with the ability to evaluate Python expressions as well](https://github.com/jqnatividad/qsv/blob/4cd00dca88addf0d287247fa27d40563b6d46985/src/cmd/python.rs#L23-L31). |
| [rename](/src/cmd/rename.rs#L2) |  Rename the columns of a CSV efficiently. |
| [replace](/src/cmd/replace.rs#L2) | Replace CSV data using a regex. Applies the regex to each field individually. |
| [reverse](/src/cmd/reverse.rs#L2)<br>🤯 | Reverse order of rows in a CSV. Unlike the `sort --reverse` command, it preserves the order of rows with the same key.  |
| <a name="safenames_deeplink"></a>[safenames](/src/cmd/safenames.rs#L2)<br>![CKAN](docs/images/ckan.png) | Modify headers of a CSV to only have ["safe" names](/src/cmd/safenames.rs#L5-L14) - guaranteed "database-ready"/"CKAN-ready" names.  |
| [sample](/src/cmd/sample.rs#L2)<br>📇🌐🏎️ | Randomly draw rows (with optional seed) from a CSV using [reservoir sampling](https://en.wikipedia.org/wiki/Reservoir_sampling), using memory proportional to the sample size. If an index is present, using random indexing with constant memory. |
| [schema](/src/cmd/schema.rs#L2)<br>📇😣🏎️ | Infer schema from CSV data, replete with data type & domain/range validation & output in [JSON Schema](https://json-schema.org/) format. Uses multithreading to go faster if an index is present. See `validate` command to use the generated JSON Schema to validate if similar CSVs comply with the schema. |
| [search](/src/cmd/search.rs#L2) | Run a regex over a CSV. Applies the regex to each field individually & shows only matching rows.  |
| [searchset](/src/cmd/searchset.rs#L2) | *Run multiple regexes over a CSV in a single pass.* Applies the regexes to each field individually & shows only matching rows.  |
| [select](/src/cmd/select.rs#L2) | Select, re-order, duplicate or drop columns.  |
| [slice](/src/cmd/slice.rs#L2)<br>📇🏎️ | Slice rows from any part of a CSV. When an index is present, this only has to parse the rows in the slice (instead of all rows leading up to the start of the slice).  |
| <a name="snappy_deeplink"></a>[snappy](/src/cmd/snappy.rs#L2)<br>🚀🌐 | Does streaming compression/decompression of the input using Google's [Snappy](https://github.com/google/snappy/blob/main/docs/README.md) framing format ([more info](#snappy-compressiondecompression)). |
| [sniff](/src/cmd/sniff.rs#L2)<br>🌐 ![CKAN](docs/images/ckan.png) | Quickly sniff & infer CSV metadata (delimiter, header row, preamble rows, quote character, flexible, is_utf8, average record length, number of records, content length & estimated number of records if sniffing a CSV on a URL, number of fields, field names & data types). It is also a general mime type detector. |
| [sort](/src/cmd/sort.rs#L2)<br>🚀🤯 | Sorts CSV data in alphabetical (with case-insensitive option), numerical, reverse, unique or random (with optional seed) order (See also `extsort` & `sortcheck` commands).  |
| [sortcheck](/src/cmd/sortcheck.rs#L2)<br>📇 | Check if a CSV is sorted. With the --json options, also retrieve record count, sort breaks & duplicate count. |
| [split](/src/cmd/split.rs#L2)<br>📇🏎️ | Split one CSV file into many CSV files of N chunks. Uses multithreading to go faster if an index is present. |
| [sqlp](/src/cmd/sqlp.rs#L2)<br>✨🚀🐻‍❄️ | Run blazing-fast Polars SQL queries against several CSVs - converting queries to [fast LazyFrame expressions](https://pola-rs.github.io/polars-book/user-guide/sql/intro/), processing larger than memory CSV files. |
| [stats](/src/cmd/stats.rs#L2)<br>📇🤯🏎️ | Compute [summary statistics](https://en.wikipedia.org/wiki/Summary_statistics) (sum, min/max/range, min/max length, mean, stddev, variance, nullcount, sparsity, quartiles, IQR, lower/upper fences, skewness, median, mode/s, antimode/s & cardinality) & make GUARANTEED data type inferences (Null, String, Float, Integer, Date, DateTime, Boolean) for each column in a CSV.<br>Uses multithreading to go faster if an index is present (with an index, can compile "streaming" stats on NYC's 311 data (15gb, 28m rows) in less than 20 seconds). |
| [table](/src/cmd/table.rs#L2)<br>🤯 | Show aligned output of a CSV using [elastic tabstops](https://github.com/BurntSushi/tabwriter).  To interactively view CSV files, qsv pairs well with [csvlens](https://github.com/YS-L/csvlens#csvlens). |
| [to](/src/cmd/to.rs#L2)<br>✨🚀 | Convert CSV files to [PostgreSQL](https://www.postgresql.org), [SQLite](https://www.sqlite.org/index.html), XLSX, [Parquet](https://parquet.apache.org) and [Data Package](https://datahub.io/docs/data-packages/tabular). |
| [tojsonl](/src/cmd/tojsonl.rs#L3)<br>📇😣🚀 | Smartly converts CSV to a newline-delimited JSON ([JSONL](https://jsonlines.org/)/[NDJSON](http://ndjson.org/)). By scanning the CSV first, it "smartly" infers the appropriate JSON data type for each column. See `jsonl` command to convert JSONL to CSV. |
| [transpose](/src/cmd/transpose.rs#L2)<br>🤯 | Transpose rows/columns of a CSV.  |
| [validate](/src/cmd/validate.rs#L2)<br>📇🚀🌐 | Validate CSV data blazingly-fast using [JSON Schema Validation](https://json-schema.org/draft/2020-12/json-schema-validation.html) & put invalid records into a separate file with an accompanying detailed validation error report file (e.g. *up to 350,000 rows/second* using [NYC's 311 schema](https://github.com/jqnatividad/qsv/blob/master/resources/test/311_Service_Requests_from_2010_to_Present-2022-03-04.csv.schema.json) generated by the `schema` command).<br>If no JSON schema file is provided, validates if a CSV conforms to the [RFC 4180 standard](#rfc-4180-csv-standard) and is UTF-8 encoded. |

<div style="text-align: right"><sub><sup>Performance metrics compiled on an M2 Pro 12-core Mac Mini with 32gb RAM</sup></sub></div>

✨: enabled by a [feature flag](#feature-flags).   
📇: uses an index when available.   
🤯: loads entire CSV into memory, though `dedup`, `stats` & `transpose` have "streaming" modes as well.   
😣: uses additional memory proportional to the cardinality of the columns in the CSV.   
🧠: expensive operations are memoized (cached) with available inter-session Redis caching for fetch commands.    
🐻‍❄️: command powered by [Pola.rs](https://pola.rs) engine.   
🤖: command uses Natural Language Processing & General AI techniques.  
🏎️: multithreaded and/or faster when an index (📇) is available.   
🚀: multithreaded even without an index.   
![CKAN](docs/images/ckan.png) : has [CKAN](https://ckan.org)-aware integration options.    
🌐: has web-aware options.

## Installation Options

### Option 1: Download Prebuilt Binaries

Full-featured prebuilt [binary variants](#variants) of the latest qsv version for Linux, macOS & Windows are available [for download](https://github.com/jqnatividad/qsv/releases/latest), including binaries compiled with [Rust Nightly](https://stackoverflow.com/questions/70745970/rust-nightly-vs-beta-version) ([more info](https://github.com/jqnatividad/qsv/blob/master/docs/PERFORMANCE.md#nightly-release-builds)).

These prebuilt binaries are also built with CPU optimizations enabled for x86_64 (e.g. [SSE4.2](https://en.wikipedia.org/wiki/SSE4#SSE4.2), [AVX2](https://en.wikipedia.org/wiki/Advanced_Vector_Extensions#Advanced_Vector_Extensions_2), [AVX512](https://en.wikipedia.org/wiki/Advanced_Vector_Extensions#Advanced_Vector_Extensions_512), etc. on Intel and AMD processors) and Apple Silicon processors ([ARM64 SIMD NEON](https://eclecticlight.co/2021/08/06/accelerating-the-m1-mac-an-introduction-to-simd/)) for even more performance gains.

For Windows, an MSI Installer wrapping the x86_64-pc-windows-msvc build is also available for download.

For macOS, ["ad-hoc" signatures](https://users.rust-lang.org/t/distributing-cli-apps-on-macos/70223) are used to sign our binaries, so you will need to [set appropriate Gatekeeper security settings](https://support.apple.com/en-us/HT202491) or run the following command to remove the quarantine attribute from qsv before you run it for the first time:

```bash
# replace qsv with qsvlite or qsvdp if you installed those binary variants
xattr -d com.apple.quarantine qsv
```

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
cargo install qsv --locked --bin qsv --features feature_capable,apply,generate,luau,fetch,foreach,python,to,self_update,polars
# or shorthand
cargo install qsv --locked --bin qsv -F all_features

# or enable only the apply and polars features
cargo install qsv --locked --bin qsv -F feature_capable,apply,polars

# or to install qsvlite
cargo install qsv --locked --bin qsvlite -F lite

# or to install qsvdp
cargo install qsv --locked --bin qsvdp -F datapusher_plus,luau
```

### Option 4: Compile from Source

Compiling from source also works similarly[^1]:

```bash
git clone https://github.com/jqnatividad/qsv.git
cd qsv
cargo build --release --locked --bin qsv --features all_features
```

The compiled binary will end up in `./target/release/`.

To compile different [variants](#variants) and enable optional [features](#feature-flags):

```bash
# to compile qsv with all features enabled
cargo build --release --locked --bin qsv --features feature_capable,apply,generate,luau,fetch,foreach,python,to,self_update,polars
# shorthand
cargo build --release --locked --bin qsv -F all_features

# or build qsv with only the fetch and foreach features enabled
cargo build --release --locked --bin qsv -F feature_capable,fetch,foreach

# for qsvlite
cargo build --release --locked --bin qsvlite -F lite

# for qsvdp
cargo build --release --locked --bin qsvdp -F datapusher_plus,luau
```

NOTE: To build with Rust nightly, see [Nightly Release Builds](docs/PERFORMANCE.md#nightly-release-builds).

### Variants

There are three binary variants of qsv:
* `qsv` - [feature](#feature-flags)-capable(✨), with the [prebuilt binaries](https://github.com/jqnatividad/qsv/releases/latest) enabling all applicable features except Python [^2]
* `qsvlite` - all features disabled (~13% of the size of `qsv`)
* `qsvdp` - optimized for use with [DataPusher+](https://github.com/dathere/datapusher-plus) with only DataPusher+ relevant commands; an embedded [`luau`](#luau_deeplink) interpreter; [`applydp`](#applydp_deeplink), a slimmed-down version of the `apply` feature; the `--progressbar` option disabled; and the self-update only checking for new releases, requiring an explicit `--update` (~12% of the the size of `qsv`).

[^2]: The `foreach` feature is not available on Windows. The `python` feature is not enabled on the prebuilt binaries. Compile qsv with Python development environment installed if you want to enable the `python` feature (Python 3.7 & above supported). The `luau` feature is enabled by default on the prebuilt binaries if the platform supports it.  

## Regular Expression Syntax

The `--select` option and several commands (`apply`, `applydp`, `replace`, `schema`, `search`, `searchset`, `select` & `sqlp`) allow the user to specify regular expressions. We use the [`regex`](https://docs.rs/regex) crate to parse, compile and execute these expressions. [^3]

[^3]: This is the same regex engine used by [`ripgrep`](https://github.com/BurntSushi/ripgrep#ripgrep-rg) - the [blazingly fast grep replacement](https://blog.burntsushi.net/ripgrep/) that powers Visual Studio's [magical](https://lab.cccb.org/en/arthur-c-clarke-any-sufficiently-advanced-technology-is-indistinguishable-from-magic/) ["Find in Files"](https://github.com/microsoft/vscode-ripgrep) feature.

Its syntax can be found [here](https://docs.rs/regex/latest/regex/#syntax) and *"is similar to other regex engines, but it lacks several features that are not known how to implement efficiently. This includes, but is not limited to, look-around and backreferences. In exchange, all regex searches in this crate have worst case O(m * n) time complexity, where m is proportional to the size of the regex and n is proportional to the size of the string being searched."*

If you want to test your regular expressions, [regex101](https://regex101.com) supports the syntax used by the `regex` crate. Just select the "Rust" flavor.

## File formats

qsv recognizes UTF-8/ASCII encoded, CSV (`.csv`) & TSV files (`.tsv` & `.tab`). CSV files are assumed to have "," (comma) as a delimiter,
and TSV files, "\t" (tab) as a delimiter. The delimiter is a single ascii character that can be set either by the `--delimiter` command-line option or
with the `QSV_DEFAULT_DELIMITER` environment variable or automatically detected when `QSV_SNIFF_DELIMITER` is set.

When using the `--output` option, qsv will UTF-8 encode the file & automatically change the delimiter used in the generated file based on the file extension - i.e. comma for `.csv`, tab for `.tsv` & `.tab` files.

[JSONL](https://jsonlines.org/)/[NDJSON](http://ndjson.org/) files are also recognized & converted to/from CSV with the [`jsonl`](/src/cmd/jsonl.rs#L11) and [`tojsonl`](/src/cmd/tojsonl.rs#L12) commands respectively.

The `fetch` & `fetchpost` commands also produces JSONL files when its invoked without the `--new-column` option & TSV files with the `--report` option.

The `excel`, `safenames`, `sniff`, `sortcheck` & `validate` commands produce JSON files with their JSON options following the [JSON API 1.1 specification](https://jsonapi.org/format/), so it can return detailed machine-friendly metadata that can be used by other systems.

The `schema` command produces a [JSON Schema Validation (Draft 7)](https://json-schema.org/draft/2020-12/json-schema-validation.html) file with the ".schema.json" file extension, which can be used with the `validate` command to validate other CSV files with a similar schema.

The `excel` command recognizes Excel & Open Document Spreadsheet(ODS) files (`.xls`, `.xlsx`, `.xlsm`, `.xlsb` & `.ods` files).

The `to` command converts CSVs to `.xlsx`, [Parquet](https://parquet.apache.org) & [Data Package](https://datahub.io/docs/data-packages/tabular) files, and populates [PostgreSQL](https://www.postgresql.org) and [SQLite](https://www.sqlite.org/index.html) databases.

The `sqlp` command returns query results in CSV, JSON, Parquet & [Arrow IPC](https://arrow.apache.org/docs/format/Columnar.html#ipc-file-format) formats. Polars SQL also supports reading external files directly in various formats with its `read_ndjson`, `read_csv`, `read_parquet` & `read_ipc` [table functions](https://github.com/pola-rs/polars/blob/c7fa66a1340418789ec66bdedad6654281afa0ab/polars/polars-sql/src/table_functions.rs#L9-L36).

### Snappy Compression/Decompression

qsv supports *automatic compression/decompression* using the [Snappy frame format](https://github.com/google/snappy/blob/main/framing_format.txt). Snappy was chosen instead of more popular compression formats like gzip because it was designed for [high-performance streaming compression & decompression](https://github.com/google/snappy/tree/main/docs#readme) (up to 2.58 gb/sec compression, 0.89 gb/sec decompression).

For all commands except the `index`, `extdedup` & `extsort` commands, if the input file has an ".sz" extension, qsv will *automatically* do streaming decompression as it reads it. Further, if the input file has an extended CSV/TSV ".sz" extension (e.g nyc311.csv.sz/nyc311.tsv.sz/nyc311.tab.sz), qsv will also use the file extension to determine the delimiter to use.   

Similarly, if the `--output` file has an ".sz" extension, qsv will *automatically* do streaming compression as it writes it.
If the output file has an extended CSV/TSV ".sz" extension, qsv will also use the file extension to determine the delimiter to use.  

Note however that compressed files cannot be indexed, so index-accelerated commands (`frequency`, `schema`, `split`, `stats`, `tojsonl`) will not be multi-threaded. Random access is also disabled without an index, so `slice` will not be instantaneous and `luau`'s random-access mode will not be available.

There is also a dedicated [`snappy`](/src/cmd/snappy.rs#L2) command with four subcommands for direct snappy file operations — a multithreaded `compress` subcommand (4-5x faster than the built-in, single-threaded auto-compression); a `decompress` subcommand with detailed compression metadata; a `check` subcommand to quickly inspect if a file has a Snappy header; and a `validate` subcommand to confirm if a Snappy file is valid.

The `snappy` command can be used to compress/decompress ANY file, not just CSV/TSV files.

Using the `snappy` command, we can compress NYC's 311 data (15gb, 28m rows) to 4.95 gb in *5.77 seconds* with the multithreaded `compress` subcommand - *2.58 gb/sec* with a 0.33 (3.01:1) compression ratio.  With `snappy decompress`, we can roundtrip decompress the same file in *16.71 seconds* - *0.89 gb/sec*.

Compare that to [zip 3.0](https://infozip.sourceforge.net/Zip.html), which compressed the same file to 2.9 gb in *248.3 seconds on the same machine - 43x slower at 0.06 gb/sec* with a 0.19 (5.17:1) compression ratio - for just an additional 14% (2.45 gb) of saved space. zip also took 4.3x longer to roundtrip decompress the same file in *72 seconds* - *0.20 gb/sec*.

## RFC 4180 CSV Standard

qsv follows the [RFC 4180](https://datatracker.ietf.org/doc/html/rfc4180) CSV standard. However, in real life, CSV formats vary significantly & qsv is actually not strictly compliant with the specification so it can process "real-world" CSV files.
qsv leverages the awesome [Rust CSV](https://docs.rs/csv/latest/csv/) crate to read/write CSV files.

Click [here](https://docs.rs/csv-core/latest/csv_core/struct.Reader.html#rfc-4180) to find out more about how qsv conforms to the standard using this crate.

When dealing with "atypical" CSV files, you can use the `input` command to normalize them to be RFC 4180-compliant.

## UTF-8 Encoding

qsv requires UTF-8 encoded input (of which ASCII is a subset).

Should you need to re-encode CSV/TSV files, you can use the `input` command to "lossy save" to UTF-8 - replacing invalid UTF-8 sequences with `�` ([U+FFFD REPLACEMENT CHARACTER](https://doc.rust-lang.org/std/char/constant.REPLACEMENT_CHARACTER.html)).

Alternatively, if you want to truly transcode to UTF-8, there are several utilities like [`iconv`](https://en.wikipedia.org/wiki/Iconv) that you can use to do so on [Linux/macOS](https://stackoverflow.com/questions/805418/how-can-i-find-encoding-of-a-file-via-a-script-on-linux) & [Windows](https://superuser.com/questions/1163753/converting-text-file-to-utf-8-on-windows-command-prompt).

### Windows Usage Note

Unlike other modern operating systems, Microsoft Windows' [default encoding is UTF16-LE](https://stackoverflow.com/questions/66072117/why-does-windows-use-utf-16le). This will cause problems when redirecting qsv's output to a CSV file & trying to open it with Excel (which ignores the comma delimiter, with everything in the first column if the file is UTF16-LE encoded):

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

As date manipulation is often needed, the [LuaDate](https://tieske.github.io/date/) module is also bundled.

Finally, as [qsv's DSL](#luau_deeplink) (👑), `luau` will gain even more features over time compared to the `python` feature.

[Luau 0.598](https://github.com/Roblox/luau/releases/tag/0.598) is currently embedded - qsv's policy is to use the latest stable Luau version at the time of each qsv release.

### Python

The `python` feature is NOT enabled by default on the prebuilt binaries as its dynamically linked to python libraries at runtime, which presents distribution issues, as various operating systems have differing Python versions.

If you wish to enable the `python` feature - you'll just have to install/compile from source, making sure you have the development libraries for the desired Python version (Python 3.7 and above are supported) installed when doing so (e.g. on Debian/Ubuntu - `apt-get install python-dev`; on CentOS/RedHat/Amazon Linux - `yum install python-devel`; on Windows and macOS - use the [Python installer](https://www.python.org/downloads/) for the desired version).

If you plan to distribute your manually built `qsv` with the `python` feature, `qsv` will look for the specific version of Python shared libraries (libpython* on Linux/macOS, python*.dll on Windows) against which it was compiled starting with the current directory & abort with an error if not found, detailing the Python library it was looking for. 

Note that this will happen on qsv startup, even if you're NOT running the `py` command.

When building from source - [PyO3](https://pyo3.rs) - the underlying crate that enables the `python` feature, uses a build script to determine the Python version & set the correct linker arguments. By default it uses the python3 executable.
You can override this by setting `PYO3_PYTHON` (e.g., `PYO3_PYTHON=python3.7`), before installing/compiling qsv. See the [PyO3 User Guide](https://pyo3.rs/v0.17.1/building_and_distribution.html) for more information.

Consider using the [`luau`](/src/cmd/luau.rs#L2) command instead of the [`py`]((/src/cmd/python.rs#L2)) command if the operation you're trying to do can be done with `luau` - as `luau` is statically linked, has no external dependencies, much faster than `py`, can do aggregations, supports random access, has a bevy of qsv helper functions, and allows mapping of multiple new columns. 

The `py` command cannot do aggregations because [PyO3's GIL-bound memory](https://pyo3.rs/v0.17.2/memory.html#gil-bound-memory) limitations will quickly consume a lot of memory (see [issue 449](https://github.com/jqnatividad/qsv/issues/449#issuecomment-1226095316) for details).
To prevent this, the `py` command processes CSVs in batches (default: 30,000 records), with a GIL pool for each batch, so no globals are available across batches.

## Memory Management
qsv supports three memory allocators - mimalloc (default), jemalloc and the standard allocator.<br>See [Memory Allocator](docs/PERFORMANCE.md#memory-allocator) for more info.

It also has Out-of-Memory prevention, with two modes - NORMAL (default) & CONSERVATIVE.<br>See [Out-of-Memory Prevention](docs/PERFORMANCE.md#out-of-memory-oom-prevention) for more info.

## Environment Variables & dotenv file support

qsv supports an extensive list of environment variables and supports `.env` files to set them.

For details, see [Environment Variables](docs/ENVIRONMENT_VARIABLES.md) and the [`dotenv.template.yaml`](dotenv.template.yaml) file.
## Feature Flags

`qsv` has several features:

* `mimalloc` (default) - use the mimalloc allocator (see [Memory Allocator](docs/PERFORMANCE.md#memory-allocator) for more info).
* `jemallocator` - use the jemalloc allocator (see [Memory Allocator](docs/PERFORMANCE.md#memory-allocator) for more info).
* `apply` - enable `apply` command. This swiss-army knife of CSV transformations is very powerful, but it has a lot of dependencies that increases both compile time and binary size.
* `fetch` - enables the `fetch` & `fetchpost` commands.
* `foreach` - enable `foreach` command (not valid for Windows).
* `geocode` - enable `geocode` command.
* `generate` - enable `generate` command.
* `luau` - enable `luau` command. Embeds a [Luau](https://luau-lang.org) interpreter into qsv. [Luau has type-checking, sandboxing, additional language operators, increased performance & other improvements](https://luau-lang.org/2022/11/04/luau-origins-and-evolution.html) over Lua.
* `polars` - enables all [Polars](https://pola.rs)-powered commands (currently, `joinp` and `sqlp`). Note that Polars is a very powerful library, but it has a lot of dependencies that drastically increases both compile time and binary size.
* `python` - enable `py` command. Note that qsv will look for the shared library for the Python version (Python 3.7 & above supported) it was compiled against & will abort on startup if the library is not found, even if you're NOT using the `py` command. Check [Python](#python) section for more info.
* `to` - enables the `to` command except the parquet option.
* `to_parquet` - enables the `parquet` option of the `to` command. This is a separate feature as it brings in the `duckdb` dependency, which markedly increases binary size and compile time.
Use the `sqlp` command with the `--format parquet` option instead if you don't need the `to` command's other options and you don't need to convert to parquet a directory of CSVs.
* `self_update` - enable self-update engine, checking GitHub for the latest release. Note that if you manually built qsv, `self-update` will only check for new releases.
It will NOT offer the choice to update itself to the prebuilt binaries published on GitHub. You need not worry that your manually built qsv will be overwritten by a self-update.

* `feature_capable` - enable to build `qsv` binary variant which is feature-capable.
* `all_features` - enable to build `qsv` binary variant with all features enabled (apply,fetch,foreach,generate,luau,polars,python,to,to_parquet,self_update).
* `lite` - enable to build `qsvlite` binary variant with all features disabled.
* `datapusher_plus` - enable to build `qsvdp` binary variant - the [DataPusher+](https://github.com/dathere/datapusher-plus) optimized qsv binary.
* `nightly` - enable to turn on nightly/unstable features in the `rand`, `regex`, `hashbrown` & `pyo3` crates when building with Rust nightly/unstable.

> ℹ️ **NOTE:** `qsvlite`, as the name implies, always has **non-default features disabled**. `qsv` can be built with any combination of the above features using the cargo `--features` & `--no-default-features` flags. The prebuilt `qsv` binaries has **all applicable features valid for the target platform**[^2].

## Minimum Supported Rust Version

qsv's MSRV policy is to require the latest [Rust version](https://github.com/rust-lang/rust/blob/master/RELEASES.md) that is [supported by Homebrew](https://formulae.brew.sh/formula/rust#default), currently [![HomeBrew](https://img.shields.io/homebrew/v/rust?logo=homebrew)](https://formulae.brew.sh/formula/rust).

## Testing
qsv has ~1,220 tests in the [tests](https://github.com/jqnatividad/qsv/tree/master/tests) directory.
Each command has its own test suite in a separate file with the convention `test_<COMMAND>.rs`.
Apart from preventing regressions, the tests also serve as good illustrative examples, and are often linked from the usage text of each corresponding command.

To test each binary variant:

```bash
# to test qsv
cargo test --features all_features

# to test qsvlite
cargo test --features lite
# to test all tests with "stats" in the name with qsvlite
cargo test stats --features lite

# to test qsvdp
cargo test --features datapusher_plus,luau

# to test a specific command
# here we test only stats and use the
# t alias for test and the -F shortcut for --features
cargo t stats -F all_features

# to test a specific command with a specific feature
# here we test only luau comand with the luau feature
cargo t luau -F feature_capable,luau

# to test the count command with multiple features
cargo t count -F feature_capable,luau,polars

# to test using an alternate allocator
# other than the default mimalloc allocator
cargo t --no-default-features -F all_features,jemallocator
```

## License

Dual-licensed under MIT or the [UNLICENSE](https://unlicense.org).

## Origins

Quicksilver (qsv) is a fork of the popular [xsv](https://github.com/BurntSushi/xsv) utility, merging several pending PRs [since xsv 0.13.0's May 2018 release](https://github.com/BurntSushi/xsv/issues/267). On top of xsv's 20 commands, it adds numerous new features; 37 additional commands; 5 `apply` subcommands & 35 operations; 5 `to` subcommands; 3 `cat` subcommands; 5 `geocode` subcommands & 4 operations; and 4 `snappy` subcommands.
See [FAQ](https://github.com/jqnatividad/qsv/discussions/categories/faq) for more details.

## Sponsor

<div align="center">

|qsv was made possible by|
:-------------------------:|
|[![datHere Logo](docs/images/datHere-logo-withtagline.png)](https://datHere.com)<br>|
|Standards-based, best-of-breed, open source solutions<br>to make your **Data Useful, Usable & Used.**   |

</div>

## Naming Collision

This project is unrelated to [Intel's Quick Sync Video](https://www.intel.com/content/www/us/en/architecture-and-technology/quick-sync-video/quick-sync-video-general.html).
