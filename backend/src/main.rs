mod grpc;
mod store;

pub mod proto {
    tonic::include_proto!("containers.v1");
}

use crate::grpc::ContainerGrpc;
use crate::store::{ContainerRecord, Store};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{delete, get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::cors::{Any, CorsLayer};
use tracing::{info, Level};

#[derive(Clone)]
struct AppState {
    version: String,
    store: Store,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .with_env_filter(
            std::env::var("BACKEND_LOG")
                .unwrap_or_else(|_| "backend=info,axum=info,tower_http=info".to_string()),
        )
        .init();

    let store = Store::new();
    let state = AppState {
        version: env!("CARGO_PKG_VERSION").to_string(),
        store: store.clone(),
    };

    let app = Router::new()
        .route("/healthz", get(health))
        .route(
            "/api/containers",
            post(create_container).get(list_containers),
        )
        .route("/api/containers/:id", delete(delete_container))
        .with_state(state.clone())
        .layer(CorsLayer::new().allow_origin(Any));

    let http_addr: SocketAddr = "0.0.0.0:8080".parse()?;
    let grpc_addr: SocketAddr = "0.0.0.0:50051".parse()?;

    let http_task = tokio::spawn(async move {
        let listener = TcpListener::bind(http_addr).await?;
        info!("Backend HTTP en http://{http_addr}");
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

async fn health(State(state): State<AppState>) -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".into(),
        version: state.version,
    })
}

async fn list_containers(State(state): State<AppState>) -> Json<Vec<ContainerRecord>> {
    Json(state.store.list().await)
}

async fn create_container(
    State(state): State<AppState>,
    Json(payload): Json<HttpCreateContainerRequest>,
) -> Json<ContainerRecord> {
    let record = state
        .store
        .create(&payload.name, payload.version.clone())
        .await;
    Json(record)
}

async fn delete_container(Path(id): Path<String>, State(state): State<AppState>) -> StatusCode {
    match state.store.delete(&id).await {
        Some(_) => StatusCode::NO_CONTENT,
        None => StatusCode::NOT_FOUND,
    }
}

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    version: String,
}

#[derive(Deserialize)]
struct HttpCreateContainerRequest {
    name: String,
    version: Option<String>,
}
