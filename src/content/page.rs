use chrono::NaiveDate;
use pulldown_cmark::{Options, Parser, html};
use serde::{Deserialize, Serialize};

use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, anyhow};
use sha2::{Digest, Sha256};

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

        // check for front matter marker
        if !content.starts_with("---") {
            return Err(anyhow!("Missing from matter marker '---' at start"));
        }

        // find end of front matter
        let mut lines = content.lines();

        // skip first marker
        lines.next();

        let mut front_matter_end = 0;
        for (i, line) in content.lines().enumerate().skip(1) {
            if line.trim() == "---" {
                // end of front matter
                front_matter_end = i;
                break;
            }
        }

        if front_matter_end == 0 {
            return Err(anyhow!("Front matter not terminated with '---'"));
        }

        // compute byte indices for slicing
        let mut iter = content.lines();
        // Skip first '---'
        iter.next();
        let byte_start = iter.next().map_or(0, |l| content.find(l).unwrap_or(0));
        let mut byte_end = byte_start;
        for (i, line) in content.lines().enumerate().skip(1) {
            if i >= front_matter_end {
                break;
            }
            byte_end += line.len() + 1; // +1 for newline
        }

        let front_matter_str = &content[byte_start..byte_end];
        let front_matter: FrontMatter = serde_yaml::from_str(front_matter_str)
            .map_err(|e| anyhow!("Failed to parse front matter YAML: {}", e))?;

        // Slice remaining body
        let body_lines: Vec<&str> = content.lines().skip(front_matter_end + 1).collect();
        let remaining_body = if body_lines.is_empty() {
            ""
        } else {
            let start_index = content.find(body_lines[0]).unwrap();
            &content[start_index..]
        };

        Ok((front_matter, remaining_body))
    }
}

pub struct Page {
    pub front_matter: FrontMatter,
    pub raw_content: String,      // original Markdown - borrowed slice
    pub rendered_content: String, // HTML after shortcode + MD processing
    pub slug: String,
    pub output_path: PathBuf,
    pub source_path: PathBuf,
    pub hash: String, // for incremental builds
}

impl Page {
    pub fn from_file(path: &Path) -> Result<Page> {
        let raw = fs::read_to_string(path)
            .with_context(|| format!("Failed to read file: {}", path.display()))?;

        // need hash prior to parsing
        let mut hasher = Sha256::new();
        hasher.update(raw.as_bytes());
        let hash = hex::encode(hasher.finalize());

        let (front_matter, body) = FrontMatter::parse(&raw)?;

        let rendered_content = to_html(body);

        let slug = slugify(front_matter.title.as_str());

        let relative = path.strip_prefix("content")?;
        let output_path = PathBuf::from("public").join(relative).join("html");

        Ok(Page {
            front_matter,
            raw_content: body.to_string(),
            rendered_content,
            slug,
            output_path,
            source_path: path.to_path_buf(),
            hash,
        })
    }
}

// Functions needed
// parse a file into Page
// determine the output path

pub fn slugify(title: &str) -> String {
    title
        .chars()
        // normalize characters
        .map(|ch| {
            if ch.is_ascii_alphanumeric() {
                ch.to_ascii_lowercase()
            } else if ch.is_whitespace() || ch == '-' {
                '-'
            } else {
                '\0' // sentinel for "drop this"
            }
        })
        // remove unwanted characters
        .filter(|&ch| ch != '\0')
        // collapse consecutive dashes
        .fold(String::new(), |mut acc, ch| {
            if ch == '-' && acc.ends_with('-') {
                return acc;
            }
            acc.push(ch);
            acc
        })
        // trim leading/trailing dash
        .trim_matches('-')
        .to_string()
}

pub fn to_html(input: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_STRIKETHROUGH);

    let parser = Parser::new_ext(input, options);

    let mut output = String::new();
    html::push_html(&mut output, parser);

    output
}
