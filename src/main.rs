use axum::http::StatusCode;
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::env;
use sea_orm::{Database};

use todo_axum::users::User;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{host}:{port}");

    let conn = Database::connect(db_url)
        .await
        .expect("Database connection failed");
    // Migrator::up(&conn, None).await.unwrap();

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
