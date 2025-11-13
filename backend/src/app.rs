use crate::{
    queue::TaskQueue,
    store::{ContainerRecord, ListFilter, Store},
};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tower_http::cors::{Any, CorsLayer};
use tracing::{error, warn};

#[derive(Clone)]
pub struct AppState {
    pub version: String,
    pub store: Store,
    pub queue: Option<TaskQueue>,
}

impl AppState {
    pub fn new(version: String, store: Store, queue: Option<TaskQueue>) -> Self {
        Self {
            version,
            store,
            queue,
        }
    }
}

#[derive(Debug, Deserialize, Default)]
pub struct ListQuery {
    pub status: Option<String>,
    pub search: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

impl ListQuery {
    fn into_filter(self) -> ListFilter {
        ListFilter {
            status: self.status.filter(|s| !s.is_empty()),
            search: self.search.filter(|s| !s.is_empty()),
            limit: self.limit.unwrap_or(25).clamp(1, 100),
            offset: self.offset.unwrap_or(0).max(0),
        }
    }
}

pub fn build_router(state: AppState) -> Router {
    Router::new()
        .route("/healthz", get(health))
        .route(
            "/api/containers",
            post(create_container).get(list_containers),
        )
        .route(
            "/api/containers/:id",
            get(get_container).delete(delete_container),
        )
        .with_state(state)
        .layer(CorsLayer::new().allow_origin(Any))
}

async fn health(State(state): State<AppState>) -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".into(),
        version: state.version,
    })
}

async fn list_containers(
    State(state): State<AppState>,
    Query(query): Query<ListQuery>,
) -> Result<Json<Vec<ContainerRecord>>, StatusCode> {
    let filter = query.into_filter();
    let items = state
        .store
        .list(&filter)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(items))
}

async fn create_container(
    State(state): State<AppState>,
    Json(payload): Json<HttpCreateContainerRequest>,
) -> Result<Json<ContainerRecord>, StatusCode> {
    if payload.name.trim().is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let record = state
        .store
        .create(payload.name.trim(), payload.version.clone())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some(queue) = &state.queue {
        if let Err(err) = queue.enqueue("containers:create", &record.id).await {
            warn!(error = ?err, "No se pudo encolar tarea de creación");
        }
    }

    Ok(Json(record))
}

async fn delete_container(Path(id): Path<String>, State(state): State<AppState>) -> StatusCode {
    match state.store.delete(&id).await {
        Ok(true) => StatusCode::NO_CONTENT,
        Ok(false) => StatusCode::NOT_FOUND,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

async fn get_container(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<ContainerRecord>, StatusCode> {
    match state.store.get(&id).await {
        Ok(Some(record)) => Ok(Json(record)),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(err) => {
            error!(container_id = id, ?err, "Error obteniendo contenedor");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
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
