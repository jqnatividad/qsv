# qsv completions with Bashly

> These completions may be outdated by the time you're using them (see the version value in `src/bashly.yml` for the expected version to use it for). Update `src/bashly.yml` to ensure they're in sync with qsv's latest commands.

This is a foundation for CLI completions for [qsv](https://github.com/jqnatividad/qsv) using [Bashly](https://bashly.dannyb.co/).

## How to enable completions

Run `source completions.bash` on the `completions.bash` file in your terminal then try out the completions!

See Bashly's [Bash Completion docs](https://bashly.dannyb.co/advanced/bash-completion/) for more info.

## Development Setup

1. Install [Ruby](https://rubygems.org/pages/download).
2. Install bashly by running `gem install bashly` in your terminal.
3. Clone/download this directory to your system and `cd` into it.
4. If you're modifying the completions in `src/bashly.yml`, then you'll want to delete `completions.bash` before the next step (e.g., `rm completions.bash`).
5. Modify the `src/bashly.yml` as you see fit, save, then run `bashly add completions_script` which will generate a new `completions.bash` file.
6. Run the completions in your terminal with `source completions.bash`.
7. Try out the completions!
