use std::path::PathBuf;

use crate::parser::{
    helpers::{extract_profile_data, rewrite_profile_data, validate_and_get_profile_data},
    types::ProfileParsingError,
};

pub fn parse_profile_data(profile_file_path: &PathBuf) -> Result<String, ProfileParsingError> {
    let profile_data = validate_and_get_profile_data(profile_file_path)?;
    let (header_string, functions_profile_data) = extract_profile_data(&profile_data)?;
    Ok(rewrite_profile_data(header_string, functions_profile_data))
}
