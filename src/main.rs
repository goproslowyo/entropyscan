use clap::Parser;
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about=None)]
struct Cli {
    #[arg(
        short,
        long,
        value_name = "TARGET",
        help = "Target file or path to scan"
    )]
    /// The target file or path to scan.
    target: PathBuf,

    #[arg(
        short,
        long,
        value_name = "MIN_ENTROPY",
        help = "Minimum entropy to display",
        default_value = "0.0"
    )]
    /// The minimum entropy to display.
    min_entropy: Option<f64>,
}

/// The maximum file size we can scan.
const MAX_FILE_SIZE: u64 = 2147483648;

/// The chunk size for our files.
const MAX_ENTROPY_CHUNK: usize = 2560000;

/// Calculate a file's entropy.
/// Accepts a [str].
fn calculate_entropy(filename: &PathBuf) -> Result<f64, String> {
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
                    let p = *count as f64 / total_bytes as f64;
                    entropy -= p * p.log2();
                }
            }
            Ok(entropy)
        } else {
            Err("Couldn't read file!".to_string())
        }
    } else {
        Err("Couldn't read file metadata!".to_string())
    }
}

/// Collect all files in a directory.
/// Accepts a [PathBuf].
fn collect_targets(parent_path: PathBuf) -> Vec<PathBuf> {
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

/// Main function.
fn main() -> Result<(), String> {
    let args = Cli::parse();
    let parent_path_buf = args.target;
    let min_entropy = args.min_entropy.unwrap();

    let targets = collect_targets(parent_path_buf);
    for target in targets {
        let entropy = calculate_entropy(&target).unwrap();
        if entropy >= min_entropy {
            println!("Scanned {target:?}: {entropy:.3}");
        }
    }

    // Fancier way to do this
    // let entropies: Vec<(&PathBuf, f64)> = targets
    //     .iter()
    //     .map(|t| (t, calculate_entropy(t).unwrap()))
    //     .filter(|(_, e)| e >= &min_entropy)
    //     .collect();
    // for (p, e) in entropies {
    //     println!("Scanned {p:?}: {e:.3}");
    // }
    Ok(())
}
