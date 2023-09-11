#!/bin/bash

# This script benchmarks with Quicksilver (qsv) using a 520mb, 41 column, 1M row sample of
# NYC's 311 data. If it doesn't exist on your system, it will be downloaded for you.
#
# Make sure you're using a release-optimized `qsv`. 
# If you can't use the prebuilt binaries at https://github.com/jqnatividad/qsv/releases/latest.
# Build it to have at least the apply, geocode, luau and polars features:
# e.g. `cargo build --release --locked -F feature_capable,apply,geocode,luau,polars` or
# `cargo install --locked qsv -F feature_capable,apply,geocode,luau,polars`
#
# This shell script has been tested on Linux, macOS and Cygwin for Windows.
# It requires hyperfine (https://github.com/sharkdp/hyperfine#hyperfine) to run the benchmarks.
# It also requires 7-Zip (https://www.7-zip.org/download.html) as we need the high compression
# ratio so we don't have to deal with git-lfs to host the large compressed file on GitHub.
# It also dogfoods `qsv` to parse and format the benchmark results. :)

set -e

pat="$1"
echo "Setting up benchmarking environment..."

SECONDS=0
bin_name=qsv
# set sevenz_bin_name  to "7z" on Windows/Linux and "7zz" on macOS
if [[ "$OSTYPE" == "darwin"* ]]; then
  sevenz_bin_name=7zz
else
  sevenz_bin_name=7z
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
  "$sevenz_bin_name" e -y "$datazip"
fi

if [ ! -r "$commboarddata" ]; then
  echo "Downloading community board data..."
  curl -sS https://raw.githubusercontent.com/wiki/jqnatividad/qsv/files/communityboards.csv > "$commboarddata"
fi

if [ ! -r "$data_to_exclude" ]; then
  echo "Creating benchmark support data..."
  "$bin_name" sample --seed 42 1000 "$data" -o "$data_to_exclude"
  "$bin_name" sort --seed 42 --random --faster "$data" -o "$data_unsorted"
  "$bin_name" sort "$data" -o "$data_sorted"
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
run apply_op_string "$bin_name apply operations lower Agency  $data"
run apply_op_similarity "$bin_name apply operations lower,simdln Agency --comparand brooklyn --new-column Agency_sim-brooklyn_score  $data"
run apply_op_eudex "$bin_name apply operations lower,eudex Agency --comparand Queens --new-column Agency_queens_soundex  $data" 
run apply_datefmt "$bin_name apply datefmt \"Created Date\"  $data"
run apply_emptyreplace "$bin_name" apply emptyreplace \"Bridge Highway Name\" --replacement Unspecified "$data"
run count "$bin_name" count "$data"
run --index count_index "$bin_name" count "$data"
run dedup "$bin_name" dedup "$data"
run enum "$bin_name" enum "$data"
run exclude "$bin_name" exclude \'Incident Zip\' "$data" \'Incident Zip\' "$data_to_exclude"
run --index exclude_index "$bin_name" exclude \'Incident Zip\' "$data" \'Incident Zip\' "$data_to_exclude"
run explode "$bin_name" explode City "-" "$data"
run fill "$bin_name" fill -v Unspecified \'Address Type\' "$data"
run fixlengths "$bin_name" fixlengths "$data"
run flatten "$bin_name" flatten "$data"
run flatten_condensed "$bin_name" flatten "$data" --condense 50
run fmt "$bin_name" fmt --crlf "$data"
run frequency "$bin_name" frequency "$data"
run --index frequency_index "$bin_name" frequency "$data"
run frequency_selregex "$bin_name" frequency -s /^R/ "$data"
run frequency_j1 "$bin_name" frequency -j 1 "$data"
run geocode_suggest "$bin_name" geocode suggest City --new-column geocoded_city "$data"
run geocode_reverse "$bin_name" geocode reverse Location --new-column geocoded_location "$data"
run index "$bin_name" index "$data"
run join "$bin_name" join \'Community Board\' "$data" community_board "$commboarddata"
run luau "$bin_name" luau map location_empty "tonumber\(Location\)==nil" "$data"
run partition "$bin_name" partition \'Community Board\' /tmp/partitioned "$data"
run pseudo "$bin_name" pseudo \'Unique Key\' "$data"
run rename "$bin_name" rename \'unique_key,created_date,closed_date,agency,agency_name,complaint_type,descriptor,loctype,zip,addr1,street,xstreet1,xstreet2,inter1,inter2,addrtype,city,landmark,facility_type,status,due_date,res_desc,res_act_date,comm_board,bbl,boro,xcoord,ycoord,opendata_type,parkname,parkboro,vehtype,taxi_boro,taxi_loc,bridge_hwy_name,bridge_hwy_dir,ramp,bridge_hwy_seg,lat,long,loc\' "$data"
run reverse "$bin_name" reverse "$data"
run sample_10 "$bin_name" sample 10 "$data"
run --index sample_10_index "$bin_name" sample 10 "$data"
run sample_1000 "$bin_name" sample 1000 "$data"
run --index sample_1000_index "$bin_name" sample 1000 "$data"
run sample_100000 "$bin_name" sample 100000 "$data"
run --index sample_100000_index "$bin_name" sample 100000 "$data"
run sample_100000_seeded "$bin_name" sample 100000 --seed 42 "$data"
run sample_100000_seeded_faster "$bin_name" sample 100000 --faster --seed 42 "$data"
run --index sample_100000_seeded_index "$bin_name" sample --seed 42 100000 "$data"
run --index sample_100000_seeded_index_faster "$bin_name" sample --faster --seed 42 100000 "$data"
run --index sample_25pct_index "$bin_name" sample 0.25 "$data"
run --index sample_25pct_seeded_index "$bin_name" sample 0.25 --seed 42 "$data"
run search "$bin_name" search -s \'Agency Name\' "'(?i)us'" "$data"
run search_unicode "$bin_name" search --unicode -s \'Agency Name\' "'(?i)us'" "$data"
run searchset "$bin_name" searchset "$searchset_patterns" "$data"
run searchset_unicode "$bin_name" searchset "$searchset_patterns" --unicode "$data"
run select "$bin_name" select \'Agency,Community Board\' "$data"
run select_regex "$bin_name" select /^L/ "$data"
run slice_one_middle "$bin_name" slice -i 500000 "$data"
run --index slice_one_middle_index "$bin_name" slice -i 500000 "$data"
run sort "$bin_name" sort -s \'Incident Zip\' "$data"
run sort_random_seeded "$bin_name" sort --random --seed 42 "$data"
run sortcheck_sorted "$bin_name" sortcheck "$data_sorted"
run sortcheck_unsorted "$bin_name" sortcheck "$data_unsorted"
run sortcheck_unsorted_all "$bin_name" sortcheck --all "$data_unsorted"
run split "$bin_name" split --size 50000 split_tempdir "$data"
run --index split_index "$bin_name" split --size 50000 split_tempdir "$data"
run --index split_index_j1 "$bin_name" split --size 50000 -j 1 split_tempdir "$data"
run stats "$bin_name" stats --force "$data"
run --index stats_index "$bin_name" stats --force "$data"
run --index stats_index_j1 "$bin_name" stats -j 1 --force "$data"
run stats_everything "$bin_name" stats "$data" --force --everything
run stats_everything_infer_dates "$bin_name" stats "$data" --force --everything --infer-dates
run stats_everything_j1 "$bin_name" stats "$data" --force --everything -j 1
run --index stats_everything_index "$bin_name" stats "$data" --force --everything
run --index stats_everything_infer_dates_index "$bin_name" stats "$data" --force --everything --infer-dates
run --index stats_everything_index_j1 "$bin_name" stats "$data" --force --everything -j 1
run table "$bin_name" table "$data"
run transpose "$bin_name" transpose "$data"
run extsort "$bin_name" extsort "$data_unsorted" test.csv
run schema "$bin_name" schema "$data"
run validate "$bin_name" validate "$data" "$schema"
run sql "$bin_name" sqlp  "$data" city.csv "'select * from _t_1 join _t_2 on _t_1.City = _t_2.City'"

# ---------------------------------------
# Prepare benchmark results directory

# Check if a results directory exists, if it doesn't create it
if [ ! -d "results" ]; then
  mkdir results
fi

# check if the file benchmark_results.csv exists, if it doesn't create it
# along with latest_results.csv
if [ ! -f "results/benchmark_results.csv" ]; then
  touch results/benchmark_results.csv
  echo "version,tstamp,name,mean,stddev,median,user,system,min,max" > results/benchmark_results.csv
  cp results/benchmark_results.csv results/latest_results.csv
fi

# get current version of qsv
version=$("$bin_name" --version | cut -d' ' -f2 | cut -d'-' -f1)

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
  "$bin_name" select '!command' results/hf_result.csv -o results/hf_result_nocmd.csv
  "$bin_name" cat columns results/results_work.csv results/hf_result_nocmd.csv \
    -o results/entry.csv
  "$bin_name" cat rowskey results/latest_results.csv results/entry.csv \
    -o results/results_work.csv
  mv results/results_work.csv results/latest_results.csv
  ((idx++))
done

# ---------------------------------------
# then, run benchmarks with an index
echo "Benchmarking WITH INDEX..."
rm -f "$data_idx"
"$bin_name" index "$data"

idx=0
for command_with_index in "${commands_with_index[@]}"; do
  echo "${commands_with_index_name[$idx]}"
  hyperfine --warmup 2 -i -r 3 --export-csv results/hf_result.csv "$command_with_index"
  echo "version,tstamp,name" > results/results_work.csv
  echo "$version,$now,${commands_with_index_name[$idx]}" >> results/results_work.csv
  "$bin_name" select '!command' results/hf_result.csv -o results/hf_result_nocmd.csv
  "$bin_name" cat columns results/results_work.csv results/hf_result_nocmd.csv \
    -o results/entry.csv
  "$bin_name" cat rowskey results/latest_results.csv results/entry.csv \
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
"$bin_name" sort --select version,tstamp,name results/latest_results.csv \
   -o results/results_work.csv

# compute records per second for each benchmark using qsv, rounding to 3 decimal places
"$bin_name" luau map recs_per_sec \
   'recs_per_sec=(1000000.0 / mean); return tonumber(string.format("%.3f",recs_per_sec))' \
   results/results_work.csv -o results/latest_results.csv

# cat the final results to results/bechmark_results.csv
"$bin_name" cat rowskey results/latest_results.csv results/benchmark_results.csv \
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
