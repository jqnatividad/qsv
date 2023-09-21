> :exclamation: **WARNING** :exclamation: These benchmarks are stale and are only
being kept around for posterity.<br>
> The latest benchmarks can be found at https://qsv.dathere.com/benchmarks.

These are some very basic and unscientific benchmarks of various commands
provided by the latest release of `qsv`. Please see below for more information.

These benchmarks were compiled against a 1M row, 512 mb, 41 column [sample of NYC's 311 data]
(https://raw.githubusercontent.com/wiki/jqnatividad/qsv/files/NYC_311_SR_2010-2020-sample-1M.7z)
on a Virtualbox v6.1 Windows 11 v21H2 VM with an AMD Ryzen 7 4800H,
32GB memory and a 1 TB SSD (VM configured with Ubuntu 20.04 LTS assigned 8 CPUs
and 12 GB of memory).

### qsv 0.20.0
```
BENCHMARK                  TIME_SECS  MB_PER_SEC  RECS_PER_SEC
apply_op_string            0.94       48.41       1,063,829.78
apply_op_similarity        1.38       32.98       724,637.68  
apply_op_soundex           1.51       30.14       662,251.65  
apply_datefmt              0.76       61.41       131,578.94  
apply_emptyreplace         0.34       137.29      294,117.64  
apply_geocode              40.45      1.15        2,472.18    
count                      0.09       505.70      11,111,111.11
count_index                0.01       4551.36     100,000,000.00
dedup                      3.20       14.22       312,500.00  
enum                       0.31       146.81      3,225,806.45
exclude                    0.27       168.56      3,703,703.70
exclude_index              0.26       175.05      3,846,153.84
explode                    0.68       66.93       1,470,588.23
fill                       0.71       64.10       1,408,450.70
fixlengths                 0.40       113.78      2,500,000.00
flatten                    4.42       10.29       226,244.34  
flatten_condensed          4.50       10.11       222,222.22  
fmt                        0.23       197.88      4,347,826.08
frequency                  2.09       21.77       478,468.89  
frequency_index            1.43       31.82       699,300.69  
frequency_selregex         0.38       119.77      2,631,578.94
frequency_j1               2.10       21.67       476,190.47  
index                      0.09       505.70      11,111,111.11
join                       0.89       51.13       1,123,595.50
lua                        5.37       8.47        186,219.73  
partition                  0.36       126.42      2,777,777.77
rename                     0.26       175.05      3,846,153.84
reverse                    0.35       130.03      2,857,142.85
sample_10                  0.17       267.72      5,882,352.94
sample_10_index            0.01       4551.36     100,000,000.00
sample_1000                0.17       267.72      5,882,352.94
sample_1000_index          0.02       2275.68     50,000,000.00
sample_100000              0.30       151.71      3,333,333.33
sample_100000_index        0.30       151.71      3,333,333.33
sample_25pct_index         0.43       105.84      2,325,581.39
scramble_index             2.80       16.25       357,142.85  
search                     0.13       350.10      7,692,307.69
searchset                  0.55       82.75       1,818,181.81
select                     0.13       350.10      7,692,307.69
select_regex               0.14       325.09      7,142,857.14
slice_one_middle           0.08       568.92      12,500,000.00
slice_one_middle_index     0.01       4551.36     100,000,000.00
sort                       2.23       20.40       448,430.49  
split                      0.26       175.05      3,846,153.84
split_index                0.06       758.56      16,666,666.66
split_index_j1             0.34       133.86      2,941,176.47
stats                      2.70       16.85       370,370.37  
stats_index                2.73       16.67       366,300.36  
stats_index_j1             2.72       16.73       367,647.05  
stats_everything           4.32       10.53       231,481.48  
stats_everything_j1        8.97       5.07        111,482.72  
stats_everything_index     4.14       10.99       241,545.89  
stats_everything_index_j1  8.86       5.13        112,866.81  
table                      2.17       20.97       460,829.49  
transpose                  0.53       85.87       1,886,792.45
```

For reference, we also benchmark the last release of xsv.
### xsv 0.13.0 (compiled and installed locally using `cargo install xsv`)
```
BENCHMARK                  TIME_SECS  MB_PER_SEC  RECS_PER_SEC
apply_op_string            NA         NA          NA
apply_op_similarity        NA         NA          NA
apply_op_soundex           NA         NA          NA
apply_datefmt              NA         NA          NA
apply_emptyreplace         NA         NA          NA
apply_geocode              NA         NA          NA
count                      0.10       455.13      10,000,000.00
count_index                0.01       4551.36     100,000,000.00
dedup                      NA         NA          NA
enum                       NA         NA          NA
exclude                    NA         NA          NA
exclude_index              NA         NA          NA
explode                    NA         NA          NA
fill                       NA         NA          NA
fixlengths                 0.52       87.52       1,923,076.92
flatten                    6.07       7.49        164,744.64  
flatten_condensed          6.19       7.35        161,550.88  
fmt                        0.25       182.05      4,000,000.00
frequency                  3.21       14.17       311,526.47  
frequency_index            2.02       22.53       495,049.50  
frequency_selregex         0.01       4551.36     100,000,000.00
frequency_j1               3.07       14.82       325,732.89  
index                      0.11       413.76      9,090,909.09
join                       1.32       34.48       757,575.75  
lua                        NA         NA          NA
partition                  0.53       85.87       1,886,792.45
rename                     NA         NA          NA
reverse                    NA         NA          NA
sample_10                  0.27       168.56      3,703,703.70
sample_10_index            0.09       505.70      11,111,111.11
sample_1000                0.27       168.56      3,703,703.70
sample_1000_index          0.08       568.92      12,500,000.00
sample_100000              0.37       123.00      2,702,702.70
sample_100000_index        0.39       116.70      2,564,102.56
sample_25pct_index         NA         NA          NA
scramble_index             NA         NA          NA
search                     0.19       239.54      5,263,157.89
searchset                  NA         NA          NA
select                     0.20       227.56      5,000,000.00
select_regex               NA         NA          NA
slice_one_middle           0.11       413.76      9,090,909.09
slice_one_middle_index     0.01       4551.36     100,000,000.00
sort                       2.15       21.16       465,116.27  
split                      0.30       151.71      3,333,333.33
split_index                0.08       568.92      12,500,000.00
split_index_j1             0.44       103.44      2,272,727.27
stats                      1.45       31.38       689,655.17  
stats_index                0.24       189.64      4,166,666.66
stats_everything           3.50       13.00       285,714.28  
stats_everything_j1        9.91       4.59        100,908.17  
stats_everything_index     2.85       15.96       350,877.19  
stats_everything_index_j1  9.95       4.57        100,502.51  
table                      2.96       15.37       337,837.83  
transpose                  NA         NA          NA
```

## Details

The primary purpose of these benchmarks is to provide a rough ballpark estimate of how
fast each command is, to catch significant performance regressions, and to help you
[fine-tune qsv's performance](https://github.com/jqnatividad/qsv#performance-tuning) in your environment.

The `count` command can be viewed as a sort of baseline of the fastest possible
command that parses every record in CSV data.

Benchmarks with the `_index` suffix are run with indexing enabled. The `_j1` suffix are run with 
parallelization disabled - i.e. `--jobs 1`.

Note that the `qsv stats` command is slower than `xsv stats` primarily because qsv computes
additional stats (specifically - lower_fence, q1, q2_median, q3, iqr, upper_fence; skew, modes & nullcount)
and does date type detection.
