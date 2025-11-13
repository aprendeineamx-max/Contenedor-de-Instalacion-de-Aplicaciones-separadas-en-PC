mod hooks;
mod launcher;
mod mount;
mod registry;
mod runtime;
mod services;

use anyhow::{Context, Result};
use launcher::LaunchRequest;
use mount::MountSession;
use registry::ContainerRegistry;
use runtime::HookEngine;
use services::ServiceSandbox;
use tokio::signal;
use tracing::{info, warn};

#[tokio::main]
async fn main() -> Result<()> {
    init_tracing();
    ensure_permissions()?;
    info!(
        version = env!("CARGO_PKG_VERSION"),
        "Agent runtime inicializado; cargando contenedores..."
    );

    let registry = ContainerRegistry::load_from("containers").await?;
    let registered = registry.list();
    let hook_engine = HookEngine::new();
    let service_sandbox = ServiceSandbox::new();
    let mut active_mounts: Vec<MountSession> = Vec::new();

    if registered.is_empty() {
        warn!("No se encontraron contenedores registrados en ./containers");
    } else {
        for container in &registered {
            info!(
                container_id = container.manifest.id.as_str(),
                name = container.manifest.name.as_str(),
                version = container.manifest.version.as_deref().unwrap_or("latest"),
                "Contenedor registrado"
            );

            let plan = hook_engine.prepare(container).await?;
            hook_engine.activate(&plan)?;

            if let Ok(mount) = MountSession::mount(&container.root, None).await {
                active_mounts.push(mount);
            }

            if let Some(entrypoint) = &container.manifest.entrypoint {
                let request = LaunchRequest {
                    executable: entrypoint.clone(),
                    args: Vec::new(),
                    working_dir: Some(container.root.to_string_lossy().into_owned()),
                    hook_plan: plan.clone(),
                };

                service_sandbox
                    .register_placeholder(&container.manifest.id)
                    .ok();

                if let Err(err) = launcher::launch(&request).await {
                    warn!(?err, "No se pudo lanzar el proceso declarado en entrypoint");
                }
            }

            info!(
                container_id = container.manifest.id.as_str(),
                mounts = ?plan.mounts,
                redirects = ?plan.redirects,
                "Hooks aplicados correctamente"
            );
        }
    }

    wait_for_shutdown().await?;

    info!("Agent apagandose de forma segura.");
    drop(active_mounts);
    Ok(())
}

fn init_tracing() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(
            std::env::var("AGENT_LOG").unwrap_or_else(|_| "agent=info,tracing=info".to_string()),
        )
        .try_init();
}

async fn wait_for_shutdown() -> Result<()> {
    signal::ctrl_c().await?;
    warn!("Se recibio Ctrl+C, iniciando limpieza.");
    Ok(())
}

fn ensure_permissions() -> Result<()> {
    #[cfg(target_os = "windows")]
    {
        let output = std::process::Command::new("powershell")
            .args([
                "-NoProfile",
                "-Command",
                "[Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent().IsInRole([Security.Principal.WindowsBuiltinRole]::Administrator)",
            ])
            .output()
            .context("No se pudo verificar permisos")?;
        if !String::from_utf8_lossy(&output.stdout)
            .trim()
            .eq_ignore_ascii_case("True")
        {
            return Err(anyhow::anyhow!(
                "El agent requiere permisos de administrador para aplicar hooks y montar volúmenes."
            ));
        }
    }
    Ok(())
}
