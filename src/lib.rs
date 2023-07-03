use crate::api::model::api_error::ApiErrorResponse;
use axum::http::Method;
use axum::response::IntoResponse;
use axum::{Json, Router};
use chrono::{SecondsFormat, Utc};
use dotenvy::dotenv;
use sea_orm::{ConnectOptions, Database};
use std::env;
use std::net::SocketAddr;
use std::time::Duration;
use tower_http::cors::{Any, CorsLayer};
use tracing::{error, info};

pub mod api {
    pub mod handlers;
    pub mod model;
}

pub mod error;
pub mod service;

pub mod db {
    pub mod entity;
    pub mod repository;
}

pub async fn run() {
    // Logging handler using tracing.
    tracing_subscriber::fmt().init();

    // load dotenv.
    dotenv().ok();

    // Get Database connection string and other properties from ENV variable.
    let db_connection_str = env::var("DATABASE_URL").expect("Error getting DB connection string");
    let server_host = env::var("SERVER_HOST").expect("Error getting server host");
    let server_port = env::var("SERVER_PORT").expect("Error getting server port");
    let server_addr = server_host + ":" + &*server_port;

    // Setup DB connection for Sea-ORM.
    let mut db_opt = ConnectOptions::new(db_connection_str);
    db_opt
        .max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(5))
        .acquire_timeout(Duration::from_secs(5))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .set_schema_search_path("axum".into());

    let db = Database::connect(db_opt)
        .await
        .map_err(|e| {
            error!("Failed to create database connection pool: {}", e);
            panic!("Failed to create database connection pool: {}", e);
        })
        .unwrap();

    // CORS Middleware
    let cors = CorsLayer::new()
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
        ])
        .allow_origin(Any);

    // build our application with a route
    let app = Router::new()
        .nest("/hello", api::handlers::hello_handler::routes())
        .nest("/users", api::handlers::users_handler::routes())
        .layer(cors)
        .with_state(db);

    let app = app.fallback(handler_404);

    // run it
    let server_address: SocketAddr = server_addr.parse().unwrap();
    info!("Starting server at {}", server_addr);
    if let Err(e) = axum::Server::bind(&server_address)
        .serve(app.into_make_service())
        .await
    {
        error!("Server error: {}", e);
    }
}

// Page not found fallback handlers
async fn handler_404() -> impl IntoResponse {
    Json(ApiErrorResponse {
        status: 404,
        message: "Endpoint not found".to_owned(),
        time: Utc::now().to_rfc3339_opts(SecondsFormat::Micros, true),
        debug_message: Some("Endpoint you are requesting is not found".to_owned()),
        sub_errors: vec![],
    })
}
