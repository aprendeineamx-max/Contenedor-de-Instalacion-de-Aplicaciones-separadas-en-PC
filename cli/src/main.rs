use anyhow::Result;
use clap::{Parser, Subcommand};
use serde::Deserialize;

#[derive(Parser)]
#[command(name = "ctnr", version)]
#[command(about = "CLI para administrar contenedores Win32")]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Endpoint del backend (por defecto localhost:8080)
    #[arg(global = true, long, default_value = "http://127.0.0.1:8080")]
    api: String,
}

#[derive(Subcommand)]
enum Commands {
    /// Lista contenedores registrados
    List,
    /// Crea un contenedor placeholder
    Create { name: String },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    match &cli.command {
        Commands::List => list_containers(&cli.api).await?,
        Commands::Create { name } => create_container(&cli.api, name).await?,
    }
    Ok(())
}

async fn list_containers(api: &str) -> Result<()> {
    let containers = fetch_containers(api).await?;
    if containers.is_empty() {
        println!("No hay contenedores registrados todavía.");
    } else {
        for c in containers {
            println!("- [{}] {}", c.status, c.name);
        }
    }
    Ok(())
}

async fn create_container(api: &str, name: &str) -> Result<()> {
    let container = send_create(api, name).await?;
    println!("Contenedor creado: {} ({})", container.name, container.id);
    Ok(())
}

pub(crate) async fn fetch_containers(api: &str) -> Result<Vec<Container>> {
    let url = format!("{api}/api/containers");
    let resp = reqwest::get(url).await?.json::<Vec<Container>>().await?;
    Ok(resp)
}

pub(crate) async fn send_create(api: &str, name: &str) -> Result<Container> {
    let url = format!("{api}/api/containers");
    let payload = serde_json::json!({ "name": name });
    let resp = reqwest::Client::new()
        .post(url)
        .json(&payload)
        .send()
        .await?
        .json::<Container>()
        .await?;
    Ok(resp)
}

#[derive(Debug, Deserialize, Clone)]
pub struct Container {
    pub id: String,
    pub name: String,
    pub status: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{extract::State, routing::get, Json, Router};
    use serde::{Deserialize, Serialize};
    use std::sync::Arc;
    use tokio::{net::TcpListener, sync::Mutex};

    #[derive(Clone, Serialize, Deserialize)]
    struct MockContainer {
        id: String,
        name: String,
        status: String,
    }

    type SharedState = Arc<Mutex<Vec<MockContainer>>>;

    fn mock_router(state: SharedState) -> Router {
        Router::new()
            .route("/api/containers", get(list).post(create))
            .with_state(state)
    }

    async fn list(State(state): State<SharedState>) -> Json<Vec<MockContainer>> {
        let guard = state.lock().await;
        Json(guard.clone())
    }

    #[derive(Deserialize)]
    struct CreatePayload {
        name: String,
    }

    async fn create(
        State(state): State<SharedState>,
        Json(payload): Json<CreatePayload>,
    ) -> Json<MockContainer> {
        let mut guard = state.lock().await;
        let container = MockContainer {
            id: format!("id-{}", guard.len() + 1),
            name: payload.name,
            status: "draft".into(),
        };
        guard.push(container.clone());
        Json(container)
    }

    async fn spawn_server() -> (String, tokio::task::JoinHandle<()>) {
        let state = Arc::new(Mutex::new(vec![]));
        let app = mock_router(state);
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let handle = tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });
        (format!("http://{}", addr), handle)
    }

    #[tokio::test]
    async fn cli_roundtrip_against_mock_backend() -> Result<()> {
        let (api, handle) = spawn_server().await;
        let empty = fetch_containers(&api).await?;
        assert!(empty.is_empty());

        let created = send_create(&api, "demo-app").await?;
        assert_eq!(created.name, "demo-app");

        let list = fetch_containers(&api).await?;
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].name, "demo-app");

        handle.abort();
        Ok(())
    }
}
