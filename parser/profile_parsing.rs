use std::fs;

use crate::parser::helpers::*;

#[derive(Debug)]
struct Header {
    // Line: "File: pool.test"
    file_name: String,
    // Line: "Type: profile_type"
    profile_type: String,
    // Line: "Time: 2025-06-21 08:00:24 PDT"
    time_stamp: String,

    parallelism: Parallelism,
    total_nodes: TotalNodes,
}

// Profiling info line: "Duration: 2.01s, Total samples = 10.90s (542.34%)"
//
// - `duration`: wall-clock time profiled (2.01 seconds).
// - `total_samples_time`: total CPU time sampled across all threads (10.90 seconds).
// - `total_samples_percentage`: CPU usage as a percentage of wall time (542.34%),
//   indicating ~5.4 CPU cores used concurrently.
#[derive(Debug)]
struct Parallelism {
    duration: String,
    total_samples_time: String,
    total_samples_percentage: String,
}

// Profiling info line: "Showing nodes accounting for 10.90s, 100% of 10.90s total"
//
// This means the profiling report displays function call nodes whose cumulative CPU time
// sums to 10.90 seconds, which accounts for 100% of the total sampled CPU time (10.90s).
// In other words, all CPU time collected during profiling is represented by these nodes,
// indicating a complete profile without omitted samples.
#[derive(Debug)]
struct TotalNodes {
    collected_nodes_accounting_time: String,
    collected_nodes_accounting_percentage: String,
    total_nodes_accounting_time: String,
}

struct ProfileLabels {}

struct FunctionProfileData {
    function_name: String,
    flat_time: String,
    flat_percentage: String,
    sum_percentage: String,
    cum_time: String,
    cum_percentage: String,
}

// ["File: pool.test", "Type: cpu", "Time: 2025-06-21 08:00:24 PDT",
//  "Duration: 2.01s, Total samples = 10.90s (542.34%)",
//   "Showing nodes accounting for 10.90s, 100% of 10.90s total",
// "      flat  flat%   sum%        cum   cum%"]

pub fn extract_profile_data(profile_file_path: &str) {
    let profile_data = fs::read_to_string(profile_file_path).expect("Failed to read profile file");
    let profile_data_lines = profile_data.split("\n").collect::<Vec<&str>>();

    let (header, header_size) = get_header(&profile_data_lines);

    let (file_name, profile_type, profile_timespamp) = get_header_basic_fields(&header);
    let (duration, total_samples_time, total_samples_percentage) = get_header_parallelism_info(&header);

    let (collected_nodes_accounting_time, collected_nodes_accounting_percentage, total_nodes_accounting_time) = get_header_total_nodes_info(&header);

    let header_struct = Header {
        file_name: file_name.to_string(),
        profile_type: profile_type.to_string(),
        time_stamp: profile_timespamp.to_string(),
        parallelism: Parallelism {
            duration: duration.to_string(),
            total_samples_time: total_samples_time.to_string(),
            total_samples_percentage: total_samples_percentage.to_string(),
        },
        total_nodes: TotalNodes {
            collected_nodes_accounting_time: collected_nodes_accounting_time.to_string(),
            collected_nodes_accounting_percentage: collected_nodes_accounting_percentage.to_string(),
            total_nodes_accounting_time: total_nodes_accounting_time.to_string(),
        },
    };
}
