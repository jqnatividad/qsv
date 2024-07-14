# qsv completions - bash, zsh, fish, powershell, nushell, fig, elvish

Generate shell completions for qsv including the following shells:

-   bash
-   zsh
-   fish
-   powershell
-   nushell
-   fig
-   elvish

There is potential to move this into a `qsv completions` command and/or add it as a feature since clap and several clap-related crates may need to be installed. Also this is currently a manual effort to keep commands up to date.

> Status: Based on qsv v0.129.0. Not all commands are available. See src/cmd for available commands.

## Usage

To generate completions for all shells into an examples folder run the `generate_examples.bash` script:

```bash
bash generate_examples.bash
```

To generate a completion manually run:

```bash
cargo run -- <shell>
```

Replace `<shell>` with any of the shells mentioned above.

The completions output should be printed to your terminal. You may redirect this to a file. For example for Bash completions:

```bash
cargo run -- bash > completions.bash
```

Then run them as your shell intends it to be ran.
