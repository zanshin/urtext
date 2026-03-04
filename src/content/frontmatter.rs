use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FrontMatter {
    pub title: String,
    pub date: chrono::NaiveDate,
    pub tags: Option<Vec<String>>,
    pub draft: Option<bool>,
    pub template: Option<String>, // override default template
}

impl FrontMatter {
    pub fn parse(content: &str) -> Result<(FrontMatter, &str)> {
        let content = content.trim_start();

        if !content.starts_with("---") {
            return Err(anyhow!("Missing front matter marker"));
        }

        let mut split = content.splitn(3, "---");

        split.next();
        let yaml = split
            .next()
            .ok_or_else(|| anyhow!("Missing YAML section"))?;
        let body = split
            .next()
            .ok_or_else(|| anyhow!("Missing body section"))?;

        let front_matter: FrontMatter = serde_yaml::from_str(yaml)?;

        Ok((front_matter, body.trim_start()))
    }
}
