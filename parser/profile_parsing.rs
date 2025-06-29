use std::path::PathBuf;

use crate::parser::{
    helpers::*,
    types::{FunctionProfileData, Header, ProfileParsingError},
};

const EMPTY_LINE_COUNT: usize = 1;

pub fn extract_profile_data(profile_file_path: &PathBuf) -> Result<(Header, Vec<FunctionProfileData>), ProfileParsingError> {
    let profile_data = validate_and_get_profile_data(profile_file_path)?;

    let profile_data_lines = profile_data.lines().collect::<Vec<&str>>();

    if profile_data_lines.is_empty() {
        return Err(ProfileParsingError::EmptyFile);
    }

    let (header, header_size) = build_header(&profile_data_lines)?;

    let body_lines = profile_data_lines.get(header_size..).ok_or_else(|| ProfileParsingError::InvalidFormat("No body found in profile data".to_string()))?;

    let mut functions_profile_data: Vec<FunctionProfileData> = Vec::with_capacity(body_lines.len() - EMPTY_LINE_COUNT);
    for line in body_lines {
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        let line_parts = line.split_whitespace().collect::<Vec<&str>>();

        if let Some(data) = collect_function_profile_data(&line_parts) {
            functions_profile_data.push(data);
        }
    }

    Ok((header, functions_profile_data))
}
