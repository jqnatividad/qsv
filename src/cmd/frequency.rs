static USAGE: &str = r#"
Compute a frequency table on CSV data.

The frequency table is formatted as CSV data:

    field,value,count,percentage

By default, there is a row for the N most frequent values for each field in the data.
The order and number of values can be configured with --asc, --limit, --unq-limit
and --lmt-threshold respectively.

The unique limit (--unq-limit) is particularly useful when a column has all unique values
(e.g. an ID column) and --limit is set to 0.
Without a unique limit, the frequency table for that column will be the same as the number
of rows in the data.
With a unique limit, the frequency table will be a sample of N unique values, all with
a count of 1.

The --lmt-threshold option allows you to apply the --limit and --unq-limit options only
when the number of unique items in a column is greater than or equal to the threshold.
This is useful when you want to apply limits only to columns with a large number of unique
items and not to columns with a small number of unique items.

Since this computes an exact frequency table, memory proportional to the
cardinality of each column is required.

For examples, see https://github.com/jqnatividad/qsv/blob/master/tests/test_frequency.rs.

Usage:
    qsv frequency [options] [<input>]
    qsv frequency --help

frequency options:
    -s, --select <arg>      Select a subset of columns to compute frequencies
                            for. See 'qsv select --help' for the format
                            details. This is provided here because piping 'qsv
                            select' into 'qsv frequency' will disable the use
                            of indexing.
    -l, --limit <arg>       Limit the frequency table to the N most common
                            items. Set to '0' to disable a limit.
                            If negative, only return values with an occurrence
                            count >= absolute value of the negative limit.
                            e.g. --limit -2 will only return values with an
                            occurrence count >= 2.
                            [default: 10]
    -u, --unq-limit <arg>   If a column has all unique values, limit the
                            frequency table to a sample of N unique items.
                            Set to '0' to disable a unique_limit.
                            [default: 10]
    --lmt-threshold <arg>   The threshold for which --limit and --unq-limit
                            will be applied. If the number of unique items
                            in a column >= threshold, the limits will be applied.
                            Set to '0' to disable the threshold and always apply limits.
                            [default: 0]
    --pct-dec-places <arg>  The number of decimal places to round the percentage to.
                            If negative, the number of decimal places will be set
                            automatically to the minimum number of decimal places needed
                            to represent the percentage accurately, up to the absolute
                            value of the negative number.
                            [default: -5]
    --other-sorted          By default, the "Other" category is placed at the
                            end of the frequency table for a field. If this is enabled, the
                            "Other" category will be sorted with the rest of the
                            values by count.
    --other-text <arg>      The text to use for the "Other" category. If set to "<NONE>",
                            the "Other" category will not be included in the frequency table.
                            [default: Other]
    -a, --asc               Sort the frequency tables in ascending order by count.
                            The default is descending order.
    --no-trim               Don't trim whitespace from values when computing frequencies.
                            The default is to trim leading and trailing whitespaces.
    --no-nulls              Don't include NULLs in the frequency table.
    -i, --ignore-case       Ignore case when computing frequencies.
    -j, --jobs <arg>        The number of jobs to run in parallel.
                            This works much faster when the given CSV data has
                            an index already created. Note that a file handle
                            is opened for each job.
                            When not set, the number of jobs is set to the
                            number of CPUs detected.

Common options:
    -h, --help             Display this message
    -o, --output <file>    Write output to <file> instead of stdout.
    -n, --no-headers       When set, the first row will NOT be included
                           in the frequency table. Additionally, the 'field'
                           column will be 1-based indices instead of header
                           names.
    -d, --delimiter <arg>  The field delimiter for reading CSV data.
                           Must be a single character. (default: ,)
    --memcheck             Check if there is enough memory to load the entire
                           CSV into memory using CONSERVATIVE heuristics.
"#;

use std::{fs, io};

use indicatif::HumanCount;
use rust_decimal::prelude::*;
use serde::Deserialize;
use stats::{merge_all, Frequencies};
use threadpool::ThreadPool;

use crate::{
    config::{Config, Delimiter},
    index::Indexed,
    select::{SelectColumns, Selection},
    util,
    util::ByteString,
    CliResult,
};

#[allow(clippy::unsafe_derive_deserialize)]
#[derive(Clone, Deserialize)]
pub struct Args {
    pub arg_input:           Option<String>,
    pub flag_select:         SelectColumns,
    pub flag_limit:          isize,
    pub flag_unq_limit:      usize,
    pub flag_lmt_threshold:  usize,
    pub flag_pct_dec_places: isize,
    pub flag_other_sorted:   bool,
    pub flag_other_text:     String,
    pub flag_asc:            bool,
    pub flag_no_trim:        bool,
    pub flag_no_nulls:       bool,
    pub flag_ignore_case:    bool,
    pub flag_jobs:           Option<usize>,
    pub flag_output:         Option<String>,
    pub flag_no_headers:     bool,
    pub flag_delimiter:      Option<Delimiter>,
    pub flag_memcheck:       bool,
}

const NULL_VAL: &[u8] = b"(NULL)";

pub fn run(argv: &[&str]) -> CliResult<()> {
    let args: Args = util::get_args(USAGE, argv)?;
    let rconfig = args.rconfig();

    // we're loading the entire file into memory, we need to check avail mem
    if let Some(path) = rconfig.path.clone() {
        util::mem_file_check(&path, false, args.flag_memcheck)?;
    }

    let mut wtr = Config::new(&args.flag_output).writer()?;
    let (headers, tables) = match args.rconfig().indexed()? {
        Some(ref mut idx) if util::njobs(args.flag_jobs) > 1 => args.parallel_ftables(idx),
        _ => args.sequential_ftables(),
    }?;

    #[allow(unused_assignments)]
    let mut header_vec: Vec<u8> = Vec::with_capacity(tables.len());
    let mut buffer = itoa::Buffer::new();
    let mut pct_decimal: Decimal;
    let mut final_pct_decimal: Decimal;
    let mut pct_string: String;
    let mut pct_scale;
    let mut current_scale;
    let abs_dec_places = args.flag_pct_dec_places.unsigned_abs() as u32;
    let mut row;

    wtr.write_record(vec!["field", "value", "count", "percentage"])?;
    let head_ftables = headers.iter().zip(tables);
    for (i, (header, ftab)) in head_ftables.enumerate() {
        header_vec = if rconfig.no_headers {
            (i + 1).to_string().into_bytes()
        } else {
            header.to_vec()
        };

        let mut sorted_counts: Vec<(Vec<u8>, u64, f64)> = args.counts(&ftab);

        // if not --other_sorted and the first value is "Other (", rotate it to the end
        if !args.flag_other_sorted
            && sorted_counts.first().is_some_and(|(value, _, _)| {
                value.starts_with(format!("{} (", args.flag_other_text).as_bytes())
            })
        {
            sorted_counts.rotate_left(1);
        }

        for (value, count, percentage) in sorted_counts {
            pct_decimal = Decimal::from_f64(percentage).unwrap_or_default();
            pct_scale = if args.flag_pct_dec_places < 0 {
                current_scale = pct_decimal.scale();
                if current_scale > abs_dec_places {
                    current_scale
                } else {
                    abs_dec_places
                }
            } else {
                abs_dec_places
            };
            final_pct_decimal = pct_decimal
                .round_dp_with_strategy(
                    pct_scale,
                    rust_decimal::RoundingStrategy::MidpointAwayFromZero,
                )
                .normalize();
            pct_string = if final_pct_decimal.fract().to_string().len() > abs_dec_places as usize {
                final_pct_decimal
                    .round_dp_with_strategy(abs_dec_places, RoundingStrategy::MidpointAwayFromZero)
                    .normalize()
                    .to_string()
            } else {
                final_pct_decimal.to_string()
            };
            row = vec![
                &*header_vec,
                &*value,
                buffer.format(count).as_bytes(),
                pct_string.as_bytes(),
            ];
            wtr.write_record(row)?;
        }
    }
    Ok(wtr.flush()?)
}

type Headers = csv::ByteRecord;
type FTable = Frequencies<Vec<u8>>;
type FTables = Vec<Frequencies<Vec<u8>>>;

impl Args {
    pub fn rconfig(&self) -> Config {
        Config::new(&self.arg_input)
            .delimiter(self.flag_delimiter)
            .no_headers(self.flag_no_headers)
            .select(self.flag_select.clone())
    }

    #[inline]
    fn counts(&self, ftab: &FTable) -> Vec<(ByteString, u64, f64)> {
        let (mut counts, total_count) = if self.flag_asc {
            // parallel sort in ascending order - least frequent values first
            ftab.par_frequent(true)
        } else {
            // parallel sort in descending order - most frequent values first
            ftab.par_frequent(false)
        };

        // check if we need to apply limits
        let unique_counts_len = counts.len();
        if self.flag_lmt_threshold == 0 || self.flag_lmt_threshold >= unique_counts_len {
            // check if the column has all unique values
            // by checking if counts length is equal to ftable length
            let abs_limit = self.flag_limit.unsigned_abs();
            let unique_limited = if self.flag_limit > 0
                && self.flag_unq_limit != abs_limit
                && self.flag_unq_limit > 0
                && unique_counts_len == ftab.len()
            {
                counts.truncate(self.flag_unq_limit);
                true
            } else {
                false
            };

            // check if we need to limit the number of values
            if self.flag_limit > 0 {
                counts.truncate(abs_limit);
            } else if self.flag_limit < 0 && !unique_limited {
                // if limit is negative, only return values with an occurrence count >= absolute
                // value of the negative limit. We only do this if we haven't
                // already unique limited the values
                let count_limit = abs_limit as u64;
                counts.retain(|(_, count)| *count >= count_limit);
            }
        }

        let mut pct_sum = 0.0_f64;
        let mut pct = 0.0_f64;
        let mut count_sum = 0_u64;
        let pct_factor = if total_count > 0 {
            100.0_f64 / total_count.to_f64().unwrap_or(1.0_f64)
        } else {
            0.0_f64
        };

        #[allow(clippy::cast_precision_loss)]
        let mut counts_final: Vec<(Vec<u8>, u64, f64)> = counts
            .into_iter()
            .map(|(byte_string, count)| {
                count_sum += count;
                pct = count as f64 * pct_factor;
                pct_sum += pct;
                if *b"" == **byte_string {
                    (NULL_VAL.to_vec(), count, pct)
                } else {
                    (byte_string.to_owned(), count, pct)
                }
            })
            .collect();

        let other_count = total_count - count_sum;
        if other_count > 0 && self.flag_other_text != "<NONE>" {
            let other_unique_count = unique_counts_len - counts_final.len();
            counts_final.push((
                format!(
                    "{} ({})",
                    self.flag_other_text,
                    HumanCount(other_unique_count as u64)
                )
                .as_bytes()
                .to_vec(),
                other_count,
                100.0_f64 - pct_sum,
            ));
        }
        counts_final
    }

    pub fn sequential_ftables(&self) -> CliResult<(Headers, FTables)> {
        let mut rdr = self.rconfig().reader()?;
        let (headers, sel) = self.sel_headers(&mut rdr)?;
        Ok((headers, self.ftables(&sel, rdr.byte_records())))
    }

    pub fn parallel_ftables(
        &self,
        idx: &Indexed<fs::File, fs::File>,
    ) -> CliResult<(Headers, FTables)> {
        let mut rdr = self.rconfig().reader()?;
        let (headers, sel) = self.sel_headers(&mut rdr)?;

        let idx_count = idx.count() as usize;
        if idx_count == 0 {
            return Ok((headers, vec![]));
        }

        let njobs = util::njobs(self.flag_jobs);
        let chunk_size = util::chunk_size(idx_count, njobs);
        let nchunks = util::num_of_chunks(idx_count, chunk_size);

        let pool = ThreadPool::new(njobs);
        let (send, recv) = channel::bounded(0);
        for i in 0..nchunks {
            let (send, args, sel) = (send.clone(), self.clone(), sel.clone());
            pool.execute(move || {
                // safety: we know the file is indexed and seekable
                let mut idx = args.rconfig().indexed().unwrap().unwrap();
                idx.seek((i * chunk_size) as u64).unwrap();
                let it = idx.byte_records().take(chunk_size);
                send.send(args.ftables(&sel, it)).unwrap();
            });
        }
        drop(send);
        Ok((headers, merge_all(recv.iter()).unwrap()))
    }

    #[inline]
    fn ftables<I>(&self, sel: &Selection, it: I) -> FTables
    where
        I: Iterator<Item = csv::Result<csv::ByteRecord>>,
    {
        let null = &b""[..].to_vec();
        let nsel = sel.normal();
        let nsel_len = nsel.len();
        let mut freq_tables: Vec<_> = (0..nsel_len).map(|_| Frequencies::new()).collect();

        #[allow(unused_assignments)]
        // amortize allocations
        let mut field_buffer: Vec<u8> = Vec::with_capacity(nsel_len);
        let mut row_buffer: csv::ByteRecord = csv::ByteRecord::with_capacity(200, nsel_len);

        let flag_no_nulls = self.flag_no_nulls;
        if self.flag_ignore_case {
            // case insensitive when computing frequencies
            let mut buf = String::new();

            if self.flag_no_trim {
                // case-insensitive, don't trim whitespace
                for row in it {
                    // safety: we know the row is not empty
                    row_buffer.clone_from(&row.unwrap());
                    for (i, field) in nsel.select(row_buffer.into_iter()).enumerate() {
                        field_buffer = {
                            if let Ok(s) = simdutf8::basic::from_utf8(field) {
                                util::to_lowercase_into(s, &mut buf);
                                buf.as_bytes().to_vec()
                            } else {
                                field.to_vec()
                            }
                        };

                        // safety: we do get_unchecked_mut on freq_tables
                        // as we know that nsel_len is the same as freq_tables.len()
                        // so we can skip the bounds check
                        if !field_buffer.is_empty() {
                            unsafe {
                                freq_tables.get_unchecked_mut(i).add(field_buffer);
                            }
                        } else if !flag_no_nulls {
                            unsafe {
                                freq_tables.get_unchecked_mut(i).add(null.clone());
                            }
                        }
                    }
                }
            } else {
                // case-insensitive, trim whitespace
                for row in it {
                    // safety: we know the row is not empty
                    row_buffer.clone_from(&row.unwrap());
                    for (i, field) in nsel.select(row_buffer.into_iter()).enumerate() {
                        field_buffer = {
                            if let Ok(s) = simdutf8::basic::from_utf8(field) {
                                util::to_lowercase_into(s.trim(), &mut buf);
                                buf.as_bytes().to_vec()
                            } else {
                                field.to_vec()
                            }
                        };

                        // safety: we do get_unchecked_mut on freq_tables
                        // as we know that nsel_len is the same as freq_tables.len()
                        // so we can skip the bounds check
                        if !field_buffer.is_empty() {
                            unsafe {
                                freq_tables.get_unchecked_mut(i).add(field_buffer);
                            }
                        } else if !flag_no_nulls {
                            unsafe {
                                freq_tables.get_unchecked_mut(i).add(null.clone());
                            }
                        }
                    }
                }
            }
        } else {
            // case sensitive by default when computing frequencies
            for row in it {
                // safety: we know the row is not empty
                row_buffer.clone_from(&row.unwrap());

                if self.flag_no_trim {
                    // case-sensitive, don't trim whitespace
                    for (i, field) in nsel.select(row_buffer.into_iter()).enumerate() {
                        field_buffer = {
                            if let Ok(s) = simdutf8::basic::from_utf8(field) {
                                s.as_bytes().to_vec()
                            } else {
                                field.to_vec()
                            }
                        };

                        // safety: we do get_unchecked_mut on freq_tables for the same reason above
                        if !field_buffer.is_empty() {
                            unsafe {
                                freq_tables.get_unchecked_mut(i).add(field_buffer);
                            }
                        } else if !flag_no_nulls {
                            unsafe {
                                freq_tables.get_unchecked_mut(i).add(null.clone());
                            }
                        }
                    }
                } else {
                    // case-sensitive, trim whitespace
                    for (i, field) in nsel.select(row_buffer.into_iter()).enumerate() {
                        field_buffer = {
                            if let Ok(s) = simdutf8::basic::from_utf8(field) {
                                s.trim().as_bytes().to_vec()
                            } else {
                                field.to_vec()
                            }
                        };

                        // safety: we do get_unchecked_mut on freq_tables for the same reason above
                        if !field_buffer.is_empty() {
                            unsafe {
                                freq_tables.get_unchecked_mut(i).add(field_buffer);
                            }
                        } else if !flag_no_nulls {
                            unsafe {
                                freq_tables.get_unchecked_mut(i).add(null.clone());
                            }
                        }
                    }
                }
            }
        }
        freq_tables
    }

    fn sel_headers<R: io::Read>(
        &self,
        rdr: &mut csv::Reader<R>,
    ) -> CliResult<(csv::ByteRecord, Selection)> {
        let headers = rdr.byte_headers()?;
        let sel = self.rconfig().selection(headers)?;
        Ok((sel.select(headers).map(<[u8]>::to_vec).collect(), sel))
    }
}
