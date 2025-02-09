pub mod event;

#[derive(Debug)]
pub struct Book {
    pub id: uuid::Uuid,
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub description: String,
}
