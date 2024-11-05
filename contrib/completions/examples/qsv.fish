# Print an optspec for argparse to handle cmd's options that are independent of any subcommand.
function __fish_qsv_global_optspecs
	string join \n list envlist update updatenow version h/help
end

function __fish_qsv_needs_command
	# Figure out if the current invocation already has a command.
	set -l cmd (commandline -opc)
	set -e cmd[1]
	argparse -s (__fish_qsv_global_optspecs) -- $cmd 2>/dev/null
	or return
	if set -q argv[1]
		# Also print the command, so this can be used to figure out what it is.
		echo $argv[1]
		return 1
	end
	return 0
end

function __fish_qsv_using_subcommand
	set -l cmd (__fish_qsv_needs_command)
	test -z "$cmd"
	and return 1
	contains -- $cmd[1] $argv
end

complete -c qsv -n "__fish_qsv_needs_command" -l list
complete -c qsv -n "__fish_qsv_needs_command" -l envlist
complete -c qsv -n "__fish_qsv_needs_command" -l update
complete -c qsv -n "__fish_qsv_needs_command" -l updatenow
complete -c qsv -n "__fish_qsv_needs_command" -l version
complete -c qsv -n "__fish_qsv_needs_command" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_needs_command" -f -a "apply"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "behead"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "cat"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "clipboard"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "count"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "datefmt"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "dedup"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "describegpt"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "diff"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "edit"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "enum"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "excel"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "exclude"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "extdedup"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "extsort"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "explode"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "fetch"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "fetchpost"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "fill"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "fixlengths"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "flatten"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "fmt"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "foreach"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "frequency"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "geocode"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "headers"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "index"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "input"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "join"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "joinp"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "json"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "jsonl"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "lens"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "luau"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "partition"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "pro"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "prompt"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "pseudo"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "py"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "rename"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "replace"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "reverse"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "safenames"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "sample"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "schema"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "search"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "searchset"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "select"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "slice"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "snappy"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "sniff"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "sort"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "sortcheck"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "split"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "sqlp"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "stats"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "table"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "template"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "to"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "tojsonl"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "transpose"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "validate"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c qsv -n "__fish_qsv_using_subcommand apply; and not __fish_seen_subcommand_from operations emptyreplace dynfmt calcconv help" -l new-column
complete -c qsv -n "__fish_qsv_using_subcommand apply; and not __fish_seen_subcommand_from operations emptyreplace dynfmt calcconv help" -l rename
complete -c qsv -n "__fish_qsv_using_subcommand apply; and not __fish_seen_subcommand_from operations emptyreplace dynfmt calcconv help" -l comparand
complete -c qsv -n "__fish_qsv_using_subcommand apply; and not __fish_seen_subcommand_from operations emptyreplace dynfmt calcconv help" -l replacement
complete -c qsv -n "__fish_qsv_using_subcommand apply; and not __fish_seen_subcommand_from operations emptyreplace dynfmt calcconv help" -l formatstr
complete -c qsv -n "__fish_qsv_using_subcommand apply; and not __fish_seen_subcommand_from operations emptyreplace dynfmt calcconv help" -l jobs
complete -c qsv -n "__fish_qsv_using_subcommand apply; and not __fish_seen_subcommand_from operations emptyreplace dynfmt calcconv help" -l batch
complete -c qsv -n "__fish_qsv_using_subcommand apply; and not __fish_seen_subcommand_from operations emptyreplace dynfmt calcconv help" -l output
complete -c qsv -n "__fish_qsv_using_subcommand apply; and not __fish_seen_subcommand_from operations emptyreplace dynfmt calcconv help" -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand apply; and not __fish_seen_subcommand_from operations emptyreplace dynfmt calcconv help" -l delimiter
complete -c qsv -n "__fish_qsv_using_subcommand apply; and not __fish_seen_subcommand_from operations emptyreplace dynfmt calcconv help" -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand apply; and not __fish_seen_subcommand_from operations emptyreplace dynfmt calcconv help" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand apply; and not __fish_seen_subcommand_from operations emptyreplace dynfmt calcconv help" -f -a "operations"
complete -c qsv -n "__fish_qsv_using_subcommand apply; and not __fish_seen_subcommand_from operations emptyreplace dynfmt calcconv help" -f -a "emptyreplace"
complete -c qsv -n "__fish_qsv_using_subcommand apply; and not __fish_seen_subcommand_from operations emptyreplace dynfmt calcconv help" -f -a "dynfmt"
complete -c qsv -n "__fish_qsv_using_subcommand apply; and not __fish_seen_subcommand_from operations emptyreplace dynfmt calcconv help" -f -a "calcconv"
complete -c qsv -n "__fish_qsv_using_subcommand apply; and not __fish_seen_subcommand_from operations emptyreplace dynfmt calcconv help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from operations" -l new-column
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from operations" -l rename
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from operations" -l comparand
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from operations" -l replacement
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from operations" -l formatstr
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from operations" -l jobs
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from operations" -l batch
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from operations" -l output
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from operations" -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from operations" -l delimiter
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from operations" -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from operations" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from emptyreplace" -l new-column
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from emptyreplace" -l rename
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from emptyreplace" -l comparand
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from emptyreplace" -l replacement
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from emptyreplace" -l formatstr
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from emptyreplace" -l jobs
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from emptyreplace" -l batch
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from emptyreplace" -l output
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from emptyreplace" -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from emptyreplace" -l delimiter
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from emptyreplace" -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from emptyreplace" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from dynfmt" -l new-column
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from dynfmt" -l rename
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from dynfmt" -l comparand
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from dynfmt" -l replacement
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from dynfmt" -l formatstr
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from dynfmt" -l jobs
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from dynfmt" -l batch
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from dynfmt" -l output
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from dynfmt" -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from dynfmt" -l delimiter
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from dynfmt" -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from dynfmt" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from calcconv" -l new-column
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from calcconv" -l rename
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from calcconv" -l comparand
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from calcconv" -l replacement
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from calcconv" -l formatstr
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from calcconv" -l jobs
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from calcconv" -l batch
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from calcconv" -l output
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from calcconv" -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from calcconv" -l delimiter
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from calcconv" -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from calcconv" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from help" -f -a "operations"
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from help" -f -a "emptyreplace"
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from help" -f -a "dynfmt"
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from help" -f -a "calcconv"
complete -c qsv -n "__fish_qsv_using_subcommand apply; and __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c qsv -n "__fish_qsv_using_subcommand behead" -l flexible
complete -c qsv -n "__fish_qsv_using_subcommand behead" -l output
complete -c qsv -n "__fish_qsv_using_subcommand behead" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand cat; and not __fish_seen_subcommand_from rows rowskey columns help" -l output
complete -c qsv -n "__fish_qsv_using_subcommand cat; and not __fish_seen_subcommand_from rows rowskey columns help" -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand cat; and not __fish_seen_subcommand_from rows rowskey columns help" -l delimiter
complete -c qsv -n "__fish_qsv_using_subcommand cat; and not __fish_seen_subcommand_from rows rowskey columns help" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand cat; and not __fish_seen_subcommand_from rows rowskey columns help" -f -a "rows"
complete -c qsv -n "__fish_qsv_using_subcommand cat; and not __fish_seen_subcommand_from rows rowskey columns help" -f -a "rowskey"
complete -c qsv -n "__fish_qsv_using_subcommand cat; and not __fish_seen_subcommand_from rows rowskey columns help" -f -a "columns"
complete -c qsv -n "__fish_qsv_using_subcommand cat; and not __fish_seen_subcommand_from rows rowskey columns help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c qsv -n "__fish_qsv_using_subcommand cat; and __fish_seen_subcommand_from rows" -l flexible
complete -c qsv -n "__fish_qsv_using_subcommand cat; and __fish_seen_subcommand_from rows" -l output
complete -c qsv -n "__fish_qsv_using_subcommand cat; and __fish_seen_subcommand_from rows" -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand cat; and __fish_seen_subcommand_from rows" -l delimiter
complete -c qsv -n "__fish_qsv_using_subcommand cat; and __fish_seen_subcommand_from rows" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand cat; and __fish_seen_subcommand_from rowskey" -l group
complete -c qsv -n "__fish_qsv_using_subcommand cat; and __fish_seen_subcommand_from rowskey" -l group-name
complete -c qsv -n "__fish_qsv_using_subcommand cat; and __fish_seen_subcommand_from rowskey" -l output
complete -c qsv -n "__fish_qsv_using_subcommand cat; and __fish_seen_subcommand_from rowskey" -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand cat; and __fish_seen_subcommand_from rowskey" -l delimiter
complete -c qsv -n "__fish_qsv_using_subcommand cat; and __fish_seen_subcommand_from rowskey" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand cat; and __fish_seen_subcommand_from columns" -l pad
complete -c qsv -n "__fish_qsv_using_subcommand cat; and __fish_seen_subcommand_from columns" -l output
complete -c qsv -n "__fish_qsv_using_subcommand cat; and __fish_seen_subcommand_from columns" -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand cat; and __fish_seen_subcommand_from columns" -l delimiter
complete -c qsv -n "__fish_qsv_using_subcommand cat; and __fish_seen_subcommand_from columns" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand cat; and __fish_seen_subcommand_from help" -f -a "rows"
complete -c qsv -n "__fish_qsv_using_subcommand cat; and __fish_seen_subcommand_from help" -f -a "rowskey"
complete -c qsv -n "__fish_qsv_using_subcommand cat; and __fish_seen_subcommand_from help" -f -a "columns"
complete -c qsv -n "__fish_qsv_using_subcommand cat; and __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c qsv -n "__fish_qsv_using_subcommand clipboard" -l save
complete -c qsv -n "__fish_qsv_using_subcommand clipboard" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand count" -l human-readable
complete -c qsv -n "__fish_qsv_using_subcommand count" -l width
complete -c qsv -n "__fish_qsv_using_subcommand count" -l width-no-delims
complete -c qsv -n "__fish_qsv_using_subcommand count" -l json
complete -c qsv -n "__fish_qsv_using_subcommand count" -l no-polars
complete -c qsv -n "__fish_qsv_using_subcommand count" -l low-memory
complete -c qsv -n "__fish_qsv_using_subcommand count" -l flexible
complete -c qsv -n "__fish_qsv_using_subcommand count" -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand count" -l delimiter
complete -c qsv -n "__fish_qsv_using_subcommand count" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand datefmt" -l formatstr
complete -c qsv -n "__fish_qsv_using_subcommand datefmt" -l new-column
complete -c qsv -n "__fish_qsv_using_subcommand datefmt" -l rename
complete -c qsv -n "__fish_qsv_using_subcommand datefmt" -l prefer-dmy
complete -c qsv -n "__fish_qsv_using_subcommand datefmt" -l keep-zero-time
complete -c qsv -n "__fish_qsv_using_subcommand datefmt" -l input-tz
complete -c qsv -n "__fish_qsv_using_subcommand datefmt" -l output-tz
complete -c qsv -n "__fish_qsv_using_subcommand datefmt" -l default-tz
complete -c qsv -n "__fish_qsv_using_subcommand datefmt" -l utc
complete -c qsv -n "__fish_qsv_using_subcommand datefmt" -l zulu
complete -c qsv -n "__fish_qsv_using_subcommand datefmt" -l ts-resolution
complete -c qsv -n "__fish_qsv_using_subcommand datefmt" -l jobs
complete -c qsv -n "__fish_qsv_using_subcommand datefmt" -l batch
complete -c qsv -n "__fish_qsv_using_subcommand datefmt" -l output
complete -c qsv -n "__fish_qsv_using_subcommand datefmt" -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand datefmt" -l delimiter
complete -c qsv -n "__fish_qsv_using_subcommand datefmt" -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand datefmt" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand dedup" -l select
complete -c qsv -n "__fish_qsv_using_subcommand dedup" -l numeric
complete -c qsv -n "__fish_qsv_using_subcommand dedup" -l ignore-case
complete -c qsv -n "__fish_qsv_using_subcommand dedup" -l sorted
complete -c qsv -n "__fish_qsv_using_subcommand dedup" -l dupes-output
complete -c qsv -n "__fish_qsv_using_subcommand dedup" -l human-readable
complete -c qsv -n "__fish_qsv_using_subcommand dedup" -l jobs
complete -c qsv -n "__fish_qsv_using_subcommand dedup" -l output
complete -c qsv -n "__fish_qsv_using_subcommand dedup" -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand dedup" -l delimiter
complete -c qsv -n "__fish_qsv_using_subcommand dedup" -l quiet
complete -c qsv -n "__fish_qsv_using_subcommand dedup" -l memcheck
complete -c qsv -n "__fish_qsv_using_subcommand dedup" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l all
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l description
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l dictionary
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l tags
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l api-key
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l max-tokens
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l json
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l jsonl
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l prompt
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l prompt-file
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l base-url
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l model
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l timeout
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l user-agent
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l output
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -l quiet
complete -c qsv -n "__fish_qsv_using_subcommand describegpt" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand diff" -l no-headers-left
complete -c qsv -n "__fish_qsv_using_subcommand diff" -l no-headers-right
complete -c qsv -n "__fish_qsv_using_subcommand diff" -l no-headers-output
complete -c qsv -n "__fish_qsv_using_subcommand diff" -l delimiter-left
complete -c qsv -n "__fish_qsv_using_subcommand diff" -l delimiter-right
complete -c qsv -n "__fish_qsv_using_subcommand diff" -l delimiter-output
complete -c qsv -n "__fish_qsv_using_subcommand diff" -l key
complete -c qsv -n "__fish_qsv_using_subcommand diff" -l sort-columns
complete -c qsv -n "__fish_qsv_using_subcommand diff" -l drop-equal-fields
complete -c qsv -n "__fish_qsv_using_subcommand diff" -l jobs
complete -c qsv -n "__fish_qsv_using_subcommand diff" -l output
complete -c qsv -n "__fish_qsv_using_subcommand diff" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand edit" -l output
complete -c qsv -n "__fish_qsv_using_subcommand edit" -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand edit" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand enum" -l new-column
complete -c qsv -n "__fish_qsv_using_subcommand enum" -l start
complete -c qsv -n "__fish_qsv_using_subcommand enum" -l increment
complete -c qsv -n "__fish_qsv_using_subcommand enum" -l constant
complete -c qsv -n "__fish_qsv_using_subcommand enum" -l copy
complete -c qsv -n "__fish_qsv_using_subcommand enum" -l uuid4
complete -c qsv -n "__fish_qsv_using_subcommand enum" -l uuid7
complete -c qsv -n "__fish_qsv_using_subcommand enum" -l hash
complete -c qsv -n "__fish_qsv_using_subcommand enum" -l output
complete -c qsv -n "__fish_qsv_using_subcommand enum" -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand enum" -l delimiter
complete -c qsv -n "__fish_qsv_using_subcommand enum" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand excel" -l sheet
complete -c qsv -n "__fish_qsv_using_subcommand excel" -l header-row
complete -c qsv -n "__fish_qsv_using_subcommand excel" -l metadata
complete -c qsv -n "__fish_qsv_using_subcommand excel" -l error-format
complete -c qsv -n "__fish_qsv_using_subcommand excel" -l flexible
complete -c qsv -n "__fish_qsv_using_subcommand excel" -l trim
complete -c qsv -n "__fish_qsv_using_subcommand excel" -l date-format
complete -c qsv -n "__fish_qsv_using_subcommand excel" -l keep-zero-time
complete -c qsv -n "__fish_qsv_using_subcommand excel" -l table
complete -c qsv -n "__fish_qsv_using_subcommand excel" -l range
complete -c qsv -n "__fish_qsv_using_subcommand excel" -l jobs
complete -c qsv -n "__fish_qsv_using_subcommand excel" -l output
complete -c qsv -n "__fish_qsv_using_subcommand excel" -l delimiter
complete -c qsv -n "__fish_qsv_using_subcommand excel" -l quiet
complete -c qsv -n "__fish_qsv_using_subcommand excel" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand exclude" -l ignore-case
complete -c qsv -n "__fish_qsv_using_subcommand exclude" -s v
complete -c qsv -n "__fish_qsv_using_subcommand exclude" -l output
complete -c qsv -n "__fish_qsv_using_subcommand exclude" -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand exclude" -l delimiter
complete -c qsv -n "__fish_qsv_using_subcommand exclude" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand extdedup" -l select
complete -c qsv -n "__fish_qsv_using_subcommand extdedup" -l no-output
complete -c qsv -n "__fish_qsv_using_subcommand extdedup" -l dupes-output
complete -c qsv -n "__fish_qsv_using_subcommand extdedup" -l human-readable
complete -c qsv -n "__fish_qsv_using_subcommand extdedup" -l memory-limit
complete -c qsv -n "__fish_qsv_using_subcommand extdedup" -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand extdedup" -l delimiter
complete -c qsv -n "__fish_qsv_using_subcommand extdedup" -l quiet
complete -c qsv -n "__fish_qsv_using_subcommand extdedup" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand extsort" -l select
complete -c qsv -n "__fish_qsv_using_subcommand extsort" -l reverse
complete -c qsv -n "__fish_qsv_using_subcommand extsort" -l memory-limit
complete -c qsv -n "__fish_qsv_using_subcommand extsort" -l tmp-dir
complete -c qsv -n "__fish_qsv_using_subcommand extsort" -l jobs
complete -c qsv -n "__fish_qsv_using_subcommand extsort" -l delimiter
complete -c qsv -n "__fish_qsv_using_subcommand extsort" -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand extsort" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand explode" -l rename
complete -c qsv -n "__fish_qsv_using_subcommand explode" -l output
complete -c qsv -n "__fish_qsv_using_subcommand explode" -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand explode" -l delimiter
complete -c qsv -n "__fish_qsv_using_subcommand explode" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand fetch" -l url-template
complete -c qsv -n "__fish_qsv_using_subcommand fetch" -l new-column
complete -c qsv -n "__fish_qsv_using_subcommand fetch" -l jaq
complete -c qsv -n "__fish_qsv_using_subcommand fetch" -l jaqfile
complete -c qsv -n "__fish_qsv_using_subcommand fetch" -l pretty
complete -c qsv -n "__fish_qsv_using_subcommand fetch" -l rate-limit
complete -c qsv -n "__fish_qsv_using_subcommand fetch" -l timeout
complete -c qsv -n "__fish_qsv_using_subcommand fetch" -l http-header
complete -c qsv -n "__fish_qsv_using_subcommand fetch" -l max-retries
complete -c qsv -n "__fish_qsv_using_subcommand fetch" -l max-errors
complete -c qsv -n "__fish_qsv_using_subcommand fetch" -l store-error
complete -c qsv -n "__fish_qsv_using_subcommand fetch" -l cookies
complete -c qsv -n "__fish_qsv_using_subcommand fetch" -l user-agent
complete -c qsv -n "__fish_qsv_using_subcommand fetch" -l report
complete -c qsv -n "__fish_qsv_using_subcommand fetch" -l no-cache
complete -c qsv -n "__fish_qsv_using_subcommand fetch" -l mem-cache-size
complete -c qsv -n "__fish_qsv_using_subcommand fetch" -l disk-cache
complete -c qsv -n "__fish_qsv_using_subcommand fetch" -l disk-cache-dir
complete -c qsv -n "__fish_qsv_using_subcommand fetch" -l redis-cache
complete -c qsv -n "__fish_qsv_using_subcommand fetch" -l cache-error
complete -c qsv -n "__fish_qsv_using_subcommand fetch" -l flush-cache
complete -c qsv -n "__fish_qsv_using_subcommand fetch" -l output
complete -c qsv -n "__fish_qsv_using_subcommand fetch" -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand fetch" -l delimiter
complete -c qsv -n "__fish_qsv_using_subcommand fetch" -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand fetch" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand fetchpost" -l payload-tpl
complete -c qsv -n "__fish_qsv_using_subcommand fetchpost" -l content-type
complete -c qsv -n "__fish_qsv_using_subcommand fetchpost" -l new-column
complete -c qsv -n "__fish_qsv_using_subcommand fetchpost" -l jaq
complete -c qsv -n "__fish_qsv_using_subcommand fetchpost" -l jaqfile
complete -c qsv -n "__fish_qsv_using_subcommand fetchpost" -l pretty
complete -c qsv -n "__fish_qsv_using_subcommand fetchpost" -l rate-limit
complete -c qsv -n "__fish_qsv_using_subcommand fetchpost" -l timeout
complete -c qsv -n "__fish_qsv_using_subcommand fetchpost" -l http-header
complete -c qsv -n "__fish_qsv_using_subcommand fetchpost" -l compress
complete -c qsv -n "__fish_qsv_using_subcommand fetchpost" -l max-retries
complete -c qsv -n "__fish_qsv_using_subcommand fetchpost" -l max-errors
complete -c qsv -n "__fish_qsv_using_subcommand fetchpost" -l store-error
complete -c qsv -n "__fish_qsv_using_subcommand fetchpost" -l cookies
complete -c qsv -n "__fish_qsv_using_subcommand fetchpost" -l user-agent
complete -c qsv -n "__fish_qsv_using_subcommand fetchpost" -l report
complete -c qsv -n "__fish_qsv_using_subcommand fetchpost" -l no-cache
complete -c qsv -n "__fish_qsv_using_subcommand fetchpost" -l mem-cache-size
complete -c qsv -n "__fish_qsv_using_subcommand fetchpost" -l disk-cache
complete -c qsv -n "__fish_qsv_using_subcommand fetchpost" -l disk-cache-dir
complete -c qsv -n "__fish_qsv_using_subcommand fetchpost" -l redis-cache
complete -c qsv -n "__fish_qsv_using_subcommand fetchpost" -l cache-error
complete -c qsv -n "__fish_qsv_using_subcommand fetchpost" -l flush-cache
complete -c qsv -n "__fish_qsv_using_subcommand fetchpost" -l output
complete -c qsv -n "__fish_qsv_using_subcommand fetchpost" -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand fetchpost" -l delimiter
complete -c qsv -n "__fish_qsv_using_subcommand fetchpost" -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand fetchpost" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand fill" -l groupby
complete -c qsv -n "__fish_qsv_using_subcommand fill" -l first
complete -c qsv -n "__fish_qsv_using_subcommand fill" -l backfill
complete -c qsv -n "__fish_qsv_using_subcommand fill" -l default
complete -c qsv -n "__fish_qsv_using_subcommand fill" -l output
complete -c qsv -n "__fish_qsv_using_subcommand fill" -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand fill" -l delimiter
complete -c qsv -n "__fish_qsv_using_subcommand fill" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand fixlengths" -l length
complete -c qsv -n "__fish_qsv_using_subcommand fixlengths" -l insert
complete -c qsv -n "__fish_qsv_using_subcommand fixlengths" -l quote
complete -c qsv -n "__fish_qsv_using_subcommand fixlengths" -l escape
complete -c qsv -n "__fish_qsv_using_subcommand fixlengths" -l output
complete -c qsv -n "__fish_qsv_using_subcommand fixlengths" -l delimiter
complete -c qsv -n "__fish_qsv_using_subcommand fixlengths" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand flatten" -l condense
complete -c qsv -n "__fish_qsv_using_subcommand flatten" -l field-separator
complete -c qsv -n "__fish_qsv_using_subcommand flatten" -l separator
complete -c qsv -n "__fish_qsv_using_subcommand flatten" -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand flatten" -l delimiter
complete -c qsv -n "__fish_qsv_using_subcommand flatten" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand fmt" -l out-delimiter
complete -c qsv -n "__fish_qsv_using_subcommand fmt" -l crlf
complete -c qsv -n "__fish_qsv_using_subcommand fmt" -l ascii
complete -c qsv -n "__fish_qsv_using_subcommand fmt" -l quote
complete -c qsv -n "__fish_qsv_using_subcommand fmt" -l quote-always
complete -c qsv -n "__fish_qsv_using_subcommand fmt" -l quote-never
complete -c qsv -n "__fish_qsv_using_subcommand fmt" -l escape
complete -c qsv -n "__fish_qsv_using_subcommand fmt" -l no-final-newline
complete -c qsv -n "__fish_qsv_using_subcommand fmt" -l output
complete -c qsv -n "__fish_qsv_using_subcommand fmt" -l delimiter
complete -c qsv -n "__fish_qsv_using_subcommand fmt" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand foreach" -l unify
complete -c qsv -n "__fish_qsv_using_subcommand foreach" -l new-column
complete -c qsv -n "__fish_qsv_using_subcommand foreach" -l dry-run
complete -c qsv -n "__fish_qsv_using_subcommand foreach" -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand foreach" -l delimiter
complete -c qsv -n "__fish_qsv_using_subcommand foreach" -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand foreach" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand frequency" -l select
complete -c qsv -n "__fish_qsv_using_subcommand frequency" -l limit
complete -c qsv -n "__fish_qsv_using_subcommand frequency" -l unq-limit
complete -c qsv -n "__fish_qsv_using_subcommand frequency" -l lmt-threshold
complete -c qsv -n "__fish_qsv_using_subcommand frequency" -l pct-dec-places
complete -c qsv -n "__fish_qsv_using_subcommand frequency" -l other-sorted
complete -c qsv -n "__fish_qsv_using_subcommand frequency" -l other-text
complete -c qsv -n "__fish_qsv_using_subcommand frequency" -l asc
complete -c qsv -n "__fish_qsv_using_subcommand frequency" -l no-trim
complete -c qsv -n "__fish_qsv_using_subcommand frequency" -l no-nulls
complete -c qsv -n "__fish_qsv_using_subcommand frequency" -l ignore-case
complete -c qsv -n "__fish_qsv_using_subcommand frequency" -l stats-mode
complete -c qsv -n "__fish_qsv_using_subcommand frequency" -l all-unique-text
complete -c qsv -n "__fish_qsv_using_subcommand frequency" -l jobs
complete -c qsv -n "__fish_qsv_using_subcommand frequency" -l output
complete -c qsv -n "__fish_qsv_using_subcommand frequency" -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand frequency" -l delimiter
complete -c qsv -n "__fish_qsv_using_subcommand frequency" -l memcheck
complete -c qsv -n "__fish_qsv_using_subcommand frequency" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from suggest suggestnow reverse reversenow countryinfo countryinfonow index-load index-check index-update index-reset help" -l new-column
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from suggest suggestnow reverse reversenow countryinfo countryinfonow index-load index-check index-update index-reset help" -l rename
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from suggest suggestnow reverse reversenow countryinfo countryinfonow index-load index-check index-update index-reset help" -l country
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from suggest suggestnow reverse reversenow countryinfo countryinfonow index-load index-check index-update index-reset help" -l min-score
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from suggest suggestnow reverse reversenow countryinfo countryinfonow index-load index-check index-update index-reset help" -l admin1
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from suggest suggestnow reverse reversenow countryinfo countryinfonow index-load index-check index-update index-reset help" -l k_weight
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from suggest suggestnow reverse reversenow countryinfo countryinfonow index-load index-check index-update index-reset help" -l formatstr
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from suggest suggestnow reverse reversenow countryinfo countryinfonow index-load index-check index-update index-reset help" -l language
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from suggest suggestnow reverse reversenow countryinfo countryinfonow index-load index-check index-update index-reset help" -l invalid-result
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from suggest suggestnow reverse reversenow countryinfo countryinfonow index-load index-check index-update index-reset help" -l jobs
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from suggest suggestnow reverse reversenow countryinfo countryinfonow index-load index-check index-update index-reset help" -l batch
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from suggest suggestnow reverse reversenow countryinfo countryinfonow index-load index-check index-update index-reset help" -l timeout
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from suggest suggestnow reverse reversenow countryinfo countryinfonow index-load index-check index-update index-reset help" -l cache-dir
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from suggest suggestnow reverse reversenow countryinfo countryinfonow index-load index-check index-update index-reset help" -l languages
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from suggest suggestnow reverse reversenow countryinfo countryinfonow index-load index-check index-update index-reset help" -l cities-url
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from suggest suggestnow reverse reversenow countryinfo countryinfonow index-load index-check index-update index-reset help" -l force
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from suggest suggestnow reverse reversenow countryinfo countryinfonow index-load index-check index-update index-reset help" -l output
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from suggest suggestnow reverse reversenow countryinfo countryinfonow index-load index-check index-update index-reset help" -l delimiter
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from suggest suggestnow reverse reversenow countryinfo countryinfonow index-load index-check index-update index-reset help" -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from suggest suggestnow reverse reversenow countryinfo countryinfonow index-load index-check index-update index-reset help" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from suggest suggestnow reverse reversenow countryinfo countryinfonow index-load index-check index-update index-reset help" -f -a "suggest"
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from suggest suggestnow reverse reversenow countryinfo countryinfonow index-load index-check index-update index-reset help" -f -a "suggestnow"
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from suggest suggestnow reverse reversenow countryinfo countryinfonow index-load index-check index-update index-reset help" -f -a "reverse"
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from suggest suggestnow reverse reversenow countryinfo countryinfonow index-load index-check index-update index-reset help" -f -a "reversenow"
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from suggest suggestnow reverse reversenow countryinfo countryinfonow index-load index-check index-update index-reset help" -f -a "countryinfo"
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from suggest suggestnow reverse reversenow countryinfo countryinfonow index-load index-check index-update index-reset help" -f -a "countryinfonow"
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from suggest suggestnow reverse reversenow countryinfo countryinfonow index-load index-check index-update index-reset help" -f -a "index-load"
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from suggest suggestnow reverse reversenow countryinfo countryinfonow index-load index-check index-update index-reset help" -f -a "index-check"
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from suggest suggestnow reverse reversenow countryinfo countryinfonow index-load index-check index-update index-reset help" -f -a "index-update"
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from suggest suggestnow reverse reversenow countryinfo countryinfonow index-load index-check index-update index-reset help" -f -a "index-reset"
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and not __fish_seen_subcommand_from suggest suggestnow reverse reversenow countryinfo countryinfonow index-load index-check index-update index-reset help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggest" -l new-column
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggest" -l rename
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggest" -l country
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggest" -l min-score
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggest" -l admin1
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggest" -l k_weight
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggest" -l formatstr
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggest" -l language
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggest" -l invalid-result
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggest" -l jobs
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggest" -l batch
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggest" -l timeout
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggest" -l cache-dir
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggest" -l languages
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggest" -l cities-url
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggest" -l force
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggest" -l output
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggest" -l delimiter
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggest" -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggest" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggestnow" -l new-column
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggestnow" -l rename
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggestnow" -l country
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggestnow" -l min-score
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggestnow" -l admin1
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggestnow" -l k_weight
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggestnow" -l formatstr
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggestnow" -l language
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggestnow" -l invalid-result
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggestnow" -l jobs
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggestnow" -l batch
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggestnow" -l timeout
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggestnow" -l cache-dir
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggestnow" -l languages
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggestnow" -l cities-url
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggestnow" -l force
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggestnow" -l output
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggestnow" -l delimiter
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggestnow" -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from suggestnow" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reverse" -l new-column
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reverse" -l rename
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reverse" -l country
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reverse" -l min-score
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reverse" -l admin1
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reverse" -l k_weight
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reverse" -l formatstr
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reverse" -l language
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reverse" -l invalid-result
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reverse" -l jobs
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reverse" -l batch
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reverse" -l timeout
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reverse" -l cache-dir
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reverse" -l languages
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reverse" -l cities-url
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reverse" -l force
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reverse" -l output
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reverse" -l delimiter
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reverse" -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reverse" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reversenow" -l new-column
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reversenow" -l rename
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reversenow" -l country
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reversenow" -l min-score
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reversenow" -l admin1
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reversenow" -l k_weight
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reversenow" -l formatstr
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reversenow" -l language
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reversenow" -l invalid-result
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reversenow" -l jobs
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reversenow" -l batch
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reversenow" -l timeout
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reversenow" -l cache-dir
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reversenow" -l languages
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reversenow" -l cities-url
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reversenow" -l force
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reversenow" -l output
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reversenow" -l delimiter
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reversenow" -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from reversenow" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfo" -l new-column
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfo" -l rename
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfo" -l country
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfo" -l min-score
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfo" -l admin1
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfo" -l k_weight
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfo" -l formatstr
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfo" -l language
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfo" -l invalid-result
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfo" -l jobs
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfo" -l batch
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfo" -l timeout
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfo" -l cache-dir
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfo" -l languages
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfo" -l cities-url
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfo" -l force
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfo" -l output
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfo" -l delimiter
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfo" -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfo" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfonow" -l new-column
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfonow" -l rename
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfonow" -l country
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfonow" -l min-score
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfonow" -l admin1
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfonow" -l k_weight
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfonow" -l formatstr
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfonow" -l language
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfonow" -l invalid-result
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfonow" -l jobs
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfonow" -l batch
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfonow" -l timeout
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfonow" -l cache-dir
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfonow" -l languages
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfonow" -l cities-url
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfonow" -l force
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfonow" -l output
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfonow" -l delimiter
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfonow" -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from countryinfonow" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-load" -l new-column
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-load" -l rename
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-load" -l country
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-load" -l min-score
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-load" -l admin1
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-load" -l k_weight
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-load" -l formatstr
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-load" -l language
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-load" -l invalid-result
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-load" -l jobs
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-load" -l batch
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-load" -l timeout
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-load" -l cache-dir
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-load" -l languages
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-load" -l cities-url
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-load" -l force
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-load" -l output
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-load" -l delimiter
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-load" -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-load" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-check" -l new-column
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-check" -l rename
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-check" -l country
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-check" -l min-score
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-check" -l admin1
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-check" -l k_weight
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-check" -l formatstr
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-check" -l language
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-check" -l invalid-result
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-check" -l jobs
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-check" -l batch
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-check" -l timeout
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-check" -l cache-dir
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-check" -l languages
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-check" -l cities-url
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-check" -l force
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-check" -l output
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-check" -l delimiter
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-check" -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-check" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-update" -l new-column
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-update" -l rename
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-update" -l country
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-update" -l min-score
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-update" -l admin1
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-update" -l k_weight
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-update" -l formatstr
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-update" -l language
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-update" -l invalid-result
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-update" -l jobs
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-update" -l batch
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-update" -l timeout
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-update" -l cache-dir
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-update" -l languages
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-update" -l cities-url
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-update" -l force
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-update" -l output
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-update" -l delimiter
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-update" -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-update" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-reset" -l new-column
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-reset" -l rename
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-reset" -l country
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-reset" -l min-score
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-reset" -l admin1
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-reset" -l k_weight
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-reset" -l formatstr
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-reset" -l language
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-reset" -l invalid-result
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-reset" -l jobs
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-reset" -l batch
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-reset" -l timeout
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-reset" -l cache-dir
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-reset" -l languages
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-reset" -l cities-url
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-reset" -l force
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-reset" -l output
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-reset" -l delimiter
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-reset" -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from index-reset" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from help" -f -a "suggest"
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from help" -f -a "suggestnow"
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from help" -f -a "reverse"
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from help" -f -a "reversenow"
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from help" -f -a "countryinfo"
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from help" -f -a "countryinfonow"
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from help" -f -a "index-load"
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from help" -f -a "index-check"
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from help" -f -a "index-update"
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from help" -f -a "index-reset"
complete -c qsv -n "__fish_qsv_using_subcommand geocode; and __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c qsv -n "__fish_qsv_using_subcommand headers" -l just-names
complete -c qsv -n "__fish_qsv_using_subcommand headers" -l just-count
complete -c qsv -n "__fish_qsv_using_subcommand headers" -l intersect
complete -c qsv -n "__fish_qsv_using_subcommand headers" -l trim
complete -c qsv -n "__fish_qsv_using_subcommand headers" -l delimiter
complete -c qsv -n "__fish_qsv_using_subcommand headers" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand index" -l output
complete -c qsv -n "__fish_qsv_using_subcommand index" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand input" -l quote
complete -c qsv -n "__fish_qsv_using_subcommand input" -l escape
complete -c qsv -n "__fish_qsv_using_subcommand input" -l no-quoting
complete -c qsv -n "__fish_qsv_using_subcommand input" -l quote-style
complete -c qsv -n "__fish_qsv_using_subcommand input" -l skip-lines
complete -c qsv -n "__fish_qsv_using_subcommand input" -l auto-skip
complete -c qsv -n "__fish_qsv_using_subcommand input" -l skip-lastlines
complete -c qsv -n "__fish_qsv_using_subcommand input" -l trim-headers
complete -c qsv -n "__fish_qsv_using_subcommand input" -l trim-fields
complete -c qsv -n "__fish_qsv_using_subcommand input" -l comment
complete -c qsv -n "__fish_qsv_using_subcommand input" -l encoding-errors
complete -c qsv -n "__fish_qsv_using_subcommand input" -l output
complete -c qsv -n "__fish_qsv_using_subcommand input" -l delimiter
complete -c qsv -n "__fish_qsv_using_subcommand input" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand join" -l ignore-case
complete -c qsv -n "__fish_qsv_using_subcommand join" -l left
complete -c qsv -n "__fish_qsv_using_subcommand join" -l left-anti
complete -c qsv -n "__fish_qsv_using_subcommand join" -l left-semi
complete -c qsv -n "__fish_qsv_using_subcommand join" -l right
complete -c qsv -n "__fish_qsv_using_subcommand join" -l full
complete -c qsv -n "__fish_qsv_using_subcommand join" -l cross
complete -c qsv -n "__fish_qsv_using_subcommand join" -l nulls
complete -c qsv -n "__fish_qsv_using_subcommand join" -l output
complete -c qsv -n "__fish_qsv_using_subcommand join" -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand join" -l delimiter
complete -c qsv -n "__fish_qsv_using_subcommand join" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l left
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l left-anti
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l left-semi
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l right
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l full
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l cross
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l coalesce
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l filter-left
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l filter-right
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l validate
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l nulls
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l streaming
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l try-parsedates
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l infer-len
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l low-memory
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l no-optimizations
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l ignore-errors
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l decimal-comma
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l asof
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l left_by
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l right_by
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l strategy
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l tolerance
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l sql-filter
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l datetime-format
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l date-format
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l time-format
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l float-precision
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l null-value
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l output
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l delimiter
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -l quiet
complete -c qsv -n "__fish_qsv_using_subcommand joinp" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand json" -l jaq
complete -c qsv -n "__fish_qsv_using_subcommand json" -l select
complete -c qsv -n "__fish_qsv_using_subcommand json" -l output
complete -c qsv -n "__fish_qsv_using_subcommand json" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand jsonl" -l ignore-errors
complete -c qsv -n "__fish_qsv_using_subcommand jsonl" -l jobs
complete -c qsv -n "__fish_qsv_using_subcommand jsonl" -l batch
complete -c qsv -n "__fish_qsv_using_subcommand jsonl" -l output
complete -c qsv -n "__fish_qsv_using_subcommand jsonl" -l delimiter
complete -c qsv -n "__fish_qsv_using_subcommand jsonl" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand lens" -l delimiter
complete -c qsv -n "__fish_qsv_using_subcommand lens" -l tab-separated
complete -c qsv -n "__fish_qsv_using_subcommand lens" -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand lens" -l columns
complete -c qsv -n "__fish_qsv_using_subcommand lens" -l filter
complete -c qsv -n "__fish_qsv_using_subcommand lens" -l find
complete -c qsv -n "__fish_qsv_using_subcommand lens" -l ignore-case
complete -c qsv -n "__fish_qsv_using_subcommand lens" -l echo-column
complete -c qsv -n "__fish_qsv_using_subcommand lens" -l debug
complete -c qsv -n "__fish_qsv_using_subcommand lens" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand luau; and not __fish_seen_subcommand_from map filter help" -l no-globals
complete -c qsv -n "__fish_qsv_using_subcommand luau; and not __fish_seen_subcommand_from map filter help" -l colindex
complete -c qsv -n "__fish_qsv_using_subcommand luau; and not __fish_seen_subcommand_from map filter help" -l remap
complete -c qsv -n "__fish_qsv_using_subcommand luau; and not __fish_seen_subcommand_from map filter help" -l begin
complete -c qsv -n "__fish_qsv_using_subcommand luau; and not __fish_seen_subcommand_from map filter help" -l end
complete -c qsv -n "__fish_qsv_using_subcommand luau; and not __fish_seen_subcommand_from map filter help" -l luau-path
complete -c qsv -n "__fish_qsv_using_subcommand luau; and not __fish_seen_subcommand_from map filter help" -l max-errors
complete -c qsv -n "__fish_qsv_using_subcommand luau; and not __fish_seen_subcommand_from map filter help" -l timeout
complete -c qsv -n "__fish_qsv_using_subcommand luau; and not __fish_seen_subcommand_from map filter help" -l ckan-api
complete -c qsv -n "__fish_qsv_using_subcommand luau; and not __fish_seen_subcommand_from map filter help" -l ckan-token
complete -c qsv -n "__fish_qsv_using_subcommand luau; and not __fish_seen_subcommand_from map filter help" -l cache-dir
complete -c qsv -n "__fish_qsv_using_subcommand luau; and not __fish_seen_subcommand_from map filter help" -l output
complete -c qsv -n "__fish_qsv_using_subcommand luau; and not __fish_seen_subcommand_from map filter help" -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand luau; and not __fish_seen_subcommand_from map filter help" -l delimiter
complete -c qsv -n "__fish_qsv_using_subcommand luau; and not __fish_seen_subcommand_from map filter help" -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand luau; and not __fish_seen_subcommand_from map filter help" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand luau; and not __fish_seen_subcommand_from map filter help" -f -a "map"
complete -c qsv -n "__fish_qsv_using_subcommand luau; and not __fish_seen_subcommand_from map filter help" -f -a "filter"
complete -c qsv -n "__fish_qsv_using_subcommand luau; and not __fish_seen_subcommand_from map filter help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from map" -l no-globals
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from map" -l colindex
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from map" -l remap
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from map" -l begin
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from map" -l end
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from map" -l luau-path
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from map" -l max-errors
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from map" -l timeout
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from map" -l ckan-api
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from map" -l ckan-token
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from map" -l cache-dir
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from map" -l output
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from map" -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from map" -l delimiter
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from map" -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from map" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from filter" -l no-globals
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from filter" -l colindex
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from filter" -l remap
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from filter" -l begin
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from filter" -l end
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from filter" -l luau-path
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from filter" -l max-errors
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from filter" -l timeout
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from filter" -l ckan-api
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from filter" -l ckan-token
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from filter" -l cache-dir
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from filter" -l output
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from filter" -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from filter" -l delimiter
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from filter" -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from filter" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from help" -f -a "map"
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from help" -f -a "filter"
complete -c qsv -n "__fish_qsv_using_subcommand luau; and __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c qsv -n "__fish_qsv_using_subcommand partition" -l filename
complete -c qsv -n "__fish_qsv_using_subcommand partition" -l prefix-length
complete -c qsv -n "__fish_qsv_using_subcommand partition" -l drop
complete -c qsv -n "__fish_qsv_using_subcommand partition" -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand partition" -l delimiter
complete -c qsv -n "__fish_qsv_using_subcommand partition" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand pro; and not __fish_seen_subcommand_from lens workflow help" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand pro; and not __fish_seen_subcommand_from lens workflow help" -f -a "lens"
complete -c qsv -n "__fish_qsv_using_subcommand pro; and not __fish_seen_subcommand_from lens workflow help" -f -a "workflow"
complete -c qsv -n "__fish_qsv_using_subcommand pro; and not __fish_seen_subcommand_from lens workflow help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c qsv -n "__fish_qsv_using_subcommand pro; and __fish_seen_subcommand_from lens" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand pro; and __fish_seen_subcommand_from workflow" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand pro; and __fish_seen_subcommand_from help" -f -a "lens"
complete -c qsv -n "__fish_qsv_using_subcommand pro; and __fish_seen_subcommand_from help" -f -a "workflow"
complete -c qsv -n "__fish_qsv_using_subcommand pro; and __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c qsv -n "__fish_qsv_using_subcommand prompt" -l msg
complete -c qsv -n "__fish_qsv_using_subcommand prompt" -l filters
complete -c qsv -n "__fish_qsv_using_subcommand prompt" -l workdir
complete -c qsv -n "__fish_qsv_using_subcommand prompt" -l fd-output
complete -c qsv -n "__fish_qsv_using_subcommand prompt" -l save-fname
complete -c qsv -n "__fish_qsv_using_subcommand prompt" -l base-delay-ms
complete -c qsv -n "__fish_qsv_using_subcommand prompt" -l output
complete -c qsv -n "__fish_qsv_using_subcommand prompt" -l quiet
complete -c qsv -n "__fish_qsv_using_subcommand prompt" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand pseudo" -l start
complete -c qsv -n "__fish_qsv_using_subcommand pseudo" -l increment
complete -c qsv -n "__fish_qsv_using_subcommand pseudo" -l formatstr
complete -c qsv -n "__fish_qsv_using_subcommand pseudo" -l output
complete -c qsv -n "__fish_qsv_using_subcommand pseudo" -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand pseudo" -l delimiter
complete -c qsv -n "__fish_qsv_using_subcommand pseudo" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand py; and not __fish_seen_subcommand_from map filter help" -l helper
complete -c qsv -n "__fish_qsv_using_subcommand py; and not __fish_seen_subcommand_from map filter help" -l batch
complete -c qsv -n "__fish_qsv_using_subcommand py; and not __fish_seen_subcommand_from map filter help" -l output
complete -c qsv -n "__fish_qsv_using_subcommand py; and not __fish_seen_subcommand_from map filter help" -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand py; and not __fish_seen_subcommand_from map filter help" -l delimiter
complete -c qsv -n "__fish_qsv_using_subcommand py; and not __fish_seen_subcommand_from map filter help" -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand py; and not __fish_seen_subcommand_from map filter help" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand py; and not __fish_seen_subcommand_from map filter help" -f -a "map"
complete -c qsv -n "__fish_qsv_using_subcommand py; and not __fish_seen_subcommand_from map filter help" -f -a "filter"
complete -c qsv -n "__fish_qsv_using_subcommand py; and not __fish_seen_subcommand_from map filter help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c qsv -n "__fish_qsv_using_subcommand py; and __fish_seen_subcommand_from map" -l helper
complete -c qsv -n "__fish_qsv_using_subcommand py; and __fish_seen_subcommand_from map" -l batch
complete -c qsv -n "__fish_qsv_using_subcommand py; and __fish_seen_subcommand_from map" -l output
complete -c qsv -n "__fish_qsv_using_subcommand py; and __fish_seen_subcommand_from map" -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand py; and __fish_seen_subcommand_from map" -l delimiter
complete -c qsv -n "__fish_qsv_using_subcommand py; and __fish_seen_subcommand_from map" -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand py; and __fish_seen_subcommand_from map" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand py; and __fish_seen_subcommand_from filter" -l helper
complete -c qsv -n "__fish_qsv_using_subcommand py; and __fish_seen_subcommand_from filter" -l batch
complete -c qsv -n "__fish_qsv_using_subcommand py; and __fish_seen_subcommand_from filter" -l output
complete -c qsv -n "__fish_qsv_using_subcommand py; and __fish_seen_subcommand_from filter" -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand py; and __fish_seen_subcommand_from filter" -l delimiter
complete -c qsv -n "__fish_qsv_using_subcommand py; and __fish_seen_subcommand_from filter" -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand py; and __fish_seen_subcommand_from filter" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand py; and __fish_seen_subcommand_from help" -f -a "map"
complete -c qsv -n "__fish_qsv_using_subcommand py; and __fish_seen_subcommand_from help" -f -a "filter"
complete -c qsv -n "__fish_qsv_using_subcommand py; and __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c qsv -n "__fish_qsv_using_subcommand rename" -l output
complete -c qsv -n "__fish_qsv_using_subcommand rename" -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand rename" -l delimiter
complete -c qsv -n "__fish_qsv_using_subcommand rename" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand replace" -l ignore-case
complete -c qsv -n "__fish_qsv_using_subcommand replace" -l literal
complete -c qsv -n "__fish_qsv_using_subcommand replace" -l select
complete -c qsv -n "__fish_qsv_using_subcommand replace" -l unicode
complete -c qsv -n "__fish_qsv_using_subcommand replace" -l size-limit
complete -c qsv -n "__fish_qsv_using_subcommand replace" -l dfa-size-limit
complete -c qsv -n "__fish_qsv_using_subcommand replace" -l output
complete -c qsv -n "__fish_qsv_using_subcommand replace" -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand replace" -l delimiter
complete -c qsv -n "__fish_qsv_using_subcommand replace" -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand replace" -l quiet
complete -c qsv -n "__fish_qsv_using_subcommand replace" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand reverse" -l output
complete -c qsv -n "__fish_qsv_using_subcommand reverse" -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand reverse" -l delimiter
complete -c qsv -n "__fish_qsv_using_subcommand reverse" -l memcheck
complete -c qsv -n "__fish_qsv_using_subcommand reverse" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand safenames" -l mode
complete -c qsv -n "__fish_qsv_using_subcommand safenames" -l reserved
complete -c qsv -n "__fish_qsv_using_subcommand safenames" -l prefix
complete -c qsv -n "__fish_qsv_using_subcommand safenames" -l output
complete -c qsv -n "__fish_qsv_using_subcommand safenames" -l delimiter
complete -c qsv -n "__fish_qsv_using_subcommand safenames" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand sample" -l seed
complete -c qsv -n "__fish_qsv_using_subcommand sample" -l rng
complete -c qsv -n "__fish_qsv_using_subcommand sample" -l user-agent
complete -c qsv -n "__fish_qsv_using_subcommand sample" -l timeout
complete -c qsv -n "__fish_qsv_using_subcommand sample" -l max-size
complete -c qsv -n "__fish_qsv_using_subcommand sample" -l output
complete -c qsv -n "__fish_qsv_using_subcommand sample" -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand sample" -l delimiter
complete -c qsv -n "__fish_qsv_using_subcommand sample" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand schema" -l enum-threshold
complete -c qsv -n "__fish_qsv_using_subcommand schema" -l ignore-case
complete -c qsv -n "__fish_qsv_using_subcommand schema" -l strict-dates
complete -c qsv -n "__fish_qsv_using_subcommand schema" -l pattern-columns
complete -c qsv -n "__fish_qsv_using_subcommand schema" -l date-whitelist
complete -c qsv -n "__fish_qsv_using_subcommand schema" -l prefer-dmy
complete -c qsv -n "__fish_qsv_using_subcommand schema" -l force
complete -c qsv -n "__fish_qsv_using_subcommand schema" -l stdout
complete -c qsv -n "__fish_qsv_using_subcommand schema" -l jobs
complete -c qsv -n "__fish_qsv_using_subcommand schema" -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand schema" -l delimiter
complete -c qsv -n "__fish_qsv_using_subcommand schema" -l memcheck
complete -c qsv -n "__fish_qsv_using_subcommand schema" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand search" -l ignore-case
complete -c qsv -n "__fish_qsv_using_subcommand search" -l literal
complete -c qsv -n "__fish_qsv_using_subcommand search" -l select
complete -c qsv -n "__fish_qsv_using_subcommand search" -l invert-match
complete -c qsv -n "__fish_qsv_using_subcommand search" -l unicode
complete -c qsv -n "__fish_qsv_using_subcommand search" -l flag
complete -c qsv -n "__fish_qsv_using_subcommand search" -l quick
complete -c qsv -n "__fish_qsv_using_subcommand search" -l preview-match
complete -c qsv -n "__fish_qsv_using_subcommand search" -l count
complete -c qsv -n "__fish_qsv_using_subcommand search" -l size-limit
complete -c qsv -n "__fish_qsv_using_subcommand search" -l dfa-size-limit
complete -c qsv -n "__fish_qsv_using_subcommand search" -l json
complete -c qsv -n "__fish_qsv_using_subcommand search" -l not-one
complete -c qsv -n "__fish_qsv_using_subcommand search" -l output
complete -c qsv -n "__fish_qsv_using_subcommand search" -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand search" -l delimiter
complete -c qsv -n "__fish_qsv_using_subcommand search" -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand search" -l quiet
complete -c qsv -n "__fish_qsv_using_subcommand search" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand searchset" -l ignore-case
complete -c qsv -n "__fish_qsv_using_subcommand searchset" -l literal
complete -c qsv -n "__fish_qsv_using_subcommand searchset" -l select
complete -c qsv -n "__fish_qsv_using_subcommand searchset" -l invert-match
complete -c qsv -n "__fish_qsv_using_subcommand searchset" -l unicode
complete -c qsv -n "__fish_qsv_using_subcommand searchset" -l flag
complete -c qsv -n "__fish_qsv_using_subcommand searchset" -l flag-matches-only
complete -c qsv -n "__fish_qsv_using_subcommand searchset" -l unmatched-output
complete -c qsv -n "__fish_qsv_using_subcommand searchset" -l quick
complete -c qsv -n "__fish_qsv_using_subcommand searchset" -l count
complete -c qsv -n "__fish_qsv_using_subcommand searchset" -l json
complete -c qsv -n "__fish_qsv_using_subcommand searchset" -l size-limit
complete -c qsv -n "__fish_qsv_using_subcommand searchset" -l dfa-size-limit
complete -c qsv -n "__fish_qsv_using_subcommand searchset" -l not-one
complete -c qsv -n "__fish_qsv_using_subcommand searchset" -l output
complete -c qsv -n "__fish_qsv_using_subcommand searchset" -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand searchset" -l delimiter
complete -c qsv -n "__fish_qsv_using_subcommand searchset" -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand searchset" -l quiet
complete -c qsv -n "__fish_qsv_using_subcommand searchset" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand select" -l random
complete -c qsv -n "__fish_qsv_using_subcommand select" -l seed
complete -c qsv -n "__fish_qsv_using_subcommand select" -l sort
complete -c qsv -n "__fish_qsv_using_subcommand select" -l output
complete -c qsv -n "__fish_qsv_using_subcommand select" -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand select" -l delimiter
complete -c qsv -n "__fish_qsv_using_subcommand select" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand slice" -l start
complete -c qsv -n "__fish_qsv_using_subcommand slice" -l end
complete -c qsv -n "__fish_qsv_using_subcommand slice" -l len
complete -c qsv -n "__fish_qsv_using_subcommand slice" -l index
complete -c qsv -n "__fish_qsv_using_subcommand slice" -l json
complete -c qsv -n "__fish_qsv_using_subcommand slice" -l output
complete -c qsv -n "__fish_qsv_using_subcommand slice" -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand slice" -l delimiter
complete -c qsv -n "__fish_qsv_using_subcommand slice" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and not __fish_seen_subcommand_from compress decompress check validate help" -l user-agent
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and not __fish_seen_subcommand_from compress decompress check validate help" -l timeout
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and not __fish_seen_subcommand_from compress decompress check validate help" -l output
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and not __fish_seen_subcommand_from compress decompress check validate help" -l jobs
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and not __fish_seen_subcommand_from compress decompress check validate help" -l quiet
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and not __fish_seen_subcommand_from compress decompress check validate help" -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and not __fish_seen_subcommand_from compress decompress check validate help" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and not __fish_seen_subcommand_from compress decompress check validate help" -f -a "compress"
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and not __fish_seen_subcommand_from compress decompress check validate help" -f -a "decompress"
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and not __fish_seen_subcommand_from compress decompress check validate help" -f -a "check"
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and not __fish_seen_subcommand_from compress decompress check validate help" -f -a "validate"
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and not __fish_seen_subcommand_from compress decompress check validate help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and __fish_seen_subcommand_from compress" -l user-agent
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and __fish_seen_subcommand_from compress" -l timeout
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and __fish_seen_subcommand_from compress" -l output
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and __fish_seen_subcommand_from compress" -l jobs
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and __fish_seen_subcommand_from compress" -l quiet
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and __fish_seen_subcommand_from compress" -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and __fish_seen_subcommand_from compress" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and __fish_seen_subcommand_from decompress" -l user-agent
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and __fish_seen_subcommand_from decompress" -l timeout
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and __fish_seen_subcommand_from decompress" -l output
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and __fish_seen_subcommand_from decompress" -l jobs
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and __fish_seen_subcommand_from decompress" -l quiet
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and __fish_seen_subcommand_from decompress" -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and __fish_seen_subcommand_from decompress" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and __fish_seen_subcommand_from check" -l user-agent
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and __fish_seen_subcommand_from check" -l timeout
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and __fish_seen_subcommand_from check" -l output
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and __fish_seen_subcommand_from check" -l jobs
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and __fish_seen_subcommand_from check" -l quiet
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and __fish_seen_subcommand_from check" -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and __fish_seen_subcommand_from check" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and __fish_seen_subcommand_from validate" -l user-agent
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and __fish_seen_subcommand_from validate" -l timeout
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and __fish_seen_subcommand_from validate" -l output
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and __fish_seen_subcommand_from validate" -l jobs
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and __fish_seen_subcommand_from validate" -l quiet
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and __fish_seen_subcommand_from validate" -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and __fish_seen_subcommand_from validate" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and __fish_seen_subcommand_from help" -f -a "compress"
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and __fish_seen_subcommand_from help" -f -a "decompress"
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and __fish_seen_subcommand_from help" -f -a "check"
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and __fish_seen_subcommand_from help" -f -a "validate"
complete -c qsv -n "__fish_qsv_using_subcommand snappy; and __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c qsv -n "__fish_qsv_using_subcommand sniff" -l sample
complete -c qsv -n "__fish_qsv_using_subcommand sniff" -l prefer-dmy
complete -c qsv -n "__fish_qsv_using_subcommand sniff" -l delimiter
complete -c qsv -n "__fish_qsv_using_subcommand sniff" -l quote
complete -c qsv -n "__fish_qsv_using_subcommand sniff" -l json
complete -c qsv -n "__fish_qsv_using_subcommand sniff" -l pretty-json
complete -c qsv -n "__fish_qsv_using_subcommand sniff" -l save-urlsample
complete -c qsv -n "__fish_qsv_using_subcommand sniff" -l timeout
complete -c qsv -n "__fish_qsv_using_subcommand sniff" -l user-agent
complete -c qsv -n "__fish_qsv_using_subcommand sniff" -l stats-types
complete -c qsv -n "__fish_qsv_using_subcommand sniff" -l no-infer
complete -c qsv -n "__fish_qsv_using_subcommand sniff" -l just-mime
complete -c qsv -n "__fish_qsv_using_subcommand sniff" -l quick
complete -c qsv -n "__fish_qsv_using_subcommand sniff" -l harvest-mode
complete -c qsv -n "__fish_qsv_using_subcommand sniff" -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand sniff" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand sort" -l select
complete -c qsv -n "__fish_qsv_using_subcommand sort" -l numeric
complete -c qsv -n "__fish_qsv_using_subcommand sort" -l reverse
complete -c qsv -n "__fish_qsv_using_subcommand sort" -l ignore-case
complete -c qsv -n "__fish_qsv_using_subcommand sort" -l unique
complete -c qsv -n "__fish_qsv_using_subcommand sort" -l random
complete -c qsv -n "__fish_qsv_using_subcommand sort" -l seed
complete -c qsv -n "__fish_qsv_using_subcommand sort" -l rng
complete -c qsv -n "__fish_qsv_using_subcommand sort" -l jobs
complete -c qsv -n "__fish_qsv_using_subcommand sort" -l faster
complete -c qsv -n "__fish_qsv_using_subcommand sort" -l output
complete -c qsv -n "__fish_qsv_using_subcommand sort" -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand sort" -l delimiter
complete -c qsv -n "__fish_qsv_using_subcommand sort" -l memcheck
complete -c qsv -n "__fish_qsv_using_subcommand sort" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand sortcheck" -l select
complete -c qsv -n "__fish_qsv_using_subcommand sortcheck" -l ignore-case
complete -c qsv -n "__fish_qsv_using_subcommand sortcheck" -l all
complete -c qsv -n "__fish_qsv_using_subcommand sortcheck" -l json
complete -c qsv -n "__fish_qsv_using_subcommand sortcheck" -l pretty-json
complete -c qsv -n "__fish_qsv_using_subcommand sortcheck" -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand sortcheck" -l delimiter
complete -c qsv -n "__fish_qsv_using_subcommand sortcheck" -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand sortcheck" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand split" -l size
complete -c qsv -n "__fish_qsv_using_subcommand split" -l chunks
complete -c qsv -n "__fish_qsv_using_subcommand split" -l kb-size
complete -c qsv -n "__fish_qsv_using_subcommand split" -l jobs
complete -c qsv -n "__fish_qsv_using_subcommand split" -l filename
complete -c qsv -n "__fish_qsv_using_subcommand split" -l pad
complete -c qsv -n "__fish_qsv_using_subcommand split" -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand split" -l delimiter
complete -c qsv -n "__fish_qsv_using_subcommand split" -l quiet
complete -c qsv -n "__fish_qsv_using_subcommand split" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand sqlp" -l format
complete -c qsv -n "__fish_qsv_using_subcommand sqlp" -l try-parsedates
complete -c qsv -n "__fish_qsv_using_subcommand sqlp" -l infer-len
complete -c qsv -n "__fish_qsv_using_subcommand sqlp" -l cache-schema
complete -c qsv -n "__fish_qsv_using_subcommand sqlp" -l streaming
complete -c qsv -n "__fish_qsv_using_subcommand sqlp" -l low-memory
complete -c qsv -n "__fish_qsv_using_subcommand sqlp" -l no-optimizations
complete -c qsv -n "__fish_qsv_using_subcommand sqlp" -l truncate-ragged-lines
complete -c qsv -n "__fish_qsv_using_subcommand sqlp" -l ignore-errors
complete -c qsv -n "__fish_qsv_using_subcommand sqlp" -l rnull-values
complete -c qsv -n "__fish_qsv_using_subcommand sqlp" -l decimal-comma
complete -c qsv -n "__fish_qsv_using_subcommand sqlp" -l datetime-format
complete -c qsv -n "__fish_qsv_using_subcommand sqlp" -l date-format
complete -c qsv -n "__fish_qsv_using_subcommand sqlp" -l time-format
complete -c qsv -n "__fish_qsv_using_subcommand sqlp" -l float-precision
complete -c qsv -n "__fish_qsv_using_subcommand sqlp" -l wnull-value
complete -c qsv -n "__fish_qsv_using_subcommand sqlp" -l compression
complete -c qsv -n "__fish_qsv_using_subcommand sqlp" -l compress-level
complete -c qsv -n "__fish_qsv_using_subcommand sqlp" -l statistics
complete -c qsv -n "__fish_qsv_using_subcommand sqlp" -l output
complete -c qsv -n "__fish_qsv_using_subcommand sqlp" -l delimiter
complete -c qsv -n "__fish_qsv_using_subcommand sqlp" -l quiet
complete -c qsv -n "__fish_qsv_using_subcommand sqlp" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand stats" -l select
complete -c qsv -n "__fish_qsv_using_subcommand stats" -l everything
complete -c qsv -n "__fish_qsv_using_subcommand stats" -l typesonly
complete -c qsv -n "__fish_qsv_using_subcommand stats" -l infer-boolean
complete -c qsv -n "__fish_qsv_using_subcommand stats" -l mode
complete -c qsv -n "__fish_qsv_using_subcommand stats" -l cardinality
complete -c qsv -n "__fish_qsv_using_subcommand stats" -l median
complete -c qsv -n "__fish_qsv_using_subcommand stats" -l mad
complete -c qsv -n "__fish_qsv_using_subcommand stats" -l quartiles
complete -c qsv -n "__fish_qsv_using_subcommand stats" -l round
complete -c qsv -n "__fish_qsv_using_subcommand stats" -l nulls
complete -c qsv -n "__fish_qsv_using_subcommand stats" -l infer-dates
complete -c qsv -n "__fish_qsv_using_subcommand stats" -l dates-whitelist
complete -c qsv -n "__fish_qsv_using_subcommand stats" -l prefer-dmy
complete -c qsv -n "__fish_qsv_using_subcommand stats" -l force
complete -c qsv -n "__fish_qsv_using_subcommand stats" -l jobs
complete -c qsv -n "__fish_qsv_using_subcommand stats" -l stats-jsonl
complete -c qsv -n "__fish_qsv_using_subcommand stats" -l cache-threshold
complete -c qsv -n "__fish_qsv_using_subcommand stats" -l output
complete -c qsv -n "__fish_qsv_using_subcommand stats" -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand stats" -l delimiter
complete -c qsv -n "__fish_qsv_using_subcommand stats" -l memcheck
complete -c qsv -n "__fish_qsv_using_subcommand stats" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand table" -l width
complete -c qsv -n "__fish_qsv_using_subcommand table" -l pad
complete -c qsv -n "__fish_qsv_using_subcommand table" -l align
complete -c qsv -n "__fish_qsv_using_subcommand table" -l condense
complete -c qsv -n "__fish_qsv_using_subcommand table" -l output
complete -c qsv -n "__fish_qsv_using_subcommand table" -l delimiter
complete -c qsv -n "__fish_qsv_using_subcommand table" -l memcheck
complete -c qsv -n "__fish_qsv_using_subcommand table" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand template" -l template
complete -c qsv -n "__fish_qsv_using_subcommand template" -l template-file
complete -c qsv -n "__fish_qsv_using_subcommand template" -l outfilename
complete -c qsv -n "__fish_qsv_using_subcommand template" -l customfilter-error
complete -c qsv -n "__fish_qsv_using_subcommand template" -l output
complete -c qsv -n "__fish_qsv_using_subcommand template" -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand template" -l delimiter
complete -c qsv -n "__fish_qsv_using_subcommand template" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand to; and not __fish_seen_subcommand_from postgres sqlite xlsx datapackage help" -l print-package
complete -c qsv -n "__fish_qsv_using_subcommand to; and not __fish_seen_subcommand_from postgres sqlite xlsx datapackage help" -l dump
complete -c qsv -n "__fish_qsv_using_subcommand to; and not __fish_seen_subcommand_from postgres sqlite xlsx datapackage help" -l stats
complete -c qsv -n "__fish_qsv_using_subcommand to; and not __fish_seen_subcommand_from postgres sqlite xlsx datapackage help" -l stats-csv
complete -c qsv -n "__fish_qsv_using_subcommand to; and not __fish_seen_subcommand_from postgres sqlite xlsx datapackage help" -l quiet
complete -c qsv -n "__fish_qsv_using_subcommand to; and not __fish_seen_subcommand_from postgres sqlite xlsx datapackage help" -l schema
complete -c qsv -n "__fish_qsv_using_subcommand to; and not __fish_seen_subcommand_from postgres sqlite xlsx datapackage help" -l drop
complete -c qsv -n "__fish_qsv_using_subcommand to; and not __fish_seen_subcommand_from postgres sqlite xlsx datapackage help" -l evolve
complete -c qsv -n "__fish_qsv_using_subcommand to; and not __fish_seen_subcommand_from postgres sqlite xlsx datapackage help" -l pipe
complete -c qsv -n "__fish_qsv_using_subcommand to; and not __fish_seen_subcommand_from postgres sqlite xlsx datapackage help" -l separator
complete -c qsv -n "__fish_qsv_using_subcommand to; and not __fish_seen_subcommand_from postgres sqlite xlsx datapackage help" -l jobs
complete -c qsv -n "__fish_qsv_using_subcommand to; and not __fish_seen_subcommand_from postgres sqlite xlsx datapackage help" -l delimiter
complete -c qsv -n "__fish_qsv_using_subcommand to; and not __fish_seen_subcommand_from postgres sqlite xlsx datapackage help" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand to; and not __fish_seen_subcommand_from postgres sqlite xlsx datapackage help" -f -a "postgres"
complete -c qsv -n "__fish_qsv_using_subcommand to; and not __fish_seen_subcommand_from postgres sqlite xlsx datapackage help" -f -a "sqlite"
complete -c qsv -n "__fish_qsv_using_subcommand to; and not __fish_seen_subcommand_from postgres sqlite xlsx datapackage help" -f -a "xlsx"
complete -c qsv -n "__fish_qsv_using_subcommand to; and not __fish_seen_subcommand_from postgres sqlite xlsx datapackage help" -f -a "datapackage"
complete -c qsv -n "__fish_qsv_using_subcommand to; and not __fish_seen_subcommand_from postgres sqlite xlsx datapackage help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from postgres" -l print-package
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from postgres" -l dump
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from postgres" -l stats
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from postgres" -l stats-csv
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from postgres" -l quiet
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from postgres" -l schema
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from postgres" -l drop
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from postgres" -l evolve
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from postgres" -l pipe
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from postgres" -l separator
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from postgres" -l jobs
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from postgres" -l delimiter
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from postgres" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from sqlite" -l print-package
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from sqlite" -l dump
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from sqlite" -l stats
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from sqlite" -l stats-csv
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from sqlite" -l quiet
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from sqlite" -l schema
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from sqlite" -l drop
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from sqlite" -l evolve
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from sqlite" -l pipe
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from sqlite" -l separator
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from sqlite" -l jobs
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from sqlite" -l delimiter
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from sqlite" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from xlsx" -l print-package
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from xlsx" -l dump
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from xlsx" -l stats
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from xlsx" -l stats-csv
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from xlsx" -l quiet
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from xlsx" -l schema
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from xlsx" -l drop
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from xlsx" -l evolve
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from xlsx" -l pipe
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from xlsx" -l separator
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from xlsx" -l jobs
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from xlsx" -l delimiter
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from xlsx" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from datapackage" -l print-package
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from datapackage" -l dump
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from datapackage" -l stats
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from datapackage" -l stats-csv
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from datapackage" -l quiet
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from datapackage" -l schema
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from datapackage" -l drop
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from datapackage" -l evolve
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from datapackage" -l pipe
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from datapackage" -l separator
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from datapackage" -l jobs
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from datapackage" -l delimiter
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from datapackage" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from help" -f -a "postgres"
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from help" -f -a "sqlite"
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from help" -f -a "xlsx"
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from help" -f -a "datapackage"
complete -c qsv -n "__fish_qsv_using_subcommand to; and __fish_seen_subcommand_from help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c qsv -n "__fish_qsv_using_subcommand tojsonl" -l trim
complete -c qsv -n "__fish_qsv_using_subcommand tojsonl" -l no-boolean
complete -c qsv -n "__fish_qsv_using_subcommand tojsonl" -l jobs
complete -c qsv -n "__fish_qsv_using_subcommand tojsonl" -l batch
complete -c qsv -n "__fish_qsv_using_subcommand tojsonl" -l delimiter
complete -c qsv -n "__fish_qsv_using_subcommand tojsonl" -l output
complete -c qsv -n "__fish_qsv_using_subcommand tojsonl" -l memcheck
complete -c qsv -n "__fish_qsv_using_subcommand tojsonl" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand transpose" -l multipass
complete -c qsv -n "__fish_qsv_using_subcommand transpose" -l output
complete -c qsv -n "__fish_qsv_using_subcommand transpose" -l delimiter
complete -c qsv -n "__fish_qsv_using_subcommand transpose" -l memcheck
complete -c qsv -n "__fish_qsv_using_subcommand transpose" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand validate" -l trim
complete -c qsv -n "__fish_qsv_using_subcommand validate" -l fail-fast
complete -c qsv -n "__fish_qsv_using_subcommand validate" -l valid
complete -c qsv -n "__fish_qsv_using_subcommand validate" -l invalid
complete -c qsv -n "__fish_qsv_using_subcommand validate" -l json
complete -c qsv -n "__fish_qsv_using_subcommand validate" -l pretty-json
complete -c qsv -n "__fish_qsv_using_subcommand validate" -l valid-output
complete -c qsv -n "__fish_qsv_using_subcommand validate" -l jobs
complete -c qsv -n "__fish_qsv_using_subcommand validate" -l batch
complete -c qsv -n "__fish_qsv_using_subcommand validate" -l timeout
complete -c qsv -n "__fish_qsv_using_subcommand validate" -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand validate" -l delimiter
complete -c qsv -n "__fish_qsv_using_subcommand validate" -l progressbar
complete -c qsv -n "__fish_qsv_using_subcommand validate" -l quiet
complete -c qsv -n "__fish_qsv_using_subcommand validate" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard count datefmt dedup describegpt diff edit enum excel exclude extdedup extsort explode fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode headers index input join joinp json jsonl lens luau partition pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "apply"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard count datefmt dedup describegpt diff edit enum excel exclude extdedup extsort explode fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode headers index input join joinp json jsonl lens luau partition pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "behead"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard count datefmt dedup describegpt diff edit enum excel exclude extdedup extsort explode fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode headers index input join joinp json jsonl lens luau partition pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "cat"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard count datefmt dedup describegpt diff edit enum excel exclude extdedup extsort explode fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode headers index input join joinp json jsonl lens luau partition pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "clipboard"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard count datefmt dedup describegpt diff edit enum excel exclude extdedup extsort explode fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode headers index input join joinp json jsonl lens luau partition pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "count"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard count datefmt dedup describegpt diff edit enum excel exclude extdedup extsort explode fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode headers index input join joinp json jsonl lens luau partition pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "datefmt"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard count datefmt dedup describegpt diff edit enum excel exclude extdedup extsort explode fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode headers index input join joinp json jsonl lens luau partition pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "dedup"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard count datefmt dedup describegpt diff edit enum excel exclude extdedup extsort explode fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode headers index input join joinp json jsonl lens luau partition pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "describegpt"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard count datefmt dedup describegpt diff edit enum excel exclude extdedup extsort explode fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode headers index input join joinp json jsonl lens luau partition pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "diff"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard count datefmt dedup describegpt diff edit enum excel exclude extdedup extsort explode fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode headers index input join joinp json jsonl lens luau partition pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "edit"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard count datefmt dedup describegpt diff edit enum excel exclude extdedup extsort explode fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode headers index input join joinp json jsonl lens luau partition pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "enum"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard count datefmt dedup describegpt diff edit enum excel exclude extdedup extsort explode fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode headers index input join joinp json jsonl lens luau partition pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "excel"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard count datefmt dedup describegpt diff edit enum excel exclude extdedup extsort explode fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode headers index input join joinp json jsonl lens luau partition pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "exclude"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard count datefmt dedup describegpt diff edit enum excel exclude extdedup extsort explode fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode headers index input join joinp json jsonl lens luau partition pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "extdedup"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard count datefmt dedup describegpt diff edit enum excel exclude extdedup extsort explode fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode headers index input join joinp json jsonl lens luau partition pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "extsort"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard count datefmt dedup describegpt diff edit enum excel exclude extdedup extsort explode fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode headers index input join joinp json jsonl lens luau partition pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "explode"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard count datefmt dedup describegpt diff edit enum excel exclude extdedup extsort explode fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode headers index input join joinp json jsonl lens luau partition pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "fetch"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard count datefmt dedup describegpt diff edit enum excel exclude extdedup extsort explode fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode headers index input join joinp json jsonl lens luau partition pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "fetchpost"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard count datefmt dedup describegpt diff edit enum excel exclude extdedup extsort explode fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode headers index input join joinp json jsonl lens luau partition pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "fill"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard count datefmt dedup describegpt diff edit enum excel exclude extdedup extsort explode fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode headers index input join joinp json jsonl lens luau partition pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "fixlengths"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard count datefmt dedup describegpt diff edit enum excel exclude extdedup extsort explode fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode headers index input join joinp json jsonl lens luau partition pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "flatten"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard count datefmt dedup describegpt diff edit enum excel exclude extdedup extsort explode fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode headers index input join joinp json jsonl lens luau partition pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "fmt"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard count datefmt dedup describegpt diff edit enum excel exclude extdedup extsort explode fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode headers index input join joinp json jsonl lens luau partition pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "foreach"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard count datefmt dedup describegpt diff edit enum excel exclude extdedup extsort explode fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode headers index input join joinp json jsonl lens luau partition pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "frequency"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard count datefmt dedup describegpt diff edit enum excel exclude extdedup extsort explode fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode headers index input join joinp json jsonl lens luau partition pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "geocode"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard count datefmt dedup describegpt diff edit enum excel exclude extdedup extsort explode fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode headers index input join joinp json jsonl lens luau partition pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "headers"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard count datefmt dedup describegpt diff edit enum excel exclude extdedup extsort explode fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode headers index input join joinp json jsonl lens luau partition pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "index"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard count datefmt dedup describegpt diff edit enum excel exclude extdedup extsort explode fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode headers index input join joinp json jsonl lens luau partition pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "input"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard count datefmt dedup describegpt diff edit enum excel exclude extdedup extsort explode fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode headers index input join joinp json jsonl lens luau partition pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "join"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard count datefmt dedup describegpt diff edit enum excel exclude extdedup extsort explode fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode headers index input join joinp json jsonl lens luau partition pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "joinp"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard count datefmt dedup describegpt diff edit enum excel exclude extdedup extsort explode fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode headers index input join joinp json jsonl lens luau partition pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "json"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard count datefmt dedup describegpt diff edit enum excel exclude extdedup extsort explode fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode headers index input join joinp json jsonl lens luau partition pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "jsonl"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard count datefmt dedup describegpt diff edit enum excel exclude extdedup extsort explode fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode headers index input join joinp json jsonl lens luau partition pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "lens"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard count datefmt dedup describegpt diff edit enum excel exclude extdedup extsort explode fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode headers index input join joinp json jsonl lens luau partition pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "luau"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard count datefmt dedup describegpt diff edit enum excel exclude extdedup extsort explode fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode headers index input join joinp json jsonl lens luau partition pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "partition"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard count datefmt dedup describegpt diff edit enum excel exclude extdedup extsort explode fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode headers index input join joinp json jsonl lens luau partition pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "pro"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard count datefmt dedup describegpt diff edit enum excel exclude extdedup extsort explode fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode headers index input join joinp json jsonl lens luau partition pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "prompt"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard count datefmt dedup describegpt diff edit enum excel exclude extdedup extsort explode fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode headers index input join joinp json jsonl lens luau partition pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "pseudo"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard count datefmt dedup describegpt diff edit enum excel exclude extdedup extsort explode fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode headers index input join joinp json jsonl lens luau partition pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "py"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard count datefmt dedup describegpt diff edit enum excel exclude extdedup extsort explode fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode headers index input join joinp json jsonl lens luau partition pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "rename"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard count datefmt dedup describegpt diff edit enum excel exclude extdedup extsort explode fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode headers index input join joinp json jsonl lens luau partition pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "replace"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard count datefmt dedup describegpt diff edit enum excel exclude extdedup extsort explode fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode headers index input join joinp json jsonl lens luau partition pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "reverse"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard count datefmt dedup describegpt diff edit enum excel exclude extdedup extsort explode fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode headers index input join joinp json jsonl lens luau partition pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "safenames"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard count datefmt dedup describegpt diff edit enum excel exclude extdedup extsort explode fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode headers index input join joinp json jsonl lens luau partition pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "sample"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard count datefmt dedup describegpt diff edit enum excel exclude extdedup extsort explode fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode headers index input join joinp json jsonl lens luau partition pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "schema"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard count datefmt dedup describegpt diff edit enum excel exclude extdedup extsort explode fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode headers index input join joinp json jsonl lens luau partition pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "search"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard count datefmt dedup describegpt diff edit enum excel exclude extdedup extsort explode fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode headers index input join joinp json jsonl lens luau partition pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "searchset"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard count datefmt dedup describegpt diff edit enum excel exclude extdedup extsort explode fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode headers index input join joinp json jsonl lens luau partition pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "select"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard count datefmt dedup describegpt diff edit enum excel exclude extdedup extsort explode fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode headers index input join joinp json jsonl lens luau partition pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "slice"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard count datefmt dedup describegpt diff edit enum excel exclude extdedup extsort explode fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode headers index input join joinp json jsonl lens luau partition pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "snappy"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard count datefmt dedup describegpt diff edit enum excel exclude extdedup extsort explode fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode headers index input join joinp json jsonl lens luau partition pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "sniff"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard count datefmt dedup describegpt diff edit enum excel exclude extdedup extsort explode fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode headers index input join joinp json jsonl lens luau partition pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "sort"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard count datefmt dedup describegpt diff edit enum excel exclude extdedup extsort explode fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode headers index input join joinp json jsonl lens luau partition pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "sortcheck"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard count datefmt dedup describegpt diff edit enum excel exclude extdedup extsort explode fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode headers index input join joinp json jsonl lens luau partition pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "split"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard count datefmt dedup describegpt diff edit enum excel exclude extdedup extsort explode fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode headers index input join joinp json jsonl lens luau partition pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "sqlp"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard count datefmt dedup describegpt diff edit enum excel exclude extdedup extsort explode fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode headers index input join joinp json jsonl lens luau partition pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "stats"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard count datefmt dedup describegpt diff edit enum excel exclude extdedup extsort explode fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode headers index input join joinp json jsonl lens luau partition pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "table"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard count datefmt dedup describegpt diff edit enum excel exclude extdedup extsort explode fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode headers index input join joinp json jsonl lens luau partition pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "template"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard count datefmt dedup describegpt diff edit enum excel exclude extdedup extsort explode fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode headers index input join joinp json jsonl lens luau partition pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "to"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard count datefmt dedup describegpt diff edit enum excel exclude extdedup extsort explode fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode headers index input join joinp json jsonl lens luau partition pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "tojsonl"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard count datefmt dedup describegpt diff edit enum excel exclude extdedup extsort explode fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode headers index input join joinp json jsonl lens luau partition pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "transpose"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard count datefmt dedup describegpt diff edit enum excel exclude extdedup extsort explode fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode headers index input join joinp json jsonl lens luau partition pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "validate"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from apply behead cat clipboard count datefmt dedup describegpt diff edit enum excel exclude extdedup extsort explode fetch fetchpost fill fixlengths flatten fmt foreach frequency geocode headers index input join joinp json jsonl lens luau partition pro prompt pseudo py rename replace reverse safenames sample schema search searchset select slice snappy sniff sort sortcheck split sqlp stats table template to tojsonl transpose validate help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from apply" -f -a "operations"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from apply" -f -a "emptyreplace"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from apply" -f -a "dynfmt"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from apply" -f -a "calcconv"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from cat" -f -a "rows"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from cat" -f -a "rowskey"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from cat" -f -a "columns"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from geocode" -f -a "suggest"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from geocode" -f -a "suggestnow"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from geocode" -f -a "reverse"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from geocode" -f -a "reversenow"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from geocode" -f -a "countryinfo"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from geocode" -f -a "countryinfonow"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from geocode" -f -a "index-load"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from geocode" -f -a "index-check"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from geocode" -f -a "index-update"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from geocode" -f -a "index-reset"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from luau" -f -a "map"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from luau" -f -a "filter"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from pro" -f -a "lens"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from pro" -f -a "workflow"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from py" -f -a "map"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from py" -f -a "filter"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from snappy" -f -a "compress"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from snappy" -f -a "decompress"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from snappy" -f -a "check"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from snappy" -f -a "validate"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from to" -f -a "postgres"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from to" -f -a "sqlite"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from to" -f -a "xlsx"
complete -c qsv -n "__fish_qsv_using_subcommand help; and __fish_seen_subcommand_from to" -f -a "datapackage"
