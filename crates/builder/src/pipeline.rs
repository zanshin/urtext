//! Build pipeline for processing content through markdown and templates.
//! Handles the flow: content file -> parsed content -> markdown -> template -> HTML output

use anyhow::{Context, Result};
use base::{Post, SiteConfig};
use markdown::MarkdownParser;
use serde_json::json;
use std::fs;
use std::path::Path;
use templates::{TemplateContext, TemplateEngine};

pub struct BuildPipeline {
    config: SiteConfig,
    markdown_parser: MarkdownParser,
    template_engine: TemplateEngine,
}

impl BuildPipeline {
    pub fn new(config: &SiteConfig) -> Result<Self> {
        let template_engine = TemplateEngine::new(config.template_dir())
            .context("Failed to intiialize template engine")?;

        Ok(Self {
            config: config.clone(),
            markdown_parser: MarkdownParser::new(),
            template_engine,
        })
    }

    pub fn process_content(&self, content_path: &Path, relative_path: &Path) -> Result<()> {
        // read content file
        let content = fs::read_to_string(content_path)
            .with_context(|| format!("Failed to read {:?}", content_path))?;

        // parse as post (for now. we'll always use Post type)
        let post = Post::from_markdown(&content).context("Failed to parse content")?;

        // skip drafts in production builds
        if post.frontmatter().draft {
            return Ok(());
        }

        // parse markdown body with shortcodes
        let html_body = self
            .markdown_parser
            .parse(post.body())
            .context("Failed to parse markdown")?;

        // build template contect with rendered HTML
        let mut context = TemplateContext::new();

        let post_data = json!({
            "title": post.frontmatter().title,
            "date": post.frontmatter().date.to_rfc3339(),
            "draft": post.frontmatter().draft,
            "tags": post.frontmatter().tags,
            "body": html_body,
            "slig": post.slug(),
        });

        context.insert("post", &post_data);
        context.insert("site", &self.config);

        // render template
        let rendered = self
            .template_engine
            .render("post.html", &context)
            .context("Failed to render template")?;

        // determine output path (use filename stem as slug)
        let slug = self.get_slug_from_path(relative_path);
        let output_path = self.get_output_path(&slug);

        // write output
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(&output_path, rendered)
            .with_context(|| format!("Failed to write {:?}", output_path))?;

        Ok(())
    }

    fn get_slug_from_path(&self, relative_path: &Path) -> String {
        relative_path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("index")
            .to_string()
    }

    fn get_output_path(&self, slug: &str) -> std::path::PathBuf {
        // create pretty URLs: content/post.md -> public/post/indec.html
        self.config.output_dir().join(slug).join("index.html")
    }
}
