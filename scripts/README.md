# SCRIPTS

This directory contains various scripts for the project.

## benchmarks.sh - configurable benchmarking script
This script runs the benchmarks for the project. It takes one argument:

Usage: ./benchmarks.sh <argument>
  where <argument> is a substring pattern of the benchmark name.
  e.g. ./benchmarks.sh sort - will run benchmarks with "sort" in the benchmark name
  if <argument> is omitted, all benchmarks are executed.

  if <argument> is "reset", the benchmark data will be downloaded and prepared again.
   though the results/benchmark_results.csv and resutls/run_info_history.tsv historical
   archives will be preserved.
  if <argument> is "clean", temporary files will be deleted.
  if <argument> is "setup", setup and install all the required tools.
  if <argument> is "help", help text is displayed.

This script benchmarks Quicksilver (qsv) using a 520mb, 41 column, 1M row sample of
NYC's 311 data. If it doesn't exist on your system, it will be downloaded for you.

Though this script was primarily created to maintain the Benchmark page on the qsv site,
it was also designed to be a useful tool for users to benchmark qsv on their own systems,
so it be can run on hardware and workloads that reflect your requirements/environment.

See [benchmarks.sh](benchmarks.sh) for more details.

## misc/docopt-wonders.bash - optional qsv tab completion support
qsv's command-line options are quite extensive. Thankfully, since it uses [docopt](http://docopt.org/) for CLI processing,
we can take advantage of [docopt.rs' tab completion support](https://github.com/docopt/docopt.rs#tab-completion-support) to make it
easier to use qsv at the command-line (currently, only bash shell is supported):

```bash
# install docopt-wordlist
cargo install docopt

# IMPORTANT: run these commands from the root directory of your qsv git repository
# to setup bash qsv tab completion
echo "DOCOPT_WORDLIST_BIN=\"$(which docopt-wordlist)"\" >> $HOME/.bash_completion
echo "source \"$(pwd)/scripts/docopt-wordlist.bash\"" >> $HOME/.bash_completion
echo "complete -F _docopt_wordlist_commands qsv" >> $HOME/.bash_completion
```
