use clap::{ Parser, Subcommand };
use std::path::PathBuf;
use tabled::Table;

mod entropy_scan;
use entropy_scan::{
    collect_entropies,
    collect_targets,
    structs::FileEntropy,
    stats::{ mean, median, variance, interquartile_range },
};

use crate::entropy_scan::stats::entropy_outliers;

/// CLI struct.
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

/// Subcommand enum.
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
            let entropies: Vec<FileEntropy> = collect_entropies(targets)
                .into_iter()
                .filter(|e| e.entropy >= min_entropy)
                .collect();
            let table = Table::new(entropies).to_string();
            print!("{table}");
            Ok(())
        }
        Stats { target, no_outliers } => {
            let targets = collect_targets(target.clone());
            let entropies = collect_entropies(targets.clone());

            let stats = entropy_scan::structs::Stats {
                target,
                total: targets.len(),
                mean: mean(entropies.clone()).unwrap(),
                median: median(entropies.clone()).unwrap(),
                variance: variance(entropies.clone()).unwrap(),
                iqr: interquartile_range(entropies.clone()).unwrap().range,
            };

            let table = Table::new(vec![stats]);
            println!("{table}");

            if !no_outliers && targets.len() > 1 {
                if let Some(outliers) = entropy_outliers(entropies.clone()) {
                    println!("\n--------Outliers--------");
                    let outliers_table = Table::new(outliers).to_string();
                    println!("{outliers_table}");
                }
            }
            Ok(())
        }
    }
}
