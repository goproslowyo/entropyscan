use std::path::PathBuf;

/// Holds info about a given target file.
#[derive(Clone, Debug)]
pub struct FileEntropy {
    pub path: PathBuf,
    pub entropy: f64,
}
