use kernel::repository::book::BookRepository;

#[derive(Clone)]
pub struct AppRegistry {
    health_check_repository: std::sync::Arc<dyn kernel::repository::health::HealthCheckRepository>,
    book_repository: std::sync::Arc<dyn BookRepository>,
}

impl AppRegistry {
    pub fn new(pool: adapter::database::ConnectionPool) -> Self {
        let health_check_repository = std::sync::Arc::new(
            adapter::repository::health::HealthCheckRepositoryImpl::new(pool.clone()),
        );
        let book_repository = std::sync::Arc::new(
            adapter::repository::book::BookRepositoryImpl::new(pool.clone()),
        );

        Self {
            health_check_repository,
            book_repository,
        }
    }

    pub fn health_check_repository(
        &self,
    ) -> std::sync::Arc<dyn kernel::repository::health::HealthCheckRepository> {
        self.health_check_repository.clone()
    }

    pub fn book_repository(&self) -> std::sync::Arc<dyn BookRepository> {
        self.book_repository.clone()
    }
}
