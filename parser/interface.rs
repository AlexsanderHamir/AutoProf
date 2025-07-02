use std::path::PathBuf;

use crate::parser::{
    helpers::*,
    types::{FunctionProfileData, ProfileParsingError},
};

pub fn parse_profile_data(profile_file_path: &PathBuf) -> Result<(String, Vec<FunctionProfileData>), ProfileParsingError> {
    let profile_data = validate_and_get_profile_data(profile_file_path)?;
    extract_profile_data(&profile_data)
}
