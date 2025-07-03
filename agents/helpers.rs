use std::path::PathBuf;

use gocortex::parser::{interface::parse_profile_data, types::ProfileParsingError};

use crate::agents::globals::PROMPT;

pub fn collect_parser_results(profile_paths: Vec<PathBuf>) -> Result<Vec<String>, ProfileParsingError> {
    let mut results = Vec::new();
    for profile_path in profile_paths {
        let profile_string = parse_profile_data(&profile_path)?;
        results.push(profile_string);
    }

    Ok(results)
}

pub fn structure_prompt(all_profile_strings: Vec<String>) -> String {
    let mut prompt = format!("{}", PROMPT);

    for (i, profile_string) in all_profile_strings.iter().enumerate() {
        prompt.push_str("\n");
        prompt.push_str(&format!("PROFILE NUMBER: {} \n", i + 1));
        prompt.push_str(&profile_string);
    }

    prompt
}
