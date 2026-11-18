//! Sample backend code
use askama::Template;
use askama_web as _;
use axum::body::Body;
use axum::extract::Query;
use axum::http::{Request, StatusCode};
use axum::response::{Html, IntoResponse, Redirect, Response};
use axum::routing::get;
use axum::{Router, serve};
use serde::Deserialize;
use std::io::Error as IOError;
use strum as _;
use tokio::net::TcpListener;
use tower_http::{services::ServeDir, trace::TraceLayer};
#[cfg(debug_assertions)]
use tower_livereload::LiveReloadLayer;
use tracing::{Level, info};

#[tokio::main]
async fn main() -> Result<(), PrometheaError> {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    let app = Router::new()
        .route("/", get(start_handler))
        .route("/{lang}/index.html", get(index_handler))
        .route("/greet-me", get(greeting_handler))
        .route("/sample", get(sample_handler))
        .nest_service(
            "/static",
            ServeDir::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../../frontend")),
        )
        .fallback(|| async { AppError::NotFound })
        .layer(TraceLayer::new_for_http());

    #[allow(clippy::shadow_reuse, reason = "name must remain")]
    #[cfg(debug_assertions)]
    let app = app.layer(
        LiveReloadLayer::new()
            .request_predicate(|req: &Request<Body>| !req.headers().contains_key("HX-Request")),
    );

    // In a real application you would most likely read the configuration from a config file.
    let listener = TcpListener::bind("127.0.0.1:8080")
        .await
        .map_err(PrometheaError::Bind)?;

    if let Ok(addr) = listener.local_addr() {
        info!("Listening on http://{addr}/");
    }
    serve(listener, app).await.map_err(PrometheaError::Run)
}

/// A custom error type.
#[derive(pretty_error_debug::Debug, thiserror::Error)]
enum PrometheaError {
    /// could not bind socket
    Bind(#[source] IOError),
    /// could not run server
    Run(#[source] IOError),
}

#[derive(Debug, thiserror::Error)]
enum AppError {
    /// not found
    NotFound,
    /// could not render template
    Render(#[from] askama::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        #[derive(Debug, Template)]
        #[template(path = "error.html")]
        struct Tmpl {
            /// the custom error type.
            err: AppError,
        }

        let status = match self {
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::Render(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        let tmpl = Tmpl { err: self };
        tmpl.render().map_or_else(
            |_| (status, "Something went wrong").into_response(),
            |body| (status, Html(body)).into_response(),
        )
    }
}

async fn start_handler() -> Redirect {
    Redirect::temporary("/en/index.html")
}

/// Handler for "/" route with query.
#[derive(Debug, Deserialize)]
struct IndexHandlerQuery {
    /// name, supplied via input field
    #[serde(default)]
    name: String,
}

async fn index_handler(
    Query(query): Query<IndexHandlerQuery>,
) -> Result<impl IntoResponse, AppError> {
    /// Template struct for askama's HTML template.
    #[derive(Debug, Template)]
    #[template(path = "index.html")]
    struct Tmpl {
        /// name parameter.
        name: String,
    }

    let template = Tmpl { name: query.name };
    Ok(Html(template.render()?))
}

/// Handler for "/greet" route with query.
#[derive(Debug, Deserialize)]
struct GreetingHandlerQuery {
    /// name, supplied via input field.
    name: String,
}

async fn greeting_handler(
    Query(query): Query<GreetingHandlerQuery>,
) -> Result<impl IntoResponse, AppError> {
    /// Template struct for askama's HTML template.
    #[derive(Debug, Template)]
    #[template(path = "greet.html")]
    struct Tmpl {
        /// name parameter.
        name: String,
    }

    let template = Tmpl { name: query.name };
    Ok(Html(template.render()?))
}

async fn sample_handler() -> Result<impl IntoResponse, AppError> {
    Ok(Html("<p>Hello World from Axum!</p>"))
}
