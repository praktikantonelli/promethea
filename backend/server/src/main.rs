//! server backend package

use axum::{Router, body::Body, http::Request, response::Html, routing::get};
use std::path::PathBuf;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
#[cfg(debug_assertions)]
use tower_livereload::LiveReloadLayer;
use tracing::info;
use tracing_subscriber::fmt::init;
/// Module containing types used in axum handlers

#[tokio::main]
async fn main() {
    init();

    let frontend_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../frontend");

    let app = Router::new()
        .route("/hello", get(hello))
        .fallback_service(ServeDir::new(frontend_dir));

    #[allow(clippy::shadow_reuse, reason = "needs to keep name")]
    #[cfg(debug_assertions)]
    let app = app.layer(
        LiveReloadLayer::new()
            .request_predicate(|req: &Request<Body>| !req.headers().contains_key("HX-Request")),
    );

    #[allow(clippy::unwrap_used, reason = "unrecoverable error")]
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();

    info!("Server running at http://localhost:3000");

    #[allow(clippy::unwrap_used, reason = "unrecoverable error")]
    axum::serve(listener, app).await.unwrap();
}

/// example GET endpoint
#[allow(clippy::single_call_fn, reason = "example HTTP endpoint")]
async fn hello() -> Html<&'static str> {
    info!("Request received!");
    Html("<p>Hello World from Axum!</p>")
}
