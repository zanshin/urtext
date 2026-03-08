// src/content/archetypes.rs

pub fn new_site(sitename: String) {
    println!("Creating new site named {}", sitename);
}

pub fn new_post(kind: String, title: String) {
    println!("Create new post of kind {} with title {}", kind, title);
}
