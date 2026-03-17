// crates/base/tests/link_post_tests.rs

use base::LinkPost;

#[test]
fn test_create_link_post_from_markdown() {
    let content = r#"---
title: Interesting Article
date: 2026-03-16T10:00:00Z
link_url: https://example.com/article
---

This is a great article about Rust."#;

    let link_post = LinkPost::from_markdown(content).expect("Failed to parse link post");

    assert_eq!(link_post.frontmatter().title, "Interesting Article");
    assert_eq!(link_post.link_url(), "https://example.com/article");
    assert_eq!(link_post.body(), "This is a great article about Rust.");
}

#[test]
fn test_link_post_missing_url() {
    let content = r#"---
title: Missing Link
date: 2026-03-16T10:00:00Z
---

Content without link_url"#;

    let result = LinkPost::from_markdown(content);
    assert!(result.is_err());
}

#[test]
fn test_link_post_slug_generation() {
    let content = r#"---
title: Great Link Post!
date: 2026-03-16T10:00:00Z
link_url: https://example.com
---

Body"#;

    let link_post = LinkPost::from_markdown(content).expect("Failed to parse link post");

    assert_eq!(link_post.slug(), "great-link-post");
}

#[test]
fn test_link_post_custom_slug() {
    let content = r#"---
title: Link Post
date: 2026-03-16T10:00:00Z
link_url: https://example.com
slug: my-custom-slug
---

Body"#;

    let link_post = LinkPost::from_markdown(content).expect("Failed to parse link post");

    assert_eq!(link_post.slug(), "my-custom-slug");
}
