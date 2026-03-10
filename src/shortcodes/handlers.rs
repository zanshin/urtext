// src/shortcodes/handlers.rs

use std::collections::HashMap;

use anyhow::Result;

#[derive(Debug, PartialEq)]
pub struct Shortcode {
    pub name: String,
    pub attrs: HashMap<String, String>,
}

pub fn process_shortcodes(content: &str) -> Result<String> {
    let mut output = String::new();
    let mut rest = content;

    while let Some(start) = rest.find("{{<") {
        let (before, after_start) = rest.split_at(start);
        output.push_str(before);

        if let Some(end) = after_start.find(">}}") {
            let (shortcode_block, after_end) = after_start.split_at(end + 3);
            let inner = &shortcode_block[3..shortcode_block.len() - 3];

            if let Some(sc) = parse_shortcode(inner) {
                let rendered = render_shortcode(&sc)?;
                output.push_str(&rendered);
            }

            rest = after_end;
        } else {
            output.push_str(after_start);
            break;
        }
    }

    output.push_str(rest);
    Ok(output)
}

fn parse_shortcode(input: &str) -> Option<Shortcode> {
    let input = input.trim();
    if input.is_empty() {
        return None;
    }

    let mut chars = input.chars().peekable();

    let mut name = String::new();
    while let Some(&c) = chars.peek() {
        if c.is_whitespace() {
            break;
        }
        name.push(c);
        chars.next();
    }

    let mut attrs = HashMap::new();

    loop {
        // skip whitespace
        while matches!(chars.peek(), Some(c) if c.is_whitespace()) {
            chars.next();
        }

        // parse key
        let mut key = String::new();
        while let Some(&c) = chars.peek() {
            if c == '=' || c.is_whitespace() {
                break;
            }
            key.push(c);
            chars.next();
        }

        if key.is_empty() {
            break;
        }

        // skip whitespace, expect '='
        while matches!(chars.peek(), Some(c) if c.is_whitespace()) {
            chars.next();
        }
        if chars.peek() != Some(&'=') {
            break;
        }
        chars.next();

        // skip whitespace, parse value
        while matches!(chars.peek(), Some(c) if c.is_whitespace()) {
            chars.next();
        }

        let value = if chars.peek() == Some(&'"') {
            chars.next(); // consume opening quote
            let mut val = String::new();
            for c in chars.by_ref() {
                if c == '"' {
                    break;
                }
                val.push(c);
            }
            val
        } else {
            let mut val = String::new();
            while let Some(&c) = chars.peek() {
                if c.is_whitespace() {
                    break;
                }
                val.push(c);
                chars.next();
            }
            val
        };

        attrs.insert(key, value);
    }

    Some(Shortcode { name, attrs })
}

fn render_shortcode(sc: &Shortcode) -> Result<String> {
    match sc.name.as_str() {
        "youtube" => render_youtube(sc),
        "mastodon" => render_mastodon(sc),
        "book" => render_book(sc),
        other => {
            eprintln!("Warning: unknown shortcode '{}'", other);
            Ok(String::new())
        }
    }
}

fn render_youtube(sc: &Shortcode) -> Result<String> {
    let id = sc
        .attrs
        .get("id")
        .ok_or_else(|| anyhow::anyhow!("youtube shortcode missing id"))?;

    Ok(format!(
        r#"<div> class="embed-youtube">
    <iframe
        src="https://www.youtube-nocookie.com/embed/{id}"
        title="YouTube video player"
        frameborder="0"
        allowfullscreen>
    </iframe>
</div>"#
    ))
}

fn render_mastodon(sc: &Shortcode) -> Result<String> {
    let _url = sc
        .attrs
        .get("url")
        .ok_or_else(|| anyhow::anyhow!("mastodon shortcode missing URL"))?;

    Ok(String::from("<!-- mastodon embed: not yet implemented -->"))
}

fn render_book(sc: &Shortcode) -> Result<String> {
    let _asin = sc
        .attrs
        .get("asin")
        .ok_or_else(|| anyhow::anyhow!("book shortcode missing ASIN"))?;

    Ok(String::from("<!-- book embed: not yet implmented -->"))
}
