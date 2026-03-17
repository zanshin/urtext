// crate/base/tests/video_post_tests.rs

use base::VideoPost;

#[test]
fn test_create_video_post_from_markdown() {
    let content = r#"---
title: Great Video
date: 2026-03-16T10:00:00Z
youtube_id: dQw4w9WgXcQ
---

Check out this amazing video!"#;

    let video_post = VideoPost::from_markdown(content).expect("Failed to parse video post");

    assert_eq!(video_post.frontmatter().title, "Great Video");
    assert_eq!(video_post.youtube_id(), "dQw4w9WgXcQ");
    assert_eq!(video_post.body(), "Check out this amazing video!");
}

#[test]
fn test_video_post_missing_youtube_id() {
    let content = r#"---
title: Video Post
date: 2026-03-16T10:00:00Z
---

No youtube_id provided"#;

    let result = VideoPost::from_markdown(content);
    assert!(result.is_err());
}

#[test]
fn test_video_post_slug_generation() {
    let content = r#"---
title: Awesome Rust Tutorial
date: 2026-03-16T10:00:00Z
youtube_id: abc123
---

Body"#;

    let video_post = VideoPost::from_markdown(content).expect("Failed to parse video post");

    assert_eq!(video_post.slug(), "awesome-rust-tutorial");
}
