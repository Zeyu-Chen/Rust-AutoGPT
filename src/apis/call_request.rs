//! This module provides functionality to call the OpenAI GPT API for generating chat completions.
//!
//! # Overview
//!
//! The `call_gpt` function sends a list of messages to the OpenAI GPT API and retrieves the generated response.
//! It uses the `reqwest` library to make HTTP requests and handles API key and organization ID through environment variables.
//!
//! # Environment Variables
//!
//! - `OPEN_AI_KEY`: The API key for authenticating with the OpenAI API.
//! - `OPEN_AI_ORG`: The organization ID for the OpenAI API.
//!
//! # Functions
//!
//! - `call_gpt`: Asynchronously sends a list of messages to the OpenAI GPT API and returns the generated response as a `Result<String, Box<dyn std::error::Error + Send>>`.
//!
//! # Example
//!
//! ```rust
//! use crate::apis::call_request::call_gpt;
//! use crate::models::general::llm::Message;
//!
//! #[tokio::main]
//! async fn main() {
//!     let message = Message {
//!         role: "user".to_string(),
//!         content: "Hello, how are you?".to_string(),
//!     };
//!     let messages = vec![message];
//!
//!     match call_gpt(messages).await {
//!         Ok(response) => println!("Response: {}", response),
//!         Err(e) => eprintln!("Error: {}", e),
//!     }
//! }
//! ```
//!
//! # Tests
//!
//! The module includes a test function `tests_call_to_openai` that verifies the `call_gpt` function by sending a test message and checking the response.
use crate::models::general::llm::{APIResponse, ChatCompletion, Message};
use dotenv::dotenv;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::Client;
use std::env;

/// Asynchronously sends a list of messages to the OpenAI GPT API and returns the generated response.
///
/// This function constructs an HTTP request to the OpenAI GPT API using the provided messages,
/// and retrieves the generated response. It handles the API key and organization ID through
/// environment variables `OPEN_AI_KEY` and `OPEN_AI_ORG`.
///
/// # Arguments
///
/// * `messages` - A vector of `Message` structs representing the conversation history.
///
/// # Returns
///
/// A `Result` which is:
/// * `Ok(String)` containing the generated response from the OpenAI GPT API.
/// * `Err(Box<dyn std::error::Error + Send>)` if there was an error during the request.
///
/// # Errors
///
/// This function will return an error if:
/// * The environment variables `OPEN_AI_KEY` or `OPEN_AI_ORG` are not set.
/// * There is an issue with constructing the HTTP request.
/// * The HTTP request to the OpenAI GPT API fails.
///
/// # Example
///
/// ```rust
/// use crate::apis::call_request::call_gpt;
/// use crate::models::general::llm::Message;
///
/// #[tokio::main]
/// async fn main() {
///     let message = Message {
///         role: "user".to_string(),
///         content: "Hello, how are you?".to_string(),
///     };
///     let messages = vec![message];
///
///     match call_gpt(messages).await {
///         Ok(response) => println!("Response: {}", response),
///         Err(e) => eprintln!("Error: {}", e),
///     }
/// }
/// ```
pub async fn call_gpt(messages: Vec<Message>) -> Result<String, Box<dyn std::error::Error + Send>> {
    dotenv().ok();

    // Extract API Key information
    let api_key: String =
        env::var("OPEN_AI_KEY").expect("OPEN_AI_KEY not found in environment variables");
    let api_org: String =
        env::var("OPEN_AI_ORG").expect("OPEN_AI_ORG not found in environment variables");

    // Confirm endpoint
    let url: &str = "https://api.openai.com/v1/chat/completions";

    // Create headers
    let mut headers: HeaderMap = HeaderMap::new();

    // Create OpenAI Api key header
    headers.insert(
        "authorization",
        HeaderValue::from_str(&format!("Bearer {}", api_key))
            .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?,
    );

    // Create OpenAI Org header
    headers.insert(
        "OpenAI-Organization",
        HeaderValue::from_str(api_org.as_str())
            .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?,
    );

    // Create client
    let client: Client = Client::builder()
        .default_headers(headers)
        .build()
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?;

    // Create chat completion
    let chat_completion: ChatCompletion = ChatCompletion {
        model: "gpt-4o".to_string(),
        messages,
        temperature: 0.1,
    };

    // // Troubleshooting
    // let res_raw = client
    //   .post(url)
    //   .json(&chat_completion)
    //   .send()
    //   .await
    //   .unwrap();
    // dbg!(res_raw.text().await.unwrap());

    // Extract API Response
    let res: APIResponse = client
        .post(url)
        .json(&chat_completion)
        .send()
        .await
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?
        .json()
        .await
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?;

    // Send Response
    Ok(res.choices[0].message.content.clone())
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

        let res: Result<String, Box<dyn std::error::Error + Send>> = call_gpt(messages).await;
        match res {
            Ok(res_str) => {
                dbg!(res_str);
                assert!(true);
            }
            Err(_) => {
                assert!(false);
            }
        }
    }
}
