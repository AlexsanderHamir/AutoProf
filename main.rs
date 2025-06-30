use std::path::PathBuf;

mod parser;

fn main() {
    let profile_file_path = PathBuf::from("tag_tests/cpu.txt");

    for _ in 0..2000 {
        parser::profile_parsing::extract_profile_data(&profile_file_path).unwrap();
    }

    println!("Done");
}
