mod agents;
mod parser;
use std::path::PathBuf;

fn main() {
    let profile_file_paths = vec![PathBuf::from("tag_tests/cpu.txt"), PathBuf::from("tag_tests/mem.txt")];

    match agents::interface::create_profiles_prompt(profile_file_paths) {
        Ok(parser_results) => {
            println!("{}", parser_results);
        }
        Err(e) => {
            eprintln!("Error parsing profile data: {}", e);
        }
    }
}
