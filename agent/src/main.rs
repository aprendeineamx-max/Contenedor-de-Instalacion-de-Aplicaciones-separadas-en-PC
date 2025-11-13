mod registry;

use anyhow::Result;
use registry::ContainerRegistry;
use tokio::signal;
use tracing::{info, warn};

#[tokio::main]
async fn main() -> Result<()> {
    init_tracing();
    info!(
        version = env!("CARGO_PKG_VERSION"),
        "Agent runtime inicializado; cargando contenedores..."
    );

    let registry = ContainerRegistry::load_from("containers").await?;
    let manifests = registry.list();
    if manifests.is_empty() {
        warn!("No se encontraron contenedores registrados en ./containers");
    } else {
        for manifest in manifests {
            info!(
                container_id = manifest.id,
                name = manifest.name,
                version = manifest.version.as_deref().unwrap_or("latest"),
                "Contenedor registrado"
            );
        }
    }

    // Placeholder: aquí se cargarán contenedores registrados y se levantará gRPC.
    wait_for_shutdown().await?;

    info!("Agent apagándose de forma segura.");
    Ok(())
}

fn init_tracing() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(
            std::env::var("AGENT_LOG").unwrap_or_else(|_| "agent=info,tracing=info".to_string()),
        )
        .try_init();
}

async fn wait_for_shutdown() -> Result<()> {
    signal::ctrl_c().await?;
    warn!("Se recibió Ctrl+C, iniciando limpieza.");
    Ok(())
}
