//! Main site builder implementation
//! Coordinates content reading, processing, and output generation

use crate::pipeline::BuildPipeline;
use anyhow::{Context, Result};
use base::SiteConfig;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

pub struct SiteBuilder {
    config: SiteConfig,
}

impl SiteBuilder {
    pub fn new(config: SiteConfig) -> Self {
        Self { config }
    }

    pub fn build(&self) -> Result<()> {
        self.build_with_static(None)
    }

    pub fn build_with_static(&self, static_dir: Option<&Path>) -> Result<()> {
        // create output directory
        fs::create_dir_all(self.config.output_dir())
            .context("Failed to create output directory")?;

        // initialize build pipeline
        let pipeline = BuildPipeline::new(&self.config)?;

        // process all content files
        let content_dir = self.config.content_dir();
        if content_dir.exists() {
            for entry in WalkDir::new(content_dir)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("md"))
            {
                let content_path = entry.path();
                let relative_path = content_path
                    .strip_prefix(content_dir)
                    .context("Failed to get relative path")?;

                pipeline.process_content(content_path, relative_path)?;
            }
        }

        // copy static files if provided
        if let Some(static_path) = static_dir {
            if static_path.exists() {
                self.copy_static_files(static_path)?;
            }
        }

        // copy static files if provided
        if let Some(static_path) = static_dir {
            if static_path.exists() {
                self.copy_static_files(static_path)?;
            }
        }

        Ok(())
    }

    fn copy_static_files(&self, static_dir: &Path) -> Result<()> {
        for entry in WalkDir::new(static_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
        {
            let src_path = entry.path();
            let relative_path = src_path
                .strip_prefix(static_dir)
                .context("Failed to get relative path")?;

            let dest_path = self.config.output_dir().join(relative_path);

            if let Some(parent) = dest_path.parent() {
                fs::create_dir_all(parent)?;
            }

            fs::copy(src_path, dest_path)
                .with_context(|| format!("Failed to copy {:?}", src_path))?;
        }

        Ok(())
    }
}
