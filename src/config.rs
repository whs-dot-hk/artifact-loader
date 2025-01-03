use serde_derive::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub artifact: HashMap<String, ArtifactConfig>,
}

#[derive(Debug, Deserialize)]
pub struct ArtifactConfig {
    pub r#type: String,
    pub bucket: String,
    pub key: String,
    pub dest: String,
}

impl Config {
    pub fn from_str(content: &str) -> anyhow::Result<Self> {
        Ok(toml::from_str(content)?)
    }

    pub fn from_file(path: impl AsRef<std::path::Path>) -> anyhow::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        Self::from_str(&content)
    }
}
