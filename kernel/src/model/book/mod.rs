pub mod event;

#[derive(Debug)]
pub struct Book {
    pub id: crate::model::id::BookId,
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub description: String,
}
