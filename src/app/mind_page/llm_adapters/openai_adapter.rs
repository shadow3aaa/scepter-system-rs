use std::{
    io::{BufRead, BufReader},
    sync::mpsc::{self, Receiver, Sender},
    thread,
};

use reqwest::blocking::{Client, Response};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use super::Message;

#[derive(Debug, Error)]
pub enum OpenAIError {
    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("API key not provided")]
    ApiKeyMissing,

    #[error("API error: {0}")]
    ApiError(String), // For errors returned by the OpenAI API itself
}

pub type Result<T> = std::result::Result<T, OpenAIError>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelInfo {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub owned_by: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListModelsResponse {
    pub object: String,
    pub data: Vec<ModelInfo>,
}

// For chat requests
#[derive(Debug, Serialize)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<Message>,
    pub stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
}

#[derive(Debug, Deserialize)]
pub struct ChatCompletionChunk {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
pub struct Choice {
    pub index: u32,
    pub delta: Delta,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Delta {
    pub role: Option<String>,
    pub content: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct OpenAIAdapter {
    pub model: String,
    pub base_url: String,
    pub api_key: String,
}

impl OpenAIAdapter {
    pub fn new(model: String, base_url: Option<String>, api_key: String) -> Self {
        Self {
            model,
            base_url: base_url.unwrap_or_else(|| "https://api.openai.com/v1".to_string()),
            api_key,
        }
    }

    pub fn chat(&self, messages: Vec<Message>) -> Receiver<String> {
        let (tx, rx) = mpsc::channel();

        let url = format!("{}/chat/completions", self.base_url);
        let model_name = self.model.clone();
        let messages_clone = messages;
        let client = Client::new();
        let api_key = self.api_key.clone();

        thread::spawn(move || {
            let request = ChatRequest {
                model: model_name,
                messages: messages_clone,
                stream: true,
                temperature: None,
                top_p: None,
            };

            match client
                .post(&url)
                .bearer_auth(&api_key)
                .json(&request)
                .send()
            {
                Ok(response) => {
                    if response.status().is_success() {
                        Self::process_chat_response(response, &tx);
                    } else {
                        let status = response.status();
                        let error_body = response
                            .text()
                            .unwrap_or_else(|_| "Failed to read error body".to_string());
                        eprintln!("OpenAI API Error: Status {status}, Body: {error_body}");
                        let _ = tx.send(format!("Error: API request failed with status {status}"));
                    }
                }
                Err(err) => {
                    eprintln!("Error sending request to OpenAI: {err}");
                    let _ = tx.send(format!("Error: {err}"));
                }
            }
        });

        rx
    }

    fn process_chat_response(response: Response, tx: &Sender<String>) {
        let reader = BufReader::new(response);

        for line in reader.lines() {
            match line {
                Ok(line) => {
                    if let Some(data) = line.strip_prefix("data: ") {
                        if data.trim() == "[DONE]" {
                            break;
                        }
                        match serde_json::from_str::<ChatCompletionChunk>(data) {
                            Ok(chunk) => {
                                if let Some(choice) = chunk.choices.first() {
                                    if let Some(content) = &choice.delta.content {
                                        let _ = tx.send(content.clone());
                                    }
                                }
                                if chunk
                                    .choices
                                    .first()
                                    .is_some_and(|c| c.finish_reason.is_some())
                                {
                                    break;
                                }
                            }
                            Err(err) => {
                                eprintln!(
                                    "Error parsing OpenAI stream chunk: {err}, line: '{data}'"
                                );
                            }
                        }
                    } else if !line.trim().is_empty() {
                        eprintln!("Received non-data line from OpenAI stream: {line}");
                    }
                }
                Err(err) => {
                    eprintln!("Error reading line from OpenAI stream: {err}");
                    let _ = tx.send(format!("Error reading stream: {err}"));
                    break;
                }
            }
        }
    }
}
