pub mod app;
pub mod grpc;
pub mod store;

pub mod proto {
    tonic::include_proto!("containers.v1");
}

use app::{build_router, AppState};
use grpc::ContainerGrpc;
use std::net::SocketAddr;
use store::Store;
use tokio::net::TcpListener;
use tracing::{info, Level};

pub async fn run() -> anyhow::Result<()> {
    init_tracing();

    let db_path =
        std::env::var("CONTAINERS_DB").unwrap_or_else(|_| "data/containers.db".to_string());
    let store = Store::new(db_path).await?;
    let state = AppState::new(env!("CARGO_PKG_VERSION").to_string(), store.clone());

    let http_addr: SocketAddr = std::env::var("CONTAINERS_HTTP_ADDR")
        .unwrap_or_else(|_| "0.0.0.0:8080".to_string())
        .parse()?;
    let grpc_addr: SocketAddr = std::env::var("CONTAINERS_GRPC_ADDR")
        .unwrap_or_else(|_| "0.0.0.0:50051".to_string())
        .parse()?;

    let http_task = tokio::spawn(async move {
        let listener = TcpListener::bind(http_addr).await?;
        info!("Backend HTTP en http://{http_addr}");
        let app = build_router(state);
        axum::serve(listener, app).await?;
        Ok::<_, anyhow::Error>(())
    });

    let grpc_task = tokio::spawn(async move {
        info!("gRPC escuchando en {grpc_addr}");
        ContainerGrpc::new(store).serve(grpc_addr).await?;
        Ok::<_, anyhow::Error>(())
    });

    let (http_res, grpc_res) = tokio::join!(http_task, grpc_task);
    http_res??;
    grpc_res??;

    Ok(())
}

fn init_tracing() {
    let _ = tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .with_env_filter(
            std::env::var("BACKEND_LOG")
                .unwrap_or_else(|_| "backend=info,axum=info,tower_http=info".to_string()),
        )
        .try_init();
}
