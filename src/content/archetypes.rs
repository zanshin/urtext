// src/content/archetypes.rs

use std::fs;
use std::path::Path;

use anyhow::Result;
use include_dir::{Dir, include_dir};

static SITE_TEMPLATE: Dir = include_dir!("$CARGO_MANIFEST_DIR/site");

pub fn new_site(sitename: impl AsRef<Path>) -> Result<()> {
    let destination = sitename.as_ref();

    for file in SITE_TEMPLATE.files() {
        let path = destination.join(file.path());
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(path, file.contents())?;
    }

    for dir in SITE_TEMPLATE.dirs() {
        let path = destination.join(dir.path());
        fs::create_dir_all(path)?;
    }

    Ok(())
}

pub fn new_post(kind: String, title: String) -> Result<()> {
    println!("Create new post of kind {} with title {}", kind, title);

    Ok(())
}
