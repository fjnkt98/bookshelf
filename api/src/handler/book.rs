pub async fn register_book(
    registry: axum::extract::State<registry::AppRegistry>,
    axum::Json(req): axum::Json<crate::model::book::CreateBookRequest>,
) -> Result<axum::http::StatusCode, AppError> {
    registry
        .book_repository()
        .create(req.into())
        .await
        .map(|_| axum::http::StatusCode::CREATED)
        .map_err(AppError::from)
}

pub async fn show_book_list(
    registry: axum::extract::State<registry::AppRegistry>,
) -> Result<axum::Json<Vec<crate::model::book::BookResponse>>, AppError> {
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
        .map_err(AppError::from)
}

pub async fn show_book(
    axum::extract::Path(book_id): axum::extract::Path<uuid::Uuid>,
    registry: axum::extract::State<registry::AppRegistry>,
) -> Result<axum::Json<crate::model::book::BookResponse>, AppError> {
    registry
        .book_repository()
        .find_by_id(book_id)
        .await
        .and_then(|b| match b {
            Some(b) => Ok(axum::Json(b.into())),
            None => Err(anyhow::anyhow!("The specific book was not found")),
        })
        .map_err(AppError::from)
}
