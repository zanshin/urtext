// crates/builder/tests/builder_tests.rs

use assert_fs::prelude::*;
use base::SiteConfig;
use builder::SiteBuilder;
use std::fs;

#[test]
fn test_build_simple_site() {
    let temp = assert_fs::TempDir::new().unwrap();

    // setup directory structure
    let content_dir = temp.child("content");
    let templates_dir = temp.child("templates");
    let output_dir = temp.child("public");

    content_dir.create_dir_all().unwrap();
    templates_dir.create_dir_all().unwrap();

    // create a simple post
    let post_content = r#"---
title: My First Post
date: 2026-03-18T10:00:00Z
tags:
  - test
---

# Hello World

This is my first post."#;

    content_dir
        .child("2026-03-18-first-post.md")
        .write_str(post_content)
        .unwrap();

    // create templates
    let base_template = r#"<!DOCTYPE html>
<html>
<head><title>{{ site.title }}</title></head>
<body>{% block content %}{% endblock %}</body>
</htm>"#;

    let post_template = r#"{% extends "base.html" %}
{% block content %}
<h1>{{ post.title }}</h1>
<div>{{ post.body | safe }}</div>
{% endblock %}"#;

    templates_dir
        .child("base.html")
        .write_str(base_template)
        .unwrap();
    templates_dir
        .child("post.html")
        .write_str(post_template)
        .unwrap();

    // create config
    let mut config = SiteConfig::default();
    config.title = "Test Site".to_string();
    config.paths.content = content_dir.path().to_path_buf();
    config.paths.output = output_dir.path().to_path_buf();
    config.paths.templates = templates_dir.path().to_path_buf();

    // build the site
    let builder = SiteBuilder::new(config);
    builder.build().expect("Build failed");

    // verify output
    output_dir.assert(predicates::path::is_dir());

    let post_output = output_dir.child("2026-03-18-first-post/index.html");
    post_output.assert(predicates::path::is_file());

    let html = fs::read_to_string(post_output.path()).unwrap();
    assert!(html.contains("<title>Test Site</title>"));
    assert!(html.contains("<h1>My First Post</h1>"));
    assert!(html.contains("Hello World"));
}
