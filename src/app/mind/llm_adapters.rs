pub mod ollama_adapter;
pub mod openai_adapter;

use std::{collections::HashMap, sync::mpsc::Receiver};

use serde::{Deserialize, Serialize};

use ollama_adapter::{OllamaAdapter, OllamaAdapterConfig};
use openai_adapter::{OpenAIAdapter, OpenAIAdapterConfig};

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

#[derive(Serialize, Deserialize)]
pub enum LLMAdapterWrapper {
    Ollama(OllamaAdapter),
    OpenAI(OpenAIAdapter),
}

pub enum LLMAdapterConfigWrapper {
    Ollama(OllamaAdapterConfig),
    OpenAI(OpenAIAdapterConfig),
}

#[derive(Serialize, Deserialize)]
pub struct LLMAdapters {
    current_adapter: Option<String>,
    adapters: HashMap<String, LLMAdapterWrapper>,
}

impl LLMAdapters {
    pub fn new() -> Self {
        Self {
            current_adapter: None,
            adapters: HashMap::new(),
        }
    }

    pub fn config(mut self, configs: Vec<(String, LLMAdapterConfigWrapper)>) -> Self {
        self.adapters = configs
            .into_iter()
            .map(|(id, config)| {
                (
                    id,
                    match config {
                        LLMAdapterConfigWrapper::Ollama(config) => {
                            LLMAdapterWrapper::Ollama(OllamaAdapter::new(config))
                        }
                        LLMAdapterConfigWrapper::OpenAI(config) => {
                            LLMAdapterWrapper::OpenAI(OpenAIAdapter::new(config).unwrap())
                        }
                    },
                )
            })
            .collect();
        self
    }

    pub fn available_adapters(&self) -> impl Iterator<Item = &String> {
        self.adapters.keys()
    }

    pub fn get_adapter(&self, id: &str) -> Option<&LLMAdapterWrapper> {
        self.adapters.get(id)
    }

    pub fn chat(&self, messages: Vec<Message>) -> Option<Receiver<String>> {
        match self.get_adapter(self.current_adapter.as_ref()?)? {
            LLMAdapterWrapper::Ollama(adapter) => {
                Some(adapter.chat(self.current_adapter.as_ref()?, messages))
            }
            LLMAdapterWrapper::OpenAI(adapter) => {
                Some(adapter.chat(self.current_adapter.as_ref()?, messages))
            }
        }
    }
}
