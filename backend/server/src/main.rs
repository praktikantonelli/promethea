//! Sample backend code.
use askama::Template;
use axum::body::Body;
use axum::extract::Query;
use axum::http::{Request, StatusCode};
use axum::response::{Html, IntoResponse, Response};
use axum::routing::get;
use axum::{Router, serve};
use serde::Deserialize;
use std::io::Error as IOError;
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
        .route("/", get(index_handler))
        .route("/greet-me", get(greeting_handler))
        .route("/get_book", get(book_handler))
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
    /// could not bind socket.
    #[error("Could not bind TCP listener: {0}")]
    Bind(#[source] IOError),
    /// could not run server.
    #[error("Could not run HTTP server: {0}")]
    Run(#[source] IOError),
}

/// An error type returnable by an axum handler.
#[derive(Debug, thiserror::Error)]
enum AppError {
    /// not found.
    #[error("Route handler not found!")]
    NotFound,
    /// could not render template.
    #[error("Failed to render HTML!")]
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

/// Handler for "/" route with query.
#[derive(Debug, Deserialize)]
struct IndexHandlerQuery {
    /// name, supplied via input field.
    #[serde(default)]
    name: String,
}

/// The axum handler for the root index.html.
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

/// The axum handler for the greet request.
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

/// Sample templated struct returned by a GET request.
#[derive(Template)]
#[template(path = "book.html")]
struct BookItem {
    /// the book's title.
    title: String,
    /// the book's list of authors.
    authors: Vec<Author>,
    /// the book's page count.
    page_count: u16,
    /// the book's series memberships.
    series: Vec<SeriesAndVolume>,
}

/// Sample author struct.
struct Author(String);

/// Sample series and volume struct.
struct SeriesAndVolume {
    /// the series in question.
    title: Series,
    /// the book's entry.
    index: f32,
}

/// Sample series struct.
struct Series(String);

/// A sample axum handler that returns a custom struct with a HTML template.
async fn book_handler() -> Result<impl IntoResponse, AppError> {
    let title = "A Memory of Light".into();
    let authors = vec![
        Author("Brandon Sanderson".to_owned()),
        Author("Robert Jordan".to_owned()),
    ];
    let page_count = 1056;
    let series = vec![SeriesAndVolume {
        title: Series("The Wheel of Time".to_owned()),
        index: 12.0,
    }];
    let book = BookItem {
        title,
        authors,
        page_count,
        series,
    };
    Ok(Html(book.render()?))
}
