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

## Memory Allocator

qsv supports three memory allocators: mimalloc, jemalloc and the standard allocator.

By default, qsv uses [mimalloc](https://github.com/microsoft/mimalloc), a performance-oriented allocator from Microsoft.

You can also use another alternative - the [jemalloc](https://jemalloc.net) allocator, which is the default Linux allocator used by the [Pola.rs](https://www.pola.rs) project for its python bindings, as benchmarks have shown that it [performs better than mimalloc on some platforms](https://docs.rs/polars/latest/polars/#custom-allocator).

If you don't want to use mimalloc, use the `--no-default-features` flag when installing/compiling qsv, e.g.:

### To use jemalloc

```bash
cargo install qsv --path . --no-default-features --features all_full,jemallocator
```

or

```bash
cargo build --release --no-default-features --features all_full,jemallocator
```

### To use the standard allocator

```bash
cargo install qsv --path . --no-default-features --features all_full
```

or

```bash
cargo build --release --no-default-features --features all_full
```

To find out what memory allocator qsv is using, run `qsv --version`. After the qsv version number, the allocator used is displayed ("`standard`", "`mimalloc`" or "`jemalloc`"). Note that mimalloc is not supported on the `x86_64-pc-windows-gnu` and `arm` targets, and you'll need to use the "standard" allocator on those platforms.

## Buffer size

Depending on your filesystem's configuration (e.g. block size, file system type, writing to remote file systems (e.g. sshfs, efs, nfs),
SSD or rotating magnetic disks, etc.), you can also fine-tune qsv's read/write buffers.

By default, the read buffer size is set to [16k](https://github.com/jqnatividad/qsv/blob/master/src/config.rs#L16), you can change it by setting the environment
variable `QSV_RDR_BUFFER_CAPACITY` in bytes.

The same is true with the write buffer (default: 64k) with the `QSV_WTR_BUFFER_CAPACITY` environment variable.

## Multithreading

Several commands support multithreading - `stats`, `frequency`, `schema`, `split` and `tojsonl` (when an index is available); `apply`, `dedup`, `extsort`, `sort` and `validate` (no index required).

qsv will automatically spawn parallel jobs equal to the detected number of logical processors. Should you want to manually override this, use the `--jobs` command-line option or the `QSV_MAX_JOBS` environment variable.

To find out your jobs setting, call `qsv --version`.

## Version details
The `--version` option shows a lot of information about qsv. It displays:
* qsv version
* the memory allocator (`standard`, `mimalloc` or `jemalloc`)
* all enabled features (`apply`, `fetch`, `foreach`, `generate`, `luau`, `python`, `self_update` & `to`)
* Python version linked if the `python` feature was enabled
* Luau version embedded if the `luau` feature was enabled
* the number of processors to use for multi-threading commands
* the number of logical processors detected
* memory-related information (max "non-streaming" input file size, free swap memory, available memory & total memory)
* the target platform
* the Rust version used to compile qsv
* QSV_KIND - `prebuilt`, `prebuilt-nightly`, `installed` & `compiled`.
   The prebuilts are the qsv binaries published on Github with every release. `prebuilt` is built using the current Rust stable at the time of release. `prebuilt-nightly` is built using Rust nightly that passes all CI tests at the time of release.
   `installed` is qsv built using `cargo install`. `compiled` is qsv built using `cargo build`.

```bash
$ qsv --version
qsv 0.88.2-mimalloc-apply;fetch;foreach;generate;Luau 0.561;python-3.11.0 (v3.11.0:deaf509e8f, Oct 24 2022, 14:43:23) [Clang 13.0.0 (clang-1300.0.29.30)];to;self_update-8-8;3.66 GiB-913.00 MiB-3.69 GiB-16.00 GiB (aarch64-apple-darwin compiled with Rust 1.67.1) compiled
```

Shows that I'm running qsv version 0.88.2, with the `mimalloc` allocator (instead of `standard` or `jemalloc`), and I have the `apply`, `fetch`, `foreach`, `generate`, `luau`, `python`, `self_update` and `to` features enabled, with the exact version of the embedded Luau interpreter, and the python version qsv is dynamically linked against. 

It shows qsv will use 8 logical processors out of 8 detected when running multithreaded commands.

It also shows that I can have a maximum input file size of 3.66 GiB for "non-streaming" commands (see [Memory Management](https://github.com/jqnatividad/qsv#memory-management) for more info), 913.00 MiB of free swap memory, 3.69 GiB of available memory and 16.00 GiB of total memory.

The qsv binary was built to target the aarch64-apple-darwin platform (Apple Silicon), compiled using Rust 1.67.1. The binary was `compiled` using `cargo build`.

## Caching
The `apply geocode` command [memoizes](https://en.wikipedia.org/wiki/Memoization) otherwise expensive geocoding operations and will report its cache hit rate. `apply geocode` memoization, however, is not persistent across sessions.

The `fetch` and `fetchpost` commands also memoizes expensive REST API calls with its optional Redis support. It effectively has a persistent cache as the default time-to-live (TTL) before a Redis cache entry is expired is 28 days and Redis entries are persisted across restarts. Redis cache settings can be fine-tuned with the `QSV_REDIS_CONNSTR`, `QSV_REDIS_TTL_SECONDS`, `QSV_REDIS_TTL_REFRESH` and `QSV_FP_REDIS_CONNSTR` environment variables.

## UTF-8 Encoding for Performance
[Rust strings are utf-8 encoded](https://doc.rust-lang.org/std/string/struct.String.html). As a result, qsv **REQUIRES** UTF-8 encoded files.

Still, users will attempt to use non UTF-8 encoded files, and for the most part, they will still work! This is because most qsv commands use [ByteRecords](https://docs.rs/csv/latest/csv/struct.ByteRecord.html), where qsv manipulates raw bytes and doesn't care about the encoding.

Where it does matter, qsv will attempt to convert the bytes to UTF-8. But instead of using [std::str::from_utf8](https://doc.rust-lang.org/stable/std/str/fn.from_utf8.html), it makes extensive use of [`simdutf8`](https://github.com/rusticstuff/simdutf8#simdutf8--high-speed-utf-8-validation) for [SIMD](https://en.wikipedia.org/wiki/Single_instruction,_multiple_data)-accelerated UTF-8 validation, which is up to 23x faster on x86-64 and 11x faster on aarch64 (Apple Silicon).

As UTF-8 is the de facto encoding standard, this shouldn't be a problem most of the time. However, should you need to process a CSV file with a different encoding, use the `input` command with the `--output` option first to "[loosely transcode](https://doc.rust-lang.org/std/string/struct.String.html#method.from_utf8_lossy)" it to UTF-8.

## Nightly Release Builds
Pre-built binaries compiled using Rust Nightly/Unstable are also [available for download](https://github.com/jqnatividad/qsv/releases/latest). These binaries are optimized for size and speed:

* compiled with the last known Rust nightly/unstable that passes all 1,000+ tests.
* stdlib is compiled from source, instead of using the pre-built stdlib. This ensures stdlib is compiled with all of qsv's release settings
  (link time optimization, opt-level, codegen-units, panic=abort, etc.), presenting more opportunities for Rust/LLVM to optimize the generated code.
  This is why we only have nightly release builds for select platforms (the platform of GitHub's action runners), as we need access to the "native hardware"
  and cannot cross-compile stdlib to other platforms.
* set `panic=abort` - removing panic-handling/formatting and backtrace code, making for smaller binaries.
* enables unstable/nightly features in the `rand`, `regex`, `hashbrown`, `parking_lot`, `pyo3` and `polars` crates, that unlock performance/SIMD features on those crates.

Despite the 'unstable' label, these binaries are actually quite stable, given how [Rust is made](https://doc.rust-lang.org/book/appendix-07-nightly-rust.html) and are really more about performance (that's why we can still compile with Rust stable). You only really loose the backtrace messages when qsv panics.

If you need to maximize performance - use the nightly builds. If you prefer a "safer", rock-solid experience, use the stable builds.

If you want to really squeeze every little bit of performance from qsv, build it locally like how the Nightly Release Builds are built, with the additional step of optimizing the build to your machine's CPU by setting `RUSTFLAGS='-C target-cpu=native'`.

Doing so will ensure CPU features are tailored to your hardware and you're using the latest Rust nightly.

For example, on Ubuntu 22.04 LTS Linux:

```bash
rustup default nightly-2023-03-14
rustup update
export RUSTFLAGS='-C target-cpu=native'

# to build qsv on nightly with all features. The binary will be in the target/release-nightly folder.
cargo build --profile release-nightly --bin qsv -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --features all_full,nightly --target x86_64-unknown-linux-gnu

# to build qsvlite
cargo build --profile release-nightly --bin qsvlite -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --features lite,nightly --target x86_64-unknown-linux-gnu

# to build qsvdp
cargo build --profile release-nightly --bin qsvdp -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --features datapusher_plus,nightly --target x86_64-unknown-linux-gnu
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
