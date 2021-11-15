use crate::config::{Config, Delimiter};
use crate::util;
use crate::CliResult;
use serde::Deserialize;
use std::env::temp_dir;
use std::fs;
use std::io::{self, Write};
use test_data_generation::data_sample_parser::DataSampleParser;
use uuid::Uuid;

static USAGE: &str = r#"
Generates test data by profiling a CSV using a Markov decision process
machine learning algorithm.

Also allows you to create and use Data Sample Parser (DSP) profiles
to generate test data without access to the original profiled CSV.
See https://docs.rs/test-data-generation/ for more info.

Examples:

Generate 100 rows of test data based on prod-data.csv and 
save it in testdata.csv.

$  qsv generate --rows 100 prod-data.csv > testdata.csv

Generate 100 rows based on prod-data.csv and save it to a
file named testdata.csv. Also create a DSP profile named
prod-profile (which is saved as prod-profile.json in the file system)

$ qsv generate -r 100 prod-data.csv --outdsp prod-profile --output testdata.csv

Generate 100 rows based on an existing DSP profile (prod-profile.json)
and save it to testdata.csv

$ qsv generate -r 100 --indsp prod-profile > testdata.csv

Create a DSP profile (prod-profile.json) based on prod-data.csv.

$ qsv generate prod-data.csv --outdsp prod-profile

Usage:
    qsv generate [options] [--rows=<count>] <input>
    qsv generate [options] [--rows=<count>] (--indsp=<file>)
    qsv generate [options] (--outdsp=<file>) [<input>]

generate options:
    -r, --rows=<count>     Number of rows of test data to generate.
                           [default: 0]
    --outdsp <file>        Create a Data Sample Parser (DSP) JSON file
                           based on the <input> file.
                           .json file extension automatically added.
    --indsp <file>         Use a DSP JSON file to generate test data.
                           .json file extension assumed.

Common options:
    -h, --help             Display this message
    -o, --output <file>    Write generated output to <file>.
    -d, --delimiter <arg>  The field delimiter for reading CSV data.
                           Must be a single character. (default: ,)
"#;

#[derive(Deserialize)]
struct Args {
    arg_input: Option<String>,
    flag_rows: u32,
    flag_output: Option<String>,
    flag_outdsp: Option<String>,
    flag_indsp: Option<String>,
    flag_delimiter: Option<Delimiter>,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;
    let conf = Config::new(&args.arg_input).delimiter(args.flag_delimiter);

    let tdir = temp_dir();
    let mut dsp = DataSampleParser::new();

    if let Some(indsp) = args.flag_indsp {
        // use an existing DSP JSON file, no need to read input CSV
        dsp = DataSampleParser::from_file(&indsp);
    } else {
        // create a DSP profile from the input CSV
        let mut rdr = conf.reader()?;

        let in_fname = format!("{}.csv", Uuid::new_v4());
        let in_fpath = tdir.join(in_fname);
        let analyze_csv_path = in_fpath
            .clone()
            .into_os_string()
            .into_string()
            .unwrap_or_default();

        let headers = rdr.byte_headers()?;
        let mut wtr = csv::Writer::from_path(in_fpath)?;
        wtr.write_byte_record(headers)?;

        let mut record = csv::ByteRecord::new();
        while rdr.read_byte_record(&mut record)? {
            wtr.write_byte_record(&record)?;
        }
        wtr.flush()?;

        dsp.analyze_csv_file(&analyze_csv_path, None).unwrap();

        // --outdsp option invoked. Save DSP JSON file that we
        // can use later with --indsp option to generate test data
        // without expensive test data profiling
        if let Some(outdsp) = args.flag_outdsp {
            dsp.save(&outdsp)?;
        };
        drop(wtr);
    }

    if args.flag_rows > 0 {
        let mut send_to_stdout: bool = false;
        let testdata_out = match args.flag_output {
            Some(path) => path,
            None => {
                send_to_stdout = true;
                let fname = format!("{}.csv", Uuid::new_v4());
                let fpath = tdir.join(fname);
                fpath.into_os_string().into_string().unwrap_or_default()
            }
        };

        dsp.generate_csv(args.flag_rows, &testdata_out, Some(conf.get_delimiter()))
            .unwrap();

        if send_to_stdout {
            let testdata = std::fs::read(&testdata_out)?;
            io::stdout().write_all(&testdata)?;
            io::stdout().flush()?;
            fs::remove_file(&testdata_out)?;
        }
    }
    Ok(())
}
