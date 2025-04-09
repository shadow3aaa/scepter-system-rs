pub mod ollama_adapter;
pub mod openai_adapter;

use std::{collections::VecDeque, sync::mpsc::Receiver};

use serde::{Deserialize, Serialize};

use ollama_adapter::OllamaAdapter;
use openai_adapter::OpenAIAdapter;

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

#[derive(Serialize, Deserialize, Default)]
pub struct LLMAdapters {
    current_adapter: Option<usize>,
    pub adapters: VecDeque<LLMAdapterWrapper>,
}

impl LLMAdapters {
    pub fn new() -> Self {
        Self::default()
    }

    pub const fn set_current_adapter(&mut self, index: usize) {
        self.current_adapter = Some(index);
    }

    pub fn get_current_adapter(&self) -> Option<&LLMAdapterWrapper> {
        self.adapters.get(self.current_adapter?)
    }

    pub fn chat(&self, messages: Vec<Message>) -> Option<Receiver<String>> {
        match self.get_current_adapter()? {
            LLMAdapterWrapper::Ollama(adapter) => Some(adapter.chat(messages)),
            LLMAdapterWrapper::OpenAI(adapter) => Some(adapter.chat(messages)),
        }
    }
}
