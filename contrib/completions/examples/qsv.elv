
use builtin;
use str;

set edit:completion:arg-completer[qsv] = {|@words|
    fn spaces {|n|
        builtin:repeat $n ' ' | str:join ''
    }
    fn cand {|text desc|
        edit:complex-candidate $text &display=$text' '(spaces (- 14 (wcswidth $text)))$desc
    }
    var command = 'qsv'
    for word $words[1..-1] {
        if (str:has-prefix $word '-') {
            break
        }
        set command = $command';'$word
    }
    var completions = [
        &'qsv'= {
            cand --list 'list'
            cand --envlist 'envlist'
            cand --update 'update'
            cand --updatenow 'updatenow'
            cand --version 'version'
            cand -h 'Print help'
            cand --help 'Print help'
            cand clipboard 'clipboard'
            cand count 'count'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'qsv;clipboard'= {
            cand --save 'save'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'qsv;count'= {
            cand --human-readable 'human-readable'
            cand --width 'width'
            cand --no-polars 'no-polars'
            cand --low-memory 'low-memory'
            cand --flexible 'flexible'
            cand --no-headers 'no-headers'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'qsv;help'= {
            cand clipboard 'clipboard'
            cand count 'count'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'qsv;help;clipboard'= {
        }
        &'qsv;help;count'= {
        }
        &'qsv;help;help'= {
        }
    ]
    $completions[$command]
}
