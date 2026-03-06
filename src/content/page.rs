use crate::content::frontmatter::FrontMatter;
use crate::shortcodes::handlers::process_shortcodes;

use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use pulldown_cmark::{Options, Parser, html};
use sha2::{Digest, Sha256};
use tera::Tera;

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
    pub fn from_file(path: &Path, tera: &Tera) -> Result<Page> {
        let raw = fs::read_to_string(path)?;

        // need hash prior to parsing
        let mut hasher = Sha256::new();
        hasher.update(raw.as_bytes());
        let hash = hex::encode(hasher.finalize());

        let (front_matter, body) = FrontMatter::parse(&raw)?;

        let content_with_shortcodes = process_shortcodes(&body, &tera)?;
        let rendered_content = to_html(&content_with_shortcodes);

        let slug = slugify(front_matter.title.as_str());

        let relative = path.strip_prefix("content")?;
        let output_path = PathBuf::from("public")
            .join(relative)
            .with_extension("html");

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
        .filter_map(|ch| {
            if ch.is_ascii_alphanumeric() {
                Some(ch.to_ascii_lowercase())
            } else if ch.is_whitespace() || ch == '-' {
                Some('-')
            } else {
                None
            }
        })
        .fold(String::new(), |mut acc, ch| {
            if ch == '-' && acc.ends_with('-') {
                return acc;
            }
            acc.push(ch);
            acc
        })
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
