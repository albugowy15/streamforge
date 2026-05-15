use crate::{
    modules::books::models::{BookResponse, CreateBookRequest, UpdateBookRequest},
    shared::app_state::AppState,
};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use std::sync::Arc;

pub async fn create_book_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateBookRequest>,
) -> Result<(StatusCode, Json<BookResponse>), (StatusCode, String)> {
    state
        .book_service
        .create(payload)
        .await
        .map(|book| (StatusCode::CREATED, Json(book)))
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))
}

pub async fn get_book_handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<Json<BookResponse>, (StatusCode, String)> {
    state
        .book_service
        .get(id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?
        .map(Json)
        .ok_or((StatusCode::NOT_FOUND, "Book not found".to_string()))
}

pub async fn update_book_handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
    Json(mut payload): Json<UpdateBookRequest>,
) -> Result<Json<BookResponse>, (StatusCode, String)> {
    payload.id = id;
    state
        .book_service
        .update(payload)
        .await
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))
}

pub async fn delete_book_handler(
    State(state): State<Arc<AppState>>,
    Path(id): Path<i64>,
) -> Result<StatusCode, (StatusCode, String)> {
    state
        .book_service
        .delete(id)
        .await
        .map(|_| StatusCode::NO_CONTENT)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))
}

pub async fn list_books_handler(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<BookResponse>>, (StatusCode, String)> {
    state
        .book_service
        .list()
        .await
        .map(Json)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))
}
