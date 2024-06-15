# Set all available subcommands for qsv
set -l qsv_commands apply applydp behead cat count datefmt dedup describegpt diff enum excel exclude explode extdedup extsort fetch fetchpost fill fixlengths flatten fmt frequency geocode headers index input join joinp jsonl jsonp luau partition prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table to tojsonl transpose validate

# Enable completions for qsv
complete -c qsv

# qsv options
complete -c qsv -n "not __fish_seen_subcommand_from $qsv_commands" -l list -d 'List all commands available'
complete -c qsv -n "not __fish_seen_subcommand_from $qsv_commands" -l envlist -d 'List all qsv-relevant environment variables'
complete -c qsv -n "not __fish_seen_subcommand_from $qsv_commands" -s u -l update -d 'Update qsv to the latest release from GitHub'
complete -c qsv -n "not __fish_seen_subcommand_from $qsv_commands" -s U -l updatenow -d 'Update qsv to the latest release from GitHub without confirming'
complete -c qsv -n "not __fish_seen_subcommand_from $qsv_commands" -s v -l version -d 'Print version info, mem allocator, features installed, max_jobs, num_cpus, build info then exit'

# If no subcommands are provided yet, show subcommands
complete -c qsv \
    -n "not __fish_seen_subcommand_from $qsv_commands" -f -a "$qsv_commands"

# qsv apply
set -l apply_subcommands operations emptyreplace dynfmt calcconv
complete -c qsv -n "__fish_seen_subcommand_from apply" -n "not __fish_seen_subcommand_from $apply_subcommands" -f -a "$apply_subcommands"
complete -c qsv -n "__fish_seen_subcommand_from apply" -l new-column
complete -c qsv -n "__fish_seen_subcommand_from apply" -l rename
complete -c qsv -n "__fish_seen_subcommand_from apply" -l comparand
complete -c qsv -n "__fish_seen_subcommand_from apply" -l replacement
complete -c qsv -n "__fish_seen_subcommand_from apply" -l formatstr
complete -c qsv -n "__fish_seen_subcommand_from apply" -l jobs
complete -c qsv -n "__fish_seen_subcommand_from apply" -l batch
complete -c qsv -n "__fish_seen_subcommand_from apply" -l output
complete -c qsv -n "__fish_seen_subcommand_from apply" -l no-headers
complete -c qsv -n "__fish_seen_subcommand_from apply" -l delimiter
complete -c qsv -n "__fish_seen_subcommand_from apply" -l progressbar

# qsv count
complete -c qsv -n "__fish_seen_subcommand_from count" -s H -l human-readable -d 'Comma separate row count'
complete -c qsv -n "__fish_seen_subcommand_from count" -l width
complete -c qsv -n "__fish_seen_subcommand_from count" -l flexible
complete -c qsv -n "__fish_seen_subcommand_from count" -l no-headers

# qsv stats
complete -c qsv -n "__fish_seen_subcommand_from stats" -xl select
complete -c qsv -n "__fish_seen_subcommand_from stats" -l everything
complete -c qsv -n "__fish_seen_subcommand_from stats" -l typesonly
complete -c qsv -n "__fish_seen_subcommand_from stats" -l infer-boolean
complete -c qsv -n "__fish_seen_subcommand_from stats" -l mode
complete -c qsv -n "__fish_seen_subcommand_from stats" -l cardinality
complete -c qsv -n "__fish_seen_subcommand_from stats" -l median
complete -c qsv -n "__fish_seen_subcommand_from stats" -l mad
complete -c qsv -n "__fish_seen_subcommand_from stats" -l quartiles
complete -c qsv -n "__fish_seen_subcommand_from stats" -xl round
complete -c qsv -n "__fish_seen_subcommand_from stats" -l nulls
complete -c qsv -n "__fish_seen_subcommand_from stats" -l infer-dates
complete -c qsv -n "__fish_seen_subcommand_from stats" -xl dates-whitelist
complete -c qsv -n "__fish_seen_subcommand_from stats" -l prefer-dmy
complete -c qsv -n "__fish_seen_subcommand_from stats" -l force
complete -c qsv -n "__fish_seen_subcommand_from stats" -xl jobs
complete -c qsv -n "__fish_seen_subcommand_from stats" -l stats-binout
complete -c qsv -n "__fish_seen_subcommand_from stats" -xl cache-threshold
complete -c qsv -n "__fish_seen_subcommand_from stats" -rl output
complete -c qsv -n "__fish_seen_subcommand_from stats" -l no-headers
complete -c qsv -n "__fish_seen_subcommand_from stats" -xl delimiter
complete -c qsv -n "__fish_seen_subcommand_from stats" -l memcheck
