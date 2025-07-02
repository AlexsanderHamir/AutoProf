use std::path::PathBuf;
mod parser;
mod rewriter;

fn main() {
    let profile_file_path = PathBuf::from("tag_tests/cpu.txt");

    let (header, functions_profile_data) = parser::interface::parse_profile_data(&profile_file_path).unwrap();
    let rewritten_profile_data = rewriter::interface::rewrite_profile_data(header, functions_profile_data);
    println!("{}", rewritten_profile_data);
}
