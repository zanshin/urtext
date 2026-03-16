// crates/base/tests/frontmatter_tests.rs

use base::FrontMatter;

#[test]
fn test_parse_basic_frontmatter() {
    let yaml = r#"
title: My First Post
date: 2026-03-16T10:00:00Z
draft: false
tags:
  - rust
  - programming
"#;

    let frontmatter = FrontMatter::parse(yaml).expect("Failed to parse frontmatter");

    assert_eq!(frontmatter.title, "My First Post");
    assert!(!frontmatter.draft);
    assert_eq!(frontmatter.tags.len(), 2);
    assert_eq!(frontmatter.tags[0], "rust");
    assert_eq!(frontmatter.tags[1], "programming");
}

#[test]
fn test_parse_frontmatter_with_defaults() {
    let yaml = r#"
title: Minimal Post
date: 2026-03-16T10:00:00Z
"#;

    let frontmatter = FrontMatter::parse(yaml).expect("Failed to parse frontmatter");

    assert_eq!(frontmatter.title, "Minimal Post");
    assert!(!frontmatter.draft); // should default to false
    assert!(frontmatter.tags.is_empty()); // should default to empty
}

#[test]
fn test_parse_frontmatter_with_extras_fields() {
    let yaml = r#"
title: Link Post
date: 2026-03-16T10:00:00Z
link_url: https://example.com
youtube_id: dQw4w9WgXcQ
"#;

    let frontmatter = FrontMatter::parse(yaml).expect("Failed to parse frontmatter");

    assert_eq!(frontmatter.title, "Link Post");

    // extra fields should be accessible
    let link_url = frontmatter.get_string("link_url");
    assert_eq!(link_url, Some("https://example.com"));

    let youtube_id = frontmatter.get_string("youtube_id");
    assert_eq!(youtube_id, Some("dQw4w9WgXcQ"));
}

#[test]
fn test_parse_invalid_yaml() {
    let yaml = "invalid yaml: content";

    let result = FrontMatter::parse(yaml);
    assert!(result.is_err());
}

#[test]
fn test_parse_missing_required_fields() {
    let yaml = r#"
draft: false
"#;

    let result = FrontMatter::parse(yaml);
    assert!(result.is_err());
}
