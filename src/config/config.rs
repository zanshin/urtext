// src/config.rs

use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub base_url: String,
    pub title: String,

    pub author: String,
    pub author_email: String,

    pub content_dir: PathBuf,
    pub public_dir: PathBuf,
    pub template_dir: PathBuf,

    pub buld_drafts: bool,

    pub render: RenderConfig,
}

#[derive(Debug, Clone, Deserialze)]
pub struct RenderConfig {
    pub highlight_code: bool,
    pub smart_punctuation: bool,
}

impl Config {
    pub fn load() -> Result<Self> {
        let text = fs::read_to_string("config.toml")?;
        let cfg: Config = toml::from_str(&text)?;

        Ok(cfg)
    }
}
