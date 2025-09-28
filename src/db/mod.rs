use crate::conf::DataBaseOptions;
use chrono::DateTime;
use sqlx::{Executor, FromRow, SqlitePool};

#[derive(Debug, FromRow)]
pub struct PipeLineEntity {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub age: Option<u8>,
    pub created_at: DateTime<chrono::Utc>,
}

pub struct Repository {
    pool: Option<SqlitePool>,
    options: DataBaseOptions,
}

impl Repository {
    pub fn new(options: DataBaseOptions) -> Self {
        Self {
            pool: None,
            options,
        }
    }

    pub async fn init_db(&self) -> Result<(), sqlx::Error> {
        let pool = SqlitePool::connect(&self.options.url()).await?;
        let query1 = sqlx::query(
            r#"CREATE TABLE IF NOT EXISTS todos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            completed BOOLEAN NOT NULL DEFAULT FALSE
        )"#);
        query1.execute(&pool).await?;
        Ok(())
    }
}
