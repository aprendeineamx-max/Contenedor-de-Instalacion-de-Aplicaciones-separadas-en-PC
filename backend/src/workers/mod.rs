use std::time::Duration;

use anyhow::Result;
use tracing::{info, warn};

use crate::{queue::TaskQueue, store::Store};

pub struct InstallWorker {
    queue: TaskQueue,
    store: Store,
}

impl InstallWorker {
    pub fn new(queue: TaskQueue, store: Store) -> Self {
        Self { queue, store }
    }

    pub async fn run(self) -> Result<()> {
        info!("Worker de instalación iniciado; esperando tareas");
        loop {
            if let Some(payload) = self
                .queue
                .dequeue("containers:create", Duration::from_secs(30))
                .await?
            {
                info!(task = payload.as_str(), "Procesando tarea de creación");
                if let Some(container) = self.store.get(&payload).await? {
                    // Aquí conectaremos el pipeline de captura / instalación
                    info!(
                        id = container.id.as_str(),
                        name = container.name.as_str(),
                        "Tarea recibida (stub)"
                    );
                } else {
                    warn!(
                        task = payload.as_str(),
                        "No se encontró el contenedor indicado"
                    );
                }
            }
        }
    }
}
