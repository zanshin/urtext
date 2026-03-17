// crate/base/src/site.rs

// ABOUTME: Site configuration and settings.
// ABOUTME: Handles loading and parsing config.toml files.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum SiteConfigError {
    #[error("Failed to parse TOML: {0}")]
    TomlError(#[from] toml::de::Error),

    #[error("Missing required field: {0}")]
    MIssingFiled(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiteConfig {
    pub title: String,
    pub base_url: String,
    #[serde(default)]
    pub author: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub paths: PathConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathConfig {
    #[serde(default = "default_content_dir")]
    pub content: PathBuf,
    #[serde(default = "default_output_dir")]
    pub output: PathBuf,
    #[serde(default = "default_template_dir")]
    pub templates: PathBuf,
}

impl Default for PathConfig {
    fn default() -> Self {
        Self {
            content: default_content_dir(),
            output: default_output_dir(),
            templates: default_template_dir(),
        }
    }
}

fn default_content_dir() -> PathBuf {
    PathBuf::from("content")
}

fn default_output_dir() -> PathBuf {
    PathBuf::from("public")
}

fn default_template_dir() -> PathBuf {
    PathBuf::from("templates")
}

impl SiteConfig {
    pub fn from_toml(toml_str: &str) -> Result<Self, SiteConfigError> {
        let config: SiteConfig = toml::from_str(toml_str)?;
        Ok(config)
    }

    pub fn content_dir(&self) -> &PathBuf {
        &self.paths.content
    }

    pub fn output_dir(&self) -> &PathBuf {
        &self.paths.output
    }

    pub fn template_dir(&self) -> &PathBuf {
        &self.paths.templates
    }
}

impl Default for SiteConfig {
    fn default() -> Self {
        Self {
            title: "My Site".to_string(),
            base_url: "http://localhost:3000".to_string(),
            author: None,
            description: None,
            paths: PathConfig::default(),
        }
    }
}
