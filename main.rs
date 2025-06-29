use std::path::PathBuf;

mod parser;

fn main() {
    let profile_file_path = PathBuf::from("tag_name_example/text/BenchmarkGenPool/BenchmarkGenPool_mutex.txt");

    match parser::profile_parsing::extract_profile_data(&profile_file_path) {
        Ok((header, functions_profile_data)) => {
            println!("{:#?}", header);
            println!("{:#?}", functions_profile_data);
        }
        Err(e) => {
            eprintln!("Error parsing profile data: {}", e);
            std::process::exit(1);
        }
    }
}
