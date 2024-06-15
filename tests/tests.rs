extern crate log;
extern crate serde;

extern crate csv;
extern crate filetime;
extern crate quickcheck;
extern crate rand;
extern crate stats;

use std::{env, fmt, mem::transmute, ops};

use quickcheck::{Arbitrary, Gen, QuickCheck, Testable};
use rand::{thread_rng, Rng};

macro_rules! svec[
    ($($x:expr),*) => (
        vec![$($x),*].into_iter()
                     .map(|s: &'static str| s.to_string())
                     .collect::<Vec<String>>()
    );
    ($($x:expr,)*) => (svec![$($x),*]);
];

macro_rules! rassert_eq {
    ($given:expr, $expected:expr) => {{
        assert_eq!($given, $expected);
        true
    }};
}

mod workdir;

mod test_100;
#[cfg(feature = "apply")]
mod test_apply;
#[cfg(feature = "datapusher_plus")]
mod test_applydp;
#[cfg(any(feature = "feature_capable", feature = "lite"))]
mod test_behead;
#[cfg(any(feature = "feature_capable", feature = "lite"))]
mod test_cat;
mod test_combos;
mod test_comments;
mod test_count;
mod test_datefmt;
mod test_dedup;
mod test_describegpt;
mod test_diff;
#[cfg(any(feature = "feature_capable", feature = "lite"))]
mod test_enumerate;
mod test_excel;
mod test_exclude;
#[cfg(any(feature = "feature_capable", feature = "lite"))]
mod test_explode;
mod test_extdedup;
#[cfg(any(feature = "feature_capable", feature = "lite"))]
mod test_extsort;
#[cfg(feature = "fetch")]
mod test_fetch;
#[cfg(any(feature = "feature_capable", feature = "lite"))]
mod test_fill;
#[cfg(any(feature = "feature_capable", feature = "lite"))]
mod test_fixlengths;
#[cfg(any(feature = "feature_capable", feature = "lite"))]
mod test_flatten;
#[cfg(any(feature = "feature_capable", feature = "lite"))]
mod test_fmt;
#[cfg(all(feature = "foreach"))]
mod test_foreach;
mod test_frequency;
#[cfg(all(feature = "feature_capable", feature = "geocode"))]
mod test_geocode;
mod test_headers;
mod test_index;
mod test_input;
#[cfg(any(feature = "feature_capable", feature = "lite"))]
mod test_join;
#[cfg(feature = "polars")]
mod test_joinp;
#[cfg(any(feature = "feature_capable", feature = "lite"))]
mod test_jsonl;
#[cfg(all(feature = "polars", not(feature = "datapusher_plus")))]
mod test_jsonp;
#[cfg(feature = "luau")]
mod test_luau;
#[cfg(any(feature = "feature_capable", feature = "lite"))]
mod test_partition;
mod test_prompt;
mod test_pseudo;
#[cfg(feature = "python")]
mod test_py;
mod test_rename;
mod test_replace;
mod test_reverse;
mod test_safenames;
mod test_sample;
#[cfg(any(feature = "feature_capable", feature = "lite"))]
mod test_schema;
mod test_search;
mod test_searchset;
mod test_select;
mod test_slice;
mod test_snappy;
mod test_sniff;
mod test_sort;
mod test_sortcheck;
#[cfg(any(feature = "feature_capable", feature = "lite"))]
mod test_split;
#[cfg(feature = "polars")]
mod test_sqlp;
mod test_stats;
#[cfg(any(feature = "feature_capable", feature = "lite"))]
mod test_table;
#[cfg(all(feature = "to", feature = "feature_capable"))]
mod test_to;
#[cfg(any(feature = "feature_capable", feature = "lite"))]
mod test_tojsonl;
#[cfg(any(feature = "feature_capable", feature = "lite"))]
mod test_transpose;
mod test_validate;

fn qcheck<T: Testable>(p: T) {
    env::set_var("QSV_SKIPUTF8_CHECK", "1");
    QuickCheck::new().gen(Gen::new(5)).quickcheck(p);
    env::set_var("QSV_SKIPUTF8_CHECK", "");
}

fn qcheck_sized<T: Testable>(p: T, size: usize) {
    env::set_var("QSV_SKIPUTF8_CHECK", "1");
    QuickCheck::new().gen(Gen::new(size)).quickcheck(p);
    env::set_var("QSV_SKIPUTF8_CHECK", "");
}

pub type CsvVecs = Vec<Vec<String>>;

pub trait Csv {
    fn to_vecs(self) -> CsvVecs;
    fn from_vecs(_: CsvVecs) -> Self;
}

impl Csv for CsvVecs {
    fn to_vecs(self) -> CsvVecs {
        self
    }

    fn from_vecs(vecs: CsvVecs) -> CsvVecs {
        vecs
    }
}

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
struct CsvRecord(Vec<String>);

impl CsvRecord {
    fn unwrap(self) -> Vec<String> {
        let CsvRecord(v) = self;
        v
    }
}

#[allow(clippy::needless_lifetimes)]
impl ops::Deref for CsvRecord {
    type Target = [String];

    fn deref<'a>(&'a self) -> &'a [String] {
        &self.0
    }
}

#[allow(clippy::needless_lifetimes)]
impl ops::DerefMut for CsvRecord {
    fn deref_mut<'a>(&'a mut self) -> &'a mut [String] {
        &mut self.0
    }
}

impl fmt::Debug for CsvRecord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let bytes: Vec<_> = self.iter().map(std::string::String::as_bytes).collect();
        write!(f, "{bytes:?}")
    }
}

impl Arbitrary for CsvRecord {
    fn arbitrary(g: &mut Gen) -> CsvRecord {
        let size = g.size();
        CsvRecord((0..size).map(|_| Arbitrary::arbitrary(g)).collect())
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = CsvRecord> + 'static> {
        Box::new(
            self.clone()
                .unwrap()
                .shrink()
                .filter(|r| !r.is_empty())
                .map(CsvRecord),
        )
    }
}

impl Csv for Vec<CsvRecord> {
    fn to_vecs(self) -> CsvVecs {
        unsafe { transmute(self) }
    }

    fn from_vecs(vecs: CsvVecs) -> Vec<CsvRecord> {
        unsafe { transmute(vecs) }
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialOrd)]
struct CsvData {
    data: Vec<CsvRecord>,
}

impl CsvData {
    fn unwrap(self) -> Vec<CsvRecord> {
        self.data
    }

    fn len(&self) -> usize {
        (**self).len()
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

#[allow(clippy::needless_lifetimes)]
impl ops::Deref for CsvData {
    type Target = [CsvRecord];

    fn deref<'a>(&'a self) -> &'a [CsvRecord] {
        &self.data
    }
}

impl Arbitrary for CsvData {
    fn arbitrary(g: &mut Gen) -> CsvData {
        let record_len = g.size();
        let mut rng = thread_rng();

        let num_records: usize = rng.gen_range(0..100);
        let mut d = CsvData {
            data: (0..num_records)
                .map(|_| CsvRecord((0..record_len).map(|_| Arbitrary::arbitrary(g)).collect()))
                .collect(),
        };
        // If the CSV data starts with a BOM, strip it, because it wreaks havoc
        // with tests that weren't designed to handle it.
        if !d.data.is_empty() && !d.data[0].is_empty() {
            if let Some(stripped) = d.data[0][0].strip_prefix('\u{FEFF}') {
                d.data[0][0] = stripped.to_string();
            }
        }
        d
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = CsvData> + 'static> {
        let len = if self.is_empty() { 0 } else { self[0].len() };
        let mut rows: Vec<CsvData> = self
            .clone()
            .unwrap()
            .shrink()
            .filter(|rows| rows.iter().all(|r| r.len() == len))
            .map(|rows| CsvData { data: rows })
            .collect();
        // We should also introduce CSV data with fewer columns...
        if len > 1 {
            rows.extend(
                self.clone()
                    .unwrap()
                    .shrink()
                    .filter(|rows| rows.iter().all(|r| r.len() == len - 1))
                    .map(|rows| CsvData { data: rows }),
            );
        }
        Box::new(rows.into_iter())
    }
}

impl Csv for CsvData {
    fn to_vecs(self) -> CsvVecs {
        unsafe { transmute(self.data) }
    }

    fn from_vecs(vecs: CsvVecs) -> CsvData {
        CsvData {
            data: unsafe { transmute(vecs) },
        }
    }
}

impl PartialEq for CsvData {
    fn eq(&self, other: &CsvData) -> bool {
        (self.data.is_empty() && other.data.is_empty()) || self.data == other.data
    }
}
