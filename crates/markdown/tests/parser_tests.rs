// crate/markdown/tests/parser_tests.rs

use markdown::MarkdownParser;

#[test]
fn test_parse_basic_markdown() {
    let markdown = r#"# Hello World

This is a **bold** statement and this is *italic*.

## Subheading

- Item 1
- Item 2
"#;

    let parser = MarkdownParser::new();
    let html = parser.parse(markdown).expect("Failed to parse markdown");

    assert!(html.contains("<h1>Hello World</h1>"));
    assert!(html.contains("<strong>bold</strong>"));
    assert!(html.contains("<em>italic</em>"));
    assert!(html.contains("<h2>Subheading</h2>"));
    assert!(html.contains("<li>Item 1</li>"));
}
