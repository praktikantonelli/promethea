//! server backend package

// silence clippy by importing and not using
use shared as _;

use axum::{Json, Router, routing::get};
use std::io::Error;
use tokio::net::TcpListener;
/// Module containing types used in axum handlers
pub mod types;

use types::{MyDummyEnum, MyDummyStruct};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let app = Router::new()
        .route("/api/hello", get(say_hello))
        .route("/api/test-types", get(return_type));

    let listener = TcpListener::bind("127.0.0.1:3000").await?;
    axum::serve(listener, app).await?;
    Ok(())
}

#[allow(clippy::single_call_fn, reason = "example axum handler")]
#[allow(clippy::print_stdout, reason = "example axum handler")]
/// Sample axum handler for GET request that returns a simple value
async fn say_hello() -> &'static str {
    println!("Request received!");
    "Hello from axum!"
}

#[allow(clippy::single_call_fn, reason = "example axum handler")]
/// Sample axum handler for GET request that returns a custom type
async fn return_type() -> Json<MyDummyStruct> {
    let dummy = MyDummyStruct {
        id: 7,
        label: "ASDF".into(),
        values: vec![0, 2, 5, 17],
        enumeration: MyDummyEnum::Three(Ok(47)),
    };
    Json(dummy)
}
