use axum::{Router, response::Html, routing::post};

use crate::content::page;

pub fn router() -> Router {
    Router::new().route("/render", post(render_markdown))
}

async fn render_markdown(body: String) -> Html<String> {
    let html = page::to_html(&body);
    Html(html)
}
