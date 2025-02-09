#[derive(derive_new::new)]
pub struct BookRepositoryImpl {
    db: crate::database::ConnectionPool,
}

#[async_trait::async_trait]
impl kernel::repository::book::BookRepository for BookRepositoryImpl {
    async fn create(&self, event: kernel::model::book::event::CreateBook) -> anyhow::Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO books (title, author, isbn, description)
            VALUES ($1, $2, $3, $4)
            "#,
            event.title,
            event.author,
            event.isbn,
            event.description,
        )
        .execute(self.db.inner_ref())
        .await?;

        Ok(())
    }

    async fn find_all(&self) -> anyhow::Result<Vec<kernel::model::book::Book>> {
        let rows: Vec<crate::database::model::book::BookRow> = sqlx::query_as!(
            crate::database::model::book::BookRow,
            r#"
            SELECT
                book_id,
                title,
                author,
                isbn,
                description
            FROM
                books
            ORDER BY
                created_at DESC
            "#,
        )
        .fetch_all(self.db.inner_ref())
        .await?;

        Ok(rows
            .into_iter()
            .map(kernel::model::book::Book::from)
            .collect())
    }

    async fn find_by_id(
        &self,
        book_id: uuid::Uuid,
    ) -> anyhow::Result<Option<kernel::model::book::Book>> {
        let row: Option<crate::database::model::book::BookRow> = sqlx::query_as!(
            crate::database::model::book::BookRow,
            r#"
            SELECT
                book_id,
                title,
                author,
                isbn,
                description
            FROM
                books
            WHERE
                book_id = $1
            "#,
            book_id,
        )
        .fetch_optional(self.db.inner_ref())
        .await?;

        Ok(row.map(kernel::model::book::Book::from))
    }
}

#[cfg(test)]
mod test {
    use kernel::repository::book::BookRepository;

    use super::*;

    #[sqlx::test]
    async fn test_register_book(
        pool: sqlx::pool::Pool<sqlx::postgres::Postgres>,
    ) -> anyhow::Result<()> {
        let repo = BookRepositoryImpl::new(crate::database::ConnectionPool::new(pool));

        let book = kernel::model::book::event::CreateBook {
            title: "Test Title".to_string(),
            author: "Test Author".to_string(),
            isbn: "Test ISBN".to_string(),
            description: "Test Description".to_string(),
        };

        repo.create(book).await?;

        let res = repo.find_all().await?;
        assert_eq!(res.len(), 1);

        let book_id = res[0].id;
        let res = repo.find_by_id(book_id).await?;
        assert!(res.is_some());

        let kernel::model::book::Book {
            id,
            title,
            author,
            isbn,
            description,
        } = res.unwrap();

        assert_eq!(id, book_id);
        assert_eq!(title, "Test Title");
        assert_eq!(author, "Test Author");
        assert_eq!(isbn, "Test ISBN");
        assert_eq!(description, "Test Description");

        Ok(())
    }
}
