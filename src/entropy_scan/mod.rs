//! This module contains the logic for scanning files for entropy.
//!
//! The main functions are: [calculate_entropy], [collect_entropies], and [collect_targets].
//!
//! [calculate_entropy] takes a [PathBuf] and returns a [FileEntropy].
//!
//! [collect_entropies] takes a [Vec] of [PathBuf]s and returns a [Vec] of [FileEntropy]s.
//!
//! [collect_targets] takes a [PathBuf] and returns a [Vec] of [PathBuf]s.
use std::fs;
use std::path::PathBuf;

pub mod stats;
pub mod structs;
use structs::FileEntropy;

/// The maximum file size we can scan.
///
/// This is set to 2GB.
const MAX_FILE_SIZE: u64 = 2147483648;

/// The chunk size for our files.
///
/// This is set to 2.5MB.
const MAX_ENTROPY_CHUNK: usize = 2560000;

/// Calculate a file's entropy.
///
/// Takes a [PathBuf] and returns a [Result] with a [FileEntropy] or an error message.
fn calculate_entropy(filename: &PathBuf) -> Result<FileEntropy, String> {
    if let Ok(metadata) = fs::metadata(filename) {
        // Check max size
        if metadata.len() > MAX_FILE_SIZE {
            return Err("File too large".to_string());
        }
        // Check whether it's a directory
        if metadata.is_dir() {
            return Err("Is a directory".to_string());
        }

        if let Ok(file_bytes) = fs::read(filename) {
            let mut entropy = 0.0f64;
            for chunk in file_bytes.chunks(MAX_ENTROPY_CHUNK) {
                let mut frequency: [u32; 256] = [0; 256];
                let mut total_bytes = 0;

                for byte in chunk {
                    frequency[*byte as usize] += 1;
                    total_bytes += 1;
                }

                for count in frequency.iter() {
                    if *count == 0 {
                        continue;
                    }
                    let p = (*count as f64) / (total_bytes as f64);
                    entropy -= p * p.log2();
                }
            }
            Ok(FileEntropy {
                path: filename.to_owned(),
                entropy,
            })
        } else {
            Err("Couldn't read file!".to_string())
        }
    } else {
        Err("Couldn't read file metadata!".to_string())
    }
}

/// Collect entropies from a [Vec] of [PathBuf]s.
///
/// Takes a [Vec] of [PathBuf]s and returns a [Vec] of [FileEntropy]s.
pub fn collect_entropies(targets: &Vec<PathBuf>) -> Vec<FileEntropy> {
    let mut entropies = Vec::with_capacity(targets.len());

    for target in targets {
        if let Ok(entropy) = calculate_entropy(target) {
            entropies.push(entropy);
        }
    }
    entropies
}

/// Collect all files in a directory.
///
/// Takes a [PathBuf] and returns a [Vec] of [PathBuf]s.
pub fn collect_targets(parent_path: PathBuf) -> Vec<PathBuf> {
    if parent_path.is_file() {
        return vec![parent_path];
    }
    let mut targets = Vec::new();
    let dir = fs::read_dir(parent_path).unwrap();
    for entry in dir {
        let path = entry.unwrap().path();
        if path.is_dir() {
            targets.extend(collect_targets(path));
        } else {
            targets.push(path);
        }
    }
    targets
}
