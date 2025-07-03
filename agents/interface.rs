use openai::{
    Credentials,
    chat::{ChatCompletion, ChatCompletionMessage, ChatCompletionMessageRole},
};

use std::path::PathBuf;

use gocortex::parser::types::ProfileParsingError;

use crate::agents::helpers::{collect_parser_results, structure_prompt};

pub fn create_profiles_prompt(profile_paths: Vec<PathBuf>) -> Result<String, ProfileParsingError> {
    let res = collect_parser_results(profile_paths)?;
    let prompt = structure_prompt(res);
    Ok(prompt)
}

pub async fn get_analysis(profile_paths: Vec<PathBuf>) -> Result<String, ProfileParsingError> {
    let final_prompt = create_profiles_prompt(profile_paths)?;

    let credentials = Credentials::from_env();
    let messages = vec![
        ChatCompletionMessage {
            role: ChatCompletionMessageRole::System,
            content: Some("You're a golang performance analist".to_string()),
            name: None,
            function_call: None,
            tool_call_id: None,
            tool_calls: None,
        },
        ChatCompletionMessage {
            role: ChatCompletionMessageRole::User,
            content: Some(final_prompt),
            name: None,
            function_call: None,
            tool_call_id: None,
            tool_calls: None,
        },
    ];

    let chat_completion = ChatCompletion::builder("gpt-4.1-nano", messages.clone())
        .credentials(credentials.clone())
        .max_tokens(32768u64)
        .temperature(0.3)
        .create()
        .await
        .unwrap();

    let returned_message = chat_completion.choices.first().unwrap().message.clone();

    Ok(returned_message.content.unwrap_or_default())
}
