const completion: Fig.Spec = {
  name: "qsv",
  description: "",
  subcommands: [
    {
      name: "apply",
      subcommands: [
        {
          name: "operations",
          options: [
            {
              name: ["-h", "--help"],
              description: "Print help",
            },
          ],
        },
        {
          name: "emptyreplace",
          options: [
            {
              name: ["-h", "--help"],
              description: "Print help",
            },
          ],
        },
        {
          name: "dynfmt",
          options: [
            {
              name: ["-h", "--help"],
              description: "Print help",
            },
          ],
        },
        {
          name: "calcconv",
          options: [
            {
              name: ["-h", "--help"],
              description: "Print help",
            },
          ],
        },
        {
          name: "help",
          description: "Print this message or the help of the given subcommand(s)",
          subcommands: [
            {
              name: "operations",
            },
            {
              name: "emptyreplace",
            },
            {
              name: "dynfmt",
            },
            {
              name: "calcconv",
            },
            {
              name: "help",
              description: "Print this message or the help of the given subcommand(s)",
            },
          ],
        },
      ],
      options: [
        {
          name: "--new-column",
        },
        {
          name: "--rename",
        },
        {
          name: "--comparand",
        },
        {
          name: "--replacement",
        },
        {
          name: "--formatstr",
        },
        {
          name: "--jobs",
        },
        {
          name: "--batch",
        },
        {
          name: "--output",
        },
        {
          name: "--no-headers",
        },
        {
          name: "--delimiter",
        },
        {
          name: "--progressbar",
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "behead",
      options: [
        {
          name: "--flexible",
        },
        {
          name: "--output",
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "cat",
      subcommands: [
        {
          name: "rows",
          options: [
            {
              name: "--flexible",
            },
            {
              name: ["-h", "--help"],
              description: "Print help",
            },
          ],
        },
        {
          name: "rowskey",
          options: [
            {
              name: "--group",
            },
            {
              name: "--group-name",
            },
            {
              name: ["-h", "--help"],
              description: "Print help",
            },
          ],
        },
        {
          name: "columns",
          options: [
            {
              name: "--pad",
            },
            {
              name: ["-h", "--help"],
              description: "Print help",
            },
          ],
        },
        {
          name: "help",
          description: "Print this message or the help of the given subcommand(s)",
          subcommands: [
            {
              name: "rows",
            },
            {
              name: "rowskey",
            },
            {
              name: "columns",
            },
            {
              name: "help",
              description: "Print this message or the help of the given subcommand(s)",
            },
          ],
        },
      ],
      options: [
        {
          name: "--output",
        },
        {
          name: "--no-headers",
        },
        {
          name: "--delimiter",
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "clipboard",
      options: [
        {
          name: "--save",
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "count",
      options: [
        {
          name: "--human-readable",
        },
        {
          name: "--width",
        },
        {
          name: "--no-polars",
        },
        {
          name: "--low-memory",
        },
        {
          name: "--flexible",
        },
        {
          name: "--no-headers",
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "datefmt",
      options: [
        {
          name: "--formatstr",
        },
        {
          name: "--new-column",
        },
        {
          name: "--rename",
        },
        {
          name: "--prefer-dmy",
        },
        {
          name: "--keep-zero-time",
        },
        {
          name: "--input-tz",
        },
        {
          name: "--output-tz",
        },
        {
          name: "--default-tz",
        },
        {
          name: "--utc",
        },
        {
          name: "--zulu",
        },
        {
          name: "--ts-resolution",
        },
        {
          name: "--jobs",
        },
        {
          name: "--batch",
        },
        {
          name: "--output",
        },
        {
          name: "--no-headers",
        },
        {
          name: "--delimiter",
        },
        {
          name: "--progressbar",
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "dedup",
      options: [
        {
          name: "--select",
        },
        {
          name: "--numeric",
        },
        {
          name: "--ignore-case",
        },
        {
          name: "--sorted",
        },
        {
          name: "--dupes-output",
        },
        {
          name: "--human-readable",
        },
        {
          name: "--jobs",
        },
        {
          name: "--output",
        },
        {
          name: "--no-headers",
        },
        {
          name: "--delimiter",
        },
        {
          name: "--quiet",
        },
        {
          name: "--memcheck",
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "describegpt",
      options: [
        {
          name: "--all",
        },
        {
          name: "--description",
        },
        {
          name: "--dictionary",
        },
        {
          name: "--tags",
        },
        {
          name: "--api-key",
        },
        {
          name: "--max-tokens",
        },
        {
          name: "--json",
        },
        {
          name: "--jsonl",
        },
        {
          name: "--prompt",
        },
        {
          name: "--prompt-file",
        },
        {
          name: "--base-url",
        },
        {
          name: "--model",
        },
        {
          name: "--timeout",
        },
        {
          name: "--user-agent",
        },
        {
          name: "--output",
        },
        {
          name: "--quiet",
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "diff",
      options: [
        {
          name: "--no-headers-left",
        },
        {
          name: "--no-headers-right",
        },
        {
          name: "--no-headers-output",
        },
        {
          name: "--delimiter-left",
        },
        {
          name: "--delimiter-right",
        },
        {
          name: "--delimiter-output",
        },
        {
          name: "--key",
        },
        {
          name: "--sort-columns",
        },
        {
          name: "--jobs",
        },
        {
          name: "--output",
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "enum",
      options: [
        {
          name: "--new-column",
        },
        {
          name: "--start",
        },
        {
          name: "--increment",
        },
        {
          name: "--constant",
        },
        {
          name: "--copy",
        },
        {
          name: "--uuid4",
        },
        {
          name: "--uuid7",
        },
        {
          name: "--hash",
        },
        {
          name: "--output",
        },
        {
          name: "--no-headers",
        },
        {
          name: "--delimiter",
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "excel",
      options: [
        {
          name: "--sheet",
        },
        {
          name: "--metadata",
        },
        {
          name: "--error-format",
        },
        {
          name: "--flexible",
        },
        {
          name: "--trim",
        },
        {
          name: "--date-format",
        },
        {
          name: "--keep-zero-time",
        },
        {
          name: "--range",
        },
        {
          name: "--jobs",
        },
        {
          name: "--output",
        },
        {
          name: "--delimiter",
        },
        {
          name: "--quiet",
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "exclude",
      options: [
        {
          name: "--ignore-case",
        },
        {
          name: "-v",
        },
        {
          name: "--output",
        },
        {
          name: "--no-headers",
        },
        {
          name: "--delimiter",
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "extdedup",
      options: [
        {
          name: "--no-output",
        },
        {
          name: "--dupes-output",
        },
        {
          name: "--human-readable",
        },
        {
          name: "--memory-limit",
        },
        {
          name: "--quiet",
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "extsort",
      options: [
        {
          name: "--memory-limit",
        },
        {
          name: "--tmp-dir",
        },
        {
          name: "--jobs",
        },
        {
          name: "--no-headers",
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "explode",
      options: [
        {
          name: "--rename",
        },
        {
          name: "--output",
        },
        {
          name: "--no-headers",
        },
        {
          name: "--delimiter",
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "fetch",
      options: [
        {
          name: "--url-template",
        },
        {
          name: "--new-column",
        },
        {
          name: "--jql",
        },
        {
          name: "--jqlfile",
        },
        {
          name: "--pretty",
        },
        {
          name: "--rate-limit",
        },
        {
          name: "--timeout",
        },
        {
          name: "--http-header",
        },
        {
          name: "--max-retries",
        },
        {
          name: "--max-errors",
        },
        {
          name: "--store-error",
        },
        {
          name: "--cookies",
        },
        {
          name: "--user-agent",
        },
        {
          name: "--report",
        },
        {
          name: "--no-cache",
        },
        {
          name: "--mem-cache-size",
        },
        {
          name: "--disk-cache",
        },
        {
          name: "--disk-cache-dir",
        },
        {
          name: "--redis-cache",
        },
        {
          name: "--cache-error",
        },
        {
          name: "--flush-cache",
        },
        {
          name: "--output",
        },
        {
          name: "--no-headers",
        },
        {
          name: "--delimiter",
        },
        {
          name: "--progressbar",
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "fetchpost",
      options: [
        {
          name: "--new-column",
        },
        {
          name: "--jql",
        },
        {
          name: "--jqlfile",
        },
        {
          name: "--pretty",
        },
        {
          name: "--rate-limit",
        },
        {
          name: "--timeout",
        },
        {
          name: "--http-header",
        },
        {
          name: "--compress",
        },
        {
          name: "--max-retries",
        },
        {
          name: "--max-errors",
        },
        {
          name: "--store-error",
        },
        {
          name: "--cookies",
        },
        {
          name: "--user-agent",
        },
        {
          name: "--report",
        },
        {
          name: "--no-cache",
        },
        {
          name: "--mem-cache-size",
        },
        {
          name: "--disk-cache",
        },
        {
          name: "--disk-cache-dir",
        },
        {
          name: "--redis-cache",
        },
        {
          name: "--cache-error",
        },
        {
          name: "--flush-cache",
        },
        {
          name: "--output",
        },
        {
          name: "--no-headers",
        },
        {
          name: "--delimiter",
        },
        {
          name: "--progressbar",
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "fill",
      options: [
        {
          name: "--groupby",
        },
        {
          name: "--first",
        },
        {
          name: "--backfill",
        },
        {
          name: "--default",
        },
        {
          name: "--output",
        },
        {
          name: "--no-headers",
        },
        {
          name: "--delimiter",
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "fixlengths",
      options: [
        {
          name: "--length",
        },
        {
          name: "--insert",
        },
        {
          name: "--output",
        },
        {
          name: "--delimiter",
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "flatten",
      options: [
        {
          name: "--condense",
        },
        {
          name: "--field-separator",
        },
        {
          name: "--separator",
        },
        {
          name: "--no-headers",
        },
        {
          name: "--delimiter",
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "fmt",
      options: [
        {
          name: "--out-delimiter",
        },
        {
          name: "--crlf",
        },
        {
          name: "--ascii",
        },
        {
          name: "--quote",
        },
        {
          name: "--quote-always",
        },
        {
          name: "--quote-never",
        },
        {
          name: "--escape",
        },
        {
          name: "--no-final-newline",
        },
        {
          name: "--output",
        },
        {
          name: "--delimiter",
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "foreach",
      options: [
        {
          name: "--unify",
        },
        {
          name: "--new-column",
        },
        {
          name: "--dry-run",
        },
        {
          name: "--no-headers",
        },
        {
          name: "--delimiter",
        },
        {
          name: "--progressbar",
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "frequency",
      options: [
        {
          name: "--select",
        },
        {
          name: "--limit",
        },
        {
          name: "--unq-limit",
        },
        {
          name: "--lmt-threshold",
        },
        {
          name: "--pct-dec-places",
        },
        {
          name: "--other-sorted",
        },
        {
          name: "--other-text",
        },
        {
          name: "--asc",
        },
        {
          name: "--no-trim",
        },
        {
          name: "--ignore-case",
        },
        {
          name: "--jobs",
        },
        {
          name: "--output",
        },
        {
          name: "--no-headers",
        },
        {
          name: "--delimiter",
        },
        {
          name: "--memcheck",
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "geocode",
      options: [
        {
          name: "--new-column",
        },
        {
          name: "--rename",
        },
        {
          name: "--country",
        },
        {
          name: "--min-score",
        },
        {
          name: "--admin1",
        },
        {
          name: "--k_weight",
        },
        {
          name: "--formatstr",
        },
        {
          name: "--language",
        },
        {
          name: "--invalid-result",
        },
        {
          name: "--jobs",
        },
        {
          name: "--batch",
        },
        {
          name: "--timeout",
        },
        {
          name: "--cache-dir",
        },
        {
          name: "--languages",
        },
        {
          name: "--cities-url",
        },
        {
          name: "--force",
        },
        {
          name: "--output",
        },
        {
          name: "--delimiter",
        },
        {
          name: "--progressbar",
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "headers",
      options: [
        {
          name: "--just-names",
        },
        {
          name: "--intersect",
        },
        {
          name: "--trim",
        },
        {
          name: "--delimiter",
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "index",
      options: [
        {
          name: "--output",
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "input",
      options: [
        {
          name: "--quote",
        },
        {
          name: "--escape",
        },
        {
          name: "--no-quoting",
        },
        {
          name: "--quote-style",
        },
        {
          name: "--skip-lines",
        },
        {
          name: "--auto-skip",
        },
        {
          name: "--skip-lastlines",
        },
        {
          name: "--trim-headers",
        },
        {
          name: "--trim-fields",
        },
        {
          name: "--comment",
        },
        {
          name: "--encoding-errors",
        },
        {
          name: "--output",
        },
        {
          name: "--delimiter",
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "join",
      options: [
        {
          name: "--ignore-case",
        },
        {
          name: "--left-anti",
        },
        {
          name: "--left-semi",
        },
        {
          name: "--right",
        },
        {
          name: "--full",
        },
        {
          name: "--cross",
        },
        {
          name: "--nulls",
        },
        {
          name: "--output",
        },
        {
          name: "--no-headers",
        },
        {
          name: "--delimiter",
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "joinp",
      options: [
        {
          name: "--left",
        },
        {
          name: "--left-anti",
        },
        {
          name: "--left-semi",
        },
        {
          name: "--right",
        },
        {
          name: "--full",
        },
        {
          name: "--cross",
        },
        {
          name: "--coalesce",
        },
        {
          name: "--filter-left",
        },
        {
          name: "--filter-right",
        },
        {
          name: "--validate",
        },
        {
          name: "--nulls",
        },
        {
          name: "--streaming",
        },
        {
          name: "--try-parsedates",
        },
        {
          name: "--infer-len",
        },
        {
          name: "--low-memory",
        },
        {
          name: "--no-optimizations",
        },
        {
          name: "--ignore-errors",
        },
        {
          name: "--decimal-comma",
        },
        {
          name: "--asof",
        },
        {
          name: "--left_by",
        },
        {
          name: "--right_by",
        },
        {
          name: "--strategy",
        },
        {
          name: "--tolerance",
        },
        {
          name: "--sql-filter",
        },
        {
          name: "--datetime-format",
        },
        {
          name: "--date-format",
        },
        {
          name: "--time-format",
        },
        {
          name: "--float-precision",
        },
        {
          name: "--null-value",
        },
        {
          name: "--output",
        },
        {
          name: "--delimiter",
        },
        {
          name: "--quiet",
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "json",
      options: [
        {
          name: "--jaq",
        },
        {
          name: "--output",
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "jsonl",
      options: [
        {
          name: "--ignore-errors",
        },
        {
          name: "--jobs",
        },
        {
          name: "--batch",
        },
        {
          name: "--output",
        },
        {
          name: "--delimiter",
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "luau",
      options: [
        {
          name: "--no-globals",
        },
        {
          name: "--colindex",
        },
        {
          name: "--remap",
        },
        {
          name: "--begin",
        },
        {
          name: "--luau-path",
        },
        {
          name: "--max-errors",
        },
        {
          name: "--timeout",
        },
        {
          name: "--ckan-api",
        },
        {
          name: "--ckan-token",
        },
        {
          name: "--cache-dir",
        },
        {
          name: "--output",
        },
        {
          name: "--no-headers",
        },
        {
          name: "--delimiter",
        },
        {
          name: "--progressbar",
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "partition",
      options: [
        {
          name: "--filename",
        },
        {
          name: "--prefix-length",
        },
        {
          name: "--drop",
        },
        {
          name: "--no-headers",
        },
        {
          name: "--delimiter",
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "prompt",
      options: [
        {
          name: "--msg",
        },
        {
          name: "--filters",
        },
        {
          name: "--workdir",
        },
        {
          name: "--fd-output",
        },
        {
          name: "--save-fname",
        },
        {
          name: "--base-delay-ms",
        },
        {
          name: "--output",
        },
        {
          name: "--quiet",
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "pseudo",
      options: [
        {
          name: "--start",
        },
        {
          name: "--increment",
        },
        {
          name: "--formatstr",
        },
        {
          name: "--no-headers",
        },
        {
          name: "--delimiter",
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "py",
      options: [
        {
          name: "--helper",
        },
        {
          name: "--batch",
        },
        {
          name: "--output",
        },
        {
          name: "--no-headers",
        },
        {
          name: "--delimiter",
        },
        {
          name: "--progressbar",
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "rename",
      options: [
        {
          name: "--output",
        },
        {
          name: "--no-headers",
        },
        {
          name: "--delimiter",
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "replace",
      options: [
        {
          name: "--ignore-case",
        },
        {
          name: "--select",
        },
        {
          name: "--unicode",
        },
        {
          name: "--size-limit",
        },
        {
          name: "--dfa-size-limit",
        },
        {
          name: "--output",
        },
        {
          name: "--no-headers",
        },
        {
          name: "--delimiter",
        },
        {
          name: "--progressbar",
        },
        {
          name: "--quiet",
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "reverse",
      options: [
        {
          name: "--output",
        },
        {
          name: "--no-headers",
        },
        {
          name: "--delimiter",
        },
        {
          name: "--memcheck",
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "safenames",
      options: [
        {
          name: "--mode",
        },
        {
          name: "--reserved",
        },
        {
          name: "--prefix",
        },
        {
          name: "--output",
        },
        {
          name: "--delimiter",
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "sample",
      options: [
        {
          name: "--seed",
        },
        {
          name: "--rng",
        },
        {
          name: "--user-agent",
        },
        {
          name: "--timeout",
        },
        {
          name: "--output",
        },
        {
          name: "--no-headers",
        },
        {
          name: "--delimiter",
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "schema",
      options: [
        {
          name: "--enum-threshold",
        },
        {
          name: "--ignore-case",
        },
        {
          name: "--strict-dates",
        },
        {
          name: "--pattern-columns",
        },
        {
          name: "--date-whitelist",
        },
        {
          name: "--prefer-dmy",
        },
        {
          name: "--force",
        },
        {
          name: "--stdout",
        },
        {
          name: "--jobs",
        },
        {
          name: "--no-headers",
        },
        {
          name: "--delimiter",
        },
        {
          name: "--memcheck",
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "search",
      options: [
        {
          name: "--ignore-case",
        },
        {
          name: "--select",
        },
        {
          name: "--invert-match",
        },
        {
          name: "--unicode",
        },
        {
          name: "--flag",
        },
        {
          name: "--quick",
        },
        {
          name: "--preview-match",
        },
        {
          name: "--count",
        },
        {
          name: "--size-limit",
        },
        {
          name: "--dfa-size-limit",
        },
        {
          name: "--json",
        },
        {
          name: "--not-one",
        },
        {
          name: "--output",
        },
        {
          name: "--no-headers",
        },
        {
          name: "--delimiter",
        },
        {
          name: "--progressbar",
        },
        {
          name: "--quiet",
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "searchset",
      options: [
        {
          name: "--ignore-case",
        },
        {
          name: "--select",
        },
        {
          name: "--invert-match",
        },
        {
          name: "--unicode",
        },
        {
          name: "--flag",
        },
        {
          name: "--flag-matches-only",
        },
        {
          name: "--unmatched-output",
        },
        {
          name: "--quick",
        },
        {
          name: "--count",
        },
        {
          name: "--json",
        },
        {
          name: "--size-limit",
        },
        {
          name: "--dfa-size-limit",
        },
        {
          name: "--output",
        },
        {
          name: "--no-headers",
        },
        {
          name: "--delimiter",
        },
        {
          name: "--progressbar",
        },
        {
          name: "--quiet",
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "select",
      options: [
        {
          name: "--random",
        },
        {
          name: "--seed",
        },
        {
          name: "--sort",
        },
        {
          name: "--output",
        },
        {
          name: "--no-headers",
        },
        {
          name: "--delimiter",
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "slice",
      options: [
        {
          name: "--start",
        },
        {
          name: "--end",
        },
        {
          name: "--len",
        },
        {
          name: "--index",
        },
        {
          name: "--json",
        },
        {
          name: "--output",
        },
        {
          name: "--no-headers",
        },
        {
          name: "--delimiter",
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "snappy",
      subcommands: [
        {
          name: "compress",
          options: [
            {
              name: ["-h", "--help"],
              description: "Print help",
            },
          ],
        },
        {
          name: "decompress",
          options: [
            {
              name: ["-h", "--help"],
              description: "Print help",
            },
          ],
        },
        {
          name: "check",
          options: [
            {
              name: ["-h", "--help"],
              description: "Print help",
            },
          ],
        },
        {
          name: "validate",
          options: [
            {
              name: ["-h", "--help"],
              description: "Print help",
            },
          ],
        },
        {
          name: "help",
          description: "Print this message or the help of the given subcommand(s)",
          subcommands: [
            {
              name: "compress",
            },
            {
              name: "decompress",
            },
            {
              name: "check",
            },
            {
              name: "validate",
            },
            {
              name: "help",
              description: "Print this message or the help of the given subcommand(s)",
            },
          ],
        },
      ],
      options: [
        {
          name: "--user-agent",
        },
        {
          name: "--timeout",
        },
        {
          name: "--output",
        },
        {
          name: "--jobs",
        },
        {
          name: "--quiet",
        },
        {
          name: "--progressbar",
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "sniff",
      options: [
        {
          name: "--sample",
        },
        {
          name: "--prefer-dmy",
        },
        {
          name: "--delimiter",
        },
        {
          name: "--quote",
        },
        {
          name: "--json",
        },
        {
          name: "--pretty-json",
        },
        {
          name: "--save-urlsample",
        },
        {
          name: "--timeout",
        },
        {
          name: "--user-agent",
        },
        {
          name: "--stats-types",
        },
        {
          name: "--no-infer",
        },
        {
          name: "--just-mime",
        },
        {
          name: "--quick",
        },
        {
          name: "--harvest-mode",
        },
        {
          name: "--progressbar",
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "sort",
      options: [
        {
          name: "--select",
        },
        {
          name: "--numeric",
        },
        {
          name: "--reverse",
        },
        {
          name: "--ignore-case",
        },
        {
          name: "--unique",
        },
        {
          name: "--random",
        },
        {
          name: "--seed",
        },
        {
          name: "--rng",
        },
        {
          name: "--jobs",
        },
        {
          name: "--faster",
        },
        {
          name: "--output",
        },
        {
          name: "--no-headers",
        },
        {
          name: "--delimiter",
        },
        {
          name: "--memcheck",
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "sortcheck",
      options: [
        {
          name: "--select",
        },
        {
          name: "--ignore-case",
        },
        {
          name: "--all",
        },
        {
          name: "--json",
        },
        {
          name: "--pretty-json",
        },
        {
          name: "--no-headers",
        },
        {
          name: "--delimiter",
        },
        {
          name: "--progressbar",
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "split",
      options: [
        {
          name: "--size",
        },
        {
          name: "--chunks",
        },
        {
          name: "--kb-size",
        },
        {
          name: "--jobs",
        },
        {
          name: "--filename",
        },
        {
          name: "--pad",
        },
        {
          name: "--no-headers",
        },
        {
          name: "--delimiter",
        },
        {
          name: "--quiet",
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "sqlp",
      options: [
        {
          name: "--format",
        },
        {
          name: "--try-parsedates",
        },
        {
          name: "--infer-len",
        },
        {
          name: "--streaming",
        },
        {
          name: "--low-memory",
        },
        {
          name: "--no-optimizations",
        },
        {
          name: "--truncate-ragged-lines",
        },
        {
          name: "--ignore-errors",
        },
        {
          name: "--rnull-values",
        },
        {
          name: "--decimal-comma",
        },
        {
          name: "--datetime-format",
        },
        {
          name: "--date-format",
        },
        {
          name: "--time-format",
        },
        {
          name: "--float-precision",
        },
        {
          name: "--wnull-value",
        },
        {
          name: "--compression",
        },
        {
          name: "--compress-level",
        },
        {
          name: "--statistics",
        },
        {
          name: "--output",
        },
        {
          name: "--delimiter",
        },
        {
          name: "--quiet",
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "stats",
      options: [
        {
          name: "--select",
        },
        {
          name: "--everything",
        },
        {
          name: "--typesonly",
        },
        {
          name: "--infer-boolean",
        },
        {
          name: "--mode",
        },
        {
          name: "--cardinality",
        },
        {
          name: "--median",
        },
        {
          name: "--mad",
        },
        {
          name: "--quartiles",
        },
        {
          name: "--round",
        },
        {
          name: "--nulls",
        },
        {
          name: "--infer-dates",
        },
        {
          name: "--prefer-dmy",
        },
        {
          name: "--force",
        },
        {
          name: "--jobs",
        },
        {
          name: "--stats-binout",
        },
        {
          name: "--cache-threshold",
        },
        {
          name: "--output",
        },
        {
          name: "--no-headers",
        },
        {
          name: "--delimiter",
        },
        {
          name: "--memcheck",
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "table",
      options: [
        {
          name: "--width",
        },
        {
          name: "--pad",
        },
        {
          name: "--align",
        },
        {
          name: "--condense",
        },
        {
          name: "--output",
        },
        {
          name: "--delimiter",
        },
        {
          name: "--memcheck",
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "to",
      options: [
        {
          name: "--print-package",
        },
        {
          name: "--dump",
        },
        {
          name: "--stats",
        },
        {
          name: "--stats-csv",
        },
        {
          name: "--quiet",
        },
        {
          name: "--schema",
        },
        {
          name: "--drop",
        },
        {
          name: "--evolve",
        },
        {
          name: "--pipe",
        },
        {
          name: "--separator",
        },
        {
          name: "--jobs",
        },
        {
          name: "--delimiter",
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "tojsonl",
      options: [
        {
          name: "--trim",
        },
        {
          name: "--no-boolean",
        },
        {
          name: "--jobs",
        },
        {
          name: "--batch",
        },
        {
          name: "--delimiter",
        },
        {
          name: "--output",
        },
        {
          name: "--memcheck",
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "transpose",
      options: [
        {
          name: "--multipass",
        },
        {
          name: "--output",
        },
        {
          name: "--delimiter",
        },
        {
          name: "--memcheck",
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "validate",
      options: [
        {
          name: "--trim",
        },
        {
          name: "--fail-fast",
        },
        {
          name: "--valid",
        },
        {
          name: "--invalid",
        },
        {
          name: "--json",
        },
        {
          name: "--pretty-json",
        },
        {
          name: "--valid-output",
        },
        {
          name: "--jobs",
        },
        {
          name: "--batch",
        },
        {
          name: "--timeout",
        },
        {
          name: "--no-headers",
        },
        {
          name: "--delimiter",
        },
        {
          name: "--progressbar",
        },
        {
          name: "--quiet",
        },
        {
          name: ["-h", "--help"],
          description: "Print help",
        },
      ],
    },
    {
      name: "help",
      description: "Print this message or the help of the given subcommand(s)",
      subcommands: [
        {
          name: "apply",
          subcommands: [
            {
              name: "operations",
            },
            {
              name: "emptyreplace",
            },
            {
              name: "dynfmt",
            },
            {
              name: "calcconv",
            },
          ],
        },
        {
          name: "behead",
        },
        {
          name: "cat",
          subcommands: [
            {
              name: "rows",
            },
            {
              name: "rowskey",
            },
            {
              name: "columns",
            },
          ],
        },
        {
          name: "clipboard",
        },
        {
          name: "count",
        },
        {
          name: "datefmt",
        },
        {
          name: "dedup",
        },
        {
          name: "describegpt",
        },
        {
          name: "diff",
        },
        {
          name: "enum",
        },
        {
          name: "excel",
        },
        {
          name: "exclude",
        },
        {
          name: "extdedup",
        },
        {
          name: "extsort",
        },
        {
          name: "explode",
        },
        {
          name: "fetch",
        },
        {
          name: "fetchpost",
        },
        {
          name: "fill",
        },
        {
          name: "fixlengths",
        },
        {
          name: "flatten",
        },
        {
          name: "fmt",
        },
        {
          name: "foreach",
        },
        {
          name: "frequency",
        },
        {
          name: "geocode",
        },
        {
          name: "headers",
        },
        {
          name: "index",
        },
        {
          name: "input",
        },
        {
          name: "join",
        },
        {
          name: "joinp",
        },
        {
          name: "json",
        },
        {
          name: "jsonl",
        },
        {
          name: "luau",
        },
        {
          name: "partition",
        },
        {
          name: "prompt",
        },
        {
          name: "pseudo",
        },
        {
          name: "py",
        },
        {
          name: "rename",
        },
        {
          name: "replace",
        },
        {
          name: "reverse",
        },
        {
          name: "safenames",
        },
        {
          name: "sample",
        },
        {
          name: "schema",
        },
        {
          name: "search",
        },
        {
          name: "searchset",
        },
        {
          name: "select",
        },
        {
          name: "slice",
        },
        {
          name: "snappy",
          subcommands: [
            {
              name: "compress",
            },
            {
              name: "decompress",
            },
            {
              name: "check",
            },
            {
              name: "validate",
            },
          ],
        },
        {
          name: "sniff",
        },
        {
          name: "sort",
        },
        {
          name: "sortcheck",
        },
        {
          name: "split",
        },
        {
          name: "sqlp",
        },
        {
          name: "stats",
        },
        {
          name: "table",
        },
        {
          name: "to",
        },
        {
          name: "tojsonl",
        },
        {
          name: "transpose",
        },
        {
          name: "validate",
        },
        {
          name: "help",
          description: "Print this message or the help of the given subcommand(s)",
        },
      ],
    },
  ],
  options: [
    {
      name: "--list",
    },
    {
      name: "--envlist",
    },
    {
      name: "--update",
    },
    {
      name: "--updatenow",
    },
    {
      name: "--version",
    },
    {
      name: ["-h", "--help"],
      description: "Print help",
    },
  ],
};

export default completion;
