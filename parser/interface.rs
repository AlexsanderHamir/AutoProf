use std::path::PathBuf;

use crate::parser::{
    helpers::*,
    types::{FunctionProfileData, ProfileParsingError},
};

pub fn parse_profile_data(profile_file_path: &PathBuf) -> Result<String, ProfileParsingError> {
    let profile_data = validate_and_get_profile_data(profile_file_path)?;
    let (header_string, functions_profile_data) = extract_profile_data(&profile_data)?;
    Ok(rewrite_profile_data(header_string, functions_profile_data))
}

fn rewrite_profile_data(header_string: String, functions_profile_data: Vec<FunctionProfileData>) -> String {
    let mut rewritten_profile_data = String::new();

    rewritten_profile_data.push_str(&header_string);
    rewritten_profile_data.push_str("\n");

    rewritten_profile_data.push_str(&format!(
        "{:<8} {:<8} {:<10} {:<8} {:<10} {}\n",
        "flat", "flat%", "sum%", "cum", "cum%", "function"
    ));

    for entry in functions_profile_data {
        rewritten_profile_data.push_str(&format!(
            "{:<8.2} {:<8} {:<10.2} {:<8.2} {:<10.2} {}\n",
            entry.flat, entry.flat_percentage, entry.sum_percentage, entry.cum, entry.cum_percentage, entry.function_name
        ));
    }

    rewritten_profile_data
}
