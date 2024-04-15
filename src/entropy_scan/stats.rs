//! Contains functions to calculate statistics on a [Vec] of [FileEntropy] structs.
//!
//! The [mean], [median], [variance], [interquartile_range], and [entropy_outliers] functions are used to calculate the statistics of a [Vec] of [FileEntropy] structs, respectively.
//!
//! The [FileEntropy] struct holds the path to a file and its entropy.
//!
//! The [Iqr] struct holds the interquartile range of a [Vec] of [FileEntropy] structs.
//!
//! The [sort_entropies] function is used to sort a [Vec] of [FileEntropy] structs by entropy.
use crate::FileEntropy;

/// Holds the [interquartile range](https://en.wikipedia.org/wiki/Interquartile_range) of a [Vec] of [FileEntropy] structs.
///
/// The q1 field is the first quartile (Q1).
///
/// The q3 field is the third quartile (Q3).
///
/// The range field is the difference between the third quartile (Q3) and the first quartile (Q1).
#[derive(Debug)]
pub struct Iqr {
    q1: f64,
    q3: f64,
    pub range: f64,
}

/// Calculate the [interquartile range](https://en.wikipedia.org/wiki/Interquartile_range) of a [Vec] of [FileEntropy] structs.
///
/// Returns the [Iqr] struct if the [Vec] is not empty. Returns [None] if the [Vec] is empty.
pub fn interquartile_range(data: &[FileEntropy]) -> Option<Iqr> {
    match data.is_empty() {
        true => None,
        false => {
            if data.len() == 1 {
                return Some(Iqr {
                    q1: data[0].entropy,
                    q3: data[0].entropy,
                    range: 0.0,
                });
            }

            let sorted_data = sort_entropies(data);
            let len = sorted_data.len();

            let q1_idx = match len % 2 {
                0 => len / 4,
                _ => (len + 1) / 4,
            };
            let q3_idx = 3 * q1_idx;

            let q1 = sorted_data[q1_idx - 1].entropy;
            let q3 = sorted_data[q3_idx - 1].entropy;
            Some(Iqr {
                q1,
                q3,
                range: q3 - q1,
            })
        }
    }
}

/// Calculate the mean of a [Vec] of [FileEntropy] structs.
///
/// Returns the mean as a [f64] if the [Vec] is not empty. Returns [None] if the [Vec] is empty.
pub fn mean(data: &[FileEntropy]) -> Option<f64> {
    match data.is_empty() {
        true => None,
        false => {
            let sum: f64 = data
                .iter()
                .map(|e| e.entropy)
                .sum();
            Some(sum / (data.len() as f64))
        }
    }
}

/// Calculate the median of a [Vec] of [FileEntropy] structs.
///
/// Returns the median as a [f64] if the [Vec] is not empty. Returns [None] if the [Vec] is empty.
pub fn median(data: &[FileEntropy]) -> Option<f64> {
    match data.is_empty() {
        true => None,
        false => {
            let sorted_data = sort_entropies(data);
            let len = sorted_data.len();
            let mid = len / 2;
            if len % 2 == 0 {
                let a = sorted_data[mid - 1].entropy;
                let b = sorted_data[mid].entropy;
                Some((a + b) / 2.0)
            } else {
                Some(sorted_data[mid].entropy)
            }
        }
    }
}

/// Calculate the [variance](https://en.wikipedia.org/wiki/Variance) of a [Vec] of [FileEntropy] structs.
///
/// Returns the variance as a [f64] if the [Vec] is not empty. Returns [None] if the [Vec] is empty.
pub fn variance(data: &[FileEntropy]) -> Option<f64> {
    match data.is_empty() {
        true => None,
        false => {
            let mean = mean(data).unwrap();
            let sum: f64 = data
                .iter()
                .map(|e| (e.entropy - mean).powi(2))
                .sum();
            let variance = sum / (data.len() as f64);
            Some(variance)
        }
    }
}

/// Calculate the outliers based on the [IQR](interquartile_range) of a [Vec] of [FileEntropy] structs.
///
/// Returns a [Vec] of [FileEntropy] structs if the [Vec] is not empty. Returns [None] if the [Vec] is empty.
pub fn entropy_outliers(data: &[FileEntropy]) -> Option<Vec<FileEntropy>> {
    match data.is_empty() {
        true => None,
        false => {
            let iqr = interquartile_range(data).unwrap();

            let outliers = data
                .iter()
                .filter(
                    |e|
                        e.entropy < iqr.q1 - 1.5 * iqr.range || e.entropy > iqr.q3 + 1.5 * iqr.range
                )
                .map(|e| e.to_owned())
                .collect();
            Some(outliers)
        }
    }
}

/// Sort a [Vec] of [FileEntropy] structs by entropy.
///
/// Returns a sorted [Vec] of [FileEntropy] structs.
fn sort_entropies(data: &[FileEntropy]) -> Vec<FileEntropy> {
    let mut sorted_data = data.to_vec();
    sorted_data.sort_by(|a, b| a.entropy.partial_cmp(&b.entropy).unwrap());
    sorted_data
}
