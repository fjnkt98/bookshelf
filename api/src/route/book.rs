pub fn build_book_routers() -> axum::Router<registry::AppRegistry> {
    let books_routers = axum::Router::new()
        .route(
            "/",
            axum::routing::post(crate::handler::book::register_book),
        )
        .route(
            "/",
            axum::routing::get(crate::handler::book::show_book_list),
        )
        .route(
            "/:book_id",
            axum::routing::get(crate::handler::book::show_book),
        );

    axum::Router::new().nest("/books", books_routers)
}
