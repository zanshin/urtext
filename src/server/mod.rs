// src/server/mod.rs

use crate::config::config;

use axum::Router;
use std::net::SocketAddr;
use tokio::net::TcpListener;

pub mod routes;
pub mod state;

use state::AppState;

pub async fn run(port: u16, watch: bool) {
    let config = Config::load().expect("config load failed");

    let state = AppState {
        config,
        root: std::env::current_dir().unwrap(),
        watch,
    };

    let app = routes::router().with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = TcpListener::bind(addr).await.unwrap();

    println!("Preview server: http://{}", addr);

    axum::serve(listener, app).await.unwrap();
}
