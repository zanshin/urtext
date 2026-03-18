// crates/cli/tests/create_command_tests.rs

use assert_fs::prelude::*;
use cli::commands::{CreateCommand, NewCommand};
use std::fs;

#[test]
fn test_create_post() {
    let temp = assert_fs::TempDir::new().unwrap();

    // first create a site
    let new_cmd = NewCommand::new("test-site", Some(temp.path().to_path_buf()));
    new_cmd.create_site().unwrap();

    let site_path = temp.child("test-site");

    // change to site directory and create a post
    let create_cmd = CreateCommand::new(
        "post".to_string(),
        Some("My First Post".to_string()),
        site_path.path().to_path_buf(),
    );

    let result = create_cmd.execute();
    assert!(result.is_ok());

    let post_path = result.unwrap();
    assert!(post_path.exists());

    let content = fs::read_to_string(&post_path).unwrap();
    assert!(content.contains("title: \"My First Post\""));
    assert!(content.contains("draft: true"));
}

#[test]
fn test_create_note_without_title() {
    let temp = assert_fs::TempDir::new().unwrap();

    let new_cmd = NewCommand::new("test-site", Some(temp.path().to_path_buf()));
    new_cmd.create_site().unwrap();

    let site_path = temp.child("test-site");

    let create_cmd = CreateCommand::new("note".to_string(), None, site_path.path().to_path_buf());

    let result = create_cmd.execute();
    assert!(result.is_ok());

    let note_path = result.unwrap();
    assert!(note_path.exists());

    let content = fs::read_to_string(&note_path).unwrap();
    assert!(content.contains("date:"));
}

#[test]
fn test_create_link_post() {
    let temp = assert_fs::TempDir::new().unwrap();

    let new_cmd = NewCommand::new("test-site", Some(temp.path().to_path_buf()));
    new_cmd.create_site().unwrap();

    let site_path = temp.child("test-site");

    let create_cmd = CreateCommand::new(
        "link-post".to_string(),
        Some("Interesting Link".to_string()),
        site_path.path().to_path_buf(),
    );

    let result = create_cmd.execute();
    assert!(result.is_ok());

    let link_post_path = result.unwrap();
    let content = fs::read_to_string(&link_post_path).unwrap();

    assert!(content.contains("title: \"Interesting Link\""));
    assert!(content.contains("link_url:"));
}

#[test]
fn test_create_video_post() {
    let temp = assert_fs::TempDir::new().unwrap();

    let new_cmd = NewCommand::new("test-site", Some(temp.path().to_path_buf()));
    new_cmd.create_site().unwrap();

    let site_path = temp.child("test-site");

    let create_cmd = CreateCommand::new(
        "video".to_string(),
        Some("Cool Video".to_string()),
        site_path.path().to_path_buf(),
    );

    let result = create_cmd.execute();
    assert!(result.is_ok());

    let video_path = result.unwrap();
    let content = fs::read_to_string(&video_path).unwrap();

    assert!(content.contains("title: \"Cool Video\""));
    assert!(content.contains("youtube_id:"));
}

#[test]
fn test_create_invalid_type() {
    let temp = assert_fs::TempDir::new().unwrap();

    let new_cmd = NewCommand::new("test-site", Some(temp.path().to_path_buf()));
    new_cmd.create_site().unwrap();

    let site_path = temp.child("test-site");

    let create_cmd = CreateCommand::new(
        "invalid-type".to_string(),
        Some("Title".to_string()),
        site_path.path().to_path_buf(),
    );

    let result = create_cmd.execute();
    assert!(result.is_err());
}
