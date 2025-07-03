use std::{fs, path::PathBuf};

use crate::parser::{
    globals::{CUM_MINIMUM, EMPTY_LINE_COUNT, FUNCTION_NAME_INDEX, INLINE_WORD, SUM_MAXIMUM},
    types::{FunctionProfileData, ProfileParsingError},
};

pub fn rewrite_profile_data(header_string: String, functions_profile_data: Vec<FunctionProfileData>) -> String {
    let mut rewritten_profile_data = String::new();

    rewritten_profile_data.push_str(&header_string);
    rewritten_profile_data.push_str("\n");

    rewritten_profile_data.push_str(&format!(
        "{:<8} {:<8} {:<10} {:<8} {:<10} {}\n",
        "flat", "flat%", "sum%", "cum", "cum%", "function"
    ));

    for entry in functions_profile_data {
        rewritten_profile_data.push_str(&format!(
            "{:<8.2} {:<8} {:<10.2} {:<8.2} {:<10.2} {}\n",
            entry.flat, entry.flat_percentage, entry.sum_percentage, entry.cum, entry.cum_percentage, entry.function_name
        ));
    }

    rewritten_profile_data
}

pub fn structure_profile_data(profile_data: &str) -> Result<(String, Vec<FunctionProfileData>), ProfileParsingError> {
    let profile_data_lines = profile_data.lines().collect::<Vec<&str>>();
    if profile_data_lines.is_empty() {
        return Err(ProfileParsingError::EmptyFile);
    }

    let (header_string, header_size) = get_header_info(&profile_data_lines)?;
    let body_lines = profile_data_lines
        .get(header_size..)
        .ok_or(ProfileParsingError::IncompleteBody("No body found in profile data".to_string()))?;

    if body_lines.is_empty() {
        return Err(ProfileParsingError::IncompleteBody("Empty body".to_string()));
    }

    let mut functions_profile_data: Vec<FunctionProfileData> = Vec::with_capacity(body_lines.len() - EMPTY_LINE_COUNT);
    for line in body_lines {
        // TODO: This entire loop is a bottleneck CPU wise.
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        let line_parts = line.split_whitespace().collect::<Vec<&str>>();
        if let Some(data) = collect_function_profile_data(&line_parts)? {
            if data.sum_percentage <= SUM_MAXIMUM || data.cum_percentage >= CUM_MINIMUM {
                functions_profile_data.push(data);
            }
        }
    }

    if functions_profile_data.is_empty() {
        return Err(ProfileParsingError::IncompleteBody("Empty functions profile data".to_string()));
    }

    Ok((header_string, functions_profile_data))
}

pub fn get_header_slice<'a>(profile_data_lines: &'a [&str]) -> Result<&'a [&'a str], ProfileParsingError> {
    let last_header_index = profile_data_lines
        .iter()
        .position(|line| line.trim_start().starts_with("flat"))
        .ok_or_else(|| ProfileParsingError::IncompleteHeader("Missing final header row (flat/cum/etc)".to_string()))?;

    profile_data_lines
        .get(0..=last_header_index)
        .ok_or_else(|| ProfileParsingError::IncompleteHeader("Header slice out of bounds".to_string()))
}

pub fn get_header_info(profile_data_lines: &[&str]) -> Result<(String, usize), ProfileParsingError> {
    let header_slice = get_header_slice(profile_data_lines)?;
    let cleaned_header_slice = clean_header_slice(header_slice);
    let header_string = cleaned_header_slice.join("\n");
    Ok((header_string, header_slice.len()))
}

fn clean_header_slice<'a>(header_slice: &'a [&'a str]) -> Vec<&'a str> {
    header_slice
        .iter()
        .copied()
        .filter(|line| !line.starts_with("Time: ") && !line.starts_with("File: ") && !line.trim_start().starts_with("flat"))
        .collect()
}

pub fn collect_function_profile_data(line_parts: &[&str]) -> Result<Option<FunctionProfileData>, ProfileParsingError> {
    if line_parts.len() < 6 {
        return Err(ProfileParsingError::IncompleteBody("Invalid line parts".to_string()));
    }

    let function_name = if let Some(&"(inline)") = line_parts.last() {
        &line_parts[FUNCTION_NAME_INDEX..line_parts.len() - INLINE_WORD].join(" ")
    } else {
        &line_parts[FUNCTION_NAME_INDEX..].join(" ")
    };

    let flat_time = trim_non_numeric_end(line_parts[0])
        .parse::<f64>()
        .map_err(|e| ProfileParsingError::InvalidFormat(format!("Flat time: {}", e.to_string())))?;

    let flat_percentage = trim_non_numeric_end(line_parts[1])
        .parse::<f64>()
        .map_err(|e| ProfileParsingError::InvalidFormat(format!("Flat percentage: {}", e.to_string())))?;

    let sum_percentage = trim_non_numeric_end(line_parts[2])
        .parse::<f64>()
        .map_err(|e| ProfileParsingError::InvalidFormat(format!("Sum percentage: {}", e.to_string())))?;

    let cum_time = trim_non_numeric_end(line_parts[3])
        .parse::<f64>()
        .map_err(|e| ProfileParsingError::InvalidFormat(format!("Cum time: {}", e.to_string())))?;

    let cum_percentage = trim_non_numeric_end(line_parts[4])
        .parse::<f64>()
        .map_err(|e| ProfileParsingError::InvalidFormat(format!("Cum percentage: {}", e.to_string())))?;

    Ok(Some(FunctionProfileData::new(
        function_name.to_string(),
        flat_time,
        flat_percentage,
        sum_percentage,
        cum_time,
        cum_percentage,
    )))
}

fn trim_non_numeric_end(s: &str) -> &str {
    let bytes = s.as_bytes();
    let mut end = s.len();

    while end > 0 {
        let c = bytes[end - 1] as char;
        if c.is_digit(10) || c == '.' {
            break;
        }
        end -= 1;
    }

    &s[..end]
}

pub fn validate_and_get_profile_string(profile_file_path: &PathBuf) -> Result<String, ProfileParsingError> {
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
