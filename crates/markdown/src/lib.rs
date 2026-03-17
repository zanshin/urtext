// crates/markdown/src/lib.rs

// ABOUTME: Markdown parsing and HTML converson for urtext.
// ABOUTEM: Uses pulldown-cmark for CommonMark-complian markdown processing.

pub mod parser;

pub use parser::MarkdownParser;
