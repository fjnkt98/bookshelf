#[async_trait::async_trait]
pub trait BookRepository: Send + Sync {
    async fn create(&self, event: crate::model::book::event::CreateBook) -> anyhow::Result<()>;
    async fn find_all(&self) -> anyhow::Result<Vec<crate::model::book::Book>>;
    async fn find_by_id(
        &self,
        book_id: uuid::Uuid,
    ) -> anyhow::Result<Option<crate::model::book::Book>>;
}
