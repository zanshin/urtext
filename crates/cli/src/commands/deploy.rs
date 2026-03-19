//! Implementation of the 'deploy' command.
//! Supports rsyn deployment to remote servers or local directory copies.
//! crates/cli/src/commands/deploy.rs

use anyhow::{Context, Result};
use base::SiteConfig;
use std::fs;
use std::path::Path;
use std::process::Command;
use walkdir::WalkDir;

pub struct DeployCommand {
    config: SiteConfig,
    target: String,
}

impl DeployCommand {
    pub fn new(config: SiteConfig, target: String) -> Self {
        Self { config, target }
    }

    pub fn execute(&self) -> Result<()> {
        let source_dir = self.config.output_dir();

        if !source_dir.exists() {
            anyhow::bail!("Output directory does not exist. Run 'build' first.");
        }

        // check if target looks like a remote path (contains : or @)
        if self.target.contains(":") || self.target.contains('@') {
            self.deploy_rsync(source_dir)?;
        } else {
            self.deploy_local(source_dir)?;
        }

        Ok(())
    }

    fn deploy_rsync(&self, source_dir: &Path) -> Result<()> {
        println!("Deploying to {} via rsync...", self.target);

        let source = format!("{}/", source_dir.display());

        let output = Command::new("rsync")
            .arg("-avz")
            .arg("--delete")
            .arg(&source)
            .arg(&self.target)
            .output()
            .context("Failed to run rsync. Is rsync installed")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("rsync failed {}", stderr);
        }

        println!("Deployment completed");
        Ok(())
    }

    fn deploy_local(&self, source_dir: &Path) -> Result<()> {
        println!("Deploying to local directory: {}", self.target);

        let dest_path = Path::new(&self.target);

        // create destination directory
        fs::create_dir_all(dest_path)
            .with_context(|| format!("Failed to create destination directory: {}", self.target))?;

        // copy all files
        for entry in WalkDir::new(source_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
        {
            let src_path = entry.path();
            let relative_path = src_path
                .strip_prefix(source_dir)
                .context("Failed to get relative path")?;

            let dest_file = dest_path.join(relative_path);

            if let Some(parent) = dest_file.parent() {
                fs::create_dir_all(parent)?;
            }

            fs::copy(src_path, &dest_file)
                .with_context(|| format!("Failed to copy {:?}", src_path))?;
        }

        println!("Deployment complete");
        Ok(())
    }
}
