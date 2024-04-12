use clap::Parser;
use std::path::PathBuf;

mod entropy_scan;
use entropy_scan::{ calculate_entropy, collect_targets, structs::FileEntropy };

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "TARGET", help = "Target file or path to scan")]
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

/// Main function.
fn main() -> Result<(), String> {
    let args = Cli::parse();
    let parent_path_buf = args.target;
    let min_entropy = args.min_entropy.unwrap();

    let targets = collect_targets(parent_path_buf);
    for target in targets {
        let fe_struct: FileEntropy = calculate_entropy(&target).unwrap();
        if fe_struct.entropy >= min_entropy {
            println!("{:?}: {:.3}", fe_struct.path, fe_struct.entropy);
        }
    }
    Ok(())
}
