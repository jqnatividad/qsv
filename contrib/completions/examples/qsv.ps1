
using namespace System.Management.Automation
using namespace System.Management.Automation.Language

Register-ArgumentCompleter -Native -CommandName 'qsv' -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $commandElements = $commandAst.CommandElements
    $command = @(
        'qsv'
        for ($i = 1; $i -lt $commandElements.Count; $i++) {
            $element = $commandElements[$i]
            if ($element -isnot [StringConstantExpressionAst] -or
                $element.StringConstantType -ne [StringConstantType]::BareWord -or
                $element.Value.StartsWith('-') -or
                $element.Value -eq $wordToComplete) {
                break
        }
        $element.Value
    }) -join ';'

    $completions = @(switch ($command) {
        'qsv' {
            [CompletionResult]::new('--list', 'list', [CompletionResultType]::ParameterName, 'list')
            [CompletionResult]::new('--envlist', 'envlist', [CompletionResultType]::ParameterName, 'envlist')
            [CompletionResult]::new('--update', 'update', [CompletionResultType]::ParameterName, 'update')
            [CompletionResult]::new('--updatenow', 'updatenow', [CompletionResultType]::ParameterName, 'updatenow')
            [CompletionResult]::new('--version', 'version', [CompletionResultType]::ParameterName, 'version')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('apply', 'apply', [CompletionResultType]::ParameterValue, 'apply')
            [CompletionResult]::new('behead', 'behead', [CompletionResultType]::ParameterValue, 'behead')
            [CompletionResult]::new('cat', 'cat', [CompletionResultType]::ParameterValue, 'cat')
            [CompletionResult]::new('clipboard', 'clipboard', [CompletionResultType]::ParameterValue, 'clipboard')
            [CompletionResult]::new('count', 'count', [CompletionResultType]::ParameterValue, 'count')
            [CompletionResult]::new('datefmt', 'datefmt', [CompletionResultType]::ParameterValue, 'datefmt')
            [CompletionResult]::new('dedup', 'dedup', [CompletionResultType]::ParameterValue, 'dedup')
            [CompletionResult]::new('describegpt', 'describegpt', [CompletionResultType]::ParameterValue, 'describegpt')
            [CompletionResult]::new('diff', 'diff', [CompletionResultType]::ParameterValue, 'diff')
            [CompletionResult]::new('enum', 'enum', [CompletionResultType]::ParameterValue, 'enum')
            [CompletionResult]::new('excel', 'excel', [CompletionResultType]::ParameterValue, 'excel')
            [CompletionResult]::new('exclude', 'exclude', [CompletionResultType]::ParameterValue, 'exclude')
            [CompletionResult]::new('extdedup', 'extdedup', [CompletionResultType]::ParameterValue, 'extdedup')
            [CompletionResult]::new('extsort', 'extsort', [CompletionResultType]::ParameterValue, 'extsort')
            [CompletionResult]::new('explode', 'explode', [CompletionResultType]::ParameterValue, 'explode')
            [CompletionResult]::new('fetch', 'fetch', [CompletionResultType]::ParameterValue, 'fetch')
            [CompletionResult]::new('fetchpost', 'fetchpost', [CompletionResultType]::ParameterValue, 'fetchpost')
            [CompletionResult]::new('fill', 'fill', [CompletionResultType]::ParameterValue, 'fill')
            [CompletionResult]::new('fixlengths', 'fixlengths', [CompletionResultType]::ParameterValue, 'fixlengths')
            [CompletionResult]::new('flatten', 'flatten', [CompletionResultType]::ParameterValue, 'flatten')
            [CompletionResult]::new('fmt', 'fmt', [CompletionResultType]::ParameterValue, 'fmt')
            [CompletionResult]::new('foreach', 'foreach', [CompletionResultType]::ParameterValue, 'foreach')
            [CompletionResult]::new('frequency', 'frequency', [CompletionResultType]::ParameterValue, 'frequency')
            [CompletionResult]::new('geocode', 'geocode', [CompletionResultType]::ParameterValue, 'geocode')
            [CompletionResult]::new('headers', 'headers', [CompletionResultType]::ParameterValue, 'headers')
            [CompletionResult]::new('index', 'index', [CompletionResultType]::ParameterValue, 'index')
            [CompletionResult]::new('input', 'input', [CompletionResultType]::ParameterValue, 'input')
            [CompletionResult]::new('join', 'join', [CompletionResultType]::ParameterValue, 'join')
            [CompletionResult]::new('joinp', 'joinp', [CompletionResultType]::ParameterValue, 'joinp')
            [CompletionResult]::new('json', 'json', [CompletionResultType]::ParameterValue, 'json')
            [CompletionResult]::new('jsonl', 'jsonl', [CompletionResultType]::ParameterValue, 'jsonl')
            [CompletionResult]::new('luau', 'luau', [CompletionResultType]::ParameterValue, 'luau')
            [CompletionResult]::new('partition', 'partition', [CompletionResultType]::ParameterValue, 'partition')
            [CompletionResult]::new('prompt', 'prompt', [CompletionResultType]::ParameterValue, 'prompt')
            [CompletionResult]::new('pseudo', 'pseudo', [CompletionResultType]::ParameterValue, 'pseudo')
            [CompletionResult]::new('py', 'py', [CompletionResultType]::ParameterValue, 'py')
            [CompletionResult]::new('rename', 'rename', [CompletionResultType]::ParameterValue, 'rename')
            [CompletionResult]::new('replace', 'replace', [CompletionResultType]::ParameterValue, 'replace')
            [CompletionResult]::new('reverse', 'reverse', [CompletionResultType]::ParameterValue, 'reverse')
            [CompletionResult]::new('safenames', 'safenames', [CompletionResultType]::ParameterValue, 'safenames')
            [CompletionResult]::new('sample', 'sample', [CompletionResultType]::ParameterValue, 'sample')
            [CompletionResult]::new('schema', 'schema', [CompletionResultType]::ParameterValue, 'schema')
            [CompletionResult]::new('search', 'search', [CompletionResultType]::ParameterValue, 'search')
            [CompletionResult]::new('searchset', 'searchset', [CompletionResultType]::ParameterValue, 'searchset')
            [CompletionResult]::new('select', 'select', [CompletionResultType]::ParameterValue, 'select')
            [CompletionResult]::new('slice', 'slice', [CompletionResultType]::ParameterValue, 'slice')
            [CompletionResult]::new('snappy', 'snappy', [CompletionResultType]::ParameterValue, 'snappy')
            [CompletionResult]::new('sniff', 'sniff', [CompletionResultType]::ParameterValue, 'sniff')
            [CompletionResult]::new('sort', 'sort', [CompletionResultType]::ParameterValue, 'sort')
            [CompletionResult]::new('sortcheck', 'sortcheck', [CompletionResultType]::ParameterValue, 'sortcheck')
            [CompletionResult]::new('split', 'split', [CompletionResultType]::ParameterValue, 'split')
            [CompletionResult]::new('sqlp', 'sqlp', [CompletionResultType]::ParameterValue, 'sqlp')
            [CompletionResult]::new('stats', 'stats', [CompletionResultType]::ParameterValue, 'stats')
            [CompletionResult]::new('table', 'table', [CompletionResultType]::ParameterValue, 'table')
            [CompletionResult]::new('to', 'to', [CompletionResultType]::ParameterValue, 'to')
            [CompletionResult]::new('tojsonl', 'tojsonl', [CompletionResultType]::ParameterValue, 'tojsonl')
            [CompletionResult]::new('transpose', 'transpose', [CompletionResultType]::ParameterValue, 'transpose')
            [CompletionResult]::new('validate', 'validate', [CompletionResultType]::ParameterValue, 'validate')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'qsv;apply' {
            [CompletionResult]::new('--new-column', 'new-column', [CompletionResultType]::ParameterName, 'new-column')
            [CompletionResult]::new('--rename', 'rename', [CompletionResultType]::ParameterName, 'rename')
            [CompletionResult]::new('--comparand', 'comparand', [CompletionResultType]::ParameterName, 'comparand')
            [CompletionResult]::new('--replacement', 'replacement', [CompletionResultType]::ParameterName, 'replacement')
            [CompletionResult]::new('--formatstr', 'formatstr', [CompletionResultType]::ParameterName, 'formatstr')
            [CompletionResult]::new('--jobs', 'jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('--batch', 'batch', [CompletionResultType]::ParameterName, 'batch')
            [CompletionResult]::new('--output', 'output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--no-headers', 'no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('--delimiter', 'delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('--progressbar', 'progressbar', [CompletionResultType]::ParameterName, 'progressbar')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('operations', 'operations', [CompletionResultType]::ParameterValue, 'operations')
            [CompletionResult]::new('emptyreplace', 'emptyreplace', [CompletionResultType]::ParameterValue, 'emptyreplace')
            [CompletionResult]::new('dynfmt', 'dynfmt', [CompletionResultType]::ParameterValue, 'dynfmt')
            [CompletionResult]::new('calcconv', 'calcconv', [CompletionResultType]::ParameterValue, 'calcconv')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'qsv;apply;operations' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;apply;emptyreplace' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;apply;dynfmt' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;apply;calcconv' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;apply;help' {
            [CompletionResult]::new('operations', 'operations', [CompletionResultType]::ParameterValue, 'operations')
            [CompletionResult]::new('emptyreplace', 'emptyreplace', [CompletionResultType]::ParameterValue, 'emptyreplace')
            [CompletionResult]::new('dynfmt', 'dynfmt', [CompletionResultType]::ParameterValue, 'dynfmt')
            [CompletionResult]::new('calcconv', 'calcconv', [CompletionResultType]::ParameterValue, 'calcconv')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'qsv;apply;help;operations' {
            break
        }
        'qsv;apply;help;emptyreplace' {
            break
        }
        'qsv;apply;help;dynfmt' {
            break
        }
        'qsv;apply;help;calcconv' {
            break
        }
        'qsv;apply;help;help' {
            break
        }
        'qsv;behead' {
            [CompletionResult]::new('--flexible', 'flexible', [CompletionResultType]::ParameterName, 'flexible')
            [CompletionResult]::new('--output', 'output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;cat' {
            [CompletionResult]::new('--output', 'output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--no-headers', 'no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('--delimiter', 'delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('rows', 'rows', [CompletionResultType]::ParameterValue, 'rows')
            [CompletionResult]::new('rowskey', 'rowskey', [CompletionResultType]::ParameterValue, 'rowskey')
            [CompletionResult]::new('columns', 'columns', [CompletionResultType]::ParameterValue, 'columns')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'qsv;cat;rows' {
            [CompletionResult]::new('--flexible', 'flexible', [CompletionResultType]::ParameterName, 'flexible')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;cat;rowskey' {
            [CompletionResult]::new('--group', 'group', [CompletionResultType]::ParameterName, 'group')
            [CompletionResult]::new('--group-name', 'group-name', [CompletionResultType]::ParameterName, 'group-name')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;cat;columns' {
            [CompletionResult]::new('--pad', 'pad', [CompletionResultType]::ParameterName, 'pad')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;cat;help' {
            [CompletionResult]::new('rows', 'rows', [CompletionResultType]::ParameterValue, 'rows')
            [CompletionResult]::new('rowskey', 'rowskey', [CompletionResultType]::ParameterValue, 'rowskey')
            [CompletionResult]::new('columns', 'columns', [CompletionResultType]::ParameterValue, 'columns')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'qsv;cat;help;rows' {
            break
        }
        'qsv;cat;help;rowskey' {
            break
        }
        'qsv;cat;help;columns' {
            break
        }
        'qsv;cat;help;help' {
            break
        }
        'qsv;clipboard' {
            [CompletionResult]::new('--save', 'save', [CompletionResultType]::ParameterName, 'save')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;count' {
            [CompletionResult]::new('--human-readable', 'human-readable', [CompletionResultType]::ParameterName, 'human-readable')
            [CompletionResult]::new('--width', 'width', [CompletionResultType]::ParameterName, 'width')
            [CompletionResult]::new('--no-polars', 'no-polars', [CompletionResultType]::ParameterName, 'no-polars')
            [CompletionResult]::new('--low-memory', 'low-memory', [CompletionResultType]::ParameterName, 'low-memory')
            [CompletionResult]::new('--flexible', 'flexible', [CompletionResultType]::ParameterName, 'flexible')
            [CompletionResult]::new('--no-headers', 'no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;datefmt' {
            [CompletionResult]::new('--formatstr', 'formatstr', [CompletionResultType]::ParameterName, 'formatstr')
            [CompletionResult]::new('--new-column', 'new-column', [CompletionResultType]::ParameterName, 'new-column')
            [CompletionResult]::new('--rename', 'rename', [CompletionResultType]::ParameterName, 'rename')
            [CompletionResult]::new('--prefer-dmy', 'prefer-dmy', [CompletionResultType]::ParameterName, 'prefer-dmy')
            [CompletionResult]::new('--keep-zero-time', 'keep-zero-time', [CompletionResultType]::ParameterName, 'keep-zero-time')
            [CompletionResult]::new('--input-tz', 'input-tz', [CompletionResultType]::ParameterName, 'input-tz')
            [CompletionResult]::new('--output-tz', 'output-tz', [CompletionResultType]::ParameterName, 'output-tz')
            [CompletionResult]::new('--default-tz', 'default-tz', [CompletionResultType]::ParameterName, 'default-tz')
            [CompletionResult]::new('--utc', 'utc', [CompletionResultType]::ParameterName, 'utc')
            [CompletionResult]::new('--zulu', 'zulu', [CompletionResultType]::ParameterName, 'zulu')
            [CompletionResult]::new('--ts-resolution', 'ts-resolution', [CompletionResultType]::ParameterName, 'ts-resolution')
            [CompletionResult]::new('--jobs', 'jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('--batch', 'batch', [CompletionResultType]::ParameterName, 'batch')
            [CompletionResult]::new('--output', 'output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--no-headers', 'no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('--delimiter', 'delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('--progressbar', 'progressbar', [CompletionResultType]::ParameterName, 'progressbar')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;dedup' {
            [CompletionResult]::new('--select', 'select', [CompletionResultType]::ParameterName, 'select')
            [CompletionResult]::new('--numeric', 'numeric', [CompletionResultType]::ParameterName, 'numeric')
            [CompletionResult]::new('--ignore-case', 'ignore-case', [CompletionResultType]::ParameterName, 'ignore-case')
            [CompletionResult]::new('--sorted', 'sorted', [CompletionResultType]::ParameterName, 'sorted')
            [CompletionResult]::new('--dupes-output', 'dupes-output', [CompletionResultType]::ParameterName, 'dupes-output')
            [CompletionResult]::new('--human-readable', 'human-readable', [CompletionResultType]::ParameterName, 'human-readable')
            [CompletionResult]::new('--jobs', 'jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('--output', 'output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--no-headers', 'no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('--delimiter', 'delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('--quiet', 'quiet', [CompletionResultType]::ParameterName, 'quiet')
            [CompletionResult]::new('--memcheck', 'memcheck', [CompletionResultType]::ParameterName, 'memcheck')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;describegpt' {
            [CompletionResult]::new('--all', 'all', [CompletionResultType]::ParameterName, 'all')
            [CompletionResult]::new('--description', 'description', [CompletionResultType]::ParameterName, 'description')
            [CompletionResult]::new('--dictionary', 'dictionary', [CompletionResultType]::ParameterName, 'dictionary')
            [CompletionResult]::new('--tags', 'tags', [CompletionResultType]::ParameterName, 'tags')
            [CompletionResult]::new('--api-key', 'api-key', [CompletionResultType]::ParameterName, 'api-key')
            [CompletionResult]::new('--max-tokens', 'max-tokens', [CompletionResultType]::ParameterName, 'max-tokens')
            [CompletionResult]::new('--json', 'json', [CompletionResultType]::ParameterName, 'json')
            [CompletionResult]::new('--jsonl', 'jsonl', [CompletionResultType]::ParameterName, 'jsonl')
            [CompletionResult]::new('--prompt', 'prompt', [CompletionResultType]::ParameterName, 'prompt')
            [CompletionResult]::new('--prompt-file', 'prompt-file', [CompletionResultType]::ParameterName, 'prompt-file')
            [CompletionResult]::new('--base-url', 'base-url', [CompletionResultType]::ParameterName, 'base-url')
            [CompletionResult]::new('--model', 'model', [CompletionResultType]::ParameterName, 'model')
            [CompletionResult]::new('--timeout', 'timeout', [CompletionResultType]::ParameterName, 'timeout')
            [CompletionResult]::new('--user-agent', 'user-agent', [CompletionResultType]::ParameterName, 'user-agent')
            [CompletionResult]::new('--output', 'output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--quiet', 'quiet', [CompletionResultType]::ParameterName, 'quiet')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;diff' {
            [CompletionResult]::new('--no-headers-left', 'no-headers-left', [CompletionResultType]::ParameterName, 'no-headers-left')
            [CompletionResult]::new('--no-headers-right', 'no-headers-right', [CompletionResultType]::ParameterName, 'no-headers-right')
            [CompletionResult]::new('--no-headers-output', 'no-headers-output', [CompletionResultType]::ParameterName, 'no-headers-output')
            [CompletionResult]::new('--delimiter-left', 'delimiter-left', [CompletionResultType]::ParameterName, 'delimiter-left')
            [CompletionResult]::new('--delimiter-right', 'delimiter-right', [CompletionResultType]::ParameterName, 'delimiter-right')
            [CompletionResult]::new('--delimiter-output', 'delimiter-output', [CompletionResultType]::ParameterName, 'delimiter-output')
            [CompletionResult]::new('--key', 'key', [CompletionResultType]::ParameterName, 'key')
            [CompletionResult]::new('--sort-columns', 'sort-columns', [CompletionResultType]::ParameterName, 'sort-columns')
            [CompletionResult]::new('--jobs', 'jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('--output', 'output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;enum' {
            [CompletionResult]::new('--new-column', 'new-column', [CompletionResultType]::ParameterName, 'new-column')
            [CompletionResult]::new('--start', 'start', [CompletionResultType]::ParameterName, 'start')
            [CompletionResult]::new('--increment', 'increment', [CompletionResultType]::ParameterName, 'increment')
            [CompletionResult]::new('--constant', 'constant', [CompletionResultType]::ParameterName, 'constant')
            [CompletionResult]::new('--copy', 'copy', [CompletionResultType]::ParameterName, 'copy')
            [CompletionResult]::new('--uuid4', 'uuid4', [CompletionResultType]::ParameterName, 'uuid4')
            [CompletionResult]::new('--uuid7', 'uuid7', [CompletionResultType]::ParameterName, 'uuid7')
            [CompletionResult]::new('--hash', 'hash', [CompletionResultType]::ParameterName, 'hash')
            [CompletionResult]::new('--output', 'output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--no-headers', 'no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('--delimiter', 'delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;excel' {
            [CompletionResult]::new('--sheet', 'sheet', [CompletionResultType]::ParameterName, 'sheet')
            [CompletionResult]::new('--metadata', 'metadata', [CompletionResultType]::ParameterName, 'metadata')
            [CompletionResult]::new('--error-format', 'error-format', [CompletionResultType]::ParameterName, 'error-format')
            [CompletionResult]::new('--flexible', 'flexible', [CompletionResultType]::ParameterName, 'flexible')
            [CompletionResult]::new('--trim', 'trim', [CompletionResultType]::ParameterName, 'trim')
            [CompletionResult]::new('--date-format', 'date-format', [CompletionResultType]::ParameterName, 'date-format')
            [CompletionResult]::new('--keep-zero-time', 'keep-zero-time', [CompletionResultType]::ParameterName, 'keep-zero-time')
            [CompletionResult]::new('--range', 'range', [CompletionResultType]::ParameterName, 'range')
            [CompletionResult]::new('--jobs', 'jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('--output', 'output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--delimiter', 'delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('--quiet', 'quiet', [CompletionResultType]::ParameterName, 'quiet')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;exclude' {
            [CompletionResult]::new('--ignore-case', 'ignore-case', [CompletionResultType]::ParameterName, 'ignore-case')
            [CompletionResult]::new('-v', 'v', [CompletionResultType]::ParameterName, 'v')
            [CompletionResult]::new('--output', 'output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--no-headers', 'no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('--delimiter', 'delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;extdedup' {
            [CompletionResult]::new('--no-output', 'no-output', [CompletionResultType]::ParameterName, 'no-output')
            [CompletionResult]::new('--dupes-output', 'dupes-output', [CompletionResultType]::ParameterName, 'dupes-output')
            [CompletionResult]::new('--human-readable', 'human-readable', [CompletionResultType]::ParameterName, 'human-readable')
            [CompletionResult]::new('--memory-limit', 'memory-limit', [CompletionResultType]::ParameterName, 'memory-limit')
            [CompletionResult]::new('--quiet', 'quiet', [CompletionResultType]::ParameterName, 'quiet')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;extsort' {
            [CompletionResult]::new('--memory-limit', 'memory-limit', [CompletionResultType]::ParameterName, 'memory-limit')
            [CompletionResult]::new('--tmp-dir', 'tmp-dir', [CompletionResultType]::ParameterName, 'tmp-dir')
            [CompletionResult]::new('--jobs', 'jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('--no-headers', 'no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;explode' {
            [CompletionResult]::new('--rename', 'rename', [CompletionResultType]::ParameterName, 'rename')
            [CompletionResult]::new('--output', 'output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--no-headers', 'no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('--delimiter', 'delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;fetch' {
            [CompletionResult]::new('--url-template', 'url-template', [CompletionResultType]::ParameterName, 'url-template')
            [CompletionResult]::new('--new-column', 'new-column', [CompletionResultType]::ParameterName, 'new-column')
            [CompletionResult]::new('--jql', 'jql', [CompletionResultType]::ParameterName, 'jql')
            [CompletionResult]::new('--jqlfile', 'jqlfile', [CompletionResultType]::ParameterName, 'jqlfile')
            [CompletionResult]::new('--pretty', 'pretty', [CompletionResultType]::ParameterName, 'pretty')
            [CompletionResult]::new('--rate-limit', 'rate-limit', [CompletionResultType]::ParameterName, 'rate-limit')
            [CompletionResult]::new('--timeout', 'timeout', [CompletionResultType]::ParameterName, 'timeout')
            [CompletionResult]::new('--http-header', 'http-header', [CompletionResultType]::ParameterName, 'http-header')
            [CompletionResult]::new('--max-retries', 'max-retries', [CompletionResultType]::ParameterName, 'max-retries')
            [CompletionResult]::new('--max-errors', 'max-errors', [CompletionResultType]::ParameterName, 'max-errors')
            [CompletionResult]::new('--store-error', 'store-error', [CompletionResultType]::ParameterName, 'store-error')
            [CompletionResult]::new('--cookies', 'cookies', [CompletionResultType]::ParameterName, 'cookies')
            [CompletionResult]::new('--user-agent', 'user-agent', [CompletionResultType]::ParameterName, 'user-agent')
            [CompletionResult]::new('--report', 'report', [CompletionResultType]::ParameterName, 'report')
            [CompletionResult]::new('--no-cache', 'no-cache', [CompletionResultType]::ParameterName, 'no-cache')
            [CompletionResult]::new('--mem-cache-size', 'mem-cache-size', [CompletionResultType]::ParameterName, 'mem-cache-size')
            [CompletionResult]::new('--disk-cache', 'disk-cache', [CompletionResultType]::ParameterName, 'disk-cache')
            [CompletionResult]::new('--disk-cache-dir', 'disk-cache-dir', [CompletionResultType]::ParameterName, 'disk-cache-dir')
            [CompletionResult]::new('--redis-cache', 'redis-cache', [CompletionResultType]::ParameterName, 'redis-cache')
            [CompletionResult]::new('--cache-error', 'cache-error', [CompletionResultType]::ParameterName, 'cache-error')
            [CompletionResult]::new('--flush-cache', 'flush-cache', [CompletionResultType]::ParameterName, 'flush-cache')
            [CompletionResult]::new('--output', 'output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--no-headers', 'no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('--delimiter', 'delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('--progressbar', 'progressbar', [CompletionResultType]::ParameterName, 'progressbar')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;fetchpost' {
            [CompletionResult]::new('--new-column', 'new-column', [CompletionResultType]::ParameterName, 'new-column')
            [CompletionResult]::new('--jql', 'jql', [CompletionResultType]::ParameterName, 'jql')
            [CompletionResult]::new('--jqlfile', 'jqlfile', [CompletionResultType]::ParameterName, 'jqlfile')
            [CompletionResult]::new('--pretty', 'pretty', [CompletionResultType]::ParameterName, 'pretty')
            [CompletionResult]::new('--rate-limit', 'rate-limit', [CompletionResultType]::ParameterName, 'rate-limit')
            [CompletionResult]::new('--timeout', 'timeout', [CompletionResultType]::ParameterName, 'timeout')
            [CompletionResult]::new('--http-header', 'http-header', [CompletionResultType]::ParameterName, 'http-header')
            [CompletionResult]::new('--compress', 'compress', [CompletionResultType]::ParameterName, 'compress')
            [CompletionResult]::new('--max-retries', 'max-retries', [CompletionResultType]::ParameterName, 'max-retries')
            [CompletionResult]::new('--max-errors', 'max-errors', [CompletionResultType]::ParameterName, 'max-errors')
            [CompletionResult]::new('--store-error', 'store-error', [CompletionResultType]::ParameterName, 'store-error')
            [CompletionResult]::new('--cookies', 'cookies', [CompletionResultType]::ParameterName, 'cookies')
            [CompletionResult]::new('--user-agent', 'user-agent', [CompletionResultType]::ParameterName, 'user-agent')
            [CompletionResult]::new('--report', 'report', [CompletionResultType]::ParameterName, 'report')
            [CompletionResult]::new('--no-cache', 'no-cache', [CompletionResultType]::ParameterName, 'no-cache')
            [CompletionResult]::new('--mem-cache-size', 'mem-cache-size', [CompletionResultType]::ParameterName, 'mem-cache-size')
            [CompletionResult]::new('--disk-cache', 'disk-cache', [CompletionResultType]::ParameterName, 'disk-cache')
            [CompletionResult]::new('--disk-cache-dir', 'disk-cache-dir', [CompletionResultType]::ParameterName, 'disk-cache-dir')
            [CompletionResult]::new('--redis-cache', 'redis-cache', [CompletionResultType]::ParameterName, 'redis-cache')
            [CompletionResult]::new('--cache-error', 'cache-error', [CompletionResultType]::ParameterName, 'cache-error')
            [CompletionResult]::new('--flush-cache', 'flush-cache', [CompletionResultType]::ParameterName, 'flush-cache')
            [CompletionResult]::new('--output', 'output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--no-headers', 'no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('--delimiter', 'delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('--progressbar', 'progressbar', [CompletionResultType]::ParameterName, 'progressbar')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;fill' {
            [CompletionResult]::new('--groupby', 'groupby', [CompletionResultType]::ParameterName, 'groupby')
            [CompletionResult]::new('--first', 'first', [CompletionResultType]::ParameterName, 'first')
            [CompletionResult]::new('--backfill', 'backfill', [CompletionResultType]::ParameterName, 'backfill')
            [CompletionResult]::new('--default', 'default', [CompletionResultType]::ParameterName, 'default')
            [CompletionResult]::new('--output', 'output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--no-headers', 'no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('--delimiter', 'delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;fixlengths' {
            [CompletionResult]::new('--length', 'length', [CompletionResultType]::ParameterName, 'length')
            [CompletionResult]::new('--insert', 'insert', [CompletionResultType]::ParameterName, 'insert')
            [CompletionResult]::new('--output', 'output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--delimiter', 'delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;flatten' {
            [CompletionResult]::new('--condense', 'condense', [CompletionResultType]::ParameterName, 'condense')
            [CompletionResult]::new('--field-separator', 'field-separator', [CompletionResultType]::ParameterName, 'field-separator')
            [CompletionResult]::new('--separator', 'separator', [CompletionResultType]::ParameterName, 'separator')
            [CompletionResult]::new('--no-headers', 'no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('--delimiter', 'delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;fmt' {
            [CompletionResult]::new('--out-delimiter', 'out-delimiter', [CompletionResultType]::ParameterName, 'out-delimiter')
            [CompletionResult]::new('--crlf', 'crlf', [CompletionResultType]::ParameterName, 'crlf')
            [CompletionResult]::new('--ascii', 'ascii', [CompletionResultType]::ParameterName, 'ascii')
            [CompletionResult]::new('--quote', 'quote', [CompletionResultType]::ParameterName, 'quote')
            [CompletionResult]::new('--quote-always', 'quote-always', [CompletionResultType]::ParameterName, 'quote-always')
            [CompletionResult]::new('--quote-never', 'quote-never', [CompletionResultType]::ParameterName, 'quote-never')
            [CompletionResult]::new('--escape', 'escape', [CompletionResultType]::ParameterName, 'escape')
            [CompletionResult]::new('--no-final-newline', 'no-final-newline', [CompletionResultType]::ParameterName, 'no-final-newline')
            [CompletionResult]::new('--output', 'output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--delimiter', 'delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;foreach' {
            [CompletionResult]::new('--unify', 'unify', [CompletionResultType]::ParameterName, 'unify')
            [CompletionResult]::new('--new-column', 'new-column', [CompletionResultType]::ParameterName, 'new-column')
            [CompletionResult]::new('--dry-run', 'dry-run', [CompletionResultType]::ParameterName, 'dry-run')
            [CompletionResult]::new('--no-headers', 'no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('--delimiter', 'delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('--progressbar', 'progressbar', [CompletionResultType]::ParameterName, 'progressbar')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;frequency' {
            [CompletionResult]::new('--select', 'select', [CompletionResultType]::ParameterName, 'select')
            [CompletionResult]::new('--limit', 'limit', [CompletionResultType]::ParameterName, 'limit')
            [CompletionResult]::new('--unq-limit', 'unq-limit', [CompletionResultType]::ParameterName, 'unq-limit')
            [CompletionResult]::new('--lmt-threshold', 'lmt-threshold', [CompletionResultType]::ParameterName, 'lmt-threshold')
            [CompletionResult]::new('--pct-dec-places', 'pct-dec-places', [CompletionResultType]::ParameterName, 'pct-dec-places')
            [CompletionResult]::new('--other-sorted', 'other-sorted', [CompletionResultType]::ParameterName, 'other-sorted')
            [CompletionResult]::new('--other-text', 'other-text', [CompletionResultType]::ParameterName, 'other-text')
            [CompletionResult]::new('--asc', 'asc', [CompletionResultType]::ParameterName, 'asc')
            [CompletionResult]::new('--no-trim', 'no-trim', [CompletionResultType]::ParameterName, 'no-trim')
            [CompletionResult]::new('--ignore-case', 'ignore-case', [CompletionResultType]::ParameterName, 'ignore-case')
            [CompletionResult]::new('--jobs', 'jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('--output', 'output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--no-headers', 'no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('--delimiter', 'delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('--memcheck', 'memcheck', [CompletionResultType]::ParameterName, 'memcheck')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;geocode' {
            [CompletionResult]::new('--new-column', 'new-column', [CompletionResultType]::ParameterName, 'new-column')
            [CompletionResult]::new('--rename', 'rename', [CompletionResultType]::ParameterName, 'rename')
            [CompletionResult]::new('--country', 'country', [CompletionResultType]::ParameterName, 'country')
            [CompletionResult]::new('--min-score', 'min-score', [CompletionResultType]::ParameterName, 'min-score')
            [CompletionResult]::new('--admin1', 'admin1', [CompletionResultType]::ParameterName, 'admin1')
            [CompletionResult]::new('--k_weight', 'k_weight', [CompletionResultType]::ParameterName, 'k_weight')
            [CompletionResult]::new('--formatstr', 'formatstr', [CompletionResultType]::ParameterName, 'formatstr')
            [CompletionResult]::new('--language', 'language', [CompletionResultType]::ParameterName, 'language')
            [CompletionResult]::new('--invalid-result', 'invalid-result', [CompletionResultType]::ParameterName, 'invalid-result')
            [CompletionResult]::new('--jobs', 'jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('--batch', 'batch', [CompletionResultType]::ParameterName, 'batch')
            [CompletionResult]::new('--timeout', 'timeout', [CompletionResultType]::ParameterName, 'timeout')
            [CompletionResult]::new('--cache-dir', 'cache-dir', [CompletionResultType]::ParameterName, 'cache-dir')
            [CompletionResult]::new('--languages', 'languages', [CompletionResultType]::ParameterName, 'languages')
            [CompletionResult]::new('--cities-url', 'cities-url', [CompletionResultType]::ParameterName, 'cities-url')
            [CompletionResult]::new('--force', 'force', [CompletionResultType]::ParameterName, 'force')
            [CompletionResult]::new('--output', 'output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--delimiter', 'delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('--progressbar', 'progressbar', [CompletionResultType]::ParameterName, 'progressbar')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;headers' {
            [CompletionResult]::new('--just-names', 'just-names', [CompletionResultType]::ParameterName, 'just-names')
            [CompletionResult]::new('--intersect', 'intersect', [CompletionResultType]::ParameterName, 'intersect')
            [CompletionResult]::new('--trim', 'trim', [CompletionResultType]::ParameterName, 'trim')
            [CompletionResult]::new('--delimiter', 'delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;index' {
            [CompletionResult]::new('--output', 'output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;input' {
            [CompletionResult]::new('--quote', 'quote', [CompletionResultType]::ParameterName, 'quote')
            [CompletionResult]::new('--escape', 'escape', [CompletionResultType]::ParameterName, 'escape')
            [CompletionResult]::new('--no-quoting', 'no-quoting', [CompletionResultType]::ParameterName, 'no-quoting')
            [CompletionResult]::new('--quote-style', 'quote-style', [CompletionResultType]::ParameterName, 'quote-style')
            [CompletionResult]::new('--skip-lines', 'skip-lines', [CompletionResultType]::ParameterName, 'skip-lines')
            [CompletionResult]::new('--auto-skip', 'auto-skip', [CompletionResultType]::ParameterName, 'auto-skip')
            [CompletionResult]::new('--skip-lastlines', 'skip-lastlines', [CompletionResultType]::ParameterName, 'skip-lastlines')
            [CompletionResult]::new('--trim-headers', 'trim-headers', [CompletionResultType]::ParameterName, 'trim-headers')
            [CompletionResult]::new('--trim-fields', 'trim-fields', [CompletionResultType]::ParameterName, 'trim-fields')
            [CompletionResult]::new('--comment', 'comment', [CompletionResultType]::ParameterName, 'comment')
            [CompletionResult]::new('--encoding-errors', 'encoding-errors', [CompletionResultType]::ParameterName, 'encoding-errors')
            [CompletionResult]::new('--output', 'output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--delimiter', 'delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;join' {
            [CompletionResult]::new('--ignore-case', 'ignore-case', [CompletionResultType]::ParameterName, 'ignore-case')
            [CompletionResult]::new('--left-anti', 'left-anti', [CompletionResultType]::ParameterName, 'left-anti')
            [CompletionResult]::new('--left-semi', 'left-semi', [CompletionResultType]::ParameterName, 'left-semi')
            [CompletionResult]::new('--right', 'right', [CompletionResultType]::ParameterName, 'right')
            [CompletionResult]::new('--full', 'full', [CompletionResultType]::ParameterName, 'full')
            [CompletionResult]::new('--cross', 'cross', [CompletionResultType]::ParameterName, 'cross')
            [CompletionResult]::new('--nulls', 'nulls', [CompletionResultType]::ParameterName, 'nulls')
            [CompletionResult]::new('--output', 'output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--no-headers', 'no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('--delimiter', 'delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;joinp' {
            [CompletionResult]::new('--left', 'left', [CompletionResultType]::ParameterName, 'left')
            [CompletionResult]::new('--left-anti', 'left-anti', [CompletionResultType]::ParameterName, 'left-anti')
            [CompletionResult]::new('--left-semi', 'left-semi', [CompletionResultType]::ParameterName, 'left-semi')
            [CompletionResult]::new('--right', 'right', [CompletionResultType]::ParameterName, 'right')
            [CompletionResult]::new('--full', 'full', [CompletionResultType]::ParameterName, 'full')
            [CompletionResult]::new('--cross', 'cross', [CompletionResultType]::ParameterName, 'cross')
            [CompletionResult]::new('--coalesce', 'coalesce', [CompletionResultType]::ParameterName, 'coalesce')
            [CompletionResult]::new('--filter-left', 'filter-left', [CompletionResultType]::ParameterName, 'filter-left')
            [CompletionResult]::new('--filter-right', 'filter-right', [CompletionResultType]::ParameterName, 'filter-right')
            [CompletionResult]::new('--validate', 'validate', [CompletionResultType]::ParameterName, 'validate')
            [CompletionResult]::new('--nulls', 'nulls', [CompletionResultType]::ParameterName, 'nulls')
            [CompletionResult]::new('--streaming', 'streaming', [CompletionResultType]::ParameterName, 'streaming')
            [CompletionResult]::new('--try-parsedates', 'try-parsedates', [CompletionResultType]::ParameterName, 'try-parsedates')
            [CompletionResult]::new('--infer-len', 'infer-len', [CompletionResultType]::ParameterName, 'infer-len')
            [CompletionResult]::new('--low-memory', 'low-memory', [CompletionResultType]::ParameterName, 'low-memory')
            [CompletionResult]::new('--no-optimizations', 'no-optimizations', [CompletionResultType]::ParameterName, 'no-optimizations')
            [CompletionResult]::new('--ignore-errors', 'ignore-errors', [CompletionResultType]::ParameterName, 'ignore-errors')
            [CompletionResult]::new('--decimal-comma', 'decimal-comma', [CompletionResultType]::ParameterName, 'decimal-comma')
            [CompletionResult]::new('--asof', 'asof', [CompletionResultType]::ParameterName, 'asof')
            [CompletionResult]::new('--left_by', 'left_by', [CompletionResultType]::ParameterName, 'left_by')
            [CompletionResult]::new('--right_by', 'right_by', [CompletionResultType]::ParameterName, 'right_by')
            [CompletionResult]::new('--strategy', 'strategy', [CompletionResultType]::ParameterName, 'strategy')
            [CompletionResult]::new('--tolerance', 'tolerance', [CompletionResultType]::ParameterName, 'tolerance')
            [CompletionResult]::new('--sql-filter', 'sql-filter', [CompletionResultType]::ParameterName, 'sql-filter')
            [CompletionResult]::new('--datetime-format', 'datetime-format', [CompletionResultType]::ParameterName, 'datetime-format')
            [CompletionResult]::new('--date-format', 'date-format', [CompletionResultType]::ParameterName, 'date-format')
            [CompletionResult]::new('--time-format', 'time-format', [CompletionResultType]::ParameterName, 'time-format')
            [CompletionResult]::new('--float-precision', 'float-precision', [CompletionResultType]::ParameterName, 'float-precision')
            [CompletionResult]::new('--null-value', 'null-value', [CompletionResultType]::ParameterName, 'null-value')
            [CompletionResult]::new('--output', 'output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--delimiter', 'delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('--quiet', 'quiet', [CompletionResultType]::ParameterName, 'quiet')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;json' {
            [CompletionResult]::new('--jaq', 'jaq', [CompletionResultType]::ParameterName, 'jaq')
            [CompletionResult]::new('--output', 'output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;jsonl' {
            [CompletionResult]::new('--ignore-errors', 'ignore-errors', [CompletionResultType]::ParameterName, 'ignore-errors')
            [CompletionResult]::new('--jobs', 'jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('--batch', 'batch', [CompletionResultType]::ParameterName, 'batch')
            [CompletionResult]::new('--output', 'output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--delimiter', 'delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;luau' {
            [CompletionResult]::new('--no-globals', 'no-globals', [CompletionResultType]::ParameterName, 'no-globals')
            [CompletionResult]::new('--colindex', 'colindex', [CompletionResultType]::ParameterName, 'colindex')
            [CompletionResult]::new('--remap', 'remap', [CompletionResultType]::ParameterName, 'remap')
            [CompletionResult]::new('--begin', 'begin', [CompletionResultType]::ParameterName, 'begin')
            [CompletionResult]::new('--luau-path', 'luau-path', [CompletionResultType]::ParameterName, 'luau-path')
            [CompletionResult]::new('--max-errors', 'max-errors', [CompletionResultType]::ParameterName, 'max-errors')
            [CompletionResult]::new('--timeout', 'timeout', [CompletionResultType]::ParameterName, 'timeout')
            [CompletionResult]::new('--ckan-api', 'ckan-api', [CompletionResultType]::ParameterName, 'ckan-api')
            [CompletionResult]::new('--ckan-token', 'ckan-token', [CompletionResultType]::ParameterName, 'ckan-token')
            [CompletionResult]::new('--cache-dir', 'cache-dir', [CompletionResultType]::ParameterName, 'cache-dir')
            [CompletionResult]::new('--output', 'output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--no-headers', 'no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('--delimiter', 'delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('--progressbar', 'progressbar', [CompletionResultType]::ParameterName, 'progressbar')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;partition' {
            [CompletionResult]::new('--filename', 'filename', [CompletionResultType]::ParameterName, 'filename')
            [CompletionResult]::new('--prefix-length', 'prefix-length', [CompletionResultType]::ParameterName, 'prefix-length')
            [CompletionResult]::new('--drop', 'drop', [CompletionResultType]::ParameterName, 'drop')
            [CompletionResult]::new('--no-headers', 'no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('--delimiter', 'delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;prompt' {
            [CompletionResult]::new('--msg', 'msg', [CompletionResultType]::ParameterName, 'msg')
            [CompletionResult]::new('--filters', 'filters', [CompletionResultType]::ParameterName, 'filters')
            [CompletionResult]::new('--workdir', 'workdir', [CompletionResultType]::ParameterName, 'workdir')
            [CompletionResult]::new('--fd-output', 'fd-output', [CompletionResultType]::ParameterName, 'fd-output')
            [CompletionResult]::new('--save-fname', 'save-fname', [CompletionResultType]::ParameterName, 'save-fname')
            [CompletionResult]::new('--base-delay-ms', 'base-delay-ms', [CompletionResultType]::ParameterName, 'base-delay-ms')
            [CompletionResult]::new('--output', 'output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--quiet', 'quiet', [CompletionResultType]::ParameterName, 'quiet')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;pseudo' {
            [CompletionResult]::new('--start', 'start', [CompletionResultType]::ParameterName, 'start')
            [CompletionResult]::new('--increment', 'increment', [CompletionResultType]::ParameterName, 'increment')
            [CompletionResult]::new('--formatstr', 'formatstr', [CompletionResultType]::ParameterName, 'formatstr')
            [CompletionResult]::new('--no-headers', 'no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('--delimiter', 'delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;py' {
            [CompletionResult]::new('--helper', 'helper', [CompletionResultType]::ParameterName, 'helper')
            [CompletionResult]::new('--batch', 'batch', [CompletionResultType]::ParameterName, 'batch')
            [CompletionResult]::new('--output', 'output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--no-headers', 'no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('--delimiter', 'delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('--progressbar', 'progressbar', [CompletionResultType]::ParameterName, 'progressbar')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;rename' {
            [CompletionResult]::new('--output', 'output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--no-headers', 'no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('--delimiter', 'delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;replace' {
            [CompletionResult]::new('--ignore-case', 'ignore-case', [CompletionResultType]::ParameterName, 'ignore-case')
            [CompletionResult]::new('--select', 'select', [CompletionResultType]::ParameterName, 'select')
            [CompletionResult]::new('--unicode', 'unicode', [CompletionResultType]::ParameterName, 'unicode')
            [CompletionResult]::new('--size-limit', 'size-limit', [CompletionResultType]::ParameterName, 'size-limit')
            [CompletionResult]::new('--dfa-size-limit', 'dfa-size-limit', [CompletionResultType]::ParameterName, 'dfa-size-limit')
            [CompletionResult]::new('--output', 'output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--no-headers', 'no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('--delimiter', 'delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('--progressbar', 'progressbar', [CompletionResultType]::ParameterName, 'progressbar')
            [CompletionResult]::new('--quiet', 'quiet', [CompletionResultType]::ParameterName, 'quiet')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;reverse' {
            [CompletionResult]::new('--output', 'output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--no-headers', 'no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('--delimiter', 'delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('--memcheck', 'memcheck', [CompletionResultType]::ParameterName, 'memcheck')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;safenames' {
            [CompletionResult]::new('--mode', 'mode', [CompletionResultType]::ParameterName, 'mode')
            [CompletionResult]::new('--reserved', 'reserved', [CompletionResultType]::ParameterName, 'reserved')
            [CompletionResult]::new('--prefix', 'prefix', [CompletionResultType]::ParameterName, 'prefix')
            [CompletionResult]::new('--output', 'output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--delimiter', 'delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;sample' {
            [CompletionResult]::new('--seed', 'seed', [CompletionResultType]::ParameterName, 'seed')
            [CompletionResult]::new('--rng', 'rng', [CompletionResultType]::ParameterName, 'rng')
            [CompletionResult]::new('--user-agent', 'user-agent', [CompletionResultType]::ParameterName, 'user-agent')
            [CompletionResult]::new('--timeout', 'timeout', [CompletionResultType]::ParameterName, 'timeout')
            [CompletionResult]::new('--output', 'output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--no-headers', 'no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('--delimiter', 'delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;schema' {
            [CompletionResult]::new('--enum-threshold', 'enum-threshold', [CompletionResultType]::ParameterName, 'enum-threshold')
            [CompletionResult]::new('--ignore-case', 'ignore-case', [CompletionResultType]::ParameterName, 'ignore-case')
            [CompletionResult]::new('--strict-dates', 'strict-dates', [CompletionResultType]::ParameterName, 'strict-dates')
            [CompletionResult]::new('--pattern-columns', 'pattern-columns', [CompletionResultType]::ParameterName, 'pattern-columns')
            [CompletionResult]::new('--date-whitelist', 'date-whitelist', [CompletionResultType]::ParameterName, 'date-whitelist')
            [CompletionResult]::new('--prefer-dmy', 'prefer-dmy', [CompletionResultType]::ParameterName, 'prefer-dmy')
            [CompletionResult]::new('--force', 'force', [CompletionResultType]::ParameterName, 'force')
            [CompletionResult]::new('--stdout', 'stdout', [CompletionResultType]::ParameterName, 'stdout')
            [CompletionResult]::new('--jobs', 'jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('--no-headers', 'no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('--delimiter', 'delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('--memcheck', 'memcheck', [CompletionResultType]::ParameterName, 'memcheck')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;search' {
            [CompletionResult]::new('--ignore-case', 'ignore-case', [CompletionResultType]::ParameterName, 'ignore-case')
            [CompletionResult]::new('--select', 'select', [CompletionResultType]::ParameterName, 'select')
            [CompletionResult]::new('--invert-match', 'invert-match', [CompletionResultType]::ParameterName, 'invert-match')
            [CompletionResult]::new('--unicode', 'unicode', [CompletionResultType]::ParameterName, 'unicode')
            [CompletionResult]::new('--flag', 'flag', [CompletionResultType]::ParameterName, 'flag')
            [CompletionResult]::new('--quick', 'quick', [CompletionResultType]::ParameterName, 'quick')
            [CompletionResult]::new('--preview-match', 'preview-match', [CompletionResultType]::ParameterName, 'preview-match')
            [CompletionResult]::new('--count', 'count', [CompletionResultType]::ParameterName, 'count')
            [CompletionResult]::new('--size-limit', 'size-limit', [CompletionResultType]::ParameterName, 'size-limit')
            [CompletionResult]::new('--dfa-size-limit', 'dfa-size-limit', [CompletionResultType]::ParameterName, 'dfa-size-limit')
            [CompletionResult]::new('--json', 'json', [CompletionResultType]::ParameterName, 'json')
            [CompletionResult]::new('--not-one', 'not-one', [CompletionResultType]::ParameterName, 'not-one')
            [CompletionResult]::new('--output', 'output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--no-headers', 'no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('--delimiter', 'delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('--progressbar', 'progressbar', [CompletionResultType]::ParameterName, 'progressbar')
            [CompletionResult]::new('--quiet', 'quiet', [CompletionResultType]::ParameterName, 'quiet')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;searchset' {
            [CompletionResult]::new('--ignore-case', 'ignore-case', [CompletionResultType]::ParameterName, 'ignore-case')
            [CompletionResult]::new('--select', 'select', [CompletionResultType]::ParameterName, 'select')
            [CompletionResult]::new('--invert-match', 'invert-match', [CompletionResultType]::ParameterName, 'invert-match')
            [CompletionResult]::new('--unicode', 'unicode', [CompletionResultType]::ParameterName, 'unicode')
            [CompletionResult]::new('--flag', 'flag', [CompletionResultType]::ParameterName, 'flag')
            [CompletionResult]::new('--flag-matches-only', 'flag-matches-only', [CompletionResultType]::ParameterName, 'flag-matches-only')
            [CompletionResult]::new('--unmatched-output', 'unmatched-output', [CompletionResultType]::ParameterName, 'unmatched-output')
            [CompletionResult]::new('--quick', 'quick', [CompletionResultType]::ParameterName, 'quick')
            [CompletionResult]::new('--count', 'count', [CompletionResultType]::ParameterName, 'count')
            [CompletionResult]::new('--json', 'json', [CompletionResultType]::ParameterName, 'json')
            [CompletionResult]::new('--size-limit', 'size-limit', [CompletionResultType]::ParameterName, 'size-limit')
            [CompletionResult]::new('--dfa-size-limit', 'dfa-size-limit', [CompletionResultType]::ParameterName, 'dfa-size-limit')
            [CompletionResult]::new('--output', 'output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--no-headers', 'no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('--delimiter', 'delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('--progressbar', 'progressbar', [CompletionResultType]::ParameterName, 'progressbar')
            [CompletionResult]::new('--quiet', 'quiet', [CompletionResultType]::ParameterName, 'quiet')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;select' {
            [CompletionResult]::new('--random', 'random', [CompletionResultType]::ParameterName, 'random')
            [CompletionResult]::new('--seed', 'seed', [CompletionResultType]::ParameterName, 'seed')
            [CompletionResult]::new('--sort', 'sort', [CompletionResultType]::ParameterName, 'sort')
            [CompletionResult]::new('--output', 'output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--no-headers', 'no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('--delimiter', 'delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;slice' {
            [CompletionResult]::new('--start', 'start', [CompletionResultType]::ParameterName, 'start')
            [CompletionResult]::new('--end', 'end', [CompletionResultType]::ParameterName, 'end')
            [CompletionResult]::new('--len', 'len', [CompletionResultType]::ParameterName, 'len')
            [CompletionResult]::new('--index', 'index', [CompletionResultType]::ParameterName, 'index')
            [CompletionResult]::new('--json', 'json', [CompletionResultType]::ParameterName, 'json')
            [CompletionResult]::new('--output', 'output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--no-headers', 'no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('--delimiter', 'delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;snappy' {
            [CompletionResult]::new('--user-agent', 'user-agent', [CompletionResultType]::ParameterName, 'user-agent')
            [CompletionResult]::new('--timeout', 'timeout', [CompletionResultType]::ParameterName, 'timeout')
            [CompletionResult]::new('--output', 'output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--jobs', 'jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('--quiet', 'quiet', [CompletionResultType]::ParameterName, 'quiet')
            [CompletionResult]::new('--progressbar', 'progressbar', [CompletionResultType]::ParameterName, 'progressbar')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('compress', 'compress', [CompletionResultType]::ParameterValue, 'compress')
            [CompletionResult]::new('decompress', 'decompress', [CompletionResultType]::ParameterValue, 'decompress')
            [CompletionResult]::new('check', 'check', [CompletionResultType]::ParameterValue, 'check')
            [CompletionResult]::new('validate', 'validate', [CompletionResultType]::ParameterValue, 'validate')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'qsv;snappy;compress' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;snappy;decompress' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;snappy;check' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;snappy;validate' {
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;snappy;help' {
            [CompletionResult]::new('compress', 'compress', [CompletionResultType]::ParameterValue, 'compress')
            [CompletionResult]::new('decompress', 'decompress', [CompletionResultType]::ParameterValue, 'decompress')
            [CompletionResult]::new('check', 'check', [CompletionResultType]::ParameterValue, 'check')
            [CompletionResult]::new('validate', 'validate', [CompletionResultType]::ParameterValue, 'validate')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'qsv;snappy;help;compress' {
            break
        }
        'qsv;snappy;help;decompress' {
            break
        }
        'qsv;snappy;help;check' {
            break
        }
        'qsv;snappy;help;validate' {
            break
        }
        'qsv;snappy;help;help' {
            break
        }
        'qsv;sniff' {
            [CompletionResult]::new('--sample', 'sample', [CompletionResultType]::ParameterName, 'sample')
            [CompletionResult]::new('--prefer-dmy', 'prefer-dmy', [CompletionResultType]::ParameterName, 'prefer-dmy')
            [CompletionResult]::new('--delimiter', 'delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('--quote', 'quote', [CompletionResultType]::ParameterName, 'quote')
            [CompletionResult]::new('--json', 'json', [CompletionResultType]::ParameterName, 'json')
            [CompletionResult]::new('--pretty-json', 'pretty-json', [CompletionResultType]::ParameterName, 'pretty-json')
            [CompletionResult]::new('--save-urlsample', 'save-urlsample', [CompletionResultType]::ParameterName, 'save-urlsample')
            [CompletionResult]::new('--timeout', 'timeout', [CompletionResultType]::ParameterName, 'timeout')
            [CompletionResult]::new('--user-agent', 'user-agent', [CompletionResultType]::ParameterName, 'user-agent')
            [CompletionResult]::new('--stats-types', 'stats-types', [CompletionResultType]::ParameterName, 'stats-types')
            [CompletionResult]::new('--no-infer', 'no-infer', [CompletionResultType]::ParameterName, 'no-infer')
            [CompletionResult]::new('--just-mime', 'just-mime', [CompletionResultType]::ParameterName, 'just-mime')
            [CompletionResult]::new('--quick', 'quick', [CompletionResultType]::ParameterName, 'quick')
            [CompletionResult]::new('--harvest-mode', 'harvest-mode', [CompletionResultType]::ParameterName, 'harvest-mode')
            [CompletionResult]::new('--progressbar', 'progressbar', [CompletionResultType]::ParameterName, 'progressbar')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;sort' {
            [CompletionResult]::new('--select', 'select', [CompletionResultType]::ParameterName, 'select')
            [CompletionResult]::new('--numeric', 'numeric', [CompletionResultType]::ParameterName, 'numeric')
            [CompletionResult]::new('--reverse', 'reverse', [CompletionResultType]::ParameterName, 'reverse')
            [CompletionResult]::new('--ignore-case', 'ignore-case', [CompletionResultType]::ParameterName, 'ignore-case')
            [CompletionResult]::new('--unique', 'unique', [CompletionResultType]::ParameterName, 'unique')
            [CompletionResult]::new('--random', 'random', [CompletionResultType]::ParameterName, 'random')
            [CompletionResult]::new('--seed', 'seed', [CompletionResultType]::ParameterName, 'seed')
            [CompletionResult]::new('--rng', 'rng', [CompletionResultType]::ParameterName, 'rng')
            [CompletionResult]::new('--jobs', 'jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('--faster', 'faster', [CompletionResultType]::ParameterName, 'faster')
            [CompletionResult]::new('--output', 'output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--no-headers', 'no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('--delimiter', 'delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('--memcheck', 'memcheck', [CompletionResultType]::ParameterName, 'memcheck')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;sortcheck' {
            [CompletionResult]::new('--select', 'select', [CompletionResultType]::ParameterName, 'select')
            [CompletionResult]::new('--ignore-case', 'ignore-case', [CompletionResultType]::ParameterName, 'ignore-case')
            [CompletionResult]::new('--all', 'all', [CompletionResultType]::ParameterName, 'all')
            [CompletionResult]::new('--json', 'json', [CompletionResultType]::ParameterName, 'json')
            [CompletionResult]::new('--pretty-json', 'pretty-json', [CompletionResultType]::ParameterName, 'pretty-json')
            [CompletionResult]::new('--no-headers', 'no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('--delimiter', 'delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('--progressbar', 'progressbar', [CompletionResultType]::ParameterName, 'progressbar')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;split' {
            [CompletionResult]::new('--size', 'size', [CompletionResultType]::ParameterName, 'size')
            [CompletionResult]::new('--chunks', 'chunks', [CompletionResultType]::ParameterName, 'chunks')
            [CompletionResult]::new('--kb-size', 'kb-size', [CompletionResultType]::ParameterName, 'kb-size')
            [CompletionResult]::new('--jobs', 'jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('--filename', 'filename', [CompletionResultType]::ParameterName, 'filename')
            [CompletionResult]::new('--pad', 'pad', [CompletionResultType]::ParameterName, 'pad')
            [CompletionResult]::new('--no-headers', 'no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('--delimiter', 'delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('--quiet', 'quiet', [CompletionResultType]::ParameterName, 'quiet')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;sqlp' {
            [CompletionResult]::new('--format', 'format', [CompletionResultType]::ParameterName, 'format')
            [CompletionResult]::new('--try-parsedates', 'try-parsedates', [CompletionResultType]::ParameterName, 'try-parsedates')
            [CompletionResult]::new('--infer-len', 'infer-len', [CompletionResultType]::ParameterName, 'infer-len')
            [CompletionResult]::new('--streaming', 'streaming', [CompletionResultType]::ParameterName, 'streaming')
            [CompletionResult]::new('--low-memory', 'low-memory', [CompletionResultType]::ParameterName, 'low-memory')
            [CompletionResult]::new('--no-optimizations', 'no-optimizations', [CompletionResultType]::ParameterName, 'no-optimizations')
            [CompletionResult]::new('--truncate-ragged-lines', 'truncate-ragged-lines', [CompletionResultType]::ParameterName, 'truncate-ragged-lines')
            [CompletionResult]::new('--ignore-errors', 'ignore-errors', [CompletionResultType]::ParameterName, 'ignore-errors')
            [CompletionResult]::new('--rnull-values', 'rnull-values', [CompletionResultType]::ParameterName, 'rnull-values')
            [CompletionResult]::new('--decimal-comma', 'decimal-comma', [CompletionResultType]::ParameterName, 'decimal-comma')
            [CompletionResult]::new('--datetime-format', 'datetime-format', [CompletionResultType]::ParameterName, 'datetime-format')
            [CompletionResult]::new('--date-format', 'date-format', [CompletionResultType]::ParameterName, 'date-format')
            [CompletionResult]::new('--time-format', 'time-format', [CompletionResultType]::ParameterName, 'time-format')
            [CompletionResult]::new('--float-precision', 'float-precision', [CompletionResultType]::ParameterName, 'float-precision')
            [CompletionResult]::new('--wnull-value', 'wnull-value', [CompletionResultType]::ParameterName, 'wnull-value')
            [CompletionResult]::new('--compression', 'compression', [CompletionResultType]::ParameterName, 'compression')
            [CompletionResult]::new('--compress-level', 'compress-level', [CompletionResultType]::ParameterName, 'compress-level')
            [CompletionResult]::new('--statistics', 'statistics', [CompletionResultType]::ParameterName, 'statistics')
            [CompletionResult]::new('--output', 'output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--delimiter', 'delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('--quiet', 'quiet', [CompletionResultType]::ParameterName, 'quiet')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;stats' {
            [CompletionResult]::new('--select', 'select', [CompletionResultType]::ParameterName, 'select')
            [CompletionResult]::new('--everything', 'everything', [CompletionResultType]::ParameterName, 'everything')
            [CompletionResult]::new('--typesonly', 'typesonly', [CompletionResultType]::ParameterName, 'typesonly')
            [CompletionResult]::new('--infer-boolean', 'infer-boolean', [CompletionResultType]::ParameterName, 'infer-boolean')
            [CompletionResult]::new('--mode', 'mode', [CompletionResultType]::ParameterName, 'mode')
            [CompletionResult]::new('--cardinality', 'cardinality', [CompletionResultType]::ParameterName, 'cardinality')
            [CompletionResult]::new('--median', 'median', [CompletionResultType]::ParameterName, 'median')
            [CompletionResult]::new('--mad', 'mad', [CompletionResultType]::ParameterName, 'mad')
            [CompletionResult]::new('--quartiles', 'quartiles', [CompletionResultType]::ParameterName, 'quartiles')
            [CompletionResult]::new('--round', 'round', [CompletionResultType]::ParameterName, 'round')
            [CompletionResult]::new('--nulls', 'nulls', [CompletionResultType]::ParameterName, 'nulls')
            [CompletionResult]::new('--infer-dates', 'infer-dates', [CompletionResultType]::ParameterName, 'infer-dates')
            [CompletionResult]::new('--prefer-dmy', 'prefer-dmy', [CompletionResultType]::ParameterName, 'prefer-dmy')
            [CompletionResult]::new('--force', 'force', [CompletionResultType]::ParameterName, 'force')
            [CompletionResult]::new('--jobs', 'jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('--stats-binout', 'stats-binout', [CompletionResultType]::ParameterName, 'stats-binout')
            [CompletionResult]::new('--cache-threshold', 'cache-threshold', [CompletionResultType]::ParameterName, 'cache-threshold')
            [CompletionResult]::new('--output', 'output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--no-headers', 'no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('--delimiter', 'delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('--memcheck', 'memcheck', [CompletionResultType]::ParameterName, 'memcheck')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;table' {
            [CompletionResult]::new('--width', 'width', [CompletionResultType]::ParameterName, 'width')
            [CompletionResult]::new('--pad', 'pad', [CompletionResultType]::ParameterName, 'pad')
            [CompletionResult]::new('--align', 'align', [CompletionResultType]::ParameterName, 'align')
            [CompletionResult]::new('--condense', 'condense', [CompletionResultType]::ParameterName, 'condense')
            [CompletionResult]::new('--output', 'output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--delimiter', 'delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('--memcheck', 'memcheck', [CompletionResultType]::ParameterName, 'memcheck')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;to' {
            [CompletionResult]::new('--print-package', 'print-package', [CompletionResultType]::ParameterName, 'print-package')
            [CompletionResult]::new('--dump', 'dump', [CompletionResultType]::ParameterName, 'dump')
            [CompletionResult]::new('--stats', 'stats', [CompletionResultType]::ParameterName, 'stats')
            [CompletionResult]::new('--stats-csv', 'stats-csv', [CompletionResultType]::ParameterName, 'stats-csv')
            [CompletionResult]::new('--quiet', 'quiet', [CompletionResultType]::ParameterName, 'quiet')
            [CompletionResult]::new('--schema', 'schema', [CompletionResultType]::ParameterName, 'schema')
            [CompletionResult]::new('--drop', 'drop', [CompletionResultType]::ParameterName, 'drop')
            [CompletionResult]::new('--evolve', 'evolve', [CompletionResultType]::ParameterName, 'evolve')
            [CompletionResult]::new('--pipe', 'pipe', [CompletionResultType]::ParameterName, 'pipe')
            [CompletionResult]::new('--separator', 'separator', [CompletionResultType]::ParameterName, 'separator')
            [CompletionResult]::new('--jobs', 'jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('--delimiter', 'delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;tojsonl' {
            [CompletionResult]::new('--trim', 'trim', [CompletionResultType]::ParameterName, 'trim')
            [CompletionResult]::new('--no-boolean', 'no-boolean', [CompletionResultType]::ParameterName, 'no-boolean')
            [CompletionResult]::new('--jobs', 'jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('--batch', 'batch', [CompletionResultType]::ParameterName, 'batch')
            [CompletionResult]::new('--delimiter', 'delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('--output', 'output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--memcheck', 'memcheck', [CompletionResultType]::ParameterName, 'memcheck')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;transpose' {
            [CompletionResult]::new('--multipass', 'multipass', [CompletionResultType]::ParameterName, 'multipass')
            [CompletionResult]::new('--output', 'output', [CompletionResultType]::ParameterName, 'output')
            [CompletionResult]::new('--delimiter', 'delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('--memcheck', 'memcheck', [CompletionResultType]::ParameterName, 'memcheck')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;validate' {
            [CompletionResult]::new('--trim', 'trim', [CompletionResultType]::ParameterName, 'trim')
            [CompletionResult]::new('--fail-fast', 'fail-fast', [CompletionResultType]::ParameterName, 'fail-fast')
            [CompletionResult]::new('--valid', 'valid', [CompletionResultType]::ParameterName, 'valid')
            [CompletionResult]::new('--invalid', 'invalid', [CompletionResultType]::ParameterName, 'invalid')
            [CompletionResult]::new('--json', 'json', [CompletionResultType]::ParameterName, 'json')
            [CompletionResult]::new('--pretty-json', 'pretty-json', [CompletionResultType]::ParameterName, 'pretty-json')
            [CompletionResult]::new('--valid-output', 'valid-output', [CompletionResultType]::ParameterName, 'valid-output')
            [CompletionResult]::new('--jobs', 'jobs', [CompletionResultType]::ParameterName, 'jobs')
            [CompletionResult]::new('--batch', 'batch', [CompletionResultType]::ParameterName, 'batch')
            [CompletionResult]::new('--timeout', 'timeout', [CompletionResultType]::ParameterName, 'timeout')
            [CompletionResult]::new('--no-headers', 'no-headers', [CompletionResultType]::ParameterName, 'no-headers')
            [CompletionResult]::new('--delimiter', 'delimiter', [CompletionResultType]::ParameterName, 'delimiter')
            [CompletionResult]::new('--progressbar', 'progressbar', [CompletionResultType]::ParameterName, 'progressbar')
            [CompletionResult]::new('--quiet', 'quiet', [CompletionResultType]::ParameterName, 'quiet')
            [CompletionResult]::new('-h', 'h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', 'help', [CompletionResultType]::ParameterName, 'Print help')
            break
        }
        'qsv;help' {
            [CompletionResult]::new('apply', 'apply', [CompletionResultType]::ParameterValue, 'apply')
            [CompletionResult]::new('behead', 'behead', [CompletionResultType]::ParameterValue, 'behead')
            [CompletionResult]::new('cat', 'cat', [CompletionResultType]::ParameterValue, 'cat')
            [CompletionResult]::new('clipboard', 'clipboard', [CompletionResultType]::ParameterValue, 'clipboard')
            [CompletionResult]::new('count', 'count', [CompletionResultType]::ParameterValue, 'count')
            [CompletionResult]::new('datefmt', 'datefmt', [CompletionResultType]::ParameterValue, 'datefmt')
            [CompletionResult]::new('dedup', 'dedup', [CompletionResultType]::ParameterValue, 'dedup')
            [CompletionResult]::new('describegpt', 'describegpt', [CompletionResultType]::ParameterValue, 'describegpt')
            [CompletionResult]::new('diff', 'diff', [CompletionResultType]::ParameterValue, 'diff')
            [CompletionResult]::new('enum', 'enum', [CompletionResultType]::ParameterValue, 'enum')
            [CompletionResult]::new('excel', 'excel', [CompletionResultType]::ParameterValue, 'excel')
            [CompletionResult]::new('exclude', 'exclude', [CompletionResultType]::ParameterValue, 'exclude')
            [CompletionResult]::new('extdedup', 'extdedup', [CompletionResultType]::ParameterValue, 'extdedup')
            [CompletionResult]::new('extsort', 'extsort', [CompletionResultType]::ParameterValue, 'extsort')
            [CompletionResult]::new('explode', 'explode', [CompletionResultType]::ParameterValue, 'explode')
            [CompletionResult]::new('fetch', 'fetch', [CompletionResultType]::ParameterValue, 'fetch')
            [CompletionResult]::new('fetchpost', 'fetchpost', [CompletionResultType]::ParameterValue, 'fetchpost')
            [CompletionResult]::new('fill', 'fill', [CompletionResultType]::ParameterValue, 'fill')
            [CompletionResult]::new('fixlengths', 'fixlengths', [CompletionResultType]::ParameterValue, 'fixlengths')
            [CompletionResult]::new('flatten', 'flatten', [CompletionResultType]::ParameterValue, 'flatten')
            [CompletionResult]::new('fmt', 'fmt', [CompletionResultType]::ParameterValue, 'fmt')
            [CompletionResult]::new('foreach', 'foreach', [CompletionResultType]::ParameterValue, 'foreach')
            [CompletionResult]::new('frequency', 'frequency', [CompletionResultType]::ParameterValue, 'frequency')
            [CompletionResult]::new('geocode', 'geocode', [CompletionResultType]::ParameterValue, 'geocode')
            [CompletionResult]::new('headers', 'headers', [CompletionResultType]::ParameterValue, 'headers')
            [CompletionResult]::new('index', 'index', [CompletionResultType]::ParameterValue, 'index')
            [CompletionResult]::new('input', 'input', [CompletionResultType]::ParameterValue, 'input')
            [CompletionResult]::new('join', 'join', [CompletionResultType]::ParameterValue, 'join')
            [CompletionResult]::new('joinp', 'joinp', [CompletionResultType]::ParameterValue, 'joinp')
            [CompletionResult]::new('json', 'json', [CompletionResultType]::ParameterValue, 'json')
            [CompletionResult]::new('jsonl', 'jsonl', [CompletionResultType]::ParameterValue, 'jsonl')
            [CompletionResult]::new('luau', 'luau', [CompletionResultType]::ParameterValue, 'luau')
            [CompletionResult]::new('partition', 'partition', [CompletionResultType]::ParameterValue, 'partition')
            [CompletionResult]::new('prompt', 'prompt', [CompletionResultType]::ParameterValue, 'prompt')
            [CompletionResult]::new('pseudo', 'pseudo', [CompletionResultType]::ParameterValue, 'pseudo')
            [CompletionResult]::new('py', 'py', [CompletionResultType]::ParameterValue, 'py')
            [CompletionResult]::new('rename', 'rename', [CompletionResultType]::ParameterValue, 'rename')
            [CompletionResult]::new('replace', 'replace', [CompletionResultType]::ParameterValue, 'replace')
            [CompletionResult]::new('reverse', 'reverse', [CompletionResultType]::ParameterValue, 'reverse')
            [CompletionResult]::new('safenames', 'safenames', [CompletionResultType]::ParameterValue, 'safenames')
            [CompletionResult]::new('sample', 'sample', [CompletionResultType]::ParameterValue, 'sample')
            [CompletionResult]::new('schema', 'schema', [CompletionResultType]::ParameterValue, 'schema')
            [CompletionResult]::new('search', 'search', [CompletionResultType]::ParameterValue, 'search')
            [CompletionResult]::new('searchset', 'searchset', [CompletionResultType]::ParameterValue, 'searchset')
            [CompletionResult]::new('select', 'select', [CompletionResultType]::ParameterValue, 'select')
            [CompletionResult]::new('slice', 'slice', [CompletionResultType]::ParameterValue, 'slice')
            [CompletionResult]::new('snappy', 'snappy', [CompletionResultType]::ParameterValue, 'snappy')
            [CompletionResult]::new('sniff', 'sniff', [CompletionResultType]::ParameterValue, 'sniff')
            [CompletionResult]::new('sort', 'sort', [CompletionResultType]::ParameterValue, 'sort')
            [CompletionResult]::new('sortcheck', 'sortcheck', [CompletionResultType]::ParameterValue, 'sortcheck')
            [CompletionResult]::new('split', 'split', [CompletionResultType]::ParameterValue, 'split')
            [CompletionResult]::new('sqlp', 'sqlp', [CompletionResultType]::ParameterValue, 'sqlp')
            [CompletionResult]::new('stats', 'stats', [CompletionResultType]::ParameterValue, 'stats')
            [CompletionResult]::new('table', 'table', [CompletionResultType]::ParameterValue, 'table')
            [CompletionResult]::new('to', 'to', [CompletionResultType]::ParameterValue, 'to')
            [CompletionResult]::new('tojsonl', 'tojsonl', [CompletionResultType]::ParameterValue, 'tojsonl')
            [CompletionResult]::new('transpose', 'transpose', [CompletionResultType]::ParameterValue, 'transpose')
            [CompletionResult]::new('validate', 'validate', [CompletionResultType]::ParameterValue, 'validate')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'qsv;help;apply' {
            [CompletionResult]::new('operations', 'operations', [CompletionResultType]::ParameterValue, 'operations')
            [CompletionResult]::new('emptyreplace', 'emptyreplace', [CompletionResultType]::ParameterValue, 'emptyreplace')
            [CompletionResult]::new('dynfmt', 'dynfmt', [CompletionResultType]::ParameterValue, 'dynfmt')
            [CompletionResult]::new('calcconv', 'calcconv', [CompletionResultType]::ParameterValue, 'calcconv')
            break
        }
        'qsv;help;apply;operations' {
            break
        }
        'qsv;help;apply;emptyreplace' {
            break
        }
        'qsv;help;apply;dynfmt' {
            break
        }
        'qsv;help;apply;calcconv' {
            break
        }
        'qsv;help;behead' {
            break
        }
        'qsv;help;cat' {
            [CompletionResult]::new('rows', 'rows', [CompletionResultType]::ParameterValue, 'rows')
            [CompletionResult]::new('rowskey', 'rowskey', [CompletionResultType]::ParameterValue, 'rowskey')
            [CompletionResult]::new('columns', 'columns', [CompletionResultType]::ParameterValue, 'columns')
            break
        }
        'qsv;help;cat;rows' {
            break
        }
        'qsv;help;cat;rowskey' {
            break
        }
        'qsv;help;cat;columns' {
            break
        }
        'qsv;help;clipboard' {
            break
        }
        'qsv;help;count' {
            break
        }
        'qsv;help;datefmt' {
            break
        }
        'qsv;help;dedup' {
            break
        }
        'qsv;help;describegpt' {
            break
        }
        'qsv;help;diff' {
            break
        }
        'qsv;help;enum' {
            break
        }
        'qsv;help;excel' {
            break
        }
        'qsv;help;exclude' {
            break
        }
        'qsv;help;extdedup' {
            break
        }
        'qsv;help;extsort' {
            break
        }
        'qsv;help;explode' {
            break
        }
        'qsv;help;fetch' {
            break
        }
        'qsv;help;fetchpost' {
            break
        }
        'qsv;help;fill' {
            break
        }
        'qsv;help;fixlengths' {
            break
        }
        'qsv;help;flatten' {
            break
        }
        'qsv;help;fmt' {
            break
        }
        'qsv;help;foreach' {
            break
        }
        'qsv;help;frequency' {
            break
        }
        'qsv;help;geocode' {
            break
        }
        'qsv;help;headers' {
            break
        }
        'qsv;help;index' {
            break
        }
        'qsv;help;input' {
            break
        }
        'qsv;help;join' {
            break
        }
        'qsv;help;joinp' {
            break
        }
        'qsv;help;json' {
            break
        }
        'qsv;help;jsonl' {
            break
        }
        'qsv;help;luau' {
            break
        }
        'qsv;help;partition' {
            break
        }
        'qsv;help;prompt' {
            break
        }
        'qsv;help;pseudo' {
            break
        }
        'qsv;help;py' {
            break
        }
        'qsv;help;rename' {
            break
        }
        'qsv;help;replace' {
            break
        }
        'qsv;help;reverse' {
            break
        }
        'qsv;help;safenames' {
            break
        }
        'qsv;help;sample' {
            break
        }
        'qsv;help;schema' {
            break
        }
        'qsv;help;search' {
            break
        }
        'qsv;help;searchset' {
            break
        }
        'qsv;help;select' {
            break
        }
        'qsv;help;slice' {
            break
        }
        'qsv;help;snappy' {
            [CompletionResult]::new('compress', 'compress', [CompletionResultType]::ParameterValue, 'compress')
            [CompletionResult]::new('decompress', 'decompress', [CompletionResultType]::ParameterValue, 'decompress')
            [CompletionResult]::new('check', 'check', [CompletionResultType]::ParameterValue, 'check')
            [CompletionResult]::new('validate', 'validate', [CompletionResultType]::ParameterValue, 'validate')
            break
        }
        'qsv;help;snappy;compress' {
            break
        }
        'qsv;help;snappy;decompress' {
            break
        }
        'qsv;help;snappy;check' {
            break
        }
        'qsv;help;snappy;validate' {
            break
        }
        'qsv;help;sniff' {
            break
        }
        'qsv;help;sort' {
            break
        }
        'qsv;help;sortcheck' {
            break
        }
        'qsv;help;split' {
            break
        }
        'qsv;help;sqlp' {
            break
        }
        'qsv;help;stats' {
            break
        }
        'qsv;help;table' {
            break
        }
        'qsv;help;to' {
            break
        }
        'qsv;help;tojsonl' {
            break
        }
        'qsv;help;transpose' {
            break
        }
        'qsv;help;validate' {
            break
        }
        'qsv;help;help' {
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}
