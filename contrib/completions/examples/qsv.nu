module completions {

  export extern qsv [
    --list
    --envlist
    --update
    --updatenow
    --version
    --help(-h)                # Print help
  ]

  export extern "qsv apply" [
    --new-column
    --rename
    --comparand
    --replacement
    --formatstr
    --jobs
    --batch
    --output
    --no-headers
    --delimiter
    --progressbar
    --help(-h)                # Print help
  ]

  export extern "qsv apply operations" [
    --help(-h)                # Print help
  ]

  export extern "qsv apply emptyreplace" [
    --help(-h)                # Print help
  ]

  export extern "qsv apply dynfmt" [
    --help(-h)                # Print help
  ]

  export extern "qsv apply calcconv" [
    --help(-h)                # Print help
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "qsv apply help" [
  ]

  export extern "qsv apply help operations" [
  ]

  export extern "qsv apply help emptyreplace" [
  ]

  export extern "qsv apply help dynfmt" [
  ]

  export extern "qsv apply help calcconv" [
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "qsv apply help help" [
  ]

  export extern "qsv behead" [
    --flexible
    --output
    --help(-h)                # Print help
  ]

  export extern "qsv cat" [
    --output
    --no-headers
    --delimiter
    --help(-h)                # Print help
  ]

  export extern "qsv cat rows" [
    --flexible
    --help(-h)                # Print help
  ]

  export extern "qsv cat rowskey" [
    --group
    --group-name
    --help(-h)                # Print help
  ]

  export extern "qsv cat columns" [
    --pad
    --help(-h)                # Print help
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "qsv cat help" [
  ]

  export extern "qsv cat help rows" [
  ]

  export extern "qsv cat help rowskey" [
  ]

  export extern "qsv cat help columns" [
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "qsv cat help help" [
  ]

  export extern "qsv clipboard" [
    --save
    --help(-h)                # Print help
  ]

  export extern "qsv count" [
    --human-readable
    --width
    --no-polars
    --low-memory
    --flexible
    --no-headers
    --help(-h)                # Print help
  ]

  export extern "qsv datefmt" [
    --formatstr
    --new-column
    --rename
    --prefer-dmy
    --keep-zero-time
    --input-tz
    --output-tz
    --default-tz
    --utc
    --zulu
    --ts-resolution
    --jobs
    --batch
    --output
    --no-headers
    --delimiter
    --progressbar
    --help(-h)                # Print help
  ]

  export extern "qsv dedup" [
    --select
    --numeric
    --ignore-case
    --sorted
    --dupes-output
    --human-readable
    --jobs
    --output
    --no-headers
    --delimiter
    --quiet
    --memcheck
    --help(-h)                # Print help
  ]

  export extern "qsv describegpt" [
    --all
    --description
    --dictionary
    --tags
    --api-key
    --max-tokens
    --json
    --jsonl
    --prompt
    --prompt-file
    --base-url
    --model
    --timeout
    --user-agent
    --output
    --quiet
    --help(-h)                # Print help
  ]

  export extern "qsv diff" [
    --no-headers-left
    --no-headers-right
    --no-headers-output
    --delimiter-left
    --delimiter-right
    --delimiter-output
    --key
    --sort-columns
    --jobs
    --output
    --help(-h)                # Print help
  ]

  export extern "qsv enum" [
    --new-column
    --start
    --increment
    --constant
    --copy
    --uuid4
    --uuid7
    --hash
    --output
    --no-headers
    --delimiter
    --help(-h)                # Print help
  ]

  export extern "qsv excel" [
    --sheet
    --metadata
    --error-format
    --flexible
    --trim
    --date-format
    --keep-zero-time
    --range
    --jobs
    --output
    --delimiter
    --quiet
    --help(-h)                # Print help
  ]

  export extern "qsv exclude" [
    --ignore-case
    -v
    --output
    --no-headers
    --delimiter
    --help(-h)                # Print help
  ]

  export extern "qsv extdedup" [
    --no-output
    --dupes-output
    --human-readable
    --memory-limit
    --quiet
    --help(-h)                # Print help
  ]

  export extern "qsv extsort" [
    --memory-limit
    --tmp-dir
    --jobs
    --no-headers
    --help(-h)                # Print help
  ]

  export extern "qsv explode" [
    --rename
    --output
    --no-headers
    --delimiter
    --help(-h)                # Print help
  ]

  export extern "qsv fetch" [
    --url-template
    --new-column
    --jql
    --jqlfile
    --pretty
    --rate-limit
    --timeout
    --http-header
    --max-retries
    --max-errors
    --store-error
    --cookies
    --user-agent
    --report
    --no-cache
    --mem-cache-size
    --disk-cache
    --disk-cache-dir
    --redis-cache
    --cache-error
    --flush-cache
    --output
    --no-headers
    --delimiter
    --progressbar
    --help(-h)                # Print help
  ]

  export extern "qsv fetchpost" [
    --new-column
    --jql
    --jqlfile
    --pretty
    --rate-limit
    --timeout
    --http-header
    --compress
    --max-retries
    --max-errors
    --store-error
    --cookies
    --user-agent
    --report
    --no-cache
    --mem-cache-size
    --disk-cache
    --disk-cache-dir
    --redis-cache
    --cache-error
    --flush-cache
    --output
    --no-headers
    --delimiter
    --progressbar
    --help(-h)                # Print help
  ]

  export extern "qsv fill" [
    --groupby
    --first
    --backfill
    --default
    --output
    --no-headers
    --delimiter
    --help(-h)                # Print help
  ]

  export extern "qsv fixlengths" [
    --length
    --insert
    --output
    --delimiter
    --help(-h)                # Print help
  ]

  export extern "qsv flatten" [
    --condense
    --field-separator
    --separator
    --no-headers
    --delimiter
    --help(-h)                # Print help
  ]

  export extern "qsv fmt" [
    --out-delimiter
    --crlf
    --ascii
    --quote
    --quote-always
    --quote-never
    --escape
    --no-final-newline
    --output
    --delimiter
    --help(-h)                # Print help
  ]

  export extern "qsv foreach" [
    --unify
    --new-column
    --dry-run
    --no-headers
    --delimiter
    --progressbar
    --help(-h)                # Print help
  ]

  export extern "qsv frequency" [
    --select
    --limit
    --unq-limit
    --lmt-threshold
    --pct-dec-places
    --other-sorted
    --other-text
    --asc
    --no-trim
    --ignore-case
    --jobs
    --output
    --no-headers
    --delimiter
    --memcheck
    --help(-h)                # Print help
  ]

  export extern "qsv geocode" [
    --new-column
    --rename
    --country
    --min-score
    --admin1
    --k_weight
    --formatstr
    --language
    --invalid-result
    --jobs
    --batch
    --timeout
    --cache-dir
    --languages
    --cities-url
    --force
    --output
    --delimiter
    --progressbar
    --help(-h)                # Print help
  ]

  export extern "qsv headers" [
    --just-names
    --intersect
    --trim
    --delimiter
    --help(-h)                # Print help
  ]

  export extern "qsv index" [
    --output
    --help(-h)                # Print help
  ]

  export extern "qsv input" [
    --quote
    --escape
    --no-quoting
    --quote-style
    --skip-lines
    --auto-skip
    --skip-lastlines
    --trim-headers
    --trim-fields
    --comment
    --encoding-errors
    --output
    --delimiter
    --help(-h)                # Print help
  ]

  export extern "qsv join" [
    --ignore-case
    --left-anti
    --left-semi
    --right
    --full
    --cross
    --nulls
    --output
    --no-headers
    --delimiter
    --help(-h)                # Print help
  ]

  export extern "qsv joinp" [
    --left
    --left-anti
    --left-semi
    --right
    --full
    --cross
    --coalesce
    --filter-left
    --filter-right
    --validate
    --nulls
    --streaming
    --try-parsedates
    --infer-len
    --low-memory
    --no-optimizations
    --ignore-errors
    --decimal-comma
    --asof
    --left_by
    --right_by
    --strategy
    --tolerance
    --sql-filter
    --datetime-format
    --date-format
    --time-format
    --float-precision
    --null-value
    --output
    --delimiter
    --quiet
    --help(-h)                # Print help
  ]

  export extern "qsv json" [
    --jaq
    --select
    --output
    --help(-h)                # Print help
  ]

  export extern "qsv jsonl" [
    --ignore-errors
    --jobs
    --batch
    --output
    --delimiter
    --help(-h)                # Print help
  ]

  export extern "qsv luau" [
    --no-globals
    --colindex
    --remap
    --begin
    --luau-path
    --max-errors
    --timeout
    --ckan-api
    --ckan-token
    --cache-dir
    --output
    --no-headers
    --delimiter
    --progressbar
    --help(-h)                # Print help
  ]

  export extern "qsv partition" [
    --filename
    --prefix-length
    --drop
    --no-headers
    --delimiter
    --help(-h)                # Print help
  ]

  export extern "qsv prompt" [
    --msg
    --filters
    --workdir
    --fd-output
    --save-fname
    --base-delay-ms
    --output
    --quiet
    --help(-h)                # Print help
  ]

  export extern "qsv pseudo" [
    --start
    --increment
    --formatstr
    --no-headers
    --delimiter
    --help(-h)                # Print help
  ]

  export extern "qsv py" [
    --helper
    --batch
    --output
    --no-headers
    --delimiter
    --progressbar
    --help(-h)                # Print help
  ]

  export extern "qsv rename" [
    --output
    --no-headers
    --delimiter
    --help(-h)                # Print help
  ]

  export extern "qsv replace" [
    --ignore-case
    --select
    --unicode
    --size-limit
    --dfa-size-limit
    --output
    --no-headers
    --delimiter
    --progressbar
    --quiet
    --help(-h)                # Print help
  ]

  export extern "qsv reverse" [
    --output
    --no-headers
    --delimiter
    --memcheck
    --help(-h)                # Print help
  ]

  export extern "qsv safenames" [
    --mode
    --reserved
    --prefix
    --output
    --delimiter
    --help(-h)                # Print help
  ]

  export extern "qsv sample" [
    --seed
    --rng
    --user-agent
    --timeout
    --output
    --no-headers
    --delimiter
    --help(-h)                # Print help
  ]

  export extern "qsv schema" [
    --enum-threshold
    --ignore-case
    --strict-dates
    --pattern-columns
    --date-whitelist
    --prefer-dmy
    --force
    --stdout
    --jobs
    --no-headers
    --delimiter
    --memcheck
    --help(-h)                # Print help
  ]

  export extern "qsv search" [
    --ignore-case
    --select
    --invert-match
    --unicode
    --flag
    --quick
    --preview-match
    --count
    --size-limit
    --dfa-size-limit
    --json
    --not-one
    --output
    --no-headers
    --delimiter
    --progressbar
    --quiet
    --help(-h)                # Print help
  ]

  export extern "qsv searchset" [
    --ignore-case
    --select
    --invert-match
    --unicode
    --flag
    --flag-matches-only
    --unmatched-output
    --quick
    --count
    --json
    --size-limit
    --dfa-size-limit
    --output
    --no-headers
    --delimiter
    --progressbar
    --quiet
    --help(-h)                # Print help
  ]

  export extern "qsv select" [
    --random
    --seed
    --sort
    --output
    --no-headers
    --delimiter
    --help(-h)                # Print help
  ]

  export extern "qsv slice" [
    --start
    --end
    --len
    --index
    --json
    --output
    --no-headers
    --delimiter
    --help(-h)                # Print help
  ]

  export extern "qsv snappy" [
    --user-agent
    --timeout
    --output
    --jobs
    --quiet
    --progressbar
    --help(-h)                # Print help
  ]

  export extern "qsv snappy compress" [
    --help(-h)                # Print help
  ]

  export extern "qsv snappy decompress" [
    --help(-h)                # Print help
  ]

  export extern "qsv snappy check" [
    --help(-h)                # Print help
  ]

  export extern "qsv snappy validate" [
    --help(-h)                # Print help
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "qsv snappy help" [
  ]

  export extern "qsv snappy help compress" [
  ]

  export extern "qsv snappy help decompress" [
  ]

  export extern "qsv snappy help check" [
  ]

  export extern "qsv snappy help validate" [
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "qsv snappy help help" [
  ]

  export extern "qsv sniff" [
    --sample
    --prefer-dmy
    --delimiter
    --quote
    --json
    --pretty-json
    --save-urlsample
    --timeout
    --user-agent
    --stats-types
    --no-infer
    --just-mime
    --quick
    --harvest-mode
    --progressbar
    --help(-h)                # Print help
  ]

  export extern "qsv sort" [
    --select
    --numeric
    --reverse
    --ignore-case
    --unique
    --random
    --seed
    --rng
    --jobs
    --faster
    --output
    --no-headers
    --delimiter
    --memcheck
    --help(-h)                # Print help
  ]

  export extern "qsv sortcheck" [
    --select
    --ignore-case
    --all
    --json
    --pretty-json
    --no-headers
    --delimiter
    --progressbar
    --help(-h)                # Print help
  ]

  export extern "qsv split" [
    --size
    --chunks
    --kb-size
    --jobs
    --filename
    --pad
    --no-headers
    --delimiter
    --quiet
    --help(-h)                # Print help
  ]

  export extern "qsv sqlp" [
    --format
    --try-parsedates
    --infer-len
    --streaming
    --low-memory
    --no-optimizations
    --truncate-ragged-lines
    --ignore-errors
    --rnull-values
    --decimal-comma
    --datetime-format
    --date-format
    --time-format
    --float-precision
    --wnull-value
    --compression
    --compress-level
    --statistics
    --output
    --delimiter
    --quiet
    --help(-h)                # Print help
  ]

  export extern "qsv stats" [
    --select
    --everything
    --typesonly
    --infer-boolean
    --mode
    --cardinality
    --median
    --mad
    --quartiles
    --round
    --nulls
    --infer-dates
    --prefer-dmy
    --force
    --jobs
    --stats-binout
    --cache-threshold
    --output
    --no-headers
    --delimiter
    --memcheck
    --help(-h)                # Print help
  ]

  export extern "qsv table" [
    --width
    --pad
    --align
    --condense
    --output
    --delimiter
    --memcheck
    --help(-h)                # Print help
  ]

  export extern "qsv to" [
    --print-package
    --dump
    --stats
    --stats-csv
    --quiet
    --schema
    --drop
    --evolve
    --pipe
    --separator
    --jobs
    --delimiter
    --help(-h)                # Print help
  ]

  export extern "qsv tojsonl" [
    --trim
    --no-boolean
    --jobs
    --batch
    --delimiter
    --output
    --memcheck
    --help(-h)                # Print help
  ]

  export extern "qsv transpose" [
    --multipass
    --output
    --delimiter
    --memcheck
    --help(-h)                # Print help
  ]

  export extern "qsv validate" [
    --trim
    --fail-fast
    --valid
    --invalid
    --json
    --pretty-json
    --valid-output
    --jobs
    --batch
    --timeout
    --no-headers
    --delimiter
    --progressbar
    --quiet
    --help(-h)                # Print help
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "qsv help" [
  ]

  export extern "qsv help apply" [
  ]

  export extern "qsv help apply operations" [
  ]

  export extern "qsv help apply emptyreplace" [
  ]

  export extern "qsv help apply dynfmt" [
  ]

  export extern "qsv help apply calcconv" [
  ]

  export extern "qsv help behead" [
  ]

  export extern "qsv help cat" [
  ]

  export extern "qsv help cat rows" [
  ]

  export extern "qsv help cat rowskey" [
  ]

  export extern "qsv help cat columns" [
  ]

  export extern "qsv help clipboard" [
  ]

  export extern "qsv help count" [
  ]

  export extern "qsv help datefmt" [
  ]

  export extern "qsv help dedup" [
  ]

  export extern "qsv help describegpt" [
  ]

  export extern "qsv help diff" [
  ]

  export extern "qsv help enum" [
  ]

  export extern "qsv help excel" [
  ]

  export extern "qsv help exclude" [
  ]

  export extern "qsv help extdedup" [
  ]

  export extern "qsv help extsort" [
  ]

  export extern "qsv help explode" [
  ]

  export extern "qsv help fetch" [
  ]

  export extern "qsv help fetchpost" [
  ]

  export extern "qsv help fill" [
  ]

  export extern "qsv help fixlengths" [
  ]

  export extern "qsv help flatten" [
  ]

  export extern "qsv help fmt" [
  ]

  export extern "qsv help foreach" [
  ]

  export extern "qsv help frequency" [
  ]

  export extern "qsv help geocode" [
  ]

  export extern "qsv help headers" [
  ]

  export extern "qsv help index" [
  ]

  export extern "qsv help input" [
  ]

  export extern "qsv help join" [
  ]

  export extern "qsv help joinp" [
  ]

  export extern "qsv help json" [
  ]

  export extern "qsv help jsonl" [
  ]

  export extern "qsv help luau" [
  ]

  export extern "qsv help partition" [
  ]

  export extern "qsv help prompt" [
  ]

  export extern "qsv help pseudo" [
  ]

  export extern "qsv help py" [
  ]

  export extern "qsv help rename" [
  ]

  export extern "qsv help replace" [
  ]

  export extern "qsv help reverse" [
  ]

  export extern "qsv help safenames" [
  ]

  export extern "qsv help sample" [
  ]

  export extern "qsv help schema" [
  ]

  export extern "qsv help search" [
  ]

  export extern "qsv help searchset" [
  ]

  export extern "qsv help select" [
  ]

  export extern "qsv help slice" [
  ]

  export extern "qsv help snappy" [
  ]

  export extern "qsv help snappy compress" [
  ]

  export extern "qsv help snappy decompress" [
  ]

  export extern "qsv help snappy check" [
  ]

  export extern "qsv help snappy validate" [
  ]

  export extern "qsv help sniff" [
  ]

  export extern "qsv help sort" [
  ]

  export extern "qsv help sortcheck" [
  ]

  export extern "qsv help split" [
  ]

  export extern "qsv help sqlp" [
  ]

  export extern "qsv help stats" [
  ]

  export extern "qsv help table" [
  ]

  export extern "qsv help to" [
  ]

  export extern "qsv help tojsonl" [
  ]

  export extern "qsv help transpose" [
  ]

  export extern "qsv help validate" [
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "qsv help help" [
  ]

}

export use completions *
