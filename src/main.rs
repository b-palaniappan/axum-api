use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tracing::info;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();
    // build our application with a route
    let app = Router::new()
        .route("/", get(handler))
        .route("/json", get(handler_json));
    let app = app.fallback(handler_404);

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Serialize, Deserialize)]
struct Message {
    message: String,
}

// Handler for route "/"
async fn handler() -> Html<&'static str> {
    info!("Show hello world.");
    Html("Hello, World!")
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Nothing to see here.")
}

async fn handler_json() -> impl IntoResponse {
    info!("Handle Json payload");
    Json(Message {
        message: "Hello".to_string(),
    })
}
