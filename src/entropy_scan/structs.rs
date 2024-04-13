use std::borrow::Cow;
use std::path::PathBuf;
use tabled::Tabled;

// use crate::entropy_scan::stats::Iqr;

/// Holds info about a given target file.
#[derive(Clone, Debug)]
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
#[derive(Debug, Clone)]
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
