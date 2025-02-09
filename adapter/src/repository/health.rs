#[derive(derive_new::new)]
pub struct HealthCheckRepositoryImpl {
    db: crate::database::ConnectionPool,
}

#[async_trait::async_trait]
impl kernel::repository::health::HealthCheckRepository for HealthCheckRepositoryImpl {
    async fn check_db(&self) -> bool {
        sqlx::query("SELECT 1")
            .execute(self.db.inner_ref())
            .await
            .is_ok()
    }
}
