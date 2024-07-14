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
complete -c qsv -n "__fish_qsv_needs_command" -f -a "clipboard"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "count"
complete -c qsv -n "__fish_qsv_needs_command" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
complete -c qsv -n "__fish_qsv_using_subcommand clipboard" -l save
complete -c qsv -n "__fish_qsv_using_subcommand clipboard" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand count" -l human-readable
complete -c qsv -n "__fish_qsv_using_subcommand count" -l width
complete -c qsv -n "__fish_qsv_using_subcommand count" -l no-polars
complete -c qsv -n "__fish_qsv_using_subcommand count" -l low-memory
complete -c qsv -n "__fish_qsv_using_subcommand count" -l flexible
complete -c qsv -n "__fish_qsv_using_subcommand count" -l no-headers
complete -c qsv -n "__fish_qsv_using_subcommand count" -s h -l help -d 'Print help'
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from clipboard count help" -f -a "clipboard"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from clipboard count help" -f -a "count"
complete -c qsv -n "__fish_qsv_using_subcommand help; and not __fish_seen_subcommand_from clipboard count help" -f -a "help" -d 'Print this message or the help of the given subcommand(s)'
