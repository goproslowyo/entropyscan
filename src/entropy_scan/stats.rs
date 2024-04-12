use crate::FileEntropy;

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
