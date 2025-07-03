use std::path::PathBuf;

use crate::parser::{
    helpers::{rewrite_profile_data, structure_profile_data, validate_and_get_profile_string},
    types::ProfileParsingError,
};

pub fn parse_profile_data(profile_file_path: &PathBuf) -> Result<String, ProfileParsingError> {
    let profile_data = validate_and_get_profile_string(profile_file_path)?;
    let (header_string, functions_profile_data) = structure_profile_data(&profile_data)?;
    Ok(rewrite_profile_data(header_string, functions_profile_data))
}
