//! Markdown parsing and HTML converson for urtext.
//! Uses pulldown-cmark for CommonMark-complian markdown processing.
//! crates/markdown/src/lib.rs

pub mod parser;

pub use parser::MarkdownParser;
