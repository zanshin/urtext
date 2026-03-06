// src/build/pipeline.rs
use crate::content::page::Page;

use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use tera::Tera;

pub fn build(drafts: bool, minify: bool) {
    println!("build drafts={} minify={}", drafts, minify);
}

// create site level Tera instance
fn build_site(content_dir: &Path, output_dir: &Path, tera: &Tera) -> Result<()> {
    let paths = discover_content_files(content_dir)?;

    for path in paths {
        let page = Page::from_file(&path, tera)?;
        write_page(&page, tera, output_dir)?;
    }

    Ok(())
}

fn discover_content_files(content_dir: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    visit_dir(content_dir, &mut files)?;

    Ok(files)
}

fn visit_dir(dir: &Path, files: &mut Vec<PathBuf>) -> Result<()> {
    for entry in
        fs::read_dir(dir).with_context(|| format!("Failed to read directory: {}", dir.display()))?
    {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            visit_dir(&path, files)?;
        } else if path.extension().and_then(|s| s.to_str()) == Some("md") {
            files.push(path);
        }
    }

    Ok(())
}

fn write_page(page: &Page, tera: &Tera, output_root: &Path) -> Result<()> {
    let mut context = tera::Context::new();
    context.insert("content", &page.rendered_content);
    context.insert("title", &page.front_matter.title);

    let final_html = tera.render("base.html", &context)?;

    let full_path = output_root.join(&page.output_path);

    if let Some(parent) = full_path.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(full_path, final_html)?;

    Ok(())
}
