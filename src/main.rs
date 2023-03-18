use std::net::SocketAddr;

use axum::{http::StatusCode, Json, response::{Html, IntoResponse}, Router, routing::get};
use serde::{Deserialize, Serialize};
use tracing::info;

pub mod api {
  pub mod handlers;
  pub mod model;
}

#[tokio::main]
async fn main() {
  // Logging handler using tracing.
  tracing_subscriber::fmt().init();

  // build our application with a route
  let app = Router::new()
    .route("/", get(handler))
    .route("/json", get(handler_json))
    .nest("/hello", api::handlers::hello_handler::routes())
    .nest("/users", api::handlers::users_handler::routes());
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
  status: String,
}

// Handler for route "/"
async fn handler() -> Html<&'static str> {
  info!("Show hello world.");
  Html("Hello, World!")
}

// Page not found fallback handlers
async fn handler_404() -> impl IntoResponse {
  (StatusCode::NOT_FOUND, "Nothing to see here.")
}

// JSON handlers
async fn handler_json() -> impl IntoResponse {
  info!("Handle Json payload");
  Json(Message {
    message: "Hello".to_string(),
    status: "Success".to_string(),
  })
}
