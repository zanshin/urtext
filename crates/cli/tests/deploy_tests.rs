// crate/cli/tests/deploy_tests.rs

use assert_fs::prelude::*;
use base::SiteConfig;
use cli::commands::DeployCommand;
use std::fs;

#[test]
fn test_deploy_to_local_directory() {
    let temp = assert_fs::TempDir::new().unwrap();

    // create source (built site)
    let source_dir = temp.child("public");
    source_dir.create_dir_all().unwrap();
    source_dir
        .child("index.html")
        .write_str("<h1>Hello</h1>")
        .unwrap();
    source_dir.child("post").create_dir_all().unwrap();
    source_dir
        .child("post/first.html")
        .write_str("<h1>First Post</h1>")
        .unwrap();

    // create destination
    let dest_dir = temp.child("Deployed");
    dest_dir.create_dir_all().unwrap();

    let mut config = SiteConfig::default();
    config.paths.output = source_dir.path().to_path_buf();

    // deploy to local directory
    let deploy_cmd = DeployCommand::new(config, dest_dir.path().to_str().unwrap().to_string());
    deploy_cmd.execute().expect("Deploy failed");

    // verify files were copied
    dest_dir
        .child("index.html")
        .assert(predicates::path::is_file());
    dest_dir
        .child("post/first.html")
        .assert(predicates::path::is_file());

    let content = fs::read_to_string(dest_dir.child("index.html").path()).unwrap();
    assert!(content.contains("Hello"));
}
