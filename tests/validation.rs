mod constants;
mod helpers;

#[cfg(test)]
mod tests {

    use crate::{constants::*, helpers::*};

    use gocortex::parser::{
        profile_parsing::extract_profile_data,
        types::{FunctionProfileData, Header, ProfileParsingError},
    };
    use std::path::PathBuf;

    #[test]
    fn test_invalid_profile_header() {
        fn subtest(
            name: &str,
            expected_err_msg: &str,
            test_fn: impl FnOnce(&PathBuf) -> Result<(Header, Vec<FunctionProfileData>), ProfileParsingError>,
            line_index: usize,
        ) {
            println!("Running subtest: {}", name);

            let profile_file_path = PathBuf::from("tag_tests/cpu.txt");
            let removed_line = remove_line_at_index(&profile_file_path, line_index)
                .expect("IO error")
                .expect("Line not found, File length insufficient");

            let result = test_fn(&profile_file_path);
            insert_line_at_index(&profile_file_path, line_index, &removed_line).expect("IO error");

            match result {
                Ok(_) => panic!("Expected error but test_fn returned Ok"),
                Err(e) => {
                    let err_msg = e.to_string();
                    assert_eq!(err_msg, expected_err_msg, "Error message mismatch");
                    println!("Test passed: {}", name);
                }
            }
        }

        subtest("missing_profile_type_line", MISSING_PROFILE_TYPE_PREFIX, extract_profile_data, 1);
        subtest("missing_duration", MISSING_DURATION, extract_profile_data, 3);
        subtest("missing_nodes_accounting", MISSING_TOTAL_NODES_PREFIX, extract_profile_data, 4);
    }

    #[test]
    fn test_invalid_profile_body() {
        fn subtest(
            name: &str,
            expected_err_msg: &str,
            test_fn: impl FnOnce(&PathBuf) -> Result<(Header, Vec<FunctionProfileData>), ProfileParsingError>,
            profile_file_path: &str,
        ) {
            println!("Running subtest: {}", name);
            let profile_file_path = PathBuf::from(profile_file_path);
            let result = test_fn(&profile_file_path);
            match result {
                Ok(_) => panic!("Expected error but test_fn returned Ok"),
                Err(e) => {
                    let err_msg = e.to_string();
                    assert_eq!(err_msg, expected_err_msg, "Error message mismatch");
                    println!("Test passed: {}", name);
                }
            }
        }

        subtest("empty_body", EMPTY_BODY, extract_profile_data, "tag_tests/cpu_missing_body.txt");
        subtest(
            "empty_body_lines",
            EMPTY_FUNCTIONS_PROFILE_DATA,
            extract_profile_data,
            "tag_tests/cpu_emptyLines_body.txt",
        );
    }
}
