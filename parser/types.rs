use std::{error::Error, fmt};

#[derive(Debug)]
pub enum ProfileParsingError {
    FileReadError(std::io::Error),
    EmptyFile,
    InvalidFormat(String),
    IncompleteHeader(String),
}

impl fmt::Display for ProfileParsingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProfileParsingError::FileReadError(err) => write!(f, "Failed to read profile file: {}", err),
            ProfileParsingError::EmptyFile => write!(f, "Profile file is empty"),
            ProfileParsingError::InvalidFormat(msg) => write!(f, "Invalid profile format: {}", msg),
            ProfileParsingError::IncompleteHeader(msg) => write!(f, "Incomplete header: {}", msg),
        }
    }
}

impl Error for ProfileParsingError {}

impl From<std::io::Error> for ProfileParsingError {
    fn from(err: std::io::Error) -> Self {
        ProfileParsingError::FileReadError(err)
    }
}

#[derive(Debug)]
pub struct Header {
    pub(crate) file_name: String,

    profile_type: String,

    parallelism: Parallelism,
    total_nodes: TotalNodes,
}

impl Header {
    pub fn new(file_name: String, profile_type: String, parallelism: Parallelism, total_nodes: TotalNodes) -> Self {
        Self { file_name, profile_type, parallelism, total_nodes }
    }
}

// Profiling info line: "Duration: 2.01s, Total samples = 10.90s (542.34%)"
//
// - `duration`: wall-clock time profiled (2.01 seconds).
// - `total_samples_time`: total CPU time sampled across all threads (10.90 seconds).
// - `total_samples_percentage`: CPU usage as a percentage of wall time (542.34%),
//   indicating ~5.4 CPU cores used concurrently.
#[derive(Debug)]
pub struct Parallelism {
    duration: f64,
    total_samples_time: f64,
    total_samples_percentage: f64,
}

impl Parallelism {
    pub fn new(duration: f64, total_samples_time: f64, total_samples_percentage: f64) -> Self {
        Self { duration, total_samples_time, total_samples_percentage }
    }
}

// Profiling info line: "Showing nodes accounting for 10.90s, 100% of 10.90s total"
//
// This means the profiling report displays function call nodes whose cumulative CPU time
// sums to 10.90 seconds, which accounts for 100% of the total sampled CPU time (10.90s).
// In other words, all CPU time collected during profiling is represented by these nodes,
// indicating a complete profile without omitted samples.
#[derive(Debug)]
pub struct TotalNodes {
    collected_nodes_accounting: f64,
    collected_nodes_accounting_percentage: f64,
    total_nodes_accounting: f64,
}

impl TotalNodes {
    pub fn new(collected_nodes_accounting: f64, collected_nodes_accounting_percentage: f64, total_nodes_accounting: f64) -> Self {
        Self {
            collected_nodes_accounting,
            collected_nodes_accounting_percentage,
            total_nodes_accounting,
        }
    }
}

#[derive(Debug)]
pub struct FunctionProfileData {
    function_name: String,
    flat: f64,
    flat_percentage: f64,
    sum_percentage: f64,
    cum: f64,
    cum_percentage: f64,
}

impl FunctionProfileData {
    pub fn new(function_name: String, flat: f64, flat_percentage: f64, sum_percentage: f64, cum: f64, cum_percentage: f64) -> Self {
        Self {
            function_name,
            flat,
            flat_percentage,
            sum_percentage,
            cum,
            cum_percentage,
        }
    }
}
