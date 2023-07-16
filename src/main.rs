use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(hello_world));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Serialize)]
struct Hello {
    message: String,
}

async fn hello_world() -> (StatusCode, Json<Hello>) {
    let msg = Hello {
        message: "Hello World".to_string(),
    };
    (StatusCode::OK, Json(msg))
}
