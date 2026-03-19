// crates/templates/tests/engine_tests.rs

use base::{Post, SiteConfig};
use std::fs;
use tempfile::TempDir;
use templates::{TemplateContext, TemplateEngine};

#[test]
fn test_render_simple_template() {
    let temp = TempDir::new().unwrap();
    let template_path = temp.path().join("test.html");

    fs::write(&template_path, "<h1>{{ title }}</h1>").unwrap();

    let engine = TemplateEngine::new(temp.path()).expect("Failed to create engine");

    let mut context = TemplateContext::new();
    context.insert("title", "Hello World");

    let rendered = engine
        .render("test.html", &context)
        .expect("Failed to render");

    assert_eq!(rendered.trim(), "<h1>Hello World</h1>");
}

#[test]
fn test_render_post_template() {
    let temp = TempDir::new().unwrap();
    let template_path = temp.path().join("post.html");

    let template = r#"<article>
<h1>{{ post.title }}</h1>
<time>{{ post.date }}</time>
<div>{{ post.body }}</div
</article>"#;

    fs::write(&template_path, template).unwrap();

    let engine = TemplateEngine::new(temp.path()).expect("Failed to create engine");

    let post_content = r#"---
title: Test Post
date: 2026-03-21T10:00:00Z
---

This is the body."#;

    let post = Post::from_markdown(post_content).expect("Failed to parse post");

    let context = TemplateContext::from_post(&post, &SiteConfig::default());

    let rendered = engine
        .render("post.html", &context)
        .expect("Failed to render");

    assert!(rendered.contains("<h1>Test Post</h1>"));
    assert!(rendered.contains("This is the body."));
}

#[test]
fn test_render_with_site_config() {
    let temp = TempDir::new().unwrap();
    let template_dir = temp.path().join("base.html");

    fs::write(&template_dir, "<title>{{ site.title }}</title>").unwrap();

    let engine = TemplateEngine::new(temp.path()).expect("Failed to create engine");

    let mut config = SiteConfig::default();
    config.title = "My Test Site".to_string();

    let context = TemplateContext::from_site(&config);

    let rendered = engine
        .render("base.html", &context)
        .expect("Failed to render");

    assert!(rendered.contains("title>My Test Site</title>"));
}

#[test]
fn test_template_inheritance() {
    let temp = TempDir::new().unwrap();

    let base = r#"<html>
<head><title>{{ site.title }}</title></head>
<body>{% block content %}{% endblock %}</body>
</html>"#;

    let child = r#"{% extends "base.html" %}
{% block content %}<h1>{{ title }}</h1>{% endblock %}"#;

    fs::write(temp.path().join("base.html"), base).unwrap();
    fs::write(temp.path().join("child.html"), child).unwrap();

    let engine = TemplateEngine::new(temp.path()).expect("Failed to create engine");

    let mut context = TemplateContext::new();
    context.insert("site", &SiteConfig::default());
    context.insert("title", "Page Title");

    let rendered = engine
        .render("child.html", &context)
        .expect("Failed to render");

    assert!(rendered.contains("<html>"));
    assert!(rendered.contains("<h1>Page Title</h1>"));
}

#[test]
fn test_template_not_found() {
    let temp = TempDir::new().unwrap();
    let engine = TemplateEngine::new(temp.path()).expect("Failed to create engine");

    let context = TemplateContext::new();
    let result = engine.render("nonexistent.html", &context);

    assert!(result.is_err());
}
