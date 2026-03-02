use urtext::server::routes;

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;
use http_body_util::BodyExt;

#[tokio::test]
async fn render_markdown_route_returns_html() {
    let app = routes::router();

    let request = Request::builder()
        .method("POST")
        .uri("/render")
        .header("content-type", "text/plain")
        .body(Body::from("# Hello"))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();

    let body_str = std::str::from_utf8(&body).unwrap();

    assert!(body_str.contains("<h1>Hello</h1>"));
}

#[tokio::test]
async fn unknown_route_returns_404() {
    let app = routes::router();

    let request = Request::builder()
        .uri("/does-not-exist")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
