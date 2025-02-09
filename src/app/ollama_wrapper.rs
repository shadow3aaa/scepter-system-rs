pub struct OllamaWrapper {
    pub current_model: Option<String>,
    pub model_list: Vec<String>,
}

impl OllamaWrapper {
    pub fn new() -> Self {
        Self {
            current_model: None,
            model_list: vec!["llm-1", "llm-2", "llm-3"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        }
    }
}