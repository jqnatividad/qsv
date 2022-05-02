# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]
## [0.45.2] - 2022-05-01
### Added
* added `headers` command to qsvdp binary

### Changed
* cargo update bump semver from 1.0.7 to 1.0.8

## [0.45.1] - 2022-05-01
### Added
* added rust-clippy GH action workflow
* added security policy

### Changed:
* `extsort`: use util::njobs to process --jobs option
* various improvements on Whirlwind tour to help users follow along
* `extsort`: add link to "External Sorting" wikipedia article
* `extsort`: made <input> and <output> mandatory docopt arguments 
* `sort`: mention `extsort` in usage text
* added markdownlint.json config to suppress noisy markdown lints in VSC
* reformatted README to apply some markdown lints
* bump whatlang from 0.14 to 0.15
* bump qsv-stats from 0.3.6 to 0.3.7 for some minor perf improvements

## [0.45.0] - 2022-04-30
### Added
* Added `extsort` command - sort arbitrarily large text files\CSVs using a multi-threaded external sort algorithm.

### Changed
* Updated whirlwind tour with simple `stats` step
* `py`: Automatically create python3.dll import libraries on Windows targets
* Updated build instructions to include `full` feature
* `index`: mention QSV_AUTOINDEX env var in usage text
* Corrected minor typos
* Bump jql from 4.0.1 to 4.0.2 by @dependabot in https://github.com/jqnatividad/qsv/pull/276
* cargo update bump several dependencies - notably mimalloc

## [0.44.0] - 2022-04-27
### Added
* Created new binary - qsvdp - binary optimized for [DataPusher+](https://github.com/dathere/datapusher-plus) in https://github.com/jqnatividad/qsv/pull/273
  qsvdp only has DataPusher+ relevant commands, with the self-update engine removed. This results in a binary that's
  3x smaller than qsvlite, and 6x smaller than qsv will all features enabled.

### Changed
* `dedup`: send dupe count to stderr in https://github.com/jqnatividad/qsv/pull/272
* `dedup`: improve usage text
* cargo update bump several crates

### Fixed
* `count`: corrected usage text typo

## [0.43.0] - 2022-04-26
### Added
* `input` can now effectively transcode non-utf-8 encoded files to utf-8 in https://github.com/jqnatividad/qsv/pull/271

### Changed
* `table`: made it flexible - i.e. each row can have varying number of columns
* `excel`: remove unneeded closure

## [0.42.2] - 2022-04-25
### Changed
* use our grex fork, as the upstream fork has an unpublished version number that prevents us from publishing on crates.io.

## [0.42.1] - 2022-04-25
### Changed
* use `[patch.crates-io]` to use crate forks, rather than using the git directive in the dependencies section.
This has the added benefit of making the dependency tree smaller, as other crates that depend on the patched crates also
use the patches. This should also result in smaller binaries.

## [0.42.0] - 2022-04-24
### Added
* `input` refactor. Added trimming and epilog skiplines option. https://github.com/jqnatividad/qsv/pull/270
* `sniff`: added note about sniff limitations
* also publish x86_64-unknown-linux-musl binary

### Changed
* Bump anyhow from 1.0.56 to 1.0.57 by @dependabot in https://github.com/jqnatividad/qsv/pull/268
* Bump jsonschema from 0.15.2 to 0.16.0
* use optimized fork of rust-csv, with non-allocating, in-place trimming and various perf tweaks
* use optimized fork of docopt.rs, with various perf & memory allocation tweaks
* use reqwest fork with unreleased changes that remove unneeded crates
* `validate`: use `from_utf8_unchecked` in creating json instances for performance

### Fixed
* `input`: Fixed line-skipping logic so CSV parsing is flexible - i.e. column count can change between records

## [0.41.0] - 2022-04-21
### Added
* `input`: add `--skip-lines` option in https://github.com/jqnatividad/qsv/pull/266

### Changed
* More verbose, matching START/END logging messages when `QSV_LOG_LEVEL` is enabled.
* Bump whatlang from 0.13.0 to 0.14.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/264
* Bump filetime from 0.2.15 to 0.2.16 by @dependabot in https://github.com/jqnatividad/qsv/pull/263
* Bump uuid from 0.8 to 1 in https://github.com/jqnatividad/qsv/pull/267
* Minor documentation improvements
* `cargo update` bumped several other second-level dependencies

## [0.40.3] - 2022-04-14
### Changed
* Bump pyo3 from 0.16.3 to 0.16.4
* `stats`: renamed `--dates` option to `--infer-dates`
### Fixed
* `stats`: fixed panic caused by wrong type inference when `--infer-dates` option is on in https://github.com/jqnatividad/qsv/pull/256

## [0.40.2] - 2022-04-14
### Changed
* Datapusher tweaks, primarily to help with datapusher error-handling in https://github.com/jqnatividad/qsv/pull/255
* `excel`: exported count with `--human-readable` option
* use calamine fork to bump dependencies, and reduce binary size
* Bump rayon from 1.5.1 to 1.5.2 by @dependabot in https://github.com/jqnatividad/qsv/pull/254
* Bump jql from 4.0.0 to 4.0.1

### Fixed
* removed unnecessary *.d dependency files from published binaries zip

## [0.40.1] - 2022-04-13
### Changed
* use performance tweaked forks of csv crate
* Made `this_error` dependency optional with `fetch` feature 
* Made `once_cell` dependency optional  with `apply` and `fetch` features

### Fixed
* Fixed qsv binary publishing. qsv binary was not built properly, it was built using a qsvlite profile

## [0.40.0] - 2022-04-12
### Added
* `excel` command in https://github.com/jqnatividad/qsv/pull/249 and https://github.com/jqnatividad/qsv/pull/252

### Changed
* Bump jql from 3.3.0 to 4.0.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/251
* Bump actions/setup-python from 3.1.1 to 3.1.2 by @dependabot in https://github.com/jqnatividad/qsv/pull/250

## [0.39.1] - 2022-04-11
### Fixed
* added version to grex dependency as its required by crates.io, though we're still using the grex fork without the CLI components.

## [0.39.0] - 2022-04-10
### Added
* `QSV_AUTOINDEX` environment variable. When set, autoindexes csv files, autoupdates stale indices 
* `replace`: \<NULL\> `--replacement` option (https://github.com/jqnatividad/qsv/pull/244)
* qsv now automatically screens files for utf-8 encoding. Set `QSV_SKIPUTF8_CHECK` env var to skip encoding check. (https://github.com/jqnatividad/qsv/pull/245 and https://github.com/jqnatividad/qsv/pull/248)

### Changed
* `foreach`: refactored. (https://github.com/jqnatividad/qsv/pull/247)
* Bump jql from 3.2.3 to 3.3.0
* Bump actions/setup-python from 3.1.0 to 3.1.1 by @dependabot in https://github.com/jqnatividad/qsv/pull/246
* use grex fork to remove unneeded CLI dependencies

## [0.38.0] - 2022-04-05
### Changed
* qsv **requires** UTF-8/ASCII encoded files. Doing so allows us to squeeze more performance by removing UTF-8 validation in https://github.com/jqnatividad/qsv/pull/239 and https://github.com/jqnatividad/qsv/pull/240
### Fixed
* fixed `--jobs` parameter parsing for multithreaded commands in https://github.com/jqnatividad/qsv/pull/236 and https://github.com/jqnatividad/qsv/pull/237

## [0.37.2] - 2022-04-03
### Fixed
* Handle/log self-update errors in https://github.com/jqnatividad/qsv/pull/233

## [0.37.1] - 2022-04-03
### Changed
* `fetch` and `apply`: use cheaper, faster lookup tables for dynamic formatting in https://github.com/jqnatividad/qsv/pull/231
* Cleanup - remove commented code; convert `match` to `if let`; more pedantic clippy recommendations, etc. in https://github.com/jqnatividad/qsv/pull/232

## [0.37.0] - 2022-04-02
### Added
* `enumerate`: added `--constant` <NULL> sentinel value in https://github.com/jqnatividad/qsv/pull/219
* `fetch`: added `--jqlfile` option in https://github.com/jqnatividad/qsv/pull/220
* `stats`: more perf tweaks in https://github.com/jqnatividad/qsv/pull/223

### Changed
* `fetch`: argument parsing refactor, removing need for dummy argument in https://github.com/jqnatividad/qsv/pull/222
* applied select pedantic clippy recommendations in https://github.com/jqnatividad/qsv/pull/224
* simplified multithreading - removed jobs div by three heuristic in https://github.com/jqnatividad/qsv/pull/225
* use qsv-dateparser fork of dateparser for incresed performance of `stats`, `schema` and `apply` in https://github.com/jqnatividad/qsv/pull/230
* Bump actions/checkout from 2.3.3 to 3 by @dependabot in https://github.com/jqnatividad/qsv/pull/228
* Bump actions/stale from 3 to 5 by @dependabot in https://github.com/jqnatividad/qsv/pull/227
* Bump actions/setup-python from 2 to 3.1.0 by @dependabot in https://github.com/jqnatividad/qsv/pull/226

## [0.36.1] - 2022-03-26
### Changed
* `validate`: use user agent & compression settings when fetching jsonschema from a URL in https://github.com/jqnatividad/qsv/pull/207
* Build and publish smaller qsvlite binary in https://github.com/jqnatividad/qsv/pull/208, https://github.com/jqnatividad/qsv/pull/210 & https://github.com/jqnatividad/qsv/pull/213
* `sniff`: now works with stdin in https://github.com/jqnatividad/qsv/pull/211 and https://github.com/jqnatividad/qsv/pull/212
* `stats`: remove smartstring in https://github.com/jqnatividad/qsv/pull/214
* various performance tweaks in `stats` and `select`
### Fixed
* README: Installation - git:// is no longer supported by GitHub  by @harrybiddle in https://github.com/jqnatividad/qsv/pull/205
* README: Fixed wrong footnote for feature flags
* Silent error when an index file is not found is now logged (https://github.com/jqnatividad/qsv/commit/7f2fe7f3259fb74a8d76396dcc2aa71585967b9b)
* bumped self-update to 0.29. This partly addresses #167, as self-update had an indirect dependency to `time` 0.1.43.

## [0.36.0] - 2022-03-20
### Added
* `sniff`: new command to quickly detect CSV metadata in https://github.com/jqnatividad/qsv/pull/202
* auto-delimiter setting with `QSV_SNIFF_DELIMITER` environment variable in https://github.com/jqnatividad/qsv/pull/203
* `apply`: new `dynfmt` multi-column, dynamic formatting subcommand in https://github.com/jqnatividad/qsv/pull/200
* `fetch`: new multi-column dynamic formatting with --url-template option in https://github.com/jqnatividad/qsv/pull/196
### Changed
* `fetch`: --url-template safety tweaks in https://github.com/jqnatividad/qsv/pull/197
* `fetch`: automatically minify JSON responses. JSON can still be pretty-printed with --pretty option in https://github.com/jqnatividad/qsv/pull/198
* `fetch` is now an optional feature in https://github.com/jqnatividad/qsv/pull/201
* `sniff`: improved display in https://github.com/jqnatividad/qsv/pull/204
* slim down dev-dependencies
### Fixed:
* `py`: now checks if first character of a column is a digit, and replaces it with an underscore

## [0.35.2] - 2022-03-13
### Added
* README: Added datHere logo
### Changed
* `py`: ensure valid python variable names https://github.com/jqnatividad/qsv/pull/192
* `fetch`: dev-dependency actix upgrade (actix-governor from 0.2->0.3; actix-web from 3.3->4.0) https://github.com/jqnatividad/qsv/pull/193
* `lua`: replace hlua with mlua  https://github.com/jqnatividad/qsv/pull/194
* `stats`: refactor for performance - skip from_utf8 check as input is utf8 transcoded as necessary; smartstring https://github.com/jqnatividad/qsv/pull/195
* Whirlwind Tour: show country-continent.csv file with comment handling
* cargo bump update several dependencies

### Fixed
* `stats`: only compute quartiles/median for int/float fields - https://github.com/jqnatividad/qsv/pull/195

## [0.35.1] - 2022-03-08

### Changed
- README: note about `--output` option changing delimiter automatically based on file extension and UTF-8 encoding the file
- README: Windows usage note about UTF16-LE encoding and `--output` workaround
### Fixed
* upgraded regex to 1.5.5 which resolves the [GHSA-m5pq-gvj9-9vr8 security advisory](https://github.com/rust-lang/regex/security/advisories/GHSA-m5pq-gvj9-9vr8)

## [0.35.0] - 2022-03-08
### Added
* `count`: `--human-readable` option in https://github.com/jqnatividad/qsv/pull/184
* Automatic utf8 transcoding in https://github.com/jqnatividad/qsv/pull/187
* Added NYC School of Data 2022 presentation
* Added ahash 0.7 and encoding_rs_io 0.1 dependencies

### Changed
* Use ahash::AHashMap instead of std::collections::HashMap for performance in https://github.com/jqnatividad/qsv/pull/186
* Revamped Whirlwind Tour
* bumped several dependencies 
  * anyhow 1.0.55 to 1.0.56
  * ipnet 2.3.1 to 2.4.0
  * pyo3 0.16.0 to 0.16.1

### Fixed
* `py`: convert spaces to underscores for valid python variable names when Column names have embedded spaces in https://github.com/jqnatividad/qsv/pull/183
* docs: CSV Kit got a 10x improvement by @jpmckinney in https://github.com/jqnatividad/qsv/pull/180
* `fetch`: added jql selector to cache key
* Corrected README mixup re `join` hashmap indices and qsv indices

## New Contributors
* @jpmckinney made their first contribution in https://github.com/jqnatividad/qsv/pull/180

## [0.34.1] - 2022.03-04
### Added
* `stats`: added `--dates` option. This option turns on date/datetime data type inferencing, which is 
a very expensive operation. Only use this option when you have date/datetime fields and you want to 
compile the proper statistics for them (otherwise, they will be treated as "String" fields.)

## [0.34.0] - 2022.03-03
### Added
* added intentionally kitschy qsv logo :grin:
* `stats`: added `datetime` data type inferencing
* `fetch`: added optional Redis response caching
* `schema`: added `--strict-dates` option by @mhuang74 in https://github.com/jqnatividad/qsv/pull/177 
* `validate`: added more robust [RFC 4180](https://datatracker.ietf.org/doc/html/rfc4180)-compliance checking when no jsonschema is provided
* added Redis to CI

### Changed
* bumped reverse-geocoder crate from 2.0.1 to 3.0.0 to modernize geonames reverse geocoder
* bumped cached crate from 0.30.0 to 0.33.0 to enable Redis response caching
* bumped various other dependencies to latest release

### Fixed
* removed invalid `--path` cargo install option in README
* `workdir.rs` was not properly cleaning up test files

## [0.33.0] - 2022.02-27
### Added
* `fetch`: add `--url-template` and `--redis` options in https://github.com/jqnatividad/qsv/pull/175
* `stats`: add `DateTime` data type (RFC3339 format) in https://github.com/jqnatividad/qsv/pull/176
* added Rust Beta to Github Actions CI

### Changed
* `validate`: improve performance and simplify error report format by @mhuang74 in https://github.com/jqnatividad/qsv/pull/172
* Addl `validate` performance tweaks in https://github.com/jqnatividad/qsv/pull/173
* changed MSRV to latest Rust stable - 1.59.0
* removed `num_cpus` crate and use new `std::thread::available_parallelism` stabilized in Rust 1.59.0
* use new cargo.toml `strip` option to strip binaries
* refactored GitHub Actions CI to make it faster

## [0.32.2] - 2022-02-20
### Changed
* `schema` (#60): pattern constraint for string types by @mhuang74 in https://github.com/jqnatividad/qsv/pull/168
* `validate`: improve performance by @mhuang74 in https://github.com/jqnatividad/qsv/pull/170
* `fetch`: Spell out k:v -> key:value in docopt usage text
* cargo update bump several dependencies

### Fixed
* `validate`: bug fix and refactor by @mhuang74 in https://github.com/jqnatividad/qsv/pull/171

## [0.32.1] - 2022-02-14
### Changed
* `fetch`: upgrade to jql 3.1.0 by @mhuang74 in https://github.com/jqnatividad/qsv/pull/160
* `schema`: refactor tests by @mhuang74 in https://github.com/jqnatividad/qsv/pull/161
* `schema`: support Enum constraint by @mhuang74 in https://github.com/jqnatividad/qsv/pull/162
* `schema`: default to include value constraints  by @mhuang74 in https://github.com/jqnatividad/qsv/pull/166
* bumped `qsv-stats` to 0.3.6 for `stats` & `frequency` performance tweaks
* specify that `apply geocode` expects WGS84 coordinate system
* cargo update bump several dependencies
* changed CI to run clippy and rustfmt automatically

### Fixed
* `schema`: Fix bug with enum by @mhuang74 in https://github.com/jqnatividad/qsv/pull/163

## [0.32.0] - 2022-02-06
### Added
* `schema` POC by @mhuang74 in https://github.com/jqnatividad/qsv/pull/155
* `schema`: add value constraints via stats  by @mhuang74 in https://github.com/jqnatividad/qsv/pull/158
* `schema`: update command description by @mhuang74 in https://github.com/jqnatividad/qsv/pull/159

### Changed
* `stats` data type inference changed to more straightforward "String" from "Unicode"
* changed CI/CD to use rust-cache GitHub Actions making it ~3x faster.
* always build and test with `--locked` flag. This allows us to use rust-cache and guarantee that
  builds are using the exact dependency versions qsv requires.
* bumped `qsv-stats` to 0.3.5 for `stats` performance tweaks  

### Fixed
* Validate: bug fixes by @mhuang74 in https://github.com/jqnatividad/qsv/pull/154

## [0.31.0] - 2022-01-31
### Changed
* Validate: bug fixes by @mhuang74 in https://github.com/jqnatividad/qsv/pull/151
* Python 3.8 (current stable version) is now required for the `py` command. Changed from Python 3.7.
* bumped jsonschema dependency to to 0.15.
* always build/publish with `--locked` flag in CI/CD.
* enclose environment variable values with double quotes when using `--envlist` option
* use more captured identifiers in format strings.

### Added
* added `--helper` option to `py` command. This allows users to load a python user helper script as a module named `qsv_uh`. [Example](https://github.com/jqnatividad/qsv/blob/78046d922e9a530c0887f18065fc325049b58687/tests/test_py.rs#L93) 
* added support for last N records in `slice` command by allowing negative values for the `slice --start` option.
* added progress bar to `py` command.

## [0.30.1] - 2022-01-23
### Changed
* convert more format strings to use captured identifiers
* bump jsonschema to 0.14.0. This will allow cross-compilation to work again as 
  we can explicitly use rustls for reqwest. This is required as cross no longer bundles openssl.

### Fixed
* fixed broken self-update ([#150](https://github.com/jqnatividad/qsv/issues/150))

## [0.30.0] - 2022-01-22
### Added
* `validate` command by @mhuang74 in https://github.com/jqnatividad/qsv/pull/145
* README: additional information on xsv fork differences

### Changed
* bumped MSRV to 1.58.1
* `validate` tweaks in https://github.com/jqnatividad/qsv/pull/148
* `validate` buffered jsonl error report in https://github.com/jqnatividad/qsv/pull/149

### Fixed
* fix `fetch` bugs by @mhuang74 in https://github.com/jqnatividad/qsv/pull/146
* README: added missing `--path` option in `cargo install`

## [0.29.1] - 2022-01-17
### Changed
* refactored `--update` to give update progress messages; run on `--help` as well
* updated README
  - remove bold formatting of commands
  - expanded descriptions of
      - fixlengths
      - foreach
      - jsonl
      - py
    - searchset
  - added reason why pre-built binaries on some platforms do not have the python feature installed.
  - drop use of "parallelism", just say "multithreading"
  - expanded Feature Flag section
* bump cached from 0.26 to 0.29
* added `update_cache_info!` macro to util.rs, replacing redundant code for progress indicators with cache info
* bump MSRV to Rust 1.58
* use new Rust 1.58 captured identifiers for format strings
* added `output_stderr` test helper to test for expected errors in CI
* added tests for invalid delimiter length; truncated comment char and unknown apply operators
* pointed documentation to Github README instead of doc.rs
* added `rustup update` to Github Actions publish workflow as Github's runners are still on Rust 1.57
* added Debian package build to publish workflow for `x86_64-unknown-linux-musl`

### Fixed
* corrected help text on job divisor is 3 not 4 for multithreaded commands (`frequency`, `split` and `stats`)
* corrected `stats` help text to state that multithreading requires an index

## [0.29.0] - 2022-01-08
### Changed
* `fetch`: enable cookies and storing error messages by @mhuang74 in https://github.com/jqnatividad/qsv/pull/141
* `fetch`: improve jql integration by @mhuang74 in https://github.com/jqnatividad/qsv/pull/139
* `--envlist` option now returns all qsv-relevant environment variables in https://github.com/jqnatividad/qsv/pull/140
* Move logging and update utility functions to util.rs in https://github.com/jqnatividad/qsv/pull/142
* `fetch`: support custom http headers by @mhuang74 in https://github.com/jqnatividad/qsv/pull/143
* bumped whatlang to 13.0 which supports Tagalog detection
* improved documentation of feature flags, environment variables & `stats` command

### Added
* added JSONL/NDJSON to Recognized File Formats (thru `jsonl` command)
* added CODE_OF_CONDUCT.md

### Deleted
* removed WIP indicator from `fetch` in README

## [0.28.0] - 2021-12-31
### Changed
* Fetch: support rate limiting by @mhuang74 in https://github.com/jqnatividad/qsv/pull/133
* Runtime minimum version check for Python 3.7 if `python` feature is enabled by @jqnatividad in https://github.com/jqnatividad/qsv/pull/138
* Fine-tuned GitHub Actions publish workflow for pre-built binaries
   * removed upx compression, as it was creating invalid binaries on certain platforms
   * enabled `python` feature on x86_64 platforms as we have access to the Python interpreter on GitHub's Action runners
   * include both `qsv` and `qsvlite` in the distribution zip file
* Formatted Cargo.toml with Even Better TOML VS code extension
* changed Cargo.toml categories and keywords
* removed patch version number from Cargo.toml dependencies. Let cargo do its semver dependency magic, and we include the Cargo.lock file anyway.

### Added
* added example of Python f-string formatting to `py` help text
* added Python f-string formatting test
* Added note in README about enabled features in pre-built binaries

### Deleted
* Removed _**NEW**_ and _**EXTENDED**_ indicators in README

## [0.27.1] - 2021-12-28
### Changed
* changed publish workflow for apple targets to use Xcode 12.5.1 from 12.4
* `jsonl` command now recognize and process JSON arrays
* `--version` option now shows binary name and enabled features
* Use upgraded [`qsv_currency`](https://crates.io/crates/qsv_currency) fork to power `apply currencytonum` operation. Now supports currency strings
  (e.g. USD, EUR, JPY, etc) in addition to currency symbols (e.g. $, €, ¥, etc)
* renamed `QSV_COMMENTS` environment variable to `QSV_COMMENT_CHAR` to make it clear that it clear that we're expecting
  a single character, not a boolean as the old name implies.

### Added
* added `create_from_string` helper function in workdir.rs
* compress select pre-built binaries with [UPX](https://upx.github.io/)
* `qsvlite` binary target, with all features disabled.
* `py` command. Evaluates a Python expression over CSV lines to transform, aggregate or filter them.

### Deleted
* removed Debian package publishing workflow, as the GH action for it
  does not support Rust 2021 edition

## [0.26.2] - 2021-12-21
## Added
* automatic self-update version check when the `--list` option is invoked.
* `QSV_NO_UPDATE` environment variable to prohibit self-update checks.
### Fixed
* explicitly include `deflate` compression method for self_update. Otherwise, `--update` unzipping doesn't work.
## [0.26.1] - 2021-12-21
### Fixed
* explicitly include `deflate` compression method for self_update. Otherwise, `--update` unzipping doesn't work.
## [0.26.0] - 2021-12-21
### Changed
* `fetch` refinements. Still WIP, but usable (See [#77](https://github.com/jqnatividad/qsv/issues/77))
  - add default user agent
  - `fetch` progress bar
  - `--jobs`, `--throttle`, `--header`, `--store-error` and `cookies` options still not functional.
* cargo update bump several crates to their latest releases. Of note are `test-data-generation`, 
`self_update` and `jql` where we worked with the crate maintainers directly with the update.

### Fixed
* `--update` bug fixed. It was not finding the binary to self update properly.


## [0.25.2-beta] - 2021-12-13
## Added
* `fetch` command by [@mhuang74](https://github.com/mhuang74). Note that the command is functional but still WIP, that's why this is a beta release.
* Download badge for GitHub pre-built binaries
* Compute hashes for pre-built binaries for verification

## Changed
* Additional helptext for `apply` NLP functions
* standardized on canonical way to suppress progress bars with `--quiet` option
* README: Mentioned `--frozen` option when installing/building qsv; wordsmithing
* rustfmt; clippy

## Deleted
* remove obsolete Makefile and .gitsubmodules
## [0.24.1] - 2021-12-06
### Changed
- changed selfupdate dependency to use pure Rust TLS implementation as cross no longer bundles OpenSSL, causing some binary builds using cross to fail.
## [0.24.0] - 2021-12-06
### Added
* Add logging by @mhuang74 in https://github.com/jqnatividad/qsv/pull/116
* Environment variables for logging - `QSV_LOG_LEVEL` and `QSV_LOG_DIR` - see [Logging](https://github.com/jqnatividad/qsv/blob/master/docs/Logging.md#logging) for more details.
* `sentiment` analysis `apply` operation by @jqnatividad in https://github.com/jqnatividad/qsv/pull/121
* `whatlang` language detection `apply` operation by @jqnatividad in https://github.com/jqnatividad/qsv/pull/122
* aarch64-apple-darwin prebuilt binary (Apple Silicon AKA M1)
* `--envlist` convenience option to list all environment variables with the `QSV_` prefix

### Changed
* changed `MAX_JOBS` heuristic logical processor divisor from 4 to 3
* `selfupdate` is no longer an optional feature

## New Contributors
* @mhuang74 made their first contribution in https://github.com/jqnatividad/qsv/pull/116
## [0.23.0] - 2021-11-29
### Added
- added `--update` option. This allows qsv to check and update itself if there are new release binaries published on GitHub.
- added `--envlist` option to show all environment variables with the `QSV_` prefix.
- `apply`, `generate`, `lua`, `foreach` and `selfupdate` are now optional features. `apply` and `generate` are marked optional since they have
large dependency trees; `lua` and `foreach` are very powerful commands that can be abused to issue system commands. Users now have the option exclude these features from their local builds.  Published binaries on GitHub still have `-all-features` enabled.
- added `QSV_COMMENTS` environment variable (contributed by [@jbertovic](https://github.com/jbertovic)). This allows qsv to ignore lines in the CSV (including headers) that start with the set character. [EXAMPLES](https://github.com/jqnatividad/qsv/blob/feae8cf5750530318b83c4b3c7bf0f72d2332079/tests/test_comments.rs#L3)
- catch input empty condition when qsv's input is empty when using `select`.   
(e.g. `cat /dev/null | qsv select 1` will now show the error "Input is empty." instead of "Selector index 1 is out of bounds. Index must be >= 1 and <= 0.")
- added `--pad <arg>` option to `split` command to zero-pad the generated filename by the number of `<arg>` places. [EXAMPLES](https://github.com/jqnatividad/qsv/blob/feae8cf5750530318b83c4b3c7bf0f72d2332079/tests/test_split.rs#L81)
- tests for `QSV_COMMENTS`, `split --pad`, `select` input empty condition, 
### Changed
- set Cargo.toml to Rust 2021 edition
- added "command-line-utilities" category to crates.io metadata
- cargo update bumped `mimalloc`, `serde_json`, `syn`, `anyhow` and `ryu`.
- GitHub Actions CI tests runs with `--all-features` enabled.
- published binaries on GitHub have `--all-features` enabled by default.
- made geocode caching a tad faster by making the transitional cache unbounded, and simplifying the key.
- `--version` now also shows the number of logical CPUs detected.
- project-wide rustfmt
- documentation for features, `QSV_COMMENTS` and `apply`
### Removed
- removed greetings.yml workflow from GitHub Actions.

## [0.22.1] - 2021-11-22
### Added
- added `lua` and `foreach` feature flags. These commands are very powerful and can be easily abused or get into "foot-shooting" scenarios.
They are now only enabled when these features are enabled during install/build.
- `censor` and `censor_check` now support the addition of custom profanities to screen for with the --comparand option.
### Changed
- removed `lazy_static` and used `once_cell` instead
- smaller stripped binaries for `x86_64-unknown-linux-gnu`, `i686-unknown-linux-gnu`, `x86_64-apple-darwin` targets
- expanded `apply` help text
- added more tests (currencytonum, censor, censor_check)

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
