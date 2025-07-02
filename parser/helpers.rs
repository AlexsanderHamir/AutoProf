use std::{fs, path::PathBuf};

use crate::parser::{
    globals::{FUNCTION_NAME_INDEX, INLINE_WORD},
    types::{FunctionProfileData, ProfileParsingError},
};

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
