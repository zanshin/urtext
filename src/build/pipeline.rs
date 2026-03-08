// src/build/pipeline.rs

use crate::content::page::Page;

use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use tera::Tera;

pub fn build(drafts: bool) {
    println!("build drafts {}", drafts);

    let content_dir = Path::new("content");
    let output_dir = Path::new("public");
    let template_glob = "templates/**/*";

    let tera = Tera::new(template_glob).expect("Failed to load templates");

    build_site(content_dir, output_dir, &tera, drafts).expect("Build failed");

    println!("Build complete");
}

pub fn build_site(content_dir: &Path, output_dir: &Path, tera: &Tera, drafts: bool) -> Result<()> {
    let paths = discover_content(content_dir)?;

    for path in paths {
        let page = Page::from_file(&path, content_dir)?;

        if page.front_matter.draft.unwrap_or(false) && !drafts {
            continue;
        }

        write_page(&page, tera, output_dir)?;
    }

    Ok(())
}

fn discover_content(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    collect_markdown(dir, &mut files)?;
    Ok(files)
}

fn collect_markdown(dir: &Path, files: &mut Vec<PathBuf>) -> Result<()> {
    for entry in
        fs::read_dir(dir).with_context(|| format!("Cannot read directory: {}", dir.display()))?
    {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            collect_markdown(&path, files)?;
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

    // Use page-specified template or fall back to base.html
    let template = page.front_matter.template.as_deref().unwrap_or("base.html");

    let html = tera.render(template, &context)?;

    let full_path = output_root.join(&page.output_path);

    if let Some(parent) = full_path.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(&full_path, html)?;

    Ok(())
}
