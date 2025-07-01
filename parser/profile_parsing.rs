use std::path::PathBuf;

use crate::parser::{
    globals::EMPTY_LINE_COUNT,
    helpers::*,
    types::{FunctionProfileData, Header, ProfileParsingError},
};

pub fn parse_profile_data(profile_file_path: &PathBuf) -> Result<(Header, Vec<FunctionProfileData>), ProfileParsingError> {
    let profile_data = validate_and_get_profile_data(profile_file_path)?;
    extract_profile_data(&profile_data)
}

pub fn extract_profile_data(profile_data: &str) -> Result<(Header, Vec<FunctionProfileData>), ProfileParsingError> {
    let profile_data_lines = profile_data.lines().collect::<Vec<&str>>();
    if profile_data_lines.is_empty() {
        return Err(ProfileParsingError::EmptyFile);
    }

    let (header, header_size) = build_header(&profile_data_lines)?;

    let body_lines = profile_data_lines
        .get(header_size..)
        .ok_or(ProfileParsingError::IncompleteBody("No body found in profile data".to_string()))?;

    if body_lines.is_empty() {
        return Err(ProfileParsingError::IncompleteBody("Empty body".to_string()));
    }

    let mut functions_profile_data: Vec<FunctionProfileData> = Vec::with_capacity(body_lines.len() - EMPTY_LINE_COUNT);
    for line in body_lines {
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        let line_parts = line.split_whitespace().collect::<Vec<&str>>();

        if let Some(data) = collect_function_profile_data(&line_parts)? {
            functions_profile_data.push(data);
        }
    }

    if functions_profile_data.is_empty() {
        return Err(ProfileParsingError::IncompleteBody("Empty functions profile data".to_string()));
    }

    Ok((header, functions_profile_data))
}
