use crate::{
    queue::TaskQueue,
    security::{self, AuthConfig},
    store::{ContainerRecord, ListFilter, Store},
};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    middleware,
    response::sse::{Event, KeepAlive, Sse},
    routing::{get, post},
    Json, Router,
};
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use std::{convert::Infallible, time::Duration};
use tokio_stream::wrappers::IntervalStream;
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
    let auth = AuthConfig::from_env();
    let rate = security::RateLimiter::new(120, Duration::from_secs(60));
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
        .route("/api/events/containers", get(stream_containers))
        .layer(middleware::from_fn_with_state(rate, security::rate_limit))
        .layer(middleware::from_fn_with_state(
            auth,
            security::require_api_key,
        ))
        .layer(CorsLayer::new().allow_origin(Any))
        .with_state(state)
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

async fn stream_containers(
    State(state): State<AppState>,
) -> Sse<impl futures_core::Stream<Item = Result<Event, Infallible>>> {
    let shared_state = state.clone();
    let stream =
        IntervalStream::new(tokio::time::interval(Duration::from_secs(5))).then(move |_| {
            let state = shared_state.clone();
            async move {
                let payload = state
                    .store
                    .list(&ListFilter::default())
                    .await
                    .ok()
                    .and_then(|items| serde_json::to_string(&items).ok())
                    .unwrap_or_else(|| "[]".into());
                Ok::<_, Infallible>(Event::default().data(payload))
            }
        });

    Sse::new(stream).keep_alive(KeepAlive::new().interval(Duration::from_secs(15)))
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
