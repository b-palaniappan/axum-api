use std::net::SocketAddr;

use axum::{
  Json,
  response::{Html, IntoResponse},
  Router, routing::get,
};
use chrono::{SecondsFormat, Utc};
use serde::{Deserialize, Serialize};
use tracing::{error, info};

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
  let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
  info!("Starting server at {}", addr);
  if let Err(e) = axum::Server::bind(&addr).serve(app.into_make_service()).await {
    error!("Server error: {}", e);
  }
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

// JSON handlers
async fn handler_json() -> impl IntoResponse {
  info!("Handle Json payload");
  Json(Message {
    message: "Hello".to_string(),
    status: "Success".to_string(),
  })
}

// Page not found fallback handlers
async fn handler_404() -> impl IntoResponse {
  Json(ApiError {
    status: 404,
    message: "Endpoint not found".to_owned(),
    time: Utc::now().to_rfc3339_opts(SecondsFormat::Micros, true),
    debug_message: Some("Endpoint you are requesting is not found".to_owned()),
  })
}

// Model for error handling.
#[derive(Debug, Serialize, Deserialize)]
pub struct ApiError {
  pub status: u16,
  pub time: String,
  pub message: String,
  pub debug_message: Option<String>,
}
