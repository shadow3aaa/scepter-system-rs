use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
    sync::mpsc::{self, Receiver, Sender},
    thread,
};

use reqwest::blocking::{Client, Response};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;

use super::Message;

#[derive(Debug, Error)]
pub enum OllamaError {
    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, OllamaError>;

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelInfo {
    pub name: String,
    pub modified_at: String,
    pub size: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListModelsResponse {
    pub models: Vec<ModelInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenerateRequest {
    pub model: String,
    pub prompt: String,
    pub stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<HashMap<String, Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<Message>,
    pub stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<HashMap<String, Value>>,
}

#[derive(Debug, Deserialize)]
pub struct GenerateResponse {
    pub model: String,
    pub created_at: String,
    pub response: String,
    pub done: bool,
}

#[derive(Debug, Deserialize)]
pub struct ChatResponse {
    pub model: String,
    pub created_at: String,
    pub message: Message,
    pub done: bool,
}

#[derive(Serialize, Deserialize)]
pub struct OllamaAdapter {
    pub model: String,
    pub base_url: String,
}

impl OllamaAdapter {
    pub fn new(model: String, base_url: Option<String>) -> Self {
        Self {
            model,
            base_url: base_url.unwrap_or_else(|| "http://localhost:11434".to_string()),
        }
    }

    pub fn chat(&self, messages: Vec<Message>) -> Receiver<String> {
        let (tx, rx) = mpsc::channel();

        let url = format!("{}/api/chat", self.base_url);
        let model_name = self.model.clone();
        let messages_clone = messages;

        thread::spawn(move || {
            let client = Client::new();
            let request = ChatRequest {
                model: model_name,
                messages: messages_clone,
                stream: true,
                options: None,
            };

            match client.post(&url).json(&request).send() {
                Ok(response) => {
                    Self::process_chat_response(response, &tx);
                }
                Err(err) => {
                    eprintln!("Error: {err}");
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
                    if line.is_empty() {
                        continue;
                    }

                    match serde_json::from_str::<ChatResponse>(&line) {
                        Ok(parsed) => {
                            let _ = tx.send(parsed.message.content);
                            if parsed.done {
                                break;
                            }
                        }
                        Err(err) => {
                            eprintln!("Parse error: {err} in line: {line}");
                        }
                    }
                }
                Err(err) => {
                    eprintln!("Error reading line: {err}");
                    let _ = tx.send(format!("Error reading line: {err}"));
                    break;
                }
            }
        }
    }
}
