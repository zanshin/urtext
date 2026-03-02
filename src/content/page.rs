use chrono::NaiveDate;
use pulldown_cmark::{Options, Parser, html};
use serde::{Deserialize, Serialize};

use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct FrontMatter {
    pub title: String,
    pub date: chrono::NaiveDate,
    pub tags: Option<Vec<String>>,
    pub draft: Option<bool>,
    pub template: Option<String>, // override default template
}

pub struct Page {
    pub front_matter: FrontMatter,
    pub raw_content: String,      // original Markdown
    pub rendered_content: String, // HTML after shortcode + MD processing
    pub slug: String,
    pub output_path: PathBuf,
    pub source_path: PathBuf,
    pub hash: String, // for incremental builds
}

// Functions needed
// parse a file into Page
// determine the output path

pub fn slugify(title: &str) -> String {
    let mut slug = String::with_capacity(title.len());
    let mut prev_dash = false;

    for ch in title.chars() {
        let ch = ch.to_ascii_lowercase();

        if ch.is_ascii_alphanumeric() {
            slug.push(ch);
            prev_dash = false;
        } else if ch.is_whitespace() || ch == '-' {
            if !prev_dash && !slug.is_empty() {
                slug.push('-');
                prev_dash = true;
            }
        }
        // everything else is dropped...
    }
    // trim trailing dash
    if slug.ends_with('-') {
        slug.pop();
    }

    slug
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
