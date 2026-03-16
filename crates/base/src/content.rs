// ABOUTME: Content types for different kinds of posts.
// ABOUTME: Implements parsing from markdown files with frontmatter.

use crate::frontmatter::{FrontMatter, FrontMatterError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ContentError {
    #[error("Frontmatter error: {0}")]
    FrontMatter(#[from] FrontMatterError),

    #[error("Missing frontmatter delimiter")]
    MissingFrontMatter,

    #[error("Invalid content format")]
    InvalidFormat,
}

#[derive(Debug, Clone)]
pub struct Post {
    frontmatter: FrontMatter,
    body: String,
    slug: String,
}

impl Post {
    pub fn from_markdown(content: &str) -> Result<Self, ContentError> {
        let (frontmatter_str, body) = parse_content(content)?;
        let frontmatter = FrontMatter::parse_require_title(&frontmatter_str)?;

        let slug = frontmatter
            .get_string("slug")
            .map(|s| s.to_string())
            .unwrap_or_else(|| slugify(&frontmatter.title));

        Ok(Post {
            frontmatter,
            body,
            slug,
        })
    }

    pub fn frontmatter(&self) -> &FrontMatter {
        &self.frontmatter
    }

    pub fn body(&self) -> &str {
        &self.body
    }

    pub fn slug(&self) -> &str {
        &self.slug
    }
}

fn parse_content(content: &str) -> Result<(String, String), ContentError> {
    let content = content.trim();

    if !content.starts_with("---") {
        return Err(ContentError::MissingFrontMatter);
    }

    let rest = &content[3..];

    let end_pos = rest
        .find("\n---\n")
        .or_else(|| rest.find("\n---"))
        .ok_or(ContentError::MissingFrontMatter)?;

    let frontmatter = rest[..end_pos].to_string();

    let body_start = end_pos + 4;
    let body = if body_start < rest.len() {
        rest[body_start..].trim().to_string()
    } else {
        String::new()
    };

    Ok((frontmatter, body))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slugify() {
        assert_eq!(slugify("Hello World"), "hello-world");
        assert_eq!(slugify("Hello  World"), "hello-world");
        assert_eq!(slugify("Hello! World?"), "hello-world");
        assert_eq!(slugify("Test-Post"), "test-post");
    }
}
