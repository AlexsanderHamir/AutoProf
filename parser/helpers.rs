use std::{fs, path::PathBuf};

use crate::parser::{
    globals::{CPU_HEADER_SIZE, NUMBER_REGEX, REST_HEADER_SIZE},
    types::{FunctionProfileData, Header, Parallelism, ProfileParsingError, TotalNodes},
};

pub fn get_header(profile_data_lines: &[&str]) -> Result<(Vec<String>, usize, String), ProfileParsingError> {
    let profile_type_line = profile_data_lines
        .get(1)
        .ok_or(ProfileParsingError::IncompleteHeader("Missing profile type line".to_string()))?;

    let profile_type = profile_type_line
        .strip_prefix("Type: ")
        .ok_or(ProfileParsingError::IncompleteHeader("Missing profile type prefix".to_string()))?;

    let header_size = if profile_type == "cpu" { CPU_HEADER_SIZE } else { REST_HEADER_SIZE };

    let header_slice = profile_data_lines
        .get(0..header_size)
        .ok_or(ProfileParsingError::IncompleteHeader("Incomplete header".to_string()))?;

    let header = header_slice.iter().map(|s| s.to_string()).collect();

    Ok((header, header_size, profile_type.to_string()))
}
pub fn get_header_basic_fields(header: &[String]) -> Result<String, ProfileParsingError> {
    let file_name_line = header
        .get(0)
        .ok_or(ProfileParsingError::IncompleteHeader("Missing profile name".to_string()))?;

    let file_name = file_name_line
        .strip_prefix("File: ")
        .ok_or(ProfileParsingError::IncompleteHeader("Invalid file line format".to_string()))?;

    Ok(file_name.to_string())
}

pub fn get_header_parallelism_info(header: &[String]) -> Result<(f64, f64, f64), ProfileParsingError> {
    let duration_samples_line = header
        .get(3)
        .ok_or(ProfileParsingError::IncompleteHeader("Missing duration line".to_string()))?;

    let duration_str = duration_samples_line
        .strip_prefix("Duration: ")
        .ok_or(ProfileParsingError::IncompleteHeader("Invalid duration format".to_string()))?
        .split(',')
        .next()
        .ok_or(ProfileParsingError::IncompleteHeader("Missing duration value".to_string()))?
        .trim();

    let duration = NUMBER_REGEX
        .find(duration_str)
        .and_then(|m| m.as_str().parse::<f64>().ok())
        .unwrap_or(0.0);

    let total_samples_line = duration_samples_line
        .split('=')
        .nth(1)
        .ok_or(ProfileParsingError::IncompleteHeader("No '=' found in duration samples line".to_string()))?
        .trim();

    let total_samples_time_str = total_samples_line
        .split('(')
        .next()
        .ok_or(ProfileParsingError::IncompleteHeader(
            "Missing opening parenthesis in total samples time".to_string(),
        ))?
        .trim();

    let total_samples_time = NUMBER_REGEX
        .find(total_samples_time_str)
        .and_then(|m| m.as_str().parse::<f64>().ok())
        .unwrap_or(0.0);

    let total_samples_percentage_str = total_samples_line
        .split('(')
        .nth(1)
        .ok_or(ProfileParsingError::IncompleteHeader(
            "No opening parenthesis found in total samples percentage".to_string(),
        ))?
        .trim_end_matches(')')
        .trim_end_matches('%')
        .trim();

    let total_samples_percentage = total_samples_percentage_str.parse::<f64>().unwrap_or(0.0);

    Ok((duration, total_samples_time, total_samples_percentage))
}

pub fn get_header_total_nodes_info(header: &[String], profile_type: &str) -> Result<(f64, f64, f64), ProfileParsingError> {
    let total_nodes_line_index = if profile_type == "cpu" { 4 } else { 3 };

    let total_nodes_line = header
        .get(total_nodes_line_index)
        .ok_or(ProfileParsingError::IncompleteHeader("Total nodes line not found".to_string()))?;

    let collected_nodes_accounting_time_line =
        total_nodes_line
            .strip_prefix("Showing nodes accounting for ")
            .ok_or(ProfileParsingError::IncompleteHeader(
                "Line doesn't belong to total nodes, missing prefix".to_string(),
            ))?;

    let collected_nodes_accounting_time_str = collected_nodes_accounting_time_line
        .split(',')
        .next()
        .ok_or(ProfileParsingError::IncompleteHeader("Missing comma".to_string()))?
        .trim();

    let collected_nodes_accounting_time = NUMBER_REGEX
        .find(collected_nodes_accounting_time_str)
        .and_then(|m| m.as_str().parse::<f64>().ok())
        .unwrap_or(0.0);

    let collected_nodes_accounting_percentage_str = collected_nodes_accounting_time_line
        .split(',')
        .nth(1)
        .ok_or(ProfileParsingError::IncompleteHeader("Missing percentage part".to_string()))?
        .split('%')
        .next()
        .ok_or(ProfileParsingError::IncompleteHeader("Missing % sign".to_string()))?
        .trim();

    let collected_nodes_accounting_percentage = collected_nodes_accounting_percentage_str.parse::<f64>().unwrap_or(0.0);

    let total_nodes_accounting_time_str = collected_nodes_accounting_time_line
        .split("of")
        .nth(1)
        .ok_or(ProfileParsingError::IncompleteHeader("Missing 'of' part".to_string()))?
        .trim();

    let total_nodes_accounting_time = NUMBER_REGEX
        .find(total_nodes_accounting_time_str)
        .and_then(|m| m.as_str().parse::<f64>().ok())
        .unwrap_or(0.0);

    Ok((
        collected_nodes_accounting_time,
        collected_nodes_accounting_percentage,
        total_nodes_accounting_time,
    ))
}

pub fn build_header(profile_data_lines: &[&str]) -> Result<(Header, usize), ProfileParsingError> {
    let (header, header_size, profile_type) = get_header(&profile_data_lines)?;
    let file_name = get_header_basic_fields(&header)?;

    let parallelism = if profile_type == "cpu" {
        let (duration, total_samples_time, total_samples_percentage) = get_header_parallelism_info(&header)?;
        Parallelism::new(duration, total_samples_time, total_samples_percentage)
    } else {
        Parallelism::new(0.0, 0.0, 0.0)
    };

    let (collected_nodes_accounting_time, collected_nodes_accounting_percentage, total_nodes_accounting_time) =
        get_header_total_nodes_info(&header, &profile_type)?;
    let header_struct = Header::new(
        file_name,
        profile_type,
        parallelism,
        TotalNodes::new(
            collected_nodes_accounting_time,
            collected_nodes_accounting_percentage,
            total_nodes_accounting_time,
        ),
    );

    Ok((header_struct, header_size))
}

pub fn collect_function_profile_data(line_parts: &[&str]) -> Result<Option<FunctionProfileData>, ProfileParsingError> {
    if line_parts.len() < 6 {
        return Err(ProfileParsingError::IncompleteBody("Invalid line parts".to_string()));
    }

    let function_name = {
        let parts = if let Some(&"(inline)") = line_parts.last() {
            &line_parts[5..line_parts.len() - 1]
        } else {
            &line_parts[5..]
        };
        parts.join(" ")
    };

    let flat_time = NUMBER_REGEX
        .find(line_parts[0])
        .map_or("0", |m| m.as_str())
        .parse::<f64>()
        .map_err(|e| ProfileParsingError::InvalidFormat(e.to_string()))?;

    let cum_time = NUMBER_REGEX
        .find(line_parts[3])
        .map_or("0", |m| m.as_str())
        .parse::<f64>()
        .map_err(|e| ProfileParsingError::InvalidFormat(e.to_string()))?;

    let flat_percentage = line_parts[1]
        .trim_end_matches('%')
        .parse::<f64>()
        .map_err(|e| ProfileParsingError::InvalidFormat(e.to_string()))?;

    let sum_percentage = line_parts[2]
        .trim_end_matches('%')
        .parse::<f64>()
        .map_err(|e| ProfileParsingError::InvalidFormat(e.to_string()))?;

    let cum_percentage = line_parts[4]
        .trim_end_matches('%')
        .parse::<f64>()
        .map_err(|e| ProfileParsingError::InvalidFormat(e.to_string()))?;

    Ok(Some(FunctionProfileData::new(
        function_name,
        flat_time,
        flat_percentage,
        sum_percentage,
        cum_time,
        cum_percentage,
    )))
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
