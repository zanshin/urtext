use urtext::content::page::slugify;

#[test]
fn basic_slug() {
    assert_eq!(
        slugify("Rust: 10 Lessons I Learned in 2026"),
        "rust-10-lessons-i-learned-in-2026"
    );
}

#[test]
fn trims_extra_whitespace() {
    assert_eq!(slugify("   Hello   World   "), "hello-world");
}

#[test]
fn collapses_multiple_separators() {
    assert_eq!(slugify("hello---world"), "hello-world");
}

#[test]
fn drops_punctuation() {
    assert_eq!(slugify("Hello, World!"), "hello-world");
}

#[test]
fn empty_string() {
    assert_eq!(slugify(""), "");
}

#[test]
fn only_symbols() {
    assert_eq!(slugify("!!@@@###"), "");
}
