//! Command implementations for the CLI.
//! Each module handles a specific command (new, create, build, serve, deploy)
//! crates/cli/src/commands/mod.rs

pub mod new;

pub use new::NewCommand;
