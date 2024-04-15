//! A command-line utility to scan files and directories for entropy.
//!
//! The utility can scan a file or directory and display the entropy of the files.
//!
//! It can also display the stats for a given target, including the [entropy_scan::stats::mean], [entropy_scan::stats::median], [entropy_scan::stats::variance], and [entropy_scan::stats::interquartile_range].
//!
//! The utility can also display the outliers with the [entropy_scan::stats::entropy_outliers].
use std::path::PathBuf;

use clap::{ Parser, Subcommand, ValueEnum };
use serde_json::json;

mod entropy_scan;
use entropy_scan::{
    collect_entropies,
    collect_targets,
    stats::{ entropy_outliers, interquartile_range, mean, median, variance },
    structs::FileEntropy,
};

/// A [Cli] struct holding a [Command] enum for the subcommands [Command::Scan] and [Command::Stats].
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

/// A custom enum to represent the chosen output format.
///
/// Valid values are [OutputFormat::Csv], [OutputFormat::Json], and [OutputFormat::Table]. Default is [OutputFormat::Table].
#[derive(Clone, ValueEnum)]
enum OutputFormat {
    Csv,
    Json,
    Table,
}

/// A [Subcommand] enum for the [Command::Scan] and [Command::Stats] subcommands.
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

        /// The output format. Valid values are [OutputFormat::Csv], [OutputFormat::Json], and [OutputFormat::Table]. Default is [OutputFormat::Table].
        #[arg(short, long, value_name = "FORMAT", help = "Output format", default_value = "table")]
        format: OutputFormat,
    },
    Stats {
        #[arg(short, long, value_name = "TARGET", help = "Target file or path to scan")]
        /// The target file or path to scan.
        target: PathBuf,

        /// Do not print outliers.
        #[arg(short, help = "Do not print outliers")]
        no_outliers: bool,

        /// The output format. Valid values are [OutputFormat::Csv], [OutputFormat::Json], and [OutputFormat::Table]. Default is [OutputFormat::Table].
        #[arg(short, long, value_name = "FORMAT", help = "Output format", default_value = "table")]
        format: OutputFormat,
    },
}

fn main() -> Result<(), String> {
    use Command::*;
    use OutputFormat::*;

    let args = Cli::parse();

    match args.command {
        Scan { target, min_entropy, format } => {
            let parent_path_buf = target;
            let min_entropy = min_entropy.unwrap();
            let targets = collect_targets(parent_path_buf);
            let entropies: Vec<FileEntropy> = collect_entropies(&targets)
                .into_iter()
                .filter(|e| e.entropy >= min_entropy)
                .collect();

            match format {
                Csv => {
                    println!("-----Entropies-----");
                    println!("path,entropy");
                    for item in entropies {
                        println!("{},{:.3}", item.path.to_string_lossy(), item.entropy);
                    }
                }
                Json => {
                    let json = serde_json::to_string_pretty(&entropies).unwrap();
                    print!("{}", json);
                }
                Table => {
                    println!("-----Entropies-----");
                    let table = tabled::Table::new(entropies).to_string();
                    print!("{table}");
                }
            }

            Ok(())
        }

        Stats { target, no_outliers, format } => {
            let targets = collect_targets(target.clone());
            let entropies = collect_entropies(&targets);
            let stats = entropy_scan::structs::Stats {
                target,
                total: targets.len(),
                mean: mean(&entropies).unwrap(),
                median: median(&entropies).unwrap(),
                variance: variance(&entropies).unwrap(),
                iqr: interquartile_range(&entropies).unwrap().range,
            };

            match format {
                Csv => {
                    println!("-----Stats-----");
                    println!("target,total,mean,median,variance,iqr");
                    println!(
                        "{},{},{:.3},{:.3},{:.3},{:.3}",
                        stats.target.to_string_lossy(),
                        stats.total,
                        stats.mean,
                        stats.median,
                        stats.variance,
                        stats.iqr
                    );
                    match no_outliers {
                        true => (),
                        false => {
                            let outliers = entropy_outliers(&entropies).unwrap();
                            println!("\n-----Outliers-----");
                            println!("path,entropy");
                            for item in outliers {
                                println!("{},{:.3}", item.path.to_string_lossy(), item.entropy);
                            }
                        }
                    }
                }

                Json => {
                    let json = json!(&stats);
                    match no_outliers {
                        true => (),
                        false => {
                            let outliers = entropy_outliers(&entropies).unwrap();
                            let json_string =
                                json![{
                                "stats": &stats,
                                "outliers": &outliers,
                        }];
                            println!("{}", json_string);
                        }
                    }
                    print!("{}", json);
                }

                Table => {
                    println!("-----Entropies-----");
                    let table = tabled::Table::new(vec![stats]);
                    println!("{table}");
                    match no_outliers {
                        true => (),
                        false => {
                            let outliers = entropy_outliers(&entropies).unwrap();
                            println!("\n-----Outliers-----");
                            let table = tabled::Table::new(outliers);
                            println!("{table}");
                        }
                    }
                }
            }

            Ok(())
        }
    }
}
