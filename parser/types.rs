use std::{error::Error, fmt};

#[derive(Debug)]
pub enum ProfileParsingError {
    FileReadError(std::io::Error),
    EmptyFile,
    InvalidFormat(String),
}

impl fmt::Display for ProfileParsingError {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            ProfileParsingError::FileReadError(err) => write!(f, "Failed to read profile file: {}", err),
            ProfileParsingError::EmptyFile => write!(f, "Profile file is empty"),
            ProfileParsingError::InvalidFormat(msg) => write!(f, "Invalid profile format: {}", msg),
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

    // TODO: Consider using an enum for profile_type instead of String
    // This would provide type safety and prevent invalid profile types
    profile_type: String,

    // TODO: Consider using chrono::DateTime<Utc> instead of String for better type safety
    // and built-in parsing/formatting capabilities
    time_stamp: String,

    // TODO: These fields could be public if they need to be accessed directly
    parallelism: Parallelism,
    total_nodes: TotalNodes,
}

impl Header {
    pub fn new(
        file_name: String,
        profile_type: String,
        time_stamp: String,
        parallelism: Parallelism,
        total_nodes: TotalNodes,
    ) -> Self {
        Self {
            file_name,
            profile_type,
            time_stamp,
            parallelism,
            total_nodes,
        }
    }
}

// TODO: Consider making this struct public if it needs to be accessed outside
// TODO: Add #[derive(Clone, Copy)] if the fields are small and frequently copied
// Profiling info line: "Duration: 2.01s, Total samples = 10.90s (542.34%)"
//
// - `duration`: wall-clock time profiled (2.01 seconds).
// - `total_samples_time`: total CPU time sampled across all threads (10.90 seconds).
// - `total_samples_percentage`: CPU usage as a percentage of wall time (542.34%),
//   indicating ~5.4 CPU cores used concurrently.
#[derive(Debug)]
pub struct Parallelism {
    // TODO: Consider using f64 or Duration for better type safety and calculations
    duration: String,
    total_samples_time: String,
    total_samples_percentage: String,
}

impl Parallelism {
    pub fn new(
        duration: String,
        total_samples_time: String,
        total_samples_percentage: String,
    ) -> Self {
        Self {
            duration,
            total_samples_time,
            total_samples_percentage,
        }
    }
}

// TODO: Consider making this struct public if it needs to be accessed outside
// Profiling info line: "Showing nodes accounting for 10.90s, 100% of 10.90s total"
//
// This means the profiling report displays function call nodes whose cumulative CPU time
// sums to 10.90 seconds, which accounts for 100% of the total sampled CPU time (10.90s).
// In other words, all CPU time collected during profiling is represented by these nodes,
// indicating a complete profile without omitted samples.
#[derive(Debug)]
pub struct TotalNodes {
    // TODO: Consider using f64 for numerical values instead of String
    // This would enable mathematical operations and better type safety
    collected_nodes_accounting_time: String,
    collected_nodes_accounting_percentage: String,
    total_nodes_accounting_time: String,
}

impl TotalNodes {
    pub fn new(
        collected_nodes_accounting_time: String,
        collected_nodes_accounting_percentage: String,
        total_nodes_accounting_time: String,
    ) -> Self {
        Self {
            collected_nodes_accounting_time,
            collected_nodes_accounting_percentage,
            total_nodes_accounting_time,
        }
    }
}

// TODO: Consider making fields public or adding getter methods if they need to be accessed
// TODO: Add #[derive(Clone)] if this struct needs to be cloned
// TODO: Consider using f64 for numerical fields instead of String for better type safety
#[derive(Debug)]
pub struct FunctionProfileData {
    function_name: String,
    flat_time: String,
    flat_percentage: String,
    sum_percentage: String,
    cum_time: String,
    cum_percentage: String,
}

impl FunctionProfileData {
    pub fn new(
        function_name: String,
        flat_time: String,
        flat_percentage: String,
        sum_percentage: String,
        cum_time: String,
        cum_percentage: String,
    ) -> Self {
        Self {
            function_name,
            flat_time,
            flat_percentage,
            sum_percentage,
            cum_time,
            cum_percentage,
        }
    }
}
