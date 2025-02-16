use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use kernel::model::id::BookId;
use registry::AppRegistry;
use shared::error::AppError;

use crate::model::book::{BookResponse, CreateBookRequest};

pub async fn register_book(
    registry: State<AppRegistry>,
    Json(req): Json<CreateBookRequest>,
) -> Result<StatusCode, AppError> {
    registry
        .book_repository()
        .create(req.into())
        .await
        .map(|_| StatusCode::CREATED)
}

pub async fn show_book_list(
    registry: State<registry::AppRegistry>,
) -> Result<Json<Vec<BookResponse>>, AppError> {
    registry
        .book_repository()
        .find_all()
        .await
        .map(|v| v.into_iter().map(BookResponse::from).collect::<Vec<_>>())
        .map(Json)
}

pub async fn show_book(
    Path(book_id): Path<BookId>,
    registry: State<registry::AppRegistry>,
) -> Result<Json<BookResponse>, AppError> {
    registry
        .book_repository()
        .find_by_id(book_id)
        .await
        .and_then(|b| match b {
            Some(b) => Ok(Json(b.into())),
            None => Err(AppError::EntityNotFound(String::from(
                "specified book was not found",
            ))),
        })
}
