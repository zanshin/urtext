// src/main.rs

use axum::Router;
use tokio::net::TcpListener;

use urtext::server::routes;

#[tokio::main]
async fn main() {
    let app: Router = routes::router();

    let listener = TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("failed to bind");

    axum::serve(listener, app).await.expect("server failed");
}
