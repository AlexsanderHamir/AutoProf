use std::{fs, path::PathBuf};

use crate::parser::{
    globals::{CPU_HEADER_SIZE, FUNCTION_NAME_INDEX, INLINE_WORD, REST_HEADER_SIZE},
    types::{FunctionProfileData, Header, Parallelism, ProfileParsingError, TotalNodes},
};

pub fn get_header<'a>(profile_data_lines: &'a [&str]) -> Result<(&'a [&'a str], usize, &'a str), ProfileParsingError> {
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

    Ok((header_slice, header_size, profile_type))
}

pub fn get_header_basic_fields<'a>(header: &'a [&str]) -> Result<&'a str, ProfileParsingError> {
    let file_name = header
        .get(0)
        .ok_or(ProfileParsingError::IncompleteHeader("Missing profile name".to_string()))?
        .strip_prefix("File: ")
        .ok_or(ProfileParsingError::IncompleteHeader("Invalid file line format".to_string()))?;

    Ok(file_name)
}

pub fn get_header_parallelism_info(header: &[&str]) -> Result<(f64, f64, f64), ProfileParsingError> {
    let duration_samples_line = header
        .get(3)
        .ok_or(ProfileParsingError::IncompleteHeader("Missing duration line".to_string()))?;

    let duration_time = duration_samples_line
        .strip_prefix("Duration: ")
        .ok_or(ProfileParsingError::IncompleteHeader("Invalid duration format".to_string()))?
        .split(',')
        .next()
        .ok_or(ProfileParsingError::IncompleteHeader("Missing duration value".to_string()))?
        .trim_end_matches(|c| c == 's' || c == 'm' || c == 'h')
        .parse::<f64>()
        .unwrap_or(0.0);

    let total_samples_line = duration_samples_line
        .split('=')
        .nth(1)
        .ok_or(ProfileParsingError::IncompleteHeader("No '=' found in duration samples line".to_string()))?
        .trim();

    let total_samples_time = total_samples_line
        .split('(')
        .next()
        .ok_or(ProfileParsingError::IncompleteHeader(
            "Missing opening parenthesis in total samples time".to_string(),
        ))?
        .trim_end_matches(|c| c == 's' || c == 'm' || c == 'h')
        .parse::<f64>()
        .unwrap_or(0.0);

    let total_samples_percentage = total_samples_line
        .split('(')
        .nth(1)
        .ok_or(ProfileParsingError::IncompleteHeader(
            "No opening parenthesis found in total samples percentage".to_string(),
        ))?
        .trim_end_matches(')')
        .trim_end_matches('%')
        .parse::<f64>()
        .unwrap_or(0.0);

    Ok((duration_time, total_samples_time, total_samples_percentage))
}

pub fn get_header_total_nodes_info(header: &[&str], profile_type: &str) -> Result<(f64, f64, f64), ProfileParsingError> {
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

    let collected_nodes_accounting_time = collected_nodes_accounting_time_line
        .split(',')
        .next()
        .ok_or(ProfileParsingError::IncompleteHeader("Missing comma".to_string()))?
        .trim_end_matches(|c| c == 's' || c == 'm' || c == 'h')
        .parse::<f64>()
        .unwrap_or(0.0);

    let collected_nodes_accounting_percentage = collected_nodes_accounting_time_line
        .split(',')
        .nth(1)
        .ok_or(ProfileParsingError::IncompleteHeader("Missing percentage part".to_string()))?
        .split('%')
        .next()
        .ok_or(ProfileParsingError::IncompleteHeader("Missing % sign".to_string()))?
        .parse::<f64>()
        .unwrap_or(0.0);

    let total_nodes_accounting_time = collected_nodes_accounting_time_line
        .split("of")
        .nth(1)
        .ok_or(ProfileParsingError::IncompleteHeader("Missing 'of' part".to_string()))?
        .trim_end_matches(|c| c == 's' || c == 'm' || c == 'h')
        .parse::<f64>()
        .unwrap_or(0.0);

    Ok((
        collected_nodes_accounting_time,
        collected_nodes_accounting_percentage,
        total_nodes_accounting_time,
    ))
}

pub fn build_header(profile_data_lines: &[&str]) -> Result<(Header, usize), ProfileParsingError> {
    let (header, header_size, profile_type) = get_header(profile_data_lines)?;
    let file_name = get_header_basic_fields(header)?;

    let parallelism = if profile_type == "cpu" {
        let (duration, total_samples_time, total_samples_percentage) = get_header_parallelism_info(header)?;
        Parallelism::new(duration, total_samples_time, total_samples_percentage)
    } else {
        Parallelism::new(0.0, 0.0, 0.0)
    };

    let (collected_nodes_accounting_time, collected_nodes_accounting_percentage, total_nodes_accounting_time) =
        get_header_total_nodes_info(header, profile_type)?;
    let header_struct = Header::new(
        file_name.to_string(),
        profile_type.to_string(),
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

    let function_name = if let Some(&"(inline)") = line_parts.last() {
        &line_parts[FUNCTION_NAME_INDEX..line_parts.len() - INLINE_WORD].join(" ")
    } else {
        &line_parts[FUNCTION_NAME_INDEX..].join(" ")
    };

    let flat_time = line_parts[0]
        .trim_end_matches(|c| c == 's' || c == 'm' || c == 'h')
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

    let cum_time = line_parts[3]
        .trim_end_matches(|c| c == 's' || c == 'm' || c == 'h')
        .parse::<f64>()
        .map_err(|e| ProfileParsingError::InvalidFormat(e.to_string()))?;

    let cum_percentage = line_parts[4]
        .trim_end_matches('%')
        .parse::<f64>()
        .map_err(|e| ProfileParsingError::InvalidFormat(e.to_string()))?;

    Ok(Some(FunctionProfileData::new(
        function_name.to_string(),
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
