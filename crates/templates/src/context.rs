//! Template context builder for passing data to templates.
//! Converts content tupes and site config into templates-friendly data

use base::{LinkPost, Note, Post, SiteConfig, VideoPost};
use serde::Serialize;
use serde_json::json;
use tera::Context;

pub struct TemplateContext {
    context: Context,
}

impl TemplateContext {
    pub fn new() -> Self {
        Self {
            context: Context::new(),
        }
    }

    pub fn insert<T: Serialize + ?Sized>(&mut self, key: &str, value: &T) {
        self.context.insert(key, value);
    }

    pub fn from_post(post: &Post, site_config: &SiteConfig) -> Self {
        let mut context = Self::new();

        let post_data = json!({
            "title": post.frontmatter().title,
            "date": post.frontmatter().date.to_rfc3339(),
            "draft": post.frontmatter().draft,
            "tags": post.frontmatter().tags,
            "body": post.body(),
            "slug": post.slug(),
        });

        context.insert("post", &post_data);
        context.insert("site", site_config);

        context
    }

    pub fn from_link_post(link_post: &LinkPost, site_config: &SiteConfig) -> Self {
        let mut context = Self::new();

        let link_post_data = json!({
            "title": link_post.frontmatter().title,
            "date": link_post.frontmatter().date.to_rfc3339(),
            "draft": link_post.frontmatter().draft,
            "tags": link_post.frontmatter().tags,
            "body": link_post.body(),
            "slug": link_post.slug(),
            "link_url": link_post.link_url(),
        });

        context.insert("content", &link_post_data);
        context.insert("link_url", link_post.link_url());
        context.insert("site", site_config);

        context
    }

    pub fn from_note(note: &Note, site_config: &SiteConfig) -> Self {
        let mut context = Self::new();

        let note_data = json!({
            "title": note.frontmatter().title,
            "date": note.frontmatter().date.to_rfc3339(),
            "draft": note.frontmatter().draft,
            "tags": note.frontmatter().tags,
            "body": note.body(),
            "slug": note.slug(),
        });

        context.insert("content", &note_data);
        context.insert("site", site_config);

        context
    }

    pub fn from_video_post(video_post: &VideoPost, site_config: &SiteConfig) -> Self {
        let mut context = Self::new();

        let video_data = json!({
            "title": video_post.frontmatter().title,
            "date": video_post.frontmatter().date.to_rfc3339(),
            "draft": video_post.frontmatter().draft,
            "tags": video_post.frontmatter().tags,
            "body": video_post.body(),
            "slug": video_post.slug(),
            "youtube_id": video_post.youtube_id(),
        });

        context.insert("content", &video_data);
        context.insert("youtube_id", video_post.youtube_id());
        context.insert("site", site_config);

        context
    }

    pub fn from_site(site_config: &SiteConfig) -> Self {
        let mut context = Self::new();
        context.insert("site", site_config);
        context
    }

    pub fn inner(&self) -> &Context {
        &self.context
    }
}

impl Default for TemplateContext {
    fn default() -> Self {
        Self::new()
    }
}
