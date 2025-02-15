pub async fn register_book(
    registry: axum::extract::State<registry::AppRegistry>,
    axum::Json(req): axum::Json<crate::model::book::CreateBookRequest>,
) -> Result<axum::http::StatusCode, shared::error::AppError> {
    registry
        .book_repository()
        .create(req.into())
        .await
        .map(|_| axum::http::StatusCode::CREATED)
}

pub async fn show_book_list(
    registry: axum::extract::State<registry::AppRegistry>,
) -> Result<axum::Json<Vec<crate::model::book::BookResponse>>, shared::error::AppError> {
    registry
        .book_repository()
        .find_all()
        .await
        .map(|v| {
            v.into_iter()
                .map(crate::model::book::BookResponse::from)
                .collect::<Vec<_>>()
        })
        .map(axum::Json)
}

pub async fn show_book(
    axum::extract::Path(book_id): axum::extract::Path<kernel::model::id::BookId>,
    registry: axum::extract::State<registry::AppRegistry>,
) -> Result<axum::Json<crate::model::book::BookResponse>, shared::error::AppError> {
    registry
        .book_repository()
        .find_by_id(book_id)
        .await
        .and_then(|b| match b {
            Some(b) => Ok(axum::Json(b.into())),
            None => Err(shared::error::AppError::EntityNotFound(String::from(
                "specified book was not found",
            ))),
        })
}
