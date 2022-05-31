use std::borrow::ToOwned;
use std::default::Default;
use std::fmt;
use std::fs;
use std::io;
use std::iter::repeat;
use std::str::{self, FromStr};

use itertools::Itertools;
use stats::{merge_all, Commute, MinMax, OnlineStats, Unsorted};
use threadpool::ThreadPool;

use crate::config::{Config, Delimiter};
use crate::index::Indexed;
use crate::select::{SelectColumns, Selection};
use crate::util;
use crate::CliResult;
use once_cell::sync::OnceCell;
use qsv_dateparser::parse_with_preference;
use serde::Deserialize;

use self::FieldType::{TDate, TDateTime, TFloat, TInteger, TNull, TString};

static USAGE: &str = "
Computes descriptive statistics on CSV data.

Descriptive statistics includes sum, min/max, min/max length, mean, stddev, variance,
nullcount, quartiles, median, modes, & cardinality. Note that some statistics are
expensive to compute and requires loading the entire file into memory,
so they must be enabled explicitly. 

By default, the following statistics are reported for *every* column in the CSV data:
sum, min/max values, min/max length, mean, stddev, variance & nullcount. The default set of statistics 
corresponds to statistics that can be computed efficiently on a stream of data (i.e., constant memory).

The data type of each column is also inferred (Unknown, NULL, Integer, String,
Float, Date and DateTime). Note that the Date and DateTime data types are only inferred with
the --infer-dates option as its an expensive operation. The date formats recognized can be found at
https://github.com/jqnatividad/belt/tree/main/dateparser#accepted-date-formats.

Computing statistics on a large file can be made much faster if you create
an index for it first with 'qsv index' to enable multithreading.

Usage:
    qsv stats [options] [<input>]

stats options:
    -s, --select <arg>        Select a subset of columns to compute stats for.
                              See 'qsv select --help' for the format details.
                              This is provided here because piping 'qsv select'
                              into 'qsv stats' will disable the use of indexing.
    --everything              Show all statistics available.
    --mode                    Show the mode/s. Multimodal-aware.
                              This requires storing all CSV data in memory.
    --cardinality             Show the cardinality.
                              This requires storing all CSV data in memory.
    --median                  Show the median.
                              This requires storing all CSV data in memory.
    --quartiles               Show the quartiles, the IQR, the lower/upper fences
                              and skew.
                              This requires storing all CSV data in memory.
    --nulls                   Include NULLs in the population size for computing
                              mean and standard deviation.
    --infer-dates             Infer date/datetime datatypes. This is a very expensive
                              option and should only be used when you know there
                              are date/datetime fields.
    --dates-whitelist <list>  The case-insensitive patterns to look for when 
                              shortlisting fields for date inference.
                              Set to <NULL> to inspect ALL fields for
                              date/datetime types. Ignored if --infer-dates is false.
                              [default: date,time,due,opened,closed]
    --prefer-dmy              Prefer to parse dates in dmy format. Otherwise, use mdy format.
                              Ignored if --infer-dates is false.
    -j, --jobs <arg>          The number of jobs to run in parallel.
                              This works only when the given CSV has an index.
                              Note that a file handle is opened for each job.
                              When not set, the number of jobs is set to the
                              number of CPUs detected.

Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
    -n, --no-headers       When set, the first row will NOT be interpreted
                           as column names. i.e., They will be included
                           in statistics.
    -d, --delimiter <arg>  The field delimiter for reading CSV data.
                           Must be a single character. (default: ,)
";

#[derive(Clone, Deserialize)]
pub struct Args {
    pub arg_input: Option<String>,
    pub flag_select: SelectColumns,
    pub flag_everything: bool,
    pub flag_mode: bool,
    pub flag_cardinality: bool,
    pub flag_median: bool,
    pub flag_quartiles: bool,
    pub flag_nulls: bool,
    pub flag_infer_dates: bool,
    pub flag_dates_whitelist: String,
    pub flag_prefer_dmy: bool,
    pub flag_jobs: Option<usize>,
    pub flag_output: Option<String>,
    pub flag_no_headers: bool,
    pub flag_delimiter: Option<Delimiter>,
}

static INFER_DATE_FLAGS: once_cell::sync::OnceCell<Vec<bool>> = OnceCell::new();
static DMY_PREFERENCE: once_cell::sync::OnceCell<bool> = OnceCell::new();

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;

    let date_whitelist_vec = if args.flag_infer_dates {
        log::info!(
            "inferring dates using date-whitelist: {}",
            args.flag_dates_whitelist
        );

        args.flag_dates_whitelist
            .to_lowercase()
            .split(',')
            .map(|s| s.trim().to_string())
            .collect_vec()
    } else {
        vec![]
    };

    let mut wtr = Config::new(&args.flag_output).writer()?;
    let (headers, stats) = match args.rconfig().indexed()? {
        None => args.sequential_stats(&date_whitelist_vec),
        Some(idx) => {
            if let Some(num_jobs) = args.flag_jobs {
                if num_jobs == 1 {
                    args.sequential_stats(&date_whitelist_vec)
                } else {
                    args.parallel_stats(&date_whitelist_vec, idx)
                }
            } else {
                args.parallel_stats(&date_whitelist_vec, idx)
            }
        }
    }?;
    let stats = args.stats_to_records(stats);

    wtr.write_record(&args.stat_headers())?;
    let fields = headers.iter().zip(stats.into_iter());
    for (i, (header, stat)) in fields.enumerate() {
        let header = if args.flag_no_headers {
            i.to_string().into_bytes()
        } else {
            header.to_vec()
        };
        let stat = stat.iter().map(str::as_bytes);
        wtr.write_record(vec![&*header].into_iter().chain(stat))?;
    }
    wtr.flush()?;
    Ok(())
}

impl Args {
    pub fn sequential_stats(
        &self,
        d_whitelist: &[String],
    ) -> CliResult<(csv::ByteRecord, Vec<Stats>)> {
        let mut rdr = self.rconfig().reader()?;
        let (headers, sel) = self.sel_headers(&mut rdr)?;

        init_date_inference(
            self.flag_infer_dates,
            self.flag_prefer_dmy,
            &headers,
            d_whitelist,
        );

        let stats = self.compute(&sel, rdr.byte_records())?;
        Ok((headers, stats))
    }

    pub fn parallel_stats(
        &self,
        d_whitelist: &[String],
        idx: Indexed<fs::File, fs::File>,
    ) -> CliResult<(csv::ByteRecord, Vec<Stats>)> {
        // N.B. This method doesn't handle the case when the number of records
        // is zero correctly. So we use `sequential_stats` instead.
        if idx.count() == 0 {
            return self.sequential_stats(d_whitelist);
        }

        let mut rdr = self.rconfig().reader()?;
        let (headers, sel) = self.sel_headers(&mut rdr)?;

        init_date_inference(
            self.flag_infer_dates,
            self.flag_prefer_dmy,
            &headers,
            d_whitelist,
        );

        let chunk_size = util::chunk_size(idx.count() as usize, util::njobs(self.flag_jobs));
        let nchunks = util::num_of_chunks(idx.count() as usize, chunk_size);

        let pool = ThreadPool::new(util::njobs(self.flag_jobs));
        let (send, recv) = channel::bounded(0);
        for i in 0..nchunks {
            let (send, args, sel) = (send.clone(), self.clone(), sel.clone());
            pool.execute(move || {
                let mut idx = args.rconfig().indexed().unwrap().unwrap();
                idx.seek((i * chunk_size) as u64).unwrap();
                let it = idx.byte_records().take(chunk_size);
                send.send(args.compute(&sel, it).unwrap()).unwrap();
            });
        }
        drop(send);
        Ok((headers, merge_all(recv.iter()).unwrap_or_default()))
    }

    pub fn stats_to_records(&self, stats: Vec<Stats>) -> Vec<csv::StringRecord> {
        let mut records: Vec<_> = repeat(csv::StringRecord::new()).take(stats.len()).collect();
        let pool = ThreadPool::new(util::njobs(self.flag_jobs));
        let mut results = vec![];
        for mut stat in stats {
            let (send, recv) = channel::bounded(0);
            results.push(recv);
            pool.execute(move || {
                send.send(stat.to_record()).unwrap();
            });
        }
        for (i, recv) in results.into_iter().enumerate() {
            records[i] = recv.recv().unwrap();
        }
        records
    }

    #[inline]
    fn compute<I>(&self, sel: &Selection, it: I) -> CliResult<Vec<Stats>>
    where
        I: Iterator<Item = csv::Result<csv::ByteRecord>>,
    {
        let mut stats = self.new_stats(sel.len());
        for row in it {
            let row = row?;
            for (i, field) in sel.select(&row).enumerate() {
                unsafe {
                    // we use unsafe here so we skip unnecessary bounds checking
                    stats
                        .get_unchecked_mut(i)
                        .add(field, INFER_DATE_FLAGS.get().unwrap()[i]);
                }
            }
        }
        Ok(stats)
    }

    fn sel_headers<R: io::Read>(
        &self,
        rdr: &mut csv::Reader<R>,
    ) -> CliResult<(csv::ByteRecord, Selection)> {
        let headers = rdr.byte_headers()?.clone();
        let sel = self.rconfig().selection(&headers)?;
        Ok((sel.select(&headers).collect(), sel))
    }

    pub fn rconfig(&self) -> Config {
        Config::new(&self.arg_input)
            .delimiter(self.flag_delimiter)
            .no_headers(self.flag_no_headers)
            .select(self.flag_select.clone())
    }

    #[inline]
    fn new_stats(&self, record_len: usize) -> Vec<Stats> {
        repeat(Stats::new(WhichStats {
            include_nulls: self.flag_nulls,
            sum: true,
            range: true,
            dist: true,
            cardinality: self.flag_cardinality || self.flag_everything,
            median: self.flag_median && !self.flag_quartiles && !self.flag_everything,
            quartiles: self.flag_quartiles || self.flag_everything,
            mode: self.flag_mode || self.flag_everything,
        }))
        .take(record_len)
        .collect()
    }

    pub fn stat_headers(&self) -> csv::StringRecord {
        let mut fields = vec![
            "field",
            "type",
            "sum",
            "min",
            "max",
            "min_length",
            "max_length",
            "mean",
            "stddev",
            "variance",
            "nullcount",
        ];
        let all = self.flag_everything;
        if self.flag_median && !self.flag_quartiles && !all {
            fields.push("median");
        }
        if self.flag_quartiles || all {
            fields.extend_from_slice(&[
                "lower_fence",
                "q1",
                "q2_median",
                "q3",
                "iqr",
                "upper_fence",
                "skew",
            ]);
        }
        if self.flag_mode || all {
            fields.push("mode");
        }
        if self.flag_cardinality || all {
            fields.push("cardinality");
        }
        csv::StringRecord::from(fields)
    }
}

fn init_date_inference(
    infer_dates: bool,
    prefer_dmy: bool,
    headers: &csv::ByteRecord,
    d_whitelist: &[String],
) {
    if infer_dates {
        if d_whitelist[0] == "<null>" {
            log::info!("inferring dates for ALL fields...");
            INFER_DATE_FLAGS.set(vec![true; headers.len()]).unwrap();
        } else {
            let mut infer_date_flag = Vec::with_capacity(headers.len());
            for header in headers {
                let header_str = from_bytes::<String>(header).to_lowercase();
                let mut date_found = false;
                for whitelist_item in d_whitelist.iter() {
                    if header_str.contains(whitelist_item) {
                        infer_date_flag.push(true);
                        date_found = true;
                        log::info!("inferring dates for {header_str}...");
                        break;
                    }
                }
                if !date_found {
                    infer_date_flag.push(false);
                }
            }
            INFER_DATE_FLAGS.set(infer_date_flag).unwrap();
        }
    } else {
        log::info!("NOT inferring dates...");
        INFER_DATE_FLAGS.set(vec![false; headers.len()]).unwrap();
    }
    log::info!("prefer DMY?: {prefer_dmy}");
    DMY_PREFERENCE.set(prefer_dmy).unwrap();
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct WhichStats {
    include_nulls: bool,
    sum: bool,
    range: bool,
    dist: bool,
    cardinality: bool,
    median: bool,
    quartiles: bool,
    mode: bool,
}

impl Commute for WhichStats {
    #[inline]
    fn merge(&mut self, other: WhichStats) {
        assert_eq!(*self, other);
    }
}

#[derive(Clone)]
pub struct Stats {
    typ: FieldType,
    sum: Option<TypedSum>,
    minmax: Option<TypedMinMax>,
    online: Option<OnlineStats>,
    nullcount: u64,
    modes: Option<Unsorted<Vec<u8>>>,
    median: Option<Unsorted<f64>>,
    quartiles: Option<Unsorted<f64>>,
    which: WhichStats,
}

impl Stats {
    fn new(which: WhichStats) -> Stats {
        let (mut sum, mut minmax, mut online, mut modes, mut median, mut quartiles) =
            (None, None, None, None, None, None);
        if which.sum {
            sum = Some(TypedSum::default());
        }
        if which.range {
            minmax = Some(TypedMinMax::default());
        }
        if which.dist {
            online = Some(stats::OnlineStats::default());
        }
        if which.mode || which.cardinality {
            modes = Some(stats::Unsorted::default());
        }
        if which.median {
            median = Some(stats::Unsorted::default());
        }
        if which.quartiles {
            quartiles = Some(stats::Unsorted::default());
        }
        Stats {
            typ: FieldType::default(),
            sum,
            minmax,
            online,
            nullcount: 0,
            modes,
            median,
            quartiles,
            which,
        }
    }

    #[allow(clippy::option_map_unit_fn)]
    #[inline]
    fn add(&mut self, sample: &[u8], infer_dates: bool) {
        let sample_type = FieldType::from_sample(infer_dates, sample);
        self.typ.merge(sample_type);

        let t = self.typ;
        if let Some(v) = self.sum.as_mut() {
            v.add(t, sample);
        };
        if let Some(v) = self.minmax.as_mut() {
            v.add(t, sample);
        };
        if let Some(v) = self.modes.as_mut() {
            v.add(sample.to_vec());
        };
        if sample_type.is_null() {
            self.nullcount += 1;
        }
        match self.typ {
            TNull => {
                if self.which.include_nulls {
                    if let Some(v) = self.online.as_mut() {
                        v.add_null();
                    };
                }
            }
            TString => {}
            TFloat | TInteger => {
                if sample_type.is_null() {
                    if self.which.include_nulls {
                        if let Some(v) = self.online.as_mut() {
                            v.add_null();
                        };
                    }
                } else {
                    let n = from_bytes::<f64>(sample);
                    self.median.as_mut().map(|v| {
                        v.add(n);
                    });
                    self.quartiles.as_mut().map(|v| {
                        v.add(n);
                    });
                    self.online.as_mut().map(|v| {
                        v.add(n);
                    });
                }
            }
            _ => {}
        }
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn to_record(&mut self) -> csv::StringRecord {
        let typ = self.typ;
        let mut pieces = Vec::with_capacity(20);
        let empty = || "".to_owned();

        pieces.push(self.typ.to_string());
        if let Some(sum) = self.sum.as_ref().and_then(|sum| sum.show(typ)) {
            pieces.push(sum);
        } else {
            pieces.push(empty());
        }

        if let Some(mm) = self.minmax.as_ref().and_then(|mm| mm.show(typ)) {
            pieces.push(mm.0);
            pieces.push(mm.1);
        } else {
            pieces.push(empty());
            pieces.push(empty());
        }

        if let Some(mm) = self.minmax.as_ref().and_then(TypedMinMax::len_range) {
            pieces.push(mm.0);
            pieces.push(mm.1);
        } else {
            pieces.push(empty());
            pieces.push(empty());
        }

        if !self.typ.is_number() {
            pieces.push(empty());
            pieces.push(empty());
            pieces.push(empty());
        } else if let Some(ref v) = self.online {
            let mut buffer = ryu::Buffer::new();
            pieces.push(buffer.format(v.mean()).to_owned());
            pieces.push(buffer.format(v.stddev()).to_owned());
            pieces.push(buffer.format(v.variance()).to_owned());
        } else {
            pieces.push(empty());
            pieces.push(empty());
            pieces.push(empty());
        }

        let mut buffer = itoa::Buffer::new();
        pieces.push(buffer.format(self.nullcount).to_owned());

        match self.median.as_mut().and_then(|v| match self.typ {
            TInteger | TFloat => v.median(),
            _ => None,
        }) {
            None => {
                if self.which.median {
                    pieces.push(empty());
                }
            }
            Some(v) => {
                let mut buffer = ryu::Buffer::new();
                pieces.push(buffer.format(v).to_owned());
            }
        }
        match self.quartiles.as_mut().and_then(|v| match self.typ {
            TInteger | TFloat => v.quartiles(),
            _ => None,
        }) {
            None => {
                if self.which.quartiles {
                    pieces.push(empty());
                    pieces.push(empty());
                    pieces.push(empty());
                    pieces.push(empty());
                    pieces.push(empty());
                    pieces.push(empty());
                    pieces.push(empty());
                }
            }
            Some((q1, q2, q3)) => {
                let iqr = q3 - q1;
                let mut buffer = ryu::Buffer::new();
                pieces.push(buffer.format(q1 - (1.5 * iqr)).to_owned());
                pieces.push(buffer.format(q1).to_owned());
                pieces.push(buffer.format(q2).to_owned());
                pieces.push(buffer.format(q3).to_owned());
                pieces.push(buffer.format(iqr).to_owned());
                pieces.push(buffer.format(1.5f64.mul_add(iqr, q3)).to_owned());
                // calculate skewnewss using Pearson's median skewness
                // https://en.wikipedia.org/wiki/Skewness#Pearson's_second_skewness_coefficient_(median_skewness)
                let mean = self.online.unwrap().mean();
                let stddev = self.online.unwrap().stddev();
                pieces.push(buffer.format((3.0 * (mean - q2)) / stddev).to_owned());
            }
        }
        match self.modes.as_mut() {
            None => {
                if self.which.mode {
                    pieces.push(empty());
                }
                if self.which.cardinality {
                    pieces.push(empty());
                }
            }
            Some(ref mut v) => {
                if self.which.mode {
                    pieces.push(
                        v.modes()
                            .iter()
                            .map(|c| unsafe { String::from_utf8_unchecked(c.clone()) })
                            .join(","),
                    );
                }
                if self.which.cardinality {
                    let mut buffer = itoa::Buffer::new();
                    pieces.push(buffer.format(v.cardinality()).to_owned());
                }
            }
        }
        csv::StringRecord::from(pieces)
    }
}

impl Commute for Stats {
    #[inline]
    fn merge(&mut self, other: Stats) {
        self.typ.merge(other.typ);
        self.sum.merge(other.sum);
        self.minmax.merge(other.minmax);
        self.online.merge(other.online);
        self.nullcount += other.nullcount;
        self.modes.merge(other.modes);
        self.median.merge(other.median);
        self.quartiles.merge(other.quartiles);
        self.which.merge(other.which);
    }
}

#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum FieldType {
    TNull,
    TString,
    TFloat,
    TInteger,
    TDate,
    TDateTime,
}

impl FieldType {
    #[inline]
    pub fn from_sample(infer_dates: bool, sample: &[u8]) -> FieldType {
        if sample.is_empty() {
            return TNull;
        }
        // we skip utf8 validation since we say we only work with utf8
        let string = unsafe { str::from_utf8_unchecked(sample) };
        if string.parse::<i64>().is_ok() {
            return TInteger;
        }
        if string.parse::<f64>().is_ok() {
            return TFloat;
        }
        if infer_dates {
            if let Ok(parsed_date) =
                parse_with_preference(string, unsafe { *DMY_PREFERENCE.get_unchecked() })
            {
                let rfc3339_date_str = parsed_date.to_string();

                // with rfc3339 format, time component
                // starts at position 17. If its shorter than 17,
                // its a plain date, otherwise, its a datetime.
                if rfc3339_date_str.len() >= 17 {
                    return TDateTime;
                }
                return TDate;
            }
        }
        TString
    }

    #[inline]
    fn is_number(self) -> bool {
        self == TFloat || self == TInteger
    }

    #[inline]
    fn is_null(self) -> bool {
        self == TNull
    }
}

impl Commute for FieldType {
    #[inline]
    fn merge(&mut self, other: FieldType) {
        *self = match (*self, other) {
            (TString, TString) => TString,
            (TFloat, TFloat) => TFloat,
            (TInteger, TInteger) => TInteger,
            // Null does not impact the type.
            (TNull, any) | (any, TNull) => any,
            // Integers can degrade to floats.
            (TFloat, TInteger) | (TInteger, TFloat) => TFloat,
            // date data types
            (TDate, TDate) => TDate,
            (TDateTime | TDate, TDateTime) | (TDateTime, TDate) => TDateTime,
            // anything else is a String
            (_, _) => TString,
        };
    }
}

impl Default for FieldType {
    // The default is the most specific type.
    // Type inference proceeds by assuming the most specific type and then
    // relaxing the type as counter-examples are found.
    fn default() -> FieldType {
        TNull
    }
}

impl fmt::Display for FieldType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TNull => write!(f, "NULL"),
            TString => write!(f, "String"),
            TFloat => write!(f, "Float"),
            TInteger => write!(f, "Integer"),
            TDate => write!(f, "Date"),
            TDateTime => write!(f, "DateTime"),
        }
    }
}

impl fmt::Debug for FieldType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TNull => write!(f, "NULL"),
            TString => write!(f, "String"),
            TFloat => write!(f, "Float"),
            TInteger => write!(f, "Integer"),
            TDate => write!(f, "Date"),
            TDateTime => write!(f, "DateTime"),
        }
    }
}

/// TypedSum keeps a rolling sum of the data seen.
/// It sums integers until it sees a float, at which point it sums floats.
#[derive(Clone, Default)]
struct TypedSum {
    integer: i64,
    float: Option<f64>,
}

impl TypedSum {
    #[inline]
    fn add(&mut self, typ: FieldType, sample: &[u8]) {
        if sample.is_empty() {
            return;
        }
        match typ {
            TFloat => {
                let float: f64 = from_bytes::<f64>(sample);
                match self.float {
                    None => {
                        self.float = Some((self.integer as f64) + float);
                    }
                    Some(ref mut f) => {
                        *f += float;
                    }
                }
            }
            TInteger => {
                if let Some(ref mut float) = self.float {
                    *float += from_bytes::<f64>(sample);
                } else {
                    self.integer = self.integer.saturating_add(from_bytes::<i64>(sample));
                }
            }
            _ => {}
        }
    }

    #[inline]
    fn show(&self, typ: FieldType) -> Option<String> {
        match typ {
            TNull | TString | TDate | TDateTime => None,
            TInteger => {
                if self.integer == i64::MAX {
                    Some("OVERFLOW".to_string())
                } else {
                    let mut buffer = itoa::Buffer::new();
                    Some(buffer.format(self.integer).to_owned())
                }
            }
            TFloat => {
                let mut buffer = ryu::Buffer::new();
                Some(buffer.format(self.float.unwrap_or(0.0)).to_owned())
            }
        }
    }
}

impl Commute for TypedSum {
    #[inline]
    fn merge(&mut self, other: TypedSum) {
        match (self.float, other.float) {
            (Some(f1), Some(f2)) => self.float = Some(f1 + f2),
            (Some(f1), None) => self.float = Some(f1 + (other.integer as f64)),
            (None, Some(f2)) => self.float = Some((self.integer as f64) + f2),
            (None, None) => self.integer = self.integer.saturating_add(other.integer),
        }
    }
}

/// TypedMinMax keeps track of minimum/maximum values for each possible type
/// where min/max makes sense.
#[derive(Clone, Default)]
struct TypedMinMax {
    strings: MinMax<Vec<u8>>,
    str_len: MinMax<usize>,
    integers: MinMax<i64>,
    floats: MinMax<f64>,
    dates: MinMax<String>,
}

impl TypedMinMax {
    #[inline]
    fn add(&mut self, typ: FieldType, sample: &[u8]) {
        self.str_len.add(sample.len());
        if sample.is_empty() {
            return;
        }
        self.strings.add(sample.to_vec());
        match typ {
            TString | TNull => {}
            TFloat => unsafe {
                let n = str::from_utf8_unchecked(&*sample)
                    .parse::<f64>()
                    .ok()
                    .unwrap();

                self.floats.add(n);
                self.integers.add(n as i64);
            },
            TInteger => unsafe {
                let n = str::from_utf8_unchecked(&*sample)
                    .parse::<i64>()
                    .ok()
                    .unwrap();
                self.integers.add(n);
                self.floats.add(n as f64);
            },
            TDate | TDateTime => unsafe {
                let tempstr = str::from_utf8_unchecked(&*sample);
                let n = parse_with_preference(tempstr, *DMY_PREFERENCE.get_unchecked()).unwrap();
                self.dates.add(n.to_string());
            },
        }
    }

    #[inline]
    fn len_range(&self) -> Option<(String, String)> {
        match (self.str_len.min(), self.str_len.max()) {
            (Some(min), Some(max)) => {
                let mut buffer = itoa::Buffer::new();
                Some((
                    buffer.format(*min).to_owned(),
                    buffer.format(*max).to_owned(),
                ))
            }
            _ => None,
        }
    }

    #[inline]
    fn show(&self, typ: FieldType) -> Option<(String, String)> {
        match typ {
            TNull => None,
            TString => match (self.strings.min(), self.strings.max()) {
                (Some(min), Some(max)) => unsafe {
                    let min = String::from_utf8_unchecked((&**min).to_vec());
                    let max = String::from_utf8_unchecked((&**max).to_vec());
                    Some((min, max))
                },
                _ => None,
            },
            TInteger => match (self.integers.min(), self.integers.max()) {
                (Some(min), Some(max)) => {
                    let mut buffer = itoa::Buffer::new();
                    Some((
                        buffer.format(*min).to_owned(),
                        buffer.format(*max).to_owned(),
                    ))
                }
                _ => None,
            },
            TFloat => match (self.floats.min(), self.floats.max()) {
                (Some(min), Some(max)) => {
                    let mut buffer = ryu::Buffer::new();
                    Some((
                        buffer.format(*min).to_owned(),
                        buffer.format(*max).to_owned(),
                    ))
                }
                _ => None,
            },
            TDate | TDateTime => match (self.dates.min(), self.dates.max()) {
                (Some(min), Some(max)) => Some((min.to_string(), max.to_string())),
                _ => None,
            },
        }
    }
}

impl Commute for TypedMinMax {
    #[inline]
    fn merge(&mut self, other: TypedMinMax) {
        self.strings.merge(other.strings);
        self.str_len.merge(other.str_len);
        self.integers.merge(other.integers);
        self.floats.merge(other.floats);
        self.dates.merge(other.dates);
    }
}

#[inline(always)]
fn from_bytes<T: FromStr>(bytes: &[u8]) -> T
where
    <T as FromStr>::Err: std::fmt::Debug,
{
    // we don't need to do UTF-8 validation as qsv requires UTF-8 encoding
    unsafe { str::from_utf8_unchecked(bytes).parse().unwrap() }
}
