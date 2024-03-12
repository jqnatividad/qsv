#!/bin/bash
#
# Usage: ./benchmarks.sh <argument>
#  where <argument> is a substring pattern of the benchmark name.
#  e.g. ./benchmarks.sh sort - will run benchmarks with "sort" in the benchmark name
#  if <argument> is omitted, all benchmarks are executed.
#
#  if <argument> is "reset", the benchmark data will be downloaded and prepared again.
#   though the results/benchmark_results.csv and resutls/run_info_history.tsv historical
#   archives will be preserved.
#  if <argument> is "clean", temporary files will be deleted.
#  if <argument> is "setup", setup and install all the required tools.
#  if <argument> is "help", help text is displayed.
#
# ==============================================================================================
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
# build it to have at least the apply, geocode, luau, to and polars features enabled:
# i.e. `cargo build --release --locked -F feature_capable,apply,geocode,luau,to,polars` or
# `cargo install --locked qsv -F feature_capable,apply,geocode,luau,to,polars`
#
# This shell script has been tested on Linux and macOS. It should work on other Unix-like systems,
# but will NOT run on native Windows. If you're on Windows, you can run it using Cygwin or WSL
# (see https://www.cygwin.com/ and https://docs.microsoft.com/en-us/windows/wsl/install-win10).
# It requires hyperfine (https://github.com/sharkdp/hyperfine#hyperfine) to run the benchmarks.
# It also requires 7-Zip (https://www.7-zip.org/download.html) as we need the high compression
# ratio so we don't have to deal with git-lfs to host the large compressed file on GitHub.
#
# And of course, it dogfoods `qsv` as well to prepare the benchmark data, fetch the rowcount,
# and to parse and format the benchmark results. :)
# It uses the following commands: apply, cat, count, luau, sample, schema, select, snappy, sort, tojsonl
# and to xlsx. It's a good example of how qsv can be used to automate data preparation & analysis tasks.

arg_pat="$1"

# the version of this script
bm_version=3.19.1

# CONFIGURABLE VARIABLES ---------------------------------------
# change as needed to reflect your environment/workloads

# the path to the qsv binary, change this if you're not using the prebuilt binaries
# e.g. you compiled a tuned version of qsv with different features and/or CPU optimizations enabled
# qsv_bin=../target/release/qsv
# qsv_bin=../target/debug/qsv
qsv_bin=qsv
# the path to the qsv binary that we dogfood to run the benchmarks
# we use several optional features when dogfooding qsv (apply, luau & to)
# and the user may be benchmarking a qsv binary variant that doesn't have these features enabled
qsv_benchmarker_bin=qsv
benchmark_data_url=https://raw.githubusercontent.com/wiki/jqnatividad/qsv/files/NYC_311_SR_2010-2020-sample-1M.7z
# where to download the benchmark data compressed file - this could be a zip or 7z file
datazip=NYC_311_SR_2010-2020-sample-1M.7z
# where to store the benchmark data
data=NYC_311_SR_2010-2020-sample-1M.csv

# Hyoerfine options - run `hyperfine --help`` for more info
# number of warmup runs for each benchmark.  A minimum of 2 is recommended
warmup_runs=2
# number of benchmark runs for each benchmark. A minimum of 3 is recommended
benchmark_runs=3
# ----------------------------  end of CONFIGURABLE VARIABLES

data_filename=$(basename -- "$data")
filestem="${data_filename%.*}"

# check if qsv is installed
if ! command -v "$qsv_bin" &>/dev/null; then
  echo "qsv could not be found"
  echo "Please install Quicksilver (qsv) from https://qsv.dathere.com"
  exit
fi

# get current version of qsv
raw_version=$("$qsv_bin" --version)
# get the version of qsv used to run this script
# we use this to determine if the user is using a different qsv binary
# than the one used to run this script
benchmarker_version=$("$qsv_benchmarker_bin" --version)

# if arg_pat is equal to "help", show usage
if [[ "$arg_pat" == "help" ]]; then
  echo "Quicksilver (qsv) Benchmark Script v$bm_version"
  echo ""
  echo "Usage: ./benchmarks.sh <argument>"
  echo ""
  echo " where <argument> is a substring pattern of the benchmark name."
  echo "       e.g. ./benchmarks.sh sort - will run benchmarks with \"sort\" in the benchmark name"
  echo "       if <argument> is omitted, all benchmarks will be executed."
  echo ""
  echo "       if <argument> is \"reset\", the benchmark data will be downloaded and prepared again."
  echo "          though the results/benchmark_results.csv historical archive will be preserved."
  echo "       if <argument> is \"clean\", temporary files will be deleted."
  echo "       if <argument> is \"setup\", setup and install all the required tools."
  echo "       if <argument> is \"help\", help text is displayed."
  echo ""
  echo "benchmarking: $raw_version"
  echo "dogfooding: $benchmarker_version"
  exit
fi

# check if required tools/dependencies are installed ---------

# check if benchmarker_bin has the apply feature enabled
if [[ "$benchmarker_version" != *"apply;"* ]]; then
  echo "ERROR: $qsv_benchmarker_bin does not have the apply feature enabled."
  echo "The qsv apply command is needed to format the benchmarks results."
  exit
fi

# check if the benchmarker_bin has the luau feature enabled
if [[ "$benchmarker_version" != *"Luau"* ]]; then
  echo "ERROR: $qsv_benchmarker_bin does not have the luau feature enabled."
  echo "The qsv luau command is needed to aggregate the benchmarks results."
  exit
fi

# check if the benchmarker_bin has the to feature enabled
if [[ "$benchmarker_version" != *"to;"* ]]; then
  # check if benchmark_data.xlsx exists
  if [ ! -r benchmark_data.xlsx ]; then
    echo "ERROR: $qsv_benchmarker_bin does not have the to feature enabled."
    echo "The qsv to xlsx command is needed to create an Excel spreadsheet"
    echo "as benchmark_data.xlsx does not exist."
    exit
  fi
fi

# set sevenz_bin to "7z" on Linux/Cygwin and "7zz" on macOS
if [[ "$OSTYPE" == "darwin"* ]]; then
  sevenz_bin=7zz
else
  sevenz_bin=7z
fi

# if arg_pat is equal to "setup", setup and install all the required tools
if [[ "$arg_pat" == "setup" ]]; then

  need_sevenz=0
  need_hyperfine=0
  need_awk=0
  need_sed=0

  # check if 7z is installed
  if ! command -v "$sevenz_bin" &>/dev/null; then
    need_sevenz=1
  fi

  # check if hyperfine is installed
  if ! command -v hyperfine &>/dev/null; then
    need_hyperfine=1
  fi

  # check if awk is installed
  if ! command -v awk &>/dev/null; then
    need_awk=1
  fi

  # check if sed is installed
  if ! command -v sed &>/dev/null; then
    need_sed=1
  fi

  # if all required tools are installed, exit
  if [[ "$need_sevenz" -eq 0 && "$need_hyperfine" -eq 0 && "$need_awk" -eq 0 && "$need_sed" -eq 0 ]]; then
    echo "> All required tools are installed..."
    exit
  fi

  # check if homebrew is installed, if not, install it
  # as we need it to install the required tools
  if ! command -v brew &>/dev/null; then
    echo "INFO: Homebrew could not be found. Installing brew first. Please enter requested info when prompted."
    curl -fsSL "https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh"
  fi

  # if 7z is not installed, install it
  if [[ "$need_sevenz" -eq 1 ]]; then
    echo "INFO: 7-Zip could not be found. Installing..."
    brew install 7zip
  fi

  # if hyperfine is not installed, install it
  if [[ "$need_hyperfine" -eq 1 ]]; then
    echo "INFO: hyperfine could not be found. Installing..."
    brew install hyperfine
  fi

  # if awk is not installed, install it
  if [[ "$need_awk" -eq 1 ]]; then
    echo "INFO: awk could not be found. Installing..."
    brew install gawk
  fi

  # if sed is not installed, install it
  if [[ "$need_sed" -eq 1 ]]; then
    echo "INFO: sed could not be found. Installing..."
    brew install gnu-sed
  fi

  echo "> All required tools installed! You can run ./benchmarks.sh now."
  exit
fi

# check if 7z is installed
if ! command -v "$sevenz_bin" &>/dev/null; then
  echo "ERROR: $sevenz_bin could not be found."
  echo "Please install 7-Zip v23.01 and above or run \"./benchmarks.sh setup\" to install it."
  exit
fi

# check if hyperfine is installed
if ! command -v hyperfine &>/dev/null; then
  echo "ERROR: hyperfine could not be found"
  echo "Please install hyperfine v1.18.0 and above or run \"./benchmarks.sh setup\" to install it."
  exit
fi

# check if awk is installed
if ! command -v awk &>/dev/null; then
  echo "ERROR: awk could not be found"
  echo "Please install awk or run \"./benchmarks.sh setup\" to install it."
  exit
fi

# check if sed is installed
if ! command -v sed &>/dev/null; then
  echo "ERROR: sed could not be found"
  echo "Please install sed or run \"./benchmarks.sh setup\" to install it."
  exit
fi

# qsv version metadata ----------------
version=$(echo "$raw_version" | cut -d' ' -f2 | cut -d'-' -f1)
# get target platform from version
platform=$(echo "$raw_version" | sed 's/.*(\([a-z0-9_-]*\) compiled with Rust.*/\1/')
# get qsv kind
kind=$(echo "$raw_version" | sed 's/.* \([a-zA-Z-]*\)$/\1/')

# get num cores & memory size
if [[ "$OSTYPE" == "darwin"* ]]; then
  # macOS
  num_cores=$(sysctl -n hw.ncpu)
  mem_size=$(sysctl -n hw.memsize)
elif [[ "$OSTYPE" == "linux-gnu"* ]]; then
  # Linux
  num_cores=$(nproc)
  mem_size=$(free -b | awk '/Mem/ {print $7}')
elif [[ "$OSTYPE" == "msys" || "$OSTYPE" == "cygwin" ]]; then
  # Windows
  num_cores=$(wmic cpu get NumberOfCores | grep -Eo '^[0-9]+')
  mem_size=$(wmic OS get FreePhysicalMemory | grep -Eo '[0-9]+')
  mem_size=$((mem_size * 1024))
else
  echo "Unsupported operating system: $OSTYPE"
  exit 1
fi

function cleanup_files {
  # Clean up temporary files
  rm -f "$filestem".csv.*
  rm -f "$filestem".stats.*
  rm -f results/hf_result.csv
  rm -f results/hf_result_nocmd.csv
  rm -f results/results_work.csv
  rm -f results/run_info_work.tsv
  rm -f results/entry.csv
  rm -r -f split_tempdir
  rm -r -f split_tempdir_chunks
  rm -r -f split_tempdir_kbs
  rm -r -f split_tempdir_idx
  rm -r -f split_tempdir_idx_j1
  rm -r -f split_tempdir_chunks_idx
  rm -r -f split_tempdir_chunks_idx_j1
  rm -f benchmark_work.*
  rm -r -f benchmark_work
  rm -f extsort_sorted.csv
}

# if arg_pat is equal to "reset", download and prepare the benchmark data again
# the results/benchmark_results.csv historical archive will be preserved
if [[ "$arg_pat" == "reset" ]]; then
  rm -f "$datazip"
  rm -f "$filestem".*
  rm -f communityboards.csv
  rm -f data_to_exclude.csv
  rm -f data_unsorted.csv
  rm -f data_sorted.csv
  rm -f benchmark_data.xlsx
  rm -f benchmark_data.jsonl
  rm -f benchmark_data.schema.json
  rm -f searchset_patterns.txt
  rm -f searchset_patterns_unicode.txt
  echo "> Benchmark data reset..."
  echo "  Historical benchmarks archive preserved in results/benchmark_results.csv"
  exit
fi

# if arg_pat is equal to "clean", clean up temporary files
if [[ "$arg_pat" == "clean" ]]; then
  cleanup_files
  echo "> Temporary files cleaned up..."
  exit
fi

echo "> Setting up Benchmark environment..."
SECONDS=0

cleanup_files

if [ ! -r "$data" ]; then
  echo "> Downloading Benchmark data..."
  curl -sS "$benchmark_data_url" >"$datazip"
  "$sevenz_bin" e -y "$datazip"
  echo ""
fi

# we get the rowcount, just in case the benchmark data was modified by the user to tailor
# the benchmark to their system/workload. We use the rowcount to compute records per second
rowcount=$("$qsv_bin" count "$data")
printf "  Benchmark data rowcount: %'.0f\n" "$rowcount"
qsv_absolute_path=$(which "$qsv_bin")
benchmarker_absolute_path=$(which "$qsv_benchmarker_bin")
printf "  Benchmarking qsv binary: %s\n" "$qsv_absolute_path"
printf "  %s\n" "$raw_version"
printf "  Dogfooding qsv binary: %s\n" "$benchmarker_absolute_path"
echo ""

if [ ! -r communityboards.csv ]; then
  echo "> Downloading community board data..."
  curl -sS https://raw.githubusercontent.com/wiki/jqnatividad/qsv/files/communityboards.csv >communityboards.csv
  echo ""
fi

if [ ! -r searchset_patterns_unicode.txt ]; then
  echo "> Preparing benchmark support data..."
  # create an index so benchmark data preparation commands can run faster
  "$qsv_bin" index "$data"
  echo "   data_to_exclude.csv..."
  "$qsv_bin" sample --seed 42 1000 "$data" -o data_to_exclude.csv
  echo "   data_unsorted.csv..."
  "$qsv_bin" sort --seed 42 --random --faster "$data" -o data_unsorted.csv
  echo "   data_sorted.csv..."
  "$qsv_bin" sort "$data" -o data_sorted.csv
  echo "   benchmark_data.xlsx..."
  "$qsv_benchmarker_bin" to xlsx benchmark_data.xlsx "$data"
  echo "   benchmark_data.jsonl..."
  "$qsv_bin" tojsonl "$data" --output benchmark_data.jsonl
  echo "   benchmark_data.schema.json..."
  "$qsv_bin" schema "$data" --stdout >benchmark_data.csv.schema.json
  echo "   benchmark_data.snappy..."
  "$qsv_bin" snappy compress "$data" --output benchmark_data.snappy
  echo "   searchset_patterns.txt..."
  printf "homeless\npark\nNoise\n" >searchset_patterns.txt
  echo "   searchset_patterns_unicode.txt..."
  printf "homeless\n💩\nNoise\n" >searchset_patterns_unicode.txt
  echo ""
fi

schema=benchmark_data.csv.schema.json

commands_without_index=()
commands_without_index_name=()
commands_with_index=()
commands_with_index_name=()

function add_command {
  local dest_array="$1"
  shift
  local cmd="$*"

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

  if [[ "$name" == *"$arg_pat"* ]]; then
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
# commands with an --index prefix will be benchmarked with an index and a stats cache
# template: run <benchmark name> <qsv command> <qsv command args>
# Note that several benchmarks assume the the benchmark_data is using the NYC 311 dataset, so
# the column names are hardcoded.
# If you're using a different dataset, you will need to modify the commands below to use the
# appropriate column names.

run apply_calcconv "$qsv_bin apply calcconv --formatstr \"{Unique Key} meters in miles\" --new-column new_col $data"
run apply_dynfmt "$qsv_bin apply dynfmt --formatstr \"{Created Date} {Complaint Type} - {BBL} {City}\" --new-column new_col $data"
run apply_emptyreplace "$qsv_bin" apply emptyreplace \"Bridge Highway Name\" --replacement Unspecified "$data"
run apply_op_eudex "$qsv_bin apply operations lower,eudex Agency --comparand Queens --new-column Agency_queens_soundex $data"
run apply_op_string "$qsv_bin apply operations lower Agency $data"
run apply_op_similarity "$qsv_bin apply operations lower,simdln Agency --comparand brooklyn --new-column Agency_sim-brooklyn_score $data"
run behead "$qsv_bin" behead "$data"
run behead_flexible "$qsv_bin" behead --flexible "$data"
run cat_columns "$qsv_bin" cat columns "$data" data_unsorted.csv
run cat_rows "$qsv_bin" cat rows "$data" data_unsorted.csv
run cat_rows_flexible "$qsv_bin" cat rows --flexible "$data" data_unsorted.csv
run cat_rowskey "$qsv_bin" cat rowskey "$data" data_unsorted.csv
run count "$qsv_bin" count "$data"
run count_flexible "$qsv_bin" count --flexible "$data"
run count_polars_lowmem "$qsv_bin" count --low-memory "$data"
run count_no_polars "$qsv_bin" count --no-polars "$data"
run --index count_index "$qsv_bin" count "$data"
run count_width "$qsv_bin" count --width "$data"
run --index count_width_index "$qsv_bin" count --width "$data"
run datefmt "$qsv_bin datefmt \"Created Date\" $data"
run datefmt_multi "$qsv_bin datefmt \"Created Date,Closed Date,Due Date\" $data"
run datefmt_multi_select "$qsv_bin datefmt '/(?i) date$/' $data"
run datefmt_formatstr_newcol "$qsv_bin datefmt --formatstr '%V' \"Created Date\" --new-column week_number $data"
run dedup "$qsv_bin" dedup "$data"
run dedup_sorted "$qsv_bin" dedup data_sorted.csv
run diff "$qsv_bin" diff "$data" data_unsorted.csv
run enum "$qsv_bin" enum "$data"
run enum_uuid "$qsv_bin" enum --uuid "$data"
run enum_constant "$qsv_bin" enum --constant "NYC" "$data"
run enum_copy "$qsv_bin" enum --copy Agency "$data"
run excel "$qsv_bin" excel benchmark_data.xlsx
run exclude "$qsv_bin" exclude \'Incident Zip\' "$data" \'Incident Zip\' data_to_exclude.csv
run --index exclude_index "$qsv_bin" exclude \'Incident Zip\' "$data" \'Incident Zip\' data_to_exclude.csv
run exclude_casei "$qsv_bin" exclude --ignore-case \'Incident Zip\' "$data" \'Incident Zip\' data_to_exclude.csv
run --index exclude_casei_index "$qsv_bin" exclude --ignore-case \'Incident Zip\' "$data" \'Incident Zip\' data_to_exclude.csv
run exclude_multi "$qsv_bin" exclude \'Incident Zip,Community Board,Agency\' "$data" \'Incident Zip,Community Board,Agency\' data_to_exclude.csv
run --index exclude_multi_index "$qsv_bin" exclude \'Incident Zip,Community Board,Agency\' "$data" \'Incident Zip,Community Board,Agency\' data_to_exclude.csv
run exclude_multi_casei "$qsv_bin" exclude --ignore-case \'Incident Zip,Community Board,Agency\' "$data" \'Incident Zip,Community Board,Agency\' data_to_exclude.csv
run --index exclude_multi_casei_index "$qsv_bin" exclude --ignore-case \'Incident Zip,Community Board,Agency\' "$data" \'Incident Zip,Community Board,Agency\' data_to_exclude.csv
run explode "$qsv_bin" explode City "-" "$data"
run extdedup "$qsv_bin" extdedup "$data"
run extsort "$qsv_bin" extsort data_unsorted.csv extsort_sorted.csv
run fill "$qsv_bin" fill -v Unspecified \'Address Type\' "$data"
run fixlengths "$qsv_bin" fixlengths "$data"
run flatten "$qsv_bin" flatten "$data"
run flatten_condensed "$qsv_bin" flatten "$data" --condense 50
run fmt "$qsv_bin" fmt --crlf "$data"
run fmt_no_crlf "$qsv_bin" fmt "$data"
run fmt_no_final_newline "$qsv_bin" fmt --no-final-newline "$data"
run frequency "$qsv_bin" frequency "$data"
run --index frequency_index "$qsv_bin" frequency "$data"
run frequency_selregex "$qsv_bin" frequency -s /^R/ "$data"
run frequency_j1 "$qsv_bin" frequency -j 1 "$data"
run frequency_ignorecase "$qsv_bin" frequency -i "$data"
run --index frequency_ignorecase_index "$qsv_bin" frequency -i "$data"
run frequency_selregex_ignorecase "$qsv_bin" frequency -s /^R/ -i "$data"
run frequency_j1_ignorecase "$qsv_bin" frequency -j 1 -i "$data"
run geocode_suggest "$qsv_bin" geocode suggest City --new-column geocoded_city "$data"
run geocode_reverse "$qsv_bin" geocode reverse Location --new-column geocoded_location "$data"
run index "$qsv_bin" index "$data"
run input "$qsv_bin" input "$data"
run join "$qsv_bin" join \'Community Board\' "$data" community_board communityboards.csv
run join_casei "$qsv_bin" join \'Community Board\' "$data" community_board --ignore-case communityboards.csv
run joinp "$qsv_bin" joinp \'Community Board\' "$data" community_board communityboards.csv
run jsonl "$qsv_bin" jsonl benchmark_data.jsonl
run jsonl_j1 "$qsv_bin" jsonl -j 1 benchmark_data.jsonl
run luau_filter "$qsv_bin" luau filter \"Location == \'\'\" "$data"
run luau_filter_colidx "$qsv_bin" luau filter --colindex \"Location == \'\'\" "$data"
run luau_filter_no_globals "$qsv_bin" luau filter --no-globals \"Location == \'\'\" "$data"
run luau_filter_no_globals_colidx "$qsv_bin" luau filter --no-globals --colindex \"Location == \'\'\" "$data"
run luau_multi "$qsv_bin" luau map dow,hourday,weekno "file:dt_format.luau" "$data"
run luau_multi_colidx "$qsv_bin" luau map dow,hourday,weekno "file:dt_format.luau" --colindex "$data"
run luau_filter_no_globals_colidx "$qsv_bin" luau filter --no-globals --colindex \"Location == \'\'\" "$data"
run luau_filter_no_globals_no_colidx "$qsv_bin" luau filter --no-globals \"Location == \'\'\" "$data"
run luau_multi_no_globals "$qsv_bin" luau map dow,hourday,weekno --no-globals "file:dt_format.luau" "$data"
run luau_multi_no_globals_colidx "$qsv_bin" luau map dow,hourday,weekno --no-globals --colindex "file:dt_format.luau" "$data"
run luau_script "$qsv_bin" luau map turnaround_time "file:turnaround_time.luau" "$data"
run luau_script_colidx "$qsv_bin" luau map turnaround_time --colindex "file:turnaround_time.luau" "$data"
run luau_script_no_globals "$qsv_bin" luau map turnaround_time --no-globals "file:turnaround_time.luau" "$data"
run luau_script_no_globals_colidx "$qsv_bin" luau map turnaround_time --no-globals --colindex "file:turnaround_time.luau" "$data"
run partition "$qsv_bin" partition \'Community Board\' /tmp/partitioned "$data"
run pseudo "$qsv_bin" pseudo \'Unique Key\' "$data"
run pseudo_formatstr "$qsv_bin" pseudo \'Unique Key\' --formatstr 'ID-{}' --increment 5 "$data"
run rename "$qsv_bin" rename \'unique_key,created_date,closed_date,agency,agency_name,complaint_type,descriptor,loctype,zip,addr1,street,xstreet1,xstreet2,inter1,inter2,addrtype,city,landmark,facility_type,status,due_date,res_desc,res_act_date,comm_board,bbl,boro,xcoord,ycoord,opendata_type,parkname,parkboro,vehtype,taxi_boro,taxi_loc,bridge_hwy_name,bridge_hwy_dir,ramp,bridge_hwy_seg,lat,long,loc\' "$data"
run replace "$qsv_bin" replace \'zip\' \'postal\' "$data"
run reverse "$qsv_bin" reverse "$data"
run --index reverse_index "$qsv_bin" reverse "$data"
run safenames "$qsv_bin" safenames "$data"
run sample_10 "$qsv_bin" sample 10 "$data"
run --index sample_10_index "$qsv_bin" sample 10 "$data"
run sample_1000 "$qsv_bin" sample 1000 "$data"
run --index sample_1000_index "$qsv_bin" sample 1000 "$data"
run sample_100000 "$qsv_bin" sample 100000 "$data"
run --index sample_100000_index "$qsv_bin" sample 100000 "$data"
run sample_100000_seeded "$qsv_bin" sample 100000 --seed 42 "$data"
run sample_100000_seeded_faster "$qsv_bin" sample 100000 --rng faster --seed 42 "$data"
run sample_100000_seeded_secure "$qsv_bin" sample 100000 --rng cryptosecure --seed 42 "$data"
run --index sample_100000_seeded_index "$qsv_bin" sample --seed 42 100000 "$data"
run --index sample_100000_seeded_index_faster "$qsv_bin" sample --rng faster --seed 42 100000 "$data"
run --index sample_100000_seeded_index_secure "$qsv_bin" sample --rng cryptosecure --seed 42 100000 "$data"
run --index sample_25pct_index "$qsv_bin" sample 0.25 "$data"
run --index sample_25pct_seeded_index "$qsv_bin" sample 0.25 --seed 42 "$data"
run schema "$qsv_bin" schema --force "$data"
run --index schema_index "$qsv_bin" schema "$data"
run search "$qsv_bin" search -s \'Agency Name\' "'(?i)us'" "$data"
run search_unicode "$qsv_bin" search --unicode -s \'Agency Name\' "'(?i)us'" "$data"
run searchset "$qsv_bin" searchset searchset_patterns.txt "$data"
run searchset_ignorecase "$qsv_bin" searchset --ignore-case searchset_patterns.txt "$data"
run searchset_unicode "$qsv_bin" searchset searchset_patterns_unicode.txt --unicode "$data"
run select "$qsv_bin" select \'Agency,Community Board\' "$data"
run select_regex "$qsv_bin" select /^L/ "$data"
run slice_one_middle "$qsv_bin" slice -i 500000 "$data"
run --index slice_one_middle_index "$qsv_bin" slice -i 500000 "$data"
run snappy_compress "$qsv_bin" snappy compress "$data" --output benchmark_data.snappy
run snappy_decompress "$qsv_bin" snappy decompress benchmark_data.snappy
run snappy_validate "$qsv_bin" snappy validate benchmark_data.snappy
run sort "$qsv_bin" sort -s \'Incident Zip\' "$data"
run sort_random_seeded "$qsv_bin" sort --random --seed 42 "$data"
run sort_random_seeded_faster "$qsv_bin" sort --random --rng faster --seed 42 "$data"
run sort_random_seeded_secure "$qsv_bin" sort --random --rng cryptosecure --seed 42 "$data"
run sortcheck_sorted "$qsv_bin" sortcheck data_sorted.csv
run sortcheck_unsorted "$qsv_bin" sortcheck data_unsorted.csv
run sortcheck_unsorted_all "$qsv_bin" sortcheck --all data_unsorted.csv
run split "$qsv_bin" split --size 50000 split_tempdir "$data"
run split_chunks "$qsv_bin" split --chunks 20 split_tempdir_chunks "$data"
run split_kbsize "$qsv_bin" split --kb-size 10000 split_tempdir_kbs "$data"
run --index split_index "$qsv_bin" split --size 50000 split_tempdir_idx "$data"
run --index split_index_j1 "$qsv_bin" split --size 50000 -j 1 split_tempdir_idx_j1 "$data"
run --index split_chunks_index "$qsv_bin" split --chunks 20 split_tempdir_chunks_idx "$data"
run --index split_chunks_index_j1 "$qsv_bin" split --chunks 20 -j 1 split_tempdir_chunks_idx_j1
run sqlp "$qsv_bin" sqlp "$data" -Q '"select * from _t_1 where \"Complaint Type\"='\''Noise'\'' and Borough='\''BROOKLYN'\''"'
run sqlp_aggregations "$qsv_bin" sqlp "$data" -Q '"select Borough, count(*) from _t_1 where \"Complaint Type\"='\''Noise'\'' group by Borough"'
run sqlp_format_arrow "$qsv_bin" sqlp --format arrow "$data" -Q '"select * from _t_1 where \"Complaint Type\"='\''Noise'\'' and Borough='\''BROOKLYN'\''"'
run sqlp_format_avro "$qsv_bin" sqlp --format avro "$data" -Q '"select * from _t_1 where \"Complaint Type\"='\''Noise'\'' and Borough='\''BROOKLYN'\''"'
run sqlp_format_json "$qsv_bin" sqlp --format json "$data" -Q '"select * from _t_1 where \"Complaint Type\"='\''Noise'\'' and Borough='\''BROOKLYN'\''"'
run sqlp_format_jsonl "$qsv_bin" sqlp --format jsonl "$data" -Q '"select * from _t_1 where \"Complaint Type\"='\''Noise'\'' and Borough='\''BROOKLYN'\''"'
run sqlp_format_parquet "$qsv_bin" sqlp --format parquet "$data" -Q '"select * from _t_1 where \"Complaint Type\"='\''Noise'\'' and Borough='\''BROOKLYN'\''"'
run sqlp_format_parquet_statistics "$qsv_bin" sqlp --format parquet --statistics "$data" -Q '"select * from _t_1 where \"Complaint Type\"='\''Noise'\'' and Borough='\''BROOKLYN'\''"'
run sqlp_lowmemory "$qsv_bin" sqlp "$data" -Q --low-memory '"select * from _t_1 where \"Complaint Type\"='\''Noise'\'' and Borough='\''BROOKLYN'\''"'
run sqlp_nooptimizations "$qsv_bin" sqlp "$data" -Q --no-optimizations '"select * from _t_1 where \"Complaint Type\"='\''Noise'\'' and Borough='\''BROOKLYN'\''"'
run sqlp_tryparsedates "$qsv_bin" sqlp "$data" -Q --try-parsedates '"select * from _t_1 where \"Complaint Type\"='\''Noise'\'' and Borough='\''BROOKLYN'\''"'
run sqlp_tryparsedates_inferlen "$qsv_bin" sqlp "$data" -Q --infer-len 10000 --try-parsedates '"select * from _t_1 where \"Complaint Type\"='\''Noise'\'' and Borough='\''BROOKLYN'\''"'
run stats "$qsv_bin" stats --force "$data"
run stats_create_cache "$qsv_bin" stats --force "$data"
run --index stats_index "$qsv_bin" stats --force "$data"
run --index stats_index_with_cache "$qsv_bin" stats "$data"
run --index stats_index_j1 "$qsv_bin" stats -j 1 --force "$data"
run --index stats_index_j1_with_cache "$qsv_bin" stats -j 1 "$data"
run stats_everything "$qsv_bin" stats "$data" --force --everything
run stats_everything_create_cache "$qsv_bin" stats "$data" --force --everything
run stats_everything_infer_dates "$qsv_bin" stats "$data" --force --everything --infer-dates
run stats_everything_j1 "$qsv_bin" stats "$data" --force --everything -j 1
run --index stats_everything_index "$qsv_bin" stats "$data" --force --everything
run --index stats_everything_index_with_cache "$qsv_bin" stats "$data" --everything
run --index stats_everything_infer_dates_index "$qsv_bin" stats "$data" --force --everything --infer-dates
run --index stats_everything_infer_dates_index_with_cache "$qsv_bin" stats "$data" --everything --infer-dates
run --index stats_everything_index_j1 "$qsv_bin" stats "$data" --force --everything -j 1
run --index stats_everything_index_j1_with_cache "$qsv_bin" stats "$data" --everything -j 1
run table "$qsv_bin" table "$data"
run to_xlsx "$qsv_bin" to xlsx benchmark_work.xlsx "$data"
run to_sqlite "$qsv_bin" to sqlite benchmark_work.db "$data"
run to_parquet "$qsv_bin" to parquet benchmark_work "$data"
run to_datapackage "$qsv_bin" to datapackage benchmark_work.json "$data"
run tojsonl "$qsv_bin" tojsonl "$data"
run tojsonl_j1 "$qsv_bin" tojsonl -j 1 "$data"
run tojsonl_trim "$qsv_bin" tojsonl --trim "$data"
run tojsonl_trim_j1 "$qsv_bin" tojsonl --trim -j 1 "$data"
run --index tojsonl_index "$qsv_bin" tojsonl "$data"
run --index tojsonl_index_j1 "$qsv_bin" tojsonl --jobs 1 "$data"
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
printf "> Commands to benchmark: %s, w/o index: %s, with index: %s\n\n" "$total_count" "$wo_index_count" "$with_index_count"
# ---------------------------------------
# Prepare benchmark results directory

# Check if a results directory exists, if it doesn't create it
if [ ! -d "results" ]; then
  mkdir results
fi

# Init latest_results.csv. It stores the benchmark results for this run
rm -f results/latest_results.csv
echo "version,tstamp,name,mean,stddev,median,user,system,min,max" >results/latest_results.csv

# check if the file benchmark_results.csv exists, if it doesn't create it
# by copying the empty latest_results.csv
if [ ! -f "results/benchmark_results.csv" ]; then
  cp results/latest_results.csv results/benchmark_results.csv
fi

# get current time to the nearest hour
now=$(date +"%Y-%m-%d-%H")

# get current time to the nearest second
now_sec=$(date +"%Y-%m-%d-%H-%M-%S")

# ---------------------------------------
# Run hyperfine to compile benchmark results. Append each individual result to the latest_results.csv
# by dogfooding qsv's cat, luau, select & sort commands.

# first, run benchmarking without an index
# each command is run five times. Two warm-up runs & three benchmarked runs.
echo "> Benchmarking WITHOUT INDEX..."
idx=0
name_idx=1
for command_no_index in "${commands_without_index[@]}"; do

  # remove the index file and the stats cache files
  rm -f "$data".idx
  rm -f "$filestem".stats.*

  pct_complete=$(((name_idx - 1) * 100 / total_count))

  echo "$name_idx. ${commands_without_index_name[$idx]} ($pct_complete%)"
  hyperfine -N --warmup "$warmup_runs" -i --runs "$benchmark_runs" --export-csv results/hf_result.csv \
    "$command_no_index"

  # prepend version, tstamp & benchmark name to the hyperfine results
  echo "version,tstamp,name" >results/results_work.csv
  echo "$version,$now,${commands_without_index_name[$idx]}" >>results/results_work.csv

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
# then, run benchmarks with an index and stats cache
# an index enables random access and unlocks multi-threading in several commands
# the stats cache enables faster stats computation as it will use the cached stats
# when its valid and available, instead of computing the stats from scratch
if [ "$with_index_count" -gt 0 ]; then
  echo "> Benchmarking WITH INDEX and STATS CACHE..."
  echo "  Preparing index and stats cache..."
  rm -f "$data".idx
  "$qsv_bin" index "$data"
  "$qsv_bin" stats "$data" --everything --infer-dates --stats-binout --force \
    --output benchmark_work.stats.csv
fi

idx=0
for command_with_index in "${commands_with_index[@]}"; do
  pct_complete=$(((name_idx - 1) * 100 / total_count))

  echo "$name_idx. ${commands_with_index_name[$idx]} ($pct_complete%)"
  hyperfine -N --warmup "$warmup_runs" -i --runs "$benchmark_runs" --export-csv results/hf_result.csv \
    "$command_with_index"
  echo "version,tstamp,name" >results/results_work.csv
  echo "$version,$now,${commands_with_index_name[$idx]}" >>results/results_work.csv
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
# We compute recs_per_sec by dividing the number of rows in the benchmark data
# by the mean run time of the three runs.
# We then append/concatenate the latest results to benchmark_results.csv - which is
# a historical archive, so we can track performance over multiple releases.

echo ""
# sort the benchmark results by version, tstamp & name
"$qsv_bin" sort --select version,tstamp,name results/latest_results.csv \
  -o results/results_work.csv

# compute records per second for each benchmark using luau by dividing rowcount by mean
# we then round the result to a whole number. We also compute the total mean

# we set the QSVBM_ROWCOUNT environment variable to the rowcount so it can be used
# by the luau script by using the qsv.get_env() function
export QSVBM_ROWCOUNT=$rowcount
# we run the benchmark_aggregations.luau script using qsv's luau command
# total_mean is the total mean of all the benchmarks
# it is computed in the END block of the script and is sent to stderr
# which we redirect to a file named total_mean.txt
"$qsv_benchmarker_bin" luau map recs_per_sec "file:benchmark_aggregations.luau" \
  results/results_work.csv -o results/latest_results.csv 2>total_mean.txt
# we read the total_mean from the total_mean.txt file
total_mean=$(<total_mean.txt)

# Concatenate the final results of this run to results/bechmark_results.csv
"$qsv_bin" cat rowskey results/latest_results.csv results/benchmark_results.csv \
  -o results/results_work.csv
mv results/results_work.csv results/benchmark_results.csv

# make "display" versions of the results
# i.e. number of decimal places is reduced to 3, and column order is changed so it's easier to read
# with recs_per_sec moved from the back after mean followed by the rest of the stats columns

# first - for benchmark_results_display.csv, move the recs_per_sec column after the
# mean column using the `qsv select` command
"$qsv_bin" select version,tstamp,name,mean,recs_per_sec,stddev,median,user,system,min,max \
  results/benchmark_results.csv -o results/benchmark_results_display.csv

# then, round the stats columns to 3 decimal places using the `qsv apply operations round` command
# it defaults to 3 decimal places if the --formatstr option is not specified
"$qsv_benchmarker_bin" apply operations round mean,stddev,median,user,system,min,max \
  results/benchmark_results_display.csv -o results/results_work.csv
mv results/results_work.csv results/benchmark_results_display.csv

# do the same for latest_results_display.csv
"$qsv_bin" select version,tstamp,name,mean,recs_per_sec,stddev,median,user,system,min,max \
  results/latest_results.csv -o results/latest_results_display.csv
"$qsv_benchmarker_bin" apply operations round mean,stddev,median,user,system,min,max \
  results/latest_results_display.csv -o results/results_work.csv
mv results/results_work.csv results/latest_results_display.csv

# Clean up temporary files
cleanup_files

# ---------------------------------------
# Finalize benchmark run info. Append the run info to results/run_info_history.csv
# we use the TSV format as some of the data has commas/quotes/whitespace/semicolon, etc.

# get the environment variables used by qsv
qsv_envvars=$("$qsv_bin" --envlist)

elapsed=$SECONDS

# Init latest_run_info.csv. It stores the benchmark run info for this run
rm -f results/latest_run_info.tsv
echo -e "version\ttstamp\tlogtime\tbm_version\tplatform\tcores\tmem\tbinary\tkind\targument\ttotal_count\two_index_count\twith_index_count\twarmup_runs\tbenchmark_runs\telapsed_secs\ttotal_mean\tqsv_env\tversion_info" >results/latest_run_info.tsv

# check if the file run_info_history.csv exists, if it doesn't create it
# by copying the empty latest_run_info.csv
if [ ! -f "results/run_info_history.tsv" ]; then
  cp results/latest_run_info.tsv results/run_info_history.tsv
fi

# append the run info to latest_run_info.csv
echo -e "$version\t$now\t$now_sec\t$bm_version\t$platform\t$num_cores\t$mem_size\t$qsv_bin\t$kind\t$arg_pat\t$total_count\t$wo_index_count\t$with_index_count\t$warmup_runs\t$benchmark_runs\t$elapsed\t$total_mean\t$qsv_envvars\t$raw_version" >>results/latest_run_info.tsv

# now update the run_info_history.tsv
"$qsv_bin" cat rowskey results/latest_run_info.tsv results/run_info_history.tsv \
  -o results/run_info_work.tsv
mv results/run_info_work.tsv results/run_info_history.tsv

echo "> 100% DONE! $total_count benchmarks executed. Elapsed time: $elapsed seconds. Total mean: $total_mean"
