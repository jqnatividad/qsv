static USAGE: &str = r#"
Create multiple new computed columns, filter rows or compute aggregations by 
executing a Luau 0.606 script for every row (SEQUENTIAL MODE) or for
specified rows (RANDOM ACCESS MODE) of a CSV file.

Luau is not just another qsv command. It is qsv's Domain-Specific Language (DSL)
for data-wrangling. 👑

The executed Luau has 3 ways to reference row columns (as strings):
  1. Directly by using column name (e.g. Amount), can be disabled with -g
  2. Indexing col variable by column name: col.Amount or col["Total Balance"]
  3. Indexing col variable by column 1-based index: col[1], col[2], etc.
     This is only available with the --colindex and --no-headers options.

Of course, if your input has no headers, then 3. will be the only available
option.

Some usage examples:

  Sum numeric columns 'a' and 'b' and call new column 'c'
  $ qsv luau map c "a + b"
  $ qsv luau map c "col.a + col['b']"
  $ qsv luau map c "col[1] + col[2]"

  There is some magic in the previous example as 'a' and 'b' are passed in
  as strings (not numbers), but Luau still manages to add them up.
  A more explicit way of doing it, is by using tonumber
  $ qsv luau map c "tonumber(a) + tonumber(b)"

  Add running total column for Amount
  $ qsv luau map Total "tot = (tot or 0) + Amount; return tot"

  Or use the --begin and --end options to compute the running & grand totals
  $ qsv luau map Total --begin "tot = 0; gtotal = 0" \
        "tot = tot + Amount; gtotal = gtotal + tot; return tot" --end "return gtotal"

  Add running total column for Amount when previous balance was 900
  $ qsv luau map Total "tot = (tot or 900) + Amount; return tot"

  Convert Amount to always-positive AbsAmount and Type (debit/credit) columns
  $ qsv luau map Type \
        "if tonumber(Amount) < 0 then return 'debit' else return 'credit' end" | \
    qsv luau map AbsAmount "math.abs(tonumber(Amount))"

  Map multiple new columns in one pass
  $ qsv luau map newcol1,newcol2,newcol3 "{cola + 1, colb + 2, colc + 3}"

  Filter some rows based on numerical filtering
  $ qsv luau filter "tonumber(a) > 45"
  $ qsv luau filter "tonumber(a) >= tonumber(b)"

  Typing long scripts on the command line gets tiresome rather quickly. Use the
  "file:" prefix or the ".lua/.luau" file extension to read non-trivial scripts 
  from the filesystem.

  In the following example, both the BEGIN and END scripts have the lua/luau file
  extension so they are read from the filesystem.  With the debitcredit.script file,
  we use the "file:" prefix to read it from the filesystem.

    $ qsv luau map Type -B init.lua file:debitcredit.script -E end.luau

With "luau map", if the MAIN script is invalid for a row, "<ERROR>" followed by a 
detailed error message is returned for that row.
With "luau filter", if the MAIN script is invalid for a row, that row is not filtered.

If any row has an invalid result, an exitcode of 1 is returned and an error count
is logged.

SPECIAL VARIABLES:
  "_IDX" - a READ-only variable that is zero during the BEGIN script and
       set to the current row number during the MAIN & END scripts.

       It is primarily used in SEQUENTIAL MODE when the CSV has no index or you
       wish to process the CSV sequentially.
 
  "_INDEX" - a READ/WRITE variable that enables RANDOM ACCESS MODE when used in
       a script. Using "_INDEX" in a script switches qsv to RANDOM ACCESS MODE 
       where setting it to a row number will change the current row to the
       specified row number. It will only work, however, if the CSV has an index.

       When using _INDEX, the MAIN script will keep looping and evaluate the row
       specified by _INDEX until _INDEX is set to an invalid row number
       (e.g. <= zero or to a value greater than _ROWCOUNT).

       If the CSV has no index, qsv will abort with an error unless "qsv_autoindex()"
       is called in the BEGIN script to create an index.
       
  "_ROWCOUNT" - a READ-only variable which is zero during the BEGIN & MAIN scripts, 
       and set to the rowcount during the END script when the CSV has no index
       (SEQUENTIAL MODE).

       When using _INDEX and the CSV has an index, _ROWCOUNT will be set to the
       rowcount of the CSV file, even from the BEGINning
       (RANDOM ACCESS MODE).

  "_LASTROW" - a READ-only variable that is set to the last row number of the CSV.
       Like _INDEX, it will also trigger RANDOM ACCESS MODE if used in a script.

       Similarly, if the CSV has no index, qsv will also abort with an error unless
       "qsv_autoindex()" is called in the BEGIN script to create an index.

For security and safety reasons as a purpose-built embeddable interpreter,
Luau's standard library is relatively minimal (https://luau-lang.org/library).
That's why qsv preloads the LuaDate library as date manipulation is a common task.
See https://tieske.github.io/date/#date-id96473 on how to use the LuaDate library.

Additional libraries can be loaded from the LUAU_PATH using luau's "require" function.
See https://github.com/LewisJEllis/awesome-lua for a list of other libraries.

With the judicious use of "require", the BEGIN script & special variables, one can
create variables, tables, arrays & functions that can be used for complex aggregation
operations in the END script.

SCRIPT DEVELOPMENT TIPS:
When developing Luau scripts, be sure to take advantage of the "qsv_log" function to
debug your script. It will log messages at the level (INFO, WARN, ERROR, DEBUG, TRACE)
specified by the QSV_LOG_LEVEL environment variable (see docs/Logging.md for details).

At the DEBUG level, the log messages will be more verbose to faciitate debugging.
It will also skip precompiling the MAIN script to bytecode so you can see more
detailed error messages with line numbers.

Bear in mind that qsv strips comments from Luau scripts before executing them.
This is done so qsv doesn't falsely trigger on special variables mentioned in comments.
When checking line numbers in DEBUG mode, be sure to refer to the comment-stripped
scripts in the log file, not the original commented scripts.

There are more Luau helper functions in addition to "qsv_log" - "qsv_break", "qsv_skip",
"qsv_insertrecord", "qsv_autoindex", "qsv_coalesce", "qsv_sleep", "qsv_writefile",
"qsv_cmd", "qsv_shellcmd", "qsv_setenv", "qsv_getenv" and last but not least -
the powerful "qsv_register_lookup" which allows you to "lookup" values against other
CSVs on the filesystem, a URL, datHere's lookup repo or CKAN instances.

Detailed descriptions of these helpers can be found in the "setup_helpers" section at
the bottom of this file.

For more detailed examples, see https://github.com/jqnatividad/qsv/blob/master/tests/test_luau.rs.

Usage:
    qsv luau map [options] -n <main-script> [<input>]
    qsv luau map [options] <new-columns> <main-script> [<input>]
    qsv luau filter [options] <main-script> [<input>]
    qsv luau map --help
    qsv luau filter --help
    qsv luau --help

Luau arguments:

    All <script> arguments/options can either be the Luau code, or if it starts with
    "file:" or ends with ".luau/.lua" - the filepath from which to load the script.

    Instead of using the --begin and --end options, you can also embed BEGIN and END
    scripts in the MAIN script by using the "BEGIN { ... }!" and "END { ... }!" syntax.

    The BEGIN script is embedded in the MAIN script by adding a BEGIN block at the
    top of the script. The BEGIN block must start at the beginning of the line.
    It can contain multiple statements.

    The MAIN script is the main Luau script to execute. It is executed for EACH ROW of
    the input CSV. It can contain multiple statements and should end with a "return" stmt.
    In map mode, the return value is/are the new value/s of the mapped column/s.
    In filter mode, the return value is a boolean indicating if the row should be filtered.

    The END script is embedded in the MAIN script by adding an END block at the bottom
    of the script. The END block must start at the beginning of the line.
    It can contain multiple statements.

    <new-columns> is a comma-separated list of new computed columns to add to the CSV
    when using "luau map". Note that the new columns are added to the CSV after the
    existing columns.

Luau options:
  -g, --no-globals        Don't create Luau global variables for each column,
                          only `col`. Useful when some column names mask standard
                          Luau globals and a bit more performance.
                          Note: access to Luau globals thru _G remains even with -g.
  --colindex              Create a 1-based column index. Useful when some column names
                          mask standard Luau globals. Automatically enabled with--no-headers.
  -r, --remap             Only the listed new columns are written to the output CSV.
                          Only applies to "map" subcommand.
  -B, --begin <script>    Luau script/file to execute in the BEGINning, before
                          processing the CSV with the main-script.
                          Typically used to initialize global variables.
                          Takes precedence over an embedded BEGIN script.
                          If <script> begins with "file:" or ends with ".luau/.lua",
                          it's interpreted as a filepath from which to load the script.
  -E, --end <script>      Luau script/file to execute at the END, after processing the
                          CSV with the main-script.
                          Typically used for aggregations.
                          The output of the END script is sent to stderr.
                          Takes precedence over an embedded END script.
                          If <script> begins with "file:" or ends with ".luau/.lua",
                          it's interpreted as a filepath from which to load the script.
  --luau-path <pattern>   The LUAU_PATH pattern to use from which the scripts 
                          can "require" lua/luau library files from.
                          See https://www.lua.org/pil/8.1.html
                          [default: ?;?.luau;?.lua]
  --max-errors <count>    The maximum number of errors to tolerate before aborting.
                          Set to zero to disable error limit.
                          [default: 100]
  --timeout <seconds>     Timeout for downloading lookup_tables using
                          the qsv_register_lookup() helper function.
                          [default: 30]
  --ckan-api <url>        The URL of the CKAN API to use for downloading lookup_table
                          resources using the qsv_register_lookup() helper function
                          with the "ckan://" scheme.
                          If the QSV_CKAN_API envvar is set, it will be used instead.
                          [default: https://data.dathere.com/api/3/action]
  --ckan-token <token>    The CKAN API token to use. Only required if downloading
                          private resources.
                          If the QSV_CKAN_TOKEN envvar is set, it will be used instead.
  --cache-dir <dir>       The directory to use for caching downloaded lookup_table
                          resources using the qsv_register_lookup() helper function.
                          If the directory does not exist, qsv will attempt to create it.
                          If the QSV_CACHE_DIR envvar is set, it will be used instead.
                          [default: ~/.qsv-cache]

Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
    -n, --no-headers       When set, the first row will not be interpreted
                           as headers. Automatically enables --colindex option.
    -d, --delimiter <arg>  The field delimiter for reading CSV data.
                           Must be a single character. (default: ,)
    -p, --progressbar      Show progress bars. Not valid for stdin.
                           Ignored in qsvdp.
                           In SEQUENTIAL MODE, the progress bar will show the
                           number of rows processed.
                           In RANDOM ACCESS MODE, the progress bar will show
                           the position of the current row being processed.
                           Enabling this option will also suppress stderr output
                           from the END script.
"#;

use std::{
    env, fs, io,
    io::Write,
    path::Path,
    sync::atomic::{AtomicBool, AtomicI8, AtomicU16, Ordering},
    time::Instant,
};

use csv_index::RandomAccessSimple;
#[cfg(any(feature = "feature_capable", feature = "lite"))]
use indicatif::{ProgressBar, ProgressDrawTarget, ProgressStyle};
use log::{debug, info, log_enabled};
use mlua::{Lua, LuaSerdeExt, Value};
use serde::Deserialize;
use simple_home_dir::expand_tilde;
use strum_macros::IntoStaticStr;
use tempfile;

use crate::{
    config::{Config, Delimiter, DEFAULT_WTR_BUFFER_CAPACITY},
    util, CliError, CliResult,
};

#[derive(Deserialize)]
struct Args {
    cmd_map:          bool,
    cmd_filter:       bool,
    arg_new_columns:  Option<String>,
    arg_main_script:  String,
    arg_input:        Option<String>,
    flag_no_globals:  bool,
    flag_colindex:    bool,
    flag_remap:       bool,
    flag_begin:       Option<String>,
    flag_end:         Option<String>,
    flag_luau_path:   String,
    flag_output:      Option<String>,
    flag_no_headers:  bool,
    flag_delimiter:   Option<Delimiter>,
    flag_progressbar: bool,
    flag_max_errors:  usize,
    flag_timeout:     u16,
    flag_ckan_api:    String,
    flag_ckan_token:  Option<String>,
    flag_cache_dir:   String,
}

impl From<mlua::Error> for CliError {
    fn from(err: mlua::Error) -> CliError {
        CliError::Other(err.to_string())
    }
}

static QSV_BREAK: AtomicBool = AtomicBool::new(false);
static QSV_SKIP: AtomicBool = AtomicBool::new(false);

// internal variables
static QSV_BREAK_MSG: &str = "_QSV_BRKMSG";
static QSV_INSERTRECORD_TBL: &str = "_QSV_IR_TBL";
static QSV_CACHE_DIR: &str = "_QSV_CACHE_DIR";

// special variables that can be used in scripts
static QSV_V_IDX: &str = "_IDX";
static QSV_V_ROWCOUNT: &str = "_ROWCOUNT";
static QSV_V_LASTROW: &str = "_LASTROW";
static QSV_V_INDEX: &str = "_INDEX";

// there are 3 stages: 1-BEGIN, 2-MAIN, 3-END
#[repr(i8)]
#[derive(IntoStaticStr)]
enum Stage {
    Begin = 1,
    Main  = 2,
    End   = 3,
}

impl TryFrom<i8> for Stage {
    type Error = ();

    fn try_from(v: i8) -> Result<Self, Self::Error> {
        match v {
            x if x == Stage::Begin as i8 => Ok(Stage::Begin),
            x if x == Stage::Main as i8 => Ok(Stage::Main),
            x if x == Stage::End as i8 => Ok(Stage::End),
            _ => Err(()),
        }
    }
}

static LUAU_STAGE: AtomicI8 = AtomicI8::new(0);

static TIMEOUT_SECS: AtomicU16 = AtomicU16::new(30);

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;

    // safety: its safe since flag_timeout is a u16
    TIMEOUT_SECS.store(
        util::timeout_secs(args.flag_timeout)?.try_into().unwrap(),
        Ordering::Relaxed,
    );

    let rconfig = Config::new(&args.arg_input)
        .delimiter(args.flag_delimiter)
        .no_headers(args.flag_no_headers);

    let mut luau_script = if let Some(script_filepath) = args.arg_main_script.strip_prefix("file:")
    {
        match fs::read_to_string(script_filepath) {
            Ok(file_contents) => file_contents,
            Err(e) => return fail_clierror!("Cannot load Luau file: {e}"),
        }
    } else if std::path::Path::new(&args.arg_main_script)
        .extension()
        .map_or(false, |ext| ext.eq_ignore_ascii_case("luau"))
        || std::path::Path::new(&args.arg_main_script)
            .extension()
            .map_or(false, |ext| ext.eq_ignore_ascii_case("lua"))
    {
        match fs::read_to_string(args.arg_main_script.clone()) {
            Ok(file_contents) => file_contents,
            Err(e) => return fail_clierror!("Cannot load .lua/.luau file: {e}"),
        }
    } else {
        args.arg_main_script.clone()
    };

    // in Luau, comments begin with two consecutive hyphens
    // let's remove them, so we don't falsely trigger on commented special variables
    let comment_remover_re = regex::Regex::new(r"(?m)(^\s*?--.*?$)").unwrap();
    luau_script = comment_remover_re.replace_all(&luau_script, "").to_string();

    let mut index_file_used =
        luau_script.contains(QSV_V_INDEX) || luau_script.contains(QSV_V_LASTROW);

    // check if the main script has BEGIN and END blocks
    // and if so, extract them and remove them from the main script
    let begin_re = regex::Regex::new(r"(?ms)^BEGIN \{(?P<begin_block>.*?)\}!").unwrap();
    let end_re = regex::Regex::new(r"(?ms)^END \{(?P<end_block>.*?)\}!").unwrap();

    let mut embedded_begin_script = String::new();
    let mut embedded_end_script = String::new();
    let mut main_script = luau_script.clone();

    if let Some(caps) = begin_re.captures(&luau_script) {
        embedded_begin_script = caps["begin_block"].to_string();
        let begin_block_replace = format!("BEGIN {{{embedded_begin_script}}}!");
        debug!("begin_block_replace: {begin_block_replace:?}");
        main_script = main_script.replace(&begin_block_replace, "");
    }
    if let Some(caps) = end_re.captures(&main_script) {
        embedded_end_script = caps["end_block"].to_string();
        let end_block_replace = format!("END {{{embedded_end_script}}}!");
        main_script = main_script.replace(&end_block_replace, "");
    }
    luau_script = main_script;

    // if the main script is a single expression, we need to prepend a return statement
    let mut main_script = if luau_script.contains("return") {
        String::new()
    } else {
        String::from("return ")
    };

    main_script.push_str(luau_script.trim());
    debug!("MAIN script: {main_script:?}");

    // check if a BEGIN script was specified
    let begin_script = if let Some(ref begin) = args.flag_begin {
        let discrete_begin = if let Some(begin_filepath) = begin.strip_prefix("file:") {
            match fs::read_to_string(begin_filepath) {
                Ok(begin) => begin,
                Err(e) => return fail_clierror!("Cannot load Luau BEGIN script file: {e}"),
            }
        } else if std::path::Path::new(begin)
            .extension()
            .map_or(false, |ext| ext.eq_ignore_ascii_case("luau"))
            || std::path::Path::new(begin)
                .extension()
                .map_or(false, |ext| ext.eq_ignore_ascii_case("lua"))
        {
            match fs::read_to_string(begin.clone()) {
                Ok(file_contents) => file_contents,
                Err(e) => return fail_clierror!("Cannot load BEGIN .lua/luau file: {e}"),
            }
        } else {
            begin.to_string()
        };
        comment_remover_re
            .replace_all(&discrete_begin, "")
            .to_string()
    } else {
        embedded_begin_script.trim().to_string()
    };

    // check if the BEGIN script uses _INDEX
    index_file_used = index_file_used
        || begin_script.contains(QSV_V_INDEX)
        || begin_script.contains(QSV_V_LASTROW);

    let qsv_register_lookup_used = begin_script.contains("qsv_register_lookup(");
    debug!("BEGIN script: {begin_script:?}");

    // check if an END script was specified
    let end_script = if let Some(ref end) = args.flag_end {
        let discrete_end = if let Some(end_filepath) = end.strip_prefix("file:") {
            match fs::read_to_string(end_filepath) {
                Ok(end) => end,
                Err(e) => return fail_clierror!("Cannot load Luau END script file: {e}"),
            }
        } else if std::path::Path::new(end)
            .extension()
            .map_or(false, |ext| ext.eq_ignore_ascii_case("luau"))
            || std::path::Path::new(end)
                .extension()
                .map_or(false, |ext| ext.eq_ignore_ascii_case("lua"))
        {
            match fs::read_to_string(end.clone()) {
                Ok(file_contents) => file_contents,
                Err(e) => return fail_clierror!("Cannot load END .lua/.luau file: {e}"),
            }
        } else {
            end.to_string()
        };
        comment_remover_re
            .replace_all(&discrete_end, "")
            .to_string()
    } else {
        embedded_end_script.trim().to_string()
    };
    // check if the END script uses _INDEX
    index_file_used =
        index_file_used || end_script.contains(QSV_V_INDEX) || end_script.contains(QSV_V_LASTROW);
    debug!("END script: {end_script:?}");

    // check if "require" was used in the scripts. If so, we need to setup LUAU_PATH;
    // we check for '= require "' using a robust regex pattern.
    // \u0022 is the unicode codepoint for a double quote.
    let requires_re = regex::Regex::new(r"(?mi)=[[:blank:]]*require[[:blank:]]+\u0022").unwrap();
    let require_used = requires_re.is_match(&main_script)
        || requires_re.is_match(&begin_script)
        || requires_re.is_match(&end_script);

    // if require_used, create a temporary directory and copy date.lua there.
    // we do this outside the "require_used" setup below as the tempdir
    // needs to persist until the end of the program.
    let temp_dir = if require_used {
        match tempfile::tempdir() {
            Ok(temp_dir) => {
                let temp_dir_path = temp_dir.into_path();
                Some(temp_dir_path)
            },
            Err(e) => {
                return fail_clierror!(
                    "Cannot create temporary directory to copy luadate library to: {e}"
                )
            },
        }
    } else {
        None
    };

    // "require " was used in the scripts, so we need to prepare luadate library and setup LUAU_PATH
    if require_used {
        // prepare luadate so users can just use 'date = require "date"' in their scripts
        let luadate_library = include_bytes!("../../resources/luau/vendor/luadate/date.lua");
        // safety: safe to unwrap as we just created the tempdir above
        let tdir_path = temp_dir.clone().unwrap();
        let luadate_path = tdir_path.join("date.lua");
        fs::write(luadate_path.clone(), luadate_library)?;

        // set LUAU_PATH to include the luadate library
        let mut luau_path = args.flag_luau_path.clone();
        luau_path.push_str(&format!(";{}", luadate_path.as_os_str().to_string_lossy()));
        env::set_var("LUAU_PATH", luau_path.clone());
        info!(r#"set LUAU_PATH to "{luau_path}""#);
    }

    // -------- setup Luau environment --------
    let luau = Lua::new();
    // see Compiler settings here: https://docs.rs/mlua/latest/mlua/struct.Compiler.html#
    let luau_compiler = if log_enabled!(log::Level::Debug) || log_enabled!(log::Level::Trace) {
        // debugging is on, set more debugging friendly compiler settings
        // so we can see more error details in the logfile
        mlua::Compiler::new()
            .set_optimization_level(0)
            .set_debug_level(2)
            .set_coverage_level(2)
    } else {
        // use more performant compiler settings
        mlua::Compiler::new()
            .set_optimization_level(2)
            .set_debug_level(1)
            .set_coverage_level(0)
    };
    // set default Luau compiler
    luau.set_compiler(luau_compiler.clone());

    let globals = luau.globals();

    // check the QSV_CKAN_API environment variable
    let ckan_api = if let Ok(api) = std::env::var("QSV_CKAN_API") {
        api
    } else {
        args.flag_ckan_api.clone()
    };

    // check the QSV_CKAN_TOKEN environment variable
    let ckan_token = if let Ok(token) = std::env::var("QSV_CKAN_TOKEN") {
        Some(token)
    } else {
        args.flag_ckan_token.clone()
    };

    setup_helpers(&luau, args.flag_delimiter, ckan_api, ckan_token)?;

    // check if qsv_registerlookup_used is set, if it is, setup the qsv_cache directory
    if qsv_register_lookup_used {
        let qsv_cache_dir = if let Ok(cache_path) = std::env::var("QSV_CACHE_DIR") {
            // if QSV_CACHE_DIR env var is set, check if it exists. If it doesn't, create it.
            if cache_path.starts_with('~') {
                // expand the tilde
                let expanded_dir = expand_tilde(&cache_path).unwrap();
                expanded_dir.to_string_lossy().to_string()
            } else {
                cache_path
            }
        } else if args.flag_cache_dir.starts_with('~') {
            // expand the tilde
            let expanded_dir = expand_tilde(&args.flag_cache_dir).unwrap();
            expanded_dir.to_string_lossy().to_string()
        } else {
            args.flag_cache_dir.clone()
        };
        if !Path::new(&qsv_cache_dir).exists() {
            fs::create_dir_all(&qsv_cache_dir)?;
        }

        info!("Using cache directory: {qsv_cache_dir}");
        globals.raw_set(QSV_CACHE_DIR, qsv_cache_dir)?;
    }

    debug!("Main processing");
    if index_file_used {
        info!("RANDOM ACCESS MODE (_INDEX or _LASTROW special variables used)");
        random_access_mode(
            &rconfig,
            &args,
            &luau,
            &luau_compiler,
            &globals,
            &begin_script,
            &main_script,
            &end_script,
            args.flag_max_errors,
        )?;
    } else {
        info!("SEQUENTIAL MODE");
        sequential_mode(
            &rconfig,
            &args,
            &luau,
            &luau_compiler,
            &globals,
            &begin_script,
            &main_script,
            &end_script,
            args.flag_max_errors,
        )?;
    }

    if let Some(temp_dir) = temp_dir {
        // delete the tempdir
        fs::remove_dir_all(temp_dir)?;
    }

    Ok(())
}

// ------------ SEQUENTIAL MODE ------------
// this mode is used when the user does not use _INDEX or _LASTROW in their script,
// so we just scan the CSV, processing the MAIN script in sequence.
fn sequential_mode(
    rconfig: &Config,
    args: &Args,
    luau: &Lua,
    luau_compiler: &mlua::Compiler,
    globals: &mlua::Table,
    begin_script: &str,
    main_script: &str,
    end_script: &str,
    max_errors: usize,
) -> Result<(), CliError> {
    globals.raw_set("cols", "{}")?;

    let mut rdr = rconfig.reader()?;
    let mut wtr = Config::new(&args.flag_output).writer()?;
    let mut headers = rdr.headers()?.clone();
    let mut remap_headers = csv::StringRecord::new();
    let mut new_column_count = 0_u8;
    let mut headers_count = headers.len();
    let debug_enabled = log_enabled!(log::Level::Debug);

    if !rconfig.no_headers {
        if !args.cmd_filter {
            let new_columns = args
                .arg_new_columns
                .as_ref()
                .ok_or("Specify new column names")?;

            let new_columns_vec: Vec<&str> = new_columns.split(',').collect();
            debug!("new_columns_vec: {new_columns_vec:?}");
            for new_column in new_columns_vec {
                new_column_count += 1;
                let new_column = new_column.trim();
                headers.push_field(new_column);
                remap_headers.push_field(new_column);
            }
        }

        if args.flag_remap {
            wtr.write_record(&remap_headers)?;
            headers_count = remap_headers.len();
        } else {
            wtr.write_record(&headers)?;
        }
    }

    // we initialize the special vars _IDX and _ROWCOUNT
    globals.raw_set(QSV_V_IDX, 0)?;
    globals.raw_set(QSV_V_ROWCOUNT, 0)?;
    if !begin_script.is_empty() {
        info!("Compiling and executing BEGIN script. _IDX: 0 _ROWCOUNT: 0");
        LUAU_STAGE.store(Stage::Begin as i8, Ordering::Relaxed);

        if let Err(e) = luau.load(begin_script).exec() {
            return fail_clierror!("BEGIN error: Failed to execute \"{begin_script}\".\n{e}");
        }
        info!("BEGIN executed.");
    }

    let mut insertrecord = csv::StringRecord::new();

    // check if qsv_insertrecord() was called in the BEGIN script
    beginend_insertrecord(luau, &mut insertrecord, headers_count, &mut wtr)?;
    if QSV_BREAK.load(Ordering::Relaxed) {
        let qsv_break_msg: String = globals.raw_get(QSV_BREAK_MSG)?;
        winfo!("{qsv_break_msg}");
        return Ok(());
    }

    // we clear the table so we don't falsely detect a call to qsv_insertrecord()
    // in the MAIN/END scripts
    luau.globals().raw_set(QSV_INSERTRECORD_TBL, Value::Nil)?;

    #[cfg(feature = "datapusher_plus")]
    let show_progress = false;

    #[cfg(any(feature = "feature_capable", feature = "lite"))]
    let show_progress =
        (args.flag_progressbar || util::get_envvar_flag("QSV_PROGRESSBAR")) && !rconfig.is_stdin();
    #[cfg(any(feature = "feature_capable", feature = "lite"))]
    let progress = ProgressBar::with_draw_target(None, ProgressDrawTarget::stderr_with_hz(5));
    #[cfg(any(feature = "feature_capable", feature = "lite"))]
    if show_progress {
        util::prep_progress(&progress, util::count_rows(rconfig)?);
    } else {
        progress.set_draw_target(ProgressDrawTarget::hidden());
    }

    // check if _IDX was used in the MAIN script
    let idx_used = main_script.contains(QSV_V_IDX);

    // only precompile main script to bytecode if debug is disabled
    let main_bytecode = if debug_enabled {
        Vec::new()
    } else {
        luau_compiler.compile(main_script)
    };

    let mut record = csv::StringRecord::new();
    let mut idx = 0_u64;
    let mut error_count = 0_usize;

    LUAU_STAGE.store(Stage::Main as i8, Ordering::Relaxed);
    info!("Executing MAIN script.");

    let mut computed_value;
    let mut must_keep_row;
    let col = luau.create_table_with_capacity(record.len(), 1)?;

    let flag_no_globals = args.flag_no_globals;
    let flag_remap = args.flag_remap;
    let no_headers = rconfig.no_headers;
    let flag_colindex = args.flag_colindex || no_headers;
    let cmd_map = args.cmd_map;
    let mut err_msg: String;
    let mut computed_result;

    // main loop
    // without an index, we stream the CSV in sequential order
    'main: while rdr.read_record(&mut record)? {
        #[cfg(any(feature = "feature_capable", feature = "lite"))]
        if show_progress {
            progress.inc(1);
        }

        idx += 1;
        if idx_used {
            // for perf reasons, only update _IDX if it was used in the MAIN script
            globals.raw_set(QSV_V_IDX, idx)?;
        }

        // Updating col
        let _ = col.clear();
        if flag_colindex {
            for (i, v) in record.iter().enumerate() {
                col.raw_set(i + 1, v)?;
            }
        }
        if !no_headers {
            for (h, v) in headers.iter().zip(record.iter()) {
                col.raw_set(h, v)?;
            }
        }
        globals.raw_set("col", col.clone())?;

        // Updating global
        if !flag_no_globals && !no_headers {
            for (h, v) in headers.iter().zip(record.iter()) {
                globals.raw_set(h, v)?;
            }
        }

        // if debug is enabled, we eval the script as string instead of precompiled bytecode
        // so we can get more detailed error messages with line numbers
        computed_result = if debug_enabled {
            luau.load(main_script).eval()
        } else {
            luau.load(&main_bytecode).eval()
        };

        computed_value = match computed_result {
            Ok(computed) => computed,
            Err(e) => {
                error_count += 1;
                err_msg = format!("<ERROR> _IDX: {idx} error({error_count}): {e:?}");
                log::error!("{err_msg}");

                mlua::IntoLua::into_lua(err_msg, luau)
                    .map_err(|e| format!("Failed to convert error message to Lua: {e}"))?
            },
        };

        if QSV_BREAK.load(Ordering::Relaxed) {
            let qsv_break_msg: String = globals.raw_get(QSV_BREAK_MSG)?;
            winfo!("{qsv_break_msg}");
            break 'main;
        }

        if max_errors > 0 && error_count > max_errors {
            info!("Maximum number of errors ({max_errors}) reached. Aborting MAIN script.");
            break 'main;
        }

        if cmd_map {
            map_computedvalue(
                computed_value.as_ref(),
                &mut record,
                flag_remap,
                new_column_count,
            )?;

            // check if the script is trying to insert a record with
            // qsv_insertrecord(). We do this by checking if the global
            // _QSV_IR_TBL exists and is not empty
            match luau.globals().raw_get(QSV_INSERTRECORD_TBL) {
                Ok(Value::Table(insertrecord_table)) => {
                    // _QSV_IR_TBL is populated, we have a record to insert
                    insertrecord.clear();

                    create_insertrecord(&insertrecord_table, &mut insertrecord, headers_count)?;

                    if QSV_SKIP.load(Ordering::Relaxed) {
                        if log_enabled!(log::Level::Debug) {
                            debug!("Skipping record {idx} because _QSV_SKIP is set to true");
                        }
                        QSV_SKIP.store(false, Ordering::Relaxed);
                    } else {
                        wtr.write_record(&record)?;
                    }
                    wtr.write_record(&insertrecord)?;
                    insertrecord_table.clear()?;
                },
                Ok(_) | Err(_) => {
                    if QSV_SKIP.load(Ordering::Relaxed) {
                        QSV_SKIP.store(false, Ordering::Relaxed);
                    } else {
                        wtr.write_record(&record)?;
                    }
                },
            }
        } else {
            // filter subcommand
            must_keep_row = if error_count > 0 {
                true
            } else {
                match computed_value {
                    Value::Boolean(boolean) => boolean,
                    Value::Nil => false,
                    Value::String(strval) => !strval.to_string_lossy().is_empty(),
                    Value::Integer(intval) => intval != 0,
                    // we compare to f64::EPSILON as float comparison to zero
                    // unlike int, where we can say intval != 0, we cannot do fltval !=0
                    // https://doc.rust-lang.org/std/primitive.f64.html#associatedconstant.EPSILON
                    Value::Number(fltval) => (fltval).abs() > f64::EPSILON,
                    _ => true,
                }
            };

            if must_keep_row {
                wtr.write_record(&record)?;
            }
        }
    }

    if !end_script.is_empty() {
        // at the END, set a convenience variable named _ROWCOUNT;
        // true, _ROWCOUNT is equal to _IDX at this point, but this
        // should make for more readable END scripts.
        // Also, _ROWCOUNT is zero during the main script, and only set
        // to _IDX during the END script.
        LUAU_STAGE.store(Stage::End as i8, Ordering::Relaxed);
        globals.raw_set(QSV_V_ROWCOUNT, idx)?;
        if !idx_used {
            // for perf reasons, we only updated _IDX in the main
            // hot loop if it was used in the main script
            // so we set it here in the END script, if it was not used in the main script
            globals.raw_set(QSV_V_IDX, idx)?;
        }

        info!("Compiling and executing END script. _ROWCOUNT: {idx}");
        let end_value: Value = match luau.load(end_script).eval() {
            Ok(computed) => computed,
            Err(e) => {
                let err_msg = format!("<ERROR> END error: Cannot evaluate \"{end_script}\".\n{e}");
                log::error!("{err_msg}");
                log::error!("END globals: {globals:?}");

                mlua::IntoLua::into_lua(err_msg, luau)
                    .map_err(|e| format!("Failed to convert error message to Lua: {e}"))?
            },
        };

        // check if qsv_insertrecord() was called in the END script
        beginend_insertrecord(luau, &mut insertrecord, headers_count, &mut wtr)?;

        let end_string = match end_value {
            Value::String(string) => string.to_string_lossy().to_string(),
            Value::Number(number) => number.to_string(),
            Value::Integer(number) => number.to_string(),
            Value::Boolean(boolean) => (if boolean { "true" } else { "false" }).to_string(),
            Value::Nil => String::new(),
            _ => {
                return fail_clierror!(
                    "Unexpected END value type returned by provided Luau expression. {end_value:?}"
                );
            },
        };
        if !end_string.is_empty() && !show_progress {
            winfo!("{end_string}");
        }
    }

    wtr.flush()?;
    #[cfg(any(feature = "feature_capable", feature = "lite"))]
    if show_progress {
        util::finish_progress(&progress);
    }
    info!("SEQUENTIAL MODE: Processed {idx} record/s.");

    if error_count > 0 {
        return fail_clierror!("Luau errors encountered: {error_count}");
    };
    Ok(())
}

// ------------ RANDOM ACCESS MODE ------------
// this function is largely similar to sequential_mode, and is triggered when
// we use the special variable _INDEX or _LASTROW in the Luau scripts.
// the primary difference being that we use an Indexed File rdr in the main loop.
// differences pointed out in comments below
fn random_access_mode(
    rconfig: &Config,
    args: &Args,
    luau: &Lua,
    luau_compiler: &mlua::Compiler,
    globals: &mlua::Table,
    begin_script: &str,
    main_script: &str,
    end_script: &str,
    max_errors: usize,
) -> Result<(), CliError> {
    // users can create an index file by calling qsv_autoindex() in their BEGIN script
    if begin_script.contains("qsv_autoindex()") {
        let result = create_index(&args.arg_input);
        if result.is_err() {
            return fail_clierror!("Unable to create/update index file");
        }
    }

    // we abort RANDOM ACCESS mode if the index file is not found
    let Some(mut idx_file) = rconfig.indexed()? else {
        return fail!(
            r#"Index required but no index file found. Use "qsv_autoindex()" in your BEGIN script."#
        );
    };

    globals.raw_set("cols", "{}")?;

    // with an index, we can fetch the row_count in advance
    let mut row_count = util::count_rows(rconfig).unwrap_or_default();
    if args.flag_no_headers {
        row_count += 1;
    }

    let mut wtr = Config::new(&args.flag_output).writer()?;
    let mut headers = idx_file.headers()?.clone();
    let mut remap_headers = csv::StringRecord::new();
    let mut new_column_count = 0_u8;
    let mut headers_count = headers.len();
    let debug_enabled = log_enabled!(log::Level::Debug);

    if !rconfig.no_headers {
        if !args.cmd_filter {
            let new_columns = args
                .arg_new_columns
                .as_ref()
                .ok_or("Specify new column names")?;

            let new_columns_vec: Vec<&str> = new_columns.split(',').collect();
            for new_column in new_columns_vec {
                new_column_count += 1;
                let new_column = new_column.trim();
                headers.push_field(new_column);
                remap_headers.push_field(new_column);
            }
        }

        if args.flag_remap {
            wtr.write_record(&remap_headers)?;
            headers_count = remap_headers.len();
        } else {
            wtr.write_record(&headers)?;
        }
    }

    // unlike sequential_mode, we actually know the row_count at the BEGINning
    globals.raw_set(QSV_V_IDX, 0)?;
    globals.raw_set(QSV_V_INDEX, 0)?;
    globals.raw_set(QSV_V_ROWCOUNT, row_count)?;
    globals.raw_set(QSV_V_LASTROW, row_count - 1)?;

    if !begin_script.is_empty() {
        info!(
            "Compiling and executing BEGIN script. _ROWCOUNT: {row_count} _LASTROW: {}",
            row_count - 1
        );
        LUAU_STAGE.store(Stage::Begin as i8, Ordering::Relaxed);

        if let Err(e) = luau.load(begin_script).exec() {
            return fail_clierror!("BEGIN error: Failed to execute \"{begin_script}\".\n{e}");
        }
        info!("BEGIN executed.");
    }

    let mut insertrecord = csv::StringRecord::new();

    // check if qsv_insertrecord() was called in the BEGIN script
    beginend_insertrecord(luau, &mut insertrecord, headers_count, &mut wtr)?;
    if QSV_BREAK.load(Ordering::Relaxed) {
        let qsv_break_msg: String = globals.raw_get(QSV_BREAK_MSG)?;
        winfo!("{qsv_break_msg}");
        return Ok(());
    }

    // we clear the table so we don't falsely detect a call to qsv_insertrecord()
    // in the MAIN/END scripts
    luau.globals().raw_set(QSV_INSERTRECORD_TBL, Value::Nil)?;

    // in random access mode, setting "_INDEX" allows us to change the current record
    // for the NEXT read
    let mut pos = globals.get::<_, isize>(QSV_V_INDEX).unwrap_or_default();
    let mut curr_record = if pos > 0 && pos <= row_count as isize {
        pos as u64
    } else {
        0_u64
    };
    debug!("BEGIN current record: {curr_record}");
    idx_file.seek(curr_record)?;

    let main_bytecode = if debug_enabled {
        Vec::new()
    } else {
        luau_compiler.compile(main_script)
    };
    let mut record = csv::StringRecord::new();
    let mut error_count = 0_usize;
    let mut processed_count = 0_usize;

    #[cfg(feature = "datapusher_plus")]
    let show_progress = false;

    #[cfg(any(feature = "feature_capable", feature = "lite"))]
    let show_progress =
        (args.flag_progressbar || util::get_envvar_flag("QSV_PROGRESSBAR")) && !rconfig.is_stdin();
    #[cfg(any(feature = "feature_capable", feature = "lite"))]
    let progress = ProgressBar::with_draw_target(None, ProgressDrawTarget::stderr_with_hz(5));
    #[cfg(any(feature = "feature_capable", feature = "lite"))]
    if show_progress {
        progress.set_style(
            ProgressStyle::default_bar()
                .template("[{elapsed_precise}] [{bar:40.cyan/blue} {human_pos}] {msg}")
                .unwrap()
                .progress_chars(" * "),
        );

        progress.set_message(format!("{row_count} records"));

        // draw progress bar for the first time using RANDOM ACCESS style
        progress.set_length(curr_record);
    } else {
        progress.set_draw_target(ProgressDrawTarget::hidden());
    }

    LUAU_STAGE.store(Stage::Main as i8, Ordering::Relaxed);
    info!(
        "Executing MAIN script. _INDEX: {curr_record} _ROWCOUNT: {row_count} _LASTROW: {}",
        row_count - 1
    );

    let mut computed_value;
    let mut must_keep_row;
    let col = luau.create_table_with_capacity(record.len(), 1)?;

    let flag_no_globals = args.flag_no_globals;
    let flag_remap = args.flag_remap;
    let no_headers = rconfig.no_headers;
    let flag_colindex = args.flag_colindex || no_headers;
    let cmd_map = args.cmd_map;
    let mut err_msg: String;
    let mut computed_result;

    // main loop - here we use an indexed file reader to implement random access mode,
    // seeking to the next record to read by looking at _INDEX special var
    'main: while idx_file.read_record(&mut record)? {
        globals.raw_set(QSV_V_IDX, curr_record)?;

        #[cfg(any(feature = "feature_capable", feature = "lite"))]
        if show_progress {
            progress.set_position(curr_record + 1);
        }

        processed_count += 1;

        // Updating col
        let _ = col.clear();
        if flag_colindex {
            for (i, v) in record.iter().enumerate() {
                col.raw_set(i + 1, v)?;
            }
        }
        if !no_headers {
            for (h, v) in headers.iter().zip(record.iter()) {
                col.raw_set(h, v)?;
            }
        }
        globals.raw_set("col", col.clone())?;

        // Updating global
        if !flag_no_globals && !no_headers {
            for (h, v) in headers.iter().zip(record.iter()) {
                globals.raw_set(h, v)?;
            }
        }

        computed_result = if debug_enabled {
            luau.load(main_script).eval()
        } else {
            luau.load(&main_bytecode).eval()
        };

        computed_value = match computed_result {
            Ok(computed) => computed,
            Err(e) => {
                error_count += 1;
                err_msg = format!("<ERROR> _IDX: {curr_record} error({error_count}): {e:?}");
                log::error!("{err_msg}");

                mlua::IntoLua::into_lua(err_msg, luau)
                    .map_err(|e| format!("Failed to convert error message to Lua: {e}"))?
            },
        };

        if QSV_BREAK.load(Ordering::Relaxed) {
            let qsv_break_msg: String = globals.raw_get(QSV_BREAK_MSG)?;
            winfo!("{qsv_break_msg}");
            break 'main;
        }

        if max_errors > 0 && error_count > max_errors {
            info!("Maximum number of errors ({max_errors}) reached. Aborting MAIN script.");
            break 'main;
        }

        if cmd_map {
            map_computedvalue(
                computed_value.as_ref(),
                &mut record,
                flag_remap,
                new_column_count,
            )?;

            // check if the MAIN script is trying to insert a record
            match luau.globals().raw_get(QSV_INSERTRECORD_TBL) {
                Ok(Value::Table(insertrecord_table)) => {
                    // QSV_INSERTRECORD_TBL is populated, we have a record to insert
                    insertrecord.clear();

                    create_insertrecord(&insertrecord_table, &mut insertrecord, headers_count)?;

                    if QSV_SKIP.load(Ordering::Relaxed) {
                        if log_enabled!(log::Level::Debug) {
                            debug!(
                                "Skipping record {curr_record} because _QSV_SKIP is set to true"
                            );
                        }
                        QSV_SKIP.store(false, Ordering::Relaxed);
                    } else {
                        wtr.write_record(&record)?;
                    }
                    wtr.write_record(&insertrecord)?;
                    insertrecord_table.clear()?;
                },
                Ok(_) | Err(_) => {
                    if QSV_SKIP.load(Ordering::Relaxed) {
                        QSV_SKIP.store(false, Ordering::Relaxed);
                    } else {
                        wtr.write_record(&record)?;
                    }
                },
            }
        } else {
            // filter subcommand
            must_keep_row = if error_count > 0 {
                true
            } else {
                match computed_value {
                    Value::Boolean(boolean) => boolean,
                    Value::Nil => false,
                    Value::String(strval) => !strval.to_string_lossy().is_empty(),
                    Value::Integer(intval) => intval != 0,
                    Value::Number(fltval) => (fltval).abs() > f64::EPSILON,
                    _ => true,
                }
            };

            if must_keep_row {
                wtr.write_record(&record)?;
            }
        }

        pos = globals.get::<_, isize>(QSV_V_INDEX).unwrap_or_default();
        if pos < 0 || pos as u64 > row_count {
            break 'main;
        }
        let next_record = if pos > 0 && pos <= row_count as isize {
            pos as u64
        } else {
            0_u64
        };
        if idx_file.seek(next_record).is_err() {
            break 'main;
        }
        curr_record = next_record;
    } // main loop

    if !end_script.is_empty() {
        info!("Compiling and executing END script. _ROWCOUNT: {row_count}");
        LUAU_STAGE.store(Stage::End as i8, Ordering::Relaxed);

        let end_value: Value = match luau.load(end_script).eval() {
            Ok(computed) => computed,
            Err(e) => {
                let err_msg = format!("<ERROR> END error: Cannot evaluate \"{end_script}\".\n{e}");
                log::error!("{err_msg}");
                log::error!("END globals: {globals:?}");

                mlua::IntoLua::into_lua(err_msg, luau)
                    .map_err(|e| format!("Failed to convert error message to Lua: {e}"))?
            },
        };

        // check if qsv_insertrecord() was called in the END script
        beginend_insertrecord(luau, &mut insertrecord, headers_count, &mut wtr)?;

        let end_string = match end_value {
            Value::String(string) => string.to_string_lossy().to_string(),
            Value::Number(number) => number.to_string(),
            Value::Integer(number) => number.to_string(),
            Value::Boolean(boolean) => (if boolean { "true" } else { "false" }).to_string(),
            Value::Nil => String::new(),
            _ => {
                return fail_clierror!(
                    "Unexpected END value type returned by provided Luau expression. {end_value:?}"
                );
            },
        };
        if !end_string.is_empty() && !show_progress {
            winfo!("{end_string}");
        }
    }

    wtr.flush()?;
    let msg = format!("RANDOM ACCESS MODE: Processed {processed_count} record/s.");
    #[cfg(any(feature = "feature_capable", feature = "lite"))]
    if show_progress {
        progress.abandon_with_message(msg.clone());
    }
    info!("{msg}");

    if error_count > 0 {
        return fail_clierror!("Luau errors encountered: {error_count}");
    };
    Ok(())
}

// -----------------------------------------------------------------------------
// UTILITY FUNCTIONS
// -----------------------------------------------------------------------------

#[inline]
fn map_computedvalue(
    computed_value: &Value,
    record: &mut csv::StringRecord,
    flag_remap: bool,
    new_column_count: u8,
) -> Result<(), CliError> {
    match computed_value {
        Value::String(string) => {
            if let Ok(utf8) = simdutf8::basic::from_utf8(string.as_bytes()) {
                record.push_field(utf8);
            } else {
                record.push_field(&string.to_string_lossy());
            }
        },
        Value::Number(number) => {
            let mut buffer = ryu::Buffer::new();
            record.push_field(buffer.format(*number));
        },
        Value::Integer(number) => {
            let mut buffer = itoa::Buffer::new();
            record.push_field(buffer.format(*number));
        },
        Value::Boolean(boolean) => {
            record.push_field(if *boolean { "true" } else { "false" });
        },
        Value::Nil => {
            record.push_field("");
        },
        Value::Table(table) => {
            if flag_remap {
                // we're in remap mode, so we clear the record
                // and only write the new columns to output
                record.clear();
            }
            let mut columns_inserted = 0_u8;
            let mut ibuffer = itoa::Buffer::new();
            let mut nbuffer = ryu::Buffer::new();
            table.for_each::<String, Value>(|_k, v| {
                if new_column_count > 0 && columns_inserted >= new_column_count {
                    // we ignore table values more than the number of
                    // new columns defined, so we return early
                    return Ok(());
                }
                match v {
                    Value::Integer(intval) => record.push_field(ibuffer.format(intval)),
                    Value::String(strval) => record.push_field(&strval.to_string_lossy()),
                    Value::Number(number) => record.push_field(nbuffer.format(number)),
                    Value::Boolean(boolean) => {
                        record.push_field(if boolean { "true" } else { "false" });
                    },
                    Value::Nil => record.push_field(""),
                    _ => {
                        return Err(mlua::Error::RuntimeError(format!(
                            "Unexpected value type returned by provided Luau expression: {v:?}"
                        )))
                    },
                }
                columns_inserted += 1;
                Ok(())
            })?;

            // on the other hand, if there are less table values than expected
            // we fill it up with empty fields
            while new_column_count > 0 && columns_inserted < new_column_count {
                record.push_field("");
                columns_inserted += 1;
            }
        },
        _ => {
            return fail_clierror!(
                "Unexpected value type returned by provided Luau expression. {computed_value:?}"
            );
        },
    };
    Ok(())
}

#[inline]
fn create_insertrecord(
    insertrecord_table: &mlua::Table,
    insertrecord: &mut csv::StringRecord,
    headers_count: usize,
) -> Result<(), CliError> {
    let mut columns_inserted = 0_usize;

    for v in insertrecord_table.clone().sequence_values::<String>() {
        let v = v?;
        insertrecord.push_field(&v);

        columns_inserted += 1;
        if columns_inserted > headers_count {
            // we ignore table values more than the number of new columns defined
            break;
        }
    }
    // on the other hand, if there are less table values than expected
    // we fill it up with empty fields
    while columns_inserted < headers_count {
        insertrecord.push_field("");
        columns_inserted += 1;
    }

    if log_enabled!(log::Level::Debug) {
        debug!("insertrecord: {insertrecord:?}");
    }
    Ok(())
}

// this is called when processing BEGIN and END scripts to
// check if qsv_insertrecord() was called in that script
fn beginend_insertrecord(
    luau: &Lua,
    insertrecord: &mut csv::StringRecord,
    headers_count: usize,
    wtr: &mut csv::Writer<Box<dyn Write>>,
) -> Result<(), CliError> {
    match luau.globals().raw_get(QSV_INSERTRECORD_TBL) {
        Ok(Value::Table(insertrecord_table)) => {
            // QSV_INSERTRECORD_TBL is populated, we have a record to insert
            insertrecord.clear();

            create_insertrecord(&insertrecord_table, insertrecord, headers_count)?;

            wtr.write_record(&*insertrecord)?;
            insertrecord_table.clear()?;
        },
        Ok(_) | Err(_) => {},
    }
    Ok(())
}

fn create_index(arg_input: &Option<String>) -> Result<bool, CliError> {
    // this is a utility function that creates an index file for the current CSV.
    // it is called when "qsv_autoindex()" is called in the BEGIN script.
    let Some(input) = &arg_input else {
        log::warn!("qsv_autoindex() does not work for stdin.");
        return Ok(false);
    };

    if input.to_lowercase().ends_with(".sz") {
        log::warn!("qsv_autoindex() does not work with snappy files.");
        return Ok(false);
    }

    let pidx = util::idx_path(Path::new(&input));
    debug!("Creating index file {pidx:?} for {input:?}.");

    let rconfig = Config::new(&Some(input.to_string()));
    let mut rdr = rconfig.reader_file()?;
    let mut wtr =
        io::BufWriter::with_capacity(DEFAULT_WTR_BUFFER_CAPACITY, fs::File::create(pidx)?);
    if RandomAccessSimple::create(&mut rdr, &mut wtr).is_err() {
        return Ok(false);
    };
    if wtr.flush().is_err() {
        return Ok(false);
    }

    log::info!("qsv_autoindex() successful.");
    Ok(true)
}

// -----------------------------------------------------------------------------
// HELPER FUNCTIONS
// -----------------------------------------------------------------------------
// setup_helpers sets up some helper functions that can be called from Luau scripts
fn setup_helpers(
    luau: &Lua,
    delimiter: Option<Delimiter>,
    ckan_api_url: String,
    ckan_token: Option<String>,
) -> Result<(), CliError> {
    macro_rules! helper_err {
        ($helper_name:literal, $($arg:tt)*) => ({
            use log::error;
            let helper_name = format!("{}: ", $helper_name);
            let err_msg = format!($($arg)*);
            error!("{helper_name}: {err_msg}");
            Err(mlua::Error::RuntimeError(err_msg))
        });
    }

    // this is a helper function that can be called from Luau scripts
    // to send log messages to the logfile
    // the first parameter is the log level, and the following parameters are concatenated
    //
    //   qsv_log(log_level, arg1, .., argN)
    //       log_level: string, one of "info", "warn", "error", "debug", "trace".
    //                  if invalid log_level is provided, "info" is assumed.
    //      arg1, argN: Up to 255 arguments to be concatenated and logged as one string.
    //         returns: Luau table of header names excluding the first header,
    //                  or Luau runtime error if the lookup table could not be loaded
    //
    let qsv_log = luau.create_function(|luau, mut args: mlua::MultiValue| {
        let mut log_msg = {
            // at which stage are we logging?
            // safety: this is safe to unwrap because we only set LUAU_STAGE using the Stage enum
            let stage: Stage = LUAU_STAGE.load(Ordering::Relaxed).try_into().unwrap();
            let stage_str: &'static str = stage.into();
            format!("{}: ", stage_str.to_ascii_uppercase())
        };
        let mut idx = 0_u8;
        let mut log_level = String::new();
        while let Some(val) = args.pop_front() {
            let val = luau.from_value::<serde_json::Value>(val)?;
            let val_str = &serde_json::to_string_pretty(&val).unwrap_or_default();
            if idx == 0 {
                log_level = val_str.trim_matches('"').to_lowercase();
            } else {
                log_msg.push_str(val_str.trim_matches('"'));
                if idx == u8::MAX {
                    break;
                }
            }
            idx += 1;
        }
        match log_level.as_str() {
            "info" => log::info!("{log_msg}"),
            "warn" => log::warn!("{log_msg}"),
            "error" => log::error!("{log_msg}"),
            "debug" => log::debug!("{log_msg}"),
            "trace" => log::trace!("{log_msg}"),
            _ => {
                log::info!("unknown log level: {log_level} msg: {log_msg}");
            },
        }
        Ok(())
    })?;
    luau.globals().set("qsv_log", qsv_log)?;

    // this is a helper function that can be called from Luau scripts
    // to coalesce - return the first non-null value in a list
    //
    //   qsv_coalesce(arg1, .., argN)
    //      returns: first non-null value of the arguments
    //               or an empty string if all arguments are null
    //
    let qsv_coalesce = luau.create_function(|luau, mut args: mlua::MultiValue| {
        while let Some(val) = args.pop_front() {
            let val = luau.from_value::<serde_json::Value>(val)?;
            let val_str = val.as_str().unwrap_or_default();
            if !val_str.is_empty() {
                return Ok(val_str.to_string());
            }
        }
        Ok(String::new())
    })?;
    luau.globals().set("qsv_coalesce", qsv_coalesce)?;

    // this is a helper function that can be called from the BEGIN and MAIN script
    // to stop processing. All the parameters are concatenated and returned as a string.
    // The string is also stored in the global variable _QSV_BRKMSG.
    // qsv_break should only be called from scripts that are processing CSVs in sequential mode.
    // When in random access mode, set _INDEX to -1 or a value greater than _LASTROW instead
    //
    //   qsv_break(arg1, .., argN)
    //      arg1, argN: up to 254 arguments
    //         returns: concatenated args as one string or an empty string if no args are passed.
    //                  Luau runtime error if called from END script
    //
    let qsv_break = luau.create_function(|luau, mut args: mlua::MultiValue| {
        if LUAU_STAGE.load(Ordering::Relaxed) == Stage::End as i8 {
            return helper_err!(
                "qsv_break",
                "qsv_break() can only be called from the BEGIN and MAIN scripts."
            );
        }

        let mut break_msg = String::new();
        let mut idx = 0_u8;
        while let Some(val) = args.pop_front() {
            let val = luau.from_value::<serde_json::Value>(val)?;
            let val_str = &serde_json::to_string_pretty(&val).unwrap_or_default();

            break_msg.push_str(val_str.trim_matches('"'));
            if idx == u8::MAX {
                break;
            }
            idx += 1;
        }
        luau.globals().raw_set(QSV_BREAK_MSG, break_msg.clone())?;
        QSV_BREAK.store(true, Ordering::Relaxed);

        Ok(break_msg)
    })?;
    luau.globals().set("qsv_break", qsv_break)?;

    // this is a helper function that can be called from Luau scripts
    // to sleep for N milliseconds.
    //
    //   qsv_sleep(milliseconds: number)
    //      returns: None
    //
    let qsv_sleep = luau.create_function(|_, args: mlua::Number| {
        let sleep_time = args as u64;
        if sleep_time > 0 {
            log::info!("sleeping for {} milliseconds", sleep_time);
            std::thread::sleep(std::time::Duration::from_millis(sleep_time));
        }

        Ok(())
    })?;
    luau.globals().set("qsv_sleep", qsv_sleep)?;

    // this is a helper function that can be called from the MAIN script
    // to skip writing the current row's output when processing CSVs.
    //
    //   qsv_skip()
    //      returns: None
    //               or Luau runtime error if called from BEGIN or END scripts
    //
    let qsv_skip = luau.create_function(|_, ()| {
        if LUAU_STAGE.load(Ordering::Relaxed) != Stage::Main as i8 {
            return helper_err!(
                "qsv_skip",
                "qsv_skip() can only be called from the MAIN script."
            );
        }

        QSV_SKIP.store(true, Ordering::Relaxed);

        Ok(())
    })?;
    luau.globals().set("qsv_skip", qsv_skip)?;

    // this is a helper function that creates an index file for the current CSV.
    // It does not work for stdin and should only be called in the BEGIN script
    // its actually just a stub and the real function is called before processing
    // the BEGIN script.
    // Calling this will also initialize the _ROWCOUNT and _LASTROW special variables
    // so that the BEGIN script can use them
    //
    //   qsv_autoindex()
    //      returns: None as this is a stub function.
    //               A Luau runtime error will be raised if the index cannot be created
    //               as soon as the BEGIN script is actually executed.
    //               A Luau runtime error is also returned if called from MAIN or END.
    //
    let qsv_autoindex = luau.create_function(|_, ()| {
        if LUAU_STAGE.load(Ordering::Relaxed) != Stage::Begin as i8 {
            return helper_err!(
                "qsv_autoindex",
                "qsv_autoindex() can only be called from the BEGIN script."
            );
        }

        Ok(())
    })?;
    luau.globals().set("qsv_autoindex", qsv_autoindex)?;

    // this is a helper function to set an environment variable.
    // Note that the environment variable is set in the current process
    // and NOT in the shell/parent process that is executing the qsv process.
    //
    //   qsv_setenv(envar: string, value: string)
    //          envvar: the name of the environment variable to set
    //           value: the value to set the environment variable to.
    //                  Set to "" to unset the environment variable.
    //         returns: None
    //                  A Luau runtime error if the envvar is empty.
    //
    let qsv_setenv = luau.create_function(|_, (envvar, value): (String, String)| {
        if envvar.is_empty() {
            return helper_err!("qsv_setenv", "envvar cannot be empty.");
        }

        if value.is_empty() {
            std::env::remove_var(envvar);
        } else {
            std::env::set_var(envvar, value);
        }

        Ok(())
    })?;
    luau.globals().set("qsv_setenv", qsv_setenv)?;

    // this is a helper function to get the value of an environment variable.
    // Note that the environment variable is read from the parent AND current processes.
    //
    //   qsv_getenv(envar: string)
    //          envvar: the name of the environment variable to get
    //         returns: The value of the environment variable or an empty string if the
    //                  environment variable is not set.
    //                  A Luau runtime error if the envvar argument is empty.
    //
    let qsv_getenv = luau.create_function(|_, envvar: String| {
        if envvar.is_empty() {
            return helper_err!("qsv_getenv", "envvar cannot be empty.");
        }

        match std::env::var(envvar) {
            Ok(val) => Ok(val),
            Err(_) => Ok(String::new()),
        }
    })?;
    luau.globals().set("qsv_getenv", qsv_getenv)?;

    // this is a helper function to check if a file exists.
    //
    //   qsv_fileexists(filepath: string)
    //        filepath: the path to the file to check
    //         returns: true if the file exists, false otherwise.
    //                  A Luau runtime error if the filepath argument is empty.
    //
    let qsv_fileexists = luau.create_function(|_, filepath: String| {
        if filepath.is_empty() {
            return helper_err!("qsv_fileexists", "filepath cannot be empty.");
        }

        let path = Path::new(&filepath);
        Ok(path.exists())
    })?;
    luau.globals().set("qsv_fileexists", qsv_fileexists)?;

    // this is a helper function to load a CSV into a Luau table.
    //
    //   qsv_loadcsv(table_name: string, filepath: string, key_column: string)
    //      table_name: the name of the Luau table to load the CSV data into.
    //        filepath: the path of the CSV file to load
    //      key_column: the name of the column to use as the key for the table.
    //                  If the column name is empty, the row number will be used as the key.
    //         returns: Luau table of column/header names.
    //                  A Luau runtime error if the filepath is invalid.
    //
    let qsv_loadcsv = luau.create_function(
        move |luau, (table_name, filepath, key_column): (String, String, String)| {
            if filepath.is_empty() {
                return helper_err!("qsv_loadcsv", "filepath cannot be empty.");
            }

            let path = Path::new(&filepath);
            if !path.exists() {
                return helper_err!("qsv_loadcsv", "\"{}\" does not exist.", path.display());
            }

            let csv_table = luau.create_table()?;
            #[allow(unused_assignments)]
            let mut record = csv::StringRecord::new();

            let conf = Config::new(&Some(filepath.clone()))
                .delimiter(delimiter)
                .comment(Some(b'#'))
                .no_headers(false);

            let mut rdr = conf.reader()?;

            let headers = match rdr.headers() {
                Ok(headers) => headers.clone(),
                Err(e) => {
                    return helper_err!("qsv_loadcsv", "Cannot read headers of CSV: {e}");
                },
            };

            let key_idx = if key_column.is_empty() {
                // if the key column is empty, set key_idx to a sentinel value
                // that will never match a valid column index, indicating that
                // we should use the row number as the key.
                usize::MAX
            } else {
                match headers.iter().position(|x| x == key_column) {
                    Some(idx) => idx,
                    None => {
                        return helper_err!(
                            "qsv_loadcsv",
                            "Cannot find key column \"{key_column}\" in CSV."
                        );
                    },
                }
            };

            for (row_idx, result) in rdr.records().enumerate() {
                record = result.unwrap_or_default();

                let key = if key_idx == usize::MAX {
                    let mut buffer = itoa::Buffer::new();
                    buffer.format(row_idx).to_owned()
                } else {
                    record.get(key_idx).unwrap_or_default().trim().to_string()
                };
                let inside_table = luau.create_table()?;
                for (i, header) in headers.iter().enumerate() {
                    let val = record.get(i).unwrap_or_default().trim();
                    inside_table.raw_set(header, val)?;
                }
                csv_table.raw_set(key, inside_table)?;
            }

            luau.globals().raw_set(table_name, csv_table)?;

            // now that we've successfully loaded the CSV, we return the headers
            // as a table so the user can use them to access the values
            let headers_table = luau.create_table()?;
            for (i, header) in headers.iter().enumerate() {
                headers_table.raw_set(i, header)?;
            }

            info!("{filepath} successfully loaded CSV.");

            Ok(headers_table)
        },
    )?;
    luau.globals().set("qsv_loadcsv", qsv_loadcsv)?;

    // this is a helper function that can be called from the BEGIN, MAIN & END scripts to write to
    // a file. The file will be created if it does not exist. The file will be appended to if it
    // already exists. The filename will be sanitized and will be written to the current working
    // directory. The file will be closed after the write is complete.
    //
    //   qsv_writefile(filename: string, data: string)
    //        filename: the name of the file to write to
    //            data: the string to write to the file. Note that a newline will
    //                  NOT be added automatically.
    //                  If data is "_NEWFILE!", a new empty file will be created and
    //                  if the file already exists, it will be overwritten.
    //         returns: Sanitized filename as a string.
    //                  A Luau runtime error if the file cannot be opened or written.
    //
    let qsv_writefile = luau.create_function(move |_, (filename, data): (String, String)| {
        use std::fs::OpenOptions;

        use sanitize_filename::sanitize;

        const NEWFILE_FLAG: &str = "_NEWFILE!";

        let sanitized_filename = sanitize(filename);

        let newfile_flag = data == NEWFILE_FLAG;

        let mut file = if newfile_flag {
            // create a new file. If the file already exists, overwrite it.
            std::fs::File::create(sanitized_filename.clone()).map_err(|e| {
                mlua::Error::RuntimeError(format!(
                    "qsv_writefile() - Error creating a new file: {e}"
                ))
            })?
        } else {
            // append to an existing file. If the file does not exist, create it.
            OpenOptions::new()
                .create(true)
                .append(true)
                .open(sanitized_filename.clone())
                .map_err(|e| {
                    mlua::Error::RuntimeError(format!(
                        "qsv_writefile() - Error opening existing file: {e}"
                    ))
                })?
        };
        if newfile_flag {
            log::info!("qsv_writefile() - created file: {sanitized_filename}");
        } else {
            let data_as_bytes = data.as_bytes();
            file.write_all(data_as_bytes).map_err(|e| {
                mlua::Error::RuntimeError(format!(
                    "qsv_writefile() - Error appending to existing file: {e}"
                ))
            })?;
            log::info!(
                "qsv_writefile() - appending {} bytes to file: {sanitized_filename}",
                data_as_bytes.len()
            );
        }

        file.flush()?;

        Ok(sanitized_filename)
    })?;
    luau.globals().set("qsv_writefile", qsv_writefile)?;

    // this is a helper function that can be called from the BEGIN, MAIN & END scripts to insert a
    // record into the output CSV. It will automatically ignore excess columns, and fill up columns
    // with empty strings if there are less columns specified than expected.
    // Note that you can only insert ONE record in the BEGIN and END scripts
    //
    //   qsv_insertrecord(col1, .., colN)
    //      col1..N: the values to insert. If there are more columns than expected, the extra
    //               columns will be ignored. If there are less columns than expected, the
    //               missing columns will be filled with empty strings.
    //               Up to 65,535 columns supported.
    //      returns: None. Will always succeed.
    //
    let qsv_insertrecord = luau.create_function(|luau, mut args: mlua::MultiValue| {
        let insertrecord_table = luau.create_table_with_capacity(args.len(), 1)?;
        // Luau tables are 1-based
        let mut idx = 1_u16;

        while let Some(val) = args.pop_front() {
            let val = luau.from_value::<serde_json::Value>(val)?;
            let val_str = val.as_str().unwrap_or_default();

            insertrecord_table.raw_set(idx, val_str).unwrap();
            idx += 1;

            if idx == u16::MAX {
                break;
            }
        }
        luau.globals()
            .raw_set(QSV_INSERTRECORD_TBL, insertrecord_table.clone())?;

        if log::log_enabled!(log::Level::Debug) {
            log::debug!("qsv_insertrecord() - inserting record: {insertrecord_table:?}");
        } else {
            log::info!("qsv_insertrecord() - inserting record");
        }

        Ok(())
    })?;
    luau.globals().set("qsv_insertrecord", qsv_insertrecord)?;

    // this is a helper function to imvoke other qsv commands from Luau scripts.
    //
    //   qsv_cmd(qsv_args: String)
    //      qsv_args: the arguments to pass to qsv. Note that the qsv binary will be
    //                automatically prepended. stdin is not supported.
    //      returns: a table with stdout and stderr output of the qsv command.
    //               A Luau runtime error if the command cannot be executed.
    //
    let qsv_cmd = luau.create_function(|luau, args: mlua::String| {
        let qsv_binary = env::current_exe().unwrap();

        let mut cmd = std::process::Command::new(qsv_binary);
        let qsv_args = args.to_str().unwrap_or_default().to_string();
        let args_vec: Vec<&str> = qsv_args.split_whitespace().collect();
        log::info!("Invoking qsv_cmd: {qsv_args}");
        let result = cmd.args(args_vec).output();

        match result {
            Ok(output) => {
                let child_stdout = if let Ok(s) = simdutf8::basic::from_utf8(&output.stdout) {
                    s.to_string()
                } else {
                    let lossy_string = String::from_utf8_lossy(output.stdout.as_slice());
                    lossy_string.to_string()
                };

                let child_stderr = if let Ok(s) = simdutf8::basic::from_utf8(&output.stderr) {
                    s.to_string()
                } else {
                    let lossy_string = String::from_utf8_lossy(output.stderr.as_slice());
                    lossy_string.to_string()
                };
                if log_enabled!(log::Level::Debug) {
                    log::debug!("qsv command stdout: {child_stdout} stderr: {child_stderr}");
                } else {
                    log::info!("qsv command executed: {qsv_args}");
                }

                let output_table = luau.create_table()?;
                output_table.set("stdout", child_stdout)?;
                output_table.set("stderr", child_stderr)?;

                Ok(output_table)
            },
            Err(e) => {
                helper_err!("qsv_cmd", "failed to execute qsv command: {qsv_args}: {e}")
            },
        }
    })?;
    luau.globals().set("qsv_cmd", qsv_cmd)?;

    // this is a helper function to imvoke shell commands from Luau scripts.
    //
    //   qsv_shellcmd(shellcmd: String, args: String)
    //      shellcmd: the shell command to execute. For safety, only the following
    //                commands are allowed: awk, cat, cp, cut, df, echo, rg, grep, head, ls,
    //                mkdir, mv, nl, pwd, sed, sort, tail, touch, tr, uname, uniq, wc, whoami
    //         args: the arguments to pass to the command. stdin is not supported.
    //      returns: a table with stdout and stderr output of the shell command.
    //               A Luau runtime error if the command cannot be executed.
    //
    let qsv_shellcmd = luau.create_function(|luau, (shellcmd, args): (String, String)| {
        use std::str::FromStr;

        use strum_macros::EnumString;

        #[derive(EnumString)]
        #[strum(ascii_case_insensitive)]
        #[allow(non_camel_case_types)]
        enum ShellCmd {
            Awk,
            Cat,
            Cp,
            Cut,
            Df,
            Echo,
            Grep,
            Head,
            Ls,
            Mkdir,
            Mv,
            Nl,
            Pwd,
            Rg,
            Sed,
            Sort,
            Tail,
            Touch,
            Tr,
            Uname,
            Uniq,
            Wc,
            Whoami,
        }

        let shellcmd_string = shellcmd.to_ascii_lowercase();
        let Ok(_) = ShellCmd::from_str(&shellcmd_string) else {
            return helper_err!(
                "qsv_shellcmd",
                "Invalid shell command: \"{shellcmd}\". Only the following commands are allowed: \
                 awk, cat, cp, cut, df, echo, rg, grep, head, ls, mkdir, mv, nl, pwd, sed, sort, \
                 tail, touch, tr, uname, uniq, wc, whoami"
            );
        };

        let args_string = args.as_str().to_string();
        let args_vec: Vec<&str> = args_string.split_whitespace().collect();
        log::info!("Invoking qsv_shellcmd: {shellcmd_string} {args_string}");

        let result = if cfg!(target_os = "windows") {
            std::process::Command::new("cmd")
                .args(["/C", &shellcmd_string])
                .args(args_vec)
                .output()
        } else {
            std::process::Command::new(shellcmd_string.clone())
                .args(args_vec)
                .output()
        };

        match result {
            Ok(output) => {
                let child_stdout = if let Ok(s) = simdutf8::basic::from_utf8(&output.stdout) {
                    s.to_string()
                } else {
                    let lossy_string = String::from_utf8_lossy(output.stdout.as_slice());
                    lossy_string.to_string()
                };

                let child_stderr = if let Ok(s) = simdutf8::basic::from_utf8(&output.stderr) {
                    s.to_string()
                } else {
                    let lossy_string = String::from_utf8_lossy(output.stderr.as_slice());
                    lossy_string.to_string()
                };
                if log_enabled!(log::Level::Debug) {
                    log::debug!("shellcmd stdout: {child_stdout} stderr: {child_stderr}");
                } else {
                    log::info!("shellcmd executed.");
                }

                let output_table = luau.create_table()?;
                output_table.set("stdout", child_stdout)?;
                output_table.set("stderr", child_stderr)?;

                Ok(output_table)
            },
            Err(e) => {
                helper_err!(
                    "qsv_shellcmd",
                    "failed to execute shell command: {shellcmd_string} {args_string}: {e}"
                )
            },
        }
    })?;
    luau.globals().set("qsv_shellcmd", qsv_shellcmd)?;

    // this is a helper function that can be called from the BEGIN script to register
    // and load a lookup table. It expects two arguments - the lookup_name & the
    // lookup_table_uri - the URI of the CSV to use as a lookup table.
    // It returns a table with the header names if successful and create a Luau table
    // named using lookup_name, storing all the lookup values.
    // The first column is the key and the rest of the columns are values stored in a
    // table indexed by column name.
    //
    //   qsv_register_lookup(lookup_name, lookup_table_uri)
    //            lookup_name: The name of the Luau table to load the CSV into
    //       lookup_table_uri: The name of the CSV file to load. Note that it will use
    //                         the luau --delimiter option if specified.
    //                         This can be a file on the filesystem or on at a URL
    //                         ("http", "https", "dathere" and "ckan" schemes supported).
    //
    //                         The dathere scheme is used to access lookup-ready CSVs
    //                         on https://github.com/dathere/qsv-lookup-tables.
    //
    //                         The ckan scheme is used to access lookup-ready CSVs
    //                         on the given --ckan-api URL. The string following the ckan
    //                         scheme is the resource ID or alias of the CSV to load.
    //                         If you don't have a resource ID or alias, you can use the
    //                         resource name to look for followed by a question mark.
    //                         If a match is found, the first resource with a matching name
    //                         will be used.
    //         cache_age_secs: The number of seconds to cache a downloaded CSV file.
    //                         If the CSV file is older than this, it will be re-downloaded unless
    //                         the server returns a 304 Not Modified response.
    //                         If 0, the cached CSV will never expire and will be used every time.
    //                         If negative, the cached CSV will be deleted if it exists and the
    //                         CSV will be re-downloaded.
    //
    //                returns: Luau table of header names excluding the first header.
    //                         Luau runtime error if the CSV could not be loaded, or
    //                         if called from the MAIN or END scripts, or
    //                         if the lookup table is empty.
    //
    let qsv_register_lookup = luau.create_function(move |luau, (lookup_name, mut lookup_table_uri, cache_age_secs): (String, String, i64)| {
        const MSG_PREFIX: &str = "qsv_register_lookup() - ";

        if LUAU_STAGE.load(Ordering::Relaxed) != Stage::Begin as i8 {
            return helper_err!("qsv_register_lookup", "can only be called from the BEGIN script.");
        }

        let call_parameters = format!("qsv_lookup_register({lookup_name}, {lookup_table_uri}, {cache_age_secs})");

        let mut cached_csv_exists = false;
        let mut cached_csv_age_secs = 0_i64;
        let mut cached_csv_size = 0;
        let mut cache_csv_last_modified: Option<std::time::SystemTime> = None;
        let qsv_cache_dir: String = luau.globals().raw_get(QSV_CACHE_DIR)?;
        let cached_csv_path = Path::new(&qsv_cache_dir).join(format!("{lookup_name}.csv"));

        // check if lookup_table_uri is a file in the local filesystem
        let lookup_table_path = Path::new(&lookup_table_uri);
        let lookup_table_is_file = lookup_table_path.exists();
        if lookup_table_is_file {
            debug!("{MSG_PREFIX}{lookup_table_uri} is a file in the local filesystem");
        } else if cached_csv_path.exists() {

            if cache_age_secs < 0 {
                // delete the cached CSV file
                debug!("{MSG_PREFIX}deleting cached CSV file {}", cached_csv_path.display());
                std::fs::remove_file(&cached_csv_path)?;
            } else {
                // get metadata for the cached CSV file
                cached_csv_exists = true;
                let metadata = cached_csv_path.metadata()?;
                cache_csv_last_modified = Some(metadata.modified()?);
                let modified_secs = cache_csv_last_modified.unwrap().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
                let now_secs = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
                cached_csv_size = metadata.len();

                // if cache_age_secs is 0, the cached file never expires
                cached_csv_age_secs = if cache_age_secs > 0 {
                        (now_secs - modified_secs).try_into().unwrap_or(0_i64)
                } else {
                    0_i64
                };
            }
        }

        // if the lookup is not a file in the local filesystem, check if we can use the cached CSV
        // i.e. if the cached CSV exists and is not older than cache_age_secs or set not to expire, and not empty
        // otherwise, we will re-download it
        if !lookup_table_is_file && cached_csv_exists && cached_csv_age_secs <= cache_age_secs && cached_csv_size > 0 {
            lookup_table_uri = cached_csv_path.display().to_string();
            log::info!("{MSG_PREFIX}Using cached lookup table {lookup_table_uri}");
        } else {
            // if the lookup_table_uri starts with "dathere://", prepend the repo URL to the lookup table
            if let Some(lookup_url) = lookup_table_uri.strip_prefix("dathere://") {
                lookup_table_uri = format!("https://raw.githubusercontent.com/dathere/qsv-lookup-tables/main/lookup-tables/{lookup_url}");
            }

            let mut lookup_ckan = false;
            let mut resource_search = false;
            if let Some(mut lookup_url) = lookup_table_uri.strip_prefix("ckan://") {
                lookup_ckan = true;
                // it's a CKAN resource. If it ends with a '?', we'll do a resource_search
                lookup_url = lookup_url.trim();
                if lookup_url.ends_with('?') {
                    lookup_table_uri = format!("{ckan_api_url}/resource_search?query=name:{lookup_url}");
                    lookup_table_uri.pop(); // remove the trailing '?'
                    resource_search = true;
                } else {
                    // otherwise, we do a resource_show
                    lookup_table_uri = format!("{ckan_api_url}/resource_show?id={lookup_url}");
                }
            }

            let lookup_on_url = lookup_table_uri.to_lowercase().starts_with("http");

            let cache_file_path = Path::new(&qsv_cache_dir).join(format!("{lookup_name}.csv"));

            if lookup_on_url {
                use reqwest::{blocking::Client, Url};

                let client_timeout = std::time::Duration::from_secs(TIMEOUT_SECS.load(Ordering::Relaxed) as u64);

                let client = match Client::builder()
                    // safety: we're using a validated QSV_USER_AGENT or if it's not set,
                    // the default user agent                    
                    .user_agent(util::set_user_agent(None).unwrap())
                    .brotli(true)
                    .gzip(true)
                    .deflate(true)
                    .use_rustls_tls()
                    .http2_adaptive_window(true)
                    .connection_verbose(log_enabled!(log::Level::Trace))
                    .timeout(client_timeout)
                    .build()
                {
                    Ok(c) => c,
                    Err(e) => {
                        return helper_err!("qsv_register_lookup", "Cannot build reqwest client to download lookup CSV: {e}.");
                    }
                };

                let now = std::time::SystemTime::now();
                let now_dt_utc: chrono::DateTime<chrono::Utc> = now.into();
                let download_start = Instant::now();
                let mut last_modified_rfc8222 = now_dt_utc.to_rfc2822();
                let mut write_csv_contents = true;
                let lookup_csv_response = if lookup_ckan {
                    // we're using the ckan scheme, so we need to get the resource

                    let mut headers = reqwest::header::HeaderMap::new();

                    if let Some(ckan_token) = &ckan_token {
                        // there's a ckan token, so use it
                        headers.insert(
                            reqwest::header::AUTHORIZATION,
                            reqwest::header::HeaderValue::from_str(ckan_token).unwrap(),
                        );
                    }

                    debug!("{MSG_PREFIX}Downloading lookup CSV from {}...", lookup_table_uri.clone());

                    // first, check if this is a resource query (i.e. ends with a question mark)
                    if resource_search {
                        // it is a resource query, so let's do a resource_search
                        // and get the first resource with a matching name

                        let validated_url = match Url::parse(&lookup_table_uri) {
                            Ok(url) => url,
                            Err(e) => {
                                return helper_err!("qsv_register_lookup", "Invalid resource_search url {e}.");
                            }
                        };

                        let resource_search_result = match client.get(validated_url).headers(headers.clone()).send() {
                            Ok(response) => response.text().unwrap_or_default(),
                            Err(e) => {
                                return helper_err!("qsv_register_lookup", "Cannot find resource name with resource_search: {e}.");
                            }
                        };

                        let resource_search_json: serde_json::Value = match serde_json::from_str(&resource_search_result) {
                            Ok(json) => json,
                            Err(e) => {
                                return helper_err!("qsv_register_lookup", "Invalid resource_search json {e}.");
                            }
                        };

                        let Some(resource_id) = resource_search_json["result"]["results"][0]["id"].as_str() else {
                            return helper_err!("qsv_register_lookup", "Cannot find a resource name.");
                        };

                        lookup_table_uri = format!("{ckan_api_url}/resource_show?id={resource_id}");
                    }

                    // get resource_show json and get the resource URL
                    let resource_show_result = match client.get(lookup_table_uri).headers(headers.clone()).send() {
                        Ok(response) => response.text().unwrap_or_default(),
                        Err(e) => {
                            return helper_err!("qsv_register_lookup", "CKAN scheme used. Cannot get lookup CSV resource: {e}.");
                        }
                    };

                    let resource_show_json: serde_json::Value = match serde_json::from_str(&resource_show_result) {
                        Ok(json) => json,
                        Err(e) => {
                            return helper_err!("qsv_register_lookup", "Invalid resource_show json: {e}.");
                        }
                    };

                    let Some(url) = resource_show_json["result"]["url"].as_str() else {
                        return helper_err!("qsv_register_lookup", "Cannot get resource URL from resource_show JSON response.: {resource_show_json}");
                    };

                    match client.get(url).headers(headers).send() {
                        Ok(response) => response,
                        Err(e) => {
                            return helper_err!("qsv_register_lookup", r#"Cannot read lookup CSV at "{url}": {e}."#);
                        }
                    }
                } else {
                    // we're not using the ckan scheme, so just get the CSV

                    let validated_url = match Url::parse(&lookup_table_uri) {
                        Ok(url) => url,
                        Err(e) => {
                            return helper_err!("qsv_register_lookup", "Invalid lookup CSV url {e}.");
                        }
                    };

                    let mut headers = reqwest::header::HeaderMap::new();

                    if let Some(modified) = cache_csv_last_modified {
                        // a cached CSV exists, we need to use the If-Modified-Since header
                        // to avoid downloading the CSV again if it hasn't changed
                        let last_modified: chrono::DateTime<chrono::Utc> = modified.into();
                        last_modified_rfc8222 = last_modified.to_rfc2822();

                        (headers).insert(
                            reqwest::header::IF_MODIFIED_SINCE,
                            reqwest::header::HeaderValue::from_str(&last_modified_rfc8222).unwrap(),
                        );
                    }

                    match client.get(validated_url.clone()).headers(headers).send() {
                        Ok(response) => response,
                        Err(e) => {
                            return helper_err!("qsv_register_lookup", r#"Cannot read lookup CSV at "{validated_url}": {e}."#);
                        }
                    }
                };

                let lookup_csv_contents = {
                    let response_status = lookup_csv_response.status();
                    if response_status == reqwest::StatusCode::NOT_MODIFIED {
                        // the CSV hasn't changed, so we can just use the cached CSV
                        debug!("Lookup CSV hasn't changed, so using cached CSV.");
                        write_csv_contents = false;
                    } else if response_status.is_success() {
                        // the CSV has changed, so we need to download it
                        write_csv_contents = true;
                    } else {
                        match lookup_csv_response.error_for_status_ref() {
                            Ok(_) => (),
                            Err(e) => {
                                return helper_err!("qsv_register_lookup", "Cannot read lookup CSV at url: {e}.");
                            }
                        }
                    }
                    lookup_csv_response.text().unwrap_or_default()
                };

                if write_csv_contents && !lookup_csv_contents.is_empty() {
                    // write the CSV contents to the cache file
                    info!("Writing lookup CSV to cache file: {}", cache_file_path.display());
                    let mut cache_file = match std::fs::File::create(&cache_file_path) {
                        Ok(f) => f,
                        Err(e) => {
                            return helper_err!("qsv_register_lookup", 
                                "Cannot create cache file {}: {e}.", cache_file_path.display());
                        }
                    };

                    // add a comment to the top of the file with the parameters used to download the CSV,
                    // the last-modified date of the CSV, and how long it took to download it in ms
                    writeln!(cache_file, "# {call_parameters}")?;
                    writeln!(cache_file, "# Last-Modified: {last_modified_rfc8222}")?;
                    let download_elapsed = download_start.elapsed().as_millis();
                    writeln!(cache_file, "# Download-duration-ms: {download_elapsed}")?;
                    cache_file.write_all(lookup_csv_contents.as_bytes())?;

                    // explicitly flush and close the file
                    cache_file.flush()?;
                    drop(cache_file);
                }

                lookup_table_uri = cache_file_path.to_string_lossy().to_string();
            }
        }

        let lookup_table = luau.create_table()?;
        let mut record: csv::StringRecord;

        let conf = Config::new(&Some(lookup_table_uri.clone()))
            .delimiter(delimiter)
            .comment(Some(b'#'))
            .no_headers(false);

        let mut rdr = conf.reader()?;

        let headers = match rdr.headers() {
            Ok(headers) => headers.clone(),
            Err(e) => {
                return helper_err!("qsv_register_lookup", "Cannot read headers of lookup table: {e}");
            }
        };
        let mut key;
        for result in rdr.records() {
            record = result.unwrap_or_default();
            key = record.get(0).unwrap_or_default().trim();
            let inside_table = luau.create_table()?;
            // we skip the first column, as its the lookup key
            for (i, header) in headers.iter().skip(1).enumerate() {
                inside_table.raw_set(header, record.get(i + 1).unwrap_or_default().trim())?;
            }
            lookup_table.raw_set(key, inside_table)?;
        }

        luau.globals()
            .raw_set(lookup_name, lookup_table)?;

        // now that we've successfully loaded the lookup table, we return the headers
        // as a table so the user can use them to access the values
        let headers_table = luau.create_table()?;

        // we skip the first column, which is the key
        for (i, header) in headers.iter().skip(1).enumerate() {
            headers_table.raw_set(i + 1, header)?;
        }

        if headers_table.raw_len() == 0 {
            return helper_err!("qsv_register_lookup", "Lookup table is empty.");
        }

        info!("{call_parameters} successfully registered.");

        Ok(headers_table)
    })?;
    luau.globals()
        .set("qsv_register_lookup", qsv_register_lookup)?;

    Ok(())
}
