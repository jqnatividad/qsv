
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
            [CompletionResult]::new('clipboard', 'clipboard', [CompletionResultType]::ParameterValue, 'clipboard')
            [CompletionResult]::new('count', 'count', [CompletionResultType]::ParameterValue, 'count')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
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
        'qsv;help' {
            [CompletionResult]::new('clipboard', 'clipboard', [CompletionResultType]::ParameterValue, 'clipboard')
            [CompletionResult]::new('count', 'count', [CompletionResultType]::ParameterValue, 'count')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'qsv;help;clipboard' {
            break
        }
        'qsv;help;count' {
            break
        }
        'qsv;help;help' {
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}
