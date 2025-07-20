use axum::{Router, routing::{get, post}};  // Removed unused put/delete
use crate::controllers::post_controller::*;
use sqlx::PgPool;

pub fn create_routes() -> Router<PgPool> {
    Router::new()
        .route("/posts", post(create_post).get(get_all_posts))
        .route("/posts/:id", get(get_post).put(update_post).delete(delete_post))
}