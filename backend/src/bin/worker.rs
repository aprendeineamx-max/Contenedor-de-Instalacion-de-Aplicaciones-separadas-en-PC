use backend::{config::Settings, queue::TaskQueue, store::Store, workers::InstallWorker};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    backend::init_tracing();

    let settings = Settings::load();
    let store = Store::open(&settings.database_url).await?;
    let queue = TaskQueue::connect(settings.redis_url.as_deref()).await?;

    let Some(queue) = queue else {
        anyhow::bail!("Redis no está configurado; defina REDIS_URL para ejecutar el worker");
    };

    InstallWorker::new(queue, store).run().await?;
    Ok(())
}
