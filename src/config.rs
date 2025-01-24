use serde::{self, Serializer};
use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

fn serialize_file_mode<S>(mode: &u32, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    // Convert directly to octal string representation
    serializer.serialize_str(&format!("{:o}", mode))
}

pub const DEFAULT_FILE_MODE: u32 = 0o755; // Using octal literal for file permissions

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub artifact: HashMap<String, ArtifactConfig>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ArtifactConfig {
    pub r#type: String,
    pub bucket: String,
    pub object_key: String,
    pub dest: String,
    pub hash: String, // SHA-256 hash of the file
    #[serde(default = "default_file_mode", serialize_with = "serialize_file_mode")]
    pub file_mode: u32,
    pub file_owner: Option<String>,
    pub file_group: Option<String>,
}

fn default_file_mode() -> u32 {
    DEFAULT_FILE_MODE
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
