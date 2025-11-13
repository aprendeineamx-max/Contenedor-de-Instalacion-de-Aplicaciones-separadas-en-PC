use anyhow::{Context, Result};
use redis::{aio::MultiplexedConnection, Client};
use tokio::time::{timeout, Duration};
use tracing::info;

#[derive(Clone)]
pub struct TaskQueue {
    client: Client,
}

impl TaskQueue {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn connect(url: Option<&str>) -> Result<Option<Self>> {
        let Some(url) = url else {
            return Ok(None);
        };
        let client = Client::open(url)?;
        let queue = Self::new(client);
        queue.ping().await?;
        info!("Redis conectado en {url}");
        Ok(Some(queue))
    }

    pub async fn ping(&self) -> Result<()> {
        let mut conn = self.connection().await?;
        timeout(
            Duration::from_secs(2),
            redis::cmd("PING").query_async::<_, String>(&mut conn),
        )
        .await
        .context("timeout when pinging redis")??;
        Ok(())
    }

    pub async fn enqueue(&self, channel: &str, payload: &str) -> Result<()> {
        let mut conn = self.connection().await?;
        redis::cmd("LPUSH")
            .arg(channel)
            .arg(payload)
            .query_async::<_, ()>(&mut conn)
            .await
            .context("No se pudo encolar mensaje")?;
        Ok(())
    }

    pub async fn dequeue(&self, channel: &str, wait: Duration) -> Result<Option<String>> {
        let mut conn = self.connection().await?;
        let res: Option<(String, String)> = timeout(
            wait,
            redis::cmd("BRPOP")
                .arg(channel)
                .arg((wait.as_secs() + 1) as usize)
                .query_async(&mut conn),
        )
        .await
        .unwrap_or(Ok(None))
        .context("No se pudo leer de la cola Redis")?;
        Ok(res.map(|(_, payload)| payload))
    }

    async fn connection(&self) -> Result<MultiplexedConnection> {
        let conn = self
            .client
            .get_multiplexed_async_connection()
            .await
            .context("No se pudo abrir conexión Redis")?;
        Ok(conn)
    }
}
