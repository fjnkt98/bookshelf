#[derive(Clone)]
pub struct AppRegistry {
    health_check_repository: std::sync::Arc<dyn kernel::repository::health::HealthCheckRepository>,
}

impl AppRegistry {
    pub fn new(pool: adapter::database::ConnectionPool) -> Self {
        let health_check_repository = std::sync::Arc::new(
            adapter::repository::health::HealthCheckRepositoryImpl::new(pool.clone()),
        );
        Self {
            health_check_repository,
        }
    }

    pub fn health_check_repository(
        &self,
    ) -> std::sync::Arc<dyn kernel::repository::health::HealthCheckRepository> {
        self.health_check_repository.clone()
    }
}
