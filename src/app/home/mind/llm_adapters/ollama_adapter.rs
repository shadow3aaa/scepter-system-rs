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

#[derive(Debug, Error)]
pub enum OllamaError {
    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Ollama error: {0}")]
    ApiError(String),
}

pub type Result<T> = std::result::Result<T, OllamaError>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

impl Message {
    pub fn user(content: impl Into<String>) -> Self {
        Self {
            role: "user".to_string(),
            content: content.into(),
        }
    }

    pub fn assistant(content: impl Into<String>) -> Self {
        Self {
            role: "assistant".to_string(),
            content: content.into(),
        }
    }

    pub fn system(content: impl Into<String>) -> Self {
        Self {
            role: "system".to_string(),
            content: content.into(),
        }
    }
}

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

pub struct OllamaAdapter {
    base_url: String,
    client: Client,
}

impl OllamaAdapter {
    pub fn new(base_url: Option<String>) -> Self {
        Self {
            base_url: base_url.unwrap_or_else(|| "http://localhost:11434".to_string()),
            client: Client::new(),
        }
    }

    pub fn list_models(&self) -> Result<Vec<ModelInfo>> {
        let url = format!("{}/api/tags", self.base_url);
        let response = self.client.get(&url).send()?;
        let data: ListModelsResponse = response.json()?;
        Ok(data.models)
    }

    pub fn generate(&self, model: &str, prompt: &str) -> Receiver<String> {
        let (tx, rx) = mpsc::channel();

        let url = format!("{}/api/generate", self.base_url);
        let model_name = model.to_string();
        let prompt_text = prompt.to_string();

        thread::spawn(move || {
            let client = Client::new();
            let request = GenerateRequest {
                model: model_name,
                prompt: prompt_text,
                stream: true,
                options: None,
            };

            match client.post(&url).json(&request).send() {
                Ok(response) => {
                    Self::process_generate_response(response, &tx);
                }
                Err(err) => {
                    let _ = tx.send(format!("Error: {err}"));
                }
            }
        });

        rx
    }

    pub fn chat(&self, model: &str, messages: Vec<Message>) -> Receiver<String> {
        let (tx, rx) = mpsc::channel();

        let url = format!("{}/api/chat", self.base_url);
        let model_name = model.to_string();
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

    fn process_generate_response(response: Response, tx: &Sender<String>) {
        let reader = BufReader::new(response);

        for line in reader.lines() {
            match line {
                Ok(line) => {
                    if line.is_empty() {
                        continue;
                    }

                    if let Some(data) = line.strip_prefix("data: ") {
                        if data == "[DONE]" {
                            break;
                        }

                        match serde_json::from_str::<GenerateResponse>(data) {
                            Ok(parsed) => {
                                let _ = tx.send(parsed.response);
                                if parsed.done {
                                    break;
                                }
                            }
                            Err(_) => continue,
                        }
                    }
                }
                Err(err) => {
                    let _ = tx.send(format!("Error reading line: {err}"));
                    break;
                }
            }
        }
    }

    fn process_chat_response(response: Response, tx: &Sender<String>) {
        let reader = BufReader::new(response);

        for line in reader.lines() {
            match line {
                Ok(line) => {
                    if line.is_empty() {
                        continue;
                    }

                    // 直接对整行数据进行 JSON 解析，而不做 strip_prefix
                    match serde_json::from_str::<ChatResponse>(&line) {
                        Ok(parsed) => {
                            let _ = tx.send(parsed.message.content);
                            if parsed.done {
                                break;
                            }
                        }
                        Err(err) => {
                            eprintln!("Parse error: {err} in line: {line}");
                            continue;
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
