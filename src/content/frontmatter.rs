// src/build/frontmatter.rs

use anyhow::{Result, anyhow};
use chrono::NaiveDate;
use serde::{Deserialize, Deserializer, Serialize};

fn deserialize_date<'de, D: Deserializer<'de>>(d: D) -> Result<NaiveDate, D::Error> {
    let s = String::deserialize(d)?;
    NaiveDate::parse_from_str(&s, "%Y-%m-%d").map_err(serde::de::Error::custom)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FrontMatter {
    pub title: String,
    #[serde(deserialize_with = "deserialize_date")]
    pub date: chrono::NaiveDate,
    pub tags: Option<Vec<String>>,
    pub draft: Option<bool>,
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
