These are some very basic and unscientific benchmarks of various commands
provided by `qsv`. Please see below for more information.

These benchmarks were run with
[worldcitiespop_mil.csv](https://burntsushi.net/stuff/worldcitiespop_mil.csv),
which is a random 1,000,000 row subset of the world city population dataset
from the [Data Science Toolkit](https://github.com/petewarden/dstkdata).

These benchmarks were run on Virtualbox VM on Windows 11 with a Ryzen 7 4800H and 32GB memory 
(VM configured with Ubuntu 20.04 LTS assigned 8 CPUs and 12 GB of memory)

```
count					0.11 seconds	413.76 MB/sec
fill					1.13 seconds	40.27 MB/sec
flatten					5.48 seconds	8.30 MB/sec
flatten_condensed		5.52 seconds	8.24 MB/sec
frequency				3.05 seconds	14.92 MB/sec
frequency_selregex		0.48 seconds	94.82 MB/sec
index					0.11 seconds	413.76 MB/sec
rename					0.36 seconds	126.42 MB/sec
sample_10				0.23 seconds	197.88 MB/sec
sample_1000				0.24 seconds	189.64 MB/sec
sample_100000			0.33 seconds	137.92 MB/sec
sample_25pct_index		0.42 seconds	108.36 MB/sec
search					0.17 seconds	267.72 MB/sec
select					0.15 seconds	303.42 MB/sec
select_regex			0.18 seconds	252.85 MB/sec
sort					2.29 seconds	19.87 MB/sec
slice_one_middle		0.11 seconds	413.76 MB/sec
slice_one_middle_index	0.01 seconds	4551.36 MB/sec
stats					1.42 seconds	32.05 MB/sec
stats_index				0.19 seconds	239.54 MB/sec
stats_everything		3.37 seconds	13.50 MB/sec
stats_everything_index	2.60 seconds	17.50 MB/sec
transpose				0.72 seconds	63.21 MB/sec
```
### Details

The purpose of these benchmarks is to provide a rough ballpark estimate of how
fast each command is. My hope is that they can also catch significant
performance regressions.

The `count` command can be viewed as a sort of baseline of the fastest possible
command that parses every record in CSV data.

The benchmarks that end with `_index` are run with indexing enabled.
