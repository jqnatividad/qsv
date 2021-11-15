# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
## [0.22.0] - 2021-11-15
### Added
- `generate` command. Generate test data by profiling a CSV using a [Markov decision process](https://docs.rs/test-data-generation).
- add `--no-headers` option to `rename` command (see [discussion #81](https://github.com/jqnatividad/qsv/discussions/81#discussioncomment-1599027))
- Auto-publish binaries for more platforms on release
- added combo-test for sort-dedup-sort (see [discussion #80](https://github.com/jqnatividad/qsv/discussions/80#discussioncomment-1610190))
- New environment variables galore
  - `QSV_DEFAULT_DELIMITER` - single ascii character to use as delimiter.  Overrides `--delimeter` option. Defaults to "," (comma) for CSV files and "\t" (tab) for TSV files, when not set. Note that this will also set the delimiter for qsv's output. Adapted from [xsv PR](https://github.com/BurntSushi/xsv/pull/94) by [@camerondavison](https://github.com/camerondavison).
  - `QSV_NO_HEADERS` - when set, the first row will **NOT** be interpreted as headers. Supersedes `QSV_TOGGLE_HEADERS`.
  - `QSV_MAX_JOBS` - number of jobs to use for parallelized commands (currently `frequency`, `split` and `stats`). If not set, max_jobs is set
to number of logical processors divided by four.  See [Parallelization](#parallelization) for more info.
  - `QSV_REGEX_UNICODE` - if set, makes `search`, `searchset` and `replace` commands unicode-aware. For increased performance, these
commands are not unicode-aware and will ignore unicode values when matching and will panic when unicode characters are used in the regex.
- Added parallelization heuristic (num_cpus/4), in connection with `QSV_MAX_JOBS`.
- Added more tests
  - `apply` (test for regex_replace, eudex, and lat/long parsing)
  - combo-test (see above) - for testing qsv command combinations
  - tests for `QSV_NO_HEADERS` environment variable
  - tests for `QSV_REGEX_UNICODE` environment variable in `search`, `searchset` and `replace` commands
  - tests for `QSV_DEFAULT_DELIMITER` environment variable
### Changed
- MSRV of Rust 1.56
- expanded `apply` help-text examples
- progress bar now only updates every 1% progress by default
- replaced English-specific soundex with multi-lingual eudex algorithm (see https://docs.rs/crate/eudex/0.1.1)
- refactored `apply geocode` subcommand to improve cache performance
- improved lat/long parsing - can now recognize embedded coordinates in text
- changed `apply operations regex_replace` behavior to do all matches in a field, instead of just the left-most one, to be consistent with the behavior of `apply operations replace`

## [0.21.0] - 2021-11-07
### Added
- added `apply geocode` caching, more than doubling performance in the geocode benchmark.
- added `--random` and `--seed` options to `sort` command from [@pjsier](https://github.com/pjsier).
- added qsv tab completion section to README.
- additional `apply operations` subcommands:
  * Match Trim operations - enables trimming of more than just whitespace, but also of multiple trim characters in one pass ([Example](https://github.com/jqnatividad/qsv/blob/9569dd7c2a897e0a47b97e1abfd1c3efab920990/tests/test_apply.rs#L214)):
    * mtrim: Trims `--comparand` matches left & right of the string ([trim_matches](https://doc.rust-lang.org/std/string/struct.String.html#method.trim_matches) wrapper)
    * mltrim: Left trim `--comparand` matches ([trim_start_matches](https://doc.rust-lang.org/std/string/struct.String.html#method.trim_start_matches) wrapper)
    * mrtrim: Right trim `--comparand` matches ([trim_end_matches](https://doc.rust-lang.org/std/string/struct.String.html#method.trim_end_matches) wrapper)
  * replace: Replace all matches of a pattern (using `--comparand`)
      with a string (using `--replacement`) (Std::String [replace](https://doc.rust-lang.org/std/string/struct.String.html#method.replace) wrapper).
  * regex_replace: Replace the leftmost-first regex match with `--replacement` (regex [replace](https://docs.rs/regex/1.1.0/regex/struct.Regex.html#method.replace) wrapper).
  * titlecase - capitalizes English text using Daring Fireball titlecase style
      https://daringfireball.net/2008/05/title_case
  * censor_check: check if profanity is detected (boolean) [Examples](https://github.com/jqnatividad/qsv/blob/9569dd7c2a897e0a47b97e1abfd1c3efab920990/tests/test_apply.rs#L66)
  * censor: profanity filter
- added parameter validation to `apply operations` subcommands
- added more robust parameter validation to `apply` command by leveraging docopt
- added more tests
- added `rust-version` in Cargo.toml to specify MSRV of rust 1.56

### Changed
- revamped benchmark script:
  * allow binary to be changed, so users can benchmark xsv and other xsv forks by simply replacing the $bin shell variable
  * now uses a much larger data file - a 1M row, 512 mb, 41 column sampling of NYC's 311 data
  * simplified and cleaned-up script now that it's just using 1 data file
- Upgrade rand and quickcheck crates to latest releases (0.8.4 and 1.0.3 respectively), and modified code accordingly.
- `cargo update` bumped addr2line (0.16.0->0.17.0), backtrace (0.3.62->0.3.63), gimli (0.25.0->0.26.1) and anyhow (1.0.44->1.0.45)

### Removed
- removed `scramble` command as its function is now subsumed by the `sort` command with the `--random` and `--seed` options
- removed `num-format` crate which has a large dependency tree with several old crates; replaced with much smaller `thousands` crate.
- removed 1M row, 48mb, 7 column world_cities_pop_mil.csv as its no longer used by the revamped benchmark script.
- removed `build.rs` build dependency that was checking for MSRV of Rust >= "1.50". Instead, took advantage of new [`rust-version`](https://blog.rust-lang.org/2021/10/21/Rust-1.56.0.html#cargo-rust-version) Cargo.toml option
introduced in Rust 1.56.

## [0.20.0] - 2021-10-31
### Added
- added string similarity operations to `apply` command:
  * simdl: Damerau-Levenshtein similarity
  * simdln: Normalized Damerau-Levenshtein similarity (between 0.0 & 1.0)
  * simjw: Jaro-Winkler similarity (between 0.0 & 1.0)
  * simsd: Sørensen-Dice similarity (between 0.0 & 1.0)
  * simhm: Hamming distance. Number of positions where characters differ.
  * simod: OSA Distance.
  * soundex: sounds like (boolean)
- added progress bars to commands that may spawn long-running jobs - for this release,
`apply`, `foreach`, and `lua`. Progress bars can be suppressed with `--quiet` option.
- added progress bar helper functions to utils.rs.
- added `apply` to benchmarks.
- added sample NYC 311 data to benchmarks.
- added records per second (RECS_PER_SEC) to benchmarks

### Changed
- major refactoring of `apply` command:
  - to take advantage of docopt parsing/validation.
  - instead of one big command, broke down apply to several subcommands:
    - operations
    - emptyreplace
    - datefmt
    - geocode
- simplified lat/long regex validator to no longer validate range, as the underlying 
geocoder function validates it already - 18% geocode speedup.
- bumped docopt back up to 1.1.1.
- improved error message when specifying an invalid apply operation.

## [0.19.0] - 2021-10-24
### Added
- new `scramble` command. Randomly scrambles a CSV's records.
- read/write buffer capacity can now be set using environment variables
`QSV_RDR_BUFFER_CAPACITY` and `QSV_WTR_BUFFER_CAPACITY` (in bytes).
- added additional test for `apply datefmt`.

### Changed
- default read buffer doubled from 8k to 16k.
- default write buffer doubled from 32k to 64k.
- benchmark script revamped. Now produces aligned output onscreen,
while also creating a benchmark TSV file; downloads the sample file from GitHub;
benchmark more commands.
- version info now also returns memory allocator being used, and number of cpus detected.
- minor refactor of `enumerate`, `explode`, `fill` and `foreach` commands.

### Removed
- removed benchmark data from repository. Moved to GitHub wiki instead.

## [0.18.2] - 2021-10-21
## Changed
- use docopt v1.1.0 instead of docopt v.1.1.1 for docopt to support all regex features

## [0.18.1] - 2021-10-20
### Added
- added `mimalloc` feature flag. [mimalloc](https://github.com/microsoft/mimalloc) is Microsoft's performance-oriented memory allocator.
Earlier versions of qsv used mimalloc by default. Now it is only used when the feature is set.
- README: Added Performance section.
- README: Document how to enable `mimalloc` feature.

### Changed
- README: Explicitly show how to set environment variables on different platforms.
## [0.18.0] - 2021-10-18
### Added
- `stats` `mode` is now also multi-modal -i.e. returns multiples modes when detected. 
e.g. mode[1,1,2,2,3,4,6,6] will return [1,2,6].
It will continue to return one mode if there is only one detected.
- `stats` `quartile` now also computes IQR, lower/upper fences and skew ([using Pearson's median skewness](https://en.wikipedia.org/wiki/Skewness#Pearson's_second_skewness_coefficient_(median_skewness))). For code simplicity, calculated skew with quartile.
- `join` now also support `left-semi` and `left-anti` joins, the same way [Spark does](https://spark.apache.org/docs/latest/sql-ref-syntax-qry-select-join.html#semi-join).
- `search` `--flag` option now returns row number, not just '1'.
- `searchset` `--flag` option now returns row number, followed by a semi-colon, and a list of matching regexes.
- README: Added badges for Security Audit, Discussion & Docs
- README: Added FAQ link in fork note.

### Changed
- point to https://docs.rs/crate/qsv for documentation.
- README: `stats` and `join` section updated with new features.
- README: wordsmithing - replaced "CSV data" and "CSV file/s" with just "CSV".
- in `stats` changed `q2` column name to `q2_median`.
- removed debug symbols in release build for smaller binaries.
- minor refactoring of `search`, `searchset` & `stats`.

### Fixed
- README: fixed `flatten` example.

### Removed
- removed Rust badge.

## [0.17.3] - 2021-10-12
### Added
- added [sample regexset file](https://github.com/jqnatividad/qsv/commit/d209436b588b88b0f92cc133ebcada726f72a2bd) for PII-screening.

### Changed
- `apply geocode --formatstr` now accepts less US-centric format selectors.
- `searchset --flag` now shows which regexes match as a list (e.g. "[1, 3, 5]"), not just "1" or "0".
### Fixed
- `foreach` command now returns error message on Windows. `foreach` still doesn't work on 
Windows (yet), but at least it returns "foreach command does not work on Windows.".
- `apply geocode` was not accepting valid lat/longs below the equator. Fixed regex validator.
- more robust `searchset` error handling when attempting to load regexset files.
- `apply` link on README was off by one. 
## [0.17.2] - 2021-10-10

### Changed
- bumped `dateparser` to 0.1.6. This now allows `apply datefmt` to properly reformat
dates without a time component. Before, when reformatting a date like "July 4, 2020", 
qsv returns "2020-07-04T00:00:00+00:00". It now returns "2020-07-04".
- minor clippy refactoring
### Removed
- removed rust-stats submodule introduced in 0.17.1. It turns out
crates.io does not allow publishing of crates with local dependencies on submodules. 
Published the modified rust-stats fork as qsv-stats instead. This allows us to publish
qsv on crates.io
- removed unused `textwrap` dependency
## [0.17.1] - 2021-10-10
### Fixed
- explicitly specified embedded modified rust-stats version in Cargo.toml. 
## [0.17.0] - 2021-10-10
### Added
- added `searchset` command. Run **multiple regexes** over CSV data in a **single pass**.
- added `--unicode` flag to `search`, `searchset` and `replace` commands.
Previously, regex unicode support was on by default, which comes at the cost of performance.
And since `qsv` optimizes for performance ("q is for quick"), it is now off by default.
- added quartiles calculation to `stats`. Pulled in upstream
[pending](https://github.com/BurntSushi/rust-stats/pull/15) [PRs](https://github.com/BurntSushi/xsv/pull/273) 
from [@m15a](https://github.com/m15a) to implement.

### Changed
- changed variance algorithm. For some reason, the previous variance algorithm was causing
intermittent test failures on macOS. Pulled in [pending upstream PR](https://github.com/BurntSushi/rust-stats/pull/11)
from [@ruppertmillard](https://github.com/ruppertmillard).
- embedded [rust-stats fork](https://github.com/jqnatividad/rust-stats) submodule which implements 
quartile and new variance algorithm.
- changed GitHub Actions to pull in submodules.

### Fixed
- the project was not following semver properly, as several new features were released 
in the 0.16.x series that should have been MINOR version bumps, not PATCH bumps.

## [0.16.4] - 2021-10-08
### Added
- added `geocode` operation to `apply` command. It geocodes to the closest city given a column   
with coordinates in Location format ('latitude, longitude') using a static geonames lookup file.   
(see https://docs.rs/reverse_geocoder)
- added `currencytonum` operation to `apply` command.
- added `getquarter.lua` helper script to support `lua` example in [Cookbook](https://github.com/jqnatividad/qsv/wiki#cookbook).
- added `turnaroundtime.lua` helper script to compute turnaround time.
- added `nyc311samp.csv` to provide sample data for recipes.
- added several Date Enrichment and Geocoding recipes to [Cookbook](https://github.com/jqnatividad/qsv/wiki#cookbook).

### Fixed
- fixed `publish.yml` Github Action workflow to properly create platform specific binaries.
- fixed variance test to eliminate false positives in macOS.

## [0.16.3] - 2021-10-06
### Added
- added `docs` directory. For README reorg, and to add detailed examples per command in the future.
- added `emptyreplace` operation to `apply` command.
- added `datefmt` operation to `apply` command.
- added support for reading from stdin to `join` command.
- setup GitHub wiki to host [Cookbook](https://github.com/jqnatividad/qsv/wiki#cookbook) and sundry docs to encourage collaborative editing.
- added footnotes to commands table in README.

### Changed
- changed GitHub Actions publish workflow so it adds the version to binary zip filename.
- changed GitHub Actions publish workflow so binary is no longer in `target/release` directory.
- reorganized README.
- moved whirlwind tour and benchmarks to `docs` directory.
- use zipped repo copy of worldcitiespop_mil.csv for benchmarks.

### Fixed
- fixed links to help text in README for `fixlengths` and `slice` cmds
- `exclude` not listed in commands table. Added to README.

### Removed
- Removed `empty0` and `emptyNA` operations in `apply` command.
Replaced with `emptyreplace`.

## [0.16.2] - 2021-09-30
### Changed
- changed Makefile to remove github recipe as we are now using GitHub Actions.
- Applied rustfmt to entire project [#56](https://github.com/jqnatividad/qsv/issues/56)
- Changed stats variance test as it was causing false positive test failures on macOS ([details](https://github.com/jqnatividad/qsv/commit/8c45c60de7598c7dc4cedd10ce7cb281ee34db46))
- removed `-amd64` suffix from binaries built by GitHub Actions.

### Fixed
- fixed publish Github Actions workflow to zip binaries before uploading.

### Removed 
- removed `.travis.yml` as we are now using GitHub Actions.
- removed scripts `build-release`, `github-release` and `github-upload` as we are now
 using GitHub Actions.
- removed `ci` folder as we are now using GitHub Actions.
- removed `py` command. [#58](https://github.com/jqnatividad/qsv/issues/58)

## [0.16.1] - 2021-09-28
### Fixed
- Bumped qsv version to 0.16.1. Inadvertently released 0.16.0 with qsv version still at 0.15.0.

## [0.16.0] - 2021-09-28
### Added
- Added a CHANGELOG.
- Added additional commands/options from [@Yomguithereal](https://github.com/Yomguithereal) 
[xsv fork](https://github.com/Yomguithereal/xsv).
  * `apply` - Apply series of string transformations to a CSV column.
  * `behead` - Drop headers from CSV file.
  * `enum` - Add a new column enumerating rows by adding a column of incremental or 
  uuid identifiers. Can also be used to copy a column or fill a new column with a constant value.
  * `explode` - Explode rows into multiple ones by splitting a column value based on the given separator.
  * `foreach` - Loop over a CSV file to execute bash commands.
  * `jsonl` - Convert newline-delimited JSON to CSV.
  * `lua` - Execute a Lua script over CSV lines to transform, aggregate or filter them.
  * `pseudo` -  Pseudonymise the value of the given column by replacing them by an incremental identifier.
  * `py` - Evaluate a Python expression over CSV lines to transform, aggregate or filter them.
  * `replace` - Replace CSV data using a regex.
  * `sort` --uniq option - When set, identical consecutive lines will be dropped to keep only one line 
  per sorted value.
  * `search` --flag `column` option -  If given, the command will not filter rows but will instead flag 
  the found rows in a new column named `column`.

- Added conditional compilation logic for `foreach` command to only 
compile on `target_family=unix` as it has a dependency on 
`std::os::unix::ffi::OsStrExt` which only works in unix-like OSes.
- Added `empty0` and `emptyNA` operations to `apply` command with 
corresponding test cases.
- Added GitHub Actions to check builds on `ubuntu-latest`, 
`windows-latest` and `macos-latest`.
- Added GitHub Action to publish binaries on release.
- Added `build.rs` build-dependency to check that Rust is at least 
at version 1.50.0 and above.

### Changed
- reformatted README listing of commands to use a table, and to link to
corresponding help text.

### Removed
- Removed appveyor.yml as qsv now uses GitHub Actions.

## [0.15.0] - 2021-09-22
### Added
- `dedup` cmd from [@ronohm](https://github.com/ronohm).
- `table` cmd `--align` option from [@alex-ozdemir](https://github.com/alex-ozdemir).
- `fmt` cmd `--quote-never` option from [@niladic](https://github.com/niladic).
- `exclude` cmd from [@lalaithion](https://github.com/lalaithion)
- Added `--dupes-output` option to `dedup` cmd.
- Added datetime type detection to `stats` cmd.
- Added datetime `min/max` calculation to `stats` cmd.
- es-ES translation from [@ZeliosAriex](https://github.com/ZeliosAriex).

### Changed
- Updated benchmarks script.
- Updated whirlwind tour to include additional commands.
- Made whirlwind tour reproducible by using `sample` `--seed` option.

### Fixed
- Fixed `sample` percentage sampling to be always reproducible even if
sample size < 10% when using `--seed` option.
- Fixed BOM issue with tests, leveraging [unreleased xsv fix](https://github.com/BurntSushi/xsv/commit/a1165e0fe58e6e39f6ed8b1a67ca87dd966c0df3).
- Fixed count help text typo.

### Removed
- Removed `session.vim` file.

## [0.14.1] - 2021-09-15
### Changed
- Performance: enabled link-time optimization (`LTO="fat"`).
- Performance: used code generation units.
- Performance: used [mimalloc](https://docs.rs/mimalloc/0.1.26/mimalloc/) allocator.
- Changed benchmark to compare xsv 0.13.0 and qsv.
- Changed chart from png to svg.
- Performance: Added note in README on how to optimize local compile 
by setting `target-cpu=native`.

## [0.14.0] - 2021-09-14
### Changed
- Renamed fork to qsv.
- Revised highlight note explaining reason for qsv renamed fork in README.
- Added **(NEW)** and **(EXPANDED)** notations to command listing.
- Adapted to Rust 2018 edition.
- used serde derive feature.

## [0.13.1] - 2020-12-27
Initial fork from xsv.
### Added
- `rename` cmd from [@Kerollmops](https://github.com/Kerollmops).
- `fill` cmd from [@alexrudy](https://github.com/alexrudy).
- `transpose` cmd from [@mintyplanet](https://github.com/mintyplanet).
- `select` cmd regex support from [@sd2k](https://github.com/sd2k).
- `stats` cmd `--nullcount` option from [@scpike](https://github.com/scpike).
- added percentage sampling to `sample` cmd.

### Changed
- Updated README with additional commands.
