# qsv fish completions

The `contrib/fish/qsv.fish` file is available for fish shell completions.

## Contributing

Currently the completions are incomplete, may differ per qsv version (e.g., may be outdated), and can be improved so feel free to contribute.

Here are some external docs:

-   https://fishshell.com/docs/current/completions.html#writing-your-own-completions
-   https://fishshell.com/docs/current/cmds/complete.html#complete-edit-command-specific-tab-completions

## Usage

Download the `qsv.fish` file and move it to `~/.config/fish/completions/` (e.g., using the `mv` command) so that you have a file `~/.config/fish/completions/qsv.fish` available. Then launch the fish shell and the completions should be available (tried this on WSL2 Ubuntu). You can modify the file, save it, then relaunch the fish shell for the updated completions.
