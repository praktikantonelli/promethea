use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/api/hello", get(say_hello));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn say_hello() -> &'static str {
    println!("Request received!");
    "Hello from axum!"
}
