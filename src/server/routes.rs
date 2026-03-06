use crate::content::page;
use crate::shortcodes::handlers;
use axum::{Router, routing::post};

// pub fn serve(port: u16, watch: bool) {
//     println!("serve port={} watch={}", port, watch);
//
//     let app = Router::new().route("/render", post(render_markdown));
//     let addr = SocketAddr::from(([127, 0, 0, 1], port));
//     let listener = TcpListener::bind(addr).await.expect("Failed to bind");
//
//     axum::serve(listener, app)
//         .await
//         .expect("Axum server failed");
// }
//
pub fn router() -> Router {
    Router::new().route("/render", post(render_markdown))
}

async fn render_markdown(body: String) -> Html<String> {
    let html = page::to_html(&body);
    Html(html)
}
