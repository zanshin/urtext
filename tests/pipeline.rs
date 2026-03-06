use urtext::build::pipeline::build;

use std::fs;
use tempfile::tempdir;
use tera::Tera;

#[test]
fn builds_single_markdown_file() {
    let temp = tempdir().unwrap();

    let content_dir = temp.path().join("content");
    let output_dir = temp.path().join("public");
    let template_dir = temp.path().join("templates");

    fs::create_dir_all(&content_dir).unwrap();
    fs::create_dir_all(&template_dir).unwrap();

    // write test markdown
    fs::write(
        content_dir.join("test.md"),
        r#"---
        title: "Test Page"
        date: 2026-03-04
        ---
        Hello World!
        "#,
    )
    .unwrap();

    // write base template
    fs::write(
        template_dir.join("base.html"),
        "<html><body>{{ content | safe }}</body></html>,",
    )
    .unwrap();

    // write youtube partial
    fs::create_dir_all(template_dir.join("partials")).unwrap();
    fs::write(
        template_dir.join("partials/youtube.html"),
        "<div>{{ id }}</div>",
    )
    .unwrap();

    // let tera = Tera::new(template_dir.join("**/*").to_str().unwrap()).unwrap();

    // build(&content_dir, &output_dir, &tera).unwrap();
    build(true, true);

    let output_file = output_dir.join("test.html");

    assert!(output_file.exists());

    let output = fs::read_to_string(output_file).unwrap();
    assert!(output.contains("Hello World"));
}

#[test]
fn builds_with_youtube_shortcode() {
    let temp = tempdir().unwrap();

    let content_dir = temp.path().join("content");
    let output_dir = temp.path().join("public");
    let template_dir = temp.path().join("templates");

    fs::create_dir_all(&content_dir).unwrap();
    fs::create_dir_all(template_dir.join("partials")).unwrap();

    fs::write(
        content_dir.join("video.md"),
        r#"---
title: "Video Page"
date: 2026-03-04
---
{{< youtube id=abc123 >}}
"#,
    )
    .unwrap();

    fs::write(template_dir.join("base.html"), "{{ content | safe }}").unwrap();

    fs::write(
        template_dir.join("partials/youtube.html"),
        "<iframe>{{ id }}</iframe>",
    )
    .unwrap();

    let tera = Tera::new(template_dir.join("**/*").to_str().unwrap()).unwrap();

    build(&content_dir, &output_dir, &tera).unwrap();

    let output = fs::read_to_string(output_dir.join("video.html")).unwrap();

    assert!(output.contains("<iframe>abc123</iframe>"));
}
