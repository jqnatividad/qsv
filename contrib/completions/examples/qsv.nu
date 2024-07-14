module completions {

  export extern qsv [
    --list
    --envlist
    --update
    --updatenow
    --version
    --help(-h)                # Print help
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

  # Print this message or the help of the given subcommand(s)
  export extern "qsv help" [
  ]

  export extern "qsv help clipboard" [
  ]

  export extern "qsv help count" [
  ]

  # Print this message or the help of the given subcommand(s)
  export extern "qsv help help" [
  ]

}

export use completions *
