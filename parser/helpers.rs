use std::{fs, path::PathBuf, process};

use crate::parser::types::{FunctionProfileData, Header, Parallelism, ProfileParsingError, TotalNodes};

pub fn get_header(profile_data_lines: &[&str]) -> (Vec<String>, usize) {
    let profile_type_line = profile_data_lines.get(1).unwrap_or_else(|| {
        eprintln!("Expected profile type on line 2, but it was missing.");
        process::exit(1);
    });

    let profile_type = profile_type_line.strip_prefix("Type: ").unwrap_or_else(|| {
        eprintln!("Invalid profile type format: expected line to start with \"Type: \"");
        process::exit(1);
    });

    let header_size = if profile_type == "cpu" { 6 } else { 5 };

    let header = profile_data_lines
        .get(0..header_size)
        .unwrap_or_else(|| {
            eprintln!(
                "No header found: expected at least {} lines, found {}",
                header_size,
                profile_data_lines.len()
            );
            process::exit(1);
        })
        .iter()
        .map(|s| s.to_string())
        .collect();

    (header, header_size)
}

pub fn get_header_basic_fields(header: &[String]) -> (String, String, String) {
    let file_name_line = header.get(0).unwrap_or_else(|| {
        eprintln!("No file name found");
        process::exit(1);
    });
    let file_name = file_name_line.strip_prefix("File: ").unwrap_or_else(|| {
        eprintln!("Invalid file line format");
        process::exit(1);
    });

    let profile_type_line = header.get(1).unwrap_or_else(|| {
        eprintln!("No profile type found");
        process::exit(1);
    });
    let profile_type = profile_type_line.strip_prefix("Type: ").unwrap_or_else(|| {
        eprintln!("Invalid profile type format");
        process::exit(1);
    });

    let profile_timestamp_line = header.get(2).unwrap_or_else(|| {
        eprintln!("No time stamp found");
        process::exit(1);
    });
    let profile_timestamp = profile_timestamp_line.strip_prefix("Time: ").unwrap_or_else(|| {
        eprintln!("Invalid time stamp format");
        process::exit(1);
    });

    (file_name.to_string(), profile_type.to_string(), profile_timestamp.to_string())
}

pub fn get_header_parallelism_info(header: &[String]) -> (String, String, String) {
    let duration_samples_line = header.get(3).unwrap_or_else(|| {
        eprintln!("No duration found");
        process::exit(1);
    });

    let duration = duration_samples_line
        .strip_prefix("Duration: ")
        .unwrap_or_else(|| {
            eprintln!("Invalid duration format");
            process::exit(1);
        })
        .split(',')
        .next()
        .unwrap_or_else(|| {
            eprintln!("Missing duration value");
            process::exit(1);
        })
        .trim();

    let total_samples_line = duration_samples_line
        .split('=')
        .nth(1)
        .unwrap_or_else(|| {
            eprintln!("No '=' found in duration samples line");
            process::exit(1);
        })
        .trim();

    let total_samples_time = total_samples_line
        .split('(')
        .next()
        .unwrap_or_else(|| {
            eprintln!("Missing opening parenthesis in total samples time");
            process::exit(1);
        })
        .trim();

    let total_samples_percentage = total_samples_line
        .split('(')
        .nth(1)
        .unwrap_or_else(|| {
            eprintln!("No opening parenthesis found in total samples percentage");
            process::exit(1);
        })
        .trim_end_matches(')')
        .trim_end_matches('%')
        .trim();

    (duration.to_string(), total_samples_time.to_string(), total_samples_percentage.to_string())
}

pub fn get_header_total_nodes_info(header: &[String]) -> (String, String, String) {
    let total_nodes_line = header.get(4).unwrap_or_else(|| {
        eprintln!("No total nodes found");
        process::exit(1);
    });

    let collected_nodes_accounting_time_line = total_nodes_line.strip_prefix("Showing nodes accounting for ").unwrap_or_else(|| {
        eprintln!("Invalid total nodes format");
        process::exit(1);
    });

    let collected_nodes_accounting_time = collected_nodes_accounting_time_line
        .split(',')
        .next()
        .unwrap_or_else(|| {
            eprintln!("Missing comma");
            process::exit(1);
        })
        .trim();

    let collected_nodes_accounting_percentage = collected_nodes_accounting_time_line
        .split(',')
        .nth(1)
        .unwrap_or_else(|| {
            eprintln!("Missing percentage part");
            process::exit(1);
        })
        .split('%')
        .next()
        .unwrap_or_else(|| {
            eprintln!("Missing % sign");
            process::exit(1);
        })
        .trim();

    let total_nodes_accounting_time = collected_nodes_accounting_time_line
        .split("of")
        .nth(1)
        .unwrap_or_else(|| {
            eprintln!("Missing 'of' part");
            process::exit(1);
        })
        .trim();

    return (
        collected_nodes_accounting_time.to_string(),
        collected_nodes_accounting_percentage.to_string(),
        total_nodes_accounting_time.to_string(),
    );
}

pub fn build_header(profile_data_lines: &[&str]) -> (Header, usize) {
    let (header, header_size) = get_header(&profile_data_lines);

    let (file_name, profile_type, profile_timestamp) = get_header_basic_fields(&header);
    let (duration, total_samples_time, total_samples_percentage) = get_header_parallelism_info(&header);

    let (collected_nodes_accounting_time, collected_nodes_accounting_percentage, total_nodes_accounting_time) = get_header_total_nodes_info(&header);

    let header_struct = Header::new(
        file_name,
        profile_type,
        profile_timestamp,
        Parallelism::new(duration, total_samples_time, total_samples_percentage),
        TotalNodes::new(
            collected_nodes_accounting_time,
            collected_nodes_accounting_percentage,
            total_nodes_accounting_time,
        ),
    );

    (header_struct, header_size)
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

    Some(FunctionProfileData::new(
        function_name,
        flat_time,
        flat_percentage,
        sum_percentage,
        cum_time,
        cum_percentage,
    ))
}

pub fn validate_and_get_profile_data(profile_file_path: &PathBuf) -> Result<String, ProfileParsingError> {
    if !profile_file_path.exists() {
        return Err(ProfileParsingError::InvalidFormat(format!(
            "Profile file does not exist: {}",
            profile_file_path.display()
        )));
    }

    if !profile_file_path.is_file() {
        return Err(ProfileParsingError::InvalidFormat(format!(
            "Path is not a file: {}",
            profile_file_path.display()
        )));
    }

    let profile_data = fs::read_to_string(profile_file_path)?;

    if profile_data.trim().is_empty() {
        return Err(ProfileParsingError::EmptyFile);
    }

    Ok(profile_data)
}
