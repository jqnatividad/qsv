These benchmarks are compiled on an Apple Mac Mini 2023 model with an M2 Pro chip
with 12 CPU cores & 32GB of RAM; a 1TB SSD primary drive & a 1TB Samsung SSD 970 Evo plus
external drive running macOS Sonoma 14.0.


It uses the prebuilt, CPU optimized qsv binary variant in aarch64-apple-darwin.zip
found at `https://github.com/jqnatividad/qsv/releases/latest`.


The benchmarks were performed on a million row, 41 column, 520 MB sample of NYC's 311 data.
`https://raw.githubusercontent.com/wiki/jqnatividad/qsv/files/NYC_311_SR_2010-2020-sample-1M.7z`

Each benchmark is executed five times using hyperfine. Two warmup runs followed by three benchmark runs.

Records per second is calculated by dividing the number of records (1M) by the mean of the three benchmark runs. All other measurements are in seconds.
