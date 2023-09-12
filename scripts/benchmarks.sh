#!/bin/bash

# This script benchmarks Quicksilver (qsv) using a 520mb, 41 column, 1M row sample of
# NYC's 311 data. If it doesn't exist on your system, it will be downloaded for you.
#
# Make sure you're using a release-optimized `qsv`. 
# If you can't use the prebuilt binaries at https://github.com/jqnatividad/qsv/releases/latest,
# build it to have at least the apply, geocode, luau and polars features enabled:
# i.e. `cargo build --release --locked -F feature_capable,apply,geocode,luau,polars` or
# `cargo install --locked qsv -F feature_capable,apply,geocode,luau,polars`
#
# This shell script has been tested on Linux, macOS and Cygwin for Windows (https://www.cygwin.com/).
# It should work on other Unix-like systems, but will NOT run on native Windows.
# It requires hyperfine (https://github.com/sharkdp/hyperfine#hyperfine) to run the benchmarks.
# It also requires 7-Zip (https://www.7-zip.org/download.html) as we need the high compression
# ratio so we don't have to deal with git-lfs to host the large compressed file on GitHub.
# And of course, it dogfoods `qsv` as well to prepare the benchmark data, and to parse and format
# the benchmark results. :)

set -e

pat="$1"
echo "Setting up benchmarking environment..."

SECONDS=0
qsv_bin=qsv

# check if qsv is installed
if ! command -v "$qsv_bin" &> /dev/null
then
    echo "qsv could not be found"
    echo "Please install Quicksilver (qsv) from https://qsv.dathere.com"
    exit
fi

# set sevenz_bin  to "7z" on Windows/Linux and "7zz" on macOS
if [[ "$OSTYPE" == "darwin"* ]]; then
  sevenz_bin=7zz
else
  sevenz_bin=7z
fi

# check if 7z is installed
if ! command -v "$sevenz_bin" &> /dev/null
then
    echo "7z could not be found"
    echo "Please install 7-Zip v23.01 and above"
    exit
fi

# check if hyperfine is installed
if ! command -v hyperfine &> /dev/null
then
    echo "hyperfine could not be found"
    echo "Please install hyperfine v1.17.0 and above"
    exit
fi

datazip=/tmp/NYC_311_SR_2010-2020-sample-1M.7z
data=NYC_311_SR_2010-2020-sample-1M.csv
data_idx=NYC_311_SR_2010-2020-sample-1M.csv.idx
data_to_exclude=data_to_exclude.csv
data_unsorted=data_unsorted.csv
data_sorted=data_sorted.csv
searchset_patterns=searchset_patterns.txt
commboarddata=communityboards.csv
urltemplate="http://localhost:4000/v1/search?text={Street Name}, {City}"
jql='"features".[0]."properties"."label"'


if [ ! -r "$data" ]; then
  echo "Downloading benchmarking data..."
  curl -sS https://raw.githubusercontent.com/wiki/jqnatividad/qsv/files/NYC_311_SR_2010-2020-sample-1M.7z > "$datazip"
  "$sevenz_bin" e -y "$datazip"
fi

if [ ! -r "$commboarddata" ]; then
  echo "Downloading community board data..."
  curl -sS https://raw.githubusercontent.com/wiki/jqnatividad/qsv/files/communityboards.csv > "$commboarddata"
fi

if [ ! -r "$data_to_exclude" ]; then
  echo "Creating benchmark support data..."
  "$qsv_bin" sample --seed 42 1000 "$data" -o "$data_to_exclude"
  "$qsv_bin" sort --seed 42 --random  --faster "$data" -o "$data_unsorted"
  "$qsv_bin" sort "$data" -o "$data_sorted"
  printf "homeless\npark\nnoise\n" > "$searchset_patterns"
fi

commands_without_index=()
commands_without_index_name=()
commands_with_index=()
commands_with_index_name=()

function add_command {
  local dest_array="$1"
  shift
  local cmd="$@"
  
  if [[ "$dest_array" == "without_index" ]]; then
    commands_without_index+=("$cmd")
  else
    commands_with_index+=("$cmd")
  fi
}

function run {
  local index=
  while true; do
    case "$1" in
      --index)
        index="yes"
        shift
        ;;
      *)
        break
        ;;
    esac
  done

  local name="$1"
  shift

  if [ -z "$index" ]; then
    commands_without_index_name+=("$name")
    add_command "without_index" "$@"
  else
    commands_with_index_name+=("$name")
    add_command "with_index" "$@"
  fi
}

# ---------------------------------------
# Queue commands for benchmarking
# commands with an --index prefix will be benchmarked with an index

echo "Queueing commands for benchmarking..."
run apply_op_string "$qsv_bin apply operations lower Agency  $data"
run apply_op_similarity "$qsv_bin apply operations lower,simdln Agency --comparand brooklyn --new-column Agency_sim-brooklyn_score  $data"
run apply_op_eudex "$qsv_bin apply operations lower,eudex Agency --comparand Queens --new-column Agency_queens_soundex  $data" 
run apply_datefmt "$qsv_bin apply datefmt \"Created Date\"  $data"
run apply_emptyreplace "$qsv_bin" apply emptyreplace \"Bridge Highway Name\" --replacement Unspecified "$data"
run count "$qsv_bin" count "$data"
run --index count_index "$qsv_bin" count "$data"
run dedup "$qsv_bin" dedup "$data"
run enum "$qsv_bin" enum "$data"
run exclude "$qsv_bin" exclude \'Incident Zip\' "$data" \'Incident Zip\' "$data_to_exclude"
run --index exclude_index "$qsv_bin" exclude \'Incident Zip\' "$data" \'Incident Zip\' "$data_to_exclude"
run explode "$qsv_bin" explode City "-" "$data"
run fill "$qsv_bin" fill -v Unspecified \'Address Type\' "$data"
run fixlengths "$qsv_bin" fixlengths "$data"
run flatten "$qsv_bin" flatten "$data"
run flatten_condensed "$qsv_bin" flatten "$data" --condense 50
run fmt "$qsv_bin" fmt --crlf "$data"
run frequency "$qsv_bin" frequency "$data"
run --index frequency_index "$qsv_bin" frequency "$data"
run frequency_selregex "$qsv_bin" frequency -s /^R/ "$data"
run frequency_j1 "$qsv_bin" frequency -j 1 "$data"
run geocode_suggest "$qsv_bin" geocode suggest City --new-column geocoded_city "$data"
run geocode_reverse "$qsv_bin" geocode reverse Location --new-column geocoded_location "$data"
run index "$qsv_bin" index "$data"
run join "$qsv_bin" join \'Community Board\' "$data" community_board "$commboarddata"
run luau "$qsv_bin" luau map location_empty "tonumber\(Location\)==nil" "$data"
run partition "$qsv_bin" partition \'Community Board\' /tmp/partitioned "$data"
run pseudo "$qsv_bin" pseudo \'Unique Key\' "$data"
run rename "$qsv_bin" rename \'unique_key,created_date,closed_date,agency,agency_name,complaint_type,descriptor,loctype,zip,addr1,street,xstreet1,xstreet2,inter1,inter2,addrtype,city,landmark,facility_type,status,due_date,res_desc,res_act_date,comm_board,bbl,boro,xcoord,ycoord,opendata_type,parkname,parkboro,vehtype,taxi_boro,taxi_loc,bridge_hwy_name,bridge_hwy_dir,ramp,bridge_hwy_seg,lat,long,loc\' "$data"
run reverse "$qsv_bin" reverse "$data"
run sample_10 "$qsv_bin" sample 10 "$data"
run --index sample_10_index "$qsv_bin" sample 10 "$data"
run sample_1000 "$qsv_bin" sample 1000 "$data"
run --index sample_1000_index "$qsv_bin" sample 1000 "$data"
run sample_100000 "$qsv_bin" sample 100000 "$data"
run --index sample_100000_index "$qsv_bin" sample 100000 "$data"
run sample_100000_seeded "$qsv_bin" sample 100000 --seed 42 "$data"
run sample_100000_seeded_faster "$qsv_bin" sample 100000 --faster --seed 42 "$data"
run --index sample_100000_seeded_index "$qsv_bin" sample --seed 42 100000 "$data"
run --index sample_100000_seeded_index_faster "$qsv_bin" sample --faster --seed 42 100000 "$data"
run --index sample_25pct_index "$qsv_bin" sample 0.25 "$data"
run --index sample_25pct_seeded_index "$qsv_bin" sample 0.25 --seed 42 "$data"
run search "$qsv_bin" search -s \'Agency Name\' "'(?i)us'" "$data"
run search_unicode "$qsv_bin" search --unicode -s \'Agency Name\' "'(?i)us'" "$data"
run searchset "$qsv_bin" searchset "$searchset_patterns" "$data"
run searchset_unicode "$qsv_bin" searchset "$searchset_patterns" --unicode "$data"
run select "$qsv_bin" select \'Agency,Community Board\' "$data"
run select_regex "$qsv_bin" select /^L/ "$data"
run slice_one_middle "$qsv_bin" slice -i 500000 "$data"
run --index slice_one_middle_index "$qsv_bin" slice -i 500000 "$data"
run sort "$qsv_bin" sort -s \'Incident Zip\' "$data"
run sort_random_seeded "$qsv_bin" sort --random --seed 42 "$data"
run sortcheck_sorted "$qsv_bin" sortcheck "$data_sorted"
run sortcheck_unsorted "$qsv_bin" sortcheck "$data_unsorted"
run sortcheck_unsorted_all "$qsv_bin" sortcheck --all "$data_unsorted"
run split "$qsv_bin" split --size 50000 split_tempdir "$data"
run --index split_index "$qsv_bin" split --size 50000 split_tempdir "$data"
run --index split_index_j1 "$qsv_bin" split --size 50000 -j 1 split_tempdir "$data"
run stats "$qsv_bin" stats --force "$data"
run --index stats_index "$qsv_bin" stats --force "$data"
run --index stats_index_j1 "$qsv_bin" stats -j 1 --force "$data"
run stats_everything "$qsv_bin" stats "$data" --force --everything
run stats_everything_infer_dates "$qsv_bin" stats "$data" --force --everything --infer-dates
run stats_everything_j1 "$qsv_bin" stats "$data" --force --everything -j 1
run --index stats_everything_index "$qsv_bin" stats "$data" --force --everything
run --index stats_everything_infer_dates_index "$qsv_bin" stats "$data" --force --everything --infer-dates
run --index stats_everything_index_j1 "$qsv_bin" stats "$data" --force --everything -j 1
run table "$qsv_bin" table "$data"
run transpose "$qsv_bin" transpose "$data"
run extsort "$qsv_bin" extsort "$data_unsorted" test.csv
run schema "$qsv_bin" schema "$data"
run validate "$qsv_bin" validate "$data" "$schema"
run sql "$qsv_bin" sqlp  "$data" city.csv "'select * from _t_1 join _t_2 on _t_1.City = _t_2.City'"

# ---------------------------------------
# Prepare benchmark results directory

# Check if a results directory exists, if it doesn't create it
if [ ! -d "results" ]; then
  mkdir results
fi

# Init latest_results.csv. It stores the benchmark results for this run
rm -f results/latest_results.csv
echo "version,tstamp,name,mean,stddev,median,user,system,min,max" > results/latest_results.csv

# check if the file benchmark_results.csv exists, if it doesn't create it
# by copying the empty latest_results.csv
if [ ! -f "results/benchmark_results.csv" ]; then
  cp results/latest_results.csv results/benchmark_results.csv
fi

# get current version of qsv
version=$("$qsv_bin" --version | cut -d' ' -f2 | cut -d'-' -f1)

# get current time to the nearest hour
now=$(date +"%Y-%m-%d-%H")

# ---------------------------------------
# Run hyperfine to compile benchmark results. Append each individual result to the latest_results.csv
# by dogfooding qsv's cat, luau, select & sort commands.

# first, run benchmarking without an index
echo "Benchmarking WITHOUT INDEX..."
idx=0
for command_no_index in "${commands_without_index[@]}"; do
  rm -f "$data_idx"
  echo "${commands_without_index_name[$idx]}"
  hyperfine --warmup 2 -i -r 3 --export-csv results/hf_result.csv "$command_no_index"
  echo "version,tstamp,name" > results/results_work.csv
  echo "$version,$now,${commands_without_index_name[$idx]}" >> results/results_work.csv
  "$qsv_bin" select '!command' results/hf_result.csv -o results/hf_result_nocmd.csv
  "$qsv_bin" cat columns results/results_work.csv results/hf_result_nocmd.csv \
    -o results/entry.csv
  "$qsv_bin" cat rowskey results/latest_results.csv results/entry.csv \
    -o results/results_work.csv
  mv results/results_work.csv results/latest_results.csv
  ((idx++))
done

# ---------------------------------------
# then, run benchmarks with an index
echo "Benchmarking WITH INDEX..."
rm -f "$data_idx"
"$qsv_bin" index "$data"

idx=0
for command_with_index in "${commands_with_index[@]}"; do
  echo "${commands_with_index_name[$idx]}"
  hyperfine --warmup 2 -i -r 3 --export-csv results/hf_result.csv "$command_with_index"
  echo "version,tstamp,name" > results/results_work.csv
  echo "$version,$now,${commands_with_index_name[$idx]}" >> results/results_work.csv
  "$qsv_bin" select '!command' results/hf_result.csv -o results/hf_result_nocmd.csv
  "$qsv_bin" cat columns results/results_work.csv results/hf_result_nocmd.csv \
    -o results/entry.csv
  "$qsv_bin" cat rowskey results/latest_results.csv results/entry.csv \
    -o results/results_work.csv
  mv results/results_work.csv results/latest_results.csv
  ((idx++))
done

# ---------------------------------------
# Finalize benchmark results. Sort the latest results by version, tstamp & name.
# compute and add records per second for each benchmark using qsv luau map,
# rounding to 3 decimal places; then append the latest results to benchmark_results.csv -
# which is a historical archive. Finally, clean up temporary files

# sort the benchmark results by version, tstamp & name
"$qsv_bin" sort --select version,tstamp,name results/latest_results.csv \
   -o results/results_work.csv

# compute records per second for each benchmark using qsv, rounding to 3 decimal places
"$qsv_bin" luau map recs_per_sec \
   'recs_per_sec=(1000000.0 / mean); return tonumber(string.format("%.3f",recs_per_sec))' \
   results/results_work.csv -o results/latest_results.csv

# cat the final results to results/bechmark_results.csv
"$qsv_bin" cat rowskey results/latest_results.csv results/benchmark_results.csv \
  -o results/results_work.csv
mv results/results_work.csv results/benchmark_results.csv

# clean up
rm -f results/hf_result.csv
rm -f results/hf_result_nocmd.csv
rm -f results/results_work.csv
rm -f results/entry.csv
rm -f results/latest_results.csv
rm -r -f split_tempdir

echo "Benchmark results completed. Elapsed time: $SECONDS seconds."
