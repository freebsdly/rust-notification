use crate::repository::DatabaseRepository;
use crate::repository::entity::PipeLineEntity;
use anyhow::Context;
use sqlx::SqlitePool;

/**
 * Repository
 */
pub struct PipelineRepository {
    pool: SqlitePool,
}

impl PipelineRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

impl DatabaseRepository<PipeLineEntity, i64> for PipelineRepository {
    async fn find_all(&self) -> Result<Vec<PipeLineEntity>, anyhow::Error> {
        let query = sqlx::query_as::<_, PipeLineEntity>("SELECT * FROM todos");
        query
            .fetch_all(&self.pool)
            .await
            .context("Failed to fetch todos")
    }

    async fn find_by_id(&self, id: i64) -> Result<Option<PipeLineEntity>, anyhow::Error> {
        let query =
            sqlx::query_as::<_, PipeLineEntity>("SELECT * FROM todos WHERE id = ?").bind(id);
        query
            .fetch_optional(&self.pool)
            .await
            .context("Failed to fetch todo")
    }

    async fn save(&self, todo: PipeLineEntity) -> Result<PipeLineEntity, anyhow::Error> {
        let query = sqlx::query("INSERT INTO todos (title, completed) VALUES (?, ?)")
            .bind(todo.name.clone())
            .bind(todo.email.clone());
        let result = query
            .execute(&self.pool)
            .await
            .context("Failed to insert todo")?;
        Ok(PipeLineEntity::new(
            Some(result.last_insert_rowid()),
            todo.name.clone().to_string(),
            todo.email.clone().to_string(),
            todo.age,
        ))
    }
}
