use regex::Regex;

use crate::parser::types::{FunctionProfileData, Header, Parallelism, ProfileParsingError, TotalNodes};
use std::{fs, path::PathBuf, process};

pub fn get_header(profile_data_lines: &[&str]) -> (Vec<String>, usize, String) {
    let profile_type_line = profile_data_lines.get(1).unwrap_or_else(|| {
        exit_with_error("Expected profile type on line 2, but it was missing.");
        unreachable!()
    });

    let profile_type = profile_type_line.strip_prefix("Type: ").unwrap_or_else(|| {
        exit_with_error("Invalid profile type format: expected line to start with \"Type: \"");
        unreachable!()
    });

    let header_size = if profile_type == "cpu" { 6 } else { 5 };

    let header = profile_data_lines
        .get(0..header_size)
        .unwrap_or_else(|| {
            exit_with_error(&format!("No header found: expected at least {} lines, found {}", header_size, profile_data_lines.len()));
            unreachable!()
        })
        .iter()
        .map(|s| s.to_string())
        .collect();

    (header, header_size, profile_type.to_string())
}

pub fn get_header_basic_fields(header: &[String]) -> String {
    let file_name_line = header.get(0).unwrap_or_else(|| {
        exit_with_error("No file name found");
        unreachable!()
    });
    let file_name = file_name_line.strip_prefix("File: ").unwrap_or_else(|| {
        exit_with_error("Invalid file line format");
        unreachable!()
    });

    file_name.to_string()
}

pub fn get_header_parallelism_info(header: &[String]) -> (f64, f64, f64) {
    let duration_samples_line = header.get(3).unwrap_or_else(|| {
        exit_with_error("No duration found");
        unreachable!()
    });

    let duration_str = duration_samples_line
        .strip_prefix("Duration: ")
        .unwrap_or_else(|| {
            exit_with_error("Invalid duration format");
            unreachable!()
        })
        .split(',')
        .next()
        .unwrap_or_else(|| {
            exit_with_error("Missing duration value");
            unreachable!()
        })
        .trim();

    let number_regex = Regex::new(r"^[\d.]+").unwrap();
    let duration = number_regex.find(duration_str).map(|m| m.as_str().parse::<f64>().unwrap()).unwrap_or(0.0);

    let total_samples_line = duration_samples_line
        .split('=')
        .nth(1)
        .unwrap_or_else(|| {
            exit_with_error("No '=' found in duration samples line");
            unreachable!()
        })
        .trim();

    let total_samples_time_str = total_samples_line
        .split('(')
        .next()
        .unwrap_or_else(|| {
            exit_with_error("Missing opening parenthesis in total samples time");
            unreachable!()
        })
        .trim();

    let total_samples_time = number_regex.find(total_samples_time_str).map(|m| m.as_str().parse::<f64>().unwrap()).unwrap_or(0.0);

    let total_samples_percentage_str = total_samples_line
        .split('(')
        .nth(1)
        .unwrap_or_else(|| {
            exit_with_error("No opening parenthesis found in total samples percentage");
            unreachable!()
        })
        .trim_end_matches(')')
        .trim_end_matches('%')
        .trim();

    let total_samples_percentage = total_samples_percentage_str.parse::<f64>().unwrap_or(0.0);

    (duration, total_samples_time, total_samples_percentage)
}

pub fn get_header_total_nodes_info(header: &[String], profile_type: &str) -> (f64, f64, f64) {
    let header_index = if profile_type == "cpu" { 4 } else { 3 };

    let total_nodes_line = header.get(header_index).unwrap_or_else(|| {
        exit_with_error("No total nodes found");
        unreachable!()
    });

    let collected_nodes_accounting_time_line = total_nodes_line.strip_prefix("Showing nodes accounting for ").unwrap_or_else(|| {
        exit_with_error("Invalid total nodes format");
        unreachable!()
    });

    let collected_nodes_accounting_time_str = collected_nodes_accounting_time_line
        .split(',')
        .next()
        .unwrap_or_else(|| {
            exit_with_error("Missing comma");
            unreachable!()
        })
        .trim();

    let number_regex = Regex::new(r"^[\d.]+").unwrap();
    let collected_nodes_accounting_time = number_regex.find(collected_nodes_accounting_time_str).map(|m| m.as_str().parse::<f64>().unwrap()).unwrap_or(0.0);

    let collected_nodes_accounting_percentage_str = collected_nodes_accounting_time_line
        .split(',')
        .nth(1)
        .unwrap_or_else(|| {
            exit_with_error("Missing percentage part");
            unreachable!()
        })
        .split('%')
        .next()
        .unwrap_or_else(|| {
            exit_with_error("Missing % sign");
            unreachable!()
        })
        .trim();

    let collected_nodes_accounting_percentage = collected_nodes_accounting_percentage_str.parse::<f64>().unwrap_or(0.0);

    let total_nodes_accounting_time_str = collected_nodes_accounting_time_line
        .split("of")
        .nth(1)
        .unwrap_or_else(|| {
            exit_with_error("Missing 'of' part");
            unreachable!()
        })
        .trim();

    let total_nodes_accounting_time = number_regex.find(total_nodes_accounting_time_str).map(|m| m.as_str().parse::<f64>().unwrap()).unwrap_or(0.0);

    (collected_nodes_accounting_time, collected_nodes_accounting_percentage, total_nodes_accounting_time)
}

pub fn build_header(profile_data_lines: &[&str]) -> (Header, usize) {
    let (header, header_size, profile_type) = get_header(&profile_data_lines);
    let file_name = get_header_basic_fields(&header);

    let parallelism = if profile_type == "cpu" {
        let (duration, total_samples_time, total_samples_percentage) = get_header_parallelism_info(&header);
        Parallelism::new(duration, total_samples_time, total_samples_percentage)
    } else {
        Parallelism::new(0.0, 0.0, 0.0)
    };

    let (collected_nodes_accounting_time, collected_nodes_accounting_percentage, total_nodes_accounting_time) = get_header_total_nodes_info(&header, &profile_type);
    let header_struct = Header::new(file_name, profile_type, parallelism, TotalNodes::new(collected_nodes_accounting_time, collected_nodes_accounting_percentage, total_nodes_accounting_time));

    (header_struct, header_size)
}

pub fn collect_function_profile_data(line_parts: &[&str]) -> Option<FunctionProfileData> {
    if line_parts.len() < 6 {
        return None;
    }

    let function_name = {
        let parts = if let Some(&"(inline)") = line_parts.last() { &line_parts[5..line_parts.len() - 1] } else { &line_parts[5..] };
        parts.join(" ")
    };

    let number_regex = Regex::new(r"^[\d.]+").unwrap();

    let flat_time_str = number_regex.find(line_parts[0]).map(|m| m.as_str()).unwrap_or("0");
    let flat_time = flat_time_str.parse::<f64>().unwrap();

    let cum_time_str = number_regex.find(line_parts[3]).map(|m| m.as_str()).unwrap_or("0");
    let cum_time = cum_time_str.parse::<f64>().unwrap();

    let flat_percentage = line_parts[1].trim_end_matches('%').parse::<f64>().unwrap();
    let sum_percentage = line_parts[2].trim_end_matches('%').parse::<f64>().unwrap();
    let cum_percentage = line_parts[4].trim_end_matches('%').parse::<f64>().unwrap();

    Some(FunctionProfileData::new(function_name, flat_time, flat_percentage, sum_percentage, cum_time, cum_percentage))
}

pub fn validate_and_get_profile_data(profile_file_path: &PathBuf) -> Result<String, ProfileParsingError> {
    if !profile_file_path.exists() {
        return Err(ProfileParsingError::InvalidFormat(format!("Profile file does not exist: {}", profile_file_path.display())));
    }

    if !profile_file_path.is_file() {
        return Err(ProfileParsingError::InvalidFormat(format!("Path is not a file: {}", profile_file_path.display())));
    }

    let profile_data = fs::read_to_string(profile_file_path)?;

    if profile_data.trim().is_empty() {
        return Err(ProfileParsingError::EmptyFile);
    }

    Ok(profile_data)
}

pub fn exit_with_error(message: &str) {
    eprintln!("Error: {}", message);
    process::exit(1);
}
