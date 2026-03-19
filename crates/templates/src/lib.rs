//! Template engine for urtext using Tera.
//! Handles template rendering, context building, and custom filters.

pub mod context;
pub mod engine;

pub use context::TemplateContext;
pub use engine::TemplateEngine;
