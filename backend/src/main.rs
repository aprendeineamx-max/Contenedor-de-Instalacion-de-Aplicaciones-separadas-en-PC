use axum::{
    extract::State,
    routing::{get, post},
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

    let state = AppState {
        version: env!("CARGO_PKG_VERSION").to_string(),
    };

    let app = Router::new()
        .route("/healthz", get(health))
        .route("/api/containers", post(create_container).get(list_containers))
        .with_state(state)
        .layer(CorsLayer::new().allow_origin(Any));

    let addr: SocketAddr = "0.0.0.0:8080".parse()?;
    let listener = TcpListener::bind(addr).await?;
    info!("Backend escuchando en http://{addr}");
    axum::serve(listener, app).await?;
    Ok(())
}

async fn health(State(state): State<AppState>) -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".into(),
        version: state.version,
    })
}

async fn list_containers() -> Json<Vec<ContainerSummary>> {
    Json(vec![])
}

async fn create_container(Json(payload): Json<CreateContainerRequest>) -> Json<ContainerSummary> {
    let summary = ContainerSummary {
        id: "demo".into(),
        name: payload.name,
        status: "draft".into(),
    };
    Json(summary)
}

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    version: String,
}

#[derive(Serialize)]
struct ContainerSummary {
    id: String,
    name: String,
    status: String,
}

#[derive(Deserialize)]
struct CreateContainerRequest {
    name: String,
}
