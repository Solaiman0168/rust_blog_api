mod routes;
mod controllers;
mod models;

use axum::{
    Router,
    http::{StatusCode, Response},
    response::IntoResponse,
    body::Body
};
use dotenvy::dotenv;
use std::{env, net::SocketAddr};
use sqlx::postgres::PgPoolOptions;
use tower_http::{trace::TraceLayer, cors::CorsLayer};
use routes::create_routes;

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::fmt::init();

    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env");

    // Create database connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool");

    // Run migrations if migrations folder exists
    if std::path::Path::new("./migrations").exists() {
        sqlx::migrate!()
            .run(&pool)
            .await
            .expect("Failed to run migrations");
    }

    // Configure CORS
    let cors = CorsLayer::permissive();

    // Create app with router and layers
    let app = Router::new()
        .nest("/", create_routes())
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(pool);

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("Server listening on {}", addr);
    
    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(),
        app
    ).await.unwrap();
}

// Custom error type for API
pub enum AppError {
    DbError(sqlx::Error),
    NotFound,
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::DbError(err)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response<Body> {
        match self {
            AppError::DbError(err) => {
                (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", err)).into_response()
            }
            AppError::NotFound => {
                (StatusCode::NOT_FOUND, "Resource not found").into_response()
            }
        }
    }
}