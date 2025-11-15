use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::{
    postgres::{PgPoolOptions, PgRow},
    PgPool, Postgres, QueryBuilder, Row,
};
use tracing::error;
use uuid::Uuid;

#[derive(Clone)]
pub struct Store {
    pool: PgPool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
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

impl<'r> sqlx::FromRow<'r, PgRow> for ContainerRecord {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        let id: String = row.try_get("id")?;
        let name: String = row.try_get("name")?;
        let status: String = row.try_get("status")?;
        let version: String = row.try_get("version")?;
        let version = if version.is_empty() {
            None
        } else {
            Some(version)
        };

        Ok(Self {
            id,
            name,
            version,
            status,
        })
    }
}

impl Store {
    pub async fn open(database_url: &str) -> Result<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(database_url)
            .await?;
        sqlx::migrate!("./migrations").run(&pool).await?;
        Ok(Self { pool })
    }

    pub async fn list(&self, filter: &ListFilter) -> Result<Vec<ContainerRecord>> {
        let mut builder = QueryBuilder::<Postgres>::new(
            "SELECT id, name, COALESCE(version, '') AS version, status FROM containers WHERE 1=1",
        );

        if let Some(status) = &filter.status {
            builder.push(" AND status = ").push_bind(status);
        }

        if let Some(search) = &filter.search {
            builder
                .push(" AND (name ILIKE ")
                .push_bind(format!("%{search}%"))
                .push(" OR id ILIKE ")
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

        if let Err(err) = sqlx::query(
            "INSERT INTO containers (id, name, version, status) VALUES ($1, $2, $3, $4)",
        )
        .bind(&record.id)
        .bind(&record.name)
        .bind(&record.version)
        .bind(&record.status)
        .execute(&self.pool)
        .await
        {
            error!(?err, container = ?record.name, "Failed to persist container");
            return Err(err.into());
        }

        Ok(record)
    }

    pub async fn get(&self, id: &str) -> Result<Option<ContainerRecord>> {
        let row = sqlx::query_as::<_, ContainerRecord>(
            "SELECT id, name, COALESCE(version, '') AS version, status FROM containers WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(row)
    }

    pub async fn delete(&self, id: &str) -> Result<bool> {
        let result = sqlx::query("DELETE FROM containers WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }
}
