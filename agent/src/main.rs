use anyhow::Result;
use tokio::signal;
use tracing::{info, warn};

#[tokio::main]
async fn main() -> Result<()> {
    init_tracing();
    info!("Agent runtime inicializado; esperando instrucciones...");

    // Placeholder: aquí se cargarán contenedores registrados y se levantará gRPC.
    wait_for_shutdown().await?;

    info!("Agent apagándose de forma segura.");
    Ok(())
}

fn init_tracing() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(
            std::env::var("AGENT_LOG")
                .unwrap_or_else(|_| "agent=info,tracing=info".to_string()),
        )
        .try_init();
}

async fn wait_for_shutdown() -> Result<()> {
    signal::ctrl_c().await?;
    warn!("Se recibió Ctrl+C, iniciando limpieza.");
    Ok(())
}
