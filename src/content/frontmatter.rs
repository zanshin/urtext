// src/build/frontmatter.rs

use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FrontMatter {
    pub title: String,
    pub data: chrono::NaiveDate,
    pub tags: Option<Vec<String>>,
    pub drafts: Option<bool>,
    pub template: Option<String>,
}

impl FrontMatter {
    pub fn parse(content: &str) -> Result<(FrontMatter, &str)> {
        let content = content.trim_start();

        if !content.starts_with("---") {
            return Err(anyhow!("Missing front matter delimiter"));
        }

        let mut parts = content.splitn(3, "---");
        parts.next(); // empty string before first ---

        let yaml = parts.next().ok_or_else(|| anyhow!("Missing YAML block"))?;
        let body = parts.next().ok_or_else(|| anyhow!("Missing body"))?;

        let front_matter: FrontMatter = serde_yaml::from_str(yaml)?;

        Ok((front_matter, body.trim_start()))
    }
}
