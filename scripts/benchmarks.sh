#!/bin/bash
#
# Usage: ./benchmarks.sh <pattern>
#  where <pattern> is a substring of the command name to be benchmarked.
#  e.g. ./benchmarks.sh sort - will run benchmarks with "sort" in the benchmark name
#  if <pattern> is omitted, all commands will be benchmarked.
#
# This script benchmarks Quicksilver (qsv) using a 520mb, 41 column, 1M row sample of
# NYC's 311 data. If it doesn't exist on your system, it will be downloaded for you.
#
# Though this script was primarily created to maintain the Benchmark page on the qsv site,
# it was also designed to be a useful tool for users to benchmark qsv on their own systems,
# so it be can run on hardware and workloads that reflect your requirements/environment.
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
#
# And of course, it dogfoods `qsv` as well to prepare the benchmark data, fetch the rowcount,
# and to parse and format the benchmark results. :)  It's actually a great example of how qsv
# can be used to automate data preparation and analysis tasks.

set -e

pat="$1"
echo "Setting up benchmarking environment..."
SECONDS=0

# configurable variables - change as needed to reflect your environment/workloads
qsv_bin=qsv
benchmark_data_url=https://raw.githubusercontent.com/wiki/jqnatividad/qsv/files/NYC_311_SR_2010-2020-sample-1M.7z
# where to download the benchmark data compressed file - this could be a zip or 7z file
datazip=/tmp/NYC_311_SR_2010-2020-sample-1M.7z
# where to store the benchmark data
data=NYC_311_SR_2010-2020-sample-1M.csv
warmup_runs=2
benchmark_runs=3

# check if qsv is installed
if ! command -v "$qsv_bin" &> /dev/null
then
    echo "qsv could not be found"
    echo "Please install Quicksilver (qsv) from https://qsv.dathere.com"
    exit
fi

# set sevenz_bin to "7z" on Linux/Cygwin and "7zz" on macOS
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

if [ ! -r "$data" ]; then
  echo "Downloading benchmarking data..."
  curl -sS "$benchmark_data_url" > "$datazip"
  "$sevenz_bin" e -y "$datazip"
fi

# we get the rowcount, just in case the benchmark data was modified by the user to tailor
# the benchmark to their system/workload. We use the rowcount to compute records per second
rowcount=$("$qsv_bin" count "$data")
printf "Benchmark data rowcount: %'.0f\n" $rowcount

if [ ! -r communityboards.csv ]; then
  echo "Downloading community board data..."
  curl -sS https://raw.githubusercontent.com/wiki/jqnatividad/qsv/files/communityboards.csv > communityboards.csv
fi

if [ ! -r data_to_exclude.csv ]; then
  echo "Creating benchmark support data..."
  "$qsv_bin" sample --seed 42 1000 "$data" -o data_to_exclude.csv
  "$qsv_bin" sort --seed 42 --random --faster "$data" -o data_unsorted.csv
  "$qsv_bin" sort "$data" -o data_sorted.csv
  "$qsv_bin" to xlsx benchmark_data.xlsx "$data"
  "$qsv_bin" tojsonl "$data" --output benchmark_data.jsonl
  printf "homeless\npark\nnoise\n" > searchset_patterns.txt
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

  if [[ "$name" == *"$pat"* ]]; then
    if [ -z "$index" ]; then
      commands_without_index_name+=("$name")
      add_command "without_index" "$@"
    else
      commands_with_index_name+=("$name")
      add_command "with_index" "$@"
    fi
  fi
}

# ---------------------------------------
# Queue commands for benchmarking
# commands with an --index prefix will be benchmarked with an index
# template: run <benchmark name> <qsv command> <qsv command args>

run apply_calcconv "$qsv_bin apply calcconv --formatstr \"{Unique Key} meters in miles\" --new-column new_col $data"
run apply_datefmt "$qsv_bin apply datefmt \"Created Date\" $data"
run apply_datefmt_multi "$qsv_bin apply datefmt \"Created Date,Closed Date,Due Date\" $data"
run apply_dynfmt "$qsv_bin apply dynfmt --formatstr \"{Created Date} {Complaint Type} - {BBL} {City}\" --new-column new_col $data"
run apply_emptyreplace "$qsv_bin" apply emptyreplace \"Bridge Highway Name\" --replacement Unspecified "$data"
run apply_op_eudex "$qsv_bin apply operations lower,eudex Agency --comparand Queens --new-column Agency_queens_soundex $data" 
run apply_op_string "$qsv_bin apply operations lower Agency $data"
run apply_op_similarity "$qsv_bin apply operations lower,simdln Agency --comparand brooklyn --new-column Agency_sim-brooklyn_score $data"
run behead "$qsv_bin" behead "$data"
run cat_columns "$qsv_bin" cat columns "$data" data_unsorted.csv
run cat_rows "$qsv_bin" cat rows "$data" data_unsorted.csv
run cat_rowskey "$qsv_bin" cat rowskey "$data" data_unsorted.csv
run count "$qsv_bin" count "$data"
run --index count_index "$qsv_bin" count "$data"
run dedup "$qsv_bin" dedup "$data"
run dedup_sorted "$qsv_bin" dedup data_sorted.csv
run diff "$qsv_bin" diff "$data" data_unsorted.csv
run enum "$qsv_bin" enum "$data"
run excel "$qsv_bin" excel benchmark_data.xlsx
run exclude "$qsv_bin" exclude \'Incident Zip\' "$data" \'Incident Zip\' data_to_exclude.csv
run --index exclude_index "$qsv_bin" exclude \'Incident Zip\' "$data" \'Incident Zip\' data_to_exclude.csv
run explode "$qsv_bin" explode City "-" "$data"
run extdedup "$qsv_bin" extdedup "$data"
run extsort "$qsv_bin" extsort data_unsorted.csv test.csv
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
run input "$qsv_bin" input "$data"
run join "$qsv_bin" join \'Community Board\' "$data" community_board communityboards.csv
run joinp "$qsv_bin" joinp \'Community Board\' "$data" community_board communityboards.csv
run jsonl "$qsv_bin" jsonl benchmark_data.jsonl
run luau "$qsv_bin" luau map location_empty "tonumber\(Location\)==nil" "$data"
run partition "$qsv_bin" partition \'Community Board\' /tmp/partitioned "$data"
run pseudo "$qsv_bin" pseudo \'Unique Key\' "$data"
run rename "$qsv_bin" rename \'unique_key,created_date,closed_date,agency,agency_name,complaint_type,descriptor,loctype,zip,addr1,street,xstreet1,xstreet2,inter1,inter2,addrtype,city,landmark,facility_type,status,due_date,res_desc,res_act_date,comm_board,bbl,boro,xcoord,ycoord,opendata_type,parkname,parkboro,vehtype,taxi_boro,taxi_loc,bridge_hwy_name,bridge_hwy_dir,ramp,bridge_hwy_seg,lat,long,loc\' "$data"
run replace "$qsv_bin" replace \'zip\' \'postal\' "$data"
run reverse "$qsv_bin" reverse "$data"
run safenames "$qsv_bin" safenames "$data"
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
run schema "$qsv_bin" schema "$data"
run --index schema_index "$qsv_bin" schema "$data"
run search "$qsv_bin" search -s \'Agency Name\' "'(?i)us'" "$data"
run search_unicode "$qsv_bin" search --unicode -s \'Agency Name\' "'(?i)us'" "$data"
run searchset "$qsv_bin" searchset searchset_patterns.txt "$data"
run searchset_ignorecase "$qsv_bin" searchset --ignore-case searchset_patterns.txt "$data"
run searchset_unicode "$qsv_bin" searchset searchset_patterns.txt --unicode "$data"
run select "$qsv_bin" select \'Agency,Community Board\' "$data"
run select_regex "$qsv_bin" select /^L/ "$data"
run slice_one_middle "$qsv_bin" slice -i 500000 "$data"
run --index slice_one_middle_index "$qsv_bin" slice -i 500000 "$data"
run snappy_compress "$qsv_bin" snappy compress "$data" --output benchmark_work.snappy
run snappy_decompress "$qsv_bin" snappy decompress benchmark_work.snappy
run snappy_validate "$qsv_bin" snappy validate benchmark_work.snappy
run sort "$qsv_bin" sort -s \'Incident Zip\' "$data"
run sort_random_seeded "$qsv_bin" sort --random --seed 42 "$data"
run sortcheck_sorted "$qsv_bin" sortcheck data_sorted.csv
run sortcheck_unsorted "$qsv_bin" sortcheck data_unsorted.csv
run sortcheck_unsorted_all "$qsv_bin" sortcheck --all data_unsorted.csv
run split "$qsv_bin" split --size 50000 split_tempdir "$data"
run --index split_index "$qsv_bin" split --size 50000 split_tempdir "$data"
run --index split_index_j1 "$qsv_bin" split --size 50000 -j 1 split_tempdir "$data"
run sqlp "$qsv_bin" sqlp  "$data" -Q '"select * from _t_1 where \"Complaint Type\"='\''Noise'\'' and Borough='\''BROOKLYN'\''"'
run sqlp_format_arrow "$qsv_bin" sqlp --format arrow "$data" -Q '"select * from _t_1 where \"Complaint Type\"='\''Noise'\'' and Borough='\''BROOKLYN'\''"'
run sqlp_format_json "$qsv_bin" sqlp --format json "$data" -Q '"select * from _t_1 where \"Complaint Type\"='\''Noise'\'' and Borough='\''BROOKLYN'\''"'
run sqlp_format_parquet "$qsv_bin" sqlp --format parquet "$data" -Q '"select * from _t_1 where \"Complaint Type\"='\''Noise'\'' and Borough='\''BROOKLYN'\''"'
run sqlp_format_parquet_statistics "$qsv_bin" sqlp --format parquet --statistics "$data" -Q '"select * from _t_1 where \"Complaint Type\"='\''Noise'\'' and Borough='\''BROOKLYN'\''"'
run sqlp_lowmemory "$qsv_bin" sqlp  "$data" -Q --low-memory '"select * from _t_1 where \"Complaint Type\"='\''Noise'\'' and Borough='\''BROOKLYN'\''"'
run sqlp_nooptimizations "$qsv_bin" sqlp  "$data" -Q --no-optimizations '"select * from _t_1 where \"Complaint Type\"='\''Noise'\'' and Borough='\''BROOKLYN'\''"'
run sqlp_tryparsedates "$qsv_bin" sqlp  "$data" -Q --try-parsedates '"select * from _t_1 where \"Complaint Type\"='\''Noise'\'' and Borough='\''BROOKLYN'\''"'
run sqlp_tryparsedates_inferlen "$qsv_bin" sqlp  "$data" -Q --infer-len 10000 --try-parsedates '"select * from _t_1 where \"Complaint Type\"='\''Noise'\'' and Borough='\''BROOKLYN'\''"'
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
run to_xlsx "$qsv_bin" to xlsx benchmark_work.xlsx "$data"
run to_sqlite "$qsv_bin" to sqlite benchmark_work.db "$data"
run to_parquet "$qsv_bin" to parquet benchmark_work "$data"
run to_datapackage "$qsv_bin" to datapackage benchmark_work.json "$data"
run tojsonl "$qsv_bin" tojsonl "$data"
run --index tojsonl_index "$qsv_bin" tojsonl "$data"
run transpose "$qsv_bin" transpose "$data"
run transpose_multipass "$qsv_bin" transpose --multipass "$data"
run validate "$qsv_bin" validate "$data" "$schema"
run validate_no_schema "$qsv_bin" validate "$data"
run --index validate_index "$qsv_bin" validate "$data" "$schema"
run --index validate_no_schema_index "$qsv_bin" validate "$data"

# show count of commands to be benchmarked
with_index_count=${#commands_with_index[@]}
wo_index_count=${#commands_without_index[@]}
total_count=$((with_index_count + wo_index_count))
printf "Commands to benchmark: $total_count, w/o index: $wo_index_count, with index: $with_index_count\n\n"

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
# each command is run five times. Two warm-up runs & three benchmarked runs.
echo "Benchmarking WITHOUT INDEX..."
idx=0
name_idx=1
for command_no_index in "${commands_without_index[@]}"; do
  rm -f "$data".idx
  echo "$name_idx. ${commands_without_index_name[$idx]}"
  hyperfine --warmup "$warmup_runs" -i --runs "$benchmark_runs" --export-csv results/hf_result.csv \
    "$command_no_index"
  
  # prepend version, tstamp & benchmark name to the hyperfine results
  echo "version,tstamp,name" > results/results_work.csv
  echo "$version,$now,${commands_without_index_name[$idx]}" >> results/results_work.csv
  
  # remove the command column from the hyperfine results, we just need the name
  "$qsv_bin" select '!command' results/hf_result.csv -o results/hf_result_nocmd.csv

  # the entry.csv file is the expanded benchmark result for this command
  "$qsv_bin" cat columns results/results_work.csv results/hf_result_nocmd.csv \
    -o results/entry.csv

  # append the entry.csv to latest_results.csv
  "$qsv_bin" cat rowskey results/latest_results.csv results/entry.csv \
    -o results/results_work.csv
  mv results/results_work.csv results/latest_results.csv
  ((idx++))
  ((name_idx++))
done

# ---------------------------------------
# then, run benchmarks with an index
# an index enables random access and unlocks multi-threading in several commands
echo "Benchmarking WITH INDEX..."
rm -f "$data".idx
"$qsv_bin" index "$data"

idx=0
for command_with_index in "${commands_with_index[@]}"; do
  echo "$name_idx. ${commands_with_index_name[$idx]}"
  hyperfine --warmup "$warmup_runs" -i --runs "$benchmark_runs" --export-csv results/hf_result.csv \
    "$command_with_index"
  echo "version,tstamp,name" > results/results_work.csv
  echo "$version,$now,${commands_with_index_name[$idx]}" >> results/results_work.csv
  "$qsv_bin" select '!command' results/hf_result.csv -o results/hf_result_nocmd.csv
  "$qsv_bin" cat columns results/results_work.csv results/hf_result_nocmd.csv \
    -o results/entry.csv
  "$qsv_bin" cat rowskey results/latest_results.csv results/entry.csv \
    -o results/results_work.csv
  mv results/results_work.csv results/latest_results.csv
  ((idx++))
  ((name_idx++))
done

# ---------------------------------------
# Finalize benchmark results. Sort the latest results by version, tstamp & name.
# compute and add records per second for each benchmark using qsv's luau command.
# We compute recs_per_sec by dividing 1M (the number of rows in NYC 311 sample data)
# by the mean run time of the three runs, rounding to 3 decimal places.
# We then append/concatenate the latest results to benchmark_results.csv - which is
# a historical archive, so we can track performance over multiple releases.

# sort the benchmark results by version, tstamp & name
"$qsv_bin" sort --select version,tstamp,name results/latest_results.csv \
   -o results/results_work.csv

# compute records per second for each benchmark using qsv, rounding to a whole number
luau_cmd="recs_per_sec=( $rowcount / mean); return numWithCommas(recs_per_sec)"
"$qsv_bin" luau --begin file:benchmark_helper.luau map recs_per_sec "$luau_cmd" \
   results/results_work.csv -o results/latest_results.csv

# Concatenate the final results of this run to results/bechmark_results.csv
"$qsv_bin" cat rowskey results/latest_results.csv results/benchmark_results.csv \
  -o results/results_work.csv
mv results/results_work.csv results/benchmark_results.csv

# Finally, clean up temporary files
rm -f results/hf_result.csv
rm -f results/hf_result_nocmd.csv
rm -f results/results_work.csv
rm -f results/entry.csv
rm -r -f split_tempdir
rm -f benchmark_work.*
rm -r -f benchmark_work

echo "$total_count benchmarks executed. Elapsed time: $SECONDS seconds."
