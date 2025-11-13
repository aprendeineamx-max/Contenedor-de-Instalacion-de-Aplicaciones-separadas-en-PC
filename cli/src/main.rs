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
    let url = format!("{api}/api/containers");
    let resp: Vec<Container> = reqwest::get(url).await?.json().await?;
    if resp.is_empty() {
        println!("No hay contenedores registrados todavía.");
    } else {
        for c in resp {
            println!("- [{}] {}", c.status, c.name);
        }
    }
    Ok(())
}

async fn create_container(api: &str, name: &str) -> Result<()> {
    let url = format!("{api}/api/containers");
    let payload = serde_json::json!({ "name": name });
    let resp: Container = reqwest::Client::new()
        .post(url)
        .json(&payload)
        .send()
        .await?
        .json()
        .await?;
    println!("Contenedor creado: {} ({})", resp.name, resp.id);
    Ok(())
}

#[derive(Deserialize)]
struct Container {
    id: String,
    name: String,
    status: String,
}
