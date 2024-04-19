# Performance Tuning

## CPU Optimization

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

## Memory Management
### Memory Allocator

qsv supports three memory allocators: mimalloc, jemalloc and the standard allocator.

By default, qsv uses [mimalloc](https://github.com/microsoft/mimalloc), a performance-oriented allocator from Microsoft.

You can also use another alternative - the [jemalloc](https://jemalloc.net) allocator, which is the default Linux allocator used by the [Pola.rs](https://www.pola.rs) project for its python bindings, as benchmarks have shown that it [performs better than mimalloc on some platforms](https://docs.rs/polars/latest/polars/#custom-allocator).

If you don't want to use mimalloc, use the `--no-default-features` flag when installing/compiling qsv, e.g.:

#### To use jemalloc

```bash
cargo install qsv --path . --no-default-features --features all_features,jemallocator
```

or

```bash
cargo build --release --no-default-features --features all_features,jemallocator
```

#### To use the standard allocator

```bash
cargo install qsv --path . --no-default-features --features all_features
```

or

```bash
cargo build --release --no-default-features --features all_features
```

To find out what memory allocator qsv is using, run `qsv --version`. After the qsv version number, the allocator used is displayed ("`standard`", "`mimalloc`" or "`jemalloc`"). Note that mimalloc is not supported on the `x86_64-pc-windows-gnu` and `arm` targets, and you'll need to use the "standard" allocator on those platforms.

### Out-of-Memory (OOM) Prevention
Most qsv commands use a "streaming" approach to processing CSVs - "streaming" in the input record-by-record while processing it. This allows it to process arbitrarily large CSVs with constant memory.

There are a number of commands/modes however (denoted by the "exploding head" emoji - 🤯), that require qsv to load the entire CSV into memory - `dedup` (when not using the --sorted option), `reverse`, `sort`, `stats` (when calculating the "non-streaming" extended stats), `table` and `transpose` (when not running in --multipass mode).

> NOTE: Though not as flexible, `dedup` and `sort` have corresponding "external" versions - `extdedup` and `extsort` respectively, that use external memory (i.e. disk) to process arbitrarily large CSVs.

In addition, `frequency`, `schema` and `tojsonl` - though they do not load the entire file into memory, uses additional memory proportional to the cardinality (number of unique values) of each column compared to other "streaming" commands (denoted by the "persevering face" emoji - 😣).

For very large files, this can be a problem, as qsv will run out of memory and panic.
To prevent this, qsv has two memory check heuristics when running "non-streaming" commands:

#### NORMAL mode
1. at startup, get the TOTAL memory of the system
2. if the size of the CSV file is greater than TOTAL memory - HEADROOM (default: 20%), qsv will abort with an error

#### CONSERVATIVE mode
1. at startup, compute total available memory by adding the current available memory and free swap space 
2. subtract a percentage headroom from the total available memory (default: 20%)
3. if this adjusted total available memory is less than the size of the CSV file, qsv will abort with an error

The percentage headroom can be changed by setting the `QSV_MEMORY_HEADROOM_PCT` environment variable to a value between 10 and 90 (default: 20).

This CONSERVATIVE heuristic can have false positives however, as modern operating systems can do a fair bit of juggling to handle file sizes larger than what this heuristic will allow, as it dynamically swaps apps to the swapfile, expand the swapfile, compress memory, etc.

For example, on a 16gb Mac mini running several common apps, it only allowed ~3gb csv files, but in practice, it was able to handle files up to 8gb before this heuristic was added.

To apply this CONSERVATIVE heuristic, you can use the command's `--memcheck` option or set the `QSV_MEMORY_CHECK` environment variable.

Otherwise, the default memory check heuristic (NORMAL mode) will only check if the input file's size is larger than the TOTAL memory of the computer minus `QSV_MEMORY_HEADROOM_PCT`.  We still do this to prevent OOM panics, but it's not as restrictive as the CONSERVATIVE heuristic. (e.g. if you have a 16gb computer, the maximum input file size is 12.8gb file - 16gb minus 20% headroom).

> NOTE: These memory checks are not invoked when using stdin as input, as the size of the input file is not known. Though `schema` and `tojsonl` will still abort if stdin is too large per this memory check as it creates a temporary file from stdin before inferring the schema.

### Buffer size

Depending on your filesystem's configuration (e.g. block size, file system type, writing to remote file systems (e.g. sshfs, efs, nfs),
SSD or rotating magnetic disks, etc.), you can also fine-tune qsv's read/write buffers.

By default, the read buffer size is set to [128k](https://github.com/jqnatividad/qsv/blob/master/src/config.rs#L16), you can change it by setting the environment
variable `QSV_RDR_BUFFER_CAPACITY` in bytes.

The same is true with the write buffer (default: 256k) with the `QSV_WTR_BUFFER_CAPACITY` environment variable.

## Multithreading

Several commands support multithreading - `stats`, `frequency`, `schema`, `split` and `tojsonl` (when an index is available); `apply`, `applydp`, `dedup`, `diff`, `excel`, `extsort`, `joinp`, `snappy`, `sort`, `sqlp`, `to` and `validate` (no index required).

qsv will automatically spawn parallel jobs equal to the detected number of logical processors. Should you want to manually override this, use the `--jobs` command-line option or the `QSV_MAX_JOBS` environment variable.

To find out your jobs setting, call `qsv --version`.

## Version details
The `--version` option shows a lot of information about qsv. It displays:
* qsv version
* the memory allocator (`standard`, `mimalloc` or `jemalloc`)
* all enabled features (`apply`, `fetch`, `foreach`, `luau`, `polars`, `python`, `self_update` & `to`)
* Python version linked if the `python` feature was enabled
* Luau version embedded if the `luau` feature was enabled
* the number of processors to use for multi-threading commands
* the number of logical processors detected
* memory-related OOM prevention info (max "non-streaming" input file size, free swap memory, available memory & total memory)
* the target platform
* the Rust version used to compile qsv
* QSV_KIND - `prebuilt`, `prebuilt-nightly`, `installed` & `compiled`.
   The prebuilts are the qsv binaries published on Github with every release. `prebuilt` is built using the current Rust stable at the time of release. `prebuilt-nightly` is built using Rust nightly that passes all CI tests at the time of release.
   `installed` is qsv built using `cargo install`. `compiled` is qsv built using `cargo build`.

```bash
$ qsv --version
qsv 0.122.0-mimalloc-apply;fetch;foreach;Luau 0.606;python-3.11.0 (v3.11.0:deaf509e8f, Oct 24 2022, 14:43:23) [Clang 13.0.0 (clang-1300.0.29.30)];to;self_update-8-8;3.66 GiB-913.00 MiB-3.69 GiB-16.00 GiB (aarch64-apple-darwin compiled with Rust 1.75.0) compiled
```

Shows that I'm running qsv version 0.122.0, with the `mimalloc` allocator (instead of `standard` or `jemalloc`), and I have the `apply`, `fetch`, `foreach`, `luau`, `python`, `to` and `self_update` features enabled, with the exact version of the embedded Luau interpreter, and the python version qsv is dynamically linked against. 

It shows qsv will use 8 logical processors out of 8 detected when running multithreaded commands.

It also shows that I can have a maximum input file size of 3.66 GiB for "non-streaming" commands (see [Memory Management](https://github.com/jqnatividad/qsv#memory-management) for more info), 913.00 MiB of free swap memory, 3.69 GiB of available memory and 16.00 GiB of total memory.

The qsv binary was built to target the aarch64-apple-darwin platform (Apple Silicon), compiled using Rust 1.75.0. The binary was `compiled` using `cargo build`.

## Caching
qsv employs several caching strategies to improve performance:

* qsv has large read and write buffers to minimize disk I/O. The default read buffer size is 128k and the default write buffer size is 512k. These can be fine-tuned with the `QSV_RDR_BUFFER_CAPACITY` and `QSV_WTR_BUFFER_CAPACITY` environment variables.
* The `stats` command caches its results in both CSV and binary formats. It does this to avoid re-computing the same statistics when the same input file/parameters are used, but also, as statistics are used in several other commands (currently - `schema` and `tojsonl`, with [more commands using cached statistics in the future](https://github.com/jqnatividad/qsv/issues/898)).   
The stats cache are automatically refreshed when the input file is modified the next time the `stats` command is run or when cache-aware commands attempt to use them. The stats cache is stored in the same directory as the input file. The stats cache files are named with the same file stem as the input file with the `stats.csv`, `stats.csv.json` and `stats.csv.bin` extensions. The CSV contains the cached stats, the JSON file contains metadata about how the stats were compiled, and the bin file is the binary encoded version of the stats that can be directly loaded into memory by other commands. The binary format is used by the `schema` and `tojsonl` commands and will only be generated when the `--stats-binout` option is set.
* The `geocode` command [memoizes](https://en.wikipedia.org/wiki/Memoization) otherwise expensive geocoding operations and will report its cache hit rate. `geocode` memoization, however, is not persistent across sessions.
* The `fetch` and `fetchpost` commands also memoizes expensive REST API calls. When the `--redis` option is enabled, it effectively has a persistent cache as the default time-to-live (TTL) before a Redis cache entry is expired is 28 days and Redis entries are persisted across restarts. Redis cache settings can be fine-tuned with the `QSV_REDIS_CONNSTR`, `QSV_REDIS_TTL_SECONDS`, `QSV_REDIS_TTL_REFRESH` and `QSV_FP_REDIS_CONNSTR` environment variables.
* The `luau` command caches lookup tables on disk using the QSV_CACHE_DIR environment variable and the `--cache-dir` command-line option. The default cache directory is `~/.qsv-cache`. The QSV_CACHE_DIR environment variable overrides the `--cache-dir` command-line option.

## SIMD-accelerated UTF-8 Validation for Performance
[Rust strings are utf-8 encoded](https://doc.rust-lang.org/std/string/struct.String.html). As a result, qsv **REQUIRES** UTF-8 encoded files.

Still, users will attempt to use non UTF-8 encoded files, and for the most part, they will still work! This is because most qsv commands use [ByteRecords](https://docs.rs/csv/latest/csv/struct.ByteRecord.html), where qsv manipulates raw bytes and doesn't care about the encoding.

Where it does matter, qsv will attempt to convert the bytes to UTF-8. But instead of using [std::str::from_utf8](https://doc.rust-lang.org/stable/std/str/fn.from_utf8.html), it makes extensive use of [`simdutf8`](https://github.com/rusticstuff/simdutf8#simdutf8--high-speed-utf-8-validation) for [SIMD](https://en.wikipedia.org/wiki/Single_instruction,_multiple_data)-accelerated UTF-8 validation, which is up to 23x faster on x86-64 and 11x faster on aarch64 (Apple Silicon).

As UTF-8 is the de facto encoding standard, this shouldn't be a problem most of the time. However, should you need to process a CSV file with a different encoding, use the `input` command with the `--output` option first to "[loosely transcode](https://doc.rust-lang.org/std/string/struct.String.html#method.from_utf8_lossy)" it to UTF-8.

## Nightly Release Builds
Pre-built binaries compiled using Rust Nightly/Unstable are also [available for download](https://github.com/jqnatividad/qsv/releases/latest). These binaries are optimized for size and speed:

* compiled with the last known Rust nightly/unstable that passes all 1,400+ tests.
* stdlib is compiled from source, instead of using the pre-built stdlib. This ensures stdlib is compiled with all of qsv's release settings
  (link time optimization, opt-level, codegen-units, panic=abort, etc.), presenting more opportunities for Rust/LLVM to optimize the generated code.
  This is why we only have nightly release builds for select platforms (the platform of GitHub's action runners), as we need access to the "native hardware"
  and cannot cross-compile stdlib to other platforms.
* set `panic=abort` - removing panic-handling/formatting and backtrace code, making for smaller binaries.
* enables unstable/nightly features in the `rand`, `regex`, `hashbrown`, `pyo3` and `polars` crates, that unlock performance/SIMD features on those crates.

Despite the 'unstable' label, these binaries are actually quite stable, given how [Rust is made](https://doc.rust-lang.org/book/appendix-07-nightly-rust.html) and are really more about performance (that's why we can still compile with Rust stable). You only really loose the backtrace messages when qsv panics.

If you need to maximize performance - use the nightly builds. If you prefer a "safer", rock-solid experience, use the stable builds.

If you want to really squeeze every little bit of performance from qsv, build it locally like how the Nightly Release Builds are built, with the additional step of optimizing the build to your machine's CPU by setting `RUSTFLAGS='-C target-cpu=native'`.

Doing so will ensure CPU features are tailored to your hardware and you're using the latest Rust nightly.

For example, on Ubuntu 22.04 LTS Linux:

```bash
rustup default nightly
rustup update
export RUSTFLAGS='-C target-cpu=native'

# to build qsv on nightly with all features. The binary will be in the target/release-nightly folder.
cargo build --profile release-nightly --bin qsv -Z build-std=std,panic_abort \
  -Z build-std-features=panic_immediate_abort \
  --features all_features,nightly --target x86_64-unknown-linux-gnu

# to build qsvlite
cargo build --profile release-nightly --bin qsvlite -Z build-std=std,panic_abort \
  -Z build-std-features=panic_immediate_abort \
  --features lite,nightly --target x86_64-unknown-linux-gnu

# to build qsvdp
cargo build --profile release-nightly --bin qsvdp -Z build-std=std,panic_abort \
  -Z build-std-features=panic_immediate_abort \
  --features datapusher_plus,nightly --target x86_64-unknown-linux-gnu
```

With that said, there are times that Rust Nightly/Unstable does "break" qsv. That's why we include `qsv_rust_version_info.txt` in the 
nightly release build zip files, should you need to pin Rust to a specific nightly version when building locally.

## Benchmarking for Performance

Use and fine-tune the [benchmark script](scripts/benchmark-basic.sh) when tweaking qsv's performance to your environment.
Don't be afraid to change the benchmark data and the qsv commands to something that is more representative of your
workloads.

Use the generated benchmark TSV files to meter and compare performance across platforms. You'd be surprised how performance varies
across environments - e.g. qsv's `join` performs abysmally on Windows's WSL running Ubuntu 20.04 LTS, taking 172.44 seconds.
On the same machine, running in a VirtualBox VM at that with the same Ubuntu version, `join` was done in 1.34 seconds -
two orders of magnitude faster!

However, `stats` performs two times faster on WSL vs the VirtualBox VM - 2.80 seconds vs 5.33 seconds for the `stats_index` benchmark.
