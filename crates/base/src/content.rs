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

#[derive(Debug, Clone)]
pub struct LinkPost {
    frontmatter: FrontMatter,
    body: String,
    slug: String,
    link_url: String,
}

impl LinkPost {
    pub fn from_markdown(content: &str) -> Result<Self, ContentError> {
        let (frontmatter_str, body) = parse_content(content)?;
        let frontmatter = FrontMatter::parse_require_title(&frontmatter_str)?;

        let link_url = frontmatter
            .get_string("link_url")
            .ok_or_else(|| ContentError::InvalidFormat)?
            .to_string();

        let slug = frontmatter
            .get_string("slug")
            .map(|s| s.to_string())
            .unwrap_or_else(|| slugify(&frontmatter.title));

        Ok(LinkPost {
            frontmatter,
            body,
            slug,
            link_url,
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

    pub fn link_url(&self) -> &str {
        &self.link_url
    }
}

#[derive(Debug, Clone)]
pub struct Note {
    frontmatter: FrontMatter,
    body: String,
    slug: String,
}

impl Note {
    pub fn from_markdown(content: &str) -> Result<Self, ContentError> {
        let (frontmatter_str, body) = parse_content(content)?;
        let mut frontmatter = FrontMatter::parse(&frontmatter_str)?;

        // notes can have empty titles, so set a default if missing
        if frontmatter.title.is_empty() {
            frontmatter.title = format!("Note {}", frontmatter.date.format("%Y-%m-%d %H:%M"));
        }

        let slug = frontmatter
            .get_string("slug")
            .map(|s| s.to_string())
            .unwrap_or_else(|| {
                if frontmatter.title.starts_with("Note ") {
                    // generate slug from date if title is auto-generated
                    frontmatter.date.format("%Y-%m-%d-%H&M&S").to_string()
                } else {
                    slugify(&frontmatter.title)
                }
            });

        Ok(Note {
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

#[derive(Debug, Clone)]
pub struct VideoPost {
    frontmatter: FrontMatter,
    body: String,
    slug: String,
    youtube_id: String,
}

impl VideoPost {
    pub fn from_markdown(content: &str) -> Result<Self, ContentError> {
        let (frontmatter_str, body) = parse_content(content)?;
        let frontmatter = FrontMatter::parse_require_title(&frontmatter_str)?;

        let youtube_id = frontmatter
            .get_string("youtube_id")
            .ok_or_else(|| ContentError::InvalidFormat)?
            .to_string();

        let slug = frontmatter
            .get_string("slug")
            .map(|s| s.to_string())
            .unwrap_or_else(|| slugify(&frontmatter.title));

        Ok(VideoPost {
            frontmatter,
            body,
            slug,
            youtube_id,
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

    pub fn youtube_id(&self) -> &str {
        &self.youtube_id
    }
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
