// tests/pipeline.rs

mod common;
use common::make_tera;
use common::setup_dirs;

use std::fs;
use tempfile::tempdir;

use urtext::build::pipeline::build_site;

const YOUTUBE_NO_ID: &str = r#"---
title: YouTube
date: 2026-03-08
---
{{< youtube >}}
"#;

#[test]
fn youtube_shortcode_missing_id() {
    let temp = tempdir().unwrap();
    let (content_dir, output_dir, template_dir) = setup_dirs(&temp);

    fs::write(content_dir.join("youtube-no-id.md"), YOUTUBE_NO_ID).unwrap();

    let tera = make_tera(&template_dir);
    let result = build_site(&content_dir, &output_dir, &tera, false);

    let err = result.unwrap_err();
    assert!(err.to_string().contains("missing id"));
}

const YOUTUBE_ID: &str = r#"---
title: YouTube
date: 2026-03-08
---
{{< youtube id=abc123 >}}
"#;

#[test]
fn youtube_shortcode_id() {
    let temp = tempdir().unwrap();
    let (content_dir, output_dir, template_dir) = setup_dirs(&temp);

    fs::write(content_dir.join("youtube.md"), YOUTUBE_ID).unwrap();

    let tera = make_tera(&template_dir);
    build_site(&content_dir, &output_dir, &tera, false).unwrap();

    let output = fs::read_to_string(output_dir.join("youtube.html")).unwrap();
    assert!(output.contains("youtube-nocookie.com"));
}
