//! Template enginer wrapper around Tera.
//! Loads templates from a directory and renders them with context

use crate::context::TemplateContext;
use std::path::Path;
use tera::Tera;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TemplateError {
    #[error("Template error: {0}")]
    TeraError(#[from] tera::Error),

    #[error("Template not found {0}")]
    NotFound(String),

    #[error("Failed to load templates: {0}")]
    LoadError(String),
}

pub struct TemplateEngine {
    tera: Tera,
}

impl TemplateEngine {
    pub fn new(template_dir: &Path) -> Result<Self, TemplateError> {
        let pattern = template_dir.join("**/*.html");
        let pattern_str = pattern
            .to_str()
            .ok_or_else(|| TemplateError::LoadError("Invalid template path".to_string()))?;

        let tera = Tera::new(pattern_str)
            .map_err(|e| TemplateError::LoadError(format!("Failed to laod template: {}", e)))?;

        Ok(Self { tera })
    }

    pub fn render(
        &self,
        template_name: &str,
        context: &TemplateContext,
    ) -> Result<String, TemplateError> {
        let rendered = self.tera.render(template_name, &context.inner())?;
        Ok(rendered)
    }

    pub fn add_template(&mut self, name: &str, content: &str) -> Result<(), TemplateError> {
        self.tera.add_raw_template(name, content)?;
        Ok(())
    }
}
