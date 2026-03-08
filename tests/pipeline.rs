// tests/pipeline.rs

use urtext::build::pipeline::build_site;

use std::fs;
use tempfile::tempdir;
use tera::Tera;

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

// --- Basic rendering ---
const HELLO_MD: &str = r#"---
title: Hello
date: 2026-03-08
---
Hello Urtext
"#;

#[test]
fn builds_single_markdown_file() {
    let temp = tempdir().unwrap();
    let (content_dir, output_dir, template_dir) = setup_dirs(&temp);

    fs::write(content_dir.join("hello.md"), HELLO_MD).unwrap();

    let tera = make_tera(&template_dir);
    build_site(&content_dir, &output_dir, &tera, false).unwrap();

    let output = fs::read_to_string(output_dir.join("hello.html")).unwrap();
    assert!(output.contains("Hello Urtext"));
}

const MY_POST: &str = r#"---
title: My Post
date: 2026-03-08
---
Content here.
"#;

#[test]
fn output_filename_matches_input() {
    let temp = tempdir().unwrap();
    let (content_dir, output_dir, template_dir) = setup_dirs(&temp);

    fs::write(content_dir.join("my-post.md"), MY_POST).unwrap();

    let tera = make_tera(&template_dir);
    build_site(&content_dir, &output_dir, &tera, false).unwrap();

    assert!(output_dir.join("my-post.html").exists());
}

const POST: &str = r#"---
title: Post
date: 2026-03-08
---
## A Heading

Some **bold** test.
"#;

#[test]
fn markdown_is_rendered_to_html() {
    let temp = tempdir().unwrap();
    let (content_dir, output_dir, template_dir) = setup_dirs(&temp);

    fs::write(content_dir.join("post.md"), POST).unwrap();

    let tera = make_tera(&template_dir);
    build_site(&content_dir, &output_dir, &tera, false).unwrap();

    let output = fs::read_to_string(output_dir.join("post.html")).unwrap();
    assert!(output.contains("<h2>A Heading</h2>"));
    assert!(output.contains("<strong>bold</strong"));
}

// Draft handling
const DRAFT_POST: &str = r#"---
title: Draft Post
date: 2026-03-08
draft: true
---
Secret content.
"#;

#[test]
fn draft_posts_excluded_by_default() {
    let temp = tempdir().unwrap();
    let (content_dir, output_dir, template_dir) = setup_dirs(&temp);

    fs::write(content_dir.join("draft.md"), DRAFT_POST).unwrap();

    let tera = make_tera(&template_dir);
    build_site(&content_dir, &output_dir, &tera, false).unwrap();

    assert!(!output_dir.join("draft.html").exists());
}

#[test]
fn draft_posts_included_when_flag_set() {
    let temp = tempdir().unwrap();
    let (content_dir, output_dir, template_dir) = setup_dirs(&temp);

    fs::write(content_dir.join("draft.md"), DRAFT_POST).unwrap();

    let tera = make_tera(&template_dir);
    build_site(&content_dir, &output_dir, &tera, true).unwrap();

    assert!(output_dir.join("draft.html").exists());
}

const PUBLISHED_MD: &str = r#"---
title: Published Post
date: 2026-03-08
draft: false
---
Published content.
"#;

#[test]
fn non_draft_posts_always_included() {
    let temp = tempdir().unwrap();
    let (content_dir, output_dir, template_dir) = setup_dirs(&temp);

    fs::write(content_dir.join("published.md"), PUBLISHED_MD).unwrap();

    let tera = make_tera(&template_dir);
    build_site(&content_dir, &output_dir, &tera, false).unwrap();

    assert!(output_dir.join("published.html").exists());
}

// --- Directory structure ---
const NESTED_POST: &str = r#"---
title: Nested Post
date: 2026-03-08
---
Nested content.
"#;

#[test]
fn builds_nested_content_directory() {
    let temp = tempdir().unwrap();
    let (content_dir, output_dir, template_dir) = setup_dirs(&temp);

    let subdir = content_dir.join("blog");
    fs::create_dir_all(&subdir).unwrap();

    fs::write(subdir.join("nested.md"), NESTED_POST).unwrap();

    let tera = make_tera(&template_dir);
    build_site(&content_dir, &output_dir, &tera, false).unwrap();

    assert!(output_dir.join("blog/nested.html").exists());
}

#[test]
fn ignores_non_markdown_files() {
    let temp = tempdir().unwrap();
    let (content_dir, output_dir, template_dir) = setup_dirs(&temp);

    fs::write(content_dir.join("image.png"), b"fake image date").unwrap();
    fs::write(content_dir.join("note.txt"), "just a note").unwrap();

    let tera = make_tera(&template_dir);
    build_site(&content_dir, &output_dir, &tera, false).unwrap();

    assert!(!output_dir.join("image.html").exists());
    assert!(!output_dir.join("note.html").exists());
}

// --- Template selection ---

const TEMPLATE_POST: &str = r#"---
title: Custom Template Post
date: 2026-03-08
template: custom.html
---
Body text.
"#;

#[test]
fn uses_custom_template_from_front_matter() {
    let temp = tempdir().unwrap();
    let (content_dir, output_dir, template_dir) = setup_dirs(&temp);

    fs::write(
        template_dir.join("custom.html"),
        "<custom>{{ content | safe }}</custom>",
    )
    .unwrap();

    fs::write(content_dir.join("post.md"), TEMPLATE_POST).unwrap();

    let tera = make_tera(&template_dir);
    build_site(&content_dir, &output_dir, &tera, false).unwrap();

    let output = fs::read_to_string(output_dir.join("post.html")).unwrap();
    assert!(output.contains("<custom>"));
}

// --- Error cases ---

#[test]
fn returns_error_for_missing_front_matter() {
    let temp = tempdir().unwrap();
    let (content_dir, output_dir, template_dir) = setup_dirs(&temp);

    fs::write(content_dir.join("bad.md"), "No front matter here.").unwrap();

    let tera = make_tera(&template_dir);
    let result = build_site(&content_dir, &output_dir, &tera, false);

    assert!(result.is_err());
}

#[test]
fn returns_error_for_empty_content_directory() {
    let temp = tempdir().unwrap();
    let (_, output_dir, template_dir) = setup_dirs(&temp);
    let empty_dir = temp.path().join("empty");
    fs::create_dir_all(&empty_dir).unwrap();

    let tera = make_tera(&template_dir);
    let result = build_site(&empty_dir, &output_dir, &tera, false);

    // Empty dir is not an error - just produces no output
    assert!(result.is_ok());
    assert!(!output_dir.exists());
}
