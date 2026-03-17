//! Markdown parser that converts markdown to HTML.
//! Handles code blocks, lists, links, images, and standard markdown features.
//! crates/markdown/src/parser.rs

use pulldown_cmark::{html, Options, Parser};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MarkdownError {
    #[error("Failed to parse markdown")]
    ParseError,
}

pub struct MarkdownParser {
    options: Options,
}

impl MarkdownParser {
    pub fn new() -> Self {
        let mut options = Options::empty();
        options.insert(Options::ENABLE_STRIKETHROUGH);
        options.insert(Options::ENABLE_TABLES);
        options.insert(Options::ENABLE_FOOTNOTES);
        options.insert(Options::ENABLE_TASKLISTS);
        options.insert(Options::ENABLE_SMART_PUNCTUATION);

        Self { options }
    }

    pub fn parse(&self, markdown: &str) -> Result<String, MarkdownError> {
        // process shortcodes later
        // let processed = self.shortcode_processor.process(markdown);

        let parser = Parser::new_ext(&markdown, self.options);
        let mut html_output = String::new();
        html::push_html(&mut html_output, parser);
        Ok(html_output)
    }
}

impl Default for MarkdownParser {
    fn default() -> Self {
        Self::new()
    }
}
