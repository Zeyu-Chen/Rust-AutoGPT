use crate::models::general::llm::{ChatCompletion, Message};
use dotenv::dotenv;
use reqwest::Client;
use std::env;

use reqwest::header::{HeaderMap, HeaderValue};

// Call large Language Model
pub async fn call_gpt(messages: Vec<Message>) {
    dotenv().ok();

    // Extract API Key information
    let api_key = env::var("OPEN_AI_KEY").expect("OPEN_AI_KEY not found in .env");
    let api_org = env::var("OPEN_AI_ORG").expect("OPEN_AI_ORG not found in .env");

    // Confirm endpoint
    let url = "https://api.openai.com/v1/chat/completions";

    // Create header
    let mut headers = HeaderMap::new();

    // Create OpenAI key header
    headers.insert(
        "authorization",
        HeaderValue::from_str(&format!("Bearer {}", api_key)).unwrap(),
    );

    // Create OpenAI Org header
    headers.insert(
        "OpenAI-Organization",
        HeaderValue::from_str(api_org.as_str()).unwrap(),
    );

    // Create Client
    let client = Client::builder().default_headers(headers).build().unwrap();

    // Create ChatCompletion
    let chat_completion = ChatCompletion {
        model: "gpt-4o".to_string(),
        messages,
        temperature: 0.1,
    };

    // Troubleshooting
    let res_raw = client
        .post(url)
        .json(&chat_completion)
        .send()
        .await
        .unwrap();
    dbg!(res_raw.text().await.unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn tests_call_to_openai() {
        let message: Message = Message {
            role: "user".to_string(),
            content: "Hi there, this is a test. Give me a short response.".to_string(),
        };

        let messages: Vec<Message> = vec![message];

        call_gpt(messages).await;
    }
}
