mod agents;
mod parser;
use crate::agents::interface::get_analysis;
use dotenvy::dotenv;
use std::path::PathBuf;
use tokio::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let profile_file_paths = vec![PathBuf::from("tag_tests/cpu.txt"), PathBuf::from("tag_tests/mem.txt")];

    match get_analysis(profile_file_paths).await {
        Ok(analysis) => fs::write("AI.txt", analysis).await?,
        Err(e) => eprintln!("Error running analysis: {}", e),
    }

    Ok(())
}
