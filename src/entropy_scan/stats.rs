use crate::FileEntropy;

/// Holds the [interquartile range](https://en.wikipedia.org/wiki/Interquartile_range) of a [Vec] of [FileEntropy] structs.
#[derive(Debug)]
pub struct Iqr {
    q1: f64,
    q3: f64,
    range: f64,
}

/// Calculate the mean of a [Vec] of [FileEntropy] structs.
pub fn mean(data: Vec<FileEntropy>) -> Option<f64> {
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

/// Sort a [Vec] of [FileEntropy] structs by entropy.
fn sort_entropies(data: Vec<FileEntropy>) -> Vec<FileEntropy> {
    let mut sorted_data = data.clone();
    sorted_data.sort_by(|a, b| a.entropy.partial_cmp(&b.entropy).unwrap());
    sorted_data
}

/// Calculate the median of a [Vec] of [FileEntropy] structs.
pub fn median(data: Vec<FileEntropy>) -> Option<f64> {
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
pub fn variance(data: Vec<FileEntropy>) -> Option<f64> {
    match data.is_empty() {
        true => None,
        false => {
            let mean = mean(data.clone()).unwrap();
            let sum: f64 = data
                .iter()
                .map(|e| (e.entropy - mean).powi(2))
                .sum();
            let variance = sum / (data.len() as f64);
            Some(variance)
        }
    }
}

/// Calculate the [interquartile range](https://en.wikipedia.org/wiki/Interquartile_range) of a [Vec] of [FileEntropy] structs.
pub fn interquartile_range(data: Vec<FileEntropy>) -> Option<Iqr> {
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

/// Calculate the outliers based on the [IQR](interquartile_range) of a [Vec] of [FileEntropy] structs.
pub fn entropy_outliers(data: Vec<FileEntropy>) -> Option<Vec<FileEntropy>> {
    match data.is_empty() {
        true => None,
        false => {
            let iqr = interquartile_range(data.clone()).unwrap();

            let outliers = data
                .into_iter()
                .filter(
                    |e|
                        e.entropy < iqr.q1 - 1.5 * iqr.range || e.entropy > iqr.q3 + 1.5 * iqr.range
                )
                .collect();
            Some(outliers)
        }
    }
}

// /// Filter a [Vec] of [FileEntropy] structs by entropy.
// pub fn filter_entropies(data: Vec<FileEntropy>, min_entropy: f64) -> Vec<FileEntropy> {
//     data.into_iter()
//         .filter(|e| e.entropy >= min_entropy)
//         .collect()
// }
