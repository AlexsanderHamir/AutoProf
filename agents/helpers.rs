use std::path::PathBuf;
use tiktoken_rs::get_bpe_from_model;

use gocortex::parser::{interface::parse_profile_data, types::ProfileParsingError};
use openai::chat::ChatCompletionMessage;

use crate::agents::globals::PROMPT;

pub fn collect_parser_results(profile_paths: Vec<PathBuf>) -> Result<Vec<String>, ProfileParsingError> {
    let mut results = Vec::new();
    for profile_path in profile_paths {
        let profile_string = parse_profile_data(&profile_path)?;
        results.push(profile_string);
    }

    Ok(results)
}

pub fn structure_prompt(all_profile_strings: Vec<String>) -> String {
    let mut prompt = format!("{}", PROMPT);

    for (i, profile_string) in all_profile_strings.iter().enumerate() {
        prompt.push_str("\n");
        prompt.push_str(&format!("PROFILE NUMBER: {} \n", i + 1));
        prompt.push_str(&profile_string);
    }

    prompt
}

fn count_tokens(messages: &[ChatCompletionMessage]) -> usize {
    let bpe = get_bpe_from_model("gpt-4.1-nano").expect("Tokenizer load failed");

    messages
        .iter()
        .map(|msg| {
            let text = msg.content.as_ref().map(String::as_str).unwrap_or("");
            let tokens = bpe.encode_with_special_tokens(text);
            tokens.len()
        })
        .sum()
}
