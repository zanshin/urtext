// src/build/pipeline.rs
use crate::content::page::Page;

use std::fs;
use std::path::{Path, PathBuf};

use anyhow::Result;
use tera::{Context, Tera};
use walkdir::WalkDir;

// create site level Tera instance
pub fn build() -> Result<()> {
    let tera = Tera::new("src/site/templates/**/*")?;

    let paths = discover_content_files()?;

    for path in paths {
        let page = Page::from_file(&path, &tera)?;
        write_page(&page, &tera)?;
    }

    Ok(())
}

fn discover_content_files() -> Result<Vec<PathBuf>> {
    let files = WalkDir::new("content")
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path().is_file() && e.path().extension().and_then(|s| s.to_str()) == Some("md")
        })
        .map(|e| e.into_path())
        .collect();

    Ok(files)
}

fn write_page(page: &Page, tera: &Tera) -> Result<()> {
    let mut context = Context::new();
    context.insert("content", &page.rendered_content);
    context.insert("title", &page.front_matter.title);

    let final_html = tera.render("base.html", &context)?;

    if let Some(parent) = page.output_path.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(&page.output_path, final_html)?;

    Ok(())
}
