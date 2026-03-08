// src/content/page.rs

use crate::content::frontmatter::FrontMatter;

use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Result;
use pulldown_cmark::{Options, Parser, html};
use sha2::{Digest, Sha256};

pub struct Page {
    pub front_matter: FrontMatter,
    pub raw_content: String,
    pub rendered_content: String,
    pub output_path: PathBuf,
    pub source_path: PathBuf,
    pub hash: String,
}

impl Page {
    pub fn from_file(path: &Path, content_root: &Path) -> Result<Page> {
        let raw = fs::read_to_string(path)?;

        let mut hasher = Sha256::new();
        hasher.update(raw.as_bytes());
        let hash = hex::encode(hasher.finalize());

        let (front_matter, body) = FrontMatter::parse(&raw)?;
        let rendered_content = markdown_to_html(body);

        let relative = path.strip_prefix(content_root)?;
        let output_path = relative.with_extension("html");

        Ok(Page {
            front_matter,
            raw_content: body.to_string(),
            rendered_content,
            output_path,
            source_path: path.to_path_buf(),
            hash,
        })
    }
}

pub fn markdown_to_html(input: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_STRIKETHROUGH);

    let parser = Parser::new_ext(input, options);
    let mut output = String::new();
    html::push_html(&mut output, parser);

    output
}
