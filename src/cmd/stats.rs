use std::borrow::ToOwned;
use std::default::Default;
use std::fmt;
use std::fs;
use std::io;
use std::iter::{repeat, FromIterator};
use std::str::{self, FromStr};

use itertools::Itertools;
use stats::{merge_all, Commute, MinMax, OnlineStats, Unsorted};
use threadpool::ThreadPool;

use crate::config::{Config, Delimiter};
use crate::index::Indexed;
use crate::select::{SelectColumns, Selection};
use crate::util;
use crate::CliResult;
use dateparser::DateTimeUtc;
use serde::Deserialize;

use self::FieldType::{TDate, TFloat, TInteger, TNull, TUnicode, TUnknown};

static USAGE: &str = "
Computes basic statistics on CSV data.

Basic statistics includes sum, min/max, min/max length, mean, stddev, variance,
quartiles, median, mode, cardinality & nullcount. Note that some statistics are
expensive to compute, so they must be enabled explicitly. By default, the following
statistics are reported for *every* column in the CSV data: sum, min/max values,
min/max length, mean, stddev & variance. The default set of statistics corresponds to
statistics that can be computed efficiently on a stream of data (i.e., constant memory).

The data type of each column is also inferred (Unknown, NULL, Integer, Unicode,
Float and Date). The date formats recognized can be found at
https://docs.rs/dateparser/0.1.6/dateparser/#accepted-date-formats.

Computing statistics on a large file can be made much faster if you create
an index for it first with 'qsv index'.

Usage:
    qsv stats [options] [<input>]

stats options:
    -s, --select <arg>     Select a subset of columns to compute stats for.
                           See 'qsv select --help' for the format details.
                           This is provided here because piping 'qsv select'
                           into 'qsv stats' will disable the use of indexing.
    --everything           Show all statistics available.
    --mode                 Show the mode/s. Multimodal-aware.
                           This requires storing all CSV data in memory.
    --cardinality          Show the cardinality.
                           This requires storing all CSV data in memory.
    --median               Show the median.
                           This requires storing all CSV data in memory.
    --nullcount            Show the number of NULLs.
    --quartiles            Show the quartiles, the IQR, the lower/upper fences
                           and skew.
                           This requires storing all CSV data in memory.
    --nulls                Include NULLs in the population size for computing
                           mean and standard deviation.
    -j, --jobs <arg>       The number of jobs to run in parallel.
                           This works better when the given CSV data has
                           an index already created. Note that a file handle
                           is opened for each job.
                           When set to '0', the number of jobs is set to the
                           number of CPUs detected divided by 4.
                           When set to '-1', the number of jobs is set to the
                           number of CPUs detected.
                           [default: 0]

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
struct Args {
    arg_input: Option<String>,
    flag_select: SelectColumns,
    flag_everything: bool,
    flag_mode: bool,
    flag_cardinality: bool,
    flag_median: bool,
    flag_quartiles: bool,
    flag_nulls: bool,
    flag_nullcount: bool,
    flag_jobs: isize,
    flag_output: Option<String>,
    flag_no_headers: bool,
    flag_delimiter: Option<Delimiter>,
}

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;

    let mut wtr = Config::new(&args.flag_output).writer()?;
    let (headers, stats) = match args.rconfig().indexed()? {
        None => args.sequential_stats(),
        Some(idx) => {
            if args.flag_jobs == 1 {
                args.sequential_stats()
            } else {
                args.parallel_stats(idx)
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
        let stat = stat.iter().map(|f| f.as_bytes());
        wtr.write_record(vec![&*header].into_iter().chain(stat))?;
    }
    wtr.flush()?;
    Ok(())
}

impl Args {
    fn sequential_stats(&self) -> CliResult<(csv::ByteRecord, Vec<Stats>)> {
        let mut rdr = self.rconfig().reader()?;
        let (headers, sel) = self.sel_headers(&mut rdr)?;
        let stats = self.compute(&sel, rdr.byte_records())?;
        Ok((headers, stats))
    }

    fn parallel_stats(
        &self,
        idx: Indexed<fs::File, fs::File>,
    ) -> CliResult<(csv::ByteRecord, Vec<Stats>)> {
        // N.B. This method doesn't handle the case when the number of records
        // is zero correctly. So we use `sequential_stats` instead.
        if idx.count() == 0 {
            return self.sequential_stats();
        }

        let mut rdr = self.rconfig().reader()?;
        let (headers, sel) = self.sel_headers(&mut rdr)?;

        let chunk_size = util::chunk_size(idx.count() as usize, self.njobs());
        let nchunks = util::num_of_chunks(idx.count() as usize, chunk_size);

        let pool = ThreadPool::new(self.njobs());
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
        Ok((headers, merge_all(recv.iter()).unwrap_or_else(Vec::new)))
    }

    fn stats_to_records(&self, stats: Vec<Stats>) -> Vec<csv::StringRecord> {
        let mut records: Vec<_> = repeat(csv::StringRecord::new()).take(stats.len()).collect();
        let pool = ThreadPool::new(self.njobs());
        let mut results = vec![];
        for mut stat in stats.into_iter() {
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

    fn compute<I>(&self, sel: &Selection, it: I) -> CliResult<Vec<Stats>>
    where
        I: Iterator<Item = csv::Result<csv::ByteRecord>>,
    {
        let mut stats = self.new_stats(sel.len());
        for row in it {
            let row = row?;
            for (i, field) in sel.select(&row).enumerate() {
                stats[i].add(field);
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
        Ok((csv::ByteRecord::from_iter(sel.select(&headers)), sel))
    }

    fn rconfig(&self) -> Config {
        Config::new(&self.arg_input)
            .delimiter(self.flag_delimiter)
            .no_headers(self.flag_no_headers)
            .select(self.flag_select.clone())
    }

    fn njobs(&self) -> usize {
        let num_cpus = util::num_cpus();
        match self.flag_jobs {
          0 => util::max_jobs(),
          flag_jobs if flag_jobs < 0 => num_cpus,
          flag_jobs if flag_jobs > num_cpus as isize => num_cpus,
          _ => self.flag_jobs as usize,
        }
    }

    fn new_stats(&self, record_len: usize) -> Vec<Stats> {
        repeat(Stats::new(WhichStats {
            include_nulls: self.flag_nulls,
            sum: true,
            range: true,
            dist: true,
            cardinality: self.flag_cardinality || self.flag_everything,
            nullcount: self.flag_nullcount || self.flag_everything,
            median: self.flag_median && !self.flag_quartiles && !self.flag_everything,
            quartiles: self.flag_quartiles || self.flag_everything,
            mode: self.flag_mode || self.flag_everything,
        }))
        .take(record_len)
        .collect()
    }

    fn stat_headers(&self) -> csv::StringRecord {
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
        ];
        let all = self.flag_everything;
        if self.flag_median && !self.flag_quartiles && !all {
            fields.push("median");
        }
        if self.flag_quartiles || all {
            fields.push("lower_fence");
            fields.push("q1");
            fields.push("q2_median");
            fields.push("q3");
            fields.push("iqr");
            fields.push("upper_fence");
            fields.push("skew");
        }
        if self.flag_mode || all {
            fields.push("mode");
        }
        if self.flag_cardinality || all {
            fields.push("cardinality");
        }
        if self.flag_nullcount || all {
            fields.push("nullcount");
        }
        csv::StringRecord::from(fields)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct WhichStats {
    include_nulls: bool,
    sum: bool,
    range: bool,
    dist: bool,
    cardinality: bool,
    nullcount: bool,
    median: bool,
    quartiles: bool,
    mode: bool,
}

impl Commute for WhichStats {
    fn merge(&mut self, other: WhichStats) {
        assert_eq!(*self, other);
    }
}

#[derive(Clone)]
struct Stats {
    typ: FieldType,
    sum: Option<TypedSum>,
    minmax: Option<TypedMinMax>,
    online: Option<OnlineStats>,
    modes: Option<Unsorted<Vec<u8>>>,
    median: Option<Unsorted<f64>>,
    nullcount: u64,
    quartiles: Option<Unsorted<f64>>,
    which: WhichStats,
}

impl Stats {
    fn new(which: WhichStats) -> Stats {
        let (mut sum, mut minmax, mut online, mut modes, mut median, mut quartiles) =
            (None, None, None, None, None, None);
        if which.sum {
            sum = Some(Default::default());
        }
        if which.range {
            minmax = Some(Default::default());
        }
        if which.dist {
            online = Some(Default::default());
        }
        if which.mode || which.cardinality {
            modes = Some(Default::default());
        }
        if which.median {
            median = Some(Default::default());
        }
        if which.quartiles {
            quartiles = Some(Default::default());
        }
        Stats {
            typ: Default::default(),
            sum,
            minmax,
            online,
            modes,
            median,
            nullcount: 0,
            quartiles,
            which,
        }
    }

    #[allow(clippy::option_map_unit_fn)]
    fn add(&mut self, sample: &[u8]) {
        let sample_type = FieldType::from_sample(sample);
        self.typ.merge(sample_type);

        let t = self.typ;
        if let Some(v) = self.sum.as_mut() {
            v.add(t, sample)
        };
        if let Some(v) = self.minmax.as_mut() {
            v.add(t, sample)
        };
        if let Some(v) = self.modes.as_mut() {
            v.add(sample.to_vec())
        };
        if sample_type.is_null() {
            self.nullcount += 1;
        }
        match self.typ {
            TUnknown => {}
            TNull => {
                if self.which.include_nulls {
                    if let Some(v) = self.online.as_mut() {
                        v.add_null();
                    };
                }
            }
            TUnicode => {}
            TFloat | TInteger => {
                if sample_type.is_null() {
                    if self.which.include_nulls {
                        if let Some(v) = self.online.as_mut() {
                            v.add_null();
                        };
                    }
                } else {
                    let n = from_bytes::<f64>(sample).unwrap();
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
            TDate => {}
        }
    }

    #[allow(clippy::wrong_self_convention)]
    fn to_record(&mut self) -> csv::StringRecord {
        let typ = self.typ;
        let mut pieces = vec![];
        let empty = || "".to_owned();

        pieces.push(self.typ.to_string());
        match self.sum.as_ref().and_then(|sum| sum.show(typ)) {
            Some(sum) => {
                pieces.push(sum);
            }
            None => {
                pieces.push(empty());
            }
        }
        match self.minmax.as_ref().and_then(|mm| mm.show(typ)) {
            Some(mm) => {
                pieces.push(mm.0);
                pieces.push(mm.1);
            }
            None => {
                pieces.push(empty());
                pieces.push(empty());
            }
        }
        match self.minmax.as_ref().and_then(|mm| mm.len_range()) {
            Some(mm) => {
                pieces.push(mm.0);
                pieces.push(mm.1);
            }
            None => {
                pieces.push(empty());
                pieces.push(empty());
            }
        }

        if !self.typ.is_number() {
            pieces.push(empty());
            pieces.push(empty());
            pieces.push(empty());
        } else {
            match self.online {
                Some(ref v) => {
                    pieces.push(v.mean().to_string());
                    pieces.push(v.stddev().to_string());
                    pieces.push(v.variance().to_string());
                }
                None => {
                    pieces.push(empty());
                    pieces.push(empty());
                    pieces.push(empty());
                }
            }
        }
        match self.median.as_mut().and_then(|v| v.median()) {
            None => {
                if self.which.median {
                    pieces.push(empty());
                }
            }
            Some(v) => {
                pieces.push(v.to_string());
            }
        }
        match self.quartiles.as_mut().and_then(|v| v.quartiles()) {
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
                pieces.push((q1 - (1.5 * iqr)).to_string());
                pieces.push(q1.to_string());
                pieces.push(q2.to_string());
                pieces.push(q3.to_string());
                pieces.push(iqr.to_string());
                pieces.push((q3 + (1.5 * iqr)).to_string());
                // calculate skewnewss using Pearson's median skewness
                // https://en.wikipedia.org/wiki/Skewness#Pearson's_second_skewness_coefficient_(median_skewness)
                let _mean = self.online.unwrap().mean();
                let _stddev = self.online.unwrap().stddev();
                pieces.push(((3.0 * (_mean - q2)) / _stddev).to_string());
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
                            .map(|c| String::from_utf8_lossy(c))
                            .join(","),
                    );
                }
                if self.which.cardinality {
                    pieces.push(v.cardinality().to_string());
                }
            }
        }
        if self.which.nullcount {
            pieces.push(self.nullcount.to_string());
        }
        csv::StringRecord::from(pieces)
    }
}

impl Commute for Stats {
    fn merge(&mut self, other: Stats) {
        self.typ.merge(other.typ);
        self.sum.merge(other.sum);
        self.minmax.merge(other.minmax);
        self.online.merge(other.online);
        self.modes.merge(other.modes);
        self.median.merge(other.median);
        self.nullcount += other.nullcount;
        self.quartiles.merge(other.quartiles);
        self.which.merge(other.which);
    }
}

#[derive(Clone, Copy, PartialEq)]
enum FieldType {
    TUnknown,
    TNull,
    TUnicode,
    TFloat,
    TInteger,
    TDate,
}

impl FieldType {
    fn from_sample(sample: &[u8]) -> FieldType {
        if sample.is_empty() {
            return TNull;
        }
        let string = match str::from_utf8(sample) {
            Err(_) => return TUnknown,
            Ok(s) => s,
        };
        if string.parse::<i64>().is_ok() {
            return TInteger;
        }
        if string.parse::<f64>().is_ok() {
            return TFloat;
        }
        if string.parse::<DateTimeUtc>().is_ok() {
            return TDate;
        }
        TUnicode
    }

    fn is_number(&self) -> bool {
        *self == TFloat || *self == TInteger
    }

    fn is_null(&self) -> bool {
        *self == TNull
    }
}

impl Commute for FieldType {
    fn merge(&mut self, other: FieldType) {
        *self = match (*self, other) {
            (TUnicode, TUnicode) => TUnicode,
            (TFloat, TFloat) => TFloat,
            (TInteger, TInteger) => TInteger,
            (TDate, TDate) => TDate,
            // Null does not impact the type.
            (TNull, any) | (any, TNull) => any,
            // There's no way to get around an unknown.
            (TUnknown, _) | (_, TUnknown) => TUnknown,
            // Integers can degrade to floats.
            (TFloat, TInteger) | (TInteger, TFloat) => TFloat,
            // when using unixtime format can degrade to int/floats.
            (TInteger, TDate) | (TDate, TInteger) => TInteger,
            (TFloat, TDate) | (TDate, TFloat) => TFloat,
            // Numbers/dates can degrade to Unicode strings.
            (TUnicode, TFloat) | (TFloat, TUnicode) => TUnicode,
            (TUnicode, TInteger) | (TInteger, TUnicode) => TUnicode,
            (TUnicode, TDate) | (TDate, TUnicode) => TUnicode,
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
            TUnknown => write!(f, "Unknown"),
            TNull => write!(f, "NULL"),
            TUnicode => write!(f, "Unicode"),
            TFloat => write!(f, "Float"),
            TInteger => write!(f, "Integer"),
            TDate => write!(f, "Date"),
        }
    }
}

/// TypedSum keeps a rolling sum of the data seen.
///
/// It sums integers until it sees a float, at which point it sums floats.
#[derive(Clone, Default)]
struct TypedSum {
    integer: i64,
    float: Option<f64>,
}

impl TypedSum {
    fn add(&mut self, typ: FieldType, sample: &[u8]) {
        if sample.is_empty() {
            return;
        }
        match typ {
            TFloat => {
                let float: f64 = from_bytes::<f64>(sample).unwrap();
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
                    *float += from_bytes::<f64>(sample).unwrap();
                } else {
                    self.integer += from_bytes::<i64>(sample).unwrap();
                }
            }
            _ => {}
        }
    }

    fn show(&self, typ: FieldType) -> Option<String> {
        match typ {
            TNull | TUnicode | TUnknown | TDate => None,
            TInteger => Some(self.integer.to_string()),
            TFloat => Some(self.float.unwrap_or(0.0).to_string()),
        }
    }
}

impl Commute for TypedSum {
    fn merge(&mut self, other: TypedSum) {
        match (self.float, other.float) {
            (Some(f1), Some(f2)) => self.float = Some(f1 + f2),
            (Some(f1), None) => self.float = Some(f1 + (other.integer as f64)),
            (None, Some(f2)) => self.float = Some((self.integer as f64) + f2),
            (None, None) => self.integer += other.integer,
        }
    }
}

/// TypedMinMax keeps track of minimum/maximum values for each possible type
/// where min/max makes sense.
#[derive(Clone)]
struct TypedMinMax {
    strings: MinMax<Vec<u8>>,
    str_len: MinMax<usize>,
    integers: MinMax<i64>,
    floats: MinMax<f64>,
    dates: MinMax<String>,
}

impl TypedMinMax {
    fn add(&mut self, typ: FieldType, sample: &[u8]) {
        self.str_len.add(sample.len());
        if sample.is_empty() {
            return;
        }
        self.strings.add(sample.to_vec());
        match typ {
            TUnicode | TUnknown | TNull => {}
            TFloat => {
                let n = str::from_utf8(&*sample)
                    .ok()
                    .and_then(|s| s.parse::<f64>().ok())
                    .unwrap();
                self.floats.add(n);
                self.integers.add(n as i64);
            }
            TInteger => {
                let n = str::from_utf8(&*sample)
                    .ok()
                    .and_then(|s| s.parse::<i64>().ok())
                    .unwrap();
                self.integers.add(n);
                self.floats.add(n as f64);
            }
            TDate => {
                let n = str::from_utf8(&*sample)
                    .ok()
                    .and_then(|s| dateparser::parse(s).ok())
                    .unwrap();

                self.dates.add(n.to_string());
            }
        }
    }

    fn len_range(&self) -> Option<(String, String)> {
        match (self.str_len.min(), self.str_len.max()) {
            (Some(min), Some(max)) => Some((min.to_string(), max.to_string())),
            _ => None,
        }
    }

    fn show(&self, typ: FieldType) -> Option<(String, String)> {
        match typ {
            TNull => None,
            TUnicode | TUnknown => match (self.strings.min(), self.strings.max()) {
                (Some(min), Some(max)) => {
                    let min = String::from_utf8_lossy(&**min).to_string();
                    let max = String::from_utf8_lossy(&**max).to_string();
                    Some((min, max))
                }
                _ => None,
            },
            TDate => match (self.dates.min(), self.dates.max()) {
                (Some(min), Some(max)) => Some((min.to_string(), max.to_string())),
                _ => None,
            },
            TInteger => match (self.integers.min(), self.integers.max()) {
                (Some(min), Some(max)) => Some((min.to_string(), max.to_string())),
                _ => None,
            },
            TFloat => match (self.floats.min(), self.floats.max()) {
                (Some(min), Some(max)) => Some((min.to_string(), max.to_string())),
                _ => None,
            },
        }
    }
}

impl Default for TypedMinMax {
    fn default() -> TypedMinMax {
        TypedMinMax {
            strings: Default::default(),
            str_len: Default::default(),
            integers: Default::default(),
            floats: Default::default(),
            dates: Default::default(),
        }
    }
}

impl Commute for TypedMinMax {
    fn merge(&mut self, other: TypedMinMax) {
        self.strings.merge(other.strings);
        self.str_len.merge(other.str_len);
        self.integers.merge(other.integers);
        self.floats.merge(other.floats);
        self.dates.merge(other.dates);
    }
}

fn from_bytes<T: FromStr>(bytes: &[u8]) -> Option<T> {
    str::from_utf8(bytes).ok().and_then(|s| s.parse().ok())
}
