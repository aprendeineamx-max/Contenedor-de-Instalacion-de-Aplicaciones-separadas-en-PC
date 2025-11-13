use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::{
    sqlite::{SqliteConnectOptions, SqliteJournalMode},
    FromRow, QueryBuilder, SqlitePool,
};
use std::{path::Path, str::FromStr};
use tokio::fs;
use uuid::Uuid;

#[derive(Clone)]
pub struct Store {
    pool: SqlitePool,
}

#[derive(Clone, Debug, Serialize, Deserialize, FromRow)]
pub struct ContainerRecord {
    pub id: String,
    pub name: String,
    pub version: Option<String>,
    pub status: String,
}

#[derive(Debug, Default)]
pub struct ListFilter {
    pub status: Option<String>,
    pub search: Option<String>,
    pub limit: i64,
    pub offset: i64,
}

impl Store {
    pub async fn new(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).await?;
        }

        let options = SqliteConnectOptions::from_str(path.to_string_lossy().as_ref())?
            .create_if_missing(true)
            .journal_mode(SqliteJournalMode::Wal);

        let pool = SqlitePool::connect_with(options).await?;
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS containers (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                version TEXT,
                status TEXT NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )
            "#,
        )
        .execute(&pool)
        .await?;

        Ok(Self { pool })
    }

    pub async fn list(&self, filter: &ListFilter) -> Result<Vec<ContainerRecord>> {
        let mut builder =
            QueryBuilder::new("SELECT id, name, version, status FROM containers WHERE 1=1");

        if let Some(status) = &filter.status {
            builder.push(" AND status = ").push_bind(status);
        }

        if let Some(search) = &filter.search {
            builder
                .push(" AND (name LIKE ")
                .push_bind(format!("%{search}%"))
                .push(" OR id LIKE ")
                .push_bind(format!("%{search}%"))
                .push(")");
        }

        builder.push(" ORDER BY created_at DESC");
        builder
            .push(" LIMIT ")
            .push_bind(filter.limit.max(1))
            .push(" OFFSET ")
            .push_bind(filter.offset.max(0));

        let query = builder.build_query_as::<ContainerRecord>();
        let rows = query.fetch_all(&self.pool).await?;
        Ok(rows)
    }

    pub async fn create(&self, name: &str, version: Option<String>) -> Result<ContainerRecord> {
        let record = ContainerRecord {
            id: Uuid::new_v4().to_string(),
            name: name.to_string(),
            version,
            status: "draft".into(),
        };

        sqlx::query(
            r#"INSERT INTO containers (id, name, version, status) VALUES (?1, ?2, ?3, ?4)"#,
        )
        .bind(&record.id)
        .bind(&record.name)
        .bind(&record.version)
        .bind(&record.status)
        .execute(&self.pool)
        .await?;

        Ok(record)
    }

    pub async fn get(&self, id: &str) -> Result<Option<ContainerRecord>> {
        let row = sqlx::query_as::<_, ContainerRecord>(
            "SELECT id, name, version, status FROM containers WHERE id = ?1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(row)
    }

    pub async fn delete(&self, id: &str) -> Result<bool> {
        let result = sqlx::query("DELETE FROM containers WHERE id = ?1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }
}
