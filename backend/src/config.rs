use std::env;

#[derive(Debug, Clone)]
pub struct Settings {
    pub database_url: String,
    pub redis_url: Option<String>,
    pub http_addr: String,
    pub grpc_addr: String,
}

impl Settings {
    pub fn load() -> Self {
        Self {
            database_url: env::var("DATABASE_URL")
                .or_else(|_| env::var("CONTAINERS_DB_URL"))
                .unwrap_or_else(|_| "sqlite://data/containers.db".to_string()),
            redis_url: env::var("REDIS_URL")
                .or_else(|_| env::var("CONTAINERS_REDIS_URL"))
                .ok(),
            http_addr: env::var("CONTAINERS_HTTP_ADDR").unwrap_or_else(|_| "0.0.0.0:8080".into()),
            grpc_addr: env::var("CONTAINERS_GRPC_ADDR").unwrap_or_else(|_| "0.0.0.0:50051".into()),
        }
    }
}
