// crates/cli/test/new_command_tests.rs

use assert_fs::prelude::*;
use cli::commands::NewCommand;

#[test]
fn test_new_command_creates_directory_structure() {
    let temp = assert_fs::TempDir::new().unwrap();
    let site_path = temp.child("my-site");

    let cmd = NewCommand::new("my-site", Some(temp.path().to_path_buf()));
    cmd.create_site().expect("Failed to create new site");

    site_path.assert(predicates::path::is_dir());
    site_path
        .child("config.toml")
        .assert(predicates::path::is_file());
    site_path
        .child("content")
        .assert(predicates::path::is_dir());
    site_path
        .child("templates")
        .assert(predicates::path::is_dir());
    site_path.child("public").assert(predicates::path::is_dir());
}

#[test]
fn test_new_command_creates_default_config() {
    let temp = assert_fs::TempDir::new().unwrap();
    let site_path = temp.child("test-site");

    let cmd = NewCommand::new("test-site", Some(temp.path().to_path_buf()));
    cmd.create_site().expect("Failed to create new site");

    let config_content = std::fs::read_to_string(site_path.child("config.toml").path())
        .expect("Failed to read config.toml");

    assert!(config_content.contains("title = \"My Urtext Site\""));
    assert!(config_content.contains("[paths]"));
}

#[test]
fn test_new_command_creates_default_templates() {
    let temp = assert_fs::TempDir::new().unwrap();
    let site_path = temp.child("my-site");

    let cmd = NewCommand::new("my-site", Some(temp.path().to_path_buf()));
    cmd.create_site().expect("Failed to create new site");

    site_path
        .child("templates/index.html")
        .assert(predicates::path::is_file());
    site_path
        .child("templates/post.html")
        .assert(predicates::path::is_file());
    site_path
        .child("templates/note.html")
        .assert(predicates::path::is_file());
}

#[test]
fn test_new_command_creates_archetypes() {
    let temp = assert_fs::TempDir::new().unwrap();
    let site_path = temp.child("my-site");

    let cmd = NewCommand::new("my-site", Some(temp.path().to_path_buf()));
    cmd.create_site().expect("Failed to create new site");

    site_path
        .child("archetypes")
        .assert(predicates::path::is_dir());
    site_path
        .child("archetypes/post.md")
        .assert(predicates::path::is_file());
    site_path
        .child("archetypes/note.md")
        .assert(predicates::path::is_file());
}

#[test]
fn test_new_command_fails_if_directory_exists() {
    let temp = assert_fs::TempDir::new().unwrap();
    let site_path = temp.child("existing-site");

    std::fs::create_dir(site_path.path()).unwrap();

    let cmd = NewCommand::new("existing-site", Some(temp.path().to_path_buf()));
    let result = cmd.create_site();

    assert!(result.is_err());
}
