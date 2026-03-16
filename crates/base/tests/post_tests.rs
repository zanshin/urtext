// crate/base/tests/post_tests.rs

use base::Post;

#[test]
fn test_create_post_from_markdown() {
    let content = r#"---
title: My First Post
date: 2026-03-16T10:00:00Z
tags:
  - rust
  - programming
---

# Hello World

This is my first post."#;

    let post = Post::from_markdown(content).expect("Failed to parse post");

    assert_eq!(post.frontmatter().title, "My First Post");
    assert_eq!(post.body(), "# Hello World\n\nThis is my first post.");
}

#[test]
fn test_generate_slug_from_title() {
    let content = r#"---
title: Hello World! This is a test
date: 2026-03-16T10:00:00Z
---

Content here"#;

    let post = Post::from_markdown(content).expect("Failed to parse post");

    assert_eq!(post.slug(), "hello-world-this-is-a-test");
}

#[test]
fn test_custom_slug_in_frontmatter() {
    let content = r#"---
title: My Post
date: 2026-03-16T10:00:00Z
slug: custom-slug-here
---

Content"#;

    let post = Post::from_markdown(content).expect("Failed to parse post");

    assert_eq!(post.slug(), "custom-slug-here");
}

#[test]
fn test_missing_frontmatter_delimiter() {
    let content = "Just some text without frontmatter";

    let result = Post::from_markdown(content);
    assert!(result.is_err());
}

#[test]
fn test_empty_body() {
    let content = r#"---
title: Empty Post
date: 2026-03-16T10:00:00Z
---"#;

    let post = Post::from_markdown(content).expect("Failed to parse post");

    assert_eq!(post.body(), "");
}
