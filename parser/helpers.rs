#[derive(Debug)]
pub struct Header {
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

pub fn get_header(profile_data_lines: &[&str]) -> (Vec<String>, usize) {
    let profile_type_line = profile_data_lines.get(1).expect("No second element found");
    let profile_type = profile_type_line.strip_prefix("Type: ").expect("Invalid profile type format");

    let header_size = if profile_type == "cpu" { 6 } else { 5 };
    let header = profile_data_lines
        .get(0..header_size)
        .expect("No header found")
        .iter()
        .map(|s| s.to_string())
        .collect();

    return (header, header_size);
}

pub fn get_header_basic_fields(header: &[String]) -> (String, String, String) {
    let file_name_line = header.get(0).expect("No file name found");
    let file_name = file_name_line.strip_prefix("File: ").expect("Invalid file line format").trim();

    let profile_type_line = header.get(1).expect("No profile type found");
    let profile_type = profile_type_line.strip_prefix("Type: ").expect("Invalid profile type format").trim();

    let profile_timespamp_line = header.get(2).expect("No time stamp found");
    let profile_timespamp = profile_timespamp_line.strip_prefix("Time: ").expect("Invalid time stamp format").trim();

    return (file_name.to_string(), profile_type.to_string(), profile_timespamp.to_string());
}

pub fn get_header_parallelism_info(header: &[String]) -> (String, String, String) {
    let duration_samples_line = header.get(3).expect("No duration found");
    let duration = duration_samples_line
        .strip_prefix("Duration: ")
        .expect("Invalid duration format")
        .split(',')
        .next()
        .expect("Missing duration value")
        .trim();

    let total_samples_line = duration_samples_line.split('=').nth(1).expect("No '=' found").trim();

    let total_samples_time = total_samples_line.split('(').next().expect("Missing opening parenthesis").trim();

    let total_samples_percentage = total_samples_line
        .split('(')
        .nth(1)
        .expect("No opening parenthesis found")
        .trim_end_matches(')')
        .trim_end_matches('%')
        .trim();

    return (duration.to_string(), total_samples_time.to_string(), total_samples_percentage.to_string());
}

pub fn get_header_total_nodes_info(header: &[String]) -> (String, String, String) {
    let total_nodes_line = header.get(4).expect("No total nodes found");
    let collected_nodes_accounting_time_line = total_nodes_line
        .strip_prefix("Showing nodes accounting for ")
        .expect("Invalid total nodes format");

    let collected_nodes_accounting_time = collected_nodes_accounting_time_line.split(',').next().expect("Missing comma").trim();

    let collected_nodes_accounting_percentage = collected_nodes_accounting_time_line
        .split(',')
        .nth(1)
        .expect("Missing percentage part")
        .split('%')
        .next()
        .expect("Missing % sign")
        .trim();

    let total_nodes_accounting_time = collected_nodes_accounting_time_line.split("of").nth(1).expect("Missing 'of' part").trim();

    return (
        collected_nodes_accounting_time.to_string(),
        collected_nodes_accounting_percentage.to_string(),
        total_nodes_accounting_time.to_string(),
    );
}

pub fn build_header(profile_data_lines: &[&str]) -> (Header, usize) {
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

    return (header_struct, header_size);
}

#[derive(Debug)]
pub struct FunctionProfileData {
    function_name: String,
    flat_time: String,
    flat_percentage: String,
    sum_percentage: String,
    cum_time: String,
    cum_percentage: String,
}

pub fn collect_function_profile_data(line_parts: &[&str]) -> Option<FunctionProfileData> {
    if line_parts.len() < 6 {
        return None;
    }

    let function_name = line_parts[5..].join(" ");
    let flat_time = line_parts[0].trim_end_matches('s').to_string();
    let flat_percentage = line_parts[1].trim_end_matches('%').to_string();
    let sum_percentage = line_parts[2].trim_end_matches('%').to_string();
    let cum_time = line_parts[3].trim_end_matches('s').to_string();
    let cum_percentage = line_parts[4].trim_end_matches('%').to_string();

    return Some(FunctionProfileData {
        function_name,
        flat_time,
        flat_percentage,
        sum_percentage,
        cum_time,
        cum_percentage,
    });
}
