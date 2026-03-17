//! # base
//!
//! The `base` crate defines the content types, handles frontmatter parsing, and site configruation.
//!
//! crates/base/src/lib.rs

pub mod content;
pub mod frontmatter;
pub mod site;

pub use content::{LinkPost, Note, Post, VideoPost};
pub use frontmatter::FrontMatter;
pub use site::SiteConfig;
