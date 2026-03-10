// tests/pipeline.rs

use std::fs;
use tempfile::tempdir;
use tera::Tera;

use urtext::build::pipeline::build_site;

fn make_tera(template_dir: &std::path::Path) -> Tera {
    Tera::new(template_dir.join("**/*").to_str().unwrap()).unwrap()
}

fn setup_dirs(
    temp: &tempfile::TempDir,
) -> (std::path::PathBuf, std::path::PathBuf, std::path::PathBuf) {
    let content_dir = temp.path().join("content");
    let output_dir = temp.path().join("public");
    let template_dir = temp.path().join("templates");

    fs::create_dir_all(&content_dir).unwrap();
    fs::create_dir_all(&template_dir).unwrap();
    fs::create_dir_all(template_dir.join("partials")).unwrap();

    fs::write(
        template_dir.join("base.html"),
        "<html><body>{{ content | safe }}</body></html>",
    )
    .unwrap();

    (content_dir, output_dir, template_dir)
}

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
