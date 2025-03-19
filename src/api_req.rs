use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct RequestPayload {
    model: String,
    messages: Vec<Message>,
    max_tokens: i32,
    temperature: f32,
}

#[derive(Deserialize)]
struct ResponseChoice {
    message: ResponseMessage,
}

#[derive(Deserialize)]
struct ResponseMessage {
    content: String,
}

#[derive(Deserialize)]
struct ApiResponse {
    choices: Vec<ResponseChoice>,
}

/// Send a prompt to DeepSeek API and get the response.
///
/// # Arguments
///
/// * `prompt` - The input prompt to send to the API
/// * `api_key` - Your DeepSeek API key
/// * `model` - The model to use (default: deepseek-chat)
/// * `max_tokens` - Maximum number of tokens to generate
/// * `temperature` - Controls randomness (0.0-2.0)
///
/// # Returns
///
/// The text response from the API
pub async fn query_openai(
    prompt: &str,
    api_key: &str,
    model: &str,
    max_tokens: i32,
    temperature: f32,
) -> Result<String, Box<dyn Error>> {
    let client = Client::new();

    let payload = RequestPayload {
        model: model.to_string(),
        messages: vec![Message {
            role: "user".to_string(),
            content: prompt.to_string(),
        }],
        max_tokens,
        temperature,
    };

    let response = client
        .post("https://api.deepseek.com/v1/chat/completions")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&payload)
        .send()
        .await?;

    if response.status().is_success() {
        let api_response: ApiResponse = response.json().await?;
        Ok(api_response.choices[0].message.content.clone())
    } else {
        let status = response.status();
        let text = response.text().await?;
        Err(format!("API request failed with status code {}: {}", status, text).into())
    }
}
