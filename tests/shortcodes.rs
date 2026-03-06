use tera::Tera;
use urtext::shortcodes::handlers::parse_shortcode_inner;
use urtext::shortcodes::handlers::process_shortcodes;

#[test]
fn parses_simple_shortcode() {
    let input = "youtube id=dQw4w9WgXcQ";
    let sc = parse_shortcode_inner(input).unwrap();

    assert_eq!(sc.name, "youtube");
    assert_eq!(sc.attrs.get("id").unwrap(), "dQw4w9WgXcQ");
}

#[test]
fn parses_quoted_attributes() {
    let input = r#"youtube id="abc123""#;
    let sc = parse_shortcode_inner(input).unwrap();

    assert_eq!(sc.attrs.get("id").unwrap(), "abc123");
}

#[test]
fn replaces_youtube_shortcode() {
    let mut tera = Tera::default();

    tera.add_raw_template("partials/youtube.html", "<div>{{ id }}</div>")
        .unwrap();

    let input = "Before {{< youtube id=abc123 >}} After";

    let output = process_shortcodes(input, &tera).unwrap();

    assert_eq!(output, "Before <div>abc123</div> After");
}

#[test]
fn parses_mixes_attributes() {
    let input = r#"youtube id=abc123 start="90""#;

    let sc = parse_shortcode_inner(input).unwrap();

    assert_eq!(sc.attrs.get("id").unwrap(), "abc123");
    assert_eq!(sc.attrs.get("start").unwrap(), "90");
}

#[test]
fn parses_mastodon_shortcode() {
    let input = r#"mastodon url="https://server.social/@user/123456""#;

    let sc = parse_shortcode_inner(input).unwrap();

    assert_eq!(sc.name, "mastodon");
    assert_eq!(
        sc.attrs.get("url").unwrap(),
        "https://server.social/@user/123456"
    );
}

#[test]
fn parses_book_shortcode() {
    let input = r#"book asin="1234567890" title="Book Title""#;

    let sc = parse_shortcode_inner(input).unwrap();

    assert_eq!(sc.name, "book");
    assert_eq!(sc.attrs.get("asin").unwrap(), "1234567890");
    assert_eq!(sc.attrs.get("title").unwrap(), "Book Title");
}
