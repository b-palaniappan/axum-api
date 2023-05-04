use crate::api::model::api_error::ApiErrorResponse;
use axum::extract::State;
use axum::headers::UserAgent;
use axum::http::{HeaderMap, Method, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Json, Router, TypedHeader};
use chrono::{SecondsFormat, Utc};
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::mysql::MySqlPoolOptions;
use sqlx::{Error, MySqlPool};
use std::env;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tracing::{error, info};

pub mod api {
    pub mod handlers;
    pub mod model;
}

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

    // Setup connection pool.
    let pool = MySqlPoolOptions::new()
        .max_connections(10)
        .min_connections(1)
        .connect(&db_connection_str)
        .await
        .map_err(|e| {
            error!("Failed to create database connection pool: {}", e);
            panic!("Failed to create database connection pool: {}", e);
        })
        .unwrap();

    // Trigger migration scripts for MySQL using SQLX
    let response = sqlx::migrate!("./migrations").run(&pool).await;
    match response {
        Ok(_) => info!("Migration completed successfully"),
        Err(e) => error!("Error in DB Migration: {}", e),
    }

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
        .route("/json", get(handler_json))
        .nest("/hello", api::handlers::hello_handler::routes())
        .nest("/users", api::handlers::users_handler::routes())
        .layer(cors)
        .with_state(pool);

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

#[derive(Serialize, Deserialize)]
struct Message {
    message: String,
    status: String,
}

// TODO: Delete this after playing with JSON handlers
async fn handler_json(
    State(pool): State<MySqlPool>,
    TypedHeader(user_agent): TypedHeader<UserAgent>,
    headers: HeaderMap,
) -> Response {
    info!("Handle Json payload");
    // Get user agent header.
    info!("User agent - {}", user_agent);
    // Get custom header from Request header.
    let header_value = headers.get("x-server-version").unwrap().to_str().unwrap();
    info!("Custom user header - {}", header_value);

    // Make a simple query to return the given parameter (use a question mark `?` instead of `$1` for MySQL)
    let response: Result<(String,), Error> =
        sqlx::query_as("SELECT 'Hello'").fetch_one(&pool).await;
    match response {
        Ok(r) => info!("DB Response -> {}", r.0),
        Err(e) => error!("Error getting data {}", e),
    }

    // With custom Response Code.
    (
        StatusCode::CREATED,
        Json(Message {
            message: "Hello".to_string(),
            status: "Success".to_string(),
        }),
    )
        .into_response()
}
