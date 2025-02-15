#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateBookRequest {
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub description: String,
}

impl From<CreateBookRequest> for kernel::model::book::event::CreateBook {
    fn from(value: CreateBookRequest) -> Self {
        let CreateBookRequest {
            title,
            author,
            isbn,
            description,
        } = value;
        Self {
            title,
            author,
            isbn,
            description,
        }
    }
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BookResponse {
    pub id: kernel::model::id::BookId,
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub description: String,
}

impl From<kernel::model::book::Book> for BookResponse {
    fn from(value: kernel::model::book::Book) -> Self {
        let kernel::model::book::Book {
            id,
            title,
            author,
            isbn,
            description,
        } = value;
        Self {
            id,
            title,
            author,
            isbn,
            description,
        }
    }
}
