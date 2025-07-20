use axum::{
    extract::{Path, State},
    Json,
    // response::{IntoResponse, Response},
    // response::IntoResponse,  // Keep only what's needed
    // http::StatusCode,
};
use uuid::Uuid;
use crate::{models::post::Post, AppError};
use sqlx::PgPool;
use crate::models::post::CreatePost;  // Add this import

#[axum::debug_handler]
// Change the parameter type in create_post
pub async fn create_post(
    State(pool): State<PgPool>,
    Json(post): Json<CreatePost>,  // Changed from Post to CreatePost
) -> Result<Json<Post>, AppError> {
    let rec = sqlx::query_as!(
        Post,
        r#"INSERT INTO posts (title, body) 
           VALUES ($1, $2)
           RETURNING id, title, body, created_at"#,
        post.title,
        post.body
    )
    .fetch_one(&pool)
    .await?;

    Ok(Json(rec))
}

#[axum::debug_handler]
pub async fn get_all_posts(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<Post>>, AppError> {
    let posts = sqlx::query_as!(
        Post,
        r#"SELECT id, title, body, created_at FROM posts"#
    )
    .fetch_all(&pool)
    .await?;

    Ok(Json(posts))
}

#[axum::debug_handler]
pub async fn get_post(
    Path(id): Path<Uuid>,
    State(pool): State<PgPool>,
) -> Result<Json<Post>, AppError> {
    let post = sqlx::query_as!(
        Post,
        r#"SELECT id, title, body, created_at FROM posts WHERE id = $1"#,
        id
    )
    .fetch_optional(&pool)
    .await?;

    match post {
        Some(post) => Ok(Json(post)),
        None => Err(AppError::NotFound),
    }
}

#[axum::debug_handler]
pub async fn update_post(
    Path(id): Path<Uuid>,
    State(pool): State<PgPool>,
    Json(post): Json<Post>,
) -> Result<Json<Post>, AppError> {
    let updated = sqlx::query_as!(
        Post,
        r#"
        UPDATE posts 
        SET title = $1, body = $2
        WHERE id = $3
        RETURNING id, title, body, created_at
        "#,
        post.title,
        post.body,
        id
    )
    .fetch_optional(&pool)
    .await?;

    match updated {
        Some(post) => Ok(Json(post)),
        None => Err(AppError::NotFound),
    }
}

#[axum::debug_handler]
pub async fn delete_post(
    Path(id): Path<Uuid>,
    State(pool): State<PgPool>,
) -> Result<Json<Post>, AppError> {
    let deleted = sqlx::query_as!(
        Post,
        r#"
        DELETE FROM posts 
        WHERE id = $1
        RETURNING id, title, body, created_at
        "#,
        id
    )
    .fetch_optional(&pool)
    .await?;

    match deleted {
        Some(post) => Ok(Json(post)),
        None => Err(AppError::NotFound),
    }
}