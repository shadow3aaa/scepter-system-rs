pub mod params;

use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use drama_llama::{Engine, VocabKind};
use params::Params;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Llama {
    #[serde(skip)]
    engine: Option<Engine>,
    #[serde(skip)]
    current_model: Option<(PathBuf, ModelInfo)>,
    models: HashMap<PathBuf, ModelInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub name: String,
    pub desc: String,
}

impl Llama {
    pub fn new() -> Self {
        Self {
            engine: None,
            current_model: None,
            models: HashMap::new(),
        }
    }

    pub const fn loaded(&self) -> bool {
        self.engine.is_some()
    }

    pub fn load_model(&mut self, path: impl AsRef<Path>, params: Params) {
        self.engine = Some(
            Engine::new(
                path.as_ref().to_path_buf(),
                Some(params.into()),
                Some(params.into()),
                None,
                Some(VocabKind::Unsafe),
            )
            .unwrap(),
        );

        let model_info = ModelInfo {
            name: self.get_current_model_name().unwrap(),
            desc: self.engine.as_ref().unwrap().model.desc(),
        };
        self.models
            .insert(path.as_ref().to_path_buf(), model_info.clone());

        self.current_model = Some((path.as_ref().to_path_buf(), model_info));
    }

    pub fn model_desc(&self) -> Option<String> {
        self.engine.as_ref().map(|engine| engine.model.desc())
    }

    pub fn is_empty(&self) -> bool {
        self.models.is_empty()
    }

    pub fn models(&mut self) -> impl IntoIterator<Item = (&PathBuf, &ModelInfo)> {
        self.models.retain(|path, _| path.exists());
        self.models.iter()
    }

    pub fn current_model(&self) -> Option<(&Path, &ModelInfo)> {
        self.current_model
            .as_ref()
            .map(|(p, info)| (p.as_path(), info))
    }

    pub fn get_model_info(&self, path: &Path) -> Option<&ModelInfo> {
        self.models.get(path)
    }

    pub fn get_current_model_name(&self) -> Option<String> {
        self.engine
            .as_ref()?
            .model
            .meta()
            .get("general.name")
            .map(std::string::ToString::to_string)
    }
}
