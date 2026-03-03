// src/shortcodes/handlers.rs
use anyhow::Result;
use std::collections::HashMap;
use tera::{Context, Tera};

// Three shortcodes:
// YouTube video embed
// Mastodon embed
// Amazon affiliate-linked card

// Shortcode formatting steals from Hugo
// {{< youtube id="XXXXXXXXXXX" >}}
// {{< mastodon url="https://hachyderm.io/@uwer/123446" >}}
// {{< book asin="1234567890" title="Book title" >}}

#[derive(Debug, PartialEq)]
pub struct Shortcode {
    pub name: String,
    pub attrs: HashMap<String, String>,
}

pub fn parse_shortcode_inner(input: &str) -> Option<Shortcode> {
    let input = input.trim();
    if input.is_empty() {
        return None;
    }

    let mut chars = input.chars().peekable();

    // parse shortcode name
    let mut name = String::new();
    while let Some(&c) = chars.peek() {
        if c.is_whitespace() {
            break;
        }
        name.push(c);
        chars.next();
    }

    let mut attrs = HashMap::new();

    while chars.peek().is_some() {
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

        // skip whitespace
        while matches!(chars.peek(), Some(c) if c.is_whitespace()) {
            chars.next();
        }

        // expect '='
        if chars.peek() != Some(&'=') {
            break;
        }
        chars.next(); // consume '='

        // skip whitespace
        while matches!(chars.peek(), Some(c) if c.is_whitespace()) {
            chars.next();
        }

        // parse value
        let value = if chars.peek() == Some(&'"') {
            chars.next(); // consume opening quote
            let mut val = String::new();

            while let Some(c) = chars.next() {
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

pub fn process_shortcodes(content: &str, tera: &Tera) -> Result<String> {
    let mut output = String::new();
    let mut rest = content;

    while let Some(start) = rest.find("{{<") {
        let (before, after_start) = rest.split_at(start);
        output.push_str(before);

        if let Some(end) = after_start.find(">}}") {
            let (shortcode_block, after_end) = after_start.split_at(end + 3);

            // Extract inner: remove {{< and >}}
            let inner = &shortcode_block[3..shortcode_block.len() - 3];

            if let Some(shortcode) = parse_shortcode_inner(inner) {
                let rendered = render_shortcode(&shortcode, tera)?;
                output.push_str(&rendered);
            }

            rest = after_end;
        } else {
            // No closing tag — just append remaining text
            output.push_str(after_start);
            break;
        }
    }

    output.push_str(rest);
    Ok(output)
}

fn render_shortcode(sc: &Shortcode, tera: &Tera) -> Result<String> {
    match sc.name.as_str() {
        "youtube" => {
            let id = sc
                .attrs
                .get("id")
                .ok_or_else(|| anyhow::anyhow!("youtube shortcode missing id"))?;

            let mut context = Context::new();
            context.insert("id", id);

            Ok(tera.render("partials/youtube.html", &context)?)
        }
        _ => Ok(String::new()), // Unknown shortcode = silently ignore
    }
}
