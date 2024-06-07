static USAGE: &str = r#"
Execute a shell command once per line in given CSV file.

NOTE: Windows users are recommended to use Git Bash as their terminal when
running this command.

WARNING: This command can be dangerous. Be careful when using it with
untrusted input.

Or per @thadguidry:
Please ensure when using foreach to use trusted arguments, variables, scripts, etc.
If you don't do due diligence and blindly use untrusted parts... foreach can indeed
become a footgun and possibly fry your computer, eat your lunch, and expose an entire
datacenter to a cancerous virus in your unvetted batch file you grabbed from some
stranger on the internet that runs...FOR EACH LINE in your CSV file. GASP!"

Examples:

Delete all files whose filenames are listed in the filename column:

  $ qsv foreach filename 'rm {}' assets.csv

Execute a command that outputs CSV once per record without repeating headers:

  $ qsv foreach query --unify 'search --year 2020 {}' queries.csv > results.csv

Same as above but with an additional column containing the current value:

  $ qsv foreach query -u -c from_query 'search {}' queries.csv > results.csv

For more examples, see https://github.com/jqnatividad/qsv/blob/master/tests/test_foreach.rs.

Usage:
    qsv foreach [options] <column> <command> [<input>]
    qsv foreach --help

foreach arguments:
    column      The column to use as input for the command.
    command     The command to execute. Use "{}" to substitute the value
                of the current input file line.
                If you need to execute multiple commands, use a shell
                script. See foreach_multiple_commands_with_shell_script()
                in tests/test_foreach.rs for an example.
    input       The CSV file to read. If not provided, will read from stdin.

foreach options:
    -u, --unify                If the output of the executed command is a CSV,
                               unify the result by skipping headers on each
                               subsequent command. Does not work when --dry-run is true.
    -c, --new-column <name>    If unifying, add a new column with given name
                               and copying the value of the current input file line.
    --dry-run <file|boolean>   If set to true (the default for safety reasons), the commands are
                               sent to stdout instead of executing them.
                               If set to a file, the commands will be written to the specified
                               text file instead of executing them. 
                               Only if set to false will the commands be actually executed.
                               [default: true]

Common options:
    -h, --help             Display this message
    -n, --no-headers       When set, the file will be considered to have no
                           headers.
    -d, --delimiter <arg>  The field delimiter for reading CSV data.
                           Must be a single character. (default: ,)
    -p, --progressbar      Show progress bars. Not valid for stdin.
"#;

#[cfg(target_family = "unix")]
#[allow(unused_imports)]
use std::os::unix::ffi::OsStrExt;
use std::{
    ffi::{OsStr, OsString},
    io::{self, BufReader, BufWriter, Read, Write},
    process::{Command, Stdio},
    str::FromStr,
};

use indicatif::{ProgressBar, ProgressDrawTarget};
#[cfg(target_family = "windows")]
use local_encoding::windows::multi_byte_to_wide_char;
use regex::bytes::{NoExpand, Regex};
use serde::Deserialize;

use crate::{
    config::{Config, Delimiter},
    select::SelectColumns,
    util, CliResult,
};

#[derive(Deserialize)]
struct Args {
    arg_column:       SelectColumns,
    arg_command:      String,
    arg_input:        Option<String>,
    flag_unify:       bool,
    flag_new_column:  Option<String>,
    flag_dry_run:     String,
    flag_no_headers:  bool,
    flag_delimiter:   Option<Delimiter>,
    flag_progressbar: bool,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;

    let rconfig = Config::new(&args.arg_input)
        .delimiter(args.flag_delimiter)
        .no_headers(args.flag_no_headers)
        .select(args.arg_column);

    let mut rdr = rconfig.reader()?;
    let mut wtr = Config::new(&None).writer()?;

    #[allow(clippy::trivial_regex)]
    // template_pattern matches pairs of curly braces, e.g. "{}".
    let template_pattern = Regex::new(r"\{\}")?;

    // splitter_pattern gets all the arguments to the command as tokens.
    // The regular expression matches any sequence of characters that consists of one or more word
    // characters (`a-z`, `A-Z`, `0-9`, `_`, `.`, `+`, `-`), or any of the following three types of
    // quoted strings: double-quoted strings ("..."), single-quoted strings ('...'), or
    // backtick-quoted strings (`...`).
    let splitter_pattern = Regex::new(r#"(?:[a-zA-Z0-9_.+-]+|"[^"]*"|'[^']*'|`[^`]*`)"#)?;

    // cleaner_pattern removes the quotes or backticks from the quoted strings matched by
    // splitter_pattern.
    let cleaner_pattern = Regex::new(r#"(?:^["'`]|["'`]$)"#)?;

    let headers = rdr.byte_headers()?.clone();
    let sel = rconfig.selection(&headers)?;
    let column_index = *sel.iter().next().unwrap();

    let mut dry_run_fname = String::new();
    let dry_run = match args.flag_dry_run.as_str() {
        "true" => true,
        "false" => false,
        file_str => {
            // if the value is not "true" or "false", then it's a file name
            // check if we can create the file
            let file = std::fs::File::create(file_str);
            match file {
                Ok(_) => {
                    dry_run_fname = file_str.to_string();
                    true
                },
                Err(e) => {
                    return fail_incorrectusage_clierror!("Error creating dry-run file: {e}");
                },
            }
        },
    };

    if dry_run && args.flag_unify {
        return fail_incorrectusage_clierror!("Cannot use --unify with --dry-run");
    }

    if args.flag_new_column.is_some() && !args.flag_unify {
        return fail_incorrectusage_clierror!("Cannot use --new-column without --unify");
    }

    // create a dry-run text file to write the commands to
    let mut dry_run_file: Box<dyn Write> = Box::new(BufWriter::new(if dry_run {
        if dry_run_fname.is_empty() {
            // if dry_run_fname is empty, then we are writing to stdout
            Box::new(std::io::stdout()) as Box<dyn Write>
        } else {
            Box::new(std::fs::File::create(&dry_run_fname)?) as Box<dyn Write>
        }
    } else {
        // we're not doing a dry-run, so we don't need to write to a file
        // to satisfy the compiler, we'll just write to /dev/null
        Box::new(io::sink()) as Box<dyn Write>
    }));

    let mut record = csv::ByteRecord::new();
    let mut output_headers_written = false;
    let mut cmd_args_string;

    // prep progress bar
    let show_progress =
        (args.flag_progressbar || util::get_envvar_flag("QSV_PROGRESSBAR")) && !rconfig.is_stdin();

    let progress = ProgressBar::with_draw_target(None, ProgressDrawTarget::stderr_with_hz(5));
    if show_progress {
        util::prep_progress(&progress, util::count_rows(&rconfig)?);
    } else {
        progress.set_draw_target(ProgressDrawTarget::hidden());
    }

    while rdr.read_byte_record(&mut record)? {
        if show_progress {
            progress.inc(1);
        }
        let current_value = &record[column_index];

        let templated_command = template_pattern
            .replace_all(args.arg_command.as_bytes(), current_value)
            .to_vec();

        #[allow(unused_mut)]
        let mut command_pieces = splitter_pattern.find_iter(&templated_command);
        #[cfg(target_family = "unix")]
        let prog = OsStr::from_bytes(command_pieces.next().unwrap().as_bytes());
        #[cfg(target_family = "windows")]
        let command_bytes = command_pieces.next().unwrap().as_bytes();
        #[cfg(target_family = "windows")]
        let command_wide_char = multi_byte_to_wide_char(65001, 0, command_bytes).unwrap();
        #[cfg(target_family = "windows")]
        let prog_str = OsString::from_str(command_wide_char.as_str()).unwrap();
        #[cfg(target_family = "windows")]
        let prog = prog_str.as_os_str();

        let cmd_args: Vec<String> = command_pieces
            .map(|piece| {
                let clean_piece = cleaner_pattern.replace_all(piece.as_bytes(), NoExpand(b""));

                simdutf8::basic::from_utf8(&clean_piece)
                    .unwrap_or_default()
                    .to_string()
            })
            .collect();

        if dry_run {
            #[cfg(target_family = "unix")]
            let prog_str = simdutf8::basic::from_utf8(prog.as_bytes()).unwrap_or_default();
            #[cfg(target_family = "windows")]
            let prog_str = simdutf8::basic::from_utf8(prog.as_encoded_bytes()).unwrap_or_default();
            cmd_args_string = cmd_args.join(" ");
            dry_run_file.write_all(format!("{prog_str} {cmd_args_string}\n").as_bytes())?;
            continue;
        }
        if args.flag_unify {
            let mut cmd = Command::new(prog)
                .args(cmd_args)
                .stdout(Stdio::piped())
                .stderr(Stdio::inherit())
                .spawn()?;

            {
                let stdout = cmd.stdout.as_mut().unwrap();
                let stdout_reader = BufReader::new(stdout);

                let mut stdout_rdr = csv::ReaderBuilder::new()
                    .delimiter(match &args.flag_delimiter {
                        Some(delimiter) => delimiter.as_byte(),
                        None => b',',
                    })
                    .has_headers(true)
                    .from_reader(stdout_reader);

                let mut output_record = csv::ByteRecord::new();

                if !output_headers_written {
                    let mut headers = stdout_rdr.byte_headers()?.clone();

                    if let Some(name) = &args.flag_new_column {
                        headers.push_field(name.as_bytes());
                    }

                    wtr.write_byte_record(&headers)?;
                    output_headers_written = true;
                }

                while stdout_rdr.read_byte_record(&mut output_record)? {
                    if args.flag_new_column.is_some() {
                        output_record.push_field(current_value);
                    }

                    wtr.write_byte_record(&output_record)?;
                }
            }

            cmd.wait()?;
        } else {
            let mut cmd = Command::new(prog)
                .args(cmd_args)
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .spawn()?;

            cmd.wait()?;
        }
    }
    if show_progress {
        util::finish_progress(&progress);
    }
    dry_run_file.flush()?;
    Ok(wtr.flush()?)
}
