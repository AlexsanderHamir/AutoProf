use std::fs;

use crate::parser::helpers::*;

pub fn extract_profile_data(profile_file_path: &str) -> (Header, Vec<FunctionProfileData>) {
    let profile_data = fs::read_to_string(profile_file_path).expect("Failed to read profile file");
    let profile_data_lines = profile_data.split("\n").collect::<Vec<&str>>();

    let (header, header_size) = build_header(&profile_data_lines);
    let mut functions_profile_data: Vec<FunctionProfileData> = Vec::new();
    let body_lines = profile_data_lines.get(header_size..).expect("No body found");

    for line in body_lines {
        let line_parts = line.split_whitespace().collect::<Vec<&str>>();
        let function_profile_data = collect_function_profile_data(&line_parts);
        if function_profile_data.is_some() {
            functions_profile_data.push(function_profile_data.unwrap());
        }
    }

    return (header, functions_profile_data);
}
