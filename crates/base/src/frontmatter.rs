//! Frontmatter parsing and representation for content files.
//! Handles YAML frontmatter with required fields (title, date) and optional extra fields.
//! crates/base/src/frontmatter.rs

use ::chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FrontMatterError {
    #[error("Failed to parse YAML: {0}")]
    YamlError(#[from] serde_yaml::Error),

    #[error("Missing required filed: {0}")]
    MissingField(String),

    #[error("Invalid date formate: {0}")]
    InvalideDate(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrontMatter {
    #[serde(default)]
    pub title: String,
    pub date: DateTime<Utc>,
    #[serde(default)]
    pub draft: bool,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(flatten)]
    extra: HashMap<String, serde_yaml::Value>,
}

impl FrontMatter {
    pub fn parse(yaml: &str) -> Result<Self, FrontMatterError> {
        let frontmatter: FrontMatter = serde_yaml::from_str(yaml)?;
        Ok(frontmatter)
    }

    pub fn parse_require_title(yaml: &str) -> Result<Self, FrontMatterError> {
        let frontmatter: FrontMatter = serde_yaml::from_str(yaml)?;

        if frontmatter.title.is_empty() {
            return Err(FrontMatterError::MissingField("title".to_string()));
        }

        Ok(frontmatter)
    }

    pub fn get_string(&self, key: &str) -> Option<&str> {
        self.extra.get(key).and_then(|v| v.as_str())
    }
}
