use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root))
        .route("/classes", get(get_classes))
        .route("/classes", post(create_class));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello World!"
}

async fn create_class(
    Json(payload): Json<CreateClass>,
) -> (StatusCode, Json<Class>) {
    let class = Class {
        crn: 3550,
        name: payload.name,
    };

    (StatusCode::CREATED, Json(class))
}

#[derive(Deserialize)]
struct CreateClass {
    name: String,
}

#[derive(Serialize)]
struct Class {
    crn: u64,
    name: String,
}

async fn get_classes() -> &'static str {
    "List of classes goes here!"
}
