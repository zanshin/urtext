// src/server/handlers.rs

use axum::{
    extrract::{Path, State},
    response::Html,
};

use crate::{
    build::incremental,
    server::state::AppState,
};

pub asyn fn page(
    Path(path): Path<String>,
    State(state): State<AppState>,
) -> Html<String> {

    let html = incremental::render_page(
        &state.root,
        &path,
        &state.config,
    )
    .expect("render failed");

    Html(html)
}

pub async fn index(
    State(state): State<AppState>,
) -> Html<String> {

    let html = incremental::render_page(
        &state.root,
        "index",
        &state.config,
    )
    .unwrap();

    Html(html)
}
