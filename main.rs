use std::{fs, path::PathBuf};
mod parser;
mod rewriter;
use tiktoken_rs::o200k_base;

fn main() {
    let profile_file_path = PathBuf::from("tag_tests/cpu.txt");
    let bpe_encoder = o200k_base().unwrap();

    match parser::interface::parse_profile_data(&profile_file_path) {
        Ok((header_string, functions_profile_data)) => {
            let rewritten_profile_data = rewriter::interface::rewrite_profile_data(header_string, functions_profile_data);
            let tokens = bpe_encoder.encode_with_special_tokens(&rewritten_profile_data);
            println!("Optimized Parsing: {}", tokens.len());
        }
        Err(e) => {
            eprintln!("Error parsing profile data: {}", e);
        }
    }

    let profile_data = fs::read_to_string(profile_file_path).unwrap();
    if profile_data.trim().is_empty() {
        println!("Empty file");
    }

    let tokens = bpe_encoder.encode_with_special_tokens(&profile_data);
    println!("Original Parsing: {}", tokens.len());
}
