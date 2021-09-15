These are some very basic and unscientific benchmarks of various commands
provided by `qsv`. Please see below for more information.

These benchmarks were run with
[worldcitiespop_mil.csv](https://burntsushi.net/stuff/worldcitiespop_mil.csv),
which is a random 1,000,000 row subset of the world city population dataset
from the [Data Science Toolkit](https://github.com/petewarden/dstkdata).

These benchmarks were run on Virtualbox VM on Windows 11 with a Ryzen 7 4800H and 32GB memory 
(VM configured with Ubuntu 20.04 LTS assigned 8 CPUs and 12 GB of memory)


### xsv 0.13.0 (compiled and installed locally using `cargo install xsv`)
```
count				0.11 seconds	413.76 MB/sec
flatten				6.46 seconds	7.04 MB/sec
flatten_condensed		6.52 seconds	6.98 MB/sec
frequency			3.37 seconds	13.50 MB/sec
index				0.12 seconds	379.28 MB/sec
sample_10			0.28 seconds	162.54 MB/sec
sample_1000			0.28 seconds	162.54 MB/sec
sample_100000			0.38 seconds	119.77 MB/sec
search				0.20 seconds	227.56 MB/sec
select				0.15 seconds	303.42 MB/sec
sort				2.51 seconds	18.13 MB/sec
slice_one_middle		0.12 seconds	379.28 MB/sec
slice_one_middle_index		0.01 seconds	4551.36 MB/sec
stats				1.53 seconds	29.74 MB/sec
stats_index			0.21 seconds	216.73 MB/sec
stats_everything		3.60 seconds	12.64 MB/sec
stats_everything_index		3.22 seconds	14.13 MB/sec
```

### qsv 0.14.0 (compiled locally)
```
count				0.11 seconds	413.76 MB/sec
fill				1.13 seconds	40.27 MB/sec
flatten				5.48 seconds	8.30 MB/sec
flatten_condensed		5.52 seconds	8.24 MB/sec
frequency			3.05 seconds	14.92 MB/sec
frequency_selregex		0.48 seconds	94.82 MB/sec
index				0.11 seconds	413.76 MB/sec
rename				0.36 seconds	126.42 MB/sec
sample_10			0.23 seconds	197.88 MB/sec
sample_1000			0.24 seconds	189.64 MB/sec
sample_100000			0.33 seconds	137.92 MB/sec
sample_25pct_index		0.42 seconds	108.36 MB/sec
search				0.17 seconds	267.72 MB/sec
select				0.15 seconds	303.42 MB/sec
select_regex			0.18 seconds	252.85 MB/sec
sort				2.29 seconds	19.87 MB/sec
slice_one_middle		0.11 seconds	413.76 MB/sec
slice_one_middle_index		0.01 seconds	4551.36 MB/sec
stats				1.42 seconds	32.05 MB/sec
stats_index			0.19 seconds	239.54 MB/sec
stats_everything		3.37 seconds	13.50 MB/sec
stats_everything_index		2.60 seconds	17.50 MB/sec
transpose			0.72 seconds	63.21 MB/sec
```

### qsv 0.14.1 (compiled locally)
```
count				0.09 seconds	505.70 MB/sec
fill				0.75 seconds	60.68 MB/sec
flatten				4.19 seconds	10.86 MB/sec
flatten_condensed		4.29 seconds	10.60 MB/sec
frequency			2.30 seconds	19.78 MB/sec
frequency_selregex		0.39 seconds	116.70 MB/sec
index				0.09 seconds	505.70 MB/sec
rename				0.27 seconds	168.56 MB/sec
sample_10			0.17 seconds	267.72 MB/sec
sample_1000			0.18 seconds	252.85 MB/sec
sample_100000			0.33 seconds	137.92 MB/sec
sample_25pct_index		0.48 seconds	94.82 MB/sec
search				0.13 seconds	350.10 MB/sec
select				0.11 seconds	413.76 MB/sec
select_regex			0.14 seconds	325.09 MB/sec
sort				2.93 seconds	15.53 MB/sec
slice_one_middle		0.08 seconds	568.92 MB/sec
slice_one_middle_index		0.01 seconds	4551.36 MB/sec
stats				0.99 seconds	45.97 MB/sec
stats_index			0.14 seconds	325.09 MB/sec
stats_everything		2.68 seconds	16.98 MB/sec
stats_everything_index		1.61 seconds	28.26 MB/sec
transpose			0.56 seconds	81.27 MB/sec
```
## Details

The purpose of these benchmarks is to provide a rough ballpark estimate of how
fast each command is. My hope is that they can also catch significant
performance regressions.

The `count` command can be viewed as a sort of baseline of the fastest possible
command that parses every record in CSV data.

The benchmarks that end with `_index` are run with indexing enabled.
