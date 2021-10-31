These are some very basic and unscientific benchmarks of various commands
provided by the latest release of `qsv`. Please see below for more information.

These benchmarks were primaly run with
[worldcitiespop_mil.csv](https://raw.githubusercontent.com/wiki/jqnatividad/qsv/files/worldcitiespop_mil.zip),
which is a 48mb, random 1,000,000 row subset of the world city population dataset
from the [Data Science Toolkit](https://github.com/petewarden/dstkdata). Select benchmarks
(`apply_datefmt`, `apply_empty_replace` and `apply_geocode`) were run with a 47mb, 100,000 row sample of NYC's 311 data
([nyc-311-sample-100k.zip](https://raw.githubusercontent.com/wiki/jqnatividad/qsv/files/nyc-311-sample-100k.zip)),
as the worldcitiespop.csv file didn't have dates, empty fields, nor lat/long coordinates in that order.

These benchmarks were run on Virtualbox VM on Windows 11 with a Ryzen 7 4800H,
32GB memory and a 1 TB SSD (VM configured with Ubuntu 20.04 LTS assigned 8 CPUs
and 12 GB of memory).

### qsv 0.20.0
```
BENCHMARK                TIME_SECS  MB_PER_SEC  RECS_PER_SEC
apply_op_string          0.95       47.90       1,052,631.57
apply_op_similarity      1.42       32.05       704,225.35
apply_op_soundex         1.60       28.44       625,000.00 
apply_datefmt            0.83       56.23       120,481.92  
apply_emptyreplace       0.42       111.13      238,095.23  
apply_geocode            51.61      0.90        1,937.60    
count                    0.09       505.70      11,111,111.11
count_index              0.01       4551.36     100,000,000.00
dedup                    3.46       13.15       289,017.34  
enum                     0.32       142.23      3,125,000.00
exclude                  0.28       162.54      3,571,428.57
exclude_index            0.28       162.54      3,571,428.57
explode                  0.71       64.10       1,408,450.70
fill                     0.76       59.88       1,315,789.47
fixlengths               0.42       108.36      2,380,952.38
flatten                  4.55       10.00       219,780.21  
flatten_condensed        4.61       9.87        216,919.73  
fmt                      0.23       197.88      4,347,826.08
frequency                2.26       20.13       442,477.87  
frequency_index          1.52       29.94       657,894.73  
frequency_selregex       0.39       116.70      2,564,102.56
index                    0.09       505.70      11,111,111.11
join                     0.96       47.41       1,041,666.66
lua                      5.49       8.29        182,149.36  
partition                0.39       116.70      2,564,102.56
rename                   0.29       156.94      3,448,275.86
reverse                  0.38       119.77      2,631,578.94
sample_10                0.17       267.72      5,882,352.94
sample_10_index          0.02       2275.68     50,000,000.00
sample_1000              0.17       267.72      5,882,352.94
sample_1000_index        0.02       2275.68     50,000,000.00
sample_100000            0.34       133.86      2,941,176.47
sample_100000_index      0.36       126.42      2,777,777.77
sample_25pct_index       0.50       91.02       2,000,000.00
scramble_index           3.31       13.75       302,114.80  
search                   0.13       350.10      7,692,307.69
searchset                0.55       82.75       1,818,181.81
select                   0.15       303.42      6,666,666.66
select_regex             0.14       325.09      7,142,857.14
slice_one_middle         0.08       568.92      12,500,000.00
slice_one_middle_index   0.01       4551.36     100,000,000.00
sort                     2.40       18.96       416,666.66  
split                    0.29       156.94      3,448,275.86
split_index              0.07       650.19      14,285,714.28
stats                    2.79       16.31       358,422.93  
stats_index              2.76       16.49       362,318.84  
stats_everything         4.84       9.40        206,611.57  
stats_everything_index   4.25       10.70       235,294.11  
table                    2.28       19.96       438,596.49  
transpose                0.59       77.14       1,694,915.25
```

For reference, we also benchmark the last release of xsv.
### xsv 0.13.0 (compiled and installed locally using `cargo install xsv`)
```
BENCHMARK                TIME_SECS  MB_PER_SEC  RECS_PER_SEC
apply_op_string          NA         NA          NA
apply_op_similarity      NA         NA          NA
apply_op_soundex         NA         NA          NA
apply_datefmt            NA         NA          NA
apply_emptyreplace       NA         NA          NA
apply_geocode            NA         NA          NA
count                    0.10       455.13      10,000,000.00
count_index              0.01       4551.36     100,000,000.00
dedup                    NA         NA          NA
enum                     NA         NA          NA
exclude                  NA         NA          NA
exclude_index            NA         NA          NA
explode                  NA         NA          NA
fill                     NA         NA          NA
fixlengths               0.52       87.52       1,923,076.92
flatten                  6.07       7.49        164,744.64  
flatten_condensed        6.19       7.35        161,550.88  
fmt                      0.25       182.05      4,000,000.00
frequency                3.21       14.17       311,526.47  
frequency_index          2.02       22.53       495,049.50  
frequency_selregex       0.01       4551.36     100,000,000.00
index                    0.11       413.76      9,090,909.09
join                     1.32       34.48       757,575.75  
lua                      NA         NA          NA
partition                0.53       85.87       1,886,792.45
rename                   NA         NA          NA
reverse                  NA         NA          NA
sample_10                0.27       168.56      3,703,703.70
sample_10_index          0.09       505.70      11,111,111.11
sample_1000              0.27       168.56      3,703,703.70
sample_1000_index        0.08       568.92      12,500,000.00
sample_100000            0.37       123.00      2,702,702.70
sample_100000_index      0.39       116.70      2,564,102.56
sample_25pct_index       NA         NA          NA
scramble_index           NA         NA          NA
search                   0.19       239.54      5,263,157.89
searchset                NA         NA          NA
select                   0.20       227.56      5,000,000.00
select_regex             NA         NA          NA
slice_one_middle         0.11       413.76      9,090,909.09
slice_one_middle_index   0.01       4551.36     100,000,000.00
sort                     2.15       21.16       465,116.27  
split                    0.30       151.71      3,333,333.33
split_index              0.08       568.92      12,500,000.00
stats                    1.45       31.38       689,655.17  
stats_index              0.24       189.64      4,166,666.66
stats_everything         3.50       13.00       285,714.28  
stats_everything_index   2.85       15.96       350,877.19  
table                    2.96       15.37       337,837.83  
transpose                NA         NA          NA
```

## Details

The primary purpose of these benchmarks is to provide a rough ballpark estimate of how
fast each command is, to catch significant performance regressions, and to help you
[fine-tune qsv's performance](https://github.com/jqnatividad/qsv#performance-tuning) in your environment.

The `count` command can be viewed as a sort of baseline of the fastest possible
command that parses every record in CSV data.

The benchmarks that end with `_index` are run with indexing enabled.

Note that the `qsv stats` command is slower than `xsv stats` primarily because qsv computes
more stats and does date type detection.
