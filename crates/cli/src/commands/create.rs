//! Implementation of the 'create' command to generate new content.
//! Uses archetypes to scaffold different content types.
//! crate/cli/src/commands/create.rs

use anyhow::{bail, Context, Result};
use chrono::Utc;
use std::fs;
use std::path::PathBuf;

pub struct CreateCommand {
    content_type: String,
    title: Option<String>,
    site_path: PathBuf,
}

impl CreateCommand {
    pub fn new(content_type: String, title: Option<String>, site_path: PathBuf) -> Self {
        Self {
            content_type,
            title,
            site_path,
        }
    }

    pub fn execute(&self) -> Result<PathBuf> {
        let archetype_name = self.normalize_content_type();
        let archetype_path = self
            .site_path
            .join("archetypes")
            .join(format!("{}.md", archetype_name));

        if !archetype_path.exists() {
            bail!("Unknown content type: {}", self.content_type);
        }

        let archetype_content =
            fs::read_to_string(&archetype_path).context("Failed to read archetype")?;

        let content = self.process_archetype(&archetype_content);
        let filename = self.generate_filename();
        let content_path = self.site_path.join("content").join(&filename);

        fs::write(&content_path, content).context("Failed to write content file")?;

        Ok(content_path)
    }

    fn normalize_content_type(&self) -> &str {
        match self.content_type.as_str() {
            "post" => "post",
            "link-post" | "linkpost" => "link_post",
            "note" => "note",
            "video" | "video-post" => "video_post",
            _ => &self.content_type,
        }
    }

    fn process_archetype(&self, template: &str) -> String {
        let now = Utc::now();
        let date_str = now.to_rfc3339();

        let mut content = template.to_string();

        if let Some(ref title) = self.title {
            content = content.replace("{{ title }}", title);
        }

        content = content.replace("{{ date }}", &date_str);

        content
    }

    fn generate_filename(&self) -> String {
        let now = Utc::now();
        let date_prefix = now.format("%Y-%m-%d");

        match self.normalize_content_type() {
            "note" => {
                let timestamp = now.format("%H%M%S");
                format!("{}-{}.md", date_prefix, timestamp)
            }
            _ => {
                if let Some(ref title) = self.title {
                    let slug = slugify(title);
                    format!("{}-{}.md", date_prefix, slug)
                } else {
                    format!("{}.md", date_prefix)
                }
            }
        }
    }
}

fn slugify(s: &str) -> String {
    s.to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { ' ' })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join("-")
}
