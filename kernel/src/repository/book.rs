#[async_trait::async_trait]
pub trait BookRepository: Send + Sync {
    async fn create(
        &self,
        event: crate::model::book::event::CreateBook,
    ) -> shared::error::AppResult<()>;
    async fn find_all(&self) -> shared::error::AppResult<Vec<crate::model::book::Book>>;
    async fn find_by_id(
        &self,
        book_id: uuid::Uuid,
    ) -> shared::error::AppResult<Option<crate::model::book::Book>>;
}
