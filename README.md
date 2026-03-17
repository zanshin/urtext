# URTEXT

Urtext is a fast, simple static site generator built in Rust. It supports
multiple content types (posts, link posts, notes, video posts), shortcodes for
embeds, and includes a development server with file watching.

## ARCHITECTURE

The project uses a Cargo workspace with separate crates for modularity:

### Workspace Structure

- urtext (root) - Main binary, CLI entry  point
- base - Base domain models (Post, LinkPost, Note, VideoPost, FrontMatter,
SiteCOnfig)

### Content Types

- Posts - Standard blog posts with title, date, and tags
- Link Posts - Posts with external link_url field
- Notes - Short-form content (optional title)
- Video Posts - Posts with youtube_id field for embedded videos

### Shortcodes

Shortcodes are Hugo-style syntax: `{{< name param="value" >}}`

Supported:

- youtube - YouTube video embeds
- mastodon - Mastodon toot embeds
- Amazon - Amazon book lines (ASIN-based)

### Key Technologies

- Tera - Template engine (Jinja2-like syntax)
- Axum + Tokio - Async HTTP server
- pulldown-cmark - CommonMark markdown parser
- notify - File system watching
- rsync - Remote deployment

## USAGE

### Create content

The following commands create new content:

    urtext create post "My First Post"
    urtext create link-post "Interesting Link"
    urtext create note
    urtext video "Cool Video"

### Serve locally

    urtext serve          # http://localhost:3000
    urtext serve -p 8080  # Custom port
    urtext serve --open   # Open browser automatically

## PROJECT STRUCTURE (Generated sites)

```
my-site/
├── config.toml           # Site configuration
├── content/              # Markdown content files
│   ├── 2026-03-15-post.md
│   └── 2026-03-16-note.md
├── templates/            # Tera templates
│   ├── base.html
│   ├── post.html
│   ├── link_post.html
│   ├── note.html
│   └── video_post.html
├── archetypes/           # Content templates
│   ├── post.md
│   ├── link_post.md
│   ├── note.md
│   └── video_post.md
├── static/               # Static assets (CSS, images, etc.)
│   └── css/
│       └── base.css
└── public/               # Generated output
    ├── post/index.html
    └── ...
```

## IMPORTANT NOTES

- All templates must use `{{ content.body | safe }}` to render HTML (Tera
escapes by default)
- Slug generation: filename stem is used as slug (e.g., 2026-03-16-first-post
-> /2026-03-15-first-post/)
- Front matter is YAML between `---` delimiters
- File watcher monitors `content/` and `templates` directories
- Static files in `static/` are copied to output during build
