use crate::runtime::HookPlan;
use anyhow::{Context, Result};
use std::{path::Path, process::Stdio};
use tokio::process::Command as TokioCommand;
use tracing::{info, warn};

#[derive(Debug, Clone)]
pub struct LaunchRequest {
    pub executable: String,
    pub args: Vec<String>,
    pub working_dir: Option<String>,
    pub hook_plan: HookPlan,
}

pub async fn launch(request: &LaunchRequest) -> Result<()> {
    validate_binary(&request.executable)?;

    let mut command = TokioCommand::new(&request.executable);
    command.args(&request.args);
    command.envs(request.hook_plan.env.clone());
    command.stdin(Stdio::null());
    command.stdout(Stdio::inherit());
    command.stderr(Stdio::inherit());
    if let Some(dir) = &request.working_dir {
        command.current_dir(dir);
    }

    info!(
        executable = request.executable.as_str(),
        args = ?request.args,
        "Lanzando proceso con entorno aislado"
    );

    let mut child = command
        .spawn()
        .context("No se pudo iniciar el proceso contenedor")?;
    let status = child.wait().await?;
    if !status.success() {
        warn!(
            code = status.code(),
            "Proceso contenedor terminó con estado inesperado"
        );
    }

    Ok(())
}

fn validate_binary(path: &str) -> Result<()> {
    if Path::new(path).exists() || which::which(path).is_ok() {
        Ok(())
    } else {
        Err(anyhow::anyhow!("No se encontró el ejecutable {path}"))
    }
}
