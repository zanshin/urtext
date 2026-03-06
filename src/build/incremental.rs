// src/build/incremental.rs

pub fn render_page(root: &Path, path: &str, config: &Config) -> Result<String> {
    println!("Rendering a page...");

    Ok("<html><body><p>Hello render</p></body></htm>".to_string())
}
