//! Implementation of the 'new' command to scaffold a new site.
//! Uses prototype site to create scaffold for the new site.
//! crates/cli/src/commands/new.rs

use anyhow::{bail, Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

pub struct NewCommand {
    name: String,
    path: PathBuf,
    prototype_path: PathBuf,
}

impl NewCommand {
    pub fn new(name: &str, base_path: Option<PathBuf>) -> Self {
        let path = base_path.unwrap_or_else(|| PathBuf::from("."));
        let prototype_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("site");

        Self {
            name: name.to_string(),
            path,
            prototype_path,
        }
    }

    pub fn create_site(&self) -> Result<()> {
        let site_path = self.path.join(&self.name);

        if site_path.exists() {
            bail!("Directory '{}' already exists", self.name);
        }

        self.copy_dir_recursive(&self.prototype_path, &site_path)
    }

    fn copy_dir_recursive(&self, src: &Path, dst: &Path) -> Result<()> {
        fs::create_dir_all(dst).context("Failed to create directory")?;

        for entry in fs::read_dir(src).context("Failed to read template directory")? {
            let entry = entry.context("Failed to read directory entry")?;
            let src_path = entry.path();
            let dst_path = dst.join(entry.file_name());

            if src_path.is_dir() {
                self.copy_dir_recursive(&src_path, &dst_path)?;
            } else {
                self.copy_file(&src_path, &dst_path)?;
            }
        }

        Ok(())
    }

    fn copy_file(&self, src: &Path, dst: &Path) -> Result<()> {
        // Only attempt substitution on text files
        match src.extension().and_then(|e| e.to_str()) {
            Some("toml" | "html" | "md" | "txt" | "css" | "js") => {
                let content = fs::read_to_string(src).context("Failed to read template file")?;
                let content = content
                    .replace("{{title}}", &self.name)
                    .replace("{{base_url}}", "https://localhost:3000");
                fs::write(dst, content).context("Failed to write file")?;
            }
            _ => {
                fs::copy(src, dst).context("Failed to copy file")?;
            }
        }
        Ok(())
    }
}
