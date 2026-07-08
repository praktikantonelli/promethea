use axum::{Json, Router, routing::get};
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

async fn say_hello() -> &'static str {
    println!("Request received!");
    "Hello from axum!"
}

async fn return_type() -> Json<MyDummyStruct> {
    let dummy = MyDummyStruct {
        id: 7,
        label: "ASDF".into(),
        values: vec![0, 2, 5, 17],
        enumeration: MyDummyEnum::Three(Ok(47)),
    };
    Json(dummy)
}
