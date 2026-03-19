//! Site builder that orchestrates content parsing, template rendering, and output generation
//! Ties together all urtext components to generate a complete static site
//! crates/builder/src/lib/rs

pub mod builder;
pub mod pipeline;

pub use builder::SiteBuilder;
