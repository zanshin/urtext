// src/content/archetypes.rs

use std::fs;
use std::io;
use std::path::PathBuf;

pub fn new_site(sitename: String) {
    println!("Creating new site named {}", sitename);

    create_subdir(sitename);

    println!("Site created");
}

pub fn new_post(kind: String, title: String) {
    println!("Create new post of kind {} with title {}", kind, title);
}

fn create_subdir(name: String) -> io::Result<PathBuf> {
    let path = PathBuf::from(name);
    fs::create_dir_all(&path)?;
    Ok(path)
}
