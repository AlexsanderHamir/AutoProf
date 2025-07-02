use std::path::PathBuf;
mod parser;

fn main() {
    let profile_file_path = PathBuf::from("tag_tests/cpu.txt");

    let (header, functions_profile_data) = parser::interface::parse_profile_data(&profile_file_path).unwrap();

    println!("Header: {:?}", header);
    println!("Functions profile data: {:?}", functions_profile_data);
    println!("Done");
}
