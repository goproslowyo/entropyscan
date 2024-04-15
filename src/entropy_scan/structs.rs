//! Contains the structs used in the entropy scan module.
//!
//! The `FileEntropy` struct holds the path to a file and its entropy.
//!
//! The `Stats` struct holds the stats for a given target.
//!
//! Both structs implement the `Tabled` and `Serialize` traits to be able to print them in a table and JSON format, respectively.
use std::borrow::Cow;
use std::path::PathBuf;

use serde::Serialize;
use tabled::Tabled;

/// Holds info about a given target file.
///
/// The `path` field holds the path to the file.
///
/// The `entropy` field holds the entropy of the file.
///
/// The `FileEntropy` struct implements the `Tabled` trait to be able to print it in a table format.
///
/// The `FileEntropy` struct also implements the `Serialize` trait to be able to print it in JSON format.
///
#[derive(Clone, Debug, Serialize)]
pub struct FileEntropy {
    pub path: PathBuf,
    pub entropy: f64,
}

impl Tabled for FileEntropy {
    const LENGTH: usize = 2;

    fn headers() -> Vec<Cow<'static, str>> {
        vec![Cow::from("PATH"), Cow::from("ENTROPY")]
    }
    fn fields(&self) -> Vec<Cow<'_, str>> {
        vec![Cow::from(self.path.to_str().unwrap()), Cow::from(format!("{:.3}", self.entropy))]
    }
}

/// Holds the stats for a given target.
///
/// The `total` field holds the total number of files scanned.
///
/// The `mean` field holds the mean entropy of the files.
///
/// The `median` field holds the median entropy of the files.
///
/// The `variance` field holds the variance of the files.
///
/// The `iqr` field holds the interquartile range of the files.
///
/// The `Stats` struct implements the `Tabled` trait to be able to print it in a table format.
///
/// The `Stats` struct also implements the `Serialize` trait to be able to print it in JSON format.
///
#[derive(Debug, Clone, Serialize)]
pub struct Stats {
    pub target: PathBuf,
    pub total: usize,
    pub mean: f64,
    pub median: f64,
    pub variance: f64,
    pub iqr: f64,
}

impl Tabled for Stats {
    const LENGTH: usize = 6;

    fn headers() -> Vec<Cow<'static, str>> {
        vec![
            Cow::from("TARGET"),
            Cow::from("TOTAL"),
            Cow::from("MEAN"),
            Cow::from("MEDIAN"),
            Cow::from("VARIANCE"),
            Cow::from("IQR")
        ]
    }

    fn fields(&self) -> Vec<Cow<'_, str>> {
        vec![
            Cow::from(self.target.to_str().unwrap()),
            Cow::from(self.total.to_string()),
            Cow::from(format!("{:.3}", self.mean)),
            Cow::from(format!("{:.3}", self.median)),
            Cow::from(format!("{:.3}", self.variance)),
            Cow::from(format!("{:.3}", self.iqr))
        ]
    }
}
