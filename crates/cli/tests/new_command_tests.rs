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
