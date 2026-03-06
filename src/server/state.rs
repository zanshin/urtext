// src/server/state.rs

use std::path::PathBuf;
use crate::config::Config;

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub root: PathBuf,
    pub watch: bool,
}
