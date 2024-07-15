# qsv completions - bash, zsh, fish, powershell, nushell, fig, elvish

Generate shell completions for qsv including the following shells:

-   bash
-   zsh
-   powershell
-   fish
-   nushell
-   fig
-   elvish

There is potential to move this into a `qsv completions` command and/or add it as a feature since clap and several clap-related crates may need to be installed. Also this is currently a manual effort to keep commands up to date.

> Status as of qsv release v0.129.0: Completions for commands except for `applydp` and `generate` (`applydp` is specific to DataPusher+ and `generate` is not usually distributed with qsv anymore) are available. Completions may not account for file paths (you may need to explicitly use a relative path for example starting with `./` to begin file completions) and other potential changes that could be improved. Not all shells have been verified to work with the generated completions.

## Usage

You may use one of the completions in the `examples` folder or follow the following instructions to generate them.

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
