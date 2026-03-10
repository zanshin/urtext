// tests/common/mod.rs

use std::fs;
use tera::Tera;

pub fn make_tera(template_dir: &std::path::Path) -> Tera {
    Tera::new(template_dir.join("**/*").to_str().unwrap()).unwrap()
}

pub fn setup_dirs(
    temp: &tempfile::TempDir,
) -> (std::path::PathBuf, std::path::PathBuf, std::path::PathBuf) {
    let content_dir = temp.path().join("content");
    let output_dir = temp.path().join("public");
    let template_dir = temp.path().join("templates");

    fs::create_dir_all(&content_dir).unwrap();
    fs::create_dir_all(&template_dir).unwrap();
    fs::create_dir_all(template_dir.join("partials")).unwrap();

    fs::write(
        template_dir.join("base.html"),
        "<html><body>{{ content | safe }}</body></html>",
    )
    .unwrap();

    (content_dir, output_dir, template_dir)
}
