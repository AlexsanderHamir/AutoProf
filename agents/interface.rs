use std::path::PathBuf;

use gocortex::parser::types::ProfileParsingError;

use crate::agents::helpers::{collect_parser_results, structure_prompt};

pub fn create_profiles_prompt(profile_paths: Vec<PathBuf>) -> Result<String, ProfileParsingError> {
    let res = collect_parser_results(profile_paths)?;
    let prompt = structure_prompt(res);
    Ok(prompt)
}
