use clap::{ Parser, Subcommand };
use std::path::PathBuf;

mod entropy_scan;
use entropy_scan::{
    calculate_entropy,
    collect_entropies,
    collect_targets,
    structs::FileEntropy,
    stats::{ mean, median, variance, interquartile_range },
};

use crate::entropy_scan::stats::entropy_outliers;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Scan {
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
    },
    Stats {
        #[arg(short, long, value_name = "TARGET", help = "Target file or path to scan")]
        /// The target file or path to scan.
        target: PathBuf,

        #[arg(short, help = "Do not print outliers")]
        no_outliers: bool,
    },
}

/// Main function.
fn main() -> Result<(), String> {
    let args = Cli::parse();

    use Command::*;
    match args.command {
        Scan { target, min_entropy } => {
            let parent_path_buf = target;
            let min_entropy = min_entropy.unwrap();

            let targets = collect_targets(parent_path_buf);
            for target in targets {
                let fe_struct: FileEntropy = calculate_entropy(&target).unwrap();
                if fe_struct.entropy >= min_entropy {
                    println!("{:?}: {:.3}", fe_struct.path, fe_struct.entropy);
                }
            }
            Ok(())
        }
        Stats { target, no_outliers } => {
            // let parent_path_buf = target;
            let targets = collect_targets(target.clone());
            let entropies = collect_entropies(targets.clone());

            println!("Statistics for {}", target.to_str().unwrap());
            println!("Total items scanned: {}", targets.len());
            println!("Mean entropy: {:.3}", mean(entropies.clone()).unwrap());
            println!("Median entropy: {:.3}", median(entropies.clone()).unwrap());
            println!("Variance: {:.3}", variance(entropies.clone()).unwrap());
            println!("IQR: {:?}", interquartile_range(entropies.clone()).unwrap());

            if !no_outliers {
                if let Some(outliers) = entropy_outliers(entropies.clone()) {
                    println!("Outliers\n--------");
                    for outlier in outliers {
                        println!("{}:\t{:.3}", outlier.path.to_str().unwrap(), outlier.entropy);
                    }
                }
            }
            Ok(())
        }
    }
}
