// crates/base/tests/site_config_tests.rs

use base::SiteConfig;
use std::path::PathBuf;

#[test]
fn test_default_site_config() {
    let config = SiteConfig::default();

    assert_eq!(config.title, "My Site");
    assert_eq!(config.base_url, "http://localhost:3000");
    assert_eq!(*config.content_dir(), PathBuf::from("content"));
    assert_eq!(*config.output_dir(), PathBuf::from("public"));
    assert_eq!(*config.template_dir(), PathBuf::from("templates"));
}

#[test]
fn test_parse_site_config_from_toml() {
    let toml = r#"
title = "Urtext Blog"
base_url = "https://example.com"
author = "Jane Doe"

[paths]
content = "posts"
output = "dist"
templates = "themes"
"#;

    let config = SiteConfig::from_toml(toml).expect("Failed to parse config");

    assert_eq!(config.title, "Urtext Blog");
    assert_eq!(config.base_url, "https://example.com");
    assert_eq!(config.author, Some("Jane Doe".to_string()));
    assert_eq!(*config.content_dir(), PathBuf::from("posts"));
    assert_eq!(*config.output_dir(), PathBuf::from("dist"));
    assert_eq!(*config.template_dir(), PathBuf::from("themes"));
}

#[test]
fn test_parse_minimal_config() {
    let toml = r#"
title = "My Blog"
base_url = "https://myblog.com"
"#;

    let config = SiteConfig::from_toml(toml).expect("Failed to parse config");

    assert_eq!(config.title, "My Blog");
    assert_eq!(config.base_url, "https://myblog.com");
    assert_eq!(*config.content_dir(), PathBuf::from("content"));
    assert_eq!(*config.output_dir(), PathBuf::from("public"));
}

#[test]
fn test_invalid_toml() {
    let toml = "Invalid toml content [[[";

    let result = SiteConfig::from_toml(toml);
    assert!(result.is_err());
}
