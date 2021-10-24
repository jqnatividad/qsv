These are some very basic and unscientific benchmarks of various commands
provided by the latest release of `qsv`. Please see below for more information.

These benchmarks were run with
[worldcitiespop_mil.csv](https://raw.githubusercontent.com/wiki/jqnatividad/qsv/files/worldcitiespop_mil.zip),
which is a 48mb, random 1,000,000 row subset of the world city population dataset
from the [Data Science Toolkit](https://github.com/petewarden/dstkdata).

These benchmarks were run on Virtualbox VM on Windows 11 with a Ryzen 7 4800H,
32GB memory and a 1 TB SSD (VM configured with Ubuntu 20.04 LTS assigned 8 CPUs
and 12 GB of memory).

### qsv 0.19.0
```
BENCHMARK                TIME_SECS  MB_PER_SEC 
count                    0.15       303.42
count_index              0.04       1137.84
dedup                    3.38       13.46
enum                     0.27       168.56
exclude                  0.26       175.05
exclude_index            0.40       113.78
explode                  0.95       47.90
fill                     0.58       78.47
fixlengths               0.52       87.52
flatten                  3.75       12.13
flatten_condensed        3.95       11.52
fmt                      0.21       216.73
frequency                2.54       17.91
frequency_index          2.77       16.43
frequency_selregex       0.66       68.96
index                    0.32       142.23
join                     1.34       33.96
lua                      4.19       10.86
partition                0.81       56.18
rename                   0.24       189.64
reverse                  0.33       137.92
sample_10                0.19       239.54
sample_10_index          0.05       910.27
sample_1000              0.30       151.71
sample_1000_index        0.04       1137.84
sample_100000            0.52       87.52
sample_100000_index      0.56       81.27
sample_25pct_index       0.64       71.11
scramble_index           2.14       21.26
search                   0.18       252.85
searchset                0.79       57.61
select                   0.17       267.72
select_regex             0.16       284.46
slice_one_middle         0.10       455.13
slice_one_middle_index   0.02       2275.68
sort                     1.67       27.25
split                    1.09       41.75
split_index              1.32       34.48
stats                    2.78       16.37
stats_index              5.33       8.53
stats_everything         4.56       9.98
stats_everything_index   7.24       6.28
table                    2.21       20.59
transpose                0.54       84.28
```

For reference, we also benchmark the last release of xsv.
### xsv 0.13.0 (compiled and installed locally using `cargo install xsv`)
```
BENCHMARK                TIME_SECS  MB_PER_SEC 
count                    0.24       189.64
count_index              0.03       1517.12
dedup                    NA         NA
enum                     NA         NA
exclude                  NA         NA
exclude_index            NA         NA
explode                  NA         NA
fill                     NA         NA
fixlengths               0.58       78.47
flatten                  4.32       10.53
flatten_condensed        4.45       10.22
fmt                      0.20       227.56
frequency                3.60       12.64
frequency_index          7.60       5.98
frequency_selregex       0.02       2275.68
index                    0.38       119.77
join                     1.37       33.22
luatest                  NA         NA
partition                NA         NA
rename                   NA         NA
reverse                  NA         NA
sample_10                0.23       197.88
sample_10_index          0.07       650.19
sample_1000              0.47       96.83
sample_1000_index        0.14       325.09
sample_100000            0.91       50.01
sample_100000_index      0.68       66.93
sample_25pct_index       NA         NA
scramble_index           NA         NA
search                   0.37       123.00
searchset                NA         NA
select                   0.17       267.72
select_regex             NA         NA
slice_one_middle         0.11       413.76
slice_one_middle_index   0.01       4551.36
sort                     1.89       24.08
split                    1.07       42.53
split_index              1.26       36.12
stats                    1.40       32.50
stats_index              0.61       74.61
stats_everything         4.76       9.56
stats_everything_index   9.73       4.67
table                    2.40       18.96
transpose                NA         NA
```

## Details

The purpose of these benchmarks is to provide a rough ballpark estimate of how
fast each command is. My hope is that they can also catch significant
performance regressions.

The `count` command can be viewed as a sort of baseline of the fastest possible
command that parses every record in CSV data.

The benchmarks that end with `_index` are run with indexing enabled.

Note that the `qsv stats` command is slower than `xsv` primarily because qsv computes
more stats and does date type detection.
