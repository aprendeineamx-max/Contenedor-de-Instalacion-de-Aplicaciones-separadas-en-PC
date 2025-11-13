use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};
use std::sync::Arc;
use tokio::{sync::Semaphore, time::Duration};

#[derive(Clone, Default)]
pub struct AuthConfig {
    api_key: Option<String>,
}

impl AuthConfig {
    pub fn from_env() -> Self {
        Self {
            api_key: std::env::var("CONTAINERS_API_KEY").ok(),
        }
    }
}

#[derive(Clone)]
pub struct RateLimiter {
    permits: Arc<Semaphore>,
    refill: Duration,
}

impl RateLimiter {
    pub fn new(burst: usize, refill: Duration) -> Self {
        Self {
            permits: Arc::new(Semaphore::new(burst)),
            refill,
        }
    }
}

pub async fn require_api_key(
    State(config): State<AuthConfig>,
    req: Request<Body>,
    next: Next,
) -> Response {
    if let Some(expected) = &config.api_key {
        let provided = req
            .headers()
            .get("x-api-key")
            .and_then(|value| value.to_str().ok())
            .unwrap_or_default();
        if provided != expected {
            return StatusCode::UNAUTHORIZED.into_response();
        }
    }

    next.run(req).await
}

pub async fn rate_limit(
    State(limiter): State<RateLimiter>,
    req: Request<Body>,
    next: Next,
) -> Response {
    let permit = limiter
        .permits
        .clone()
        .acquire_owned()
        .await
        .map_err(|_| StatusCode::TOO_MANY_REQUESTS.into_response());

    if let Ok(permit) = permit {
        let refill = limiter.refill;
        tokio::spawn(async move {
            tokio::time::sleep(refill).await;
            drop(permit);
        });
        next.run(req).await
    } else {
        StatusCode::TOO_MANY_REQUESTS.into_response()
    }
}
