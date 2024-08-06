
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
            cand apply 'apply'
            cand behead 'behead'
            cand cat 'cat'
            cand clipboard 'clipboard'
            cand count 'count'
            cand datefmt 'datefmt'
            cand dedup 'dedup'
            cand describegpt 'describegpt'
            cand diff 'diff'
            cand enum 'enum'
            cand excel 'excel'
            cand exclude 'exclude'
            cand extdedup 'extdedup'
            cand extsort 'extsort'
            cand explode 'explode'
            cand fetch 'fetch'
            cand fetchpost 'fetchpost'
            cand fill 'fill'
            cand fixlengths 'fixlengths'
            cand flatten 'flatten'
            cand fmt 'fmt'
            cand foreach 'foreach'
            cand frequency 'frequency'
            cand geocode 'geocode'
            cand headers 'headers'
            cand index 'index'
            cand input 'input'
            cand join 'join'
            cand joinp 'joinp'
            cand json 'json'
            cand jsonl 'jsonl'
            cand luau 'luau'
            cand partition 'partition'
            cand prompt 'prompt'
            cand pseudo 'pseudo'
            cand py 'py'
            cand rename 'rename'
            cand replace 'replace'
            cand reverse 'reverse'
            cand safenames 'safenames'
            cand sample 'sample'
            cand schema 'schema'
            cand search 'search'
            cand searchset 'searchset'
            cand select 'select'
            cand slice 'slice'
            cand snappy 'snappy'
            cand sniff 'sniff'
            cand sort 'sort'
            cand sortcheck 'sortcheck'
            cand split 'split'
            cand sqlp 'sqlp'
            cand stats 'stats'
            cand table 'table'
            cand to 'to'
            cand tojsonl 'tojsonl'
            cand transpose 'transpose'
            cand validate 'validate'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'qsv;apply'= {
            cand --new-column 'new-column'
            cand --rename 'rename'
            cand --comparand 'comparand'
            cand --replacement 'replacement'
            cand --formatstr 'formatstr'
            cand --jobs 'jobs'
            cand --batch 'batch'
            cand --output 'output'
            cand --no-headers 'no-headers'
            cand --delimiter 'delimiter'
            cand --progressbar 'progressbar'
            cand -h 'Print help'
            cand --help 'Print help'
            cand operations 'operations'
            cand emptyreplace 'emptyreplace'
            cand dynfmt 'dynfmt'
            cand calcconv 'calcconv'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'qsv;apply;operations'= {
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'qsv;apply;emptyreplace'= {
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'qsv;apply;dynfmt'= {
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'qsv;apply;calcconv'= {
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'qsv;apply;help'= {
            cand operations 'operations'
            cand emptyreplace 'emptyreplace'
            cand dynfmt 'dynfmt'
            cand calcconv 'calcconv'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'qsv;apply;help;operations'= {
        }
        &'qsv;apply;help;emptyreplace'= {
        }
        &'qsv;apply;help;dynfmt'= {
        }
        &'qsv;apply;help;calcconv'= {
        }
        &'qsv;apply;help;help'= {
        }
        &'qsv;behead'= {
            cand --flexible 'flexible'
            cand --output 'output'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'qsv;cat'= {
            cand --output 'output'
            cand --no-headers 'no-headers'
            cand --delimiter 'delimiter'
            cand -h 'Print help'
            cand --help 'Print help'
            cand rows 'rows'
            cand rowskey 'rowskey'
            cand columns 'columns'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'qsv;cat;rows'= {
            cand --flexible 'flexible'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'qsv;cat;rowskey'= {
            cand --group 'group'
            cand --group-name 'group-name'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'qsv;cat;columns'= {
            cand --pad 'pad'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'qsv;cat;help'= {
            cand rows 'rows'
            cand rowskey 'rowskey'
            cand columns 'columns'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'qsv;cat;help;rows'= {
        }
        &'qsv;cat;help;rowskey'= {
        }
        &'qsv;cat;help;columns'= {
        }
        &'qsv;cat;help;help'= {
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
        &'qsv;datefmt'= {
            cand --formatstr 'formatstr'
            cand --new-column 'new-column'
            cand --rename 'rename'
            cand --prefer-dmy 'prefer-dmy'
            cand --keep-zero-time 'keep-zero-time'
            cand --input-tz 'input-tz'
            cand --output-tz 'output-tz'
            cand --default-tz 'default-tz'
            cand --utc 'utc'
            cand --zulu 'zulu'
            cand --ts-resolution 'ts-resolution'
            cand --jobs 'jobs'
            cand --batch 'batch'
            cand --output 'output'
            cand --no-headers 'no-headers'
            cand --delimiter 'delimiter'
            cand --progressbar 'progressbar'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'qsv;dedup'= {
            cand --select 'select'
            cand --numeric 'numeric'
            cand --ignore-case 'ignore-case'
            cand --sorted 'sorted'
            cand --dupes-output 'dupes-output'
            cand --human-readable 'human-readable'
            cand --jobs 'jobs'
            cand --output 'output'
            cand --no-headers 'no-headers'
            cand --delimiter 'delimiter'
            cand --quiet 'quiet'
            cand --memcheck 'memcheck'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'qsv;describegpt'= {
            cand --all 'all'
            cand --description 'description'
            cand --dictionary 'dictionary'
            cand --tags 'tags'
            cand --api-key 'api-key'
            cand --max-tokens 'max-tokens'
            cand --json 'json'
            cand --jsonl 'jsonl'
            cand --prompt 'prompt'
            cand --prompt-file 'prompt-file'
            cand --base-url 'base-url'
            cand --model 'model'
            cand --timeout 'timeout'
            cand --user-agent 'user-agent'
            cand --output 'output'
            cand --quiet 'quiet'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'qsv;diff'= {
            cand --no-headers-left 'no-headers-left'
            cand --no-headers-right 'no-headers-right'
            cand --no-headers-output 'no-headers-output'
            cand --delimiter-left 'delimiter-left'
            cand --delimiter-right 'delimiter-right'
            cand --delimiter-output 'delimiter-output'
            cand --key 'key'
            cand --sort-columns 'sort-columns'
            cand --jobs 'jobs'
            cand --output 'output'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'qsv;enum'= {
            cand --new-column 'new-column'
            cand --start 'start'
            cand --increment 'increment'
            cand --constant 'constant'
            cand --copy 'copy'
            cand --uuid4 'uuid4'
            cand --uuid7 'uuid7'
            cand --hash 'hash'
            cand --output 'output'
            cand --no-headers 'no-headers'
            cand --delimiter 'delimiter'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'qsv;excel'= {
            cand --sheet 'sheet'
            cand --metadata 'metadata'
            cand --error-format 'error-format'
            cand --flexible 'flexible'
            cand --trim 'trim'
            cand --date-format 'date-format'
            cand --keep-zero-time 'keep-zero-time'
            cand --range 'range'
            cand --jobs 'jobs'
            cand --output 'output'
            cand --delimiter 'delimiter'
            cand --quiet 'quiet'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'qsv;exclude'= {
            cand --ignore-case 'ignore-case'
            cand -v 'v'
            cand --output 'output'
            cand --no-headers 'no-headers'
            cand --delimiter 'delimiter'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'qsv;extdedup'= {
            cand --no-output 'no-output'
            cand --dupes-output 'dupes-output'
            cand --human-readable 'human-readable'
            cand --memory-limit 'memory-limit'
            cand --quiet 'quiet'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'qsv;extsort'= {
            cand --memory-limit 'memory-limit'
            cand --tmp-dir 'tmp-dir'
            cand --jobs 'jobs'
            cand --no-headers 'no-headers'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'qsv;explode'= {
            cand --rename 'rename'
            cand --output 'output'
            cand --no-headers 'no-headers'
            cand --delimiter 'delimiter'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'qsv;fetch'= {
            cand --url-template 'url-template'
            cand --new-column 'new-column'
            cand --jql 'jql'
            cand --jqlfile 'jqlfile'
            cand --pretty 'pretty'
            cand --rate-limit 'rate-limit'
            cand --timeout 'timeout'
            cand --http-header 'http-header'
            cand --max-retries 'max-retries'
            cand --max-errors 'max-errors'
            cand --store-error 'store-error'
            cand --cookies 'cookies'
            cand --user-agent 'user-agent'
            cand --report 'report'
            cand --no-cache 'no-cache'
            cand --mem-cache-size 'mem-cache-size'
            cand --disk-cache 'disk-cache'
            cand --disk-cache-dir 'disk-cache-dir'
            cand --redis-cache 'redis-cache'
            cand --cache-error 'cache-error'
            cand --flush-cache 'flush-cache'
            cand --output 'output'
            cand --no-headers 'no-headers'
            cand --delimiter 'delimiter'
            cand --progressbar 'progressbar'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'qsv;fetchpost'= {
            cand --new-column 'new-column'
            cand --jql 'jql'
            cand --jqlfile 'jqlfile'
            cand --pretty 'pretty'
            cand --rate-limit 'rate-limit'
            cand --timeout 'timeout'
            cand --http-header 'http-header'
            cand --compress 'compress'
            cand --max-retries 'max-retries'
            cand --max-errors 'max-errors'
            cand --store-error 'store-error'
            cand --cookies 'cookies'
            cand --user-agent 'user-agent'
            cand --report 'report'
            cand --no-cache 'no-cache'
            cand --mem-cache-size 'mem-cache-size'
            cand --disk-cache 'disk-cache'
            cand --disk-cache-dir 'disk-cache-dir'
            cand --redis-cache 'redis-cache'
            cand --cache-error 'cache-error'
            cand --flush-cache 'flush-cache'
            cand --output 'output'
            cand --no-headers 'no-headers'
            cand --delimiter 'delimiter'
            cand --progressbar 'progressbar'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'qsv;fill'= {
            cand --groupby 'groupby'
            cand --first 'first'
            cand --backfill 'backfill'
            cand --default 'default'
            cand --output 'output'
            cand --no-headers 'no-headers'
            cand --delimiter 'delimiter'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'qsv;fixlengths'= {
            cand --length 'length'
            cand --insert 'insert'
            cand --output 'output'
            cand --delimiter 'delimiter'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'qsv;flatten'= {
            cand --condense 'condense'
            cand --field-separator 'field-separator'
            cand --separator 'separator'
            cand --no-headers 'no-headers'
            cand --delimiter 'delimiter'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'qsv;fmt'= {
            cand --out-delimiter 'out-delimiter'
            cand --crlf 'crlf'
            cand --ascii 'ascii'
            cand --quote 'quote'
            cand --quote-always 'quote-always'
            cand --quote-never 'quote-never'
            cand --escape 'escape'
            cand --no-final-newline 'no-final-newline'
            cand --output 'output'
            cand --delimiter 'delimiter'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'qsv;foreach'= {
            cand --unify 'unify'
            cand --new-column 'new-column'
            cand --dry-run 'dry-run'
            cand --no-headers 'no-headers'
            cand --delimiter 'delimiter'
            cand --progressbar 'progressbar'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'qsv;frequency'= {
            cand --select 'select'
            cand --limit 'limit'
            cand --unq-limit 'unq-limit'
            cand --lmt-threshold 'lmt-threshold'
            cand --pct-dec-places 'pct-dec-places'
            cand --other-sorted 'other-sorted'
            cand --other-text 'other-text'
            cand --asc 'asc'
            cand --no-trim 'no-trim'
            cand --ignore-case 'ignore-case'
            cand --stats-mode 'stats-mode'
            cand --all-unique-text 'all-unique-text'
            cand --jobs 'jobs'
            cand --output 'output'
            cand --no-headers 'no-headers'
            cand --delimiter 'delimiter'
            cand --memcheck 'memcheck'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'qsv;geocode'= {
            cand --new-column 'new-column'
            cand --rename 'rename'
            cand --country 'country'
            cand --min-score 'min-score'
            cand --admin1 'admin1'
            cand --k_weight 'k_weight'
            cand --formatstr 'formatstr'
            cand --language 'language'
            cand --invalid-result 'invalid-result'
            cand --jobs 'jobs'
            cand --batch 'batch'
            cand --timeout 'timeout'
            cand --cache-dir 'cache-dir'
            cand --languages 'languages'
            cand --cities-url 'cities-url'
            cand --force 'force'
            cand --output 'output'
            cand --delimiter 'delimiter'
            cand --progressbar 'progressbar'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'qsv;headers'= {
            cand --just-names 'just-names'
            cand --just-count 'just-count'
            cand --intersect 'intersect'
            cand --trim 'trim'
            cand --delimiter 'delimiter'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'qsv;index'= {
            cand --output 'output'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'qsv;input'= {
            cand --quote 'quote'
            cand --escape 'escape'
            cand --no-quoting 'no-quoting'
            cand --quote-style 'quote-style'
            cand --skip-lines 'skip-lines'
            cand --auto-skip 'auto-skip'
            cand --skip-lastlines 'skip-lastlines'
            cand --trim-headers 'trim-headers'
            cand --trim-fields 'trim-fields'
            cand --comment 'comment'
            cand --encoding-errors 'encoding-errors'
            cand --output 'output'
            cand --delimiter 'delimiter'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'qsv;join'= {
            cand --ignore-case 'ignore-case'
            cand --left-anti 'left-anti'
            cand --left-semi 'left-semi'
            cand --right 'right'
            cand --full 'full'
            cand --cross 'cross'
            cand --nulls 'nulls'
            cand --output 'output'
            cand --no-headers 'no-headers'
            cand --delimiter 'delimiter'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'qsv;joinp'= {
            cand --left 'left'
            cand --left-anti 'left-anti'
            cand --left-semi 'left-semi'
            cand --right 'right'
            cand --full 'full'
            cand --cross 'cross'
            cand --coalesce 'coalesce'
            cand --filter-left 'filter-left'
            cand --filter-right 'filter-right'
            cand --validate 'validate'
            cand --nulls 'nulls'
            cand --streaming 'streaming'
            cand --try-parsedates 'try-parsedates'
            cand --infer-len 'infer-len'
            cand --low-memory 'low-memory'
            cand --no-optimizations 'no-optimizations'
            cand --ignore-errors 'ignore-errors'
            cand --decimal-comma 'decimal-comma'
            cand --asof 'asof'
            cand --left_by 'left_by'
            cand --right_by 'right_by'
            cand --strategy 'strategy'
            cand --tolerance 'tolerance'
            cand --sql-filter 'sql-filter'
            cand --datetime-format 'datetime-format'
            cand --date-format 'date-format'
            cand --time-format 'time-format'
            cand --float-precision 'float-precision'
            cand --null-value 'null-value'
            cand --output 'output'
            cand --delimiter 'delimiter'
            cand --quiet 'quiet'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'qsv;json'= {
            cand --jaq 'jaq'
            cand --select 'select'
            cand --output 'output'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'qsv;jsonl'= {
            cand --ignore-errors 'ignore-errors'
            cand --jobs 'jobs'
            cand --batch 'batch'
            cand --output 'output'
            cand --delimiter 'delimiter'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'qsv;luau'= {
            cand --no-globals 'no-globals'
            cand --colindex 'colindex'
            cand --remap 'remap'
            cand --begin 'begin'
            cand --luau-path 'luau-path'
            cand --max-errors 'max-errors'
            cand --timeout 'timeout'
            cand --ckan-api 'ckan-api'
            cand --ckan-token 'ckan-token'
            cand --cache-dir 'cache-dir'
            cand --output 'output'
            cand --no-headers 'no-headers'
            cand --delimiter 'delimiter'
            cand --progressbar 'progressbar'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'qsv;partition'= {
            cand --filename 'filename'
            cand --prefix-length 'prefix-length'
            cand --drop 'drop'
            cand --no-headers 'no-headers'
            cand --delimiter 'delimiter'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'qsv;prompt'= {
            cand --msg 'msg'
            cand --filters 'filters'
            cand --workdir 'workdir'
            cand --fd-output 'fd-output'
            cand --save-fname 'save-fname'
            cand --base-delay-ms 'base-delay-ms'
            cand --output 'output'
            cand --quiet 'quiet'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'qsv;pseudo'= {
            cand --start 'start'
            cand --increment 'increment'
            cand --formatstr 'formatstr'
            cand --no-headers 'no-headers'
            cand --delimiter 'delimiter'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'qsv;py'= {
            cand --helper 'helper'
            cand --batch 'batch'
            cand --output 'output'
            cand --no-headers 'no-headers'
            cand --delimiter 'delimiter'
            cand --progressbar 'progressbar'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'qsv;rename'= {
            cand --output 'output'
            cand --no-headers 'no-headers'
            cand --delimiter 'delimiter'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'qsv;replace'= {
            cand --ignore-case 'ignore-case'
            cand --select 'select'
            cand --unicode 'unicode'
            cand --size-limit 'size-limit'
            cand --dfa-size-limit 'dfa-size-limit'
            cand --output 'output'
            cand --no-headers 'no-headers'
            cand --delimiter 'delimiter'
            cand --progressbar 'progressbar'
            cand --quiet 'quiet'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'qsv;reverse'= {
            cand --output 'output'
            cand --no-headers 'no-headers'
            cand --delimiter 'delimiter'
            cand --memcheck 'memcheck'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'qsv;safenames'= {
            cand --mode 'mode'
            cand --reserved 'reserved'
            cand --prefix 'prefix'
            cand --output 'output'
            cand --delimiter 'delimiter'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'qsv;sample'= {
            cand --seed 'seed'
            cand --rng 'rng'
            cand --user-agent 'user-agent'
            cand --timeout 'timeout'
            cand --output 'output'
            cand --no-headers 'no-headers'
            cand --delimiter 'delimiter'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'qsv;schema'= {
            cand --enum-threshold 'enum-threshold'
            cand --ignore-case 'ignore-case'
            cand --strict-dates 'strict-dates'
            cand --pattern-columns 'pattern-columns'
            cand --date-whitelist 'date-whitelist'
            cand --prefer-dmy 'prefer-dmy'
            cand --force 'force'
            cand --stdout 'stdout'
            cand --jobs 'jobs'
            cand --no-headers 'no-headers'
            cand --delimiter 'delimiter'
            cand --memcheck 'memcheck'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'qsv;search'= {
            cand --ignore-case 'ignore-case'
            cand --select 'select'
            cand --invert-match 'invert-match'
            cand --unicode 'unicode'
            cand --flag 'flag'
            cand --quick 'quick'
            cand --preview-match 'preview-match'
            cand --count 'count'
            cand --size-limit 'size-limit'
            cand --dfa-size-limit 'dfa-size-limit'
            cand --json 'json'
            cand --not-one 'not-one'
            cand --output 'output'
            cand --no-headers 'no-headers'
            cand --delimiter 'delimiter'
            cand --progressbar 'progressbar'
            cand --quiet 'quiet'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'qsv;searchset'= {
            cand --ignore-case 'ignore-case'
            cand --select 'select'
            cand --invert-match 'invert-match'
            cand --unicode 'unicode'
            cand --flag 'flag'
            cand --flag-matches-only 'flag-matches-only'
            cand --unmatched-output 'unmatched-output'
            cand --quick 'quick'
            cand --count 'count'
            cand --json 'json'
            cand --not-one 'not-one'
            cand --size-limit 'size-limit'
            cand --dfa-size-limit 'dfa-size-limit'
            cand --output 'output'
            cand --no-headers 'no-headers'
            cand --delimiter 'delimiter'
            cand --progressbar 'progressbar'
            cand --quiet 'quiet'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'qsv;select'= {
            cand --random 'random'
            cand --seed 'seed'
            cand --sort 'sort'
            cand --output 'output'
            cand --no-headers 'no-headers'
            cand --delimiter 'delimiter'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'qsv;slice'= {
            cand --start 'start'
            cand --end 'end'
            cand --len 'len'
            cand --index 'index'
            cand --json 'json'
            cand --output 'output'
            cand --no-headers 'no-headers'
            cand --delimiter 'delimiter'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'qsv;snappy'= {
            cand --user-agent 'user-agent'
            cand --timeout 'timeout'
            cand --output 'output'
            cand --jobs 'jobs'
            cand --quiet 'quiet'
            cand --progressbar 'progressbar'
            cand -h 'Print help'
            cand --help 'Print help'
            cand compress 'compress'
            cand decompress 'decompress'
            cand check 'check'
            cand validate 'validate'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'qsv;snappy;compress'= {
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'qsv;snappy;decompress'= {
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'qsv;snappy;check'= {
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'qsv;snappy;validate'= {
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'qsv;snappy;help'= {
            cand compress 'compress'
            cand decompress 'decompress'
            cand check 'check'
            cand validate 'validate'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'qsv;snappy;help;compress'= {
        }
        &'qsv;snappy;help;decompress'= {
        }
        &'qsv;snappy;help;check'= {
        }
        &'qsv;snappy;help;validate'= {
        }
        &'qsv;snappy;help;help'= {
        }
        &'qsv;sniff'= {
            cand --sample 'sample'
            cand --prefer-dmy 'prefer-dmy'
            cand --delimiter 'delimiter'
            cand --quote 'quote'
            cand --json 'json'
            cand --pretty-json 'pretty-json'
            cand --save-urlsample 'save-urlsample'
            cand --timeout 'timeout'
            cand --user-agent 'user-agent'
            cand --stats-types 'stats-types'
            cand --no-infer 'no-infer'
            cand --just-mime 'just-mime'
            cand --quick 'quick'
            cand --harvest-mode 'harvest-mode'
            cand --progressbar 'progressbar'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'qsv;sort'= {
            cand --select 'select'
            cand --numeric 'numeric'
            cand --reverse 'reverse'
            cand --ignore-case 'ignore-case'
            cand --unique 'unique'
            cand --random 'random'
            cand --seed 'seed'
            cand --rng 'rng'
            cand --jobs 'jobs'
            cand --faster 'faster'
            cand --output 'output'
            cand --no-headers 'no-headers'
            cand --delimiter 'delimiter'
            cand --memcheck 'memcheck'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'qsv;sortcheck'= {
            cand --select 'select'
            cand --ignore-case 'ignore-case'
            cand --all 'all'
            cand --json 'json'
            cand --pretty-json 'pretty-json'
            cand --no-headers 'no-headers'
            cand --delimiter 'delimiter'
            cand --progressbar 'progressbar'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'qsv;split'= {
            cand --size 'size'
            cand --chunks 'chunks'
            cand --kb-size 'kb-size'
            cand --jobs 'jobs'
            cand --filename 'filename'
            cand --pad 'pad'
            cand --no-headers 'no-headers'
            cand --delimiter 'delimiter'
            cand --quiet 'quiet'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'qsv;sqlp'= {
            cand --format 'format'
            cand --try-parsedates 'try-parsedates'
            cand --infer-len 'infer-len'
            cand --streaming 'streaming'
            cand --low-memory 'low-memory'
            cand --no-optimizations 'no-optimizations'
            cand --truncate-ragged-lines 'truncate-ragged-lines'
            cand --ignore-errors 'ignore-errors'
            cand --rnull-values 'rnull-values'
            cand --decimal-comma 'decimal-comma'
            cand --datetime-format 'datetime-format'
            cand --date-format 'date-format'
            cand --time-format 'time-format'
            cand --float-precision 'float-precision'
            cand --wnull-value 'wnull-value'
            cand --compression 'compression'
            cand --compress-level 'compress-level'
            cand --statistics 'statistics'
            cand --output 'output'
            cand --delimiter 'delimiter'
            cand --quiet 'quiet'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'qsv;stats'= {
            cand --select 'select'
            cand --everything 'everything'
            cand --typesonly 'typesonly'
            cand --infer-boolean 'infer-boolean'
            cand --mode 'mode'
            cand --cardinality 'cardinality'
            cand --median 'median'
            cand --mad 'mad'
            cand --quartiles 'quartiles'
            cand --round 'round'
            cand --nulls 'nulls'
            cand --infer-dates 'infer-dates'
            cand --prefer-dmy 'prefer-dmy'
            cand --force 'force'
            cand --jobs 'jobs'
            cand --stats-binout 'stats-binout'
            cand --cache-threshold 'cache-threshold'
            cand --output 'output'
            cand --no-headers 'no-headers'
            cand --delimiter 'delimiter'
            cand --memcheck 'memcheck'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'qsv;table'= {
            cand --width 'width'
            cand --pad 'pad'
            cand --align 'align'
            cand --condense 'condense'
            cand --output 'output'
            cand --delimiter 'delimiter'
            cand --memcheck 'memcheck'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'qsv;to'= {
            cand --print-package 'print-package'
            cand --dump 'dump'
            cand --stats 'stats'
            cand --stats-csv 'stats-csv'
            cand --quiet 'quiet'
            cand --schema 'schema'
            cand --drop 'drop'
            cand --evolve 'evolve'
            cand --pipe 'pipe'
            cand --separator 'separator'
            cand --jobs 'jobs'
            cand --delimiter 'delimiter'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'qsv;tojsonl'= {
            cand --trim 'trim'
            cand --no-boolean 'no-boolean'
            cand --jobs 'jobs'
            cand --batch 'batch'
            cand --delimiter 'delimiter'
            cand --output 'output'
            cand --memcheck 'memcheck'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'qsv;transpose'= {
            cand --multipass 'multipass'
            cand --output 'output'
            cand --delimiter 'delimiter'
            cand --memcheck 'memcheck'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'qsv;validate'= {
            cand --trim 'trim'
            cand --fail-fast 'fail-fast'
            cand --valid 'valid'
            cand --invalid 'invalid'
            cand --json 'json'
            cand --pretty-json 'pretty-json'
            cand --valid-output 'valid-output'
            cand --jobs 'jobs'
            cand --batch 'batch'
            cand --timeout 'timeout'
            cand --no-headers 'no-headers'
            cand --delimiter 'delimiter'
            cand --progressbar 'progressbar'
            cand --quiet 'quiet'
            cand -h 'Print help'
            cand --help 'Print help'
        }
        &'qsv;help'= {
            cand apply 'apply'
            cand behead 'behead'
            cand cat 'cat'
            cand clipboard 'clipboard'
            cand count 'count'
            cand datefmt 'datefmt'
            cand dedup 'dedup'
            cand describegpt 'describegpt'
            cand diff 'diff'
            cand enum 'enum'
            cand excel 'excel'
            cand exclude 'exclude'
            cand extdedup 'extdedup'
            cand extsort 'extsort'
            cand explode 'explode'
            cand fetch 'fetch'
            cand fetchpost 'fetchpost'
            cand fill 'fill'
            cand fixlengths 'fixlengths'
            cand flatten 'flatten'
            cand fmt 'fmt'
            cand foreach 'foreach'
            cand frequency 'frequency'
            cand geocode 'geocode'
            cand headers 'headers'
            cand index 'index'
            cand input 'input'
            cand join 'join'
            cand joinp 'joinp'
            cand json 'json'
            cand jsonl 'jsonl'
            cand luau 'luau'
            cand partition 'partition'
            cand prompt 'prompt'
            cand pseudo 'pseudo'
            cand py 'py'
            cand rename 'rename'
            cand replace 'replace'
            cand reverse 'reverse'
            cand safenames 'safenames'
            cand sample 'sample'
            cand schema 'schema'
            cand search 'search'
            cand searchset 'searchset'
            cand select 'select'
            cand slice 'slice'
            cand snappy 'snappy'
            cand sniff 'sniff'
            cand sort 'sort'
            cand sortcheck 'sortcheck'
            cand split 'split'
            cand sqlp 'sqlp'
            cand stats 'stats'
            cand table 'table'
            cand to 'to'
            cand tojsonl 'tojsonl'
            cand transpose 'transpose'
            cand validate 'validate'
            cand help 'Print this message or the help of the given subcommand(s)'
        }
        &'qsv;help;apply'= {
            cand operations 'operations'
            cand emptyreplace 'emptyreplace'
            cand dynfmt 'dynfmt'
            cand calcconv 'calcconv'
        }
        &'qsv;help;apply;operations'= {
        }
        &'qsv;help;apply;emptyreplace'= {
        }
        &'qsv;help;apply;dynfmt'= {
        }
        &'qsv;help;apply;calcconv'= {
        }
        &'qsv;help;behead'= {
        }
        &'qsv;help;cat'= {
            cand rows 'rows'
            cand rowskey 'rowskey'
            cand columns 'columns'
        }
        &'qsv;help;cat;rows'= {
        }
        &'qsv;help;cat;rowskey'= {
        }
        &'qsv;help;cat;columns'= {
        }
        &'qsv;help;clipboard'= {
        }
        &'qsv;help;count'= {
        }
        &'qsv;help;datefmt'= {
        }
        &'qsv;help;dedup'= {
        }
        &'qsv;help;describegpt'= {
        }
        &'qsv;help;diff'= {
        }
        &'qsv;help;enum'= {
        }
        &'qsv;help;excel'= {
        }
        &'qsv;help;exclude'= {
        }
        &'qsv;help;extdedup'= {
        }
        &'qsv;help;extsort'= {
        }
        &'qsv;help;explode'= {
        }
        &'qsv;help;fetch'= {
        }
        &'qsv;help;fetchpost'= {
        }
        &'qsv;help;fill'= {
        }
        &'qsv;help;fixlengths'= {
        }
        &'qsv;help;flatten'= {
        }
        &'qsv;help;fmt'= {
        }
        &'qsv;help;foreach'= {
        }
        &'qsv;help;frequency'= {
        }
        &'qsv;help;geocode'= {
        }
        &'qsv;help;headers'= {
        }
        &'qsv;help;index'= {
        }
        &'qsv;help;input'= {
        }
        &'qsv;help;join'= {
        }
        &'qsv;help;joinp'= {
        }
        &'qsv;help;json'= {
        }
        &'qsv;help;jsonl'= {
        }
        &'qsv;help;luau'= {
        }
        &'qsv;help;partition'= {
        }
        &'qsv;help;prompt'= {
        }
        &'qsv;help;pseudo'= {
        }
        &'qsv;help;py'= {
        }
        &'qsv;help;rename'= {
        }
        &'qsv;help;replace'= {
        }
        &'qsv;help;reverse'= {
        }
        &'qsv;help;safenames'= {
        }
        &'qsv;help;sample'= {
        }
        &'qsv;help;schema'= {
        }
        &'qsv;help;search'= {
        }
        &'qsv;help;searchset'= {
        }
        &'qsv;help;select'= {
        }
        &'qsv;help;slice'= {
        }
        &'qsv;help;snappy'= {
            cand compress 'compress'
            cand decompress 'decompress'
            cand check 'check'
            cand validate 'validate'
        }
        &'qsv;help;snappy;compress'= {
        }
        &'qsv;help;snappy;decompress'= {
        }
        &'qsv;help;snappy;check'= {
        }
        &'qsv;help;snappy;validate'= {
        }
        &'qsv;help;sniff'= {
        }
        &'qsv;help;sort'= {
        }
        &'qsv;help;sortcheck'= {
        }
        &'qsv;help;split'= {
        }
        &'qsv;help;sqlp'= {
        }
        &'qsv;help;stats'= {
        }
        &'qsv;help;table'= {
        }
        &'qsv;help;to'= {
        }
        &'qsv;help;tojsonl'= {
        }
        &'qsv;help;transpose'= {
        }
        &'qsv;help;validate'= {
        }
        &'qsv;help;help'= {
        }
    ]
    $completions[$command]
}
