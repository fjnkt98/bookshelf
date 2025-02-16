use axum::{routing::post, Router};
use registry::AppRegistry;

use crate::handler::book::{delete_book, register_book, show_book, show_book_list, update_book};

pub fn build_book_routers() -> Router<AppRegistry> {
    let books_routers = Router::new()
        .route("/", post(register_book))
        .route("/", axum::routing::get(show_book_list))
        .route("/:book_id", axum::routing::get(show_book))
        .route("/:book_id", axum::routing::put(update_book))
        .route("/:book_id", axum::routing::delete(delete_book));

    Router::new().nest("/books", books_routers)
}
