use std::env;
use std::net::SocketAddr;
use std::time::Duration;

use axum::extract::MatchedPath;
use axum::http::{Method, Request};
use axum::response::{IntoResponse, Response};
use axum::{Json, Router};
use chrono::{SecondsFormat, Utc};
use dotenvy::dotenv;
use sea_orm::{ConnectOptions, Database};
use tower_http::classify::ServerErrorsFailureClass;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing::{error, info, info_span, Span};

use crate::api::model::api_error::ApiErrorResponse;

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
        .set_schema_search_path("axum".to_owned());

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
        // Logging middleware with Tower & Tracing.
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request<_>| {
                    let matched_path = request
                        .extensions()
                        .get::<MatchedPath>()
                        .map(MatchedPath::as_str);

                    info_span!(
                        "http_request",
                        method = ?request.method(),
                        matched_path,
                        some_other_field = tracing::field::Empty,
                    )
                })
                .on_request(|_request: &Request<_>, _span: &Span| {
                    info!("Request {:?}", _span);
                })
                .on_response(|_response: &Response, _latency: Duration, _span: &Span| {
                    info!("Response latency {:?}", _latency);
                })
                .on_failure(
                    |_error: ServerErrorsFailureClass, _latency: Duration, _span: &Span| {
                        info!("Failure {:?}", _error);
                    },
                ),
        )
        .with_state(db);

    // 404 NOT FOUND handler
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
